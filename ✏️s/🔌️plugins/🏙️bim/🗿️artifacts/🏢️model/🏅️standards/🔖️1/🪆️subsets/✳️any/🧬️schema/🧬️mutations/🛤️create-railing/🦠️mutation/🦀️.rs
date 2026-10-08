//! 🛤️ `create-railing` payload. Brings a new railing onto a storey; its posts, rails and extent are inferred from the authored path, height and post spacing.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, Railing};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateRailing {
    pub id: String,
    pub railing: Railing,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateRailing {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "railing", kind: "create-railing", record: "CreateRailing" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create railing \"{}\"", self.railing.name), &format!("Geländer \"{}\" anlegen", self.railing.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
