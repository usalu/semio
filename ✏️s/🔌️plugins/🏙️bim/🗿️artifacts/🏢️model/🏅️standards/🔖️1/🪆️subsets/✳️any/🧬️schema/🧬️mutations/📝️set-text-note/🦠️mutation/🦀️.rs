//! 📝️ `set-text-note` payload. Sets exactly the provided fields of a text note: position, text, rotation and style; absent fields stay untouched.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, Point2, TextNotePatch};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetTextNote {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<Point2>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rotation: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

impl SetTextNote {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> TextNotePatch {
        TextNotePatch { position: self.position, text: self.text.clone(), rotation: self.rotation, style: self.style.clone(), ..Default::default() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: TextNotePatch) -> Self {
        Self { id, position: patch.position, text: patch.text, rotation: patch.rotation, style: patch.style }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetTextNote {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "text-note", kind: "set-text-note", record: "SetTextNote" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change text note \"{}\"", self.id), &format!("Textnotiz \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
