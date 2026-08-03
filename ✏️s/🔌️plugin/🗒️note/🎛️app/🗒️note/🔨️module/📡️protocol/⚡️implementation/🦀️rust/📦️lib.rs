//! ⚖️ Note app — binary command protocol surface + laws (constitutional: protocol).

use note::NoteCamera;
use note_op::NoteOperation;
use protocol::OpBinary;
use serde::{Deserialize, Serialize};

/// 📦️ Encodes a `NoteOperation` to its binary command form.
pub fn encode_op(operation: &NoteOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `NoteOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<NoteOperation, protocol::ProtocolError> {
    NoteOperation::decode_op(bytes)
}

//#region 🔖️NoteCommand
/// 🎯️ `NotePlayApp::Command` — the SOLE dispatch surface for note's own behavior (B1 pure-trait
/// migration, mirroring `shooting_protocol::ShootingCommand`). One variant per action id the pre-B1
/// `NotePlayApp::handle_action` matched; combined `"x" | "y"` arms (e.g. the old
/// `"setGridVisible" | "toggleGrid"` alias, never independently wired anywhere in the note ui crate or
/// its hosts) collapse onto the one surviving action id's command instead of keeping a dead synonym.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum NoteCommand {
    // 🔧️ Document-mutating — dispatched as VCS operations with a true inverse.
    #[dsl(key = "set-grid-visible")]
    SetGridVisible { value: Option<bool> },
    #[dsl(key = "set-grid-spacing")]
    SetGridSpacing { value: f64 },
    #[dsl(key = "set-grid-subdivisions")]
    SetGridSubdivisions { value: f64 },
    #[dsl(key = "set-grid-opacity")]
    SetGridOpacity { value: f64 },
    #[dsl(key = "set-snap-enabled")]
    SetSnapEnabled { value: Option<bool> },
    #[dsl(key = "set-snap-grid-spacing")]
    SetSnapGridSpacing { value: f64 },
    #[dsl(key = "set-pencil-width")]
    SetPencilWidth { value: f64 },
    #[dsl(key = "set-eraser-radius")]
    SetEraserRadius { value: f64 },
    #[dsl(key = "add-block")]
    AddBlock { kind: String, x: f64, y: f64 },
    #[dsl(key = "move-block")]
    MoveBlock { block_id: String, target_row_id: String, drop_position: String },
    #[dsl(key = "delete-block")]
    DeleteBlock { block_id: String },
    #[dsl(key = "delete-selection")]
    DeleteSelection,
    #[dsl(key = "duplicate-block")]
    DuplicateBlock { block_id: String },
    #[dsl(key = "duplicate-selection")]
    DuplicateSelection,
    #[dsl(key = "patch-blocks")]
    PatchBlocks { block_ids: Vec<String>, field: String, value: String },
    #[dsl(key = "set-active-example")]
    SetActiveExample { example_id: String },
    #[dsl(key = "set-fixture-json")]
    SetFixtureJson { json: String },
    #[dsl(key = "ink-apply-events")]
    InkApplyEvents { events_json: String, phase: String, select_ids: Option<Vec<String>> },
    #[dsl(key = "engagement-submit")]
    EngagementSubmit { value: Option<String> },
    #[dsl(key = "nudge-selection")]
    NudgeSelection { dx: f64, dy: f64 },
    #[dsl(key = "nudge-selection-up")]
    NudgeSelectionUp,
    #[dsl(key = "nudge-selection-down")]
    NudgeSelectionDown,
    #[dsl(key = "nudge-selection-left")]
    NudgeSelectionLeft,
    #[dsl(key = "nudge-selection-right")]
    NudgeSelectionRight,
    #[dsl(key = "nudge-selection-up-fast")]
    NudgeSelectionUpFast,
    #[dsl(key = "nudge-selection-down-fast")]
    NudgeSelectionDownFast,
    #[dsl(key = "nudge-selection-left-fast")]
    NudgeSelectionLeftFast,
    #[dsl(key = "nudge-selection-right-fast")]
    NudgeSelectionRightFast,

    // 👁️ Config-only (was ephemeral `NotePlayRuntime`/`ViewState` state) — emit `config_operations`,
    // never document operations.
    #[dsl(key = "camera")]
    SetCamera { #[dsl(block)] camera: NoteCamera },
    #[dsl(key = "camera-zoom")]
    SetCameraZoom { value: f64 },
    #[dsl(key = "active-utility")]
    SetActiveUtility { utility_id: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
    #[dsl(key = "select-all")]
    SelectAll,
    #[dsl(key = "clear-selection")]
    ClearSelection,
    #[dsl(key = "set-selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "set-hover")]
    SetHover { block_id: Option<String> },
    #[dsl(key = "engagement-input")]
    EngagementInput { value: String },
    #[dsl(key = "navigator-engagement-input")]
    NavigatorEngagementInput,

    // 🐚️ Shell effects — export/import round-trips through the host, no operations either way.
    #[dsl(key = "save-download")]
    SaveDownload,
    #[dsl(key = "load-request")]
    LoadRequest,
}
//#endregion 🔖️NoteCommand

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = NoteOperation::SetGridSpacing { spacing: Some(24.0) };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn note_document_text_round_trips_store_with_applied_operation() {
        use note::NoteDocument;

        let envelope = store::create_document_envelope::<NoteDocument, NoteOperation>(
            "note.document",
            "doc-text-test",
            note_engine::empty_note_document(),
            None,
        );
        let mut doc_store = store::DocumentStore::new(envelope);
        doc_store
            .dispatch(store::DocumentCommand::Apply {
                operations: vec![NoteOperation::SetGridSpacing { spacing: Some(48.0) }],
                description: None,
            })
            .expect("apply");
        store::test_support::assert_document_text_round_trip(&doc_store);
        store::test_support::assert_document_pack_round_trip(&doc_store);
    }
}
//#endregion 🧪️Tests
