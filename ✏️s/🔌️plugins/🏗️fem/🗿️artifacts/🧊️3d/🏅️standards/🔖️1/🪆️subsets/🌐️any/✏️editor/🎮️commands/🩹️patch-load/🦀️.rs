//! 🎮️ `elementId`, `wx`, `wy`, `wz`, `solidId`, `pressure`) → `ChangeLoad`.
//! 🩹️ Fem3d play app command — `patch-load`: one-field edit of a load (`nodeId`, `dof`, `value`,

use semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::fem3d_load_owner;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::change_load::ChangeLoad;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dLoadPatch;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

type Fem3dSnapshot = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;

//#region 🔖️PatchLoad
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "patch-load")]
pub struct PatchLoad {
    pub id: String,
    pub field: String,
    pub value: String,
}

/// 🩹️ Edits one case-scoped interaction target through its raw semantic IDs. The admissible field set is the one the
/// addressed variant actually has, so `wx` on a nodal load is an unknown field, not a silent no-op.
pub fn handle(payload: &PatchLoad, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let (case_id, load) = fem3d_load_owner(doc.snapshot, &payload.id).ok_or_else(|| Fault::from("fem3d.patch.load-missing"))?;
    let mut patch = Fem3dLoadPatch::empty_for(load);
    let number = || -> Result<f64, Fault> { payload.value.trim().parse().map_err(|_| Fault::from("fem3d.patch.load-value")) };
    let text = payload.value.trim();
    match (&mut patch, payload.field.as_str()) {
        (Fem3dLoadPatch::Nodal { node_id, .. }, "nodeId") => *node_id = Some(text.into()),
        (Fem3dLoadPatch::Nodal { dof, .. }, "dof") => *dof = Some(match text.to_ascii_lowercase().as_str() {
            "tx" => FemDof::Tx, "ty" => FemDof::Ty, "tz" => FemDof::Tz, "rx" => FemDof::Rx, "ry" => FemDof::Ry, "rz" => FemDof::Rz,
            _ => return Err(Fault::from("fem3d.patch.load-value")),
        }),
        (Fem3dLoadPatch::Nodal { value, .. }, "value") => *value = Some(number()?),
        (Fem3dLoadPatch::MemberUdl { element_id, .. }, "elementId") => *element_id = Some(text.into()),
        (Fem3dLoadPatch::MemberUdl { wx, .. }, "wx") => *wx = Some(number()?),
        (Fem3dLoadPatch::MemberUdl { wy, .. }, "wy") => *wy = Some(number()?),
        (Fem3dLoadPatch::MemberUdl { wz, .. }, "wz") => *wz = Some(number()?),
        (Fem3dLoadPatch::Area { solid_id, .. }, "solidId") => *solid_id = Some(text.into()),
        (Fem3dLoadPatch::Area { pressure, .. }, "pressure") => *pressure = Some(number()?),
        _ => return Err(Fault::from("fem3d.patch.load-field")),
    }
    if patch.candidate(load).map_err(|_| Fault::from("fem3d.patch.load-field"))? == *load { return Ok(Emit::default()); }
    Ok(Emit::mutations(vec![Fem3dMutation::ChangeLoad(ChangeLoad { case_id: case_id.to_string(), id: semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::load_id(load).into(), patch })]))
}
//#endregion 🔖️PatchLoad

//#region 🧪️Tests
//#endregion 🧪️Tests
