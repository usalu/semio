//! 🪑️ `set-space` payload. Sets any of a room's number, name, boundary and usage; absent fields stay untouched and the number stays unique within the storey.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, SpaceBoundary, SpacePatch};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSpace {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub boundary: Option<SpaceBoundary>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<String>,
}

impl SetSpace {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> SpacePatch {
        SpacePatch { number: self.number.clone(), name: self.name.clone(), boundary: self.boundary.clone(), usage: self.usage.clone(), ..Default::default() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: SpacePatch) -> Self {
        Self { id, number: patch.number, name: patch.name, boundary: patch.boundary, usage: patch.usage }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetSpace {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "space", kind: "set-space", record: "SetSpace" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Edit space \"{}\"", self.id), &format!("Raum \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
