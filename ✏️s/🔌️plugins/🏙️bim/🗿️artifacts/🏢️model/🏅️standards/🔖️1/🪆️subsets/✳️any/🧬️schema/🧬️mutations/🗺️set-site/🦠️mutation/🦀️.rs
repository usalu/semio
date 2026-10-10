//! 🗺️ `set-site` payload. Patches a site (name, latitude, longitude, elevation, true north, boundary loop): exactly the provided fields change. Changing the site elevation re-infers the absolute elevation of every storey of every building on it (parametric).

use crate::{ModelDiff, ModelMutation, ModelSnapshot, Point2, SitePatch};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
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

impl MutationKind<ModelSnapshot, ModelMutation> for SetSite {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "site", kind: "set-site", record: "SetSite" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Update site \"{}\"", self.id), &format!("Standort \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
