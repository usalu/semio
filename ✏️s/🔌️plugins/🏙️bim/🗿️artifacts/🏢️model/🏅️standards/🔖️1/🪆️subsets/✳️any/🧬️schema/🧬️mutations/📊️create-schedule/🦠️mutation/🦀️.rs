//! 📊️ `create-schedule` payload. Brings a new schedule into the model: the authored definition (category, columns, sort, filter, grouping, scope) of a table whose rows and totals are inferred.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, Schedule};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateSchedule {
    pub id: String,
    pub schedule: Schedule,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateSchedule {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "schedule", kind: "create-schedule", record: "CreateSchedule" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create schedule \"{}\"", self.schedule.name), &format!("Bauteilliste \"{}\" anlegen", self.schedule.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
