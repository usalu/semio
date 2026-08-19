//! 🔅️ Shooting mutation payload — `ChangeSceneAmbientIntensity`. One of the scene's independently-settable fields.

use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeSceneAmbientIntensity {
    pub new_intensity: f64,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ChangeSceneAmbientIntensity {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "scene-ambient-intensity", kind: "change-scene-ambient-intensity", record: "ChangedSceneAmbientIntensity" };
    async fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change ambient intensity to {}", self.new_intensity)
    }
}
