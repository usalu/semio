//! 🥞️ `set-wall-type-of` payload. Builds a wall as another wall type; its thickness and layers follow by inference.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetWallTypeOf {
    pub id: String,
    pub wall_type: String,
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetWallTypeOf {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "wall", kind: "set-wall-type-of", record: "SetWallTypeOf" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set type of wall \"{}\" to \"{}\"", self.id, self.wall_type), &format!("Typ von Wand \"{}\" auf \"{}\" setzen", self.id, self.wall_type))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
