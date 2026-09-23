use super::*;
use crate::editor::layout::commands::{canvas_drag_leave, canvas_drag_over, canvas_drop, canvas_pointer_move, set_camera};
use crate::editor::layout::unit_tests::context::{dispatch, dispatch_in, layout_app, layout_app_with_registry, scene, test_screen_point, LayoutApp, PREVIEW_WINDOW};
use crate::editor::layout::modes::edit::windows::blueprint::config::LayoutBlueprintWindowConfigOwner;
use crate::editor::layout::{LayoutCommand, LAYOUT_INTERACTION_ELEMENTS, LAYOUT_PLAY_SURFACE_BLUEPRINT, LAYOUT_PLAY_SURFACE_PREVIEW};
use semio_framework::kernel::Effect;
use semio_framework_plugin::artifact_app_laws;
use semio_framework_plugin::{ActionMeta, PluginApp, ViewModel, ViewWindowInstance, WindowConfigOwner, INTERACTION_HOVER_ACTION_ID};

/// 🕹️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C: the interaction verbs a canvas gesture emits
/// (`Effect::DispatchAction { action ∈ INTERACTION_ACTION_IDS }`) are folded in-reactor by the
/// typed-operation ladder, so the only witness left is the selection/hover state itself — which
/// needs everything the plugin host supplies and the bare `dispatch` helper does not: the manifest
/// registry (the six verbs are `Migrated` rows there), a bound live instance, a `ViewModel` naming
/// the Blueprint window the gesture addresses (the retained work's `extent` refuses a command
/// without one), and the host's settle protocol ([`settled_dispatch`]) — exactly
/// `🧪️tests/🔬️window-ownership`'s recipe. Retire it with [`close_registered`], never drop it.
///
/// 🚧️ `canvasPointerDown`/`canvasPointerMove` are still `BatchOnlyPendingRewrite` in the manifest
/// (no retained factory row), so `dispatch_typed` refuses them with `interactive-job.missing-factory`
/// before any fold — the same pre-existing gate `registry_backed_pointer_move_is_view_only` hits.
/// The assertions below are what holds the moment those two rows migrate.
async fn registered_layout_app() -> LayoutApp {
    let mut app = layout_app_with_registry().await;
    app.bind_instance_id(blueprint_meta().instance_id).await;
    app
}

fn blueprint_meta() -> ActionMeta {
    let view = ViewModel { window_instances: vec![ViewWindowInstance { id: "layout-blueprint".into(), window_kind_id: LayoutBlueprintWindowConfigOwner::WINDOW_KIND_ID.into() }], ..Default::default() };
    ActionMeta { view_state: Some(view.for_window_instance("layout-blueprint").expect("blueprint window instance")), ..artifact_app_laws::meta("local") }
}

fn close_registered(mut app: LayoutApp) {
    artifact_app_laws::close_registered_fixture_app(&mut app.0);
}

/// 🔁️ Dispatches one command against the Blueprint window and settles its retained publication the
/// way the plugin host does; the receipt's `effects` are exactly what the host would have been handed.
async fn settled_dispatch(app: &mut LayoutApp, command: LayoutCommand) -> artifact_app_laws::TypedOperationFixtureReceipt {
    let meta = blueprint_meta();
    let result = app.dispatch_typed(command, &meta).await.expect("dispatch");
    assert!(result.mutations.is_empty(), "canvas gestures never mutate the document directly");
    artifact_app_laws::settle_registered_typed_operation(&mut app.0, meta.instance_id).await.expect("retained publication settles")
}

async fn selected_elements(app: &LayoutApp) -> Vec<String> {
    app.interaction_state().await.selection.get(LAYOUT_INTERACTION_ELEMENTS).map(|selection| selection.ids.clone()).unwrap_or_default()
}

#[semio_framework_async_macros::async_test]
async fn set_camera_mutates_config_and_emits_no_operations() {
    let mut app = layout_app().await;
    let before = app.snapshot().expect("projection");
    let result = dispatch(&mut app, LayoutCommand::SetCamera(set_camera::SetCamera { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), camera: LayoutCamera { x: 10.0, y: 20.0, zoom: 1.5 } })).await;
    assert!(result.mutations.is_empty(), "camera is a config action and emits no operations");
    assert_eq!(app.snapshot().expect("projection"), before, "camera never mutates the document");
}

/// 📷️ The camera is owned by the addressed WINDOW instance (`blueprint::config::addressed`), so a
/// `setCamera` the preview window dispatches moves the preview scene and leaves the blueprint's alone.
/// The scene rides packed inside the surface doc, so the camera is read off the decoded scene.
#[semio_framework_async_macros::async_test]
async fn set_camera_preview_surface_updates_independently_of_blueprint() {
    let mut app = layout_app().await;
    dispatch_in(&mut app, LayoutCommand::SetCamera(set_camera::SetCamera { surface_id: Some(LAYOUT_PLAY_SURFACE_PREVIEW.into()), camera: LayoutCamera { x: 3.0, y: 4.0, zoom: 2.0 } }), PREVIEW_WINDOW).await;
    let preview = scene(&mut app, crate::editor::layout::modes::edit::windows::preview::LAYOUT_PLAY_BODY_PREVIEW).await;
    assert_eq!((preview.camera_x, preview.camera_y, preview.zoom), (3.0, 4.0, 2.0), "preview scene reflects its window's camera");
    let blueprint = scene(&mut app, crate::editor::layout::modes::edit::windows::blueprint::LAYOUT_PLAY_BODY_BLUEPRINT).await;
    assert_eq!((blueprint.camera_x, blueprint.camera_y), (0.0, 0.0), "blueprint window camera stays independent");
}

/// 🕹️ Selection is framework-owned: a hit never mutates config, it emits `interactionSelect`
/// (`Effect::DispatchAction`) — and since ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C the reactor
/// folds that verb INLINE in the same turn, so the effect never leaves the reactor and the
/// witness is the selection snapshot itself: the hit frame is selected when the pointer-down
/// result returns. Fails-before: the effect was bounced to the host and selection landed one
/// guest round trip later.
#[semio_framework_async_macros::async_test]
async fn pointer_down_selects_the_hit_frame_inline() {
    let mut app = registered_layout_app().await;
    let (sx, sy) = test_screen_point(0.0, 0.0, 1.0, 800.0, 600.0, 136.0, 435.0);
    let receipt = settled_dispatch(&mut app, LayoutCommand::CanvasPointerDown(CanvasPointerDown { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), button: 0, extend: false, x: sx, y: sy, width: 800.0, height: 600.0 })).await;
    assert!(!receipt.effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { .. } | Effect::ReplayShellCommand { .. })), "the interaction verb is folded in-reactor, never handed to the host: {:?}", receipt.effects);
    assert_eq!(selected_elements(&app).await, vec!["frame-image-1".to_string()], "a replace pick selects exactly the hit frame inside the carrying turn");
    close_registered(app);
}

#[semio_framework_async_macros::async_test]
async fn pointer_down_extend_click_inverts_the_hit_frame_inline() {
    let mut app = registered_layout_app().await;
    let (sx, sy) = test_screen_point(0.0, 0.0, 1.0, 800.0, 600.0, 136.0, 435.0);
    settled_dispatch(&mut app, LayoutCommand::CanvasPointerDown(CanvasPointerDown { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), button: 0, extend: true, x: sx, y: sy, width: 800.0, height: 600.0 })).await;
    assert_eq!(selected_elements(&app).await, vec!["frame-image-1".to_string()], "an invertive pick on an unselected frame selects it");
    // ⚖️ `dispatch_interaction_action` never lets a fold with targets present empty the selection —
    // an `Invertive` pick that would toggle the last frame off re-selects the targets instead
    // (`next.ids.is_empty() && !targets.is_empty()` fallback), so the frame stays selected.
    settled_dispatch(&mut app, LayoutCommand::CanvasPointerDown(CanvasPointerDown { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), button: 0, extend: true, x: sx, y: sy, width: 800.0, height: 600.0 })).await;
    assert_eq!(selected_elements(&app).await, vec!["frame-image-1".to_string()], "a second invertive pick on the sole selected frame keeps it selected (framework fallback)");
    close_registered(app);
}

#[semio_framework_async_macros::async_test]
async fn pointer_down_on_empty_space_clears_the_selection_inline() {
    let mut app = registered_layout_app().await;
    let (hit_x, hit_y) = test_screen_point(0.0, 0.0, 1.0, 800.0, 600.0, 136.0, 435.0);
    settled_dispatch(&mut app, LayoutCommand::CanvasPointerDown(CanvasPointerDown { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), button: 0, extend: false, x: hit_x, y: hit_y, width: 800.0, height: 600.0 })).await;
    assert_eq!(selected_elements(&app).await.len(), 1, "a frame is selected before the background click");
    let (sx, sy) = test_screen_point(0.0, 0.0, 1.0, 800.0, 600.0, 5.0, 5.0);
    let receipt = settled_dispatch(&mut app, LayoutCommand::CanvasPointerDown(CanvasPointerDown { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), button: 0, extend: false, x: sx, y: sy, width: 800.0, height: 600.0 })).await;
    assert!(!receipt.effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { .. })), "clearSelection is folded in-reactor: {:?}", receipt.effects);
    assert!(selected_elements(&app).await.is_empty(), "a background click clears the selection inside the carrying turn");
    close_registered(app);
}

#[semio_framework_async_macros::async_test]
async fn pointer_move_hovers_the_hit_frame_inline() {
    let mut app = registered_layout_app().await;
    let (sx, sy) = test_screen_point(0.0, 0.0, 1.0, 800.0, 600.0, 156.0, 220.0);
    let receipt = settled_dispatch(&mut app, LayoutCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), x: sx, y: sy, width: 800.0, height: 600.0, samples: Vec::new() })).await;
    assert!(!receipt.effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { .. })), "interactionHover is folded in-reactor: {:?}", receipt.effects);
    let hovered = app.interaction_state().await.hover.get(LAYOUT_INTERACTION_ELEMENTS).map(|hover| hover.ids.clone()).unwrap_or_default();
    assert_eq!(hovered, vec!["frame-text-1".to_string()], "the hit frame is hovered inside the carrying turn");
    close_registered(app);
}

/// 🧪️ Handler-level fixture: the default document + default window config, as `handle` sees them
/// (the `dispatch` route is gated by the tool-proof catalog and `canvasPointerMove`'s
/// `BatchOnlyPendingRewrite` classification, which are framework-owned).
fn hover_targets(payload: &canvas_pointer_move::CanvasPointerMove) -> String {
    let document = crate::standards::v1::subsets::any::schema::default_document();
    let history = semio_framework_plugin::HistoryView::empty();
    let view = ArtifactView::new(&document, &history);
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = canvas_pointer_move::handle(payload, &view, &cfg).expect("hover");
    assert!(emit.artifact_mutations.is_empty(), "hover never mutates the document directly");
    let hovers: Vec<_> = emit.effects.iter().filter(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == INTERACTION_HOVER_ACTION_ID)).collect();
    assert_eq!(hovers.len(), 1, "one hover per batch, not one per sample");
    let Effect::DispatchAction { args, .. } = hovers[0] else { unreachable!() };
    let args = args.clone().map(store::pack_rt::dsl_value_to_json).expect("hover args");
    args["targets"].as_str().expect("targets json").to_string()
}

/// 🧵️ LAW (design L4 / §2 D): a batched move hover hit-tests its LAST sample only — the `x`/`y`
/// of the command and the intermediate samples (here over empty space) never win.
#[semio_framework_async_macros::async_test]
async fn a_batched_pointer_move_hovers_its_last_sample_only() {
    let (empty_x, empty_y) = test_screen_point(0.0, 0.0, 1.0, 800.0, 600.0, 5.0, 5.0);
    let (hit_x, hit_y) = test_screen_point(0.0, 0.0, 1.0, 800.0, 600.0, 156.0, 220.0);
    let single = hover_targets(&canvas_pointer_move::CanvasPointerMove { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), x: hit_x, y: hit_y, width: 800.0, height: 600.0, samples: Vec::new() });
    assert!(single.contains("frame-text-1"), "a legacy single-sample move hovers the frame: {single}");

    let samples = vec![[empty_x, empty_y], [empty_x + 1.0, empty_y + 1.0], [empty_x + 2.0, empty_y + 2.0], [hit_x, hit_y]];
    let batched = hover_targets(&canvas_pointer_move::CanvasPointerMove { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), x: hit_x, y: hit_y, width: 800.0, height: 600.0, samples });
    assert_eq!(batched, single, "four samples ending on the frame hover exactly what one move there did");

    // 🎯️ ...and a batch whose LAST sample is over empty space hovers nothing even though `x`/`y` name the frame.
    let ending_empty = hover_targets(&canvas_pointer_move::CanvasPointerMove { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), x: hit_x, y: hit_y, width: 800.0, height: 600.0, samples: vec![[hit_x, hit_y], [empty_x, empty_y]] });
    assert!(!ending_empty.contains("frame-text-1"), "the last sample wins over x/y: {ending_empty}");
}

/// 🚫️ LAW (design §2 D): a cancelled release selects and commits nothing (Layout has no drag
/// gesture, so it is inert exactly like a release); the wire defaults keep a legacy sender valid.
#[semio_framework_async_macros::async_test]
async fn a_cancelled_release_is_inert_and_the_wire_defaults_hold() {
    use dsl::FromValue;
    let document = crate::standards::v1::subsets::any::schema::default_document();
    let history = semio_framework_plugin::HistoryView::empty();
    let view = ArtifactView::new(&document, &history);
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = crate::editor::layout::commands::canvas_pointer_up::handle(&crate::editor::layout::commands::canvas_pointer_up::CanvasPointerUp { cancelled: true }, &view, &cfg).expect("cancel");
    assert!(emit.artifact_mutations.is_empty() && emit.effects.is_empty(), "a cancel never selects or commits");

    let f = dsl::DslValue::float;
    let legacy = dsl::DslValue::Object(vec![("x".into(), f(5.0)), ("y".into(), f(6.0)), ("width".into(), f(800.0)), ("height".into(), f(600.0))]);
    let moved = canvas_pointer_move::CanvasPointerMove::from_value(legacy).expect("legacy move decodes");
    assert_eq!(moved.samples_or_last(), vec![[5.0, 6.0]], "an absent `samples` is the single (x, y)");
    assert_eq!(moved.last_sample(), [5.0, 6.0]);
    let released = crate::editor::layout::commands::canvas_pointer_up::CanvasPointerUp::from_value(dsl::DslValue::Object(Vec::new())).expect("legacy release decodes");
    assert!(!released.cancelled, "an absent `cancelled` is a real release");
}

#[semio_framework_async_macros::async_test]
async fn canvas_drop_adds_frame_at_world_coords() {
    let mut app = layout_app().await;
    let (sx, sy) = test_screen_point(0.0, 0.0, 1.0, 800.0, 600.0, 100.0, 200.0);
    // 🧾️ A MOUNTED app publishes through its retained typed operation, so `result.mutations` is EMPTY
    // (fleet brief v2, stale-test bucket 3) — the settled document below is the committed record.
    let before = app.snapshot().expect("projection").pages[0].frames.len();
    dispatch(&mut app, LayoutCommand::CanvasDrop(canvas_drop::CanvasDrop { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), kind: "rect".into(), x: sx, y: sy, width: 800.0, height: 600.0, artifact_ref: String::new(), proxy_data_url: String::new() })).await;
    let doc = app.snapshot().expect("projection");
    assert_eq!(doc.pages[0].frames.len(), before + 1, "one drop appends exactly one frame");
    let frame = doc.pages[0].frames.last().unwrap();
    let bounds = frame.bounds();
    assert!((bounds.x - 100.0).abs() < 0.01);
    assert!((bounds.y - 200.0).abs() < 0.01);
}

#[semio_framework_async_macros::async_test]
async fn canvas_drop_page_kind_adds_page() {
    let mut app = layout_app().await;
    let before = app.snapshot().expect("projection").pages.len();
    // 🧾️ Mounted: the settled document is the committed record, never `result.mutations` (bucket 3).
    dispatch(&mut app, LayoutCommand::CanvasDrop(canvas_drop::CanvasDrop { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), kind: "page".into(), x: 0.0, y: 0.0, width: 800.0, height: 600.0, artifact_ref: String::new(), proxy_data_url: String::new() })).await;
    assert_eq!(app.snapshot().expect("projection").pages.len(), before + 1);
}

#[semio_framework_async_macros::async_test]
async fn drag_over_emits_ghost_and_leave_clears() {
    let mut app = layout_app().await;
    dispatch(&mut app, LayoutCommand::CanvasDragOver(canvas_drag_over::CanvasDragOver { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), kind: "rect".into(), x: 400.0, y: 300.0, width: 800.0, height: 600.0 })).await;
    assert!(scene(&mut app, crate::editor::layout::modes::edit::windows::blueprint::LAYOUT_PLAY_BODY_BLUEPRINT).await.layers_json.contains("layout.drop-preview"), "the blueprint window's transient carries the drop ghost");

    dispatch(&mut app, LayoutCommand::CanvasDragLeave(canvas_drag_leave::CanvasDragLeave {})).await;
    assert!(!scene(&mut app, crate::editor::layout::modes::edit::windows::blueprint::LAYOUT_PLAY_BODY_BLUEPRINT).await.layers_json.contains("layout.drop-preview"), "drag leave clears the ghost");
}
