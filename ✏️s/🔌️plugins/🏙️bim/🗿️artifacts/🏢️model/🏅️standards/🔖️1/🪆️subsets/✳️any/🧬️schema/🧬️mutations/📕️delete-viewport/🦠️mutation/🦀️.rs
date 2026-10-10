//! 📕️ `delete-viewport` payload. Removes a viewport together with its properties and classifications. The sheet and the view stay.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteViewport {
    pub id: String,
}

impl MutationKind<ModelSnapshot, ModelMutation> for DeleteViewport {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "viewport", kind: "delete-viewport", record: "DeleteViewport" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete viewport \"{}\"", self.id), &format!("Ansichtsfenster \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
