//! 🫧️ `remove-element-property` payload. Removes one property of an element; removing its last property deletes the element's property entry.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveElementProperty {
    pub id: String,
    pub pset: String,
    pub property: String,
}

impl MutationKind<ModelSnapshot, ModelMutation> for RemoveElementProperty {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "element-property", kind: "remove-element-property", record: "RemoveElementProperty" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove property {}.{} of \"{}\"", self.pset, self.property, self.id), &format!("Eigenschaft {}.{} von \"{}\" entfernen", self.pset, self.property, self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
