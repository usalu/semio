//! 🔁 `update-camera` payload — document-level scalar facet: `CameraJson { x, y, zoom }` is a
//! single inseparable viewport facet, never meaningfully set one-field-at-a-time
//! (`📓️derivation-rules.md` rule 1's `update-<facet>` exception).

use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::CameraJson;
//#region 🔖️UpdateCamera
/// 🔁 Whole-artifact scope — the fixture has exactly one camera.
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, semio_framework_value::RetireOwned)]
#[mutation_leaf(contract = ::protocol)]
pub struct UpdateCamera {
    pub camera: CameraJson,
}

impl<Op> protocol::MutationKind<Generation3dSnapshot, Op> for UpdateCamera
where
    Op: protocol::Mutation<Generation3dSnapshot, Diff = Generation3dDiff> + From<UpdateCamera>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "camera", kind: "update-camera", record: "UpdatedCamera" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Update camera", "Kamera aktualisieren")
    }
}

impl<Op> protocol::ColdMutationKind<Generation3dSnapshot, Op> for UpdateCamera
where
    Op: protocol::ColdMutation<Generation3dSnapshot, ReferenceDiff = Generation3dDiff> + From<UpdateCamera>,
{
    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::update_camera::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::update_camera::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️UpdateCamera
