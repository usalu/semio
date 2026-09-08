//! 🧮️ Sequence play app — view state (`SequenceConfig`) and its operation enum
//! (`SequenceConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/`
//! because nothing in it survives into the `.sequence` document. It still round-trips through a real
//! `ArtifactStore` (with a real `backwards`), so selection/camera/orientation edits are VCS'd exactly
//! like document content.

use crate::SequenceCamera;
#[cfg(test)]
use protocol::Mutation;
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🧮️ B1: sequence's real `ArtifactApp::Config` — absorbs every former `SequencePlayRuntime` field
/// (`last_run_json`/`orientation`) plus the node-graph viewport camera (session-only, never a document
/// field) and the locale the pre-B1 host-pushed `ViewModel` used to carry (see
/// `crate::editor::sequence::terminology::sequence_play_labels`) — same "absorb every runtime field"
/// shape `shooting_engine::ShootingConfig` established for the pilot. 🕹️ ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `selected_step_ids` no longer lives here —
/// selection is framework-owned now, read via `InteractionView::selection("steps")`.
#[derive(Clone, Debug, PartialEq, dsl::DslArtifact)]
#[derive(dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "sequencecfg")]
#[dsl(id = "sequence.config")]
#[dsl(layout = "lines")]
pub struct SequenceConfig {
    /// 🏃️ Last `run` command's `RunResult` JSON, rendered under the compiled script — was
    /// `SequencePlayRuntime::last_run_json`.
    pub last_run_json: String,
    /// 🌳️ Layered-layout flow direction (`"leftRight"`/`"topBottom"`) `reorganize` reads — was
    /// `SequencePlayRuntime::orientation`. Kept as a string rather than `DagLayoutOrientation`
    /// directly: that enum is foreign to this crate and only derives `Serialize`/`Deserialize`, not
    /// `dsl::DslField` (see `crate::editor::sequence::commands::layout`'s conversion helper).
    pub orientation: String,
    /// 🎥️ The node-graph viewport pan/zoom — session-only, never a document field. Was
    /// `SequencePlayRuntime::camera`.
    #[dsl(block)]
    pub camera: SequenceCamera,
    /// 🗣️ BCP-47 locale tag — was read off the host-pushed `ViewModel.locale`.
    pub locale: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for SequenceConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for SequenceConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

//#endregion 🔖️ArtifactCodec

impl Default for SequenceConfig {
    fn default() -> Self {
        Self { last_run_json: String::new(), orientation: "leftRight".into(), camera: SequenceCamera::default(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(SequenceConfig);
//#endregion 🔖️Config

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;


//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn sequence_config_default_matches_the_existing_runtime_defaults() {
        let config = SequenceConfig::default();
        assert!(config.last_run_json.is_empty());
        assert_eq!(config.orientation, "leftRight");
        assert_eq!(config.locale, "en-US");
    }

    #[semio_framework_async_macros::async_test]
    async fn sequence_config_dsl_round_trips() {
        let config = SequenceConfig { last_run_json: "{}".into(), orientation: "topBottom".into(), camera: SequenceCamera { x: 1.0, y: 2.0, zoom: 3.0 }, locale: "de-DE".into() };
        let text = store::ArtifactDsl::print_dsl(&config);
        let parsed = <SequenceConfig as store::ArtifactDsl>::parse_dsl(&text).expect("config dsl round trip");
        assert_eq!(parsed, config);
    }

    #[semio_framework_async_macros::async_test]
    async fn sequence_config_pack_round_trips() {
        let config = SequenceConfig { last_run_json: "{\"ok\":true}".into(), orientation: "leftRight".into(), camera: SequenceCamera::default(), locale: "en-US".into() };
        let bytes = store::ArtifactPack::encode_pack(&config);
        let decoded = <SequenceConfig as store::ArtifactPack>::decode_pack(&bytes).expect("config pack round trip");
        assert_eq!(decoded, config);
    }

    //#region 🔖️ConfigMutationTests
    fn round_trip_config(config: &SequenceConfig, operation: &SequenceConfigMutation) -> SequenceConfig {
        let forward = operation.diff(config).diff().clone();
        let backwards = operation.inverse(config);
        assert_eq!(backwards.len(), 1);
        let restored = backwards[0].diff(&forward).diff().clone();
        assert_eq!(&restored, config, "backwards() must exactly restore the pre-operation config");
        forward
    }

    #[semio_framework_async_macros::async_test]
    async fn config_set_last_run_round_trips() {
        let config = SequenceConfig::default();
        let next = round_trip_config(&config, &SequenceConfigMutation::SetLastRun(SetLastRun { json: "{\"ok\":true}".into() }));
        assert_eq!(next.last_run_json, "{\"ok\":true}");
    }

    #[semio_framework_async_macros::async_test]
    async fn config_set_orientation_round_trips() {
        let config = SequenceConfig::default();
        let next = round_trip_config(&config, &SequenceConfigMutation::SetOrientation(SetOrientation { value: "topBottom".into() }));
        assert_eq!(next.orientation, "topBottom");
    }

    #[semio_framework_async_macros::async_test]
    async fn config_set_camera_round_trips() {
        let config = SequenceConfig::default();
        let camera = SequenceCamera { x: 5.0, y: 6.0, zoom: 2.0 };
        let next = round_trip_config(&config, &SequenceConfigMutation::SetCamera(SetCamera { camera: camera.clone() }));
        assert_eq!(next.camera, camera);
    }

    #[semio_framework_async_macros::async_test]
    async fn config_set_locale_round_trips() {
        let config = SequenceConfig::default();
        let next = round_trip_config(&config, &SequenceConfigMutation::SetLocale(SetLocale { value: "de-DE".into() }));
        assert_eq!(next.locale, "de-DE");
    }

    #[semio_framework_async_macros::async_test]
    async fn config_op_text_round_trips_every_variant() {
        store::os_store::test_support::assert_op_line_round_trip(&SequenceConfigMutation::SetLastRun(SetLastRun { json: "{}".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&SequenceConfigMutation::SetOrientation(SetOrientation { value: "leftRight".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&SequenceConfigMutation::SetCamera(SetCamera { camera: SequenceCamera { x: 1.0, y: 2.0, zoom: 3.0 } }));
        store::os_store::test_support::assert_op_line_round_trip(&SequenceConfigMutation::SetLocale(SetLocale { value: "en-US".into() }));
    }
    //#endregion 🔖️ConfigMutationTests
}
//#endregion 🧪️Tests

#[cfg(test)]
mod contract_vectors {
    use super::*;
    use protocol::{Mutation, MutationDiff, OpBinary, OpText};
    use dsl::os_pack as pack;

    #[test]
    fn sequence_configuration_contract_vectors_match_the_json_oracle() {
        let vectors: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔁️mutation-contracts.json")).expect("neutral contract vectors");
        let base: SequenceConfig = pack::from_json_str(&vectors["base"].to_string()).expect("owned base decoder");
        assert_eq!(<SequenceConfigMutation as Mutation<SequenceConfig>>::DESCRIPTORS.len(), vectors["cases"].as_array().expect("cases").len());
        for vector in vectors["cases"].as_array().expect("cases") {
            let mutation: SequenceConfigMutation = pack::from_json_str(&vector["mutation"].to_string()).expect("owned operation decoder");
            assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&mutation)).expect("independent operation oracle"), vector["mutation"]);
            assert_eq!(mutation.descriptor().semantic_kind, vector["kind"].as_str().expect("semantic kind"));
            assert_eq!(SequenceConfigMutation::parse_op(&mutation.print_op()).expect("operation text"), mutation);
            assert_eq!(SequenceConfigMutation::decode_op(&mutation.encode_op().expect("operation binary")).expect("binary decode"), mutation);
            let outcome = mutation.diff(&base);
            assert!(outcome.messages().is_empty());
            let next = outcome.diff().apply(&base).expect("apply diff");
            assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&next)).expect("independent state oracle"), vector["expected"]);
            let restored = mutation.inverse(&base).into_iter().fold(next, |state, inverse| inverse.diff(&state).diff().apply(&state).expect("apply inverse"));
            assert_eq!(restored, base);
        }
    }
}
