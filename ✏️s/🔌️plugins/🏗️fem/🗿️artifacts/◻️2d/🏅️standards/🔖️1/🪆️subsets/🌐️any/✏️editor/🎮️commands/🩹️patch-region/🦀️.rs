//! 🩹️ Fem2d play app command — `patch-region`: one-field edit of a meshed region (`name`, `thickness`, `meshSize`, `materialId`) → `ChangeRegion`.

use semio_s_artifact_fem_2d_mesh_regions::standards::v1::subsets::any::schema::mutations::change_region::ChangeRegion;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dRegionPatch;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

type Fem2dSnapshot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

//#region 🔖️PatchRegion
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "patch-region")]
pub struct PatchRegion {
    pub id: String,
    pub field: String,
    pub value: String,
}

/// 🩹️ Edits a region's scalar properties. The `outline`/`holes` polygons are deliberately not
/// reachable from a single-field patch — a polygon is edited in the viewport, not in a text box —
/// so the inspector shows their vertex counts read-only and every editable field lands here.
pub fn handle(payload: &PatchRegion, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let region = doc.snapshot.regions.iter().find(|region| region.id == payload.id).ok_or_else(|| Fault::from("fem2d.patch.region-missing"))?;
    let mut patch = Fem2dRegionPatch::default();
    match payload.field.as_str() {
        "name" => patch.name = Some(payload.value.clone()),
        "materialId" => patch.material_id = Some(payload.value.clone()),
        "thickness" => patch.thickness = Some(payload.value.trim().parse().map_err(|_| Fault::from("fem2d.patch.region-value"))?),
        "meshSize" => patch.mesh_size = Some(payload.value.trim().parse().map_err(|_| Fault::from("fem2d.patch.region-value"))?),
        _ => return Err(Fault::from("fem2d.patch.region-field")),
    }
    if patch.candidate(region) == *region {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![Fem2dMutation::ChangeRegion(ChangeRegion { id: payload.id.clone(), patch })]))
}
//#endregion 🔖️PatchRegion

//#region 🧪️Tests
//#endregion 🧪️Tests
