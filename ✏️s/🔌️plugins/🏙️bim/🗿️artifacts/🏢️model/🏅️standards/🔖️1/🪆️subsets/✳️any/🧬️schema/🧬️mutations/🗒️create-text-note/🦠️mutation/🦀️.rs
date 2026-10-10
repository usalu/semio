//! 🗒️ `create-text-note` payload. Brings a free text onto a storey plan at a position, rotated and set in an annotation style.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, TextNote};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateTextNote {
    pub id: String,
    pub text_note: TextNote,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateTextNote {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "text-note", kind: "create-text-note", record: "CreateTextNote" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create text note \"{}\"", self.id), &format!("Textnotiz \"{}\" anlegen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
