//! ⭕ `insert-bar-layer` mutation leaf: inserts one bar layer row.

use semio_s_artifact_norm_en1992_model::En1992Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertBarLayer {
    pub member_id: String,
    pub index: usize,
    pub bar_layer: semio_s_artifact_norm_en1992_model::BarLayer,
}

impl<Op> protocol::MutationKind<En1992Snapshot, Op> for InsertBarLayer where Op: protocol::Mutation<En1992Snapshot, Diff = semio_s_artifact_norm_en1992_diff::En1992Diff> + From<crate::remove_bar_layer::RemoveBarLayer> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "insert",
        entity: "bar-layer",
        kind: "insert-bar-layer",
        record: "InsertedBarLayer",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native(&format!("Insert bar layer {}", self.bar_layer.id), &format!("Bewehrungslage {} einfügen", self.bar_layer.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.member_id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<En1992Snapshot, Op> for InsertBarLayer where Op: protocol::ColdMutation<En1992Snapshot, ReferenceDiff = semio_s_artifact_norm_en1992_diff::En1992Diff> + From<crate::remove_bar_layer::RemoveBarLayer> {
    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<semio_s_artifact_norm_en1992_diff::En1992Diff> {
        crate::insert_bar_layer::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::insert_bar_layer::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
