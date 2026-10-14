//! 🔒️ Pure artifact publication laws serialize their process-local lease table.
use std::sync::{Mutex, MutexGuard};
static TEST_SERIAL: Mutex<()> = Mutex::new(());
pub type TestSerialGuard = MutexGuard<'static, ()>;
pub fn lock() -> TestSerialGuard { TEST_SERIAL.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) }
