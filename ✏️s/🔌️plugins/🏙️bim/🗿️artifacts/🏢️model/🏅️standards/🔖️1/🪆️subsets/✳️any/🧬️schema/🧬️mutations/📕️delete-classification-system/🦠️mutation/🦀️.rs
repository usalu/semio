//! 📕️ `delete-classification-system` payload. Removes a classification system with every element and type classification that names it; the inverse restores the system and each classification.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteClassificationSystem {
    pub id: String,
}

impl MutationKind<ModelSnapshot, ModelMutation> for DeleteClassificationSystem {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "classification-system", kind: "delete-classification-system", record: "DeleteClassificationSystem" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete classification system \"{}\"", self.id), &format!("Klassifikationssystem \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
