//! ↗️ `create-leader` payload. Brings a new leader onto a storey plan: a text joined by a line to an anchor (a free point or an element of the model), placed beside the anchor so it follows the element.

use crate::{Leader, ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateLeader {
    pub id: String,
    pub leader: Leader,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateLeader {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "leader", kind: "create-leader", record: "CreateLeader" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create leader \"{}\"", self.id), &format!("Hinweislinie \"{}\" anlegen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
