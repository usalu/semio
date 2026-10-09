//! 🏬️ `create-curtain-wall` payload. Brings a new curtain wall onto a storey; its height is never stored, it is inferred from the top constraint, its grid, mullions and panels from its type.

use crate::{CurtainWall, ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateCurtainWall {
    pub id: String,
    pub curtain_wall: CurtainWall,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateCurtainWall {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "curtain-wall", kind: "create-curtain-wall", record: "CreateCurtainWall" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create curtain wall \"{}\"", self.curtain_wall.name), &format!("Vorhangfassade \"{}\" anlegen", self.curtain_wall.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
