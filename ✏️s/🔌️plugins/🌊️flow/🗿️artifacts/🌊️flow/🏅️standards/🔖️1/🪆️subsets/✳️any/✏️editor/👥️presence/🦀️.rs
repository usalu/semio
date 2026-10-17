//! 👥️ Flow presence — shareable live ephemeral state + mutations.

use semio_framework_artifact_flow_flow::CameraJson;
use pack::snapshot::ArtifactPack;

#[path = "♻️retirement/🦀️.rs"]
pub mod retirement;

//#region 🔖️Presence
/// 👥️ Shareable live APP-SPECIFIC subset of flow view state — preview toggles + the node-graph camera
/// only. Ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `selected_node_ids`/
/// `selected_edge_ids`/`selected_handle_ids` deleted — the "graph" interaction domain's selection is
/// now broadcast generically by the framework (`PresenceInteraction`/`PresenceDomain`), for every peer,
/// with no per-app mirror needed.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[artifact(extension = "flow.presence")]
#[dsl(layout = "lines")]
pub struct FlowPresence {
    pub preview_off_node_ids: Vec<String>,
    #[dsl(block)]
    pub camera: CameraJson,
}

impl store::ArtifactPresenceSnapshot for FlowPresence {}

impl Default for FlowPresence {
    fn default() -> Self {
        Self { preview_off_node_ids: Vec::new(), camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 } }
    }
}

store::sparse_record_diff! {
    record: FlowPresence,
    diff: FlowPresenceDiff,
    fields: {
        preview_off_node_ids: Vec<String>,
        camera: CameraJson,
    },
}



impl ArtifactPack for FlowPresence {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as pack::snapshot::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::os_store::serialization::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for FlowPresence {

}
//#endregion 🔖️Presence

//#region 🔖️PresenceMutation
store::field_set_mutations! {
    record: FlowPresence,
    diff: FlowPresenceDiff,
    set: FlowPresenceMutation,
    owner: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence",
    payload_schema: "🧬️schema/🔣️.json",
    emoji: "👥️",
    fields: {
        preview_off_node_ids: Vec<String> => SetPreviewOffNodeIds "set-preview-off-node-ids",
        camera: CameraJson => SetCamera "set-camera",
    },
}
//#endregion 🔖️PresenceMutation



#[path="🚪️io/📝️text/🧬️mutations/🦀️.rs"]
mod field_set_text_io;

#[path="🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
mod field_set_binary_io;

#[path="🚪️io/📝️text/📸️snapshot/🦀️.rs"]
mod text_snapshot_io;
