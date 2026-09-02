//! 🧹 Shooting mutation payload — `DeleteSavedCamera`. Removes a saved camera by id; inverse recreates it.

use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteSavedCamera {
    pub id: String,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for DeleteSavedCamera {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "saved-camera", kind: "delete-saved-camera", record: "DeletedSavedCamera" };
    async fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete saved camera \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
