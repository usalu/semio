#![cfg(test)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

use semio_s_artifact_layout_layout_editor::unit_tests::context;
use semio_s_artifact_layout_layout_editor_panels::document::LAYOUT_PLAY_BODY_ARTIFACT;
use semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::{LAYOUT_PLAY_BODY_BLUEPRINT, LAYOUT_PLAY_SURFACE_BLUEPRINT, LAYOUT_PLAY_WINDOW_BLUEPRINT};
use semio_s_artifact_layout_layout_editor_panels::catalogue::LAYOUT_PLAY_BODY_CATALOGUE;
use semio_s_artifact_layout_layout_editor_panels::inspection::LAYOUT_PLAY_BODY_INSPECTION;
use semio_s_artifact_layout_layout_editor_panels::preflight::{LAYOUT_PLAY_BODY_PREFLIGHT, run_layout_preflight};
use semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::preview::{LAYOUT_PLAY_BODY_PREVIEW, LAYOUT_PLAY_WINDOW_PREVIEW};
use semio_s_artifact_layout_layout_editor::{LayoutCommand, LayoutPlayApp, create_layout_app};
use semio_s_artifact_layout_layout_editor_panels::terminology::layout_labels;
use semio_s_artifact_layout_layout_editor::unit_tests::context::{dispatch, layout_app, layout_app_with_registry, render, test_screen_point};
use semio_framework_plugin::artifact_app_laws;
use semio_framework_plugin::PluginApp;
use semio_framework_plugin::{ArtifactEditor, ArtifactView, DslValue, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType};
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;

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
    let mut declared: Vec<&str> = LayoutCommand::TOOL_JOB_IDS.to_vec();
    declared.sort_unstable();
    assert_eq!(sorted, declared, "every LayoutCommand row declared by `app_commands!` must be covered by every_command()");
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
            "exportStep" => "export-step",
            "exportIdml" => "export-idml",
            "exportPackage" => "export-package",
            "engagementSubmit" => "engagement-submit",
            "deleteSelection" => "delete-selection",
            "translateSelection" => "translate-selection",
            "rotateSelection" => "rotate-selection",
            "scaleSelection" => "scale-selection",
            "patchDocument" => "patch-document",
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
    use semio_s_artifact_layout_layout_model::LayoutCamera;
    let cases: [(LayoutCommand, &str); 3] = [
        (LayoutCommand::CanvasPointerMove(semio_s_artifact_layout_layout_editor_commands::canvas_pointer_move::CanvasPointerMove { surface_id: None, x: 1.0, y: 2.0, width: 800.0, height: 600.0, samples: Vec::new() }), "canvas-pointer-move x=1 y=2 width=800 height=600 samples=[ ]"),
        (LayoutCommand::AddFrame(semio_s_artifact_layout_layout_editor_commands::add_frame::AddFrame { kind: "rect".into(), x: Some(1.0), y: None }), "add-frame kind=rect x=1"),
        (LayoutCommand::SetCamera(semio_s_artifact_layout_layout_editor_commands::set_camera::SetCamera { surface_id: None, camera: LayoutCamera { x: 1.0, y: 2.0, zoom: 1.5 } }), "camera camera { x=1 y=2 zoom=1.5 }"),
    ];
    for (command, text) in cases {
        assert_eq!(protocol::OpText::print_op(&command), text);
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
pub fn every_command() -> Vec<LayoutCommand> {
    use semio_s_artifact_layout_layout_model::LayoutCamera;
    vec![
        LayoutCommand::SetActivePage(semio_s_artifact_layout_layout_editor_commands::set_active_page::SetActivePage { page_id: "page-2".into() }),
        LayoutCommand::FocusPreflightIssue(semio_s_artifact_layout_layout_editor_panels::focus_preflight_issue::FocusPreflightIssue { object_id: Some("frame-1".into()), page_id: Some("page-1".into()) }),
        LayoutCommand::EngagementInput(semio_s_artifact_layout_layout_editor_commands::engagement_input::EngagementInput { value: "export png".into() }),
        LayoutCommand::CanvasPointerDown(semio_s_artifact_layout_layout_editor_commands::canvas_pointer_down::CanvasPointerDown { surface_id: Some("layout.play.blueprint".into()), button: 0, extend: false, x: 1.0, y: 2.0, width: 800.0, height: 600.0 }),
        LayoutCommand::CanvasPointerMove(semio_s_artifact_layout_layout_editor_commands::canvas_pointer_move::CanvasPointerMove { surface_id: None, x: 1.0, y: 2.0, width: 800.0, height: 600.0, samples: vec![[0.5, 1.5], [1.0, 2.0]] }),
        LayoutCommand::CanvasPointerUp(semio_s_artifact_layout_layout_editor_commands::canvas_pointer_up::CanvasPointerUp { cancelled: false }),
        LayoutCommand::CanvasDragOver(semio_s_artifact_layout_layout_editor_commands::canvas_drag_over::CanvasDragOver { surface_id: Some("layout.play.blueprint".into()), kind: "rect".into(), x: 1.0, y: 2.0, width: 800.0, height: 600.0 }),
        LayoutCommand::CanvasDragLeave(semio_s_artifact_layout_layout_editor_commands::canvas_drag_leave::CanvasDragLeave {}),
        LayoutCommand::SetCamera(semio_s_artifact_layout_layout_editor_commands::set_camera::SetCamera { surface_id: None, camera: LayoutCamera { x: 1.0, y: 2.0, zoom: 1.5 } }),
        LayoutCommand::AddFrame(semio_s_artifact_layout_layout_editor_commands::add_frame::AddFrame { kind: "rect".into(), x: Some(1.0), y: None }),
        LayoutCommand::AddPage(semio_s_artifact_layout_layout_editor_commands::add_page::AddPage {}),
        LayoutCommand::PatchPage(semio_s_artifact_layout_layout_editor_commands::patch_page::PatchPage { page_id: Some("page-1".into()), field: "width".into(), value: "300".into() }),
        LayoutCommand::PatchFrame(semio_s_artifact_layout_layout_editor_commands::patch_frame::PatchFrame { frame_id: "frame-1".into(), page_id: Some("page-1".into()), field: "fill".into(), value: "0.5, 0.4, 0.3, 1".into() }),
        LayoutCommand::CanvasDrop(semio_s_artifact_layout_layout_editor_commands::canvas_drop::CanvasDrop { surface_id: Some("layout.play.blueprint".into()), kind: "rect".into(), x: 1.0, y: 2.0, width: 800.0, height: 600.0, artifact_ref: String::new(), proxy_data_url: String::new() }),
        LayoutCommand::ExportPng(semio_s_artifact_layout_layout_editor_commands::export_png::ExportPng { page_id: Some("page-1".into()) }),
        LayoutCommand::ExportSvg(semio_s_artifact_layout_layout_editor_commands::export_svg::ExportSvg { page_id: None }),
        LayoutCommand::ExportPdf(semio_s_artifact_layout_layout_editor_commands::export_pdf::ExportPdf { page_id: None }),
        LayoutCommand::ExportStep(semio_s_artifact_layout_layout_editor_commands::export_step::ExportStep { page_id: None }),
        LayoutCommand::ExportIdml(semio_s_artifact_layout_layout_editor_commands::export_idml::ExportIdml {}),
        LayoutCommand::ExportPackage(semio_s_artifact_layout_layout_editor_commands::export_package::ExportPackage {}),
        LayoutCommand::EngagementSubmit(semio_s_artifact_layout_layout_editor_commands::engagement_submit::EngagementSubmit { value: "export png".into() }),
        LayoutCommand::DeleteSelection(semio_s_artifact_layout_layout_editor_commands::delete_selection::DeleteSelection {}),
        LayoutCommand::TranslateSelection(semio_s_artifact_layout_layout_editor_commands::gumball::TranslateSelection { ids: vec!["frame-1".into()], dx: 3.0, dy: 4.0, phase: Some("stream".into()), reason: None }),
        LayoutCommand::RotateSelection(semio_s_artifact_layout_layout_editor_commands::gumball::RotateSelection { ids: vec!["frame-1".into()], angle: 0.25, phase: None, reason: None }),
        LayoutCommand::ScaleSelection(semio_s_artifact_layout_layout_editor_commands::gumball::ScaleSelection { ids: vec!["frame-1".into()], sx: 1.5, sy: 1.25, phase: Some("abort".into()), reason: Some("captureLost".into()) }),
        LayoutCommand::PatchDocument(semio_s_artifact_layout_layout_editor_commands::patch_document::PatchDocument { field: "name".into(), value: "Press sheet".into() }),
    ]
}

//#region 🔖️ContextMenu
use semio_s_artifact_layout_layout_editor_panels::focus_preflight_issue::FocusPreflightIssue;
use semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::config::LayoutBlueprintWindowConfigOwner;
use semio_s_artifact_layout_layout_editor::unit_tests::context::LayoutApp;
use semio_framework_plugin::{ActionMeta, INTERACTION_SELECT_ACTION_ID, ViewModel, ViewWindowInstance, WindowConfigOwner};

async fn context_menu_json(app: &mut LayoutApp, surface: Option<semio_framework_plugin::ContextMenuSurfaceTarget>) -> String {
    use semio_framework_plugin::{ContextMenuRequest, PluginApp, UiMenuRef};
    let request = ContextMenuRequest { menu: UiMenuRef { id: "layout".into(), args: None }, surface, window_instance_id: None, point: None };
    serde_json::to_string(&app.context_menu(&request, &semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await).unwrap_or_default()
}

/// ⚖️ LAW: an empty blueprint canvas offers creation verbs and never delete-selection.
#[semio_framework_async_macros::async_test]
async fn context_menu_on_empty_canvas_offers_create_verbs_not_delete() {
    let mut app = layout_app_with_registry().await;
    let menu = context_menu_json(
        &mut app,
        Some(semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::LAYOUT_PLAY_SURFACE_BLUEPRINT.into(),
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
    app.close_fixture();
}

/// ⚖️ LAW: a frame selection ends with delete-selection and clearSelection.
#[semio_framework_async_macros::async_test]
async fn context_menu_on_frame_selection_offers_delete_last() {
    let mut app = layout_app_with_registry().await;
    let frame_id = app.snapshot().expect("projection").pages[0].frames.first().map(|frame| frame.id().to_string()).unwrap_or_else(|| "frame-1".into());
    let menu = context_menu_json(
        &mut app,
        Some(semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::LAYOUT_PLAY_SURFACE_BLUEPRINT.into(),
            kind: "canvas2d".into(),
            hits: vec![],
            selection: vec![semio_framework_plugin::ContextMenuSelectionGroup { domain: semio_s_artifact_layout_layout_editor_commands::LAYOUT_GRANULARITY_ELEMENT.into(), ids: vec![frame_id] }],
            text: None,
        }),
    )
    .await;
    assert!(menu.contains(r#""action":"deleteSelection""#), "selection menu must include deleteSelection: {menu}");
    assert!(menu.contains(r#""action":"clearSelection""#), "selection menu must include clearSelection: {menu}");
    app.close_fixture();
}

/// ⚖️ LAW: a page-tree row hit offers setActivePage, not canvas delete verbs.
#[semio_framework_async_macros::async_test]
async fn context_menu_on_page_row_offers_set_active_page() {
    let mut app = layout_app_with_registry().await;
    let page_id = app.snapshot().expect("projection").pages.first().map(|page| page.id.clone()).unwrap_or_else(|| "page-1".into());
    let menu = context_menu_json(
        &mut app,
        Some(semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: semio_s_artifact_layout_layout_editor_panels::document::LAYOUT_PLAY_BODY_ARTIFACT.into(),
            kind: "tree".into(),
            hits: vec![semio_framework_plugin::ContextMenuHit { domain: "row".into(), id: format!("layout-document.page.{page_id}"), label: None }],
            selection: vec![],
            text: None,
        }),
    )
    .await;
    assert!(menu.contains(r#""action":"setActivePage""#), "page row menu must set active page: {menu}");
    assert!(!menu.contains(r#""action":"deleteSelection""#), "page row menu must not delete frames: {menu}");
    app.close_fixture();
}

/// ⚖️ LAW: preview is read-only — selectAll only, no authoring delete/create.
#[semio_framework_async_macros::async_test]
async fn context_menu_on_preview_surface_is_read_only() {
    let mut app = layout_app_with_registry().await;
    let menu = context_menu_json(
        &mut app,
        Some(semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::preview::LAYOUT_PLAY_SURFACE_PREVIEW.into(),
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
    app.close_fixture();
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
            surface_id: semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::LAYOUT_PLAY_SURFACE_BLUEPRINT.into(),
            kind: "canvas2d".into(),
            hits: vec![],
            selection: vec![semio_framework_plugin::ContextMenuSelectionGroup { domain: semio_s_artifact_layout_layout_editor_commands::LAYOUT_GRANULARITY_ELEMENT.into(), ids: frames }],
            text: None,
        }),
    )
    .await;
    assert!(menu.contains(r#""action":"deleteSelection""#), "multi selection must delete: {menu}");
    assert!(menu.contains("frames") || menu.contains("Rahmen"), "multi selection label should mention frames: {menu}");
    app.close_fixture();
}

/// ⚖️ LAW: a links-tree row hit offers interactionSelect over referencing image frames.
#[semio_framework_async_macros::async_test]
async fn context_menu_on_link_row_offers_select_linked_frames() {
    let mut app = layout_app_with_registry().await;
    let link_id = app.snapshot().expect("projection").links.first().map(|link| link.id.clone()).expect("demo link");
    let menu = context_menu_json(
        &mut app,
        Some(semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: semio_s_artifact_layout_layout_editor_panels::document::LAYOUT_PLAY_BODY_ARTIFACT.into(),
            kind: "tree".into(),
            hits: vec![semio_framework_plugin::ContextMenuHit { domain: "row".into(), id: format!("layout-document.link.{link_id}"), label: None }],
            selection: vec![],
            text: None,
        }),
    )
    .await;
    assert!(menu.contains(INTERACTION_SELECT_ACTION_ID), "link row must select referencing frames: {menu}");
    assert!(menu.contains("frame-image-1"), "link row must target the demo image frame: {menu}");
    app.close_fixture();
}

/// ⚖️ LAW: a preflight issue row hit offers focusPreflightIssue with the issue payload.
#[semio_framework_async_macros::async_test]
async fn context_menu_on_preflight_row_offers_focus_issue() {
    let mut app = layout_app_with_registry().await;
    let snapshot = app.snapshot().expect("projection");
    let labels = layout_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native));
    let issue = run_layout_preflight(&snapshot, &labels).into_iter().next().expect("demo document must surface at least one preflight issue");
    let row_id = format!("layout-preflight.{}.{}", issue.code, issue.object_id.clone().unwrap_or_else(|| issue.message.clone()));
    let menu = context_menu_json(
        &mut app,
        Some(semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: semio_s_artifact_layout_layout_editor_panels::preflight::LAYOUT_PLAY_BODY_PREFLIGHT.into(),
            kind: "tree".into(),
            hits: vec![semio_framework_plugin::ContextMenuHit { domain: "row".into(), id: row_id, label: None }],
            selection: vec![],
            text: None,
        }),
    )
    .await;
    assert!(menu.contains(r#""action":"focusPreflightIssue""#), "preflight row must focus the issue: {menu}");
    assert!(menu.contains(&issue.code), "preflight menu must carry the issue code: {menu}");
    app.close_fixture();
}

/// ⚖️ LAW: right-clicking an unselected frame hit offers Select before create verbs.
#[semio_framework_async_macros::async_test]
async fn context_menu_on_frame_hit_without_selection_offers_select() {
    let mut app = layout_app_with_registry().await;
    let frame_id = "frame-1".to_string();
    let menu = context_menu_json(
        &mut app,
        Some(semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::LAYOUT_PLAY_SURFACE_BLUEPRINT.into(),
            kind: "canvas2d".into(),
            hits: vec![semio_framework_plugin::ContextMenuHit { domain: semio_s_artifact_layout_layout_editor_commands::LAYOUT_GRANULARITY_ELEMENT.into(), id: frame_id.clone(), label: None }],
            selection: vec![],
            text: None,
        }),
    )
    .await;
    assert!(menu.contains(r#""id":"select-hit""#), "frame hit must offer select: {menu}");
    assert!(menu.contains(INTERACTION_SELECT_ACTION_ID), "select must dispatch interactionSelect: {menu}");
    app.close_fixture();
}

/// ⚖️ LAW: a single text-frame selection adds a same-kind create row in the create group.
#[semio_framework_async_macros::async_test]
async fn context_menu_on_text_frame_selection_offers_same_kind_create() {
    let mut app = layout_app_with_registry().await;
    let frame_id = "frame-text-1".to_string();
    let menu = context_menu_json(
        &mut app,
        Some(semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::LAYOUT_PLAY_SURFACE_BLUEPRINT.into(),
            kind: "canvas2d".into(),
            hits: vec![],
            selection: vec![semio_framework_plugin::ContextMenuSelectionGroup { domain: semio_s_artifact_layout_layout_editor_commands::LAYOUT_GRANULARITY_ELEMENT.into(), ids: vec![frame_id] }],
            text: None,
        }),
    )
    .await;
    assert!(menu.contains(r#""id":"add-text-frame""#), "text selection must offer add text frame: {menu}");
    assert!(menu.contains(r#""kind":"text""#), "create row must target text frames: {menu}");
    app.close_fixture();
}

/// ⚖️ LAW: when the surface carries no selection, the live interaction snapshot supplies element ids.
#[semio_framework_async_macros::async_test]
async fn context_menu_uses_live_interaction_when_surface_selection_empty() {
    let mut app = layout_app_with_registry().await;
    let frame_id = app.snapshot().expect("projection").pages[0].frames.first().expect("frame").id().to_string();
    let view = ViewModel { window_instances: vec![ViewWindowInstance { id: "layout-blueprint".into(), window_kind_id: LayoutBlueprintWindowConfigOwner::WINDOW_KIND_ID.into() }], ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let meta = ActionMeta { view_state: Some(view.for_window_instance("layout-blueprint").expect("blueprint window instance")), ..artifact_app_laws::meta("local") };
    app.bind_instance_id(meta.instance_id).await;
    semio_framework_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &LayoutCommand::FocusPreflightIssue(FocusPreflightIssue { object_id: Some(frame_id.clone()), page_id: Some("page-1".into()) }), &meta, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("select frame");
    artifact_app_laws::settle_registered_typed_operation(&mut app.0, meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("focus settles");
    let menu = context_menu_json(
        &mut app,
        Some(semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::LAYOUT_PLAY_SURFACE_BLUEPRINT.into(),
            kind: "canvas2d".into(),
            hits: vec![],
            selection: vec![],
            text: None,
        }),
    )
    .await;
    assert!(menu.contains(r#""action":"deleteSelection""#), "live interaction selection must drive delete menu: {menu}");
    app.close_fixture();
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
        let args = semio_framework_value::ToValue::to_value(&command);
        let payload = match &args {
            semio_framework_value::DslValue::Object(entries) if entries.len() == 1 => entries[0].1.clone(),
            other => other.clone(),
        };
        let mut camel = camel_case_keys(&payload);
        if matches!(id, "canvasDragOver" | "canvasDrop") {
            let DslValue::Object(entries) = &mut camel else { panic!("catalogue payload object") };
            let index = entries.iter().position(|(key, _)| key == "kind").expect("typed catalogue kind");
            let (_, kind) = entries.remove(index);
            if id == "canvasDrop" {
                entries.push(("dragData".into(), DslValue::String(semio_framework_pack_json::to_json_string(&DslValue::Object(vec![("kind".into(), kind)])))));
            } else {
                let DslValue::String(kind) = kind else { panic!("typed catalogue kind text") };
                entries.push(("types".into(), DslValue::Array(vec![DslValue::String("application/x-semio-catalogue-item".into()), DslValue::String(format!("application/x-semio-catalogue-kind.{kind}"))])));
            }
        }
        let bridged = LayoutPlayApp::command_from_action(id, &mut Some((camel).clone()), &mut None).unwrap_or_else(|error| panic!("action {id} failed to bridge: {}", error.message));
        assert_eq!(bridged.command_id(), id, "command_id mismatch for action {id}");
        assert_eq!(bridged, command, "payload drifted through the bridge for action {id}");
    }
    assert!(LayoutPlayApp::command_from_action("nonsense", &mut None, &mut None).is_err());
}

/// 🐫️ The shell's spelling of the payload keys.
fn camel_case_keys(value: &semio_framework_value::DslValue) -> semio_framework_value::DslValue {
    match value {
        semio_framework_value::DslValue::Object(entries) => semio_framework_value::DslValue::Object(
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
    use semio_s_artifact_layout_layout_model::LayoutCamera;
    let args = |json: serde_json::Value| semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(&json.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("fixture JSON"));
    assert_eq!(
        LayoutPlayApp::command_from_action("addFrame", &mut Some((args(serde_json::json!({ "kind": "text", "x": 12, "y": 24.5 }))).clone()), &mut None).expect("addFrame bridge"),
        LayoutCommand::AddFrame(semio_s_artifact_layout_layout_editor_commands::add_frame::AddFrame { kind: "text".into(), x: Some(12.0), y: Some(24.5) })
    );
    assert_eq!(LayoutPlayApp::command_from_action("addFrame", &mut None, &mut None).expect("addFrame default kind"), LayoutCommand::AddFrame(semio_s_artifact_layout_layout_editor_commands::add_frame::AddFrame { kind: "rect".into(), x: None, y: None }));
    assert_eq!(LayoutPlayApp::command_from_action("addPage", &mut Some((args(serde_json::json!({ "windowId": "layout-blueprint" }))).clone()), &mut None).expect("addPage ignores window keys"), LayoutCommand::AddPage(semio_s_artifact_layout_layout_editor_commands::add_page::AddPage {}));
    assert_eq!(
        LayoutPlayApp::command_from_action("patchFrame", &mut Some((args(serde_json::json!({ "frameId": "frame-1", "field": "columns", "value": 2 }))).clone()), &mut None).expect("patchFrame bridge"),
        LayoutCommand::PatchFrame(semio_s_artifact_layout_layout_editor_commands::patch_frame::PatchFrame { frame_id: "frame-1".into(), page_id: None, field: "columns".into(), value: "2".into() })
    );
    assert_eq!(
        LayoutPlayApp::command_from_action("patchPage", &mut Some((args(serde_json::json!({ "pageId": "page-1", "field": "width", "value": "300" }))).clone()), &mut None).expect("patchPage bridge"),
        LayoutCommand::PatchPage(semio_s_artifact_layout_layout_editor_commands::patch_page::PatchPage { page_id: Some("page-1".into()), field: "width".into(), value: "300".into() })
    );
    assert_eq!(
        LayoutPlayApp::command_from_action("setActivePage", &mut Some((args(serde_json::json!({ "pageId": "page-2" }))).clone()), &mut None).expect("setActivePage bridge"),
        LayoutCommand::SetActivePage(semio_s_artifact_layout_layout_editor_commands::set_active_page::SetActivePage { page_id: "page-2".into() })
    );
    assert_eq!(
        LayoutPlayApp::command_from_action("setCamera", &mut Some((args(serde_json::json!({ "surfaceId": "layout.play.blueprint", "x": 1, "y": 2, "zoom": 1.5 }))).clone()), &mut None).expect("flat camera bridge"),
        LayoutCommand::SetCamera(semio_s_artifact_layout_layout_editor_commands::set_camera::SetCamera { surface_id: Some("layout.play.blueprint".into()), camera: LayoutCamera { x: 1.0, y: 2.0, zoom: 1.5 } })
    );
    assert_eq!(
        LayoutPlayApp::command_from_action("canvasPointerDown", &mut Some((args(serde_json::json!({ "surfaceId": "layout.play.blueprint", "button": 0, "shiftKey": true, "x": 10, "y": 20, "width": 800, "height": 600 }))).clone()), &mut None).expect("pointer down bridge"),
        LayoutCommand::CanvasPointerDown(semio_s_artifact_layout_layout_editor_commands::canvas_pointer_down::CanvasPointerDown { surface_id: Some("layout.play.blueprint".into()), button: 0, extend: true, x: 10.0, y: 20.0, width: 800.0, height: 600.0 })
    );
    assert_eq!(
        LayoutPlayApp::command_from_action("engagementSubmit", &mut Some((args(serde_json::json!({ "value": "export png" }))).clone()), &mut None).expect("engagement bridge"),
        LayoutCommand::EngagementSubmit(semio_s_artifact_layout_layout_editor_commands::engagement_submit::EngagementSubmit { value: "export png".into() })
    );
    assert!(LayoutPlayApp::command_from_action("patchFrame", &mut Some((args(serde_json::json!({ "field": "fill" }))).clone()), &mut None).is_err(), "a required field must not be defaulted silently");
}
//#endregion 🔖️ActionBridge

//#region 🔖️ManifestSanity
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let json = serde_json::to_string(&create_layout_app()).expect("app definition json");
    for id in [LAYOUT_PLAY_WINDOW_BLUEPRINT, LAYOUT_PLAY_WINDOW_PREVIEW] {
        assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
    }
    assert!(json.contains(semio_s_artifact_layout_layout_editor_commands::modes::edit::LAYOUT_PLAY_MODE_EDIT), "mode missing from the manifest");
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
    for authoring in ["addFrame", "addPage", "patchPage", "patchFrame", "patchDocument"] {
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
    let doc = semio_s_artifact_layout_layout_model::io::text::snapshot::parse_dsl(semio_s_artifact_layout_layout_model::io::text::snapshot::LAYOUT_SAMPLE_TEXT).expect("sample fixture");
    assert_eq!(doc.schema, semio_s_artifact_layout_layout_model::LAYOUT_DOCUMENT_SCHEMA);
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
    for window_id in [LAYOUT_PLAY_WINDOW_BLUEPRINT, LAYOUT_PLAY_WINDOW_PREVIEW] {
        let view = semio_framework_plugin::ViewModel {
            window_id: Some(window_id.into()),
            window_instances: vec![semio_framework_plugin::ViewWindowInstance { id: window_id.into(), window_kind_id: window_id.into() }],
            ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
        };
        let engagements = app.window_engagements(&view).await;
        let engagement = engagements.get(window_id).unwrap_or_else(|| panic!("{window_id} engagement"));
        let status = engagement.status.as_ref().and_then(|rows| rows.first()).unwrap_or_else(|| panic!("{window_id} status"));
        assert!(status.text.contains("Page"), "{window_id} status must name the active page: {}", status.text);
        let input = engagement.input.as_ref().unwrap_or_else(|| panic!("{window_id} input"));
        assert_eq!(input.placeholder.as_deref(), Some("undo, redo, export png"));
    }
}

#[semio_framework_async_macros::async_test]
async fn registry_backed_add_frame_emits_operation() {
    let mut app = layout_app_with_registry().await;
    let before = app.snapshot().expect("projection").pages[0].frames.len();
    dispatch(&mut app, LayoutCommand::AddFrame(semio_s_artifact_layout_layout_editor_commands::add_frame::AddFrame { kind: "rect".into(), x: None, y: None })).await;
    assert_eq!(app.snapshot().expect("projection").pages[0].frames.len(), before + 1);
}

#[semio_framework_async_macros::async_test]
async fn registry_backed_pointer_move_is_view_only() {
    let mut app = layout_app_with_registry().await;
    let (sx, sy) = test_screen_point(0.0, 0.0, 1.0, 800.0, 600.0, 156.0, 220.0);
    let result = dispatch(&mut app, LayoutCommand::CanvasPointerMove(semio_s_artifact_layout_layout_editor_commands::canvas_pointer_move::CanvasPointerMove { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), x: sx, y: sy, width: 800.0, height: 600.0, samples: Vec::new() })).await;
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
    assert_eq!(schema, semio_s_artifact_layout_layout_model::LAYOUT_DOCUMENT_SCHEMA);
    let bytes = store::pack_rt::pack_value_from_base64(&json).expect("decode base64 pack");
    let decoded = <LayoutSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&bytes).expect("decode pack");
    assert_eq!(decoded, document);
}

#[semio_framework_async_macros::async_test]
async fn import_media_fields_in_sets_typed_dictionary() {
    let mut app = layout_app().await;
    let media = Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Intrinsic { schema: "form.dictionary".into(), value: semio_framework_value::DslValue::Object(vec![("name".into(),semio_framework_value::DslValue::String("Ada".into()))]) } };
    app.0.import_media("fields:in", media, &artifact_app_laws::meta("local"), app.1.cx()).await.expect("import fields:in");
    let document = app.snapshot().expect("projection");
    assert_eq!(document.data_fields,Some(semio_framework_value_dictionary::QuestionDictionary{entries:vec![semio_framework_value_dictionary::QuestionDictionaryEntry{question_id:"name".into(),value:semio_framework_value::DslValue::String("Ada".into())}]}));
}

#[semio_framework_async_macros::async_test]
async fn layout_io_exposes_declared_ports() {
    let io = LayoutPlayApp::io().expect("layout declares io");
    assert!(io.ports.iter().any(|port| port.id == "fields:in"));
    assert!(io.ports.iter().any(|port| port.id == "layout:out"));
}

#[semio_framework_async_macros::async_test]
async fn layout_io_declares_fields_in_and_layout_out_ports() {
    let io = semio_s_artifact_layout_layout_engine::layout_io();
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🧪️tests/🧫️fixtures/🛍️canvas-catalogue/🔣️.json" )).expect("neutral catalogue actions");
    for case in fixture["cases"].as_array().unwrap() {
        let args: semio_framework_value::DslValue = case["args"].clone().into();
        let result = LayoutPlayApp::command_from_action(case["action"].as_str().unwrap(), &mut Some((args).clone()), &mut None);
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
    use semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::config::LayoutBlueprintWindowConfigOwner;
    use semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::transient::LayoutBlueprintWindowTransientOwner;
    use semio_framework_plugin::{ActionMeta, ViewModel, ViewWindowInstance, WindowConfigOwner};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🧪️tests/🧫️fixtures/🛍️canvas-catalogue/🔣️.json")).unwrap();
    let view = ViewModel {
        window_instances: ["catalogue-left", "catalogue-right"].map(|id| ViewWindowInstance { id: id.into(), window_kind_id: LayoutBlueprintWindowConfigOwner::WINDOW_KIND_ID.into() }).into(),
        ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
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
                let command = LayoutPlayApp::command_from_action(action, &mut Some((args).clone()), &mut None).map_err(|error| error.message)?;
                semio_framework_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &command, &meta, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.map_err(|error| format!("{error:?}"))?;
                artifact_app_laws::settle_registered_typed_operation(&mut app.0, meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.map_err(|error| format!("{error:?}"))?;
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
        app.close_fixture();
        outcome.unwrap_or_else(|error| panic!("{kind}: {error}"));
    }
}

//#region 🔤️PackageFonts
mod package_fonts {
    use semio_s_artifact_layout_layout_editor::LayoutPlayApp;
    use semio_s_artifact_layout_layout_engine::scene::{layout_engine_and_views::LayoutEngine, text_fonts::text_font_asset_declarations};
    use semio_framework_plugin::reactor::{admit_package_assets_for_testing, package_asset};

    fn catalog() -> std::path::PathBuf {
        let relative = std::path::Path::new(semio_s_artifact_layout_layout_engine::scene::text_fonts::TEXT_FONT_ASSET_ROOT);
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().find_map(|ancestor| ancestor.join(relative).is_dir().then(|| ancestor.to_path_buf())).expect("the repository root sits above the editor package")
    }

    #[test]
    fn the_plugin_declares_assets_whose_bytes_the_open_event_delivers_to_the_app_and_the_engine_shapes_with_them() {
        let root = catalog();
        let delivered: Vec<(String, Vec<u8>)> = text_font_asset_declarations().into_iter().map(|declaration| (declaration.name.clone(), std::fs::read(root.join(&declaration.name)).expect("declared font file"))).collect();
        assert_eq!(delivered.len(), 15);
        admit_package_assets_for_testing(delivered);
        assert!(text_font_asset_declarations().iter().all(|declaration| package_asset(&declaration.name).is_some_and(|bytes| bytes.len() as u64 == declaration.size_bytes)), "every declared font must read back through the SDK accessor");
        let _app = LayoutPlayApp::default();
        let mut engine = LayoutEngine::new();
        let shaped = engine.layout_story(&semio_s_artifact_layout_layout_model::TextStory { id: "story".into(), content: "Hello".into(), style_runs: Vec::new() }, &semio_s_artifact_layout_layout_model::ParagraphStyle { id: "p".into(), name: "p".into(), font_family: "Layout Sans".into(), font_size: 12.0, font_weight: 400, leading: 14.4, tracking: 0.0, alignment: "left".into() }, 200.0, 100.0);
        assert_eq!(shaped.0.glyphs.len(), 5);
        assert!(shaped.0.glyphs.iter().all(|glyph| glyph.glyph_id != 0), "the layout font and the delivered built-in families shape real glyphs");
    }
}
//#endregion 🔤️PackageFonts

#[test]
fn interchange_shell_actions_decode_the_selected_page_and_document_payloads(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🚪️io/🔀️interchange/🧪️fixtures/🔣️.json")).unwrap();let case=fixture["cases"].as_array().unwrap().iter().find(|case|case["id"]=="interchange-shell-controls").unwrap();
    for action in case["actions"].as_array().unwrap(){let id=action["id"].as_str().unwrap();let command=LayoutPlayApp::command_from_action(id,&mut Some(semio_framework_pack_json::from_json_str::<DslValue>(&action["args"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()),&mut None).unwrap();assert_eq!(command.command_id(),id);match command{LayoutCommand::ExportStep(payload)=>assert_eq!(payload.page_id.as_deref(),Some("selected-page")),LayoutCommand::ExportIdml(_)=>{},_=>panic!("interchange export command")}}
}

#[semio_framework_async_macros::async_test]
async fn nested_preflight_action_selects_and_switches_its_addressed_window_page() {
    use semio_framework_plugin::ArtifactEditor;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🔀️args-bridge/🧪️fixture.json")).unwrap();
    let case = &fixture["cases"][0];
    let mut args = Some(semio_framework_pack_json::from_json_str::<DslValue>(&case["args"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
    let command = LayoutPlayApp::command_from_action("focusPreflightIssue", &mut args, &mut None).unwrap();
    let mut app = layout_app().await;
    dispatch(&mut app, command).await;
    let selection = app.interaction_state().await;
    assert_eq!(selection.selection.get(semio_s_artifact_layout_layout_editor_commands::LAYOUT_INTERACTION_ELEMENTS).unwrap().ids, vec![case["expected"]["objectId"].as_str().unwrap().to_string()]);
    let projection = semio_framework_plugin::artifact_app_laws::render_registered_fixture_app(&mut app.0, LAYOUT_PLAY_BODY_INSPECTION, &context::windowed_view().for_window_instance(context::WINDOW).unwrap(), app.1.cx()).await.unwrap();
    assert!(projection.contains(&format!("\"description\":\"{}\"", case["expected"]["pageId"].as_str().unwrap())), "the focused page must be the addressed window inspector summary: {projection}");
}

#[test]
fn read_only_context_keeps_actual_media_identity_and_denies_entity_authoring() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🧪️tests/🧫️fixtures/🪪️entity-id/🔣️.json")).unwrap();
    for row in fixture["readOnly"].as_array().unwrap() {
        let operation=semio_framework_plugin::__job::Operation::new(semio_framework_plugin::__job::OperationId(row["operationId"].as_u64().unwrap()),semio_framework_plugin::__job::RevisionId(17),semio_framework_plugin::__job::Generation(row["generation"].as_u64().unwrap()),19);
        let retained=semio_framework_value::RetainedCloneGrant{maximum_items:3,maximum_copy_bytes:71,maximum_capacity_bytes:83,maximum_release_bytes:97,maximum_depth:5};
        let context=semio_framework_plugin::AppOperationContext::for_read_only(row["appInstanceId"].as_u64().unwrap()as u32,row["documentId"].as_str().unwrap().into(),operation,[row["canonicalRevisionByte"].as_u64().unwrap()as u8;32],retained);
        assert_eq!(context.app_instance_id,row["appInstanceId"].as_u64().unwrap()as u32);
        assert_eq!(context.parent_document_id,row["documentId"].as_str().unwrap());
        assert_eq!((context.operation_id,context.generation),(operation.operation.0,operation.generation.0));
        assert_eq!(context.canonical_base_revision,[row["canonicalRevisionByte"].as_u64().unwrap()as u8;32]);
        assert_eq!(context.retained,retained);
        assert_eq!(context.entity_id("frame","created").is_ok(),row["authoringAccepted"].as_bool().unwrap());
    }
}

#[semio_framework_async_macros::async_test]
async fn original_media_export_handoff_preserves_snapshot_and_closes_completed_or_cancelled_custody() {
    use semio_framework_plugin::artifact_app_laws::{submit_fixture_media_export,poll_fixture_media_export,fixture_close_grant};
    use semio_framework_plugin::ArtifactMediaExportPoll;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🎥️media-export-job-and-factories/🧪️fixture.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap(){
        let mut app=layout_app().await;
        let before=app.snapshot().expect("original projection");
        let close=fixture_close_grant(semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);
        let handle=submit_fixture_media_export(&mut app.0,fixture["port"].as_str().unwrap(),app.1.cx(),close).await.expect("real media request admission");
        let cancelled=row["cancel"].as_bool().unwrap();
        if cancelled{app.0.cancel_media_export(&handle).await.expect("cancel actual media handle")}
        let mut terminal=false;
        for _ in 0..artifact_app_laws::OriginalFixtureCaller::maximum_turns(){
            match poll_fixture_media_export(&mut app.0,&handle,app.1.cx(),close).await.expect("actual media poll"){
                ArtifactMediaExportPoll::Running{..}=>{},
                ArtifactMediaExportPoll::Cancelled=>{assert!(cancelled);println!("[media-handoff] {}",serde_json::json!({"name":row["name"],"cancelled":true,"unchanged":app.snapshot().unwrap()==before}));terminal=true;break},
                ArtifactMediaExportPoll::Complete(result)=>{assert!(!cancelled);assert_eq!(result.schema,fixture["schema"].as_str().unwrap());assert_eq!(result.mime_type,fixture["mime"].as_str().unwrap());let media=result.into_batch_media().expect("actual original chunks");let MediaPayload::Structured{schema,json}=media.payload else{panic!("real vector text payload")};let projection:serde_json::Value=serde_json::from_str(&json).expect("actual vector projection JSON");assert_eq!(projection["schema"],fixture["projectionSchema"]);assert_eq!(projection["source"]["documentId"],handle.parent_document_id);let revision=projection["source"]["frontierRevision"].as_str().unwrap();assert_eq!(revision.len(),64);assert!(revision.bytes().all(|byte|byte.is_ascii_hexdigit()&&!byte.is_ascii_uppercase()));assert!(projection["svg"].as_str().unwrap().contains("<svg"));println!("[media-handoff] {}",serde_json::json!({"name":row["name"],"schema":schema,"mime":fixture["mime"],"text":json,"cancelled":false,"unchanged":app.snapshot().unwrap()==before}));terminal=true;break},
                ArtifactMediaExportPoll::Failed(reason)=>panic!("original media export refused: {reason}"),
            }
        }
        assert!(terminal,"actual submitted media must terminate");
        assert_eq!(app.snapshot().unwrap(),before);
        app.close_fixture();
    }
}

store::space_members! {
    native_producer:&::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer,
    history_limits:semio_framework_os_kernel::os_store::HistoryReadLimits{file_bytes:67108864,frame_body_bytes:1048576,records:8192,maximum_fields:128,maximum_text_bytes:1048576,maximum_binary_bytes:1048576,maximum_json_bytes:1048576,maximum_json_allocation_bytes:8388608,maximum_json_items:65536,maximum_json_depth:64},
    maximum_history_allocation_bytes:8388608,
    pub enum MediaDrawingMembers, MediaDrawingMembersOpen {
        Drawing("s.draw.drawing", "1", "*", "drawing.document") => (semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot, semio_s_artifact_draw_drawing_mutations::DrawingMutation, semio_s_artifact_draw_drawing_editor_store_preparation::DrawingMemberStoreProducer, semio_s_artifact_draw_drawing_editor_store_preparation::DrawingMemberStoreProducer),
    }
}

#[semio_framework_async_macros::async_test]
async fn original_media_projection_reads_the_genuinely_registered_drawing_child() {
    use semio_framework_plugin::{App,EditorApp};
    use semio_framework_plugin::artifact_app_laws::{new_app_with_registry_and_members,OriginalFixtureScope,fixture_close_grant,submit_fixture_media_export,poll_fixture_media_export,close_registered_fixture_app};
    use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
    use semio_s_artifact_draw_drawing_mutations::DrawingMutation;
    fn manifest()->App{App{definition:create_layout_app(),examples:Vec::new()}}
    for cancel_after_output in [false,true]{
    let policy=semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
    let mut identity=artifact_app_laws::fixture_identity();
    let mut app=new_app_with_registry_and_members::<EditorApp<LayoutPlayApp>,MediaDrawingMembers>(manifest,protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()),policy,&mut identity).await;
    app.bind_instance_id(context::INSTANCE).await;
    let mut caller=OriginalFixtureScope::admit();
    let media_laws:serde_json::Value=serde_json::from_str(include_str!("../../🎥️media-export-job-and-factories/🧪️fixture.json")).unwrap();let child_law=&media_laws["mountedChild"];
    let transport:serde_json::Value=serde_json::from_str(include_str!("../../../🧬️schema/🧬️mutations/🖊️set-page-background-drawing/🧪️tests/🔣️transport.json")).unwrap();
    let mut document=app.snapshot().unwrap();document.pages[0].frames.clear();for layer in &mut document.pages[0].layers{layer.object_ids.clear()}
    document.pages[0].background_drawing=Some(semio_framework_pack_json::from_json_str(&transport["cases"][0]["input"]["SetPageBackgroundDrawing"]["drawing"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
    let member=document.pages[0].background_drawing.as_ref().unwrap();let child_id=member.handle.child_id.clone();let dialect=member.handle.target.dialect.clone();
    app.stores.store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply{mutations:vec![semio_s_artifact_layout_layout_mutations::LayoutMutation::ReplaceDocument(semio_s_artifact_layout_layout_mutations_content::replace_document::ReplaceDocument{document})],transaction:None},&mut identity).await.expect("real parent declares its Drawing member");
    let corpus:serde_json::Value=serde_json::from_str(include_str!("../../⚙️engine/🎬️scene/🧵️drawing/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let drawing:DrawingSnapshot=semio_framework_pack_json::from_json_str(&corpus["cases"][child_law["sourceCase"].as_u64().unwrap()as usize]["content"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let mut envelope=store::os_store::document_materialize::create_document_envelope::<DrawingSnapshot,DrawingMutation>("drawing.document",&child_id,drawing,None,&store::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);envelope.dialect=Some(dialect.clone());envelope.owner=Some(store::OwnerRef{parent:semio_framework_artifact_reference::ArtifactRef{artifact_id:app.stores.store.envelope().id.clone(),dialect:app.stores.store.envelope().dialect.clone().expect("actual parent dialect")},slot:child_law["slot"].as_str().unwrap().to_string(),child_id:child_id.clone()});
    let mut child=store::ArtifactStore::new::<store::os_store::ArtifactSnapshotPublication>(envelope,protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.unwrap();
    store::install_funded_member_store_owners(&mut child,&semio_s_artifact_draw_drawing_editor_store_preparation::DrawingMemberStoreProducer).expect("actual Drawing owner catalog");
    assert_eq!(child_id,child_law["childId"].as_str().unwrap());app.register_child(child_law["slot"].as_str().unwrap().to_string(),child_id.clone(),dialect,MediaDrawingMembers::Drawing(Box::new(child)),&mut identity).await.unwrap_or_else(|_|panic!("real parent reference admits its actual child"));
    let before=app.snapshot().unwrap();let close=fixture_close_grant(policy);
    let handle=submit_fixture_media_export(&mut app,"layout:out",caller.cx(),close).await.unwrap();let mut complete=false;let mut cancellation_bytes=None;
    for _ in 0..artifact_app_laws::OriginalFixtureCaller::maximum_turns(){match poll_fixture_media_export(&mut app,&handle,caller.cx(),close).await.unwrap(){semio_framework_plugin::ArtifactMediaExportPoll::Running{..}=>{if cancel_after_output&&cancellation_bytes.is_none(){let bytes=app.media.media_exports.get(handle.operation_id.0).map_or(0,|active|active.output_credit.bytes());if bytes>=child_law["cancelAfterOutputBytes"].as_u64().unwrap()as usize{cancellation_bytes=Some(bytes);app.cancel_media_export(&handle).await.unwrap()}}},semio_framework_plugin::ArtifactMediaExportPoll::Cancelled=>{assert!(cancel_after_output);let bytes=cancellation_bytes.expect("cancel only after actual child export output was credited");println!("[media-child-cancel] {}",serde_json::json!({"childId":child_id,"creditedBytes":bytes,"cancelled":true,"unchanged":app.snapshot().unwrap()==before}));complete=true;break},semio_framework_plugin::ArtifactMediaExportPoll::Complete(result)=>{assert!(!cancel_after_output,"cancellation cut must occur after actual child source use and before completion");let media=result.into_batch_media().unwrap();let MediaPayload::Structured{json,..}=media.payload else{panic!("actual projection JSON")};let value:serde_json::Value=serde_json::from_str(&json).unwrap();assert_eq!(value["source"]["documentId"],handle.parent_document_id);assert!(value["svg"].as_str().unwrap().contains(child_law["expectedSvgCommand"].as_str().unwrap()),"genuine child quadratic marker must survive composition");println!("[media-child] {}",serde_json::json!({"childId":child_id,"documentId":handle.parent_document_id,"projection":value,"unchanged":app.snapshot().unwrap()==before}));complete=true;break},other=>panic!("actual Drawing media unexpectedly terminated: {other:?}")}}
    assert!(complete);assert_eq!(app.snapshot().unwrap(),before);
    close_registered_fixture_app(&mut app,policy,caller.cx()).unwrap();caller.retire();
    }
}
