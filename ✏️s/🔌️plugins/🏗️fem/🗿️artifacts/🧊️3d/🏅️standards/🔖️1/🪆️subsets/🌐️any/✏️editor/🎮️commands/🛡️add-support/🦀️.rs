//! 🧱️ 🧱️ FEM 3D app commands command — `add-support`.

use semio_framework_plugin::NoConfig;

use semio_framework_plugin::NoConfigMutation;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "add-support")]
pub struct AddSupport {
    pub node_id: String,
    pub fixed: Vec<semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof>,
}

pub fn handle(payload: &AddSupport, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let id = doc.operation()?.entity_id("sup", "supports")?;
    Ok(Emit::mutations(vec![Fem3dMutation::CreateSupport(semio_s_artifact_fem_3d_boundary::standards::v1::subsets::any::schema::mutations::create_support::CreateSupport { support: semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemSupport { id, node_id: payload.node_id.clone(), fixed: payload.fixed.clone() }, index: None })]))
}
