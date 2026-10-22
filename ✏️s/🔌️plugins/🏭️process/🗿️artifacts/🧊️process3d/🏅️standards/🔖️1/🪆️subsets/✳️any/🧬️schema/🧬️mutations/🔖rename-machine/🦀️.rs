//! 🏷️ `rename-machine` payload — changes an id-keyed [`WorkshopMachine`]'s `label`.

use semio_s_artifact_process_process3d_model_diff::Process3dDiff;
use semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️RenameMachine
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RenameMachine {
    pub id: String,
    pub new_label: String,
}

impl<Op> protocol::MutationKind<Process3dSnapshot, Op> for RenameMachine
where
    Op: protocol::Mutation<Process3dSnapshot, Diff = semio_s_artifact_process_process3d_model_diff::Process3dDiff> + From<crate::rename_machine::RenameMachine>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "machine", kind: "rename-machine", record: "RenamedMachine" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rename machine to \"{}\"", self.new_label), &format!("Maschine in \"{}\" umbenennen", self.new_label))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Process3dSnapshot, Op> for RenameMachine
where
    Op: protocol::ColdMutation<Process3dSnapshot, ReferenceDiff = semio_s_artifact_process_process3d_model_diff::Process3dDiff> + From<crate::rename_machine::RenameMachine>,
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
//#endregion 🔖️RenameMachine
