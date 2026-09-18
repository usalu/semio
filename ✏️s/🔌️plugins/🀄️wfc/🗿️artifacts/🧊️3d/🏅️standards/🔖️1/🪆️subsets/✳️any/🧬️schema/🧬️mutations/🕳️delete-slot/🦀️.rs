//! 🗑️ `wfc3d` mutation — `DeleteSlot`: removes an id-addressed slot and cascades to every incident
//! adjacency edge (a slot cannot be referenced by a dangling edge).

use crate::diff::Wfc3dDiff;
use crate::mutations::Wfc3dMutation;
use crate::schema::snapshot::Wfc3dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️DeleteSlot
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteSlot {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_slot(id: String) -> Wfc3dMutation {
    Wfc3dMutation::DeleteSlot(DeleteSlot { id })
}

impl MutationKind<Wfc3dSnapshot, Wfc3dMutation> for DeleteSlot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "slot", kind: "delete-slot", record: "DeletedSlot" };

    fn diff(&self, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc3dSnapshot) -> Vec<Wfc3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete slot \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️DeleteSlot
