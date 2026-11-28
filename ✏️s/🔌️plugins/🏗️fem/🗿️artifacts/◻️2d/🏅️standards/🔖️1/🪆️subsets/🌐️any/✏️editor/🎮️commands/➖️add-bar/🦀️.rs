//! 🧱️ 🧱️ Fem2d play app commands command — `add-bar`.

use semio_plugin::NoConfig;

use semio_plugin::NoConfigMutation;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::element_id;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemElement;
use semio_plugin::ArtifactView;
use semio_plugin::ConfigView;
use semio_plugin::Emit;
use semio_plugin::Fault;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

type Fem2dSnapshot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

//#region 🔖️AddNode
//#endregion 🔖️AddNode

//#region 🔖️AddBar
//#endregion 🔖️AddBar

//#region 🔖️AddBeam
//#endregion 🔖️AddBeam

//#region 🔖️AddMaterial
//#endregion 🔖️AddMaterial

//#region 🔖️AddSection
//#endregion 🔖️AddSection

//#region 🔖️AddSupport
//#endregion 🔖️AddSupport

//#region 🔖️AddRegion
//#endregion 🔖️AddRegion

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "add-bar")]
pub struct AddBar {
    pub start: String,
    pub end: String,
    pub material_id: String,
    pub section_id: String,
}

pub fn handle(payload: &AddBar, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let known = |id: &str| doc.snapshot.nodes.iter().any(|node| node.id.as_str() == id);
    super::add_nodal_load::require(known(&payload.start), "fem2d.reference.node-unknown", &payload.start)?;
    super::add_nodal_load::require(known(&payload.end), "fem2d.reference.node-unknown", &payload.end)?;
    super::add_nodal_load::require(payload.start != payload.end, "fem2d.member.degenerate", &payload.start)?;
    super::add_nodal_load::require(doc.snapshot.materials.iter().any(|material| material.id.as_str() == payload.material_id), "fem2d.reference.material-unknown", &payload.material_id)?;
    super::add_nodal_load::require(doc.snapshot.sections.iter().any(|section| section.id.as_str() == payload.section_id), "fem2d.reference.section-unknown", &payload.section_id)?;
    let id = doc.operation()?.entity_id("e", "elements")?;
    let element = FemElement::Bar { id, start: payload.start.clone(), end: payload.end.clone(), material_id: payload.material_id.clone(), section_id: payload.section_id.clone() };
    Ok(Emit::mutations(vec![Fem2dMutation::CreateElement(semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::create_element::CreateElement { element: Box::new(element), index: None })]))
}
