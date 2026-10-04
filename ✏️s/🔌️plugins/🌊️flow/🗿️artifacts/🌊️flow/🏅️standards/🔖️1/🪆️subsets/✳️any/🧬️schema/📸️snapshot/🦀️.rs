//! 🧬️ Flow snapshot schema — artifact-lane fields only.

use crate::{flow_content_child_handle_and_cache, flow_working_scene, FlowContentChild};
use framework_schema::ArtifactSchema;

//#region 🔹Snapshot
/// 📸️ Persisted flow document snapshot (persistent fields of the artifact). Ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`flow→C:flow`, the canonical editor for stdio's
/// `flow` subset): the inline `widgets`/`synapses`/`layout` content fields are replaced by a fixed
/// composed `s.stdio.semio@v1/flow` CHILD slot — the flow plugin no longer defines its own node-graph
/// content model, it composes stdio's `flow` subset instead. Viewport state belongs to each
/// concrete Flow main window and never enters this document snapshot.
///
/// Distinct from `semio_framework_artifact_flow_flow::FlowHostSnapshot` in `semio-framework-os-flow`, which remains the framework
/// host/kernel document type. This plugin snapshot converts at the host boundary via
/// `to_host_snapshot`/`from_host_snapshot`, now bridging through the composed child + working-scene cache.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(extension = "flow")]
#[artifact_schema(id = "s.flow.flow")]
pub struct FlowSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub content: FlowContentChild,
}
//#endregion 🔹Snapshot

//#region 🔹DefaultsAndBridge
impl Default for FlowSnapshot {
    fn default() -> Self {
        Self::from_host_snapshot(semio_framework_artifact_flow_flow::FlowHostSnapshot::default())
    }
}

impl FlowSnapshot {
    /// 🌊️ Builds a plugin snapshot from the framework `semio_framework_artifact_flow_flow::FlowHostSnapshot` document type — mints and
    /// caches a fresh content-addressed handle for the fixture's widgets/synapses/layout.
    pub fn from_host_snapshot(host_snapshot: semio_framework_artifact_flow_flow::FlowHostSnapshot) -> Self {
        Self { schema: host_snapshot.schema, content: flow_content_child_handle_and_cache(host_snapshot.widgets, host_snapshot.synapses, host_snapshot.layout) }
    }

    /// 🌊️ Converts this snapshot into the framework `semio_framework_artifact_flow_flow::FlowHostSnapshot` for `FlowHost` / kernel
    /// codecs — reads the live widgets/synapses/layout off the working-scene cache (see
    /// `flow_working_scene`'s doc comment for the staleness gap this bridges).
    pub fn to_host_snapshot(&self) -> semio_framework_artifact_flow_flow::FlowHostSnapshot {
        let (widgets, synapses, layout) = flow_working_scene(self).into_parts();
        semio_framework_artifact_flow_flow::FlowHostSnapshot { schema: self.schema.clone(), camera: default_window_camera(), widgets, synapses, layout }
    }
}

/// 🎥️ Deterministic viewport used only at the framework fixture boundary.
pub fn default_window_camera() -> semio_framework_artifact_flow_flow::CameraJson {
    semio_framework_artifact_flow_flow::CameraJson { x: 0.0, y: 0.0, zoom: 1.0 }
}

impl From<semio_framework_artifact_flow_flow::FlowHostSnapshot> for FlowSnapshot {
    fn from(host_snapshot: semio_framework_artifact_flow_flow::FlowHostSnapshot) -> Self {
        Self::from_host_snapshot(host_snapshot)
    }
}

impl From<FlowSnapshot> for semio_framework_artifact_flow_flow::FlowHostSnapshot {
    fn from(snapshot: FlowSnapshot) -> Self {
        snapshot.to_host_snapshot()
    }
}
//#endregion 🔹DefaultsAndBridge

//#region 🔹HandcraftedArtifactCodecs
/// ✉️ Literal persisted parent and child fields under the owned record grammar.
impl store::ArtifactDsl for FlowSnapshot {
 const EXTENSION:&'static str="flow";
 fn envelope_id()->&'static str{"flow.flow"}
 fn parse_dsl(text:&str)->Result<Self,semio_framework_diagnostic::TextError>{
  let body=store::semio_format::split_text_preamble(text).map(|(_,body)|body).unwrap_or(text);
  let record=semio_framework_dsl_record::parse(body,&Self::__dsl_spec(),&semio_framework_dsl_record::ParseOptions{limits:semio_framework_diagnostic::Limits::default(),mode:semio_framework_dsl_record::SourceMode::Document})?;Self::__dsl_from_record(&record)
 }
 fn print_dsl(&self)->String{
  let body=semio_framework_dsl_record::print(&self.__dsl_to_record(),&Self::__dsl_spec(),semio_framework_dsl_record::JoinMode::Document);
  let envelope=store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Dsl,1).expect("valid envelope identity");store::semio_format::wrap_text(&envelope,&body)
 }
}
/// 📦️ Typed physical Pack record for the same six literal strings.
impl store::ArtifactPack for FlowSnapshot{
 fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{
  let body=store::pack_rt::encode_document(&Self::__dsl_spec(),&self.__dsl_to_record(),options)?;
  let envelope=store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1).map_err(|e|store::PackError::from(e.into_value_error()))?;Ok(store::semio_format::wrap_binary(&envelope,&body))
 }
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{
  let(envelope,body)=store::semio_format::unwrap_binary(bytes).map_err(|e|store::PackError::from(e.into_value_error()))?;
  if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1){return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Flow pack identity differs")))}
  let(record,_)=store::pack_rt::decode_document(&body,&Self::__dsl_spec(),options)?;Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
 }
 fn record_spec()->Option<semio_framework_dsl_record::RecordSpec>{Some(Self::__dsl_spec())}
}
//#endregion 🔹HandcraftedArtifactCodecs

#[cfg(test)]
#[path="🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_snapshot_tests;

#[path="🪶️sqlite/🦀️.rs"]
pub mod sqlite;
