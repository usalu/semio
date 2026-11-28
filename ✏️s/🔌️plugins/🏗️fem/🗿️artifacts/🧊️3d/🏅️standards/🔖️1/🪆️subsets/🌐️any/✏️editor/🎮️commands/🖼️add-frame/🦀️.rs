//! 🧱️ 🧱️ FEM 3D app commands command — `add-frame`.

use semio_plugin::NoConfig;

use semio_plugin::NoConfigMutation;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use semio_plugin::ArtifactView;
use semio_plugin::ConfigView;
use semio_plugin::Emit;
use semio_plugin::Fault;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

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
    let known = |id: &str| doc.snapshot.nodes.iter().any(|node| node.id.as_str() == id);
    super::add_nodal_load::require(known(&payload.start), "fem3d.reference.node-unknown", &payload.start)?;
    super::add_nodal_load::require(known(&payload.end), "fem3d.reference.node-unknown", &payload.end)?;
    super::add_nodal_load::require(payload.start != payload.end, "fem3d.member.degenerate", &payload.start)?;
    super::add_nodal_load::require(doc.snapshot.materials.iter().any(|material| material.id.as_str() == payload.material_id), "fem3d.reference.material-unknown", &payload.material_id)?;
    super::add_nodal_load::require(doc.snapshot.sections.iter().any(|section| section.id.as_str() == payload.section_id), "fem3d.reference.section-unknown", &payload.section_id)?;
    let id = doc.operation()?.entity_id("e", "elements")?;
    let element = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemElement::Frame { id, start: payload.start.clone(), end: payload.end.clone(), material_id: payload.material_id.clone(), section_id: payload.section_id.clone(), roll: payload.roll };
    Ok(Emit::mutations(vec![Fem3dMutation::CreateElement(semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::create_element::CreateElement { element: Box::new(element), index: None })]))
}
