//! 📐 Shooting mutation payload — `ChangeShotHeight`. Sets a shot's render `height`.

use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeShotHeight {
    pub id: String,
    pub new_height: u32,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ChangeShotHeight {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "shot-height", kind: "change-shot-height", record: "ChangedShotHeight" };
    async fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change shot \"{}\" height", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
