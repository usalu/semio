//! 🔒️ Pure artifact publication laws serialize their process-local lease table.
use std::sync::{Mutex, MutexGuard};
static TEST_SERIAL: Mutex<()> = Mutex::new(());
pub(crate) type TestSerialGuard = MutexGuard<'static, ()>;
pub(crate) fn lock() -> TestSerialGuard { TEST_SERIAL.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) }

/// 🧩️ Binds a fixture snapshot to the catalogue admitted by its native test owner.
pub fn geometry_input(snapshot: &crate::Generation3dSnapshot) -> crate::standards::v1::subsets::any::schema::inferences::geometry::GeometryInput<'_> {
    crate::standards::v1::subsets::any::schema::inferences::geometry::GeometryInput::new(snapshot, std::sync::Arc::clone(crate::standards::v1::subsets::any::io::text::snapshot::catalogue::catalogue()),crate::standards::v1::subsets::any::io::geometry::context())
}
