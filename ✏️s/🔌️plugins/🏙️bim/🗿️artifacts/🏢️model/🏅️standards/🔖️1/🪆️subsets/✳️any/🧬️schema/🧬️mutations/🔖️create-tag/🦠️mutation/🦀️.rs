//! 🔖️ `create-tag` payload. Brings a new tag onto a storey plan: text read from an element (name, type, number or size) and placed beside it, so the tag follows the element.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, Tag};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateTag {
    pub id: String,
    pub tag: Tag,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateTag {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "tag", kind: "create-tag", record: "CreateTag" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create tag \"{}\"", self.id), &format!("Kennzeichnung \"{}\" anlegen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
