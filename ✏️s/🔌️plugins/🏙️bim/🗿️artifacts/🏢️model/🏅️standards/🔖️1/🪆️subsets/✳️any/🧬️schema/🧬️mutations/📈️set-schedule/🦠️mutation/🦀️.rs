//! 📈️ `set-schedule` payload. Sets any of a schedule's name, category, columns, sort, filter, grouping, itemization, storey scope and phase scope; absent fields stay untouched and a list replaces the whole list.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::documentation::SchedulePatch;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{Phase, ScheduleCategory, ScheduleColumn, ScheduleFilter, ScheduleGroup, ScheduleSort};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSchedule {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<ScheduleCategory>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<ScheduleColumn>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<ScheduleSort>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<Vec<ScheduleFilter>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<Vec<ScheduleGroup>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub itemize: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub storeys: Option<Vec<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub phases: Option<Vec<Phase>>,
}

impl SetSchedule {
    /// \u{1fa79} The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> SchedulePatch {
        SchedulePatch { name: self.name.clone(), category: self.category, columns: self.columns.clone(), sort: self.sort.clone(), filter: self.filter.clone(), group: self.group.clone(), itemize: self.itemize, storeys: self.storeys.clone(), phases: self.phases.clone() }
    }

    /// \u{1f9e9} The payload that provides exactly the fields \u00B6patch\u00B6 names.
    pub fn from_patch(id: String, patch: SchedulePatch) -> Self {
        Self { id, name: patch.name, category: patch.category, columns: patch.columns, sort: patch.sort, filter: patch.filter, group: patch.group, itemize: patch.itemize, storeys: patch.storeys, phases: patch.phases }
    }
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetSchedule
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetSchedule>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "schedule", kind: "set-schedule", record: "SetSchedule" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_schedule::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_schedule::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Edit schedule \"{}\"", self.id), &format!("Bauteilliste \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
