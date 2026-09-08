//! 🧮️ Playbook play app — view state (`PlaybookConfig`) and its operation enum (`PlaybookConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.playbook` document. It still round-trips through a real
//! `ArtifactStore` (with a real `backwards`). B1: absorbs `locale` (was read off `view_state.locale`) —
//! mirrors `writer_engine::WriterConfig`/`forms::config::FormsConfig`'s B1 shape. 🕹️ ticket
//! 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the former app-struct `RefCell<Vec<String>>`
//! selection state that B1 had absorbed here as `selected_ids` moved OUT again, into the framework's own
//! `InteractionState` (the "blocks" domain, declared on `PlaybookPlayApp`'s manifest) — this config no
//! longer carries any selection.

#[cfg(test)]
use protocol::Mutation;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Config
/// 🧮️ `PlaybookPlayApp::Config` — the pure-trait `ArtifactEditor::Config` for the playbook app.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "playbookcfg")]
#[dsl(layout = "lines")]
pub struct PlaybookConfig {
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
    /// 🧩️ Host-pushed `ProgramContributionEntry[]` JSON for `playbook.blockKind` hot-swap installs.
    #[value(default = "default_contributions_json")]
    pub contributions_json: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for PlaybookConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        "playbook.config"
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
impl store::ArtifactPack for PlaybookConfig {
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

fn default_contributions_json() -> String {
    "[]".into()
}

impl Default for PlaybookConfig {
    fn default() -> Self {
        Self { locale: "en-US".into(), contributions_json: default_contributions_json() }
    }
}

store::impl_whole_record_config!(PlaybookConfig);
//#endregion 🔖️Config

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn playbook_config_default_matches_the_existing_runtime_defaults() {
        let config = PlaybookConfig::default();
        assert_eq!(config.locale, "en-US");
    }

    #[semio_framework_async_macros::async_test]
    async fn playbook_config_dsl_round_trips_default_and_populated() {
        store::os_store::test_support::assert_config_round_trip(&PlaybookConfig::default());
        let populated = PlaybookConfig { locale: "de-DE".into(), contributions_json: "[]".into() };
        store::os_store::test_support::assert_config_round_trip(&populated);
    }

    #[semio_framework_async_macros::async_test]
    async fn playbook_config_pack_round_trips() {
        let config = PlaybookConfig { locale: "de-DE".into(), contributions_json: "[]".into() };
        let bytes = store::ArtifactPack::encode_pack(&config);
        let decoded = <PlaybookConfig as store::ArtifactPack>::decode_pack(&bytes).expect("decode playbook config pack");
        assert_eq!(decoded, config);
    }

    fn config_round_trip(base: &PlaybookConfig, operation: &PlaybookConfigMutation) -> PlaybookConfig {
        let forward = operation.diff(base).diff().clone();
        let backwards = operation.inverse(base);
        let mut restored = forward.clone();
        for back in &backwards {
            restored = back.diff(&restored).diff().clone();
        }
        assert_eq!(&restored, base, "backwards() must exactly restore the pre-operation config");
        forward
    }

    #[semio_framework_async_macros::async_test]
    async fn config_mutations_apply_and_restore_every_field() {
        let base = PlaybookConfig::default();
        assert_eq!(config_round_trip(&base, &PlaybookConfigMutation::SetLocale(SetLocale { value: "de-DE".into() })).locale, "de-DE");
        assert_eq!(config_round_trip(&base, &PlaybookConfigMutation::SetContributions(SetContributions { json: "[]".into() })).contributions_json, "[]");
    }

    #[semio_framework_async_macros::async_test]
    async fn playbook_config_operation_binary_matches_text() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&PlaybookConfigMutation::SetLocale(SetLocale { value: "de-DE".into() }));
        store::os_store::test_support::assert_op_text_binary_equivalence(&PlaybookConfigMutation::ReplaceConfig(ReplaceConfig { config: PlaybookConfig::default() }));
    }
}
//#endregion 🧪️Tests

#[cfg(test)]
mod contract_vectors {
    use super::*;
    use dsl::os_pack as pack;
    use protocol::{Mutation, MutationDiff, OpBinary, OpText};

    #[test]
    fn configuration_and_presence_contract_vectors_match_the_json_oracle() {
        let vectors: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔁️mutation-contracts.json")).unwrap();
        let base: PlaybookConfig = pack::from_json_str(&vectors["base"].to_string()).unwrap();
        assert_eq!(<PlaybookConfigMutation as Mutation<PlaybookConfig>>::DESCRIPTORS.len(), vectors["cases"].as_array().unwrap().len());
        for vector in vectors["cases"].as_array().unwrap() {
            let mutation: PlaybookConfigMutation = pack::from_json_str(&vector["mutation"].to_string()).unwrap();
            assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&mutation)).unwrap(), vector["mutation"]);
            assert_eq!(mutation.descriptor().semantic_kind, vector["kind"].as_str().unwrap());
            let next = mutation.diff(&base).diff().apply(&base).unwrap();
            assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&next)).unwrap(), vector["expected"]);
            assert_eq!(PlaybookConfigMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
            assert_eq!(PlaybookConfigMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
            let restored = mutation.inverse(&base).into_iter().fold(next, |state, inverse| inverse.diff(&state).diff().apply(&state).unwrap());
            assert_eq!(restored, base);
        }
    }
}
