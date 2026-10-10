//! 🧱️ `create-wall` payload. Brings a new wall onto a storey; its height is never stored, it is inferred from the top constraint.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, Wall};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateWall {
    pub id: String,
    pub wall: Wall,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateWall {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "wall", kind: "create-wall", record: "CreateWall" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create wall \"{}\"", self.wall.name), &format!("Wand \"{}\" anlegen", self.wall.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
