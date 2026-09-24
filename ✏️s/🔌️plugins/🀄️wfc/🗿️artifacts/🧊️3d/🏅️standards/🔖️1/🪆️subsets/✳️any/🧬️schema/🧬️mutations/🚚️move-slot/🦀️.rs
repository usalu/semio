//! 🚚️ `wfc3d` mutation — `MoveSlot`: relocates one slot's box, by moving its MINIMUM corner; the
//! extent is `resize-slot`'s business. A drag gesture coalesces into exactly one of these on release;
//! mid-drag ticks never author a document edit.

use crate::diff::Wfc3dDiff;
use crate::mutations::Wfc3dMutation;
use crate::schema::snapshot::Wfc3dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️MoveSlot
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct MoveSlot {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn move_slot(id: String, x: f64, y: f64, z: f64) -> Wfc3dMutation {
    Wfc3dMutation::MoveSlot(MoveSlot { id, x, y, z })
}

impl MutationKind<Wfc3dSnapshot, Wfc3dMutation> for MoveSlot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "slot", kind: "move-slot", record: "MovedSlot" };

    fn diff(&self, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc3dSnapshot) -> Vec<Wfc3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Move slot \"{}\"", self.id), &format!("Slot \"{}\" verschieben", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️MoveSlot
