//! 🧬️ schema leaf
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema, semio_framework_value::RetireOwned, semio_framework_value_derive::NativeReceive)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[artifact_schema(id = "s.vcs.vcs.presence")]
pub struct VcsDemoPresence {}

impl store::ArtifactPresenceSnapshot for VcsDemoPresence {
    fn decode_presence_native(bytes: &[u8], owner: &mut store::io::control::NativeSnapshotDecodeOwner<'_, '_>) -> Result<Self, semio_framework_value::ValueError> {
        store::decode_presence_json(bytes, owner)
    }
}
