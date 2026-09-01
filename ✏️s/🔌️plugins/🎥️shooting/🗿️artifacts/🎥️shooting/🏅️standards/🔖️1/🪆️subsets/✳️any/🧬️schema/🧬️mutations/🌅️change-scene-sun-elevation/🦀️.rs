//! 🌅️ Shooting mutation payload — `ChangeSceneSunElevation`. One of the scene's independently-settable fields.

use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeSceneSunElevation {
    pub new_elevation: f64,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ChangeSceneSunElevation {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "scene-sun-elevation", kind: "change-scene-sun-elevation", record: "ChangedSceneSunElevation" };
    async fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change sun elevation to {}", self.new_elevation)
    }
}
