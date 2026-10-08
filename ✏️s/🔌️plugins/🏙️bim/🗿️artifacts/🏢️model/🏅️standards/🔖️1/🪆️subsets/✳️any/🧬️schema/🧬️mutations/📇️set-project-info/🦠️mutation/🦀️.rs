//! 📇️ `set-project-info` payload. Patches the project metadata (name, description, author, organization, phase names): exactly the provided fields change, an empty patch is a no-op.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, ProjectPatch};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetProjectInfo {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub phase_names: Option<Vec<String>>,
}

impl SetProjectInfo {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> ProjectPatch {
        ProjectPatch { name: self.name.clone(), description: self.description.clone(), author: self.author.clone(), organization: self.organization.clone(), phase_names: self.phase_names.clone() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(patch: ProjectPatch) -> Self {
        Self { name: patch.name, description: patch.description, author: patch.author, organization: patch.organization, phase_names: patch.phase_names }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetProjectInfo {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "project", kind: "set-project-info", record: "SetProjectInfo" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&String::from("Update the project info"), &String::from("Projektinformationen ändern"))
    }
    fn target(&self) -> Vec<String> {
        vec!["project".to_string()]
    }
}
