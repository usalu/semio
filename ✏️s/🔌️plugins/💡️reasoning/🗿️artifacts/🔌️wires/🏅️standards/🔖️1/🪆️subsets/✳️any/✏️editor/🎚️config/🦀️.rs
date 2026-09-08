//! 🧮️ Wires play app — view state (`WiresConfig`) and its operation enum (`WiresConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.wires` document. It still round-trips through a real
//! `ArtifactStore` (with a real `backwards`), so selection/drag/locale edits are VCS'd exactly like
//! document content. Absorbs everything that used to live in the pre-B1 `ReasoningWiresPlayApp`'s
//! ephemeral `WiresPlayRuntime` (selection + in-flight pointer drag of one board node) plus the `locale`
//! the deleted `ViewModel` used to carry.

#[cfg(test)]
use protocol::Mutation;

//#region 🔖️Config
/// 🧮️ `ReasoningWiresPlayApp::Config` — the pure-trait `ArtifactApp::Config` for the wires app.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "reasoning.wirescfg")]
#[dsl(id = "wires.config")]
#[dsl(layout = "lines")]
pub struct WiresConfig {
    /// 🖱️ In-flight pointer-drag target node id — was `WiresDragState::node_id`
    /// (`WiresPlayRuntime::drag`); `None` means no drag is in progress.
    pub drag_node_id: Option<String>,
    /// 🖱️ Last observed drag pointer X (screen space) — was `WiresDragState::last_x`.
    pub drag_last_x: f64,
    /// 🖱️ Last observed drag pointer Y (screen space) — was `WiresDragState::last_y`.
    pub drag_last_y: f64,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for WiresConfig {
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
impl store::ArtifactPack for WiresConfig {
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

impl Default for WiresConfig {
    fn default() -> Self {
        Self { drag_node_id: None, drag_last_x: 0.0, drag_last_y: 0.0, locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(WiresConfig);
//#endregion 🔖️Config

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🔖️ConfigTests
    /// 🕹️ Selection lives in the framework-owned "graph" interaction domain now (ticket
    /// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — `WiresConfig` only carries drag/locale.
    #[semio_framework_async_macros::async_test]
    async fn wires_config_default_matches_no_drag_and_en_locale() {
        let config = WiresConfig::default();
        assert!(config.drag_node_id.is_none());
        assert_eq!(config.locale, "en-US");
    }

    /// 🔁️ B1 dsl/pack round-trip law for `WiresConfig` — a non-default fixture exercising every field.
    #[semio_framework_async_macros::async_test]
    async fn wires_config_dsl_pack_round_trip() {
        let config = WiresConfig { drag_node_id: Some("node-1".into()), drag_last_x: 12.5, drag_last_y: -7.25, locale: "de-DE".into() };
        store::os_store::test_support::assert_dsl_pack_equivalence(&config);
    }
    //#endregion 🔖️ConfigTests

    //#region 🔖️ConfigOperationTests
    #[semio_framework_async_macros::async_test]
    async fn config_drag_op_text_round_trip() {
        store::os_store::test_support::assert_op_line_round_trip(&WiresConfigMutation::SetDrag(SetDrag { node_id: Some("node-1".into()), last_x: 12.5, last_y: -7.25 }));
        store::os_store::test_support::assert_op_line_round_trip(&WiresConfigMutation::SetDrag(SetDrag { node_id: None, last_x: 0.0, last_y: 0.0 }));
    }

    #[semio_framework_async_macros::async_test]
    async fn config_locale_op_text_round_trip() {
        store::os_store::test_support::assert_op_line_round_trip(&WiresConfigMutation::SetLocale(SetLocale { value: "de-DE".into() }));
    }

    /// ⏪️ `backwards()` returns the SAME variant re-addressed at the pre-op field value — a targeted,
    /// in-kind inverse, not a whole-config replace.
    #[semio_framework_async_macros::async_test]
    async fn config_backwards_restores_the_same_field_from_base() {
        let base = WiresConfig { drag_node_id: Some("node-1".into()), drag_last_x: 1.0, drag_last_y: 2.0, ..Default::default() };
        let forward = WiresConfigMutation::SetDrag(SetDrag { node_id: Some("node-2".into()), last_x: 5.0, last_y: 6.0 });
        let inverse = forward.inverse(&base);
        assert_eq!(inverse, vec![WiresConfigMutation::SetDrag(SetDrag { node_id: base.drag_node_id.clone(), last_x: base.drag_last_x, last_y: base.drag_last_y })]);
        assert_eq!(forward.diff(&base).diff().clone(), WiresConfig { drag_node_id: Some("node-2".into()), drag_last_x: 5.0, drag_last_y: 6.0, ..base });
    }
    //#endregion 🔖️ConfigOperationTests
}
//#endregion 🧪️Tests

#[cfg(test)]
mod contract_vectors {
    use super::*;
    use protocol::{Mutation, MutationDiff, OpBinary, OpText};
    use dsl::os_pack as pack;

    #[test]
    fn wires_configuration_contract_vectors_match_the_json_oracle() {
        let vectors: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔁️mutation-contracts.json")).expect("neutral contract vectors");
        let base: WiresConfig = pack::from_json_str(&vectors["base"].to_string()).expect("owned base decoder");
        assert_eq!(<WiresConfigMutation as Mutation<WiresConfig>>::DESCRIPTORS.len(), vectors["cases"].as_array().expect("cases").len());
        for vector in vectors["cases"].as_array().expect("cases") {
            let mutation: WiresConfigMutation = pack::from_json_str(&vector["mutation"].to_string()).expect("owned operation decoder");
            assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&mutation)).expect("independent operation oracle"), vector["mutation"]);
            assert_eq!(mutation.descriptor().semantic_kind, vector["kind"].as_str().expect("semantic kind"));
            assert_eq!(WiresConfigMutation::parse_op(&mutation.print_op()).expect("operation text"), mutation);
            assert_eq!(WiresConfigMutation::decode_op(&mutation.encode_op().expect("operation binary")).expect("binary decode"), mutation);
            let outcome = mutation.diff(&base);
            assert!(outcome.messages().is_empty());
            let next = outcome.diff().apply(&base).expect("apply diff");
            assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&next)).expect("independent state oracle"), vector["expected"]);
            let restored = mutation.inverse(&base).into_iter().fold(next, |state, inverse| inverse.diff(&state).diff().apply(&state).expect("apply inverse"));
            assert_eq!(restored, base);
        }
    }
}
