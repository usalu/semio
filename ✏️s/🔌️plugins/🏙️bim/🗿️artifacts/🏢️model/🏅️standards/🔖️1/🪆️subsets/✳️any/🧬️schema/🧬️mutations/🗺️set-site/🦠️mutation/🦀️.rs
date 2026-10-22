//! 🗺️ `set-site` payload. Patches a site (name, latitude, longitude, elevation, true north, boundary loop): exactly the provided fields change. Changing the site elevation re-infers the absolute elevation of every storey of every building on it (parametric).

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::site_and_spaces::SitePatch;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::Point2;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSite {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub elevation: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub true_north: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub boundary: Option<Vec<Point2>>,
}

impl SetSite {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> SitePatch {
        SitePatch { name: self.name.clone(), latitude: self.latitude.clone(), longitude: self.longitude.clone(), elevation: self.elevation.clone(), true_north: self.true_north.clone(), boundary: self.boundary.clone() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: SitePatch) -> Self {
        Self { id, name: patch.name, latitude: patch.latitude, longitude: patch.longitude, elevation: patch.elevation, true_north: patch.true_north, boundary: patch.boundary }
    }
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetSite
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetSite>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "site", kind: "set-site", record: "SetSite" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Update site \"{}\"", self.id), &format!("Standort \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for SetSite
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<SetSite>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_site::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_site::inverse::inverse(self, base))
    }
}
