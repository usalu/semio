//! 🎞️ `set-beam-type` payload. Changes exactly the provided fields of a beam type; every element of the type follows by inference.

use crate::{BeamTypePatch, ModelDiff, ModelMutation, ModelSnapshot, Profile};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetBeamType {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<Profile>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
}

impl SetBeamType {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> BeamTypePatch {
        BeamTypePatch { name: self.name.clone(), profile: self.profile.clone(), material: self.material.clone() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: BeamTypePatch) -> Self {
        Self { id, name: patch.name, profile: patch.profile, material: patch.material }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetBeamType {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "beam-type", kind: "set-beam-type", record: "SetBeamType" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change beam type \"{}\"", self.id), &format!("Trägertyp \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
