//! ⚖️ Layout app — binary command protocol surface + laws (constitutional: protocol). Also hosts
//! `LayoutCommand` — the app-engine `AppCommand::Command` binary command envelope (B1 pure-trait
//! flip). One variant per `create_layout_app`'s real declared action.

use layout::LayoutCamera;
use layout_op::LayoutOperation;
use protocol::OpBinary;
use serde::{Deserialize, Serialize};

/// 📦️ Encodes a `LayoutOperation` to its binary command form.
pub fn encode_op(operation: &LayoutOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `LayoutOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<LayoutOperation, protocol::ProtocolError> {
    LayoutOperation::decode_op(bytes)
}

//#region 🔖️LayoutCommand
/// 🎯️ B1: `LayoutPlayApp::Command` — the SOLE dispatch surface for layout's own behavior, covering
/// every declared action. Field shapes mirror each action's real (pre-B1 JSON) `args` object; generic
/// field-name/value-text patches (`PatchPage`/`PatchFrame`) keep the same "field key + text value"
/// shape the inspector already used, parsed per-field in `layout_ui::LayoutPlayApp::handle` — matching
/// `shooting_protocol::ShootingCommand::PatchShots`'s identical idiom for the same "many similar
/// scalar fields" problem.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum LayoutCommand {
    // 👁️ Config-only — mutate ephemeral config state, never emit document operations.
    #[dsl(key = "selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "active-page")]
    SetActivePage { page_id: String },
    #[dsl(key = "hover")]
    SetHover { id: Option<String> },
    #[dsl(key = "focus-preflight-issue")]
    FocusPreflightIssue { object_id: Option<String>, page_id: Option<String> },
    #[dsl(key = "engagement-input")]
    EngagementInput { value: String },
    #[dsl(key = "canvas-pointer-down")]
    CanvasPointerDown { surface_id: Option<String>, button: i64, extend: bool, x: f64, y: f64, width: f64, height: f64 },
    #[dsl(key = "canvas-pointer-move")]
    CanvasPointerMove { surface_id: Option<String>, x: f64, y: f64, width: f64, height: f64 },
    #[dsl(key = "canvas-pointer-up")]
    CanvasPointerUp,
    #[dsl(key = "canvas-drag-over")]
    CanvasDragOver { surface_id: Option<String>, kind: String, x: f64, y: f64, width: f64, height: f64 },
    #[dsl(key = "canvas-drag-leave")]
    CanvasDragLeave,
    #[dsl(key = "camera")]
    SetCamera { surface_id: Option<String>, #[dsl(block)] camera: LayoutCamera },
    #[dsl(key = "locale")]
    SetLocale { value: String },

    // 🔧️ Document-mutating — dispatched as VCS operations with a true inverse.
    #[dsl(key = "add-frame")]
    AddFrame { kind: String, x: Option<f64>, y: Option<f64> },
    #[dsl(key = "add-page")]
    AddPage,
    #[dsl(key = "patch-page")]
    PatchPage { page_id: Option<String>, field: String, value: String },
    #[dsl(key = "patch-frame")]
    PatchFrame { frame_id: String, page_id: Option<String>, field: String, value: String },
    #[dsl(key = "canvas-drop")]
    CanvasDrop { surface_id: Option<String>, kind: String, x: f64, y: f64, width: f64, height: f64 },

    // 🐚️ Shell effects — export round-trips through the host, no operations either way.
    #[dsl(key = "export-png")]
    ExportPng { page_id: Option<String> },
    #[dsl(key = "export-svg")]
    ExportSvg { page_id: Option<String> },
    #[dsl(key = "export-pdf")]
    ExportPdf { page_id: Option<String> },
    #[dsl(key = "export-package")]
    ExportPackage,
    #[dsl(key = "engagement-submit")]
    EngagementSubmit { value: String },
}
//#endregion 🔖️LayoutCommand

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use layout::PagePatch;
    use protocol::CollectionOperation;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let document = layout_engine::default_document();
        let page_id = document.pages[0].id.clone();
        let operation = LayoutOperation::Pages(CollectionOperation::Patch { id: page_id, patch: PagePatch { name: Some("Renamed".into()), ..Default::default() } });
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn document_text_round_trips_a_store_with_applied_operations() {
        use layout::LAYOUT_FIXTURE_SCHEMA;

        let initial = layout_engine::default_document();
        let envelope = store::create_document_envelope(LAYOUT_FIXTURE_SCHEMA, "layout-doc-text-test", initial, None);
        let mut doc_store: store::DocumentStore<layout::LayoutDocument, LayoutOperation> = store::DocumentStore::new(envelope);
        doc_store
            .dispatch(store::DocumentCommand::Apply {
                operations: vec![LayoutOperation::Pages(CollectionOperation::Patch { id: "page-1".into(), patch: PagePatch { width: Some(640.0), ..Default::default() } })],
                description: Some("resize page".into()),
            })
            .expect("apply patch page width");
        doc_store
            .dispatch(store::DocumentCommand::Apply {
                operations: vec![LayoutOperation::Pages(CollectionOperation::Patch { id: "page-1".into(), patch: PagePatch { name: Some("Renamed".into()), ..Default::default() } })],
                description: Some("rename page".into()),
            })
            .expect("apply patch page");
        store::test_support::assert_document_text_round_trip(&doc_store);
        store::test_support::assert_document_pack_round_trip(&doc_store);
        store::test_support::assert_live_equals_replay(&doc_store);
    }

    //#region 🧪️LayoutCommand
    #[test]
    fn layout_command_op_text_and_binary_round_trip_every_variant() {
        store::test_support::assert_op_line_round_trip(&LayoutCommand::SetSelection { ids: vec!["frame-1".into()] });
        store::test_support::assert_op_line_round_trip(&LayoutCommand::SetActivePage { page_id: "page-2".into() });
        store::test_support::assert_op_line_round_trip(&LayoutCommand::SetHover { id: Some("frame-1".into()) });
        store::test_support::assert_op_line_round_trip(&LayoutCommand::SetHover { id: None });
        store::test_support::assert_op_line_round_trip(&LayoutCommand::FocusPreflightIssue { object_id: Some("frame-1".into()), page_id: Some("page-1".into()) });
        store::test_support::assert_op_line_round_trip(&LayoutCommand::EngagementInput { value: "export png".into() });
        store::test_support::assert_op_line_round_trip(&LayoutCommand::CanvasPointerDown { surface_id: Some("layout.play.blueprint".into()), button: 0, extend: false, x: 1.0, y: 2.0, width: 800.0, height: 600.0 });
        store::test_support::assert_op_line_round_trip(&LayoutCommand::CanvasPointerMove { surface_id: None, x: 1.0, y: 2.0, width: 800.0, height: 600.0 });
        store::test_support::assert_op_line_round_trip(&LayoutCommand::CanvasPointerUp);
        store::test_support::assert_op_line_round_trip(&LayoutCommand::CanvasDragOver { surface_id: Some("layout.play.blueprint".into()), kind: "rect".into(), x: 1.0, y: 2.0, width: 800.0, height: 600.0 });
        store::test_support::assert_op_line_round_trip(&LayoutCommand::CanvasDragLeave);
        store::test_support::assert_op_line_round_trip(&LayoutCommand::SetCamera { surface_id: None, camera: LayoutCamera { x: 1.0, y: 2.0, zoom: 1.5 } });
        store::test_support::assert_op_line_round_trip(&LayoutCommand::SetLocale { value: "de-DE".into() });
        store::test_support::assert_op_line_round_trip(&LayoutCommand::AddFrame { kind: "rect".into(), x: Some(1.0), y: None });
        store::test_support::assert_op_line_round_trip(&LayoutCommand::AddPage);
        store::test_support::assert_op_line_round_trip(&LayoutCommand::PatchPage { page_id: Some("page-1".into()), field: "width".into(), value: "300".into() });
        store::test_support::assert_op_line_round_trip(&LayoutCommand::PatchFrame { frame_id: "frame-1".into(), page_id: Some("page-1".into()), field: "fill".into(), value: "0.5, 0.4, 0.3, 1".into() });
        store::test_support::assert_op_line_round_trip(&LayoutCommand::CanvasDrop { surface_id: Some("layout.play.blueprint".into()), kind: "rect".into(), x: 1.0, y: 2.0, width: 800.0, height: 600.0 });
        store::test_support::assert_op_line_round_trip(&LayoutCommand::ExportPng { page_id: Some("page-1".into()) });
        store::test_support::assert_op_line_round_trip(&LayoutCommand::ExportSvg { page_id: None });
        store::test_support::assert_op_line_round_trip(&LayoutCommand::ExportPdf { page_id: None });
        store::test_support::assert_op_line_round_trip(&LayoutCommand::ExportPackage);
        store::test_support::assert_op_line_round_trip(&LayoutCommand::EngagementSubmit { value: "export png".into() });

        let command = LayoutCommand::SetActivePage { page_id: "page-9".into() };
        let bytes = command.encode_op().expect("encode command");
        assert_eq!(LayoutCommand::decode_op(&bytes).expect("decode command"), command);
    }
    //#endregion 🧪️LayoutCommand
}
//#endregion 🧪️Tests
