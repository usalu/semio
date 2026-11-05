//! 🧱️ `change-buckling-length-column-stiffness-b` mutation leaf: sets `column_stiffness_b` of one buckling_length block.

use semio_s_artifact_norm_en1993_diff::En1993Diff;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeBucklingLengthColumnStiffnessB {
    pub id: String,
    pub new_column_stiffness_b: f64,
}

impl<Op> protocol::MutationKind<En1993Snapshot, Op> for ChangeBucklingLengthColumnStiffnessB where Op: protocol::Mutation<En1993Snapshot, Diff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::change_buckling_length_column_stiffness_b::ChangeBucklingLengthColumnStiffnessB> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "buckling-length-column-stiffness-b",
        kind: "change-buckling-length-column-stiffness-b",
        record: "ChangedBucklingLengthColumnStiffnessB",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change buckling length: Node B: column stiffness sum", "Knicklänge: Knoten B: Stützensteifigkeit ändern")
    }
}

impl<Op> protocol::ColdMutationKind<En1993Snapshot, Op> for ChangeBucklingLengthColumnStiffnessB where Op: protocol::ColdMutation<En1993Snapshot, ReferenceDiff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::change_buckling_length_column_stiffness_b::ChangeBucklingLengthColumnStiffnessB> {
    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
        crate::change_buckling_length_column_stiffness_b::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_buckling_length_column_stiffness_b::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
