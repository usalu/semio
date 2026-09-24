//! 🧩 WFC 2D mutation — `CreateSlot`: brings a new id-keyed slot into existence at its CANONICAL
//! ascending-id position, never at the end, so `delete-slot`'s inverse restores the same index.

use crate::diff::Wfc2dDiff;
use crate::mutations::Wfc2dMutation;
use crate::schema::snapshot::Wfc2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use crate::schema::snapshot::Wfc2dSlot;

//#region 🔖️CreateSlot
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateSlot {
    pub slot: Wfc2dSlot,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_slot(slot: Wfc2dSlot) -> Wfc2dMutation {
    Wfc2dMutation::CreateSlot(CreateSlot { slot })
}

impl MutationKind<Wfc2dSnapshot, Wfc2dMutation> for CreateSlot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "slot", kind: "create-slot", record: "CreatedSlot" };

    fn diff(&self, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Create Slot", "Slot erstellen")
    }
}
//#endregion 🔖️CreateSlot
