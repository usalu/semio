//! 🧹 Shooting mutation payload — `DeleteSavedCamera`. Removes a saved camera by id; inverse recreates it.

use crate::ShootingSnapshot;
use crate::diff::ShootingDiff;
use crate::mutations::ShootingMutation;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteSavedCamera {
    pub id: String,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for DeleteSavedCamera {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "saved-camera", kind: "delete-saved-camera", record: "DeletedSavedCamera" };
    fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete saved camera \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
