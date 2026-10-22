//! 📏️ `change-beam-span-m` mutation leaf.

use semio_s_artifact_norm_en1994_model::En1994Snapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeBeamSpanM {
    pub index: usize,
    pub new_span_m: f64,
}

impl<Op> protocol::MutationKind<En1994Snapshot, Op> for ChangeBeamSpanM where Op: protocol::Mutation<En1994Snapshot, Diff = semio_s_artifact_norm_en1994_diff::En1994Diff> + From<crate::change_beam_span_m::ChangeBeamSpanM> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "beam-span-m", kind: "change-beam-span-m", record: "ChangedBeamSpanM" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change beam span", "Stützweite des Trägers ändern")
    }
}

impl<Op> protocol::ColdMutationKind<En1994Snapshot, Op> for ChangeBeamSpanM where Op: protocol::ColdMutation<En1994Snapshot, ReferenceDiff = semio_s_artifact_norm_en1994_diff::En1994Diff> + From<crate::change_beam_span_m::ChangeBeamSpanM> {
    fn diff(&self, base: &En1994Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1994Snapshot>>::ReferenceDiff> {
        crate::change_beam_span_m::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_beam_span_m::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
