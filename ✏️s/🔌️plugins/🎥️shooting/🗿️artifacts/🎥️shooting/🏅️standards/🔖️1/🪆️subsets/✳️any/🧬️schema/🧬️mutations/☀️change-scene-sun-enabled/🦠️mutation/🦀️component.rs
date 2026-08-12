//! ☀️ Shooting mutation payload — `ChangeSceneSunEnabled`. One of the scene's independently-settable fields (no bundled `update-scene-sun` facet — the play app's `☀️scene` commands set each field separately).

use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeSceneSunEnabled {
    pub new_enabled: bool,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ChangeSceneSunEnabled {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "scene-sun-enabled", kind: "change-scene-sun-enabled", record: "ChangedSceneSunEnabled" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("{} sun", if self.new_enabled { "Enable" } else { "Disable" })
    }
}
