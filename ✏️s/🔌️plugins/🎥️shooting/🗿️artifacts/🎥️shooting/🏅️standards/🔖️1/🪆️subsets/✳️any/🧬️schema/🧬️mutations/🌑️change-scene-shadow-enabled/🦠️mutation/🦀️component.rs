//! 🌑️ Shooting mutation payload — `ChangeSceneShadowEnabled`. One of the scene's independently-settable fields.

use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeSceneShadowEnabled {
    pub new_enabled: bool,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ChangeSceneShadowEnabled {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "scene-shadow-enabled", kind: "change-scene-shadow-enabled", record: "ChangedSceneShadowEnabled" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("{} shadows", if self.new_enabled { "Enable" } else { "Disable" })
    }
}
