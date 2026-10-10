//! 🪠️ `set-wall-sweep` payload. Sets exactly the provided fields of a wall sweep: host wall, face, profile, height above the base, inset, material and name; absent fields stay untouched.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, Profile, WallSide, WallSweepPatch};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetWallSweep {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub side: Option<WallSide>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<Profile>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub inset: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl SetWallSweep {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> WallSweepPatch {
        WallSweepPatch { host: self.host.clone(), side: self.side, profile: self.profile.clone(), height: self.height, inset: self.inset, material: self.material.clone(), name: self.name.clone() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: WallSweepPatch) -> Self {
        Self { id, host: patch.host, side: patch.side, profile: patch.profile, height: patch.height, inset: patch.inset, material: patch.material, name: patch.name }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetWallSweep {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "wall-sweep", kind: "set-wall-sweep", record: "SetWallSweep" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change wall sweep \"{}\"", self.id), &format!("Wandprofil \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
