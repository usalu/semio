//! 🔏️ `set-classification-system` payload. Sets any of a classification system's name, edition, source and entry table (an assigned null removes the source); absent fields stay untouched and the entry table replaces the whole table.

use crate::{Assigned, ClassificationItem, ClassificationSystemPatch, ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetClassificationSystem {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<Assigned<Option<String>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<ClassificationItem>>,
}

impl SetClassificationSystem {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> ClassificationSystemPatch {
        ClassificationSystemPatch { name: self.name.clone(), edition: self.edition.clone(), source: self.source.clone(), entries: self.entries.clone() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: ClassificationSystemPatch) -> Self {
        Self { id, name: patch.name, edition: patch.edition, source: patch.source, entries: patch.entries }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetClassificationSystem {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "classification-system", kind: "set-classification-system", record: "SetClassificationSystem" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Edit classification system \"{}\"", self.id), &format!("Klassifikationssystem \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
