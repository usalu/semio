//! 🔑️ `set-door-type` payload. Changes exactly the provided fields of a door type; every element of the type follows by inference.

use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{DoorLeaves, Swing};
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::library::DoorTypePatch;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetDoorType {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub frame_width: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub frame_depth: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub leaves: Option<DoorLeaves>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub swing: Option<Swing>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
}

impl SetDoorType {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> DoorTypePatch {
        DoorTypePatch { name: self.name.clone(), width: self.width.clone(), height: self.height.clone(), frame_width: self.frame_width.clone(), frame_depth: self.frame_depth.clone(), leaves: self.leaves.clone(), swing: self.swing.clone(), material: self.material.clone(), ..Default::default() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: DoorTypePatch) -> Self {
        Self { id, name: patch.name, width: patch.width, height: patch.height, frame_width: patch.frame_width, frame_depth: patch.frame_depth, leaves: patch.leaves, swing: patch.swing, material: patch.material }
    }
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetDoorType
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetDoorType>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "door-type", kind: "set-door-type", record: "SetDoorType" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_door_type::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_door_type::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change door type \"{}\"", self.id), &format!("Türtyp \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
