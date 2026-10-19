//! 🏋️ 🏋️ FEM 3D app commands command — `add-load-case`.

use semio_framework_plugin::NoConfig;

use semio_framework_plugin::NoConfigMutation;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::create_load_case;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemLoadCase;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "add-load-case")]
pub struct AddLoadCase {
    pub name: String,
    pub self_weight: bool,
}

pub fn handle(payload: &AddLoadCase, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let id = doc.operation()?.entity_id("case", "load_cases")?;
    Ok(Emit::mutations(vec![Fem3dMutation::CreateLoadCase(semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::create_load_case::CreateLoadCase { load_case: FemLoadCase { id, name: payload.name.clone(), loads: Vec::new(), self_weight: payload.self_weight }, index: None })]))
}
