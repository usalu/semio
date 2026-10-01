//! 🌀️ `rotate-selection` — the gumball's turn of the current selection stated as its INTENT: the object, the vertices it
//! names (every vertex of the object when empty), the pivot, the axis and the angle in radians. The geometry is derived on
//! every application from the object's persisted mesh, so the turn replays onto whatever mesh the object holds by then and
//! every parameter stays editable in history.
//! (design `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5, §17.6).

use crate::mutations::LowpolySelectionMotion;
use crate::{LowpolyMutation, LowpolySnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RotateSelection {
    pub object_id: String,
    pub vertex_ids: Vec<u32>,
    pub pivot: [f32; 3],
    pub axis: [f32; 3],
    pub angle: f32,
}

impl RotateSelection {
    /// 🧲️ The motion this leaf applies to the vertices it names.
    pub fn motion(&self) -> LowpolySelectionMotion {
        LowpolySelectionMotion::Turn { pivot: self.pivot, axis: self.axis, angle: self.angle }
    }
}

impl protocol::MutationKind<LowpolySnapshot, LowpolyMutation> for RotateSelection {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rotate", entity: "selection", kind: "rotate-selection", record: "RotatedSelection" };

    fn diff(&self, base: &LowpolySnapshot) -> protocol::MutationOutcome<<LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &LowpolySnapshot) -> Vec<LowpolyMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        self.motion().label(&self.object_id, self.vertex_ids.len())
    }
    fn target(&self) -> Vec<String> {
        vec![self.object_id.clone()]
    }
}
//#endregion 🔖️Payload

//#region 🧪️Laws
#[cfg(test)]
pub use crate::mutations::laws;
//#endregion 🧪️Laws
