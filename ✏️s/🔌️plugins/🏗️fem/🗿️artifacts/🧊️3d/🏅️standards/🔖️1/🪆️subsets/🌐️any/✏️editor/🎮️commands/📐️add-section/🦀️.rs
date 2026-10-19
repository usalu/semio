//! 🧱️ 🧱️ FEM 3D app commands command — `add-section`.

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
#[dsl(keyword = "add-section")]
pub struct AddSection {
    pub name: String,
    pub area: f64,
    pub iy: f64,
    pub iz: f64,
    pub j: f64,
}

pub fn handle(payload: &AddSection, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let id = doc.operation()?.entity_id("s", "sections")?;
    Ok(Emit::mutations(vec![Fem3dMutation::CreateSection(semio_s_artifact_fem_3d_mesh_sections::standards::v1::subsets::any::schema::mutations::create_section::CreateSection {
        section: semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemSection { id, name: payload.name.clone(), area: payload.area, iy: payload.iy, iz: payload.iz, j: payload.j }, index: None,
    })]))
}
