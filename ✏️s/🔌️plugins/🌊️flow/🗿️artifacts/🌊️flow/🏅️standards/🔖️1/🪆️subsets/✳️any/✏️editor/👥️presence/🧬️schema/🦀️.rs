//! 🧬️ schema leaf
use framework_schema::ArtifactSchema;
use semio_framework_artifact_flow_flow::CameraJson;

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.flow.flow.presence")]
pub struct FlowPresence {
    #[state(presence)]
    pub preview_off_node_ids: Vec<String>,
    #[state(presence)]
    pub camera: CameraJson,
}
