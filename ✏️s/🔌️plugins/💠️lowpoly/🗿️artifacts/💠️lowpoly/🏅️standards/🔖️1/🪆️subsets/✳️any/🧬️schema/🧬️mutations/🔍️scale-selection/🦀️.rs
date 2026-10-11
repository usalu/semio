//! 🔍️ `scale-selection` — the gumball's scaling of the current selection stated as its INTENT: the object, the vertices it
//! names (every vertex of the object when empty), the pivot and one factor per axis. The geometry is derived on every
//! application from the object's persisted mesh, so the scaling replays onto whatever mesh the object holds by then and
//! every factor stays editable in history.
//! (design `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5, §17.6).

use crate::mutations::LowpolySelectionMotion;
use crate::{LowpolyMutation, LowpolySnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ScaleSelection {
    pub object_id: String,
    pub vertex_ids: Vec<u32>,
    pub pivot: [f32; 3],
    pub factor: [f32; 3],
}

impl ScaleSelection {
    /// 🧲️ The motion this leaf applies to the vertices it names.
    pub fn motion(&self) -> LowpolySelectionMotion {
        LowpolySelectionMotion::Stretch { pivot: self.pivot, factor: self.factor }
    }

    /// 🧪️ The payload's own invariant, independent of any document: every vertex named once, a finite pivot and every
    /// factor positive and finite.
    pub fn invariant_violation(&self) -> Option<String> {
        let clean = self.pivot.iter().all(|value| value.is_finite()) && self.factor.iter().all(|value| value.is_finite() && *value > 0.0);
        crate::mutations::lowpoly_selection_vertex_violation(&self.vertex_ids).or_else(|| (!clean).then(|| "The pivot must be finite and every factor positive and finite.".into()))
    }
}

impl protocol::MutationKind<LowpolySnapshot, LowpolyMutation> for ScaleSelection {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "scale", entity: "selection", kind: "scale-selection", record: "ScaledSelection" };

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
