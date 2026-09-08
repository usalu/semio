//! 🧮️ Layout play app — view state (`LayoutConfig`) and its operation enum (`LayoutConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.layout` document. It still round-trips through a real
//! `ArtifactStore` (with a real `backwards`), so camera/drop-ghost edits are VCS'd exactly like
//! document content. Selection/hover moved OUT of this config into the framework-owned "elements"
//! interaction domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).

use crate::LayoutCamera;
use semio_framework_value_derive::{FromValue, ToValue};
pub use crate::LayoutDropPreviewState;
#[cfg(test)]
use protocol::Mutation;

//#region 🔖️Config
/// 🧮️ B1: layout's real `ArtifactApp::Config` — absorbs every field that used to live on
/// `layout_ui::LayoutPlayApp`'s `RefCell<LayoutPlayRuntime>` (active page, drop-ghost, engagement
/// draft, and the two independent blueprint/preview camera poses) plus `locale`, the one `ViewModel`
/// field the layout UI actually reads — session-only view state now round-trips through the config
/// `ArtifactStore` exactly like document content, with a real `backwards` per `LayoutConfigMutation`
/// instead of never being VCS'd at all.
#[derive(Clone, Debug, PartialEq, dsl::DslArtifact, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "layout.config")]
#[dsl(id = "layout.config")]
#[dsl(layout = "lines")]
pub struct LayoutConfig {
    /// 👁️ Active page shown/edited on the Blueprint surface — was `LayoutPlayRuntime::active_page_id`.
    pub active_page_id: String,
    /// 👁️ Live catalogue drag-ghost — was `LayoutPlayRuntime::drop_preview` (`Option<LayoutDropPreviewState>`).
    #[dsl(block)]
    pub drop_preview: LayoutDropPreviewState,
    /// 👁️ In-progress engagement-bar input draft — was `LayoutPlayRuntime::engagement_input`.
    pub engagement_input: String,
    /// 📷️ The Blueprint surface's ephemeral camera pose — was `LayoutPlayRuntime::camera`.
    #[dsl(block)]
    pub camera: LayoutCamera,
    /// 📷️ The Preview surface's ephemeral camera pose — was `LayoutPlayRuntime::preview_camera`.
    #[dsl(block)]
    pub preview_camera: LayoutCamera,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for LayoutConfig {
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
impl store::ArtifactPack for LayoutConfig {
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

impl Default for LayoutConfig {
    fn default() -> Self {
        Self { active_page_id: "page-1".into(), drop_preview: LayoutDropPreviewState::default(), engagement_input: String::new(), camera: LayoutCamera::default(), preview_camera: LayoutCamera::default(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(LayoutConfig);
//#endregion 🔖️Config

/// 🧬️ Physical mutation declarations for the layout.config channel.
#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn layout_config_default_matches_the_existing_runtime_defaults() {
        let config = LayoutConfig::default();
        assert_eq!(config.active_page_id, "page-1");
        assert_eq!(config.drop_preview, LayoutDropPreviewState::default());
        assert_eq!(config.camera, LayoutCamera::default());
        assert_eq!(config.preview_camera, LayoutCamera::default());
        assert_eq!(config.locale, "en-US");
    }

    #[semio_framework_async_macros::async_test]
    async fn layout_config_dsl_and_pack_round_trip() {
        let config = LayoutConfig {
            active_page_id: "page-2".into(),
            drop_preview: LayoutDropPreviewState { kind: "text".into(), x: 12.0, y: 34.0 },
            engagement_input: "export svg".into(),
            camera: LayoutCamera { x: 5.0, y: 6.0, zoom: 1.25 },
            preview_camera: LayoutCamera { x: 7.0, y: 8.0, zoom: 0.75 },
            locale: "de-DE".into(),
        };
        store::os_store::test_support::assert_dsl_round_trip(&config);
        store::os_store::test_support::assert_dsl_pack_equivalence(&config);
    }

    fn sample_config() -> LayoutConfig {
        LayoutConfig {
            active_page_id: "page-2".into(),
            drop_preview: LayoutDropPreviewState { kind: "rect".into(), x: 1.0, y: 2.0 },
            engagement_input: "export png".into(),
            camera: LayoutCamera { x: 10.0, y: 20.0, zoom: 1.5 },
            preview_camera: LayoutCamera { x: 3.0, y: 4.0, zoom: 2.0 },
            locale: "de-DE".into(),
        }
    }

    fn config_round_trip(base: &LayoutConfig, operation: &LayoutConfigMutation) -> LayoutConfig {
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
        let base = LayoutConfig::default();
        assert_eq!(config_round_trip(&base, &LayoutConfigMutation::SetActivePage(SetActivePage { page_id: "page-9".into() })).active_page_id, "page-9");
        let previewed = config_round_trip(&base, &LayoutConfigMutation::SetDropPreview(SetDropPreview { preview: LayoutDropPreviewState { kind: "rect".into(), x: 5.0, y: 6.0 } }));
        assert_eq!(previewed.drop_preview.kind, "rect");
        assert_eq!(config_round_trip(&base, &LayoutConfigMutation::SetEngagementInput(SetEngagementInput { value: "undo".into() })).engagement_input, "undo");
        let cam = config_round_trip(&base, &LayoutConfigMutation::SetCamera(SetCamera { camera: LayoutCamera { x: 1.0, y: 2.0, zoom: 3.0 } }));
        assert_eq!(cam.camera, LayoutCamera { x: 1.0, y: 2.0, zoom: 3.0 });
        let preview_cam = config_round_trip(&base, &LayoutConfigMutation::SetPreviewCamera(SetPreviewCamera { camera: LayoutCamera { x: 4.0, y: 5.0, zoom: 6.0 } }));
        assert_eq!(preview_cam.preview_camera, LayoutCamera { x: 4.0, y: 5.0, zoom: 6.0 });
        assert_eq!(config_round_trip(&base, &LayoutConfigMutation::SetLocale(SetLocale { value: "de-DE".into() })).locale, "de-DE");
    }

    #[semio_framework_async_macros::async_test]
    async fn config_snapshot_op_text_round_trips() {
        store::os_store::test_support::assert_op_line_round_trip(&LayoutConfigMutation::SetActivePage(SetActivePage { page_id: "page-2".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&LayoutConfigMutation::SetLocale(SetLocale { value: "en-US".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn config_mutation_inverses_restore_each_field_without_a_snapshot_sentinel() {
        let base = sample_config();
        assert_eq!(config_round_trip(&base, &LayoutConfigMutation::SetActivePage(SetActivePage { page_id: "page-9".into() })).active_page_id, "page-9");
        assert_eq!(config_round_trip(&base, &LayoutConfigMutation::SetLocale(SetLocale { value: "fr-FR".into() })).locale, "fr-FR");
    }
}
//#endregion 🧪️Tests

#[cfg(test)]
mod contract_vectors {
    use super::*;
    use protocol::{Mutation, MutationDiff, OpBinary, OpText};
    use dsl::os_pack as pack;

    #[test]
    fn layout_configuration_contract_vectors_match_the_json_oracle() {
        let vectors: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔁️mutation-contracts.json")).expect("neutral contract vectors");
        let base: LayoutConfig = pack::from_json_str(&vectors["base"].to_string()).expect("owned base decoder");
        assert_eq!(<LayoutConfigMutation as Mutation<LayoutConfig>>::DESCRIPTORS.len(), vectors["cases"].as_array().expect("cases").len());
        for vector in vectors["cases"].as_array().expect("cases") {
            let mutation: LayoutConfigMutation = pack::from_json_str(&vector["mutation"].to_string()).expect("owned operation decoder");
            assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&mutation)).expect("independent operation oracle"), vector["mutation"]);
            assert_eq!(mutation.descriptor().semantic_kind, vector["kind"].as_str().expect("semantic kind"));
            assert_eq!(LayoutConfigMutation::parse_op(&mutation.print_op()).expect("operation text"), mutation);
            assert_eq!(LayoutConfigMutation::decode_op(&mutation.encode_op().expect("operation binary")).expect("binary decode"), mutation);
            let outcome = mutation.diff(&base);
            assert!(outcome.messages().is_empty());
            let next = outcome.diff().apply(&base).expect("apply diff");
            assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&next)).expect("independent state oracle"), vector["expected"]);
            let restored = mutation.inverse(&base).into_iter().fold(next, |state, inverse| inverse.diff(&state).diff().apply(&state).expect("apply inverse"));
            assert_eq!(restored, base);
        }
    }
}
