
use std::sync::{Mutex, MutexGuard};

static TEST_SERIAL: Mutex<()> = Mutex::new(());

pub fn lock() -> MutexGuard<'static, ()> {
    TEST_SERIAL.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}
