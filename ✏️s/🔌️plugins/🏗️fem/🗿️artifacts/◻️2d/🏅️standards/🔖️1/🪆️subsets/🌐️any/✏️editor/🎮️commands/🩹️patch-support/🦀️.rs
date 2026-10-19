//! 🩹️ Fem2d play app command — `patch-support`: re-targets a support (`nodeId`) or toggles one restrained DOF (`tx`, `ty`, `rz`) → `ChangeSupport`.

use semio_s_artifact_fem_2d_boundary::standards::v1::subsets::any::schema::mutations::change_support::ChangeSupport;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dSupportPatch;
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

//#region 🔖️PatchSupport
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "patch-support")]
pub struct PatchSupport {
    pub id: String,
    pub field: String,
    pub value: String,
}

/// 🩹️ Each inspector control emits only its owned node reference or DOF membership.
pub fn handle(payload: &PatchSupport, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let row = doc.snapshot.supports.iter().find(|row| row.id == payload.id).ok_or_else(|| Fault::from("fem2d.patch.support-missing"))?;
    let mut patch = Fem2dSupportPatch::default();
    match payload.field.as_str() {
        "nodeId" => patch.node_id = Some(payload.value.clone()),
        "tx" => patch.tx = Some(payload.value.trim().parse().map_err(|_| Fault::from("fem2d.patch.support-value"))?),
        "ty" => patch.ty = Some(payload.value.trim().parse().map_err(|_| Fault::from("fem2d.patch.support-value"))?),
        "rz" => patch.rz = Some(payload.value.trim().parse().map_err(|_| Fault::from("fem2d.patch.support-value"))?),
        _ => return Err(Fault::from("fem2d.patch.support-field")),
    }
    if patch.candidate(row) == *row { return Ok(Emit::default()); }
    Ok(Emit::mutations(vec![Fem2dMutation::ChangeSupport(ChangeSupport { id: payload.id.clone(), patch })]))
}
//#endregion 🔖️PatchSupport

//#region 🧪️Tests
//#endregion 🧪️Tests
