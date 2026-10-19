//! 🩹️ Fem2d play app command — `patch-load`: one-field edit of a load (`nodeId`, `dof`, `value`, `elementId`, `wx`, `wy`, `regionId`, `pressure`) → `ChangeLoad`.

use semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::change_load::ChangeLoad;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dLoadPatch;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::fem2d_load_owner;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

type Fem2dSnapshot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

//#region 🔖️PatchLoad
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "patch-load")]
pub struct PatchLoad {
    pub id: String,
    pub field: String,
    pub value: String,
}

/// 🩹️ Edits one load addressed by its case-scoped interaction identity. Both semantic ids reach
/// `ChangeLoad`. The admissible field set is the one the
/// addressed variant actually has, so `wx` on a nodal load is an unknown field, not a silent no-op.
pub fn handle(payload: &PatchLoad, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let (case_id, load) = fem2d_load_owner(doc.snapshot, &payload.id).ok_or_else(|| Fault::from("fem2d.patch.load-missing"))?;
    let mut patch = Fem2dLoadPatch::empty_for(load);
    let number = || -> Result<f64, Fault> { payload.value.trim().parse().map_err(|_| Fault::from("fem2d.patch.load-value")) };
    let text = payload.value.trim();
    match (&mut patch, payload.field.as_str()) {
        (Fem2dLoadPatch::Nodal { node_id, .. }, "nodeId") => *node_id = Some(text.into()),
        (Fem2dLoadPatch::Nodal { dof, .. }, "dof") => *dof = Some(match text.to_ascii_lowercase().as_str() {
            "tx" => FemDof::Tx, "ty" => FemDof::Ty, "tz" => FemDof::Tz, "rx" => FemDof::Rx, "ry" => FemDof::Ry, "rz" => FemDof::Rz,
            _ => return Err(Fault::from("fem2d.patch.load-value")),
        }),
        (Fem2dLoadPatch::Nodal { value, .. }, "value") => *value = Some(number()?),
        (Fem2dLoadPatch::MemberUdl { element_id, .. }, "elementId") => *element_id = Some(text.into()),
        (Fem2dLoadPatch::MemberUdl { wx, .. }, "wx") => *wx = Some(number()?),
        (Fem2dLoadPatch::MemberUdl { wy, .. }, "wy") => *wy = Some(number()?),

        (Fem2dLoadPatch::Area { region_id, .. }, "regionId") => *region_id = Some(text.into()),
        (Fem2dLoadPatch::Area { pressure, .. }, "pressure") => *pressure = Some(number()?),
        _ => return Err(Fault::from("fem2d.patch.load-field")),
    }
    if patch.candidate(load).map_err(|_| Fault::from("fem2d.patch.load-field"))? == *load { return Ok(Emit::default()); }
    Ok(Emit::mutations(vec![Fem2dMutation::ChangeLoad(ChangeLoad { case_id: case_id.to_string(), id: semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::load_id(load).into(), patch })]))
}
//#endregion 🔖️PatchLoad

//#region 🧪️Tests
//#endregion 🧪️Tests
