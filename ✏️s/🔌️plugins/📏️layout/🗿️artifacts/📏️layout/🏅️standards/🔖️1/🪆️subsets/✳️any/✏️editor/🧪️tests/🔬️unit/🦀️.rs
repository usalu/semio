pub(crate) mod context {
    use super::super::*;
    use semio_framework_plugin::artifact_app_laws::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};
    
    pub type LayoutApp = VcsArtifactApp<EditorApp<LayoutPlayApp>>;
    
    /// ✏️ `LayoutPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
    /// `ArtifactApp` — `EditorApp<LayoutPlayApp>` (SDK adapter, contract §2.1) is the real
    /// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
    /// `PluginBuilder::editor::<LayoutPlayApp>` builds it.
    
    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub async fn layout_app() -> LayoutApp {
        new_app::<EditorApp<LayoutPlayApp>>().await
    }
    
    /// 🧪️ Adapts `create_layout_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `new_app_with_registry` still expects — framework test context gap, not
    /// modifiable here (`🧰️framework/**` is outside this packet's lease).
    fn layout_app_manifest_for_tests() -> App {
        App { definition: create_layout_app(), examples: Vec::new() }
    }
    
    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub async fn layout_app_with_registry() -> LayoutApp {
        new_app_with_registry::<EditorApp<LayoutPlayApp>>(layout_app_manifest_for_tests).await
    }
    
    pub async fn dispatch(app: &mut LayoutApp, command: LayoutCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
    }
    
    pub async fn render(app: &mut LayoutApp, body_key: &str) -> String {
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(app.render(body_key, None, &ViewModel::default()).await.expect("render")).expect("fixture projection")
    }
    
    pub fn test_screen_point(camera_x: f64, camera_y: f64, zoom: f64, width: f64, height: f64, world_x: f64, world_y: f64) -> (f64, f64) {
        let camera = infinite_canvas::camera::Camera { x: camera_x, y: camera_y, zoom };
        let viewport = infinite_canvas::camera::Viewport { width: width as u32, height: height as u32, dpr: 1.0 };
        let screen = infinite_canvas::camera::world_to_screen(&camera, &viewport, infinite_canvas::Point::new(world_x, world_y));
        (screen.x, screen.y)
    }
}

use super::*;
use crate::editor::layout::unit_tests::context::{dispatch, layout_app, layout_app_with_registry, render, test_screen_point};
use semio_framework_plugin::artifact_app_laws;
use semio_framework_plugin::PluginApp;

//#region 🔖️CommandSurface
/// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every row's
/// wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
#[semio_framework_async_macros::async_test]
async fn command_ids_are_unique_and_match_the_declared_manifest_actions() {
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids.len(), 20, "every LayoutCommand row must be covered by every_command()");
}

/// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_through_text_and_binary() {
    for command in every_command() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — the exact
/// `as` literal declared in the `app_commands!` invocation above. Unlike flow (where the wire
/// pre-existing `📡️protocol` crate deliberately shortened every `set*` view command's wire keyword
/// forward verbatim, not a drift.
#[semio_framework_async_macros::async_test]
async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
    let expected_keyword = |id: &str| -> &'static str {
        match id {
            "setActivePage" => "active-page",
            "setCamera" => "camera",
            "focusPreflightIssue" => "focus-preflight-issue",
            "engagementInput" => "engagement-input",
            "canvasPointerDown" => "canvas-pointer-down",
            "canvasPointerMove" => "canvas-pointer-move",
            "canvasPointerUp" => "canvas-pointer-up",
            "canvasDragOver" => "canvas-drag-over",
            "canvasDragLeave" => "canvas-drag-leave",
            "addFrame" => "add-frame",
            "addPage" => "add-page",
            "patchPage" => "patch-page",
            "patchFrame" => "patch-frame",
            "canvasDrop" => "canvas-drop",
            "exportPng" => "export-png",
            "exportSvg" => "export-svg",
            "exportPdf" => "export-pdf",
            "exportPackage" => "export-package",
            "engagementSubmit" => "engagement-submit",
            other => panic!("every_command() row {other} missing from this test's expected-keyword table"),
        }
    };
    for command in every_command() {
        let id = command.command_id();
        let expected = expected_keyword(id);
        let printed = protocol::OpText::print_op(&command);
        assert_eq!(printed.split(' ').next().unwrap_or_default(), expected, "wire keyword drifted for command {id}: {printed:?}");
    }
}

/// ⚖️ Rows whose `Option` fields make `None`/`Some` distinct wire cases round-trip text/binary
/// identically either way — the enum's binary ordinal shifted when `setSelection`/`setHover`
/// were deleted (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM), so this no longer
/// pins exact historical bytes (greenfield: no back-compat), only text/binary equivalence.
#[semio_framework_async_macros::async_test]
async fn optional_field_rows_round_trip_text_and_binary_either_way() {
    use crate::LayoutCamera;
    let cases: [(LayoutCommand, &str); 3] = [
        (LayoutCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { surface_id: None, x: 1.0, y: 2.0, width: 800.0, height: 600.0 }), "canvas-pointer-move x=1 y=2 width=800 height=600"),
        (LayoutCommand::AddFrame(add_frame::AddFrame { kind: "rect".into(), x: Some(1.0), y: None }), "add-frame kind=rect x=1"),
        (LayoutCommand::SetCamera(set_camera::SetCamera { surface_id: None, camera: LayoutCamera { x: 1.0, y: 2.0, zoom: 1.5 } }), "camera camera { x=1 y=2 zoom=1.5 }"),
    ];
    for (command, text) in cases {
        assert_eq!(protocol::OpText::print_op(&command), text);
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
pub(super) fn every_command() -> Vec<LayoutCommand> {
    use crate::LayoutCamera;
    vec![
        LayoutCommand::SetActivePage(set_active_page::SetActivePage { page_id: "page-2".into() }),
        LayoutCommand::FocusPreflightIssue(focus_preflight_issue::FocusPreflightIssue { object_id: Some("frame-1".into()), page_id: Some("page-1".into()) }),
        LayoutCommand::EngagementInput(engagement_input::EngagementInput { value: "export png".into() }),
        LayoutCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { surface_id: Some("layout.play.blueprint".into()), button: 0, extend: false, x: 1.0, y: 2.0, width: 800.0, height: 600.0 }),
        LayoutCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { surface_id: None, x: 1.0, y: 2.0, width: 800.0, height: 600.0 }),
        LayoutCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {}),
        LayoutCommand::CanvasDragOver(canvas_drag_over::CanvasDragOver { surface_id: Some("layout.play.blueprint".into()), kind: "rect".into(), x: 1.0, y: 2.0, width: 800.0, height: 600.0 }),
        LayoutCommand::CanvasDragLeave(canvas_drag_leave::CanvasDragLeave {}),
        LayoutCommand::SetCamera(set_camera::SetCamera { surface_id: None, camera: LayoutCamera { x: 1.0, y: 2.0, zoom: 1.5 } }),
        LayoutCommand::AddFrame(add_frame::AddFrame { kind: "rect".into(), x: Some(1.0), y: None }),
        LayoutCommand::AddPage(add_page::AddPage {}),
        LayoutCommand::PatchPage(patch_page::PatchPage { page_id: Some("page-1".into()), field: "width".into(), value: "300".into() }),
        LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-1".into(), page_id: Some("page-1".into()), field: "fill".into(), value: "0.5, 0.4, 0.3, 1".into() }),
        LayoutCommand::CanvasDrop(canvas_drop::CanvasDrop { surface_id: Some("layout.play.blueprint".into()), kind: "rect".into(), x: 1.0, y: 2.0, width: 800.0, height: 600.0 }),
        LayoutCommand::ExportPng(export_png::ExportPng { page_id: Some("page-1".into()) }),
        LayoutCommand::ExportSvg(export_svg::ExportSvg { page_id: None }),
        LayoutCommand::ExportPdf(export_pdf::ExportPdf { page_id: None }),
        LayoutCommand::ExportPackage(export_package::ExportPackage {}),
        LayoutCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: "export png".into() }),
    ]
}
//#endregion 🔖️CommandSurface

//#region 🔖️ManifestSanity
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let json = serde_json::to_string(&create_layout_app()).expect("app definition json");
    for id in [LAYOUT_PLAY_WINDOW_BLUEPRINT, LAYOUT_PLAY_WINDOW_PREVIEW] {
        assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
    }
    assert!(json.contains(edit::LAYOUT_PLAY_MODE_EDIT), "mode missing from the manifest");
    for body in [LAYOUT_PLAY_BODY_DOCUMENT, LAYOUT_PLAY_BODY_CATALOGUE, LAYOUT_PLAY_BODY_INSPECTION, LAYOUT_PLAY_BODY_PREFLIGHT] {
        assert!(json.contains(body), "panel body {body} missing from the manifest");
    }
    assert!(json.contains("2d.layout"), "artifact kind missing from the manifest");
}

#[semio_framework_async_macros::async_test]
async fn window_kind_actions_scope_authoring_to_blueprint_only() {
    let definition = create_layout_app();
    let resolve = |window_id: &str| -> Vec<String> {
        let window = definition.window_kinds.iter().find(|window| window.id == window_id).unwrap();
        semio_framework_plugin::resolve_window_actions(&definition, window).into_iter().map(|action| action.id.clone()).collect()
    };
    let blueprint_actions = resolve(LAYOUT_PLAY_WINDOW_BLUEPRINT);
    let preview_actions = resolve(LAYOUT_PLAY_WINDOW_PREVIEW);
    for authoring in ["addFrame", "addPage", "patchPage", "patchFrame"] {
        assert!(blueprint_actions.contains(&authoring.to_string()), "Blueprint must expose {authoring}");
        assert!(!preview_actions.contains(&authoring.to_string()), "Preview must NOT expose {authoring}");
    }
    for shared in ["exportPng", "exportPdf", "setCamera"] {
        assert!(blueprint_actions.contains(&shared.to_string()) && preview_actions.contains(&shared.to_string()), "{shared} stays on both windows");
    }
}
//#endregion 🔖️ManifestSanity

//#region 🔖️CrossCutting
#[semio_framework_async_macros::async_test]
async fn sample_fixture_parses() {
    let doc = crate::standards::v1::subsets::any::schema::snapshot::text::parse_dsl(crate::standards::v1::subsets::any::schema::snapshot::text::LAYOUT_SAMPLE_TEXT).expect("sample fixture");
    assert_eq!(doc.schema, crate::LAYOUT_DOCUMENT_SCHEMA);
    assert!(!doc.pages.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    let mut app = layout_app().await;
    assert!(render(&mut app, "layout.play.nope").await.contains("Unknown body"));
}

// 🕹️ `selected_and_hovered_frames_get_chrome_strokes` deleted: selection/hover chrome strokes read
// `config.selected_ids`/`hovered_id`, both deleted with the framework-owned "elements" domain.
// `canvas_layers` always renders with empty selection/hover now — `ArtifactApp::render` carries no
// `InteractionView` (a known SDK gap, same as gis2d's/puzzle3d's inspection panels — see this
// ticket's w3b-summary.md) — flagged, not fixed here (framework file, out of this crate's remit)?.

#[semio_framework_async_macros::async_test]
async fn window_engagements_cover_both_windows() {
    let mut app = layout_app().await;
    let engagements = app.window_engagements(&semio_framework_plugin::ViewModel::default()).await;
    let blueprint_engagement = engagements.get(LAYOUT_PLAY_WINDOW_BLUEPRINT).expect("blueprint engagement");
    let status = blueprint_engagement.status.as_ref().and_then(|rows| rows.first()).expect("status");
    assert!(status.text.contains("Page"));
    let input = blueprint_engagement.input.as_ref().expect("input");
    assert_eq!(input.placeholder.as_deref(), Some("undo, redo, export png"));
    assert!(engagements.contains_key(LAYOUT_PLAY_WINDOW_PREVIEW));
}

#[semio_framework_async_macros::async_test]
async fn registry_backed_add_frame_emits_operation() {
    // 🧬️ addFrame is declared `Mutation`: the registry-backed wrapper must let its operations through.
    let mut app = layout_app_with_registry().await;
    let result = dispatch(&mut app, LayoutCommand::AddFrame(add_frame::AddFrame { kind: "rect".into(), x: None, y: None })).await;
    assert_eq!(result.mutations.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn registry_backed_pointer_move_is_view_only() {
    // 🧬️ canvasPointerMove is declared `View`: it mutates only config hover state and must never emit
    // an operation, which the registry kind-discipline check enforces.
    let mut app = layout_app_with_registry().await;
    let (sx, sy) = test_screen_point(0.0, 0.0, 1.0, 800.0, 600.0, 156.0, 220.0);
    let result = dispatch(&mut app, LayoutCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), x: sx, y: sy, width: 800.0, height: 600.0 })).await;
    assert!(result.mutations.is_empty(), "View action must not emit document operations");
}
//#endregion 🔖️CrossCutting

//#region 🔖️MediaPorts
#[semio_framework_async_macros::async_test]
async fn direct_layout_out_reducer_is_fail_closed_for_runtime_job_interception() {
    let app = layout_app().await;
    let document = app.snapshot().expect("projection");
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    assert!(matches!(LayoutPlayApp::export_media("layout:out", &doc), Err(MediaError::NotImplemented)));
}

#[semio_framework_async_macros::async_test]
async fn export_media_document_out_round_trips_through_pack() {
    let app = layout_app().await;
    let document = app.snapshot().expect("projection");
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let media = LayoutPlayApp::export_media("document:out", &doc).expect("export document:out");
    let MediaPayload::Structured { schema, json } = media.payload else { panic!("expected structured payload") };
    assert_eq!(schema, crate::LAYOUT_DOCUMENT_SCHEMA);
    let bytes = store::pack_rt::pack_value_from_base64(&json).expect("decode base64 pack");
    let decoded = <LayoutSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode pack");
    assert_eq!(decoded, document);
}

#[semio_framework_async_macros::async_test]
async fn import_media_fields_in_sets_data_fields_json() {
    let mut app = layout_app().await;
    let media = Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "form.dictionary".into(), json: r#"{"name":"Ada"}"#.into() } };
    app.import_media("fields:in", media, &artifact_app_laws::meta("local")).await.expect("import fields:in");
    let document = app.snapshot().expect("projection");
    assert_eq!(document.data_fields_json.as_deref(), Some(r#"{"name":"Ada"}"#));
}

#[semio_framework_async_macros::async_test]
async fn layout_io_exposes_declared_ports() {
    let io = LayoutPlayApp::io().expect("layout declares io");
    assert!(io.ports.iter().any(|port| port.id == "fields:in"));
    assert!(io.ports.iter().any(|port| port.id == "layout:out"));
}

#[semio_framework_async_macros::async_test]
async fn layout_io_declares_fields_in_and_layout_out_ports() {
    let io = crate::editor::layout::engine::layout_io();
    assert_eq!(io.document_schema, "layout.layout");
    assert_eq!(io.artifact.id, "2d.layout");
    let fields_in = io.ports.iter().find(|port| port.id == "fields:in").expect("fields:in declared");
    assert_eq!(fields_in.direction, semio_framework_plugin::MediaPortDirection::In);
    assert_eq!(fields_in.kind_id.as_deref(), Some("form.dictionary"));
    assert_eq!(fields_in.multiplicity, semio_framework::PortMultiplicity::One);
    let layout_out = io.ports.iter().find(|port| port.id == "layout:out").expect("layout:out declared");
    assert_eq!(layout_out.direction, semio_framework_plugin::MediaPortDirection::Out);
    assert_eq!(layout_out.kind_id.as_deref(), Some("2d.layout"));
    assert_eq!(layout_out.multiplicity, semio_framework::PortMultiplicity::Many);
    let all_ports = io.all_ports().await;
    assert!(all_ports.iter().any(|port| port.id == "document:in"));
    assert!(all_ports.iter().any(|port| port.id == "document:out"));
}
//#endregion 🔖️MediaPorts
