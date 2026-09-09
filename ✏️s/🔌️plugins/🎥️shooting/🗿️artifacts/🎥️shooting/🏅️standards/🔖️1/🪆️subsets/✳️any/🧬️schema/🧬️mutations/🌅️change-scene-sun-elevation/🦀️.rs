//! 🌅️ Shooting mutation payload — `ChangeSceneSunElevation`. One of the scene's independently-settable fields.

use crate::diff::ShootingDiff;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeSceneSunElevation {
    pub new_elevation: f64,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ChangeSceneSunElevation {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "scene-sun-elevation", kind: "change-scene-sun-elevation", record: "ChangedSceneSunElevation" };
    fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change sun elevation to {}", self.new_elevation)
    }
}
