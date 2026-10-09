//! 🏟️ `create-curtain-wall-type` payload. Brings a new curtain wall type into the library: the grid rules of both directions, the mullion sections of the interior grid lines and of the border, the default panel and the materials.

use crate::{CurtainWallType, ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateCurtainWallType {
    pub id: String,
    pub curtain_wall_type: CurtainWallType,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateCurtainWallType {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "curtain-wall-type", kind: "create-curtain-wall-type", record: "CreateCurtainWallType" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create curtain wall type \"{}\"", self.curtain_wall_type.name), &format!("Fassadentyp \"{}\" anlegen", self.curtain_wall_type.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
