//! 🧾️ `set-element-property` payload. Sets one typed property of an element's property set; an element without properties gets its first entry, an existing value is replaced.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, PropertyValue};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetElementProperty {
    pub id: String,
    pub pset: String,
    pub property: String,
    pub value: PropertyValue,
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetElementProperty {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "element-property", kind: "set-element-property", record: "SetElementProperty" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set property {}.{} of \"{}\"", self.pset, self.property, self.id), &format!("Eigenschaft {}.{} von \"{}\" setzen", self.pset, self.property, self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
