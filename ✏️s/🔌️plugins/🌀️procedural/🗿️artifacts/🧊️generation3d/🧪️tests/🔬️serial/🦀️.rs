//! 🔒️ Pure artifact publication laws serialize their process-local lease table.
use std::sync::{Mutex, MutexGuard};
static TEST_SERIAL: Mutex<()> = Mutex::new(());
pub(crate) type TestSerialGuard = MutexGuard<'static, ()>;
pub(crate) fn lock() -> TestSerialGuard { TEST_SERIAL.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) }

/// 🧩️ Binds a fixture snapshot to the catalogue admitted by its native test owner.
pub fn geometry_input(snapshot: &semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot) -> semio_s_artifact_procedural_3d_geometry::GeometryInput<'_> {
    semio_s_artifact_procedural_3d_geometry::GeometryInput::new(snapshot,semio_s_artifact_procedural_3d_io_text::snapshot::catalogue::catalogue(),&semio_s_artifact_procedural_3d_io_geometry::NativeGeometryComputeContext)
}
