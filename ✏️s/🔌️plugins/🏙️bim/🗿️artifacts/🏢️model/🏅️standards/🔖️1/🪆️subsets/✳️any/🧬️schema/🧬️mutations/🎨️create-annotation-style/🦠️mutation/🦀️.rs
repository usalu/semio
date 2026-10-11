//! 🎨️ `create-annotation-style` payload. Brings a new annotation style into the library: text height, line end mark, printed unit and precision shared by dimensions, tags, notes and leaders.

use crate::{AnnotationStyle, ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateAnnotationStyle {
    pub id: String,
    pub annotation_style: AnnotationStyle,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateAnnotationStyle {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "annotation-style", kind: "create-annotation-style", record: "CreateAnnotationStyle" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create annotation style \"{}\"", self.annotation_style.name), &format!("Beschriftungsstil \"{}\" anlegen", self.annotation_style.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
