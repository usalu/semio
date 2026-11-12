//! 🧹️ Exact CAD presence ownership retirement, including variable-length engagement identifiers.

use super::{CadPresence, CadPresenceMutation};
use std::sync::Arc;
//#region 🏪️StoreRetirement
pub fn empty_terminal() -> CadPresence {
    CadPresence { camera_position: [0.0; 3], camera_target: [0.0; 3], camera_zoom: 0.0, camera_fov: 0.0, engagement_step: String::new(), engagement_pane: None }
}

pub fn terminal_is_empty(value: &CadPresence) -> bool {
    value.engagement_step.is_empty() && value.engagement_pane.is_none()
}

/// 🧹️ Closes the presence store through the framework's exact owner, seeded with CAD's empty terminal root.
pub fn store_disposer() -> Box<dyn semio_plugin::ArtifactOwnedDisposer<store::PresenceStore<CadPresence, CadPresenceMutation>>> {
    Box::new(semio_plugin::PresenceStoreOwnedDisposer::new(Arc::new(empty_terminal()), terminal_is_empty).expect("CAD empty terminal root is its own empty witness"))
}
//#endregion 🏪️StoreRetirement

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
