//! 🪪️ `rename-element` payload. Changes the display name of any element (the label of a grid line) in whichever collection holds the id.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RenameElement {
    pub id: String,
    pub name: String,
}

impl MutationKind<ModelSnapshot, ModelMutation> for RenameElement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "element", kind: "rename-element", record: "RenameElement" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rename element \"{}\" to \"{}\"", self.id, self.name), &format!("Element \"{}\" in \"{}\" umbenennen", self.id, self.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
