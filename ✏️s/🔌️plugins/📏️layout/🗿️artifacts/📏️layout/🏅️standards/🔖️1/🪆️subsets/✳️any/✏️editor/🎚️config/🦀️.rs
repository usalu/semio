//! 🧮️ Layout play app — view state (`LayoutConfig`) and its operation enum (`LayoutConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.layout` document. It still round-trips through a real
//! `ArtifactStore` (with a real `backwards`), so camera/drop-ghost edits are VCS'd exactly like
//! document content. Selection/hover moved OUT of this config into the framework-owned "elements"
//! interaction domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).

use crate::artifacts::layout::LayoutCamera;
use semio_framework_value_derive::{FromValue, ToValue};
pub use crate::artifacts::layout::LayoutDropPreviewState;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

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
    async fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    async fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for LayoutConfig {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    async fn record_spec() -> Option<dsl::RecordSpec> {
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

//#region 🔖️ConfigMutations
/// 🧮️ [`LayoutConfig`]'s operation enum — one variant per settled interaction; each variant's
/// `backwards()` re-emits the SAME variant with the old field value read from `base` (no
/// whole-config snapshot sentinel). `Mutation::Diff` is the WHOLE `LayoutConfig` (not a granular
/// patch type): `diff()` returns "the full config after this op", and
/// `store::impl_whole_record_config!` supplies the `MutationDiff<LayoutConfig>` that accepts that
/// snapshot as a successful replacement, ignoring `base`.
#[derive(Clone, Debug, PartialEq, dsl::DslOps, ToValue, FromValue)]
pub enum LayoutConfigMutation {
    #[dsl(key = "active-page")]
    SetActivePage { page_id: String },
    #[dsl(key = "drop-preview")]
    SetDropPreview {
        #[dsl(block)]
        preview: LayoutDropPreviewState,
    },
    #[dsl(key = "engagement-input")]
    SetEngagementInput { value: String },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: LayoutCamera,
    },
    #[dsl(key = "preview-camera")]
    SetPreviewCamera {
        #[dsl(block)]
        camera: LayoutCamera,
    },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for LayoutConfigMutation {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    async fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for LayoutConfigMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1)();
        let body = store::pack_rt::encode_record_body(&spec, &record, &store::PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        store::pack_rt::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = store::pack_rt::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed { what: "op record", offset: reader.position() as u64, detail: error.to_string() })
    }
}

//#endregion 🔖️OpCodec

impl Mutation<LayoutConfig> for LayoutConfigMutation {
    type Diff = LayoutConfig;

    async fn diff(&self, base: &LayoutConfig) -> protocol::MutationOutcome<LayoutConfig> {
        let mut next = base.clone();
        match self {
            LayoutConfigMutation::SetActivePage { page_id } => next.active_page_id = page_id.clone(),
            LayoutConfigMutation::SetDropPreview { preview } => next.drop_preview = preview.clone(),
            LayoutConfigMutation::SetEngagementInput { value } => next.engagement_input = value.clone(),
            LayoutConfigMutation::SetCamera { camera } => next.camera = camera.clone(),
            LayoutConfigMutation::SetPreviewCamera { camera } => next.preview_camera = camera.clone(),
            LayoutConfigMutation::SetLocale { value } => next.locale = value.clone(),
        }
        protocol::MutationOutcome::new(next)
    }

    async fn inverse(&self, base: &LayoutConfig) -> Vec<Self> {
        match self {
            LayoutConfigMutation::SetActivePage { .. } => vec![LayoutConfigMutation::SetActivePage { page_id: base.active_page_id.clone() }],
            LayoutConfigMutation::SetDropPreview { .. } => vec![LayoutConfigMutation::SetDropPreview { preview: base.drop_preview.clone() }],
            LayoutConfigMutation::SetEngagementInput { .. } => vec![LayoutConfigMutation::SetEngagementInput { value: base.engagement_input.clone() }],
            LayoutConfigMutation::SetCamera { .. } => vec![LayoutConfigMutation::SetCamera { camera: base.camera.clone() }],
            LayoutConfigMutation::SetPreviewCamera { .. } => vec![LayoutConfigMutation::SetPreviewCamera { camera: base.preview_camera.clone() }],
            LayoutConfigMutation::SetLocale { .. } => vec![LayoutConfigMutation::SetLocale { value: base.locale.clone() }],
        }
    }
}
//#endregion 🔖️ConfigMutations

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

    async fn sample_config() -> LayoutConfig {
        LayoutConfig {
            active_page_id: "page-2".into(),
            drop_preview: LayoutDropPreviewState { kind: "rect".into(), x: 1.0, y: 2.0 },
            engagement_input: "export png".into(),
            camera: LayoutCamera { x: 10.0, y: 20.0, zoom: 1.5 },
            preview_camera: LayoutCamera { x: 3.0, y: 4.0, zoom: 2.0 },
            locale: "de-DE".into(),
        }
    }

    async fn config_round_trip(base: &LayoutConfig, operation: &LayoutConfigMutation) -> LayoutConfig {
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
        assert_eq!(config_round_trip(&base, &LayoutConfigMutation::SetActivePage { page_id: "page-9".into() }).active_page_id, "page-9");
        let previewed = config_round_trip(&base, &LayoutConfigMutation::SetDropPreview { preview: LayoutDropPreviewState { kind: "rect".into(), x: 5.0, y: 6.0 } });
        assert_eq!(previewed.drop_preview.kind, "rect");
        assert_eq!(config_round_trip(&base, &LayoutConfigMutation::SetEngagementInput { value: "undo".into() }).engagement_input, "undo");
        let cam = config_round_trip(&base, &LayoutConfigMutation::SetCamera { camera: LayoutCamera { x: 1.0, y: 2.0, zoom: 3.0 } });
        assert_eq!(cam.camera, LayoutCamera { x: 1.0, y: 2.0, zoom: 3.0 });
        let preview_cam = config_round_trip(&base, &LayoutConfigMutation::SetPreviewCamera { camera: LayoutCamera { x: 4.0, y: 5.0, zoom: 6.0 } });
        assert_eq!(preview_cam.preview_camera, LayoutCamera { x: 4.0, y: 5.0, zoom: 6.0 });
        assert_eq!(config_round_trip(&base, &LayoutConfigMutation::SetLocale { value: "de-DE".into() }).locale, "de-DE");
    }

    #[semio_framework_async_macros::async_test]
    async fn config_snapshot_op_text_round_trips() {
        store::os_store::test_support::assert_op_line_round_trip(&LayoutConfigMutation::SetActivePage { page_id: "page-2".into() });
        store::os_store::test_support::assert_op_line_round_trip(&LayoutConfigMutation::SetLocale { value: "en-US".into() });
    }

    #[semio_framework_async_macros::async_test]
    async fn config_mutation_inverses_restore_each_field_without_a_snapshot_sentinel() {
        let base = sample_config();
        assert_eq!(config_round_trip(&base, &LayoutConfigMutation::SetActivePage { page_id: "page-9".into() }).active_page_id, "page-9");
        assert_eq!(config_round_trip(&base, &LayoutConfigMutation::SetLocale { value: "fr-FR".into() }).locale, "fr-FR");
    }
}
//#endregion 🧪️Tests
