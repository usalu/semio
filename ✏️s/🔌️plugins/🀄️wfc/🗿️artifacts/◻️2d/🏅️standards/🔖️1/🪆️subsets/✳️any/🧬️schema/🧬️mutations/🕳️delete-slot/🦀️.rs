//! 🕳 WFC 2D mutation — `DeleteSlot`: removes a slot AND cascades every edge incident to it, so the
//! adjacency graph never keeps a dangling endpoint.

use crate::diff::Wfc2dDiff;
use crate::mutations::Wfc2dMutation;
use crate::schema::snapshot::Wfc2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️DeleteSlot
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DeleteSlot {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_slot(id: String) -> Wfc2dMutation {
    Wfc2dMutation::DeleteSlot(DeleteSlot { id })
}

impl MutationKind<Wfc2dSnapshot, Wfc2dMutation> for DeleteSlot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "slot", kind: "delete-slot", record: "DeletedSlot" };

    fn diff(&self, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Delete Slot", "Slot löschen")
    }
}
//#endregion 🔖️DeleteSlot
