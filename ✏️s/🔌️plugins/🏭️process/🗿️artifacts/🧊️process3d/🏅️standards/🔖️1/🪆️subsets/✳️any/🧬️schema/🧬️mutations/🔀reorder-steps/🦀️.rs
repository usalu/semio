//! 🔀 `reorder-steps` payload — repositions one id-keyed [`ProcessStep`] within the document's
//! ordered timeline (order is user-meaningful here, unlike the unordered `machines` collection).

use semio_s_artifact_process_process3d_model_diff::Process3dDiff;
use semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ReorderSteps
/// 🔀 `to_index` is FINAL-state, clamped to the list length — mirrors the pre-migration
/// generic-move semantics the old collection-op engine implemented.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ReorderSteps {
    pub id: String,
    pub to_index: usize,
}

impl<Op> protocol::MutationKind<Process3dSnapshot, Op> for ReorderSteps
where
    Op: protocol::Mutation<Process3dSnapshot, Diff = semio_s_artifact_process_process3d_model_diff::Process3dDiff> + From<crate::reorder_steps::ReorderSteps>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "step", kind: "reorder-steps", record: "ReorderedSteps" };

    fn diff(&self, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Process3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Reorder step \"{}\"", self.id), &format!("Reihenfolge von Schritt \"{}\" ändern", self.id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️ReorderSteps
