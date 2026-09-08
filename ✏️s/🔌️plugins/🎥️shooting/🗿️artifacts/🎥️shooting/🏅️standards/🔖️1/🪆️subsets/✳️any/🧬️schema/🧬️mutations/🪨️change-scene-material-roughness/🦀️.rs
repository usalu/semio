//! 🪨️ Shooting mutation payload — `ChangeSceneMaterialRoughness`. One of the scene's independently-settable fields.

use crate::ShootingSnapshot;
use crate::diff::ShootingDiff;
use crate::mutations::ShootingMutation;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeSceneMaterialRoughness {
    pub new_roughness: f64,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ChangeSceneMaterialRoughness {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "scene-material-roughness", kind: "change-scene-material-roughness", record: "ChangedSceneMaterialRoughness" };
    fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change material roughness to {}", self.new_roughness)
    }
}
