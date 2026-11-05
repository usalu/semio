//! ➕️ `insert-plated-action` mutation leaf: inserts one Plated panel action element into the addressed row.

use semio_s_artifact_norm_en1993_model::{ForceAction, En1993Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertPlatedAction {
    pub id: String,
    pub index: Option<usize>,
    pub element: ForceAction,
}

impl<Op> protocol::MutationKind<En1993Snapshot, Op> for InsertPlatedAction where Op: protocol::Mutation<En1993Snapshot, Diff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::remove_plated_action::RemovePlatedAction> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "plated-action", kind: "insert-plated-action", record: "InsertedPlatedAction" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native(&format!("Insert plated panel action into {}", self.id), &format!("Plattenfeld-Einwirkung in {} einfügen", self.id))
    }
    fn target(&self) -> Vec<String> { vec![self.id.clone(), self.element.id.clone()] }
}

impl<Op> protocol::ColdMutationKind<En1993Snapshot, Op> for InsertPlatedAction where Op: protocol::ColdMutation<En1993Snapshot, ReferenceDiff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::remove_plated_action::RemovePlatedAction> {
    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1993Snapshot>>::ReferenceDiff> { crate::insert_plated_action::diff::diff(self, base) }
    fn inverse(&self, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> { crate::insert_plated_action::inverse::inverse(self, base) }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
