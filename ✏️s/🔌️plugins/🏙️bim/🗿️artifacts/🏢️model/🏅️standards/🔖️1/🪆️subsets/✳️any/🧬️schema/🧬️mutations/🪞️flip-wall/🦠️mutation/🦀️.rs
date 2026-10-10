//! 🪞️ `flip-wall` payload. Reverses the orientation of a wall: its axis swaps start and end and the bulge changes sign, so the same curve runs the other way.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct FlipWall {
    pub id: String,
}

impl MutationKind<ModelSnapshot, ModelMutation> for FlipWall {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "toggle", entity: "wall", kind: "flip-wall", record: "FlipWall" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Flip wall \"{}\"", self.id), &format!("Wand \"{}\" wenden", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
