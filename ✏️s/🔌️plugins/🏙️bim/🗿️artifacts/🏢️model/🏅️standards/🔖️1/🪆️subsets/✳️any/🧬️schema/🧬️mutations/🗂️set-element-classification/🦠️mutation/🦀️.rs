//! 🗂️ `set-element-classification` payload. Sets the classification reference (system, code, title) of an element; an element without one gets it created, an existing one is patched field by field.

use crate::{Classification, ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetElementClassification {
    pub id: String,
    pub classification: Classification,
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetElementClassification {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "element-classification", kind: "set-element-classification", record: "SetElementClassification" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Classify element \"{}\" as {} {}", self.id, self.classification.system, self.classification.code), &format!("Element \"{}\" als {} {} klassifizieren", self.id, self.classification.system, self.classification.code))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
