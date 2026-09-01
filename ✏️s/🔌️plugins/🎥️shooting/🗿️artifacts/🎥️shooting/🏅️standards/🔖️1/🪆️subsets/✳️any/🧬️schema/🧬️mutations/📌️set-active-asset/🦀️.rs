//! 📌 Shooting mutation payload — `SetActiveAsset`. A narrow addressed single-field setter on the document root (taxonomy's `set` verb).

use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetActiveAsset {
    pub asset_id: Option<String>,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for SetActiveAsset {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "active-asset", kind: "set-active-asset", record: "SetActiveAsset" };
    async fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        match &self.asset_id {
            Some(id) => format!("Set active asset to \"{id}\""),
            None => "Clear active asset".into(),
        }
    }
}
