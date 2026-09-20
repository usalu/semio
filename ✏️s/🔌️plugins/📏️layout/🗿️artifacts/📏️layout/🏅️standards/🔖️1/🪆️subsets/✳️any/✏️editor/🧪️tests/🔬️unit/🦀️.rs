pub(crate) mod context {
    use super::super::*;
    use semio_framework_plugin::artifact_app_laws::{meta, new_app_with_registry};
    use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};
    
    pub type LayoutApp = VcsArtifactApp<EditorApp<LayoutPlayApp>>;
    
    /// ✏️ `LayoutPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
    /// `ArtifactApp` — `EditorApp<LayoutPlayApp>` (SDK adapter, contract §2.1) is the real
    /// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
    /// `PluginBuilder::editor::<LayoutPlayApp>` builds it.
    
    /// 🧪️ The app instance every test builds — registry-backed, because there is no other kind.
    /// `EditorApp<LayoutPlayApp>` publishes a `bounded_first_step_tool_proofs!` roster, and
    /// `with_registry_on_bus` joins that roster against the registry's `Migrated` tool ids
    /// (`AppActionRegistry::validate_tool_job_rows`): an empty registry declares none of them, so the
    /// registry-LESS `artifact_app_laws::new_app` fails construction outright with
    /// `interactive-job.catalog-authority … generated_migrated=false, migrated={}`.
    pub async fn layout_app() -> LayoutApp {
        layout_app_with_registry().await
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
    assert_eq!(ids.len(), 21, "every LayoutCommand row must be covered by every_command()");
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
            "deleteSelection" => "delete-selection",
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
        (LayoutCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { surface_id: None, x: 1.0, y: 2.0, width: 800.0, height: 600.0, samples: Vec::new() }), "canvas-pointer-move x=1 y=2 width=800 height=600 samples=[ ]"),
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
        LayoutCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { surface_id: None, x: 1.0, y: 2.0, width: 800.0, height: 600.0, samples: vec![[0.5, 1.5], [1.0, 2.0]] }),
        LayoutCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { cancelled: false }),
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
        LayoutCommand::DeleteSelection(crate::editor::layout::commands::delete_selection::DeleteSelection {}),
    ]
}

//#region 🔖️ContextMenu
use crate::editor::layout::commands::focus_preflight_issue::FocusPreflightIssue;
use crate::editor::layout::modes::edit::windows::blueprint::config::LayoutBlueprintWindowConfigOwner;
use crate::editor::layout::panels::preflight::run_layout_preflight;
use crate::editor::layout::terminology::layout_labels;
use crate::editor::layout::unit_tests::context::LayoutApp;
use semio_framework_plugin::{ActionMeta, INTERACTION_SELECT_ACTION_ID, ViewModel, ViewWindowInstance, WindowConfigOwner};

async fn context_menu_json(app: &mut LayoutApp, surface: Option<semio_framework_plugin::ContextMenuSurfaceTarget>) -> String {
    use semio_framework_plugin::{ContextMenuRequest, PluginApp, UiMenuRef};
    let request = ContextMenuRequest { menu: UiMenuRef { id: "layout".into(), args: None }, surface, window_instance_id: None, point: None };
    serde_json::to_string(&app.context_menu(&request, &semio_framework_plugin::ViewModel::default()).await).unwrap_or_default()
}

/// ⚖️ LAW: an empty blueprint canvas offers creation verbs and never delete-selection.
#[semio_framework_async_macros::async_test]
async fn context_menu_on_empty_canvas_offers_create_verbs_not_delete() {
    let mut app = layout_app_with_registry().await;
    let menu = context_menu_json(
        &mut app,
        Some(semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: crate::editor::layout::LAYOUT_PLAY_SURFACE_BLUEPRINT.into(),
            kind: "canvas2d".into(),
            hits: vec![],
            selection: vec![],
            text: None,
        }),
    )
    .await;
    assert!(menu.contains(r#""id":"addPage""#), "empty canvas should offer addPage: {menu}");
    assert!(menu.contains(r#""action":"addFrame""#), "empty canvas should offer addFrame kinds: {menu}");
    assert!(!menu.contains(r#""action":"deleteSelection""#), "empty canvas must not offer deleteSelection: {menu}");
    artifact_app_laws::close_registered_fixture_app(&mut app);
}

/// ⚖️ LAW: a frame selection ends with delete-selection and clearSelection.
#[semio_framework_async_macros::async_test]
async fn context_menu_on_frame_selection_offers_delete_last() {
    let mut app = layout_app_with_registry().await;
    let frame_id = app.snapshot().expect("projection").pages[0].frames.first().map(|frame| frame.id().to_string()).unwrap_or_else(|| "frame-1".into());
    let menu = context_menu_json(
        &mut app,
        Some(semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: crate::editor::layout::LAYOUT_PLAY_SURFACE_BLUEPRINT.into(),
            kind: "canvas2d".into(),
            hits: vec![],
            selection: vec![semio_framework_plugin::ContextMenuSelectionGroup { domain: crate::editor::layout::LAYOUT_GRANULARITY_ELEMENT.into(), ids: vec![frame_id] }],
            text: None,
        }),
    )
    .await;
    assert!(menu.contains(r#""action":"deleteSelection""#), "selection menu must include deleteSelection: {menu}");
    assert!(menu.contains(r#""action":"clearSelection""#), "selection menu must include clearSelection: {menu}");
    artifact_app_laws::close_registered_fixture_app(&mut app);
}

/// ⚖️ LAW: a page-tree row hit offers setActivePage, not canvas delete verbs.
#[semio_framework_async_macros::async_test]
async fn context_menu_on_page_row_offers_set_active_page() {
    let mut app = layout_app_with_registry().await;
    let page_id = app.snapshot().expect("projection").pages.first().map(|page| page.id.clone()).unwrap_or_else(|| "page-1".into());
    let menu = context_menu_json(
        &mut app,
        Some(semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: crate::editor::layout::panels::document::LAYOUT_PLAY_BODY_ARTIFACT.into(),
            kind: "tree".into(),
            hits: vec![semio_framework_plugin::ContextMenuHit { domain: "row".into(), id: format!("layout-document.page.{page_id}"), label: None }],
            selection: vec![],
            text: None,
        }),
    )
    .await;
    assert!(menu.contains(r#""action":"setActivePage""#), "page row menu must set active page: {menu}");
    assert!(!menu.contains(r#""action":"deleteSelection""#), "page row menu must not delete frames: {menu}");
    artifact_app_laws::close_registered_fixture_app(&mut app);
}

/// ⚖️ LAW: preview is read-only — selectAll only, no authoring delete/create.
#[semio_framework_async_macros::async_test]
async fn context_menu_on_preview_surface_is_read_only() {
    let mut app = layout_app_with_registry().await;
    let menu = context_menu_json(
        &mut app,
        Some(semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: crate::editor::layout::modes::edit::windows::preview::LAYOUT_PLAY_SURFACE_PREVIEW.into(),
            kind: "canvas2d".into(),
            hits: vec![],
            selection: vec![],
            text: None,
        }),
    )
    .await;
    assert!(menu.contains(r#""action":"selectAll""#), "preview should offer selectAll: {menu}");
    assert!(!menu.contains(r#""action":"addFrame""#), "preview must not offer addFrame: {menu}");
    assert!(!menu.contains(r#""action":"deleteSelection""#), "preview must not offer deleteSelection: {menu}");
    artifact_app_laws::close_registered_fixture_app(&mut app);
}

/// ⚖️ LAW: multi-frame selection uses a plural delete label and still exposes clipboard verbs.
#[semio_framework_async_macros::async_test]
async fn context_menu_on_multi_frame_selection_offers_plural_delete() {
    let mut app = layout_app_with_registry().await;
    let frames = app.snapshot().expect("projection").pages[0].frames.iter().take(2).map(|frame| frame.id().to_string()).collect::<Vec<_>>();
    assert_eq!(frames.len(), 2, "demo document must expose two frames on page 1");
    let menu = context_menu_json(
        &mut app,
        Some(semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: crate::editor::layout::LAYOUT_PLAY_SURFACE_BLUEPRINT.into(),
            kind: "canvas2d".into(),
            hits: vec![],
            selection: vec![semio_framework_plugin::ContextMenuSelectionGroup { domain: crate::editor::layout::LAYOUT_GRANULARITY_ELEMENT.into(), ids: frames }],
            text: None,
        }),
    )
    .await;
    assert!(menu.contains(r#""action":"deleteSelection""#), "multi selection must delete: {menu}");
    assert!(menu.contains("frames") || menu.contains("Rahmen"), "multi selection label should mention frames: {menu}");
    artifact_app_laws::close_registered_fixture_app(&mut app);
}

/// ⚖️ LAW: a links-tree row hit offers interactionSelect over referencing image frames.
#[semio_framework_async_macros::async_test]
async fn context_menu_on_link_row_offers_select_linked_frames() {
    let mut app = layout_app_with_registry().await;
    let link_id = app.snapshot().expect("projection").links.first().map(|link| link.id.clone()).expect("demo link");
    let menu = context_menu_json(
        &mut app,
        Some(semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: crate::editor::layout::panels::document::LAYOUT_PLAY_BODY_ARTIFACT.into(),
            kind: "tree".into(),
            hits: vec![semio_framework_plugin::ContextMenuHit { domain: "row".into(), id: format!("layout-document.link.{link_id}"), label: None }],
            selection: vec![],
            text: None,
        }),
    )
    .await;
    assert!(menu.contains(INTERACTION_SELECT_ACTION_ID), "link row must select referencing frames: {menu}");
    assert!(menu.contains("frame-image-1"), "link row must target the demo image frame: {menu}");
    artifact_app_laws::close_registered_fixture_app(&mut app);
}

/// ⚖️ LAW: a preflight issue row hit offers focusPreflightIssue with the issue payload.
#[semio_framework_async_macros::async_test]
async fn context_menu_on_preflight_row_offers_focus_issue() {
    let mut app = layout_app_with_registry().await;
    let snapshot = app.snapshot().expect("projection");
    let labels = layout_labels(&semio_framework_plugin::ViewModel::default());
    let issue = run_layout_preflight(&snapshot, &labels).into_iter().next().expect("demo document must surface at least one preflight issue");
    let row_id = format!("layout-preflight.{}.{}", issue.code, issue.object_id.clone().unwrap_or_else(|| issue.message.clone()));
    let menu = context_menu_json(
        &mut app,
        Some(semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: crate::editor::layout::panels::preflight::LAYOUT_PLAY_BODY_PREFLIGHT.into(),
            kind: "tree".into(),
            hits: vec![semio_framework_plugin::ContextMenuHit { domain: "row".into(), id: row_id, label: None }],
            selection: vec![],
            text: None,
        }),
    )
    .await;
    assert!(menu.contains(r#""action":"focusPreflightIssue""#), "preflight row must focus the issue: {menu}");
    assert!(menu.contains(&issue.code), "preflight menu must carry the issue code: {menu}");
    artifact_app_laws::close_registered_fixture_app(&mut app);
}

/// ⚖️ LAW: right-clicking an unselected frame hit offers Select before create verbs.
#[semio_framework_async_macros::async_test]
async fn context_menu_on_frame_hit_without_selection_offers_select() {
    let mut app = layout_app_with_registry().await;
    let frame_id = "frame-1".to_string();
    let menu = context_menu_json(
        &mut app,
        Some(semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: crate::editor::layout::LAYOUT_PLAY_SURFACE_BLUEPRINT.into(),
            kind: "canvas2d".into(),
            hits: vec![semio_framework_plugin::ContextMenuHit { domain: crate::editor::layout::LAYOUT_GRANULARITY_ELEMENT.into(), id: frame_id.clone(), label: None }],
            selection: vec![],
            text: None,
        }),
    )
    .await;
    assert!(menu.contains(r#""id":"select-hit""#), "frame hit must offer select: {menu}");
    assert!(menu.contains(INTERACTION_SELECT_ACTION_ID), "select must dispatch interactionSelect: {menu}");
    artifact_app_laws::close_registered_fixture_app(&mut app);
}

/// ⚖️ LAW: a single text-frame selection adds a same-kind create row in the create group.
#[semio_framework_async_macros::async_test]
async fn context_menu_on_text_frame_selection_offers_same_kind_create() {
    let mut app = layout_app_with_registry().await;
    let frame_id = "frame-text-1".to_string();
    let menu = context_menu_json(
        &mut app,
        Some(semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: crate::editor::layout::LAYOUT_PLAY_SURFACE_BLUEPRINT.into(),
            kind: "canvas2d".into(),
            hits: vec![],
            selection: vec![semio_framework_plugin::ContextMenuSelectionGroup { domain: crate::editor::layout::LAYOUT_GRANULARITY_ELEMENT.into(), ids: vec![frame_id] }],
            text: None,
        }),
    )
    .await;
    assert!(menu.contains(r#""id":"add-text-frame""#), "text selection must offer add text frame: {menu}");
    assert!(menu.contains(r#""kind":"text""#), "create row must target text frames: {menu}");
    artifact_app_laws::close_registered_fixture_app(&mut app);
}

/// ⚖️ LAW: when the surface carries no selection, the live interaction snapshot supplies element ids.
#[semio_framework_async_macros::async_test]
async fn context_menu_uses_live_interaction_when_surface_selection_empty() {
    let mut app = layout_app_with_registry().await;
    let frame_id = app.snapshot().expect("projection").pages[0].frames.first().expect("frame").id().to_string();
    let view = ViewModel { window_instances: vec![ViewWindowInstance { id: "layout-blueprint".into(), window_kind_id: LayoutBlueprintWindowConfigOwner::WINDOW_KIND_ID.into() }], ..Default::default() };
    let meta = ActionMeta { view_state: Some(view.for_window_instance("layout-blueprint").expect("blueprint window instance")), ..artifact_app_laws::meta("local") };
    app.bind_instance_id(meta.instance_id).await;
    app.dispatch_typed(LayoutCommand::FocusPreflightIssue(FocusPreflightIssue { object_id: Some(frame_id.clone()), page_id: Some("page-1".into()) }), &meta).await.expect("select frame");
    artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.expect("focus settles");
    let menu = context_menu_json(
        &mut app,
        Some(semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: crate::editor::layout::LAYOUT_PLAY_SURFACE_BLUEPRINT.into(),
            kind: "canvas2d".into(),
            hits: vec![],
            selection: vec![],
            text: None,
        }),
    )
    .await;
    assert!(menu.contains(r#""action":"deleteSelection""#), "live interaction selection must drive delete menu: {menu}");
    artifact_app_laws::close_registered_fixture_app(&mut app);
}
//#endregion 🔖️ContextMenu
//#endregion 🔖️CommandSurface

//#region 🔖️ActionBridge
/// 🌉️ Every command row the shells reach by action id must decode through `command_from_action`
/// (camelCase host keys → the payloads' own snake_case `FromValue` names), and its `command_id`
/// must round-trip — the boundary that was missing entirely before ticket
/// 26/09/16/LAYOUT-PLUGIN-END-TO-END (every shell action was refused as "not framework-reserved").
#[test]
fn command_from_action_round_trips_every_command_id() {
    for command in every_command() {
        let id = command.command_id();
        let args = dsl::ToValue::to_value(&command);
        // 🔁️ The `DslOps` wire shape is `{keyword: payload}`; the shell sends the bare payload object.
        let payload = match &args {
            dsl::DslValue::Object(entries) if entries.len() == 1 => entries[0].1.clone(),
            other => other.clone(),
        };
        let mut camel = camel_case_keys(&payload);
        if matches!(id, "canvasDragOver" | "canvasDrop") {
            let DslValue::Object(entries) = &mut camel else { panic!("catalogue payload object") };
            let index = entries.iter().position(|(key, _)| key == "kind").expect("typed catalogue kind");
            let (_, kind) = entries.remove(index);
            if id == "canvasDrop" {
                entries.push(("dragData".into(), DslValue::String(dsl::json::to_json_string(&DslValue::Object(vec![("kind".into(), kind)])))));
            } else {
                let DslValue::String(kind) = kind else { panic!("typed catalogue kind text") };
                entries.push(("types".into(), DslValue::Array(vec![DslValue::String("application/x-semio-catalogue-item".into()), DslValue::String(format!("application/x-semio-catalogue-kind.{kind}"))])));
            }
        }
        let bridged = LayoutPlayApp::command_from_action(id, Some(&camel)).unwrap_or_else(|error| panic!("action {id} failed to bridge: {}", error.message));
        assert_eq!(bridged.command_id(), id, "command_id mismatch for action {id}");
        assert_eq!(bridged, command, "payload drifted through the bridge for action {id}");
    }
    assert!(LayoutPlayApp::command_from_action("nonsense", None).is_err());
}

/// 🐫️ The shell's spelling of the payload keys.
fn camel_case_keys(value: &dsl::DslValue) -> dsl::DslValue {
    match value {
        dsl::DslValue::Object(entries) => dsl::DslValue::Object(
            entries
                .iter()
                .map(|(key, value)| {
                    let mut camel = String::new();
                    let mut upper = false;
                    for ch in key.chars() {
                        if ch == '_' { upper = true; } else if upper { camel.push(ch.to_ascii_uppercase()); upper = false; } else { camel.push(ch); }
                    }
                    (camel, value.clone())
                })
                .collect(),
        ),
        other => other.clone(),
    }
}

/// 🧱️ The host's control contracts (JSON floats, numeric `value`s, flat camera poses, palette
/// defaults) reach the same rows.
#[test]
fn command_from_action_bridges_host_control_contracts() {
    use crate::LayoutCamera;
    let args = |json: serde_json::Value| dsl::os_pack::json_to_dsl_value(&dsl::os_pack::json::parse(&json.to_string()).expect("fixture JSON"));
    assert_eq!(
        LayoutPlayApp::command_from_action("addFrame", Some(&args(serde_json::json!({ "kind": "text", "x": 12, "y": 24.5 })))).expect("addFrame bridge"),
        LayoutCommand::AddFrame(add_frame::AddFrame { kind: "text".into(), x: Some(12.0), y: Some(24.5) })
    );
    assert_eq!(LayoutPlayApp::command_from_action("addFrame", None).expect("addFrame default kind"), LayoutCommand::AddFrame(add_frame::AddFrame { kind: "rect".into(), x: None, y: None }));
    assert_eq!(LayoutPlayApp::command_from_action("addPage", Some(&args(serde_json::json!({ "windowId": "layout-blueprint" })))).expect("addPage ignores window keys"), LayoutCommand::AddPage(add_page::AddPage {}));
    assert_eq!(
        LayoutPlayApp::command_from_action("patchFrame", Some(&args(serde_json::json!({ "frameId": "frame-1", "field": "columns", "value": 2 })))).expect("patchFrame bridge"),
        LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-1".into(), page_id: None, field: "columns".into(), value: "2".into() })
    );
    assert_eq!(
        LayoutPlayApp::command_from_action("patchPage", Some(&args(serde_json::json!({ "pageId": "page-1", "field": "width", "value": "300" })))).expect("patchPage bridge"),
        LayoutCommand::PatchPage(patch_page::PatchPage { page_id: Some("page-1".into()), field: "width".into(), value: "300".into() })
    );
    assert_eq!(
        LayoutPlayApp::command_from_action("setActivePage", Some(&args(serde_json::json!({ "pageId": "page-2" })))).expect("setActivePage bridge"),
        LayoutCommand::SetActivePage(set_active_page::SetActivePage { page_id: "page-2".into() })
    );
    assert_eq!(
        LayoutPlayApp::command_from_action("setCamera", Some(&args(serde_json::json!({ "surfaceId": "layout.play.blueprint", "x": 1, "y": 2, "zoom": 1.5 })))).expect("flat camera bridge"),
        LayoutCommand::SetCamera(set_camera::SetCamera { surface_id: Some("layout.play.blueprint".into()), camera: LayoutCamera { x: 1.0, y: 2.0, zoom: 1.5 } })
    );
    assert_eq!(
        LayoutPlayApp::command_from_action("canvasPointerDown", Some(&args(serde_json::json!({ "surfaceId": "layout.play.blueprint", "button": 0, "shiftKey": true, "x": 10, "y": 20, "width": 800, "height": 600 })))).expect("pointer down bridge"),
        LayoutCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { surface_id: Some("layout.play.blueprint".into()), button: 0, extend: true, x: 10.0, y: 20.0, width: 800.0, height: 600.0 })
    );
    assert_eq!(
        LayoutPlayApp::command_from_action("engagementSubmit", Some(&args(serde_json::json!({ "value": "export png" })))).expect("engagement bridge"),
        LayoutCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: "export png".into() })
    );
    assert!(LayoutPlayApp::command_from_action("patchFrame", Some(&args(serde_json::json!({ "field": "fill" })))).is_err(), "a required field must not be defaulted silently");
}
//#endregion 🔖️ActionBridge

//#region 🔖️ManifestSanity
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let json = serde_json::to_string(&create_layout_app()).expect("app definition json");
    for id in [LAYOUT_PLAY_WINDOW_BLUEPRINT, LAYOUT_PLAY_WINDOW_PREVIEW] {
        assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
    }
    assert!(json.contains(edit::LAYOUT_PLAY_MODE_EDIT), "mode missing from the manifest");
    for body in [LAYOUT_PLAY_BODY_ARTIFACT, LAYOUT_PLAY_BODY_CATALOGUE, LAYOUT_PLAY_BODY_INSPECTION, LAYOUT_PLAY_BODY_PREFLIGHT] {
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
    let result = dispatch(&mut app, LayoutCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), x: sx, y: sy, width: 800.0, height: 600.0, samples: Vec::new() })).await;
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
    let media = LayoutPlayApp::export_media("artifact:out", &doc).expect("export document:out");
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
    assert_eq!(io.artifact_schema, "layout.layout");
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
    assert!(all_ports.iter().any(|port| port.id == "artifact:in"));
    assert!(all_ports.iter().any(|port| port.id == "artifact:out"));
}
//#endregion 🔖️MediaPorts

#[test]
fn canvas_catalogue_actions_consume_the_neutral_renderer_envelope() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🧫️fixtures/🛍️canvas-catalogue/🔣️.json" )).expect("neutral catalogue actions");
    for case in fixture["cases"].as_array().unwrap() {
        let args: dsl::DslValue = case["args"].clone().into();
        let result = LayoutPlayApp::command_from_action(case["action"].as_str().unwrap(), Some(&args));
        let Some(kind) = case["kind"].as_str() else {
            assert!(result.is_err(), "{} must refuse malformed or ambiguous catalogue input", case["id"]);
            continue;
        };
        let command = result.unwrap_or_else(|fault| panic!("{}: {}", case["id"], fault.message));
        let (actual, x, y, width, height) = match command {
            LayoutCommand::CanvasDragOver(payload) => (payload.kind, payload.x, payload.y, payload.width, payload.height),
            LayoutCommand::CanvasDrop(payload) => (payload.kind, payload.x, payload.y, payload.width, payload.height),
            _ => panic!("catalogue action routed to another command"),
        };
        assert_eq!(actual, kind, "{}", case["id"]);
        assert_eq!([x,y,width,height], [24.0,36.0,100.0,100.0]);
    }
}

#[semio_framework_async_macros::async_test]
async fn canvas_catalogue_retained_actions_preview_and_create_in_the_addressed_window() {
    use blueprint::config::LayoutBlueprintWindowConfigOwner;
    use blueprint::transient::LayoutBlueprintWindowTransientOwner;
    use semio_framework_plugin::{ActionMeta, ViewModel, ViewWindowInstance, WindowConfigOwner};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🧫️fixtures/🛍️canvas-catalogue/🔣️.json")).unwrap();
    let view = ViewModel {
        window_instances: ["catalogue-left", "catalogue-right"].map(|id| ViewWindowInstance { id: id.into(), window_kind_id: LayoutBlueprintWindowConfigOwner::WINDOW_KIND_ID.into() }).into(),
        ..Default::default()
    };
    let left = view.for_window_instance("catalogue-left").unwrap();
    let right = view.for_window_instance("catalogue-right").unwrap();
    for kind in ["page", "rect", "text", "image"] {
        let mut app = Box::new(layout_app_with_registry().await);
        let meta = ActionMeta { instance_id: 91, view_state: Some(left.clone()), ..artifact_app_laws::meta("catalogue") };
        app.bind_instance_id(meta.instance_id).await;
        let outcome: Result<(), String> = async {
            let before = app.snapshot().map_err(|error| format!("{error:?}"))?;
            for action in ["canvasDragOver", "canvasDrop"] {
                let case = fixture["cases"].as_array().unwrap().iter().find(|case| case["action"] == action && case["kind"] == kind).unwrap();
                let args: DslValue = case["args"].clone().into();
                let command = LayoutPlayApp::command_from_action(action, Some(&args)).map_err(|error| error.message)?;
                app.dispatch_typed(command, &meta).await.map_err(|error| format!("{error:?}"))?;
                artifact_app_laws::settle_registered_typed_operation(&mut *app, meta.instance_id).await.map_err(|error| format!("{error:?}"))?;
                let transient = app.window_transient_snapshot(&left).map_err(|error| format!("{error:?}"))?.ok_or("addressed transient missing")?;
                let preview = &transient.get::<LayoutBlueprintWindowTransientOwner>().ok_or("blueprint transient missing")?.drop_preview;
                if action == "canvasDragOver" {
                    if preview.kind != kind || (preview.x, preview.y) != (-26.0, -14.0) { return Err(format!("preview mismatch: {preview:?}")); }
                    if app.snapshot().map_err(|error| format!("{error:?}"))? != before { return Err("preview changed document".into()); }
                } else if !preview.kind.is_empty() {
                    return Err("drop did not clear preview".into());
                }
                if let Some(other) = app.window_transient_snapshot(&right).map_err(|error| format!("{error:?}"))? {
                    if other.get::<LayoutBlueprintWindowTransientOwner>().is_some_and(|state| !state.drop_preview.kind.is_empty()) { return Err("preview crossed window identity".into()); }
                }
            }
            let after = app.snapshot().map_err(|error| format!("{error:?}"))?;
            if kind == "page" {
                if after.pages.len() != before.pages.len() + 1 { return Err("page drop did not create exactly one page".into()); }
            } else {
                if after.pages[0].frames.len() != before.pages[0].frames.len() + 1 { return Err("frame drop did not create exactly one frame".into()); }
                let frame = after.pages[0].frames.last().ok_or("created frame missing")?;
                if frame.kind_str() != kind || (frame.bounds().x, frame.bounds().y) != (-26.0, -14.0) { return Err(format!("created frame mismatch: {frame:?}")); }
            }
            Ok(())
        }.await;
        artifact_app_laws::close_registered_fixture_app(&mut *app);
        outcome.unwrap_or_else(|error| panic!("{kind}: {error}"));
        eprintln!("[DEBUG] Layout catalogue {kind}: addressed preview, one creation, terminal preview retirement");
    }
}
