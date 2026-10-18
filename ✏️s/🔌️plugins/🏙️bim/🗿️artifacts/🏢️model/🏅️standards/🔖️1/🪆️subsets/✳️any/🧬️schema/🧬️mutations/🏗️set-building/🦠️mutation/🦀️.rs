//! 🏗️ `set-building` payload. Patches a building (name, origin, rotation, elevation): exactly the provided fields change; the site it stands on never changes here. Changing the building elevation re-infers the absolute elevation of every storey of the building (parametric).

use crate::standards::v1::subsets::any::schema::{diff::{patches::BuildingPatch, ModelDiff}, snapshot::{ModelSnapshot, Point2}};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetBuilding {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<Point2>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rotation: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub elevation: Option<f64>,
}

impl SetBuilding {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> BuildingPatch {
        BuildingPatch { name: self.name.clone(), origin: self.origin.clone(), rotation: self.rotation.clone(), elevation: self.elevation.clone(), ..Default::default() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: BuildingPatch) -> Self {
        Self { id, name: patch.name, origin: patch.origin, rotation: patch.rotation, elevation: patch.elevation }
    }
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetBuilding
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetBuilding>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "building", kind: "set-building", record: "SetBuilding" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_building::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_building::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Update building \"{}\"", self.id), &format!("Gebäude \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
