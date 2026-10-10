//! 🪵️ `create-wall-type` payload. Brings a new layered wall type into the library; every layer names an existing material and has a positive thickness.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, WallType};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateWallType {
    pub id: String,
    pub wall_type: WallType,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateWallType {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "wall-type", kind: "create-wall-type", record: "CreateWallType" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create wall type \"{}\"", self.wall_type.name), &format!("Wandtyp \"{}\" anlegen", self.wall_type.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
