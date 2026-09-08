//! 🧮️ Animate presentation app — view state (`PresentationConfig`) and its operation enum
//! (`PresentationConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/`
//! because nothing in it survives into the `.presentation` document. It still round-trips through a real
//! `ArtifactStore` (with a real `backwards`), so selection/engagement/locale edits are VCS'd exactly
//! like document content.

#[cfg(test)]
use protocol::Mutation;

//#region 🔖️Config
/// 🧮️ B1: animate presentation's real `ArtifactApp::Config` — absorbs every former
/// `AnimatePresentationPlayRuntime` field (`selected_ids`/`engagement_input`) plus the locale the pre-B1
/// host-pushed `ViewModel` used to carry (see `crate::editor::animate::terminology`) — same "absorb every
/// runtime field" shape `shooting_engine::ShootingConfig` established for the pilot.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "presentcfg")]
#[dsl(id = "presentation.config")]
#[dsl(layout = "lines")]
pub struct PresentationConfig {
    /// ⌨️ In-progress engagement-bar input draft — was `AnimatePresentationPlayRuntime::engagement_input`.
    pub engagement_input: String,
    /// 🗣️ BCP-47 locale tag — was read off the host-pushed `ViewModel.locale`.
    pub locale: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for PresentationConfig {
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
impl store::ArtifactPack for PresentationConfig {
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

impl Default for PresentationConfig {
    fn default() -> Self {
        Self { engagement_input: String::new(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(PresentationConfig);
//#endregion 🔖️Config

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presentation_config_default_matches_the_existing_runtime_defaults() {
        let config = PresentationConfig::default();
        assert!(config.engagement_input.is_empty());
        assert_eq!(config.locale, "en-US");
    }

    #[test]
    fn presentation_config_dsl_round_trips() {
        let config = PresentationConfig { engagement_input: "2x2".into(), locale: "de-DE".into() };
        let text = store::ArtifactDsl::print_dsl(&config);
        let parsed = <PresentationConfig as store::ArtifactDsl>::parse_dsl(&text).expect("config dsl round trip");
        assert_eq!(parsed, config);
    }

    #[test]
    fn presentation_config_pack_round_trips() {
        let config = PresentationConfig { engagement_input: "add".into(), locale: "en-US".into() };
        let bytes = store::ArtifactPack::encode_pack(&config);
        let decoded = <PresentationConfig as store::ArtifactPack>::decode_pack(&bytes).expect("config pack round trip");
        assert_eq!(decoded, config);
    }

    //#region 🔖️ConfigMutationTests
    fn round_trip_config(config: &PresentationConfig, operation: &PresentationConfigMutation) -> PresentationConfig {
        let forward = operation.diff(config).diff().clone();
        let backwards = operation.inverse(config);
        assert_eq!(backwards.len(), 1);
        let restored = backwards[0].diff(&forward).diff().clone();
        assert_eq!(&restored, config, "backwards() must exactly restore the pre-operation config");
        forward
    }

    #[test]
    fn config_set_engagement_input_round_trips() {
        let config = PresentationConfig::default();
        let next = round_trip_config(&config, &PresentationConfigMutation::SetEngagementInput(SetEngagementInput { value: "2x2".into() }));
        assert_eq!(next.engagement_input, "2x2");
    }

    #[test]
    fn config_set_locale_round_trips() {
        let config = PresentationConfig::default();
        let next = round_trip_config(&config, &PresentationConfigMutation::SetLocale(SetLocale { value: "de-DE".into() }));
        assert_eq!(next.locale, "de-DE");
    }

    #[test]
    fn config_op_text_round_trips_every_variant() {
        store::os_store::test_support::assert_op_line_round_trip(&PresentationConfigMutation::SetEngagementInput(SetEngagementInput { value: "add".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&PresentationConfigMutation::SetLocale(SetLocale { value: "en-US".into() }));
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
    fn presentation_configuration_contract_vectors_match_the_json_oracle() {
        let vectors: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔁️mutation-contracts.json")).expect("neutral contract vectors");
        let base: PresentationConfig = pack::from_json_str(&vectors["base"].to_string()).expect("owned base decoder");
        assert_eq!(<PresentationConfigMutation as Mutation<PresentationConfig>>::DESCRIPTORS.len(), vectors["cases"].as_array().expect("cases").len());
        for vector in vectors["cases"].as_array().expect("cases") {
            let mutation: PresentationConfigMutation = pack::from_json_str(&vector["mutation"].to_string()).expect("owned operation decoder");
            assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&mutation)).expect("independent operation oracle"), vector["mutation"]);
            assert_eq!(mutation.descriptor().semantic_kind, vector["kind"].as_str().expect("semantic kind"));
            assert_eq!(PresentationConfigMutation::parse_op(&mutation.print_op()).expect("operation text"), mutation);
            assert_eq!(PresentationConfigMutation::decode_op(&mutation.encode_op().expect("operation binary")).expect("binary decode"), mutation);
            let outcome = mutation.diff(&base);
            assert!(outcome.messages().is_empty());
            let next = outcome.diff().apply(&base).expect("apply diff");
            assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&next)).expect("independent state oracle"), vector["expected"]);
            let restored = mutation.inverse(&base).into_iter().fold(next, |state, inverse| inverse.diff(&state).diff().apply(&state).expect("apply inverse"));
            assert_eq!(restored, base);
        }
    }
}
