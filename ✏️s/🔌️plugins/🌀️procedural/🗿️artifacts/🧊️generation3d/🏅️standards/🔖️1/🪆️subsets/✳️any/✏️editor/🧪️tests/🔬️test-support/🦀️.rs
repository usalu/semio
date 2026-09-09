use std::sync::{Mutex, MutexGuard};

static TEST_SERIAL: Mutex<()> = Mutex::new(());

/// 🔒️ Serialises every test that drives the process-wide flow-eval kernel cache, and installs the
/// packaged `brep`/`math` operator sets before the first one runs — without that installation
/// `FlowHost::evaluate` answers `unknown kind: …` for every bundled fixture's nodes
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub fn lock() -> MutexGuard<'static, ()> {
    crate::flow_operators::installed();
    TEST_SERIAL.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}
