//! 🧮️ Writer play app — view state (`WriterConfig`) and its operation enum (`WriterConfigOperation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.writer` document. It still round-trips through a real
//! `DocumentStore` (with a real `backwards`), so selection/hover/camera/editor-settings edits are VCS'd
//! exactly like document content. `WriterEditorSelection`/`WriterEditorSettings` were carried by the old
//! `⚙️engine` crate's `WriterConfig` before this migration — they move here alongside it, since neither
//! survives into the document either.

use crate::artifacts::writer::WriterCamera;
use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Types
/// 📐️ Editor text selection range.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct WriterEditorSelection {
    pub start: usize,
    pub end: usize,
}

/// ⚙️ Editor chrome settings (line numbers, font/line/tab size).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct WriterEditorSettings {
    pub show_line_numbers: bool,
    pub font_px: u32,
    pub line_height: u32,
    pub tab_size: u32,
}

impl Default for WriterEditorSettings {
    fn default() -> Self {
        Self { show_line_numbers: true, font_px: 14, line_height: 22, tab_size: 2 }
    }
}
//#endregion 🔖️Types

//#region 🔖️Config
/// 🧮️ B1: writer's real `DocumentApp::Config` — absorbs every former `WriterPlayRuntime` app-struct
/// field (selection, editor selection, format/lint signals, revision, editor settings, AST hover,
/// engagement draft, and the session-only viewport camera — see `WriterCamera`'s doc comment) plus
/// `locale`, the one `ViewModel` field the writer UI actually reads (`resolve_labels`/`is_de_locale`
/// — see `crate::apps::writer::WriterPlayApp::render`), mirroring `shooting_engine::ShootingConfig`'s
/// B1 shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "writer.writercfg")]
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
pub enum WriterConfigOperation {
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

impl Operation<WriterConfig> for WriterConfigOperation {
    type Diff = WriterConfig;

    fn diff(&self, base: &WriterConfig) -> WriterConfig {
        let mut next = base.clone();
        match self {
            WriterConfigOperation::Snapshot { config } => return config.clone(),
            WriterConfigOperation::SetSelectedAstIds { ids } => next.selected_ast_ids = ids.clone(),
            WriterConfigOperation::SetEditorSelection { selection } => next.editor_selection = selection.clone(),
            WriterConfigOperation::SetFormatSignal { value } => next.format_signal = *value,
            WriterConfigOperation::SetLintSignal { value } => next.lint_signal = *value,
            WriterConfigOperation::SetRevision { value } => next.revision = *value,
            WriterConfigOperation::SetEditorSettings { settings } => next.editor_settings = settings.clone(),
            WriterConfigOperation::SetTreeHoveredAstId { id } => next.tree_hovered_ast_id = id.clone(),
            WriterConfigOperation::SetEditorHoverOffset { offset } => next.editor_hover_offset = *offset,
            WriterConfigOperation::SetEngagementInput { value } => next.engagement_input = value.clone(),
            WriterConfigOperation::SetCamera { camera } => next.camera = camera.clone(),
            WriterConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &WriterConfig) -> Vec<Self> {
        vec![WriterConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writer_config_dsl_round_trips_default_and_populated() {
        store::test_support::assert_config_round_trip(&WriterConfig::default());
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
        store::test_support::assert_config_round_trip(&populated);
    }

    #[test]
    fn writer_config_operation_backwards_restores_pre_state() {
        let pre = WriterConfig::default();
        store::test_support::assert_operation_round_trip(&pre, WriterConfigOperation::SetLocale { value: "de-DE".into() });
        store::test_support::assert_operation_round_trip(&pre, WriterConfigOperation::SetSelectedAstIds { ids: vec!["a".into()] });
        store::test_support::assert_operation_round_trip(&pre, WriterConfigOperation::SetCamera { camera: WriterCamera { x: 5.0, y: -2.0, zoom: 1.5 } });
    }

    #[test]
    fn writer_config_operation_binary_matches_text() {
        store::test_support::assert_op_text_binary_equivalence(&WriterConfigOperation::SetLocale { value: "de-DE".into() });
        store::test_support::assert_op_text_binary_equivalence(&WriterConfigOperation::Snapshot { config: WriterConfig::default() });
    }

    #[test]
    fn writer_config_pack_round_trips() {
        let config = WriterConfig { locale: "de-DE".into(), engagement_input: "format".into(), ..WriterConfig::default() };
        let bytes = store::DocumentPack::encode_pack(&config);
        let decoded = <WriterConfig as store::DocumentPack>::decode_pack(&bytes).expect("decode writer config pack");
        assert_eq!(decoded, config);
    }
}
//#endregion 🧪️Tests
