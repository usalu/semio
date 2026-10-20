//! 🧱️ 🧱️ FEM 3D app commands command — `add-bar`.

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
#[dsl(keyword = "add-bar")]
pub struct AddBar {
    pub start: String,
    pub end: String,
    pub material_id: String,
    pub section_id: String,
}

pub fn handle(payload: &AddBar, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let id = doc.operation()?.entity_id("e", "elements")?;
    let element = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemElement::Bar { id, start: payload.start.clone(), end: payload.end.clone(), material_id: payload.material_id.clone(), section_id: payload.section_id.clone() };
    Ok(Emit::mutations(vec![Fem3dMutation::CreateElement(semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::create_element::CreateElement { element: Box::new(element), index: None })]))
}
