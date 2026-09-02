//! ☀️ Shooting mutation payload — `ChangeSceneSunEnabled`. One of the scene's independently-settable fields (no bundled `update-scene-sun` facet — the play app's `☀️scene` commands set each field separately).

use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeSceneSunEnabled {
    pub new_enabled: bool,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ChangeSceneSunEnabled {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "scene-sun-enabled", kind: "change-scene-sun-enabled", record: "ChangedSceneSunEnabled" };
    async fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("{} sun", if self.new_enabled { "Enable" } else { "Disable" })
    }
}
