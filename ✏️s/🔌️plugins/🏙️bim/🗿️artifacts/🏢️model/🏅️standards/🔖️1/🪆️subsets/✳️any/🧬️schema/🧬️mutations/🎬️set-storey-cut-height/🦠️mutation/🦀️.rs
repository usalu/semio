//! 🎬️ `set-storey-cut-height` payload. Sets the height above its elevation at which the plan view cuts a storey; an assigned null returns to the 1.2 m default. Pure view convention: nothing else in the model depends on it.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use crate::Assigned;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetStoreyCutHeight {
    pub id: String,
    pub cut_height: Assigned<Option<f64>>,
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetStoreyCutHeight {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "storey", kind: "set-storey-cut-height", record: "SetStoreyCutHeight" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set plan cut height of storey \"{}\"", self.id), &format!("Schnitthöhe von Geschoss \"{}\" setzen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
