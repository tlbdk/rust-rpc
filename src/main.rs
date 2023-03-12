use std::sync::{Arc, Mutex};

pub struct SharedData {
    current_temp: f32,
}

impl SharedData {
    pub fn new(current_temp: f32) -> Self { Self { current_temp } }
}

pub struct AccessSharedData {
    pub sd: Arc<Mutex<SharedData>>,
}

impl AccessSharedData {
    pub fn set_current_temp(&self, current_temp: f32) {
        let mut lock = self.sd.lock().unwrap();
        lock.current_temp = current_temp;
    }

    pub fn get_current_temp(&self) -> f32 {
        let lock = self.sd.lock().unwrap();
        lock.current_temp
    }
}

fn main() {
    let common_data = SharedData::new(1.0);
    let sd = AccessSharedData {
        sd: Arc::new(Mutex::new(common_data)),
    };
    sd.set_current_temp(1.0);
    println!("Hello, world!");
}
