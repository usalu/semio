//! 🔅️ Shooting mutation payload — `ChangeSceneAmbientIntensity`. One of the scene's independently-settable fields.

use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
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
