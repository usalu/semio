//! 📈️ `set-schedule` payload. Sets any of a schedule's name, category, columns, sort, filter, grouping, itemization, storey scope and phase scope; absent fields stay untouched and a list replaces the whole list.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, Phase, SchedulePatch, ScheduleCategory, ScheduleColumn, ScheduleFilter, ScheduleGroup, ScheduleSort};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
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

impl MutationKind<ModelSnapshot, ModelMutation> for SetSchedule {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "schedule", kind: "set-schedule", record: "SetSchedule" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Edit schedule \"{}\"", self.id), &format!("Bauteilliste \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
