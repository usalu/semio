//! 🛋️ `create-space` payload. Brings a new room onto a storey; a bounded space stores only its seed point, its outline, area and volume are inferred from the walls around it.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, Space};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateSpace {
    pub id: String,
    pub space: Space,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateSpace {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "space", kind: "create-space", record: "CreateSpace" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create space \"{}\"", self.space.name), &format!("Raum \"{}\" anlegen", self.space.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
