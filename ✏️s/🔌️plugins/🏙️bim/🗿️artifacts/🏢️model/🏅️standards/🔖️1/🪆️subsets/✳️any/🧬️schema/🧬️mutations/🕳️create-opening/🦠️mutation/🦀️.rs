//! 🕳️ `create-opening` payload. Brings a window, door or void into a wall or curtain wall; only its centre offset along the host axis and its optional sill override are stored, the frame follows the host by inference.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, Opening};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateOpening {
    pub id: String,
    pub opening: Opening,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateOpening {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "opening", kind: "create-opening", record: "CreateOpening" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create opening \"{}\"", self.opening.name), &format!("Öffnung \"{}\" anlegen", self.opening.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
