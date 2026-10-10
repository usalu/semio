//! 🗳️ `set-area-scheme` payload. Sets any of an area scheme's name, measure, counted usages and counted zones; a list replaces the whole rule list and an empty list counts everything.

use crate::{AreaMeasure, AreaSchemePatch, ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetAreaScheme {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub measure: Option<AreaMeasure>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub usages: Option<Vec<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub zones: Option<Vec<String>>,
}

impl SetAreaScheme {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> AreaSchemePatch {
        AreaSchemePatch { name: self.name.clone(), measure: self.measure, usages: self.usages.clone(), zones: self.zones.clone() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: AreaSchemePatch) -> Self {
        Self { id, name: patch.name, measure: patch.measure, usages: patch.usages, zones: patch.zones }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetAreaScheme {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "area-scheme", kind: "set-area-scheme", record: "SetAreaScheme" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Edit area scheme \"{}\"", self.id), &format!("Flächenschema \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
