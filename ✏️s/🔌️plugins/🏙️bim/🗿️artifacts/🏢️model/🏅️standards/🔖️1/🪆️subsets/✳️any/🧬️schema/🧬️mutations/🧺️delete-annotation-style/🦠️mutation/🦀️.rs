//! 🧺️ `delete-annotation-style` payload. Removes an annotation style that no dimension, tag, text note or leader uses.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteAnnotationStyle {
    pub id: String,
}

impl MutationKind<ModelSnapshot, ModelMutation> for DeleteAnnotationStyle {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "annotation-style", kind: "delete-annotation-style", record: "DeleteAnnotationStyle" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete annotation style \"{}\"", self.id), &format!("Beschriftungsstil \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
