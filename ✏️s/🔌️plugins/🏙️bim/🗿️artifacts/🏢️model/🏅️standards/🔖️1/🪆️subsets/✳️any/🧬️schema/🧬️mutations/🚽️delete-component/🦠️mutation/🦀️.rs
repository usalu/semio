//! 🚽️ `delete-component` payload. Removes a component together with its parameter overrides, properties and classifications.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteComponent {
    pub id: String,
}

impl MutationKind<ModelSnapshot, ModelMutation> for DeleteComponent {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "component", kind: "delete-component", record: "DeleteComponent" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete component \"{}\" with its overrides", self.id), &format!("Komponente \"{}\" mit ihren Überschreibungen löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
