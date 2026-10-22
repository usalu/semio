//! 🔆️ `set-curtain-wall` payload. Adjusts the named parameters of a curtain wall (axis, base offset, top, name); every field left out stays as it is. Its type is set by `set-curtain-wall-type-of`, its grid by `set-curtain-wall-grid`.

use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{Axis, TopConstraint};
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::building_elements::walls_and_curtain_walls::CurtainWallPatch;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCurtainWall {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub axis: Option<Axis>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub base_offset: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub top: Option<TopConstraint>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl SetCurtainWall {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> CurtainWallPatch {
        CurtainWallPatch { axis: self.axis.clone(), base_offset: self.base_offset, top: self.top.clone(), name: self.name.clone(), ..Default::default() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: CurtainWallPatch) -> Self {
        Self { id, axis: patch.axis, base_offset: patch.base_offset, top: patch.top, name: patch.name }
    }
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetCurtainWall
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetCurtainWall>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "curtain-wall", kind: "set-curtain-wall", record: "SetCurtainWall" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Adjust curtain wall \"{}\"", self.id), &format!("Vorhangfassade \"{}\" anpassen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for SetCurtainWall
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<SetCurtainWall>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_curtain_wall::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_curtain_wall::inverse::inverse(self, base))
    }
}
