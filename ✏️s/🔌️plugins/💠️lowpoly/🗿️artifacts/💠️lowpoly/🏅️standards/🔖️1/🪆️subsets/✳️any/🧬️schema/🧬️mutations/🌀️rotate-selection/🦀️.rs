//! 🌀️ `rotate-selection` — the gumball's turn of the current selection stated as its INTENT: the object, the vertices it
//! names (every vertex of the object when empty), the pivot, the axis and the angle in radians. The geometry is derived on
//! every application from the object's persisted mesh, so the turn replays onto whatever mesh the object holds by then and
//! every parameter stays editable in history.
//! (design `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5, §17.6).

use semio_s_artifact_lowpoly_lowpoly_model_diff::LowpolySelectionMotion;
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
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

    /// 🧪️ The payload's own invariant, independent of any document: every vertex named once, a finite pivot, axis and
    /// angle, and an axis other than the zero vector (`x-semio-invariant` `axis-nonzero`).
    pub fn invariant_violation(&self) -> Option<String> {
        if let Some(reason) = semio_s_artifact_lowpoly_lowpoly_model_diff::lowpoly_selection_vertex_violation(&self.vertex_ids) {
            return Some(reason);
        }
        if !self.pivot.iter().chain(&self.axis).chain([&self.angle]).all(|value| value.is_finite()) {
            return Some("The pivot, axis and angle must be finite.".into());
        }
        self.axis.iter().all(|value| *value == 0.0).then(|| "The rotation axis must not be the zero vector.".into())
    }
}

impl<Op> protocol::MutationKind<LowpolySnapshot, Op> for RotateSelection
where
    Op: protocol::Mutation<LowpolySnapshot, Diff = semio_s_artifact_lowpoly_lowpoly_model_diff::LowpolyDiff> + From<crate::set_vertex_positions::SetVertexPositions>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rotate", entity: "selection", kind: "rotate-selection", record: "RotatedSelection" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        self.motion().label(&self.object_id, self.vertex_ids.len())
    }
    fn target(&self) -> Vec<String> {
        vec![self.object_id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<LowpolySnapshot, Op> for RotateSelection
where
    Op: protocol::ColdMutation<LowpolySnapshot, ReferenceDiff = semio_s_artifact_lowpoly_lowpoly_model_diff::LowpolyDiff> + From<crate::set_vertex_positions::SetVertexPositions>,
{
    fn diff(&self, base: &LowpolySnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<LowpolySnapshot>>::ReferenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &LowpolySnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload

//#region 🧪️Laws
//#endregion 🧪️Laws
