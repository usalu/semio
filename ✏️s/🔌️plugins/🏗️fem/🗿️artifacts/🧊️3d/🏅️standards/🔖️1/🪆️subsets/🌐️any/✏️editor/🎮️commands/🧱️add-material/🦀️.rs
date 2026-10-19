//! 🧱️ 🧱️ FEM 3D app commands command — `add-material`.

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
#[dsl(keyword = "add-material")]
pub struct AddMaterial {
    pub name: String,
    pub e: f64,
    pub g: f64,
}

/// 🧱️ New materials default to `nu = 0.3`/`rho = 7850.0` (mild steel) — the manifest's `addMaterial`
/// arg form only stages `name`/`e`/`g`, matching the pre-migration `handle_action` behavior verbatim.
pub fn handle(payload: &AddMaterial, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let id = doc.operation()?.entity_id("m", "materials")?;
    Ok(Emit::mutations(vec![Fem3dMutation::CreateMaterial(semio_s_artifact_fem_3d_material::standards::v1::subsets::any::schema::mutations::create_material::CreateMaterial {
        material: semio_s_fem_material::volumetric::FemMaterialValue { id, name: payload.name.clone(), e: payload.e, g: payload.g, nu: 0.3, rho: 7850.0 }, index: None,
    })]))
}
