//! 🧱️ 🧱️ FEM 3D app commands command — `add-frame`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "add-frame")]
pub struct AddFrame {
    pub start: String,
    pub end: String,
    pub material_id: String,
    pub section_id: String,
    pub roll: f64,
}

pub fn handle(payload: &AddFrame, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let id = semio_s_fem_app_surface::next_id(snapshot.elements.iter().map(|e| semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::element_id(e).to_string()), "e");
    let element = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemElement::Frame { id, start: payload.start.clone(), end: payload.end.clone(), material_id: payload.material_id.clone(), section_id: payload.section_id.clone(), roll: payload.roll };
    Ok(Emit::mutations(vec![Fem3dMutation::CreateElement(semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::create_element::CreateElement { element: Box::new(element), index: None })]))
}
