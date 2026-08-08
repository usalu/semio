//! 🧬️ Flow snapshot schema — persistent fields only.

use flow::{CameraJson, SynapseSpec, Widget, WidgetLayout};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔹Snapshot
/// 📸️ Persisted flow document snapshot (persistent fields of the artifact).
///
/// Distinct from `flow::FlowFixture` in `semio-framework-os-flow`, which remains the framework
/// host/kernel document type. This plugin snapshot is isomorphic and converts at the host boundary.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.flow.flow")]
pub struct FlowSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub camera: CameraJson,
    #[state(persistent)]
    #[serde(default)]
    pub widgets: Vec<Widget>,
    #[state(persistent)]
    #[serde(default)]
    pub synapses: Vec<SynapseSpec>,
    #[state(persistent)]
    #[serde(default)]
    pub layout: BTreeMap<String, WidgetLayout>,
}
//#endregion 🔹Snapshot

//#region 🔹DefaultsAndBridge
impl Default for FlowSnapshot {
    fn default() -> Self {
        Self::from_fixture(flow::FlowFixture::default())
    }
}

impl FlowSnapshot {
    /// 🌊️ Builds a plugin snapshot from the framework `flow::FlowFixture` document type.
    pub fn from_fixture(fixture: flow::FlowFixture) -> Self {
        Self {
            schema: fixture.schema,
            camera: fixture.camera,
            widgets: fixture.widgets,
            synapses: fixture.synapses,
            layout: fixture.layout,
        }
    }

    /// 🌊️ Converts this snapshot into the framework `flow::FlowFixture` for `FlowHost` / kernel codecs.
    pub fn to_fixture(&self) -> flow::FlowFixture {
        flow::FlowFixture {
            schema: self.schema.clone(),
            camera: self.camera.clone(),
            widgets: self.widgets.clone(),
            synapses: self.synapses.clone(),
            layout: self.layout.clone(),
        }
    }
}

impl From<flow::FlowFixture> for FlowSnapshot {
    fn from(fixture: flow::FlowFixture) -> Self {
        Self::from_fixture(fixture)
    }
}

impl From<FlowSnapshot> for flow::FlowFixture {
    fn from(snapshot: FlowSnapshot) -> Self {
        snapshot.to_fixture()
    }
}
//#endregion 🔹DefaultsAndBridge

//#region 🔹HandcraftedDocumentCodecs
/// ✉️ DocumentDsl — JSON body under envelope id `flow.flow`.
///
/// Does not call `flow::FlowFixture`'s codecs: that framework type still emits envelope id `flow`,
/// which `SemioEnvelope::from_envelope_id` rejects (`plugin.artifact` required). Fixup belongs in
/// `semio-framework-os-flow`; this plugin snapshot owns a valid envelope of its own.
impl store::DocumentDsl for FlowSnapshot {
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
            return serde_json::from_str(trimmed).map_err(|error| {
                store::TextError::new(error.to_string(), store::TextSpan::at(1, 1))
            });
        }
        <flow::FlowFixture as store::DocumentDsl>::parse_dsl(text).map(Self::from_fixture)
    }
    fn print_dsl(&self) -> String {
        let body = serde_json::to_string_pretty(self).expect("FlowSnapshot serde");
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ DocumentPack — JSON body under envelope id `flow.flow` (see DocumentDsl note).
impl store::DocumentPack for FlowSnapshot {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let body = serde_json::to_vec(self).map_err(|error| store::PackError::Schema(error.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        let our_id = <Self as store::DocumentDsl>::envelope_id();
        if envelope.envelope_id() != our_id {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {our_id}, got {}",
                envelope.envelope_id()
            )));
        }
        serde_json::from_slice(&body).map_err(|error| store::PackError::Schema(error.to_string()))
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        None
    }
}
//#endregion 🔹HandcraftedDocumentCodecs
