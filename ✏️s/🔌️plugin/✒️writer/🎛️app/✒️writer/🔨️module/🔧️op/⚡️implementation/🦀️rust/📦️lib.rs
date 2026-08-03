//! ⚡️ Writer app — operation enum + laws (constitutional: op).

use protocol::{Operation, OperationDiff};
use writer::{WriterCamera, WriterProjection};
use writer_engine::{WriterConfig, WriterEditorSelection, WriterEditorSettings};

//#region 🔖️Types
/// 📐️ Typed content mutation for a `WriterProjection`. The editor viewport camera is session-only
/// runtime state now (never a document operation — see `WriterPlayRuntime::camera` in the ui crate).
/// Each variant's op keyword is the auto-derived kebab-case of its own name (`SetText` -> `set-text`,
/// ...) — see {@link protocol::OpText}.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum WriterOperation {
    SetText { text: String },
    SetDocument {
        #[dsl(block)]
        document: WriterProjection,
    }
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WriterDiff {
    pub text: Option<String>,
    pub document: Option<WriterProjection>,
}

impl OperationDiff<WriterProjection> for WriterDiff {
    fn apply(&self, projection: &WriterProjection) -> WriterProjection {
        if let Some(document) = &self.document {
            return document.clone();
        }
        WriterProjection { text: self.text.clone().unwrap_or_else(|| projection.text.clone()), ..projection.clone() }
    }

    fn absorb(&mut self, other: Self) {
        if other.document.is_some() {
            *self = other;
            return;
        }
        if other.text.is_some() {
            self.text = other.text;
        }
    }
}

impl Operation<WriterProjection> for WriterOperation {
    type Diff = WriterDiff;

    fn diff(&self, _projection: &WriterProjection) -> WriterDiff {
        match self {
            WriterOperation::SetText { text } => WriterDiff { text: Some(text.clone()), ..Default::default() },
            WriterOperation::SetDocument { document } => WriterDiff { document: Some(document.clone()), ..Default::default() },
        }
    }

    fn backwards(&self, projection: &WriterProjection) -> Vec<Self> {
        match self {
            WriterOperation::SetText { .. } => vec![WriterOperation::SetText { text: projection.text.clone() }],
            WriterOperation::SetDocument { .. } => vec![WriterOperation::SetDocument { document: projection.clone() }],
        }
    }
}
//#endregion 🔖️Types

//#region 🔖️ConfigOperations
/// @emoji 🧮️ B1: `writer_engine::WriterConfig`'s operation enum — one variant per settled interaction
/// (mirrors the pre-B1 `WriterPlayRuntime` field writes), plus a generic `Snapshot` every variant's
/// `backwards()` returns — mirrors `shooting_op::ShootingConfigOperation` exactly (see that type's
/// doc comment for the whole-config-snapshot inverse rationale).
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslOps)]
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
    use store::{create_document_envelope, DocumentCommand};

    type WriterStore = store::DocumentStore<WriterProjection, WriterOperation>;

    fn seeded_store() -> WriterStore {
        WriterStore::new(create_document_envelope("writer.document", "writer", writer_engine::empty_writer_projection(), None))
    }

    #[test]
    fn writer_document_vcs_replays_text_operations() {
        let mut store = seeded_store();
        store.dispatch(DocumentCommand::Apply { operations: vec![WriterOperation::SetText { text: "hello".into() }], description: None }).expect("apply");
        assert_eq!(store.projection().expect("projection").text, "hello");
    }

    #[test]
    fn writer_document_vcs_replays_document_operation() {
        let mut store = seeded_store();
        let replacement = WriterProjection { schema: "writer.document".into(), id: "jack".into(), language_id: "jack".into(), uri: "writer://jack".into(), text: "MATCH (a) RETURN a".into() };
        store.dispatch(DocumentCommand::Apply { operations: vec![WriterOperation::SetDocument { document: replacement }], description: None }).expect("apply document");
        let projection = store.projection().expect("projection");
        assert_eq!(projection.id, "jack");
        assert_eq!(projection.text, "MATCH (a) RETURN a");
    }

    #[test]
    fn writer_document_vcs_undoes_text_operation() {
        let mut store = seeded_store();
        store.dispatch(DocumentCommand::Apply { operations: vec![WriterOperation::SetText { text: "hello".into() }], description: None }).expect("apply");
        store.dispatch(DocumentCommand::Undo).expect("undo");
        assert_eq!(store.projection().expect("projection").text, "");
    }

    /// ✍️ Hand-built representative document — verbatim from the original file's `🔖️DslAndOpText`
    /// test region (duplicated per-crate since each constitutional crate's tests compile independently).
    fn jack_projection() -> WriterProjection {
        WriterProjection {
            schema: "writer.document".into(),
            id: "jack".into(),
            language_id: "jack".into(),
            uri: "writer://jack".into(),
            text: "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = \"core\"\nRETURN a.name, b.name".into(),
        }
    }

    #[test]
    fn writer_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&WriterOperation::SetText { text: "line one\nline two".into() });
        store::test_support::assert_op_line_round_trip(&WriterOperation::SetDocument { document: jack_projection() });
    }

    //#region 🔖️ConfigTests
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
    //#endregion 🔖️ConfigTests
}
//#endregion 🧪️Tests
