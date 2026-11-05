//! 🧱️ `change-buckling-length-beam-stiffness-b` mutation leaf: sets `beam_stiffness_b` of one buckling_length block.

use semio_s_artifact_norm_en1993_diff::En1993Diff;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeBucklingLengthBeamStiffnessB {
    pub id: String,
    pub new_beam_stiffness_b: f64,
}

impl<Op> protocol::MutationKind<En1993Snapshot, Op> for ChangeBucklingLengthBeamStiffnessB where Op: protocol::Mutation<En1993Snapshot, Diff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::change_buckling_length_beam_stiffness_b::ChangeBucklingLengthBeamStiffnessB> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "buckling-length-beam-stiffness-b",
        kind: "change-buckling-length-beam-stiffness-b",
        record: "ChangedBucklingLengthBeamStiffnessB",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change buckling length: Node B: beam stiffness sum", "Knicklänge: Knoten B: Riegelsteifigkeit ändern")
    }
}

impl<Op> protocol::ColdMutationKind<En1993Snapshot, Op> for ChangeBucklingLengthBeamStiffnessB where Op: protocol::ColdMutation<En1993Snapshot, ReferenceDiff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::change_buckling_length_beam_stiffness_b::ChangeBucklingLengthBeamStiffnessB> {
    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
        crate::change_buckling_length_beam_stiffness_b::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_buckling_length_beam_stiffness_b::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
