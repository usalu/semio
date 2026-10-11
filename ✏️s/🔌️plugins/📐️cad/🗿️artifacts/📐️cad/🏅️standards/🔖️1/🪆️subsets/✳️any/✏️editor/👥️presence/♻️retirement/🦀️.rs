//! 🧹️ Exact CAD presence ownership retirement, including variable-length engagement identifiers.

use super::{CadPresence, CadPresenceMutation};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, ValueError};
use std::sync::Arc;

semio_framework_value::artifact_retire_struct!(CadPresence { camera_position, camera_target, camera_zoom, camera_fov, engagement_step, engagement_pane });

//#region 🧹️SnapshotRetirement
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct CadPresenceRetirementFactory;

impl store::SnapshotRetirementFactory<CadPresence> for CadPresenceRetirementFactory {
    fn retirement_birth_bytes(&self, _snapshot: &Arc<CadPresence>) -> usize { semio_framework_value::retirement::shared::shared_retirement_birth_bytes::<CadPresence>() }

    fn retire(&self, source: Arc<CadPresence>, grant: RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Arc<CadPresence>)> {
        semio_framework_value::retirement::shared::admit_shared_retirement(source, grant, true)
    }
}
//#endregion 🧹️SnapshotRetirement

//#region 🏪️StoreRetirement
pub fn empty_terminal() -> CadPresence {
    CadPresence { camera_position: [0.0; 3], camera_target: [0.0; 3], camera_zoom: 0.0, camera_fov: 0.0, engagement_step: String::new(), engagement_pane: None }
}

pub fn terminal_is_empty(value: &CadPresence) -> bool {
    value.engagement_step.is_empty() && value.engagement_pane.is_none()
}

/// 🧹️ Closes the presence store through the framework's exact owner, seeded with CAD's empty terminal root.
pub fn store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<CadPresence, CadPresenceMutation>>> {
    Box::new(semio_framework_plugin::PresenceStoreOwnedDisposer::new(Arc::new(empty_terminal()), terminal_is_empty).expect("CAD empty terminal root is its own empty witness"))
}
//#endregion 🏪️StoreRetirement

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
