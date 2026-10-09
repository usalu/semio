//! 📐️ `set-beam` payload. Edits a beam sparsely: type, top offset at the start (signed: positive above, negative below the storey top), top offset at the end (assigned: an inclined beam, or none for a level one) and name; absent fields stay untouched. Its axis is set by `set-beam-axis`.

use crate::{Assigned, BeamPatch, ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetBeam {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub beam_type: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub top_offset: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub end_top_offset: Option<Assigned<Option<f64>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl SetBeam {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> BeamPatch {
        BeamPatch { beam_type: self.beam_type.clone(), top_offset: self.top_offset, end_top_offset: self.end_top_offset.clone(), name: self.name.clone(), ..Default::default() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: BeamPatch) -> Self {
        Self { id, beam_type: patch.beam_type, top_offset: patch.top_offset, end_top_offset: patch.end_top_offset, name: patch.name }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetBeam {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "beam", kind: "set-beam", record: "SetBeam" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Edit beam \"{}\"", self.id), &format!("Träger \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
