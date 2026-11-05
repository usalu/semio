//! 🏋️ `insert-anchor-action` mutation leaf: inserts one anchor action row.

use semio_s_artifact_norm_en1992_model::En1992Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertAnchorAction {
    pub anchor_id: String,
    pub index: usize,
    pub anchor_action: semio_s_artifact_norm_en1992_model::LoadCaseActions,
}

impl<Op> protocol::MutationKind<En1992Snapshot, Op> for InsertAnchorAction where Op: protocol::Mutation<En1992Snapshot, Diff = semio_s_artifact_norm_en1992_diff::En1992Diff> + From<crate::remove_anchor_action::RemoveAnchorAction> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "insert",
        entity: "anchor-action",
        kind: "insert-anchor-action",
        record: "InsertedAnchorAction",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native(&format!("Insert anchor action {}", self.anchor_action.id), &format!("Dübeleinwirkung {} einfügen", self.anchor_action.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.anchor_id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<En1992Snapshot, Op> for InsertAnchorAction where Op: protocol::ColdMutation<En1992Snapshot, ReferenceDiff = semio_s_artifact_norm_en1992_diff::En1992Diff> + From<crate::remove_anchor_action::RemoveAnchorAction> {
    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<semio_s_artifact_norm_en1992_diff::En1992Diff> {
        crate::insert_anchor_action::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::insert_anchor_action::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
