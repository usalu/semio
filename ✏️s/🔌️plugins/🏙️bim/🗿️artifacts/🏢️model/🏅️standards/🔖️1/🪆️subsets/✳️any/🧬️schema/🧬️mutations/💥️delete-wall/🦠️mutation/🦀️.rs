//! 💥️ `delete-wall` payload. Removes a wall together with the openings it hosts and the properties and classifications of both.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteWall {
    pub id: String,
}

impl MutationKind<ModelSnapshot, ModelMutation> for DeleteWall {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "wall", kind: "delete-wall", record: "DeleteWall" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete wall \"{}\" with its openings", self.id), &format!("Wand \"{}\" samt Öffnungen löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
