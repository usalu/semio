//! 🪠️ `set-wall-sweep` payload. Sets exactly the provided fields of a wall sweep: host wall, face, profile, height above the base, inset, material and name; absent fields stay untouched.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::site_and_spaces::WallSweepPatch;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{Profile, WallSide};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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

impl<Op> MutationKind<ModelSnapshot, Op> for SetWallSweep
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetWallSweep>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "wall-sweep", kind: "set-wall-sweep", record: "SetWallSweep" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change wall sweep \"{}\"", self.id), &format!("Wandprofil \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for SetWallSweep
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<SetWallSweep>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_wall_sweep::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_wall_sweep::inverse::inverse(self, base))
    }
}
