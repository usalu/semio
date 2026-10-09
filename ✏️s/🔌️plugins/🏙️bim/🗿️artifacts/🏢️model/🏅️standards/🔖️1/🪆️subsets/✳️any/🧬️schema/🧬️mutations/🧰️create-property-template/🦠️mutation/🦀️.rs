//! 🧰️ `create-property-template` payload. Brings a new property set template into the model library: the property set name, the element and type kinds it applies to and the definition of each of its properties. It changes no element; the effective properties are inferred.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, PropertyTemplate};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreatePropertyTemplate {
    pub id: String,
    pub template: PropertyTemplate,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreatePropertyTemplate {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "property-template", kind: "create-property-template", record: "CreatePropertyTemplate" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create property template \"{}\"", self.template.name), &format!("Eigenschaftsvorlage \"{}\" anlegen", self.template.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
