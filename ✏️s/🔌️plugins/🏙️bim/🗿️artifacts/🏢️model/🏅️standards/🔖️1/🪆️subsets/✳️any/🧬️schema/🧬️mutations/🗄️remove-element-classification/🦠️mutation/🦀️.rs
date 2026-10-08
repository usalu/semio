//! 🗄️ `remove-element-classification` payload. Removes the classification reference of an element.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveElementClassification {
    pub id: String,
}

impl MutationKind<ModelSnapshot, ModelMutation> for RemoveElementClassification {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "element-classification", kind: "remove-element-classification", record: "RemoveElementClassification" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove classification of \"{}\"", self.id), &format!("Klassifizierung von \"{}\" entfernen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
