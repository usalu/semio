//! 🔧️ `set-railing` payload. Sets any of a railing's authored path, height, post spacing, rail and post sections, baluster row (an assigned null removes it), infill, host (an assigned null releases it), material, base offset and name; absent fields stay untouched.

use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Assigned;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::diff::patches::building_elements::stairs_railings_ramps::RailingPatch;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{Baluster, Infill, Point2, Profile, RailingHost};
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetRailing {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<Point2>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub post_spacing: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<Profile>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub post_profile: Option<Profile>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub baluster: Option<Assigned<Option<Baluster>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub infill: Option<Infill>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<Assigned<Option<RailingHost>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub base_offset: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl SetRailing {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> RailingPatch {
        RailingPatch { path: self.path.clone(), height: self.height.clone(), post_spacing: self.post_spacing.clone(), profile: self.profile.clone(), post_profile: self.post_profile.clone(), baluster: self.baluster.clone(), infill: self.infill.clone(), host: self.host.clone(), material: self.material.clone(), base_offset: self.base_offset.clone(), name: self.name.clone(), ..Default::default() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: RailingPatch) -> Self {
        Self { id, path: patch.path, height: patch.height, post_spacing: patch.post_spacing, profile: patch.profile, post_profile: patch.post_profile, baluster: patch.baluster, infill: patch.infill, host: patch.host, material: patch.material, base_offset: patch.base_offset, name: patch.name }
    }
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetRailing
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetRailing>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "railing", kind: "set-railing", record: "SetRailing" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Edit railing \"{}\"", self.id), &format!("Geländer \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for SetRailing
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<SetRailing>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_railing::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_railing::inverse::inverse(self, base))
    }
}
