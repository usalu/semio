//! 🚿️ `set-component-override` payload. Overrides the formula of the family parameter `name` for one component, or replaces its override. The formula must parse, use only parameters of the family of the component and never close a circle under the overrides of the component.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetComponentOverride {
    pub component: String,
    pub name: String,
    pub value: String,
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetComponentOverride {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "component-override", kind: "set-component-override", record: "SetComponentOverride" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Override parameter \"{}\" of component \"{}\"", self.name, self.component), &format!("Parameter \"{}\" der Komponente \"{}\" überschreiben", self.name, self.component))
    }
    fn target(&self) -> Vec<String> {
        vec![format!("{}.{}", self.component, self.name)]
    }
}
