//! ↔ WFC 2D mutation — `MoveSlot`: the settled end of a node drag in the `wfc-graph` window. One
//! mutation per GESTURE, never per pointer tick — mid-drag frames ride the window transient.

use crate::diff::Wfc2dDiff;
use crate::mutations::Wfc2dMutation;
use crate::schema::snapshot::Wfc2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️MoveSlot
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct MoveSlot {
    pub id: String,
    pub x: f64,
    pub y: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn move_slot(id: String, x: f64, y: f64) -> Wfc2dMutation {
    Wfc2dMutation::MoveSlot(MoveSlot { id, x, y })
}

impl MutationKind<Wfc2dSnapshot, Wfc2dMutation> for MoveSlot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "slot", kind: "move-slot", record: "MovedSlot" };

    fn diff(&self, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Move Slot", "Slot verschieben")
    }
}
//#endregion 🔖️MoveSlot
