//! 🏴️ `set-tag` payload. Sets exactly the provided fields of a tag: element, category, offset from the element and style; absent fields stay untouched.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, Point2, TagCategory, TagPatch};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetTag {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub element: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<TagCategory>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<Point2>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

impl SetTag {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> TagPatch {
        TagPatch { element: self.element.clone(), category: self.category, offset: self.offset, style: self.style.clone(), ..Default::default() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: TagPatch) -> Self {
        Self { id, element: patch.element, category: patch.category, offset: patch.offset, style: patch.style }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetTag {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "tag", kind: "set-tag", record: "SetTag" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change tag \"{}\"", self.id), &format!("Kennzeichnung \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
