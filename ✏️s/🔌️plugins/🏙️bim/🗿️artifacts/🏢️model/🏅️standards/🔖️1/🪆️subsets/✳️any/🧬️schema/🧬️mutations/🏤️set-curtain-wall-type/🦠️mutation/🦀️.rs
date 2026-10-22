//! 🏤️ `set-curtain-wall-type` payload. Patches exactly the provided fields of a curtain wall type: name, grid rules, mullion sections, default panel and materials; every curtain wall of the type follows by inference.

use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{CurtainGrid, CurtainPanel, Profile};
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::library::CurtainWallTypePatch;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCurtainWallType {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub u_grid: Option<CurtainGrid>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub v_grid: Option<CurtainGrid>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub interior_mullion: Option<Profile>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub border_mullion: Option<Profile>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub panel: Option<CurtainPanel>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub panel_material: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mullion_material: Option<String>,
}

impl SetCurtainWallType {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> CurtainWallTypePatch {
        CurtainWallTypePatch { name: self.name.clone(), u_grid: self.u_grid.clone(), v_grid: self.v_grid.clone(), interior_mullion: self.interior_mullion.clone(), border_mullion: self.border_mullion.clone(), panel: self.panel.clone(), panel_material: self.panel_material.clone(), mullion_material: self.mullion_material.clone(), u_value: None, g_value: None, frame_fraction: None }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: CurtainWallTypePatch) -> Self {
        Self { id, name: patch.name, u_grid: patch.u_grid, v_grid: patch.v_grid, interior_mullion: patch.interior_mullion, border_mullion: patch.border_mullion, panel: patch.panel, panel_material: patch.panel_material, mullion_material: patch.mullion_material }
    }
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetCurtainWallType
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetCurtainWallType>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "curtain-wall-type", kind: "set-curtain-wall-type", record: "SetCurtainWallType" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Edit curtain wall type \"{}\"", self.id), &format!("Fassadentyp \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for SetCurtainWallType
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<SetCurtainWallType>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_curtain_wall_type::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_curtain_wall_type::inverse::inverse(self, base))
    }
}
