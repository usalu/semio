//! ➖️ `remove-joint-action` mutation leaf: removes the Joint action element at an index from the addressed row.

use semio_s_artifact_norm_en1993_model::En1993Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct RemoveJointAction {
    pub id: String,
    pub index: usize,
}

impl<Op> protocol::MutationKind<En1993Snapshot, Op> for RemoveJointAction where Op: protocol::Mutation<En1993Snapshot, Diff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::insert_joint_action::InsertJointAction> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "joint-action", kind: "remove-joint-action", record: "RemovedJointAction" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native(&format!("Remove joint action #{} of {}", self.index, self.id), &format!("Anschlusseinwirkung #{} von {} entfernen", self.index, self.id))
    }
    fn target(&self) -> Vec<String> { vec![self.id.clone(), self.index.to_string()] }
}

impl<Op> protocol::ColdMutationKind<En1993Snapshot, Op> for RemoveJointAction where Op: protocol::ColdMutation<En1993Snapshot, ReferenceDiff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::insert_joint_action::InsertJointAction> {
    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1993Snapshot>>::ReferenceDiff> { crate::remove_joint_action::diff::diff(self, base) }
    fn inverse(&self, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> { crate::remove_joint_action::inverse::inverse(self, base) }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
