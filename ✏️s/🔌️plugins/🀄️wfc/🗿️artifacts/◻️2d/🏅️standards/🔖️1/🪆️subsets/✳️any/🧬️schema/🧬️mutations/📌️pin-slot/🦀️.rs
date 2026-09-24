//! 📌 WFC 2D mutation — `PinSlot`: hard-assigns one tile to one slot. A pin is a DOMAIN RESTRICTION
//! the solve must respect, never a solved assignment written back into the document.

use crate::diff::Wfc2dDiff;
use crate::mutations::Wfc2dMutation;
use crate::schema::snapshot::Wfc2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️PinSlot
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct PinSlot {
    pub id: String,
    pub tile_id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn pin_slot(id: String, tile_id: String) -> Wfc2dMutation {
    Wfc2dMutation::PinSlot(PinSlot { id, tile_id })
}

impl MutationKind<Wfc2dSnapshot, Wfc2dMutation> for PinSlot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "fix", entity: "slot", kind: "pin-slot", record: "FixedSlot" };

    fn diff(&self, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Pin Slot", "Slot fixieren")
    }
}
//#endregion 🔖️PinSlot
