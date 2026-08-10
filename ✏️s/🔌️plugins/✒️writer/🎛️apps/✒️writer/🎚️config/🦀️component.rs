//! 🧮️ Writer play app — view state (`WriterConfig`) and its operation enum (`WriterConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.writer` document. It still round-trips through a real
//! `ArtifactStore` (with a real `backwards`), so selection/hover/camera/editor-settings edits are VCS'd
//! exactly like document content. `WriterEditorSelection`/`WriterEditorSettings` were carried by the old
//! `⚙️engine` crate's `WriterConfig` before this migration — they move here alongside it, since neither
//! survives into the document either.

use crate::artifacts::writer::WriterCamera;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

pub use crate::artifacts::writer::{WriterEditorSelection, WriterEditorSettings};
/// 🧮️ B1: writer's real `ArtifactApp::Config` — absorbs every former `WriterPlayRuntime` app-struct
/// field (selection, editor selection, format/lint signals, revision, editor settings, AST hover,
/// engagement draft, and the session-only viewport camera — see `WriterCamera`'s doc comment) plus
/// `locale`, the one `ViewModel` field the writer UI actually reads (`resolve_labels`/`is_de_locale`
/// — see `crate::apps::writer::WriterPlayApp::render`), mirroring `shooting_engine::ShootingConfig`'s
/// B1 shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "writer.config")]
#[dsl(layout = "lines")]
pub struct WriterConfig {
    /// 👁️ Selected AST node ids — was `WriterPlayRuntime::selected_ast_ids`.
    pub selected_ast_ids: Vec<String>,
    /// 👁️ Editor text selection range — was `WriterPlayRuntime::editor_selection`.
    #[dsl(block)]
    pub editor_selection: Option<WriterEditorSelection>,
    /// 🔔️ Bumped on every format pass — was `WriterPlayRuntime::format_signal`.
    pub format_signal: u32,
    /// 🔔️ Bumped on every lint pass — was `WriterPlayRuntime::lint_signal`.
    pub lint_signal: u32,
    /// 🔔️ Bumped on every ephemeral view mutation — was `WriterPlayRuntime::revision`.
    pub revision: u32,
    /// ⚙️ Editor chrome settings (line numbers, font/line/tab size) — was `WriterPlayRuntime::editor_settings`.
    #[dsl(block)]
    pub editor_settings: WriterEditorSettings,
    /// 🐁️ AST node id whose tree row is hovered — was `WriterPlayRuntime::tree_hovered_ast_id`.
    pub tree_hovered_ast_id: Option<String>,
    /// 🐁️ Byte offset last reported as hovered by the editor surface — was `WriterPlayRuntime::editor_hover_offset`.
    pub editor_hover_offset: Option<usize>,
    /// 💬️ In-progress engagement-bar input draft — was `WriterPlayRuntime::engagement_input`.
    pub engagement_input: String,
    /// 🎥️ Editor viewport pan/zoom — session-only, never a document field. Was `WriterPlayRuntime::camera`.
    #[dsl(block)]
    pub camera: WriterCamera,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for WriterConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for WriterConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

//#endregion 🔖️ArtifactCodec


impl Default for WriterConfig {
    fn default() -> Self {
        Self {
            selected_ast_ids: Vec::new(),
            editor_selection: None,
            format_signal: 0,
            lint_signal: 0,
            revision: 0,
            editor_settings: WriterEditorSettings::default(),
            tree_hovered_ast_id: None,
            editor_hover_offset: None,
            engagement_input: String::new(),
            camera: WriterCamera::default(),
            locale: "en-US".into(),
        }
    }
}

store::impl_whole_record_config!(WriterConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// @emoji 🧮️ B1: `WriterConfig`'s operation enum — one variant per settled interaction (mirrors the
/// pre-B1 `WriterPlayRuntime` field writes), plus a generic `Snapshot` every variant's `backwards()`
/// returns — mirrors `shooting_op::ShootingConfigOperation` exactly (see that type's doc comment for the
/// whole-config-snapshot inverse rationale).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum WriterConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: WriterConfig,
    },
    #[dsl(key = "selected-ast-ids")]
    SetSelectedAstIds { ids: Vec<String> },
    #[dsl(key = "editor-selection")]
    SetEditorSelection {
        #[dsl(block)]
        selection: Option<WriterEditorSelection>,
    },
    #[dsl(key = "format-signal")]
    SetFormatSignal { value: u32 },
    #[dsl(key = "lint-signal")]
    SetLintSignal { value: u32 },
    #[dsl(key = "revision")]
    SetRevision { value: u32 },
    #[dsl(key = "editor-settings")]
    SetEditorSettings {
        #[dsl(block)]
        settings: WriterEditorSettings,
    },
    #[dsl(key = "tree-hovered-ast-id")]
    SetTreeHoveredAstId { id: Option<String> },
    #[dsl(key = "editor-hover-offset")]
    SetEditorHoverOffset { offset: Option<usize> },
    #[dsl(key = "engagement-input")]
    SetEngagementInput { value: String },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: WriterCamera,
    },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for WriterConfigMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for WriterConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 0,
            detail: format!("keyword {keyword:?} is not a declared variant"),
        })?;
        let spec = (variants[ordinal].1)();
        let body = store::pack_rt::encode_record_body(&spec, &record, &store::PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        store::pack_rt::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = store::pack_rt::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 1,
            detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()),
        })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed {
            what: "op record",
            offset: reader.position() as u64,
            detail: error.to_string(),
        })
    }
}

//#endregion 🔖️OpCodec


impl Mutation<WriterConfig> for WriterConfigMutation {
    type Diff = WriterConfig;

    fn diff(&self, base: &WriterConfig) -> WriterConfig {
        let mut next = base.clone();
        match self {
            WriterConfigMutation::Snapshot { config } => return config.clone(),
            WriterConfigMutation::SetSelectedAstIds { ids } => next.selected_ast_ids = ids.clone(),
            WriterConfigMutation::SetEditorSelection { selection } => next.editor_selection = selection.clone(),
            WriterConfigMutation::SetFormatSignal { value } => next.format_signal = *value,
            WriterConfigMutation::SetLintSignal { value } => next.lint_signal = *value,
            WriterConfigMutation::SetRevision { value } => next.revision = *value,
            WriterConfigMutation::SetEditorSettings { settings } => next.editor_settings = settings.clone(),
            WriterConfigMutation::SetTreeHoveredAstId { id } => next.tree_hovered_ast_id = id.clone(),
            WriterConfigMutation::SetEditorHoverOffset { offset } => next.editor_hover_offset = *offset,
            WriterConfigMutation::SetEngagementInput { value } => next.engagement_input = value.clone(),
            WriterConfigMutation::SetCamera { camera } => next.camera = camera.clone(),
            WriterConfigMutation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn inverse(&self, base: &WriterConfig) -> Vec<Self> {
        vec![WriterConfigMutation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writer_config_dsl_round_trips_default_and_populated() {
        store::os_store::test_support::assert_config_round_trip(&WriterConfig::default());
        let populated = WriterConfig {
            selected_ast_ids: vec!["jack-ast-1".into()],
            editor_selection: Some(WriterEditorSelection { start: 3, end: 7 }),
            format_signal: 2,
            lint_signal: 1,
            revision: 9,
            engagement_input: "format".into(),
            locale: "de-DE".into(),
            ..WriterConfig::default()
        };
        store::os_store::test_support::assert_config_round_trip(&populated);
    }

    #[test]
    fn writer_config_operation_backwards_restores_pre_state() {
        let pre = WriterConfig::default();
        store::os_store::test_support::assert_operation_round_trip(&pre, WriterConfigMutation::SetLocale { value: "de-DE".into() });
        store::os_store::test_support::assert_operation_round_trip(&pre, WriterConfigMutation::SetSelectedAstIds { ids: vec!["a".into()] });
        store::os_store::test_support::assert_operation_round_trip(&pre, WriterConfigMutation::SetCamera { camera: WriterCamera { x: 5.0, y: -2.0, zoom: 1.5 } });
    }

    #[test]
    fn writer_config_operation_binary_matches_text() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&WriterConfigMutation::SetLocale { value: "de-DE".into() });
        store::os_store::test_support::assert_op_text_binary_equivalence(&WriterConfigMutation::Snapshot { config: WriterConfig::default() });
    }

    #[test]
    fn writer_config_pack_round_trips() {
        let config = WriterConfig { locale: "de-DE".into(), engagement_input: "format".into(), ..WriterConfig::default() };
        let bytes = store::ArtifactPack::encode_pack(&config);
        let decoded = <WriterConfig as store::ArtifactPack>::decode_pack(&bytes).expect("decode writer config pack");
        assert_eq!(decoded, config);
    }
}
//#endregion 🧪️Tests
