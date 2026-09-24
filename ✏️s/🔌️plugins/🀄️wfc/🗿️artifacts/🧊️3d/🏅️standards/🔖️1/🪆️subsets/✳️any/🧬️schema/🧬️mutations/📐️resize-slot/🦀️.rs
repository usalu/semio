//! 📐️ `wfc3d` mutation — `ResizeSlot`: sets one slot's box extent. The preview scales the assigned
//! tile's unit-box media into exactly this extent, so a zero or negative dimension is refused.

use crate::diff::Wfc3dDiff;
use crate::mutations::Wfc3dMutation;
use crate::schema::snapshot::Wfc3dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️ResizeSlot
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ResizeSlot {
    pub id: String,
    pub width: f64,
    pub height: f64,
    pub depth: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn resize_slot(id: String, width: f64, height: f64, depth: f64) -> Wfc3dMutation {
    Wfc3dMutation::ResizeSlot(ResizeSlot { id, width, height, depth })
}

impl MutationKind<Wfc3dSnapshot, Wfc3dMutation> for ResizeSlot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "resize", entity: "slot", kind: "resize-slot", record: "ResizedSlot" };

    fn diff(&self, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc3dSnapshot) -> Vec<Wfc3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Resize slot \"{}\"", self.id), &format!("Größe von Slot \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️ResizeSlot
