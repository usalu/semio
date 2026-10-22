//! 📉️ `delete-schedule` payload. Removes a schedule; its table is inferred, so nothing else depends on it.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteSchedule {
    pub id: String,
}

impl<Op> MutationKind<ModelSnapshot, Op> for DeleteSchedule
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<DeleteSchedule> + From<crate::standards::v1::subsets::any::schema::mutations::create_schedule::CreateSchedule>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "schedule", kind: "delete-schedule", record: "DeleteSchedule" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete schedule \"{}\"", self.id), &format!("Bauteilliste \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for DeleteSchedule
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<DeleteSchedule> + From<crate::standards::v1::subsets::any::schema::mutations::create_schedule::CreateSchedule>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::delete_schedule::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::delete_schedule::inverse::inverse(self, base))
    }
}
