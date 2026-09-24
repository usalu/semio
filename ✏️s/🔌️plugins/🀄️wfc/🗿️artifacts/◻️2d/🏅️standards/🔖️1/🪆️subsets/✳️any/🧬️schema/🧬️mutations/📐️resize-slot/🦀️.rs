//! 📐 WFC 2D mutation — `ResizeSlot`: changes the rectangle a slot occupies. The tile media is drawn
//! SCALED into that rectangle, so this is also how a tile's on-canvas size is authored.

use crate::diff::Wfc2dDiff;
use crate::mutations::Wfc2dMutation;
use crate::schema::snapshot::Wfc2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ResizeSlot
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ResizeSlot {
    pub id: String,
    pub width: f64,
    pub height: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn resize_slot(id: String, width: f64, height: f64) -> Wfc2dMutation {
    Wfc2dMutation::ResizeSlot(ResizeSlot { id, width, height })
}

impl MutationKind<Wfc2dSnapshot, Wfc2dMutation> for ResizeSlot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "resize", entity: "slot", kind: "resize-slot", record: "ResizedSlot" };

    fn diff(&self, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Resize Slot", "Größe des Slots ändern")
    }
}
//#endregion 🔖️ResizeSlot
