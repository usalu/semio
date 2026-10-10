//! 🪜️ `create-storey` payload. Brings a new storey into a building; its level index is unique within the building.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, Storey};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateStorey {
    pub id: String,
    pub storey: Storey,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateStorey {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "storey", kind: "create-storey", record: "CreateStorey" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create storey \"{}\"", self.storey.name), &format!("Geschoss \"{}\" anlegen", self.storey.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
