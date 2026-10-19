//! 🔘️ the six DOF toggles `tx`…`rz`) → `ChangeSupport`.
//! 🩹️ Fem3d play app command — `patch-support`: one-field edit of a support (`nodeId`, or one of

use semio_s_artifact_fem_3d_boundary::standards::v1::subsets::any::schema::mutations::change_support::ChangeSupport;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dSupportPatch;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;

use semio_framework_plugin::ArtifactView;

use semio_framework_plugin::ConfigView;

use semio_framework_plugin::Emit;

use semio_framework_plugin::Fault;

use semio_framework_plugin::NoConfig;

use semio_framework_plugin::NoConfigMutation;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

type Fem3dSnapshot = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;

//#region 🔖️PatchSupport
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "patch-support")]
pub struct PatchSupport {
    pub id: String,
    pub field: String,
    pub value: String,
}

fn flag(value: &str) -> Result<bool, Fault> {
    match value.trim() {
        "true" | "1" | "on" => Ok(true),
        "false" | "0" | "off" | "" => Ok(false),
        _ => Err(Fault::from("fem3d.patch.support-value")),
    }
}

/// 🛡️ A DOF toggle keeps the restrained set in the canonical `Tx…Rz` order whatever order the
/// user pressed them in, so two supports restraining the same DOFs compare equal.
pub fn handle(payload: &PatchSupport, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let row = doc.snapshot.supports.iter().find(|row| row.id == payload.id).ok_or_else(|| Fault::from("fem3d.patch.support-missing"))?;
    let mut patch = Fem3dSupportPatch::default();
    match payload.field.as_str() {
        "nodeId" => patch.node_id = Some(payload.value.clone()),
        "tx" => patch.tx = Some(payload.value.trim().parse().map_err(|_| Fault::from("fem3d.patch.support-value"))?),
        "ty" => patch.ty = Some(payload.value.trim().parse().map_err(|_| Fault::from("fem3d.patch.support-value"))?),
        "tz" => patch.tz = Some(payload.value.trim().parse().map_err(|_| Fault::from("fem3d.patch.support-value"))?),
        "rx" => patch.rx = Some(payload.value.trim().parse().map_err(|_| Fault::from("fem3d.patch.support-value"))?),
        "ry" => patch.ry = Some(payload.value.trim().parse().map_err(|_| Fault::from("fem3d.patch.support-value"))?),
        "rz" => patch.rz = Some(payload.value.trim().parse().map_err(|_| Fault::from("fem3d.patch.support-value"))?),
        _ => return Err(Fault::from("fem3d.patch.support-field")),
    }
    if patch.candidate(row) == *row { return Ok(Emit::default()); }
    Ok(Emit::mutations(vec![Fem3dMutation::ChangeSupport(ChangeSupport { id: payload.id.clone(), patch })]))
}
//#endregion 🔖️PatchSupport

//#region 🧪️Tests
//#endregion 🧪️Tests
