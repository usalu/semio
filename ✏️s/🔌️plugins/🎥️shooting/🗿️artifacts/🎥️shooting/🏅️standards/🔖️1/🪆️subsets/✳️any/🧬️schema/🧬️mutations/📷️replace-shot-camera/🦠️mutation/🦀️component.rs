//! 📷 Shooting mutation payload — `ReplaceShotCamera`. Overwrites the *saved* camera `shot_id` references with a new pose — a no-op (empty diff) when that shot has no saved camera. The free/live viewport camera is session-only runtime state and never reaches this mutation.

use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::ShootingCamera;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplaceShotCamera {
    pub shot_id: String,
    pub new_camera: ShootingCamera,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ReplaceShotCamera {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "shot-camera", kind: "replace-shot-camera", record: "ReplacedShotCamera" };
    fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace shot \"{}\" camera", self.shot_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.shot_id.clone()]
    }
}
