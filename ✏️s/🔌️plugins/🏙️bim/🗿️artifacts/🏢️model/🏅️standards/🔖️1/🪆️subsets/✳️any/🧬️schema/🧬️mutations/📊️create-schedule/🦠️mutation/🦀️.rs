//! 📊️ `create-schedule` payload. Brings a new schedule into the model: the authored definition (category, columns, sort, filter, grouping, scope) of a table whose rows and totals are inferred.

use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::{ModelSnapshot, Schedule}};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateSchedule {
    pub id: String,
    pub schedule: Schedule,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateSchedule
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateSchedule> + From<crate::standards::v1::subsets::any::schema::mutations::delete_schedule::DeleteSchedule>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "schedule", kind: "create-schedule", record: "CreateSchedule" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_schedule::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_schedule::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create schedule \"{}\"", self.schedule.name), &format!("Bauteilliste \"{}\" anlegen", self.schedule.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
