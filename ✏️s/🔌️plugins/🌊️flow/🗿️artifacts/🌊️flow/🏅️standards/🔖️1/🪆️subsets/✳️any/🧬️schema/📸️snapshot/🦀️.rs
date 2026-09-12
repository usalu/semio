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
/// Distinct from `semio_framework_artifact_flow_flow::FlowFixture` in `semio-framework-os-flow`, which remains the framework
/// host/kernel document type. This plugin snapshot converts at the host boundary via
/// `to_fixture`/`from_fixture`, now bridging through the composed child + working-scene cache.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
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
        Self::from_fixture(semio_framework_artifact_flow_flow::FlowFixture::default())
    }
}

impl FlowSnapshot {
    /// 🌊️ Builds a plugin snapshot from the framework `semio_framework_artifact_flow_flow::FlowFixture` document type — mints and
    /// caches a fresh content-addressed handle for the fixture's widgets/synapses/layout.
    pub fn from_fixture(fixture: semio_framework_artifact_flow_flow::FlowFixture) -> Self {
        Self { schema: fixture.schema, content: flow_content_child_handle_and_cache(fixture.widgets, fixture.synapses, fixture.layout) }
    }

    /// 🌊️ Converts this snapshot into the framework `semio_framework_artifact_flow_flow::FlowFixture` for `FlowHost` / kernel
    /// codecs — reads the live widgets/synapses/layout off the working-scene cache (see
    /// `flow_working_scene`'s doc comment for the staleness gap this bridges).
    pub fn to_fixture(&self) -> semio_framework_artifact_flow_flow::FlowFixture {
        let scene = flow_working_scene(self);
        semio_framework_artifact_flow_flow::FlowFixture { schema: self.schema.clone(), camera: default_window_camera(), widgets: scene.widgets, synapses: scene.synapses, layout: scene.layout }
    }
}

/// 🎥️ Deterministic viewport used only at the framework fixture boundary.
pub fn default_window_camera() -> semio_framework_artifact_flow_flow::CameraJson {
    semio_framework_artifact_flow_flow::CameraJson { x: 0.0, y: 0.0, zoom: 1.0 }
}

impl From<semio_framework_artifact_flow_flow::FlowFixture> for FlowSnapshot {
    fn from(fixture: semio_framework_artifact_flow_flow::FlowFixture) -> Self {
        Self::from_fixture(fixture)
    }
}

impl From<FlowSnapshot> for semio_framework_artifact_flow_flow::FlowFixture {
    fn from(snapshot: FlowSnapshot) -> Self {
        snapshot.to_fixture()
    }
}
//#endregion 🔹DefaultsAndBridge

//#region 🔹HandcraftedArtifactCodecs
/// ✉️ ArtifactDsl — JSON body under envelope id `flow.flow`.
///
/// Does not call `semio_framework_artifact_flow_flow::FlowFixture`'s codecs: that framework type still emits envelope id `flow`,
/// which `SemioEnvelope::from_envelope_id` rejects (`plugin.artifact` required). Fixup belongs in
/// `semio-framework-os-flow`; this plugin snapshot owns a valid envelope of its own.
impl store::ArtifactDsl for FlowSnapshot {
    const EXTENSION: &'static str = "flow";
    fn envelope_id() -> &'static str {
        "flow.flow"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let trimmed = body.trim_start();
        if trimmed.starts_with('{') {
            let json: serde_json::Value = serde_json::from_str(trimmed).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))?;
            let value: dsl::DslValue = json.into();
            return dsl::FromValue::from_value(value).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)));
        }
        <semio_framework_artifact_flow_flow::FlowFixture as store::ArtifactDsl>::parse_dsl(text).map(Self::from_fixture)
    }
    fn print_dsl(&self) -> String {
        let value = dsl::ToValue::to_value(self);
        let json: serde_json::Value = value.into();
        let body = serde_json::to_string_pretty(&json).expect("FlowSnapshot serde");
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ ArtifactPack — JSON body under envelope id `flow.flow` (see ArtifactDsl note).
impl store::ArtifactPack for FlowSnapshot {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let value = dsl::ToValue::to_value(self);
        let json: serde_json::Value = value.into();
        let body = serde_json::to_vec(&json).map_err(|error| store::PackError::Schema(error.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        let our_id = <Self as store::ArtifactDsl>::envelope_id();
        if !envelope.matches_identity(our_id, store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {our_id}.pack v1, got {}", envelope.binary_token())));
        }
        let json: serde_json::Value = serde_json::from_slice(&body).map_err(|error| store::PackError::Schema(error.to_string()))?;
        let value: dsl::DslValue = json.into();
        dsl::FromValue::from_value(value).map_err(|error| store::PackError::Schema(error.to_string()))
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        None
    }
}
//#endregion 🔹HandcraftedArtifactCodecs
