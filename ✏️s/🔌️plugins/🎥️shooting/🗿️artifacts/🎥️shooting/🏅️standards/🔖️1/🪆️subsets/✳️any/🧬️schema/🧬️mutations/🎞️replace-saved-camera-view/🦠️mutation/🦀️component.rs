//! 🎞️ Shooting mutation payload — `ReplaceSavedCameraView`. Whole-value swap of a saved camera's `camera` pose — overwrites rather than merges, so this is a `replace`, not a `change`.

use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::ShootingCamera;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplaceSavedCameraView {
    pub id: String,
    pub new_camera: ShootingCamera,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ReplaceSavedCameraView {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "saved-camera-view", kind: "replace-saved-camera-view", record: "ReplacedSavedCameraView" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace saved camera \"{}\" view", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
