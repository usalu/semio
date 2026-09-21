//! 📌️ `wfc3d` mutation — `PinSlot`: binds one slot to a tile the solve must keep. A pin is a domain
//! restriction the solver reads, never an assignment the solver writes.

use crate::diff::Wfc3dDiff;
use crate::mutations::Wfc3dMutation;
use crate::schema::snapshot::Wfc3dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️PinSlot
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct PinSlot {
    pub id: String,
    pub tile_id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn pin_slot(id: String, tile_id: String) -> Wfc3dMutation {
    Wfc3dMutation::PinSlot(PinSlot { id, tile_id })
}

impl MutationKind<Wfc3dSnapshot, Wfc3dMutation> for PinSlot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "fix", entity: "slot", kind: "pin-slot", record: "Fixed" };

    fn diff(&self, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc3dSnapshot) -> Vec<Wfc3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Pin slot \"{}\" to tile \"{}\"", self.id, self.tile_id), &format!("Platz \"{}\" an Kachel \"{}\" anheften", self.id, self.tile_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️PinSlot
