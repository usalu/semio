//! 🪨️ Shooting mutation payload — `ChangeSceneMaterialRoughness`. One of the scene's independently-settable fields.

use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeSceneMaterialRoughness {
    pub new_roughness: f64,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ChangeSceneMaterialRoughness {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "scene-material-roughness", kind: "change-scene-material-roughness", record: "ChangedSceneMaterialRoughness" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change material roughness to {}", self.new_roughness)
    }
}
