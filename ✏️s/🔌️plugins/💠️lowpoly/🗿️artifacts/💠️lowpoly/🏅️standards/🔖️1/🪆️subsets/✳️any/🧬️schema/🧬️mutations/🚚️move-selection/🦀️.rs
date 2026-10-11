//! 🚚️ `move-selection` — the gumball's drag of the current selection stated as its INTENT: the object, the vertices it
//! names (every vertex of the object when empty) and one offset. The geometry is derived on every application from the
//! object's persisted mesh, so the drag replays onto whatever mesh the object holds by then and the offset stays editable
//! in history.
//! (design `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5, §17.6).

use crate::mutations::LowpolySelectionMotion;
use crate::{LowpolyMutation, LowpolySnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct MoveSelection {
    pub object_id: String,
    pub vertex_ids: Vec<u32>,
    pub offset: [f32; 3],
}

impl MoveSelection {
    /// 🧲️ The motion this leaf applies to the vertices it names.
    pub fn motion(&self) -> LowpolySelectionMotion {
        LowpolySelectionMotion::Offset(self.offset)
    }

    /// 🧪️ The payload's own invariant, independent of any document: every vertex named once and a finite offset.
    pub fn invariant_violation(&self) -> Option<String> {
        crate::mutations::lowpoly_selection_vertex_violation(&self.vertex_ids).or_else(|| (!self.offset.iter().all(|value| value.is_finite())).then(|| "The offset must be finite.".into()))
    }
}

impl protocol::MutationKind<LowpolySnapshot, LowpolyMutation> for MoveSelection {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "selection", kind: "move-selection", record: "MovedSelection" };

    fn diff(&self, base: &LowpolySnapshot) -> protocol::MutationOutcome<<LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &LowpolySnapshot) -> Result<Vec<LowpolyMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
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
