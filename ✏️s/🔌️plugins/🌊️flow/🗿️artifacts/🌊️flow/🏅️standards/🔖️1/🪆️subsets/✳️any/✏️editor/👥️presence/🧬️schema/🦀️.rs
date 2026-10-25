//! 🧬️ schema leaf
use framework_schema::ArtifactSchema;
use semio_framework_artifact_flow_flow::CameraJson;

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_value_derive::NativeReceive, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.flow.flow.presence")]
pub struct FlowPresence {
    #[state(presence)]
    pub preview_off_node_ids: Vec<String>,
    #[state(presence)]
    pub camera: CameraJson,
}

impl store::ArtifactPresenceSnapshot for FlowPresence {
    fn decode_presence_native(bytes: &[u8], owner: &mut store::io::control::NativeSnapshotDecodeOwner<'_, '_>) -> Result<Self, semio_framework_value::ValueError> {
        store::decode_presence_json(bytes, owner)
    }
}
