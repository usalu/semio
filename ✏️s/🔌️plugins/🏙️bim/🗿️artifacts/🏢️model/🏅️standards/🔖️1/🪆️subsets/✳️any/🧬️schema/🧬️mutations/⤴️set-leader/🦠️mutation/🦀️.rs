//! ⤴️ `set-leader` payload. Sets exactly the provided fields of a leader: anchor, offset of the text from the anchor, text and style; absent fields stay untouched.

use crate::{AnnotationAnchor, LeaderPatch, ModelDiff, ModelMutation, ModelSnapshot, Point2};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetLeader {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<AnnotationAnchor>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<Point2>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

impl SetLeader {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> LeaderPatch {
        LeaderPatch { anchor: self.anchor.clone(), offset: self.offset, text: self.text.clone(), style: self.style.clone(), ..Default::default() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: LeaderPatch) -> Self {
        Self { id, anchor: patch.anchor, offset: patch.offset, text: patch.text, style: patch.style }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetLeader {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "leader", kind: "set-leader", record: "SetLeader" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change leader \"{}\"", self.id), &format!("Hinweislinie \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
