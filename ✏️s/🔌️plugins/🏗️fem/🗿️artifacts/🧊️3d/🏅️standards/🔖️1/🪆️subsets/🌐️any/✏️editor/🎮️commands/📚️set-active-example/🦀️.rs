//! 📚️ 📚️ FEM 3D app commands command — `set-active-example`.

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
#[dsl(keyword = "active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

/// 📚️ Loads the chosen example through the host document replacement effect while preserving exact-window preferences.
pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let text = match payload.example_id.as_str() {
        id if id == semio_s_artifact_fem_3d_io::examples::demo::ID => Some(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::snapshot::FEM3D_EXAMPLE_TEXT),
        id if id == semio_s_artifact_fem_3d_io::examples::concrete_forest::ID => Some(semio_s_artifact_fem_3d_io::examples::concrete_forest::PRIMARY_TEXT),
        id if id == semio_s_artifact_fem_3d_io::examples::house::ID => Some(semio_s_artifact_fem_3d_io::examples::house::PRIMARY_TEXT),
        _ => None,
    };
    let document = text.map(|text| <Fem3dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).unwrap_or_default()).unwrap_or_default();
    eprintln!("[TRACE] fem3d setActiveExample id={} nodes={} elements={} solids={}", payload.example_id, document.nodes.len(), document.elements.len(), document.solids.len());
    Ok(Emit { effects: vec![semio_s_artifact_fem_3d_io::standards::v1::subsets::any::io::reset_document_effect(&document)], ..Default::default() })
}

