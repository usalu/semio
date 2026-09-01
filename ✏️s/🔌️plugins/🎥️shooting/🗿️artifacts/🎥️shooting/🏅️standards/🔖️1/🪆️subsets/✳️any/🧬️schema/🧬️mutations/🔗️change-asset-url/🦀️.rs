//! 🔗 Shooting mutation payload — `ChangeAssetUrl`. Sets an asset's mesh `url`.

use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeAssetUrl {
    pub id: String,
    pub new_url: String,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ChangeAssetUrl {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "asset-url", kind: "change-asset-url", record: "ChangedAssetUrl" };
    async fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change asset \"{}\" url", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
