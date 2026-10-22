//! 🏷️ `rename-step` payload — changes an id-keyed [`ProcessStep`]'s `label` (its identity/display
//! field — the step's `id` itself is immutable).

use semio_s_artifact_process_process3d_model_diff::Process3dDiff;
use semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️RenameStep
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RenameStep {
    pub id: String,
    pub new_label: String,
}

impl<Op> protocol::MutationKind<Process3dSnapshot, Op> for RenameStep
where
    Op: protocol::Mutation<Process3dSnapshot, Diff = semio_s_artifact_process_process3d_model_diff::Process3dDiff> + From<crate::rename_step::RenameStep>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "step", kind: "rename-step", record: "RenamedStep" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rename step to \"{}\"", self.new_label), &format!("Schritt in \"{}\" umbenennen", self.new_label))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Process3dSnapshot, Op> for RenameStep
where
    Op: protocol::ColdMutation<Process3dSnapshot, ReferenceDiff = semio_s_artifact_process_process3d_model_diff::Process3dDiff> + From<crate::rename_step::RenameStep>,
{
    fn diff(&self, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Process3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️RenameStep
