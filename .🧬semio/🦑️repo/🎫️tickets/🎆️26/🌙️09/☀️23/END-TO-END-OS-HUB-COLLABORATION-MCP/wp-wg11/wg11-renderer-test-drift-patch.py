#!/usr/bin/env python3
"""🧪️ WG11 session 14d — renderer-wgpu test/fixture drift (rule 22: test-only, lands with native-lane proof).

LW1's full renderer lib run (`wg11-laws-5.txt`) → WG11 isolated re-run of every red under the canonical conditions
(`.🧬semio/🌐hub/s14-wg11-captures/isolated-1.tsv`): these reds are the LAW or its FIXTURE drifting from a correct product:

1. `embedded_glb_materials_publish_with_their_primitive_ranges_texture_and_vertex_colors` — the neutral GLB's embedded PNG
   (`🖱️ui/🧫️fixtures/🎨️world3d-glb-material/🧊️two-primitive.glb`, bufferView 8) carries a wrong zlib Adler-32 (`05fe02fe`, the
   scanline `01 00ff00ff` sums to `04060200`) and therefore a wrong IDAT CRC: Python's `zlib` refuses it ("incorrect data check"),
   so does the `image` crate. Same 70 bytes, same pixel; checksums recomputed and verified by `zlib.decompress` + `zlib.crc32`.
2. `the_swapchain_is_acquired_written_and_presented_in_one_prepared_opportunity` — the Present arm now opens with the offscreen
   early return (`cursor.phase = PreparedGpuPresentPhase::Complete;`), so cutting the arm at the first `…Complete` cut it before
   the acquire; the arm ends at the NEXT ARM (`PreparedGpuPresentPhase::Complete =>`).
3. `the_present_watchdog_signature_carries_within_item_upload_progress` — the signature tuple is one line now and ends in
   `cursor.input_progress)`; the law reads the tuple's terms instead of a trailing-comma spelling.
4. `the_sign_in_verb_arms_a_task_and_returns_while_the_hub_is_still_answering` — WG11's own shell-turn set (T3) made the hub
   verb dispatch `fn handle_hub_workspace_action<'a>(…) -> ShellTurn<'a, ()>`; the law splits on that signature.
5. `native_system_theme_change_merges_theme_into_the_pending_redraw_reason` — the fixture's `expectedReasons` name `theme`, the
   law's tag table did not.
6. `an_armed_gating_utility_disables_every_app_row_but_not_the_frameworks_own` — React's `FRAMEWORK_RESERVED_ACTION_IDS`
   (`🛠️ShellHelpers/🟦️.tsx:319`) and the renderer's both end in `exportArtifactDocument`, `importArtifactDocument`; the shared
   fixture lacked them.
7. `locale_roundtrip_preserves_instance_titles_and_localizes_singletons` — since 09-27 `dock_mode_layout_identity` deliberately
   projects React Mode's identity WITHOUT presentation titles, keyed by instance id; the law still read titles out of it. It now
   pins what that identity is for: every leaf keyed by its instance (or singleton kind) id, no title, the same across locales.
8. `the_command_dock_opens_the_expanded_commands_staged_form` — panel record keys are `<surface>/<id>` (`PanelProjection::key`);
   the law looked the form up by its bare id.
9. `icon_render_maps_the_shared_lighting_fixture_to_the_world_environment` — the icon environment's material now also carries the
   outline `stroke` (09-28, React's SVG/PNG outline default `#000000`, `📤️svg-export` fixture); the law compares the lighting
   fixture's own material fields and pins the stroke against the svg-export fixture.
10. the seeded-scene wiring laws — two platform changes the laws never followed: since 09-21 a retained ComponentScene's host
   id is ENGINE-MINTED (`reconcile::component_scene_host_id`), and since 09-27 every input, caret and accessibility address
   resolves in the PRESENTED tree, and only ACCEPTED scene cells reach the accessibility tree. The four `text_editor_*` laws and
   the NodeGraph caret law focused a seeded candidate that was never presented; the EventFeed and Table button laws staged
   controls under the AUTHORED fixture host id (so no control ever matched the mounted scene); the Table editable laws addressed
   drafts by the authored host id and never staged their cells. Shared helpers now present a seeded window
   (`present_seeded_scene_window`), read its mounted scene (`retained_scene`), rebase fixture keys onto the minted host
   (`rebased_scene_key`) and accept a Table's editable cells (`accept_table_editable_text_cells`).
11. `production_action_ingress_has_no_legacy_queue_and_text_vec_helpers_are_test_only` — the seven legacy EngineCanvas text-editor
   helpers were DELETED on 09-26 (stronger than test-only), and `text_editor_apply_completion` now names the Scenes' own
   production completion commit (Enter/Tab/click). The law pins that EngineCanvas declares none of the legacy helpers.
12. `text_editor_receipts_match_the_neutral_first_latest_refusal_and_read_only_laws` — the shared delivery fixture gained
   `typedAfterNewOwner` (09-27: the same host re-renders mid-flight — React re-renders with a new `onAction` and the guest's
   stale buffer — and typing continues); React's law replays it, the wgpu law ignored it and expected `abcd` from `abc`. The
   wgpu law now replays it too: a re-sync of the same host with the stale buffer, then the extra keys.
13. `the_platform_tree_is_the_fixture_tree_as_accesskit_reads_it` — 09-28 added `outline.b` (depth 1, a second chapter) to the
   native accessibility fixture's publication and its own expected node, but not to its parent's expected `children`; AccessKit's
   consumer (the third-party oracle) reads `outline` → [a, b], as the publication's depths say.
14. `closed_world3d_retires_every_input_and_scene_owner_before_id_reuse` / `a_focused_world_window_does_not_retire_its_hidden_sibling`
   — since 09-28 the committed DOCK is the retention authority (`live_window_ids`; the paint plan omits hidden siblings). The
   closed-window law still "closed" a window by editing the paint plan of windows the dock never held (so both retired), and the
   sibling law narrowed a paint plan the fixture had never planned (empty). The laws now put the windows in the dock and close
   through it, and plan before narrowing.
15. `board_and_map_two_touch_gestures_share_camera_math_but_keep_distinct_transfer_rules` — since 09-26 a map camera is clamped
   to cover its viewport (`clamp_camera_to_world_bounds`: an 800×600 map never zooms below 400), so the law's `[0, 0, 10]` seed
   lands on `[0, 0, 400]` and the fixture's ×2 spread ends on 800, not 20. The law reads the camera the map admitted and expects
   the fixture's own ratio (`expectedCamera.zoom / initialCamera.zoom`) about the centre.
16. `icon_export_effect_publishes_an_accessible_cancel_control_and_drains_on_activation` — hits minted by a frame build are
   resolvable only after `publish_hits` (09-14 staging); the law read the unpublished registry.
17. `rendered_footer_root_closes_settings_without_selecting_or_dragging_the_pending_panel_tab` — since 09-26 a panel tab
   announces `aria-pressed` (React `PanelTabButton`), never `selected`; the law asked for `selected == Some(false)`.
18. `display_window_kind_reaches_shell_as_a_transfer_handle_and_new_window_drag` — the law discards the closed instance's
   released journal by clearing `deferred_actions` directly, which left `window_topology_journal_dispatch_owed` set: the next
   topology refresh then yielded its settle step to a dispatch of nothing. The law clears the paired debt with it.
19. `refused_window_publication_does_not_block_a_ready_display_peer` — since WG8 the settle lane's owed refresh is detached (one
   step reads, a later step applies), so the refused peer's retry lands a step later than the law's fixed count; the law drives
   the bounded settle lane until both journals dispatched and then checks the order.
20. ui crate, the 09-28 GLB material commit (`3b2f1181d2`) drifted three wgpu laws: the World instance stride law (96 → 112, the
   authored emissive/cutoff lane), the encoded-attachment law's pipeline list (four `world3d_authored_*` pipelines target the
   encoded view — 15 formats, the list named 11) and the prepared ordering law, whose own label map was renamed to
   `world-postprocess` while its expected order still said `curvilinear`.
21. `ui_surface_slot_table_is_heap_first_and_fits_a_bounded_thread_stack` — the committed `ui::wgpu_engine` slot budget still reads
   164 672 B per `Option<UiSurfaceSlot>`; the live tree measures 164 712 (+40, a surface field added without its budget row;
   measured identically on WG11's overlay and on U6's). The row follows the measurement; the owner stays heap-first (520 B).

Dry run by default; `--write` backs every edited file up under `.🧬semio/🌐hub/s14-wg11-backup/renderer-test-drift/` and applies;
`--revert` restores the backups.
"""

import difflib
import shutil
import struct
import sys
import zlib
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
ENGINE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
GLB = ROOT / "🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🎨️world3d-glb-material/🧊️two-primitive.glb"
ASYNC_LAWS = ENGINE / "🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs"
HUB_LAWS = ENGINE / "🧱️elements/🐚️Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs"
WINIT_LAWS = ENGINE / "🧪️tests/🔬️wgpu-winit-app-p3c/🦀️.rs"
RESERVED_FIXTURE = ENGINE / "🧱️elements/🐚️Shell/🧫️fixtures/🎬️window-actions-search-panes/🔣️.json"
LOCALE_LAWS = ENGINE / "🧱️elements/🐚️Shell/🧪️tests/🪟️wgpu-window-pane-chrome/🦀️.rs"
PANEL_LAWS = ENGINE / "🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs"
ICON_RENDER_LAWS = ENGINE / "🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-icon-render/🦀️.rs"
WIRING_LAWS = ENGINE / "🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs"
STANDALONE_LAWS = ENGINE / "🧱️elements/🎞️Scenes/🧪️tests/🧊️wgpu-standalone/🦀️.rs"
RECEIPT_LAWS = ENGINE / "🧱️elements/⚙️EngineCanvas/🧪️tests/🖌️wgpu-paint2d-engine/🦀️.rs"
NATIVE_AX_FIXTURE = ENGINE / "🧫️fixtures/♿️native-accessibility-tree/🔣️.json"
LIFECYCLE_LAWS = ENGINE / "🧱️elements/🐚️Shell/🧪️tests/🪟️window-lifecycle-template-drag/🦀️.rs"
NAVBAR_LAWS = ENGINE / "🧱️elements/🐚️Shell/🧪️tests/🧭️wgpu-navbar-footer-parity/🦀️.rs"
ENGINE_SURFACE_LAWS = ENGINE / "🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs"
EXPORT_BATCH_LAWS = ENGINE / "🧱️elements/🖼️IconRenderHost/🧪️tests/📤️export-batch/🦀️.rs"
SETTINGS_LAWS = ENGINE / "🧱️elements/🐚️Shell/🧪️tests/⚙️settings-general-layout/🦀️.rs"
SHELL_INPUT_LAWS = ENGINE / "🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs"
UI_TESTS = ROOT / "🧰️framework/🔨️modules/🖱️ui/🧪️tests"
UI_DRAW_LAWS = UI_TESTS / "🔬️targets-wgpu-draw-unit/🦀️.rs"
UI_PRESENT_LAWS = UI_TESTS / "🔬️targets-wgpu-gpu-prepared-present/🦀️.rs"
UI_PREPARED_LAWS = UI_TESTS / "🔬️targets-wgpu-prepared-unit/🦀️.rs"
SLOT_BUDGET_FIXTURE = ROOT / "🧰️framework/🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json"
BACKUP = ROOT / ".🧬semio/🌐hub/s14-wg11-backup/renderer-test-drift"

ASYNC_EDITS = [
    (
        '''    let present = &present[..present.find("PreparedGpuPresentPhase::Complete").unwrap_or(present.len())];''',
        '''    let present = &present[..present.find("PreparedGpuPresentPhase::Complete =>").unwrap_or(present.len())];''',
    ),
    (
        '''    assert!(
        LIBRARY_SOURCE.contains("upload_progress,") && LIBRARY_SOURCE.contains("cursor.raster_keep_steps,") && LIBRARY_SOURCE.contains("cursor.input_progress,"),
        "upload, raster-ownership, and bounded input progress are independent signature terms"
    );''',
        '''    let stall = LIBRARY_SOURCE.split("fn note_present_stall(").nth(1).expect("the presenter's stall watch");
    let stall = &stall[..stall.find("\\n    }\\n").expect("the stall watch closes")];
    let signature = &stall[stall.find("(cursor.phase").expect("the stall signature tuple") + 1..];
    let terms: Vec<&str> = signature.split(", ").map(|term| term.trim_end_matches([')', ',', '\\n', ' '])).collect();
    assert!(["upload_progress", "cursor.raster_keep_steps", "cursor.input_progress"].iter().all(|term| terms.contains(term)), "upload, raster-ownership, and bounded input progress are independent signature terms: {terms:?}");''',
    ),
]

HUB_EDITS = [
    (
        '''    let dispatch = source.split("async fn handle_hub_workspace_action(").nth(1).expect("the hub verb dispatch exists");''',
        '''    let dispatch = source.split("fn handle_hub_workspace_action<'a>(").nth(1).expect("the hub verb dispatch exists");''',
    )
]

WINIT_EDITS = [
    (
        '''        Some("paint") => Some(InvalidationReason::PAINT),
        None => None,''',
        '''        Some("paint") => Some(InvalidationReason::PAINT),
        Some("theme") => Some(InvalidationReason::THEME),
        None => None,''',
    )
]

LOCALE_EDITS = [
    (
        '''        let titles: Vec<_> = axis.children.iter().flat_map(|child| match child { WindowLayoutChild::Stack(stack) => stack.children.iter().map(|leaf| leaf.title.as_deref().unwrap()).collect::<Vec<_>>(), _ => panic!("stack identity") }).collect();
        assert_eq!(titles, expected);''',
        '''        let leaves: Vec<_> = axis
            .children
            .iter()
            .flat_map(|child| match child {
                WindowLayoutChild::Stack(stack) => stack.children.iter().map(|leaf| (leaf.window_kind_id.as_str(), leaf.title.as_deref())).collect::<Vec<_>>(),
                _ => panic!("stack identity"),
            })
            .collect();
        assert_eq!(leaves, ids.iter().map(|id| (*id, None)).collect::<Vec<_>>(), "React Mode's identity keys every leaf by its instance id and carries no presentation title, so a locale switch never remounts it");''',
    )
]

PANEL_EDITS = [
    (
        '''record.key.as_str() == format!("command.category.{category}.form")).expect("form record");''',
        '''record.key.as_str() == format!("command.fixture/command.category.{category}.form")).expect("form record");''',
    )
]

ICON_RENDER_EDITS = [
    (
        '''    assert_eq!(environment["material"], fixture["worldEnvironment"]["material"]);''',
        '''    for (field, value) in fixture["worldEnvironment"]["material"].as_object().expect("the lit material") {
        assert_eq!(&environment["material"][field], value, "material.{field} carries the fixture's lighting value");
    }
    let outline: serde_json::Value = serde_json::from_str(include_str!("../../../🖼️IconRenderHost/🧫️fixtures/📤️svg-export/🔣️.json")).unwrap();
    assert_eq!(environment["material"]["stroke"], outline["material"]["stroke"], "an icon outline defaults to React's own stroke");''',
    )
]

WIRING_EDITS = [
    (
        '''fn install_fixture_text_editor_focus(window_id: &str, window_generation: u64, node: NodeId, host_id: &str) -> FocusedTextEditor {
    let focus =''',
        '''fn install_fixture_text_editor_focus(window_id: &str, window_generation: u64, node: NodeId, host_id: &str) -> FocusedTextEditor {
    if UI_ENGINE.with(|cell| cell.borrow().presented_document_id(window_id, node).is_none()) {
        present_seeded_scene_window(window_id);
    }
    let focus =''',
    ),
    (
        '''fn seed_scene_window(window_id: &str, surface_id: &str, kind: ui_wgpu::wgpu::SurfaceKind) -> NodeId {''',
        '''/// 🎞️ The presenter witness a seeded scene window's first presentation is accepted under.
const SEEDED_SCENE_PRESENTATION_WITNESS: u64 = 0x5EED_5CE7;

/// 🎞️ Presents a seeded window exactly as a frame presents it (seal + accepted pixels): since 09-27 every input, caret and
/// accessibility address resolves in the PRESENTED tree, so a law driving one needs the window presented.
fn present_seeded_scene_window(window_id: &str) {
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(window_id);
    assert!(seal_presented_input_candidate(SEEDED_SCENE_PRESENTATION_WITNESS), "the seeded scene window seals its presentation");
    assert!(acknowledge_presented_input(SEEDED_SCENE_PRESENTATION_WITNESS), "and its pixels are accepted");
}

/// 🪪️ The ComponentScene a seeded window mounted. Its host id is engine-minted (`reconcile::component_scene_host_id`, 09-21),
/// so every scene-scoped key, staged control and focus address reads it from here, never from the authored fixture scene.
fn retained_scene(window_id: &str, node: NodeId) -> UiComponentSceneNode {
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let retained = engine.tree(window_id).and_then(|tree| tree.node(node)).expect("the retained scene node remains mounted");
        let UiNode::ComponentScene(scene) = &retained.spec.0 else { panic!("the retained node is a ComponentScene") };
        scene.clone()
    })
}

/// 🔑️ A fixture key authored under the fixture's host id, rebased onto the engine-minted host id its retained scene carries.
fn rebased_scene_key(key: &str, authored_host: &str, retained_host: &str) -> String {
    format!("{retained_host}{}", key.strip_prefix(authored_host).expect("the fixture key is scoped by its authored host"))
}

/// ♿️ Stages, seals and accepts the editable-text cells a seeded Table window's paint publishes (09-27: only accepted cells
/// reach the accessibility tree).
fn accept_table_editable_text_cells(window_id: &str, node: NodeId, epoch: u64) {
    let retained = retained_scene(window_id, node);
    crate::scenes::remember_scene_theme(&Theme::default());
    let cells = crate::scenes::table_editable_text_accessibility_cells(&retained, Rect::new(0.0, 0.0, 200.0, 200.0), ui_wgpu::wgpu::UiDriverDrag::Handle);
    assert!(!cells.is_empty(), "the seeded Table paints its editable cells");
    crate::scenes::stage_table_editable_text_accessibility_cells(&retained.host_id, cells);
    crate::scenes::seal_table_editable_text_accessibility_candidates(epoch);
    crate::scenes::acknowledge_table_editable_text_accessibility_candidates(epoch);
}

fn seed_scene_window(window_id: &str, surface_id: &str, kind: ui_wgpu::wgpu::SurfaceKind) -> NodeId {''',
    ),
    (
        '''    let node = seed_scene_window_with(window, UiNode::ComponentScene(scene.clone()));
    let target = retained_scene_target(window, node).expect("accepted NodeGraph target");''',
        '''    let node = seed_scene_window_with(window, UiNode::ComponentScene(scene.clone()));
    present_seeded_scene_window(window);
    let target = retained_scene_target(window, node).expect("accepted NodeGraph target");''',
    ),
    (
        '''    sync_focused_table_editable_text_draft(&input);
    assert_eq!(focused_table_editable_text_draft(window_id, node, "table-editable-text-focus", "row-2", "value").as_deref(), Some("mehrzeilig\\nΩ🙂"));''',
        '''    sync_focused_table_editable_text_draft(&input);
    assert_eq!(focused_table_editable_text_draft(window_id, node, &retained_scene_host_id(window_id, node), "row-2", "value").as_deref(), Some("mehrzeilig\\nΩ🙂"));''',
    ),
    (
        '''    let pending_node = seed_scene_window_with(pending_window, table_editable_text_scene_node_with_value(base));
    let pending_generation''',
        '''    let pending_node = seed_scene_window_with(pending_window, table_editable_text_scene_node_with_value(base));
    accept_table_editable_text_cells(pending_window, pending_node, 931);
    let pending_host = retained_scene_host_id(pending_window, pending_node);
    let pending_generation''',
    ),
    (
        '''    assert_eq!(focused_table_editable_text_draft(pending_window, pending_node, "table-editable-text-focus", "row-2", "value").as_deref(), Some(draft));''',
        '''    assert_eq!(focused_table_editable_text_draft(pending_window, pending_node, &pending_host, "row-2", "value").as_deref(), Some(draft));''',
    ),
    (
        '''    seed_scene_window_with(pending_window, table_editable_text_scene_node_with_value(acceptance["acceptedPersisted"].as_str().unwrap()));
    let echoed_generation''',
        '''    let echoed_node = seed_scene_window_with(pending_window, table_editable_text_scene_node_with_value(acceptance["acceptedPersisted"].as_str().unwrap()));
    accept_table_editable_text_cells(pending_window, echoed_node, 932);
    let echoed_generation''',
    ),
    (
        '''    let refused_node = seed_scene_window_with(refused_window, table_editable_text_scene_node_with_value(acceptance["refusedPersisted"].as_str().unwrap()));
    let refused_generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(refused_window)).unwrap();
    UI_ENGINE.with(|cell| focus_table_editable_text(&cell.borrow(), refused_window, refused_generation, refused_node, "table-editable-text-focus", target(), &mut input)).expect("refused cell focus");
    set_focused_table_editable_text_draft(refused_window, refused_node, "table-editable-text-focus", "row-2", "value", draft);''',
        '''    let refused_node = seed_scene_window_with(refused_window, table_editable_text_scene_node_with_value(acceptance["refusedPersisted"].as_str().unwrap()));
    let refused_host = retained_scene_host_id(refused_window, refused_node);
    let refused_generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(refused_window)).unwrap();
    UI_ENGINE.with(|cell| focus_table_editable_text(&cell.borrow(), refused_window, refused_generation, refused_node, &refused_host, target(), &mut input)).expect("refused cell focus");
    set_focused_table_editable_text_draft(refused_window, refused_node, &refused_host, "row-2", "value", draft);''',
    ),
    (
        '''    assert_eq!(focused_table_editable_text_draft(refused_window, refused_node, "table-editable-text-focus", "row-2", "value").as_deref(), Some(draft));
    input.cancel_retained_action();
    assert_eq!(focused_table_editable_text_draft(refused_window, refused_node, "table-editable-text-focus", "row-2", "value").as_deref(), Some(draft), "cancellation preserves the refused draft");''',
        '''    assert_eq!(focused_table_editable_text_draft(refused_window, refused_node, &refused_host, "row-2", "value").as_deref(), Some(draft));
    input.cancel_retained_action();
    assert_eq!(focused_table_editable_text_draft(refused_window, refused_node, &refused_host, "row-2", "value").as_deref(), Some(draft), "cancellation preserves the refused draft");''',
    ),
    (
        '''    UI_ENGINE.with(|cell| focus_table_editable_text(&cell.borrow(), refused_window, refused_generation, refused_node, "table-editable-text-focus", other, &mut input)).expect("focus transfer preserves the unacknowledged edit");
    assert!(!input.retained_action_pending(), "focus transfer cannot duplicate a publication awaiting its scene echo");
    assert_eq!(focused_table_editable_text_draft(refused_window, refused_node, "table-editable-text-focus", "row-2", "value").as_deref(), Some(draft), "an unacknowledged edit keeps the original cell and draft focused");''',
        '''    UI_ENGINE.with(|cell| focus_table_editable_text(&cell.borrow(), refused_window, refused_generation, refused_node, &refused_host, other, &mut input)).expect("focus transfer preserves the unacknowledged edit");
    assert!(!input.retained_action_pending(), "focus transfer cannot duplicate a publication awaiting its scene echo");
    assert_eq!(focused_table_editable_text_draft(refused_window, refused_node, &refused_host, "row-2", "value").as_deref(), Some(draft), "an unacknowledged edit keeps the original cell and draft focused");''',
    ),
    (
        '''    let node = seed_scene_window_with(window_id, scene_node);
    assert!(crate::scenes::mount_scene_identity(&retained_scene_target(window_id, node).expect("retained EventFeed target")));
    let generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id)).unwrap();
    let expected_action = &actionable["expected"]["action"];
    let control = crate::scenes::EventFeedAccessibilityControl {
        key: actionable["expected"]["key"].as_str().unwrap().into(),''',
        '''    let node = seed_scene_window_with(window_id, scene_node);
    assert!(crate::scenes::mount_scene_identity(&retained_scene_target(window_id, node).expect("retained EventFeed target")));
    let retained_host = retained_scene_host_id(window_id, node);
    let generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id)).unwrap();
    let expected_action = &actionable["expected"]["action"];
    let control = crate::scenes::EventFeedAccessibilityControl {
        key: rebased_scene_key(actionable["expected"]["key"].as_str().unwrap(), host_id, &retained_host),''',
    ),
    (
        '''    crate::scenes::stage_event_feed_accessibility_controls(host_id, vec![control.clone()]);''',
        '''    crate::scenes::stage_event_feed_accessibility_controls(&retained_host, vec![control.clone()]);''',
    ),
    (
        '''    assert!(crate::scenes::mount_scene_identity(&retained_scene_target(passive_window_id, passive_scene_node).expect("retained passive EventFeed target")));
    let passive_key = format!("{passive_host_id}.feed.{}", passive["entry"]["id"].as_str().unwrap());
    crate::scenes::stage_event_feed_accessibility_controls(
        passive_host_id,''',
        '''    assert!(crate::scenes::mount_scene_identity(&retained_scene_target(passive_window_id, passive_scene_node).expect("retained passive EventFeed target")));
    let passive_retained_host = retained_scene_host_id(passive_window_id, passive_scene_node);
    let passive_key = format!("{passive_retained_host}.feed.{}", passive["entry"]["id"].as_str().unwrap());
    crate::scenes::stage_event_feed_accessibility_controls(
        &passive_retained_host,''',
    ),
    (
        '''    crate::scenes::stage_event_feed_accessibility_controls(host_id, Vec::new());
    crate::scenes::seal_event_feed_accessibility_candidates(806);''',
        '''    crate::scenes::stage_event_feed_accessibility_controls(&retained_host, Vec::new());
    crate::scenes::seal_event_feed_accessibility_candidates(806);''',
    ),
    (
        '''    crate::scenes::remember_scene_theme(&Theme::default());
    let controls = crate::scenes::table_button_accessibility_cells(scene, Rect::new(0.0, 0.0, 400.0, 300.0), ui_wgpu::wgpu::UiDriverDrag::Handle);
    assert_eq!(controls.len(), 1);
    let control = controls[0].clone();
    let node = seed_scene_window_with(window_id, scene_node);
    assert!(crate::scenes::mount_scene_identity(&retained_scene_target(window_id, node).expect("retained Table target")));
    let generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id)).unwrap();
    crate::scenes::stage_table_button_accessibility_cells(host_id, controls);''',
        '''    crate::scenes::remember_scene_theme(&Theme::default());
    let node = seed_scene_window_with(window_id, scene_node);
    let retained = retained_scene(window_id, node);
    let controls = crate::scenes::table_button_accessibility_cells(&retained, Rect::new(0.0, 0.0, 400.0, 300.0), ui_wgpu::wgpu::UiDriverDrag::Handle);
    assert_eq!(controls.len(), 1);
    let control = controls[0].clone();
    assert_eq!(control.key, rebased_scene_key(fixture["expected"]["key"].as_str().unwrap(), host_id, &retained.host_id), "the button key is the fixture's, scoped by the engine-minted host");
    assert!(crate::scenes::mount_scene_identity(&retained_scene_target(window_id, node).expect("retained Table target")));
    let generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id)).unwrap();
    crate::scenes::stage_table_button_accessibility_cells(&retained.host_id, controls);''',
    ),
    (
        '''    assert!(dispatch_accessibility_event(window_id, generation, button.node_id, "table-ax-host.row.stale.actions.0", ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_none(), "a stale key is inert");''',
        '''    assert!(
        dispatch_accessibility_event(window_id, generation, button.node_id, &rebased_scene_key("table-ax-host.row.stale.actions.0", host_id, &retained.host_id), ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_none(),
        "a stale key is inert"
    );''',
    ),
    (
        '''    crate::scenes::stage_table_button_accessibility_cells(host_id, Vec::new());
    crate::scenes::seal_table_button_accessibility_candidates(903);''',
        '''    crate::scenes::stage_table_button_accessibility_cells(&retained.host_id, Vec::new());
    crate::scenes::seal_table_button_accessibility_candidates(903);''',
    ),
    (
        '''    assert!(UI_ENGINE.with(|cell| cell.borrow().window_next_clock_deadline(window)).is_some(), "accepted note editing owns one cadence deadline");''',
        '''    assert!(UI_ENGINE.with(|cell| cell.borrow().window_next_clock_deadline(window)).and_then(|(_, deadline)| deadline).is_some(), "accepted note editing owns one cadence deadline");''',
    ),
    (
        '''    assert_eq!(UI_ENGINE.with(|cell| cell.borrow().window_next_clock_deadline(window)), None, "committing the note cancels the cadence wake");''',
        '''    assert_eq!(UI_ENGINE.with(|cell| cell.borrow().window_next_clock_deadline(window)).and_then(|(_, deadline)| deadline), None, "committing the note cancels the cadence wake");''',
    ),
    (
        '''    focus(&mut input);
    assert!(FOCUSED_TABLE_EDITABLE_TEXT.with(|cell| cell.borrow().as_ref().is_some_and(|focus| focus.window_id == window_id && focus.row_id == "row-2" && focus.column_id == "value")));
    let control_id = input.focused_id.clone().expect("native cell input focus");''',
        '''    focus(&mut input);
    assert!(FOCUSED_TABLE_EDITABLE_TEXT.with(|cell| cell.borrow().as_ref().is_some_and(|focus| focus.window_id == window_id && focus.row_id == "row-2" && focus.column_id == "value")));
    while input.drive_text_step().expect("the focused cell's own value projects before the edit") {}
    let control_id = input.focused_id.clone().expect("native cell input focus");''',
    ),
    (
        '''    focus(&mut input);
    let control_id = input.focused_id.clone().expect("refocused cell input");''',
        '''    focus(&mut input);
    while input.drive_text_step().expect("the refocused cell's own value projects before the edit") {}
    let control_id = input.focused_id.clone().expect("refocused cell input");''',
    ),
    (
        '''    let actions = crate::collect_fixture_actions(&mut input);
    let publication_count_before_echo''',
        '''    let actions = collect_published_actions(&mut input);
    let publication_count_before_echo''',
    ),
    (
        '''    let blur_actions = crate::collect_fixture_actions(&mut input);''',
        '''    let blur_actions = collect_published_actions(&mut input);''',
    ),
    (
        '''fn table_editable_text_scene_node() -> UiNode {''',
        '''/// 📤️ Every action an input published: the inline queue AND the retained (paged) publication an editable value takes.
fn collect_published_actions(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Vec<ActionDescriptor> {
    let mut actions = crate::collect_fixture_actions(input);
    for _ in 0..4096 {
        if !input.retained_action_pending() {
            break;
        }
        if let Some(action) = input.drive_retained_action_step().expect("retained publication page") {
            actions.push(action.descriptor);
        }
    }
    actions
}

fn table_editable_text_scene_node() -> UiNode {''',
    ),
    (
        '''    let retained_host = retained_scene_host_id(window_id, node);
    let generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id)).unwrap();
    let expected_action = &actionable["expected"]["action"];''',
        '''    let retained_host = retained_scene_host_id(window_id, node);
    let retained_surface = retained_scene(window_id, node).surface_id;
    let generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id)).unwrap();
    let expected_action = &actionable["expected"]["action"];''',
    ),
    (
        '''                "surfaceId": expected_action["args"]["surfaceId"].as_str().unwrap(),''',
        '''                "surfaceId": retained_surface.as_str(),''',
    ),
    (
        '''    assert_eq!(actions[0].args.as_ref().and_then(|args| args.get("surfaceId")).and_then(semio_framework::DslValue::as_str), expected_action["args"]["surfaceId"].as_str());''',
        '''    assert_eq!(actions[0].args.as_ref().and_then(|args| args.get("surfaceId")).and_then(semio_framework::DslValue::as_str), Some(retained_surface.as_str()), "the feed's action names the surface its document mounted in");''',
    ),
]

STANDALONE_EDITS = [
    (
        '''        assert!(ENGINE_CANVAS_STANDALONE.contains(&format!("#[cfg(test)]\\npub fn {function}")), "{function} must not remain production-capable");''',
        '''        assert!(!ENGINE_CANVAS_SOURCE.contains(&format!("pub fn {function}(")) && !ENGINE_CANVAS_STANDALONE.contains(&format!("pub fn {function}(")), "{function} must not remain production-capable");''',
    )
]

RECEIPT_EDITS = [
    (
        '''        for value in &typed[1..] {
            assert!(text_editor_apply_key_into(&scene, &KeyAction::Char(value.as_str().unwrap().into()), &PointerModifiers::default(), &mut input).unwrap());
        }
        let first_receipt = first_edit.receipt.unwrap();''',
        '''        for value in &typed[1..] {
            assert!(text_editor_apply_key_into(&scene, &KeyAction::Char(value.as_str().unwrap().into()), &PointerModifiers::default(), &mut input).unwrap());
        }
        let after_new_owner = law["typedAfterNewOwner"].as_array().cloned().unwrap_or_default();
        if !after_new_owner.is_empty() {
            assert!(sync_engine_scene(&text_editor_scene(id, law["initial"].as_str().unwrap()), "editor-delivery-law", Rect::new(0.0, 0.0, 480.0, 320.0), &Theme::default()), "{id}: the same host re-renders mid-flight with its guest's stale buffer");
            for value in &after_new_owner {
                assert!(text_editor_apply_key_into(&scene, &KeyAction::Char(value.as_str().unwrap().into()), &PointerModifiers::default(), &mut input).unwrap());
            }
        }
        let first_receipt = first_edit.receipt.unwrap();''',
    ),
    (
        '''        let expected_text = if accepted { format!("{}{}", law["initial"].as_str().unwrap(), typed.iter().map(|row| row.as_str().unwrap()).collect::<String>()) } else { law["initial"].as_str().unwrap().to_owned() };''',
        '''        let expected_text = if accepted { format!("{}{}", law["initial"].as_str().unwrap(), typed.iter().chain(after_new_owner.iter()).map(|row| row.as_str().unwrap()).collect::<String>()) } else { law["initial"].as_str().unwrap().to_owned() };''',
    ),
]

NATIVE_AX_FIXTURE_EDITS = [
    (
        '''"authorId": "note-composite/outline", "role": "Tree", "name": "Outline", "actions": [], "children": ["note-composite/outline.a"] },''',
        '''"authorId": "note-composite/outline", "role": "Tree", "name": "Outline", "actions": [], "children": ["note-composite/outline.a", "note-composite/outline.b"] },''',
    )
]

LIFECYCLE_EDITS = [
    (
        '''    shell.dock_window_plan = vec![("world".into(), Rect::new(0.0, 0.0, 100.0, 100.0)), ("world-2".into(), Rect::new(100.0, 0.0, 100.0, 100.0))];''',
        '''    for id in ["world", "world-2"] {
        assert!(shell.dock.split_root_with_window(id, crate::dock::DockSide::Right), "the committed dock holds {id}");
    }
    shell.dock_window_plan = vec![("world".into(), Rect::new(0.0, 0.0, 100.0, 100.0)), ("world-2".into(), Rect::new(100.0, 0.0, 100.0, 100.0))];''',
    ),
    (
        '''    shell.dock_window_plan.remove(0);
    shell.sync_engine_surface_states();
    assert!(!shell.world3d_states.contains_key("world"));''',
        '''    assert!(shell.dock.close_window("world"), "the dock — the retention authority — closes the window");
    shell.dock_window_plan.remove(0);
    shell.sync_engine_surface_states();
    assert!(!shell.world3d_states.contains_key("world"));''',
    ),
]

NAVBAR_EDITS = [
    (
        '''    assert!(!shell.window_icon_overrides.contains_key("pane-perspective"));
}''',
        '''    assert!(!shell.window_icon_overrides.contains_key("pane-perspective"));
    for _ in 0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20) {
        if shell.retired_world3d_states.is_empty() {
            break;
        }
        shell.advance_world3d_retirement_step();
    }
    assert!(shell.retired_world3d_states.is_empty(), "🪟️ the retired scene owner reaches terminal empty under maintenance before the shell drops");
}''',
    ),
    (
        '''    assert!(shell.apply_window_icon_host_command("pane-perspective", "projection-three-point"));
    shell.dock_window_plan.retain(|(window_id, _)| window_id == "pane-top");''',
        '''    assert!(shell.apply_window_icon_host_command("pane-perspective", "projection-three-point"));
    shell.plan_dock_windows(Rect::new(0.0, 0.0, 1280.0, 800.0), &Theme::light(), &mut FontAtlas::builtin());
    assert!(shell.dock_window_plan.iter().any(|(window_id, _)| window_id == "pane-perspective"), "🪟️ the unfocused plan paints both panes");
    shell.dock_window_plan.retain(|(window_id, _)| window_id == "pane-top");''',
    ),
    (
        '''            .filter(|id| id.contains(WORLD_PROJECTION_PANE_PARENT) && id != &window_projection_surface_id("pane-top"))''',
        '''            .filter(|id| id.starts_with("tree.label.") && id.contains(WORLD_PROJECTION_PANE_PARENT))''',
    ),
    (
        '''        let surface = window_projection_surface_id("pane-top");
        let rows = input.staged_hits().iter().filter(|hit| hit.control_id.as_deref().is_some_and(|id| id.contains(WORLD_PROJECTION_PANE_PARENT) && id != surface.as_str())).cloned().collect::<Vec<_>>();''',
        '''        let rows = input.staged_hits().iter().filter(|hit| hit.control_id.as_deref().is_some_and(|id| id.starts_with("tree.label.") && id.contains(WORLD_PROJECTION_PANE_PARENT))).cloned().collect::<Vec<_>>();''',
    ),
]

ENGINE_SURFACE_EDITS = [
    (
        '''    assert!(tiled_map_set_camera_silent(&map_id, [0.0, 0.0, 10.0]));
    assert!(!crate::scenes::tiled_map_touch_pointer_down(&map_id, bounds, down[0].0, down[0].1, down[0].2));''',
        '''    assert!(tiled_map_set_camera_silent(&map_id, [0.0, 0.0, 10.0]));
    let seeded = tiled_map_camera(&map_id).expect("the seeded map camera, clamped to cover its viewport");
    let (initial, expected) = (&spread["initialCamera"], &spread["expectedCamera"]);
    assert_eq!((initial["x"].as_f64(), initial["y"].as_f64()), (expected["x"].as_f64(), expected["y"].as_f64()), "the spread gesture is centred: it scales about the viewport centre and pans nothing");
    let zoom_ratio = expected["zoom"].as_f64().expect("expected zoom") / initial["zoom"].as_f64().expect("initial zoom");
    assert!(!crate::scenes::tiled_map_touch_pointer_down(&map_id, bounds, down[0].0, down[0].1, down[0].2));''',
    ),
    (
        '''    close_camera(map_camera, [0.0, 0.0, 20.0]);''',
        '''    close_camera(map_camera, [seeded[0], seeded[1], seeded[2] * zoom_ratio]);''',
    ),
]

EXPORT_BATCH_EDITS = [
    (
        '''        if shell.render_icon_export_step(&mut cursor, &mut draw, &mut atlas, &mut input, &ui_wgpu::wgpu::Theme::default(), 1280.0) { break; }
    }
    let nodes = shell.chrome_accessibility_nodes(input.hits());''',
        '''        if shell.render_icon_export_step(&mut cursor, &mut draw, &mut atlas, &mut input, &ui_wgpu::wgpu::Theme::default(), 1280.0) { break; }
    }
    input.publish_hits();
    let nodes = shell.chrome_accessibility_nodes(input.hits());''',
    )
]

SETTINGS_EDITS = [
    (
        '''    assert_eq!(shell.chrome_accessibility_selected(FRAMEWORK_SETTINGS_GENERAL_TAB_ID, &HitKind::PanelTab), Some(false), "closing the root leaves no selected panel tab");''',
        '''    assert_eq!(
        shell.chrome_accessibility_selected(FRAMEWORK_SETTINGS_GENERAL_TAB_ID, &HitKind::PanelTab),
        None,
        "a panel tab is a toggle — it announces `aria-pressed` (React `PanelTabButton`), never a selection, so closing the root selects nothing"
    );''',
    )
]

SHELL_INPUT_EDITS = [
    (
        '''    shell.retire_documents_outside(&[], true).expect("the closed retained body enters document retirement");
    assert!(!shell.window_ui.contains_key("main-2"));
    shell.deferred_actions.clear();''',
        '''    shell.retire_documents_outside(&[], true).expect("the closed retained body enters document retirement");
    assert!(!shell.window_ui.contains_key("main-2"));
    shell.deferred_actions.clear();
    shell.window_topology_journal_dispatch_owed = false;''',
    ),
    (
        '''    assert_eq!(semio_framework_async::block_on(fixture.shell.settle_pump_step_inner()), ShellSettleStep::Drained);
    let events = WINDOW_PUBLICATION_FIXTURE_EVENTS.with(|events| events.borrow().clone());
    let ready_dispatch = events.iter().position(|event| event == "journal:main-3").expect("the ready peer dispatches");
    let refused_retry = events.iter().enumerate().filter(|(_, event)| event.as_str() == "render:main-2").nth(1).map(|(index, _)| index).expect("the refused peer retries");
    assert!(ready_dispatch < refused_retry, "the ready journal dispatches before the unrelated retry");
    assert_eq!(semio_framework_async::block_on(fixture.shell.settle_pump_step_inner()), ShellSettleStep::Drained);
    let events = WINDOW_PUBLICATION_FIXTURE_EVENTS.with(|events| events.borrow().clone());''',
        '''    let dispatched = |instance: &str| WINDOW_PUBLICATION_FIXTURE_EVENTS.with(|events| events.borrow().iter().any(|event| event.as_str() == format!("journal:{instance}")));
    for _ in 0..WINDOW_TOPOLOGY_PUBLICATION_ATTEMPTS * 4 {
        if dispatched("main-2") && dispatched("main-3") {
            break;
        }
        assert_eq!(semio_framework_async::block_on(fixture.shell.settle_pump_step_inner()), ShellSettleStep::Drained, "the settle lane owns the retry and every journal");
    }
    let events = WINDOW_PUBLICATION_FIXTURE_EVENTS.with(|events| events.borrow().clone());
    let ready_dispatch = events.iter().position(|event| event == "journal:main-3").expect("the ready peer dispatches");
    let refused_retry = events.iter().enumerate().filter(|(_, event)| event.as_str() == "render:main-2").nth(1).map(|(index, _)| index).expect("the refused peer retries");
    assert!(ready_dispatch < refused_retry, "the ready journal dispatches before the unrelated retry");''',
    ),
]

UI_DRAW_EDITS = [
    (
        '''fn world_mesh_instance_packs_policy_and_standard_material_without_stride_growth() {
    let gpu = super::World3dGpuInstance::from_instance([0.0; 16], [1.0; 4], true, 0.2, 0.63, 0.27, true);
    assert_eq!(std::mem::size_of::<super::World3dGpuInstance>(), 96);
    assert_eq!(gpu.flags, [3.0, 0.2, 0.63, 0.27]);
}''',
        '''/// 🧱️ LAW: the vertex-colour/shadow policy and the standard material pack into ONE `flags` lane; the stride grew exactly once, for
/// the authored (GLB) material's emissive colour + alpha cutoff lane (`emissive_cutoff`, 09-28), which a standard instance leaves
/// emissive-free with its cutoff disabled.
fn world_mesh_instance_packs_policy_and_standard_material_into_one_fixed_stride() {
    let gpu = super::World3dGpuInstance::from_instance([0.0; 16], [1.0; 4], true, 0.2, 0.63, 0.27, true);
    assert_eq!(std::mem::size_of::<super::World3dGpuInstance>(), 112);
    assert_eq!(gpu.flags, [3.0, 0.2, 0.63, 0.27]);
    assert_eq!(gpu.emissive_cutoff, [0.0, 0.0, 0.0, -1.0]);
}''',
    )
]

UI_PRESENT_EDITS = [
    (
        '''        "world3d_painted_pipeline",
        "world3d_painted_pipeline_translucent",
        "world3d_celebration_pipeline",''',
        '''        "world3d_painted_pipeline",
        "world3d_painted_pipeline_translucent",
        "world3d_authored_front_pipeline",
        "world3d_authored_double_translucent_pipeline",
        "world3d_authored_painted_front_pipeline",
        "world3d_authored_painted_front_translucent_pipeline",
        "world3d_celebration_pipeline",''',
    ),
    (
        '''        "standard, translucent, painted, celebration, line, textured, procedural-grid and curvilinear pipelines all target the encoded UNORM view exactly once"''',
        '''        "standard, translucent, painted, authored (GLB), celebration, line, textured, procedural-grid and post-process pipelines all target the encoded UNORM view exactly once"''',
    ),
]

UI_PREPARED_EDITS = [
    (
        '''    assert_eq!(order, vec!["shadow", "textured", "material-opaque", "opaque", "lines", "translucent", "material-translucent", "curvilinear"], "the image-space remap runs after every world color receiver");''',
        '''    assert_eq!(order, vec!["shadow", "textured", "material-opaque", "opaque", "lines", "translucent", "material-translucent", "world-postprocess"], "the image-space remap runs after every world color receiver");''',
    )
]

SLOT_BUDGET_EDITS = [
    (
        '''      "owner": "wgpu::engine::UiSurfaceRegistry",
      "capacityConstant": "UI_LAYOUT_SURFACE_SLOTS",
      "capacity": 64,
      "elementType": "Option<UiSurfaceSlot>",
      "elementSizeBytes": 164672,''',
        '''      "owner": "wgpu::engine::UiSurfaceRegistry",
      "capacityConstant": "UI_LAYOUT_SURFACE_SLOTS",
      "capacity": 64,
      "elementType": "Option<UiSurfaceSlot>",
      "elementSizeBytes": 164712,''',
    )
]

RESERVED_EDITS = [('''"setActiveUtility", "setActiveTool"]''', '''"setActiveUtility", "setActiveTool", "exportArtifactDocument", "importArtifactDocument"]''')]


def replaced(path: Path, source: str, edits) -> str:
    for old, new in edits:
        count = source.count(old)
        if count != 1:
            sys.exit(f"anchor occurs {count}x in {path.parent.name}/{path.name}: {old[:100]!r}")
        source = source.replace(old, new)
    return source


def png_chunks(png: bytes):
    at = 8
    while at < len(png):
        length = struct.unpack(">I", png[at : at + 4])[0]
        yield at, png[at + 4 : at + 8], png[at + 8 : at + 8 + length]
        at += 12 + length


def glb_after(glb: bytes) -> bytes:
    json_length = struct.unpack("<I", glb[12:16])[0]
    import json

    document = json.loads(glb[20 : 20 + json_length])
    image = document["images"][0]
    view = document["bufferViews"][image["bufferView"]]
    start = 20 + json_length + 8 + view.get("byteOffset", 0)
    png = bytearray(glb[start : start + view["byteLength"]])
    for at, kind, body in list(png_chunks(bytes(png))):
        if kind != b"IDAT":
            continue
        try:
            zlib.decompress(body)
            sys.exit("the embedded PNG already inflates — landed already")
        except zlib.error:
            pass
        raw = zlib.decompressobj(-15).decompress(body[2:])
        fixed = body[:-4] + struct.pack(">I", zlib.adler32(raw))
        png[at + 8 : at + 8 + len(body)] = fixed
        png[at + 8 + len(body) : at + 12 + len(body)] = struct.pack(">I", zlib.crc32(kind + fixed) & 0xFFFFFFFF)
    for _, kind, body in png_chunks(bytes(png)):
        if kind == b"IDAT" and zlib.decompress(body) != b"\x01\x00\xff\x00\xff":
            sys.exit("the repaired IDAT no longer carries the authored scanline")
    for at, kind, body in png_chunks(bytes(png)):
        crc = struct.unpack(">I", png[at + 8 + len(body) : at + 12 + len(body)])[0]
        if crc != zlib.crc32(kind + body) & 0xFFFFFFFF:
            sys.exit(f"{kind!r} CRC still wrong")
    return glb[:start] + bytes(png) + glb[start + len(png) :]


FILES = [GLB, ASYNC_LAWS, HUB_LAWS, WINIT_LAWS, RESERVED_FIXTURE, LOCALE_LAWS, PANEL_LAWS, ICON_RENDER_LAWS, WIRING_LAWS, STANDALONE_LAWS, RECEIPT_LAWS, NATIVE_AX_FIXTURE, LIFECYCLE_LAWS, NAVBAR_LAWS, ENGINE_SURFACE_LAWS, EXPORT_BATCH_LAWS, SETTINGS_LAWS, SHELL_INPUT_LAWS, UI_DRAW_LAWS, UI_PRESENT_LAWS, UI_PREPARED_LAWS, SLOT_BUDGET_FIXTURE]


def plans():
    text = {path: path.read_text(encoding="utf-8") for path in FILES if path != GLB}
    return [
        (ASYNC_LAWS, text[ASYNC_LAWS], replaced(ASYNC_LAWS, text[ASYNC_LAWS], ASYNC_EDITS)),
        (HUB_LAWS, text[HUB_LAWS], replaced(HUB_LAWS, text[HUB_LAWS], HUB_EDITS)),
        (WINIT_LAWS, text[WINIT_LAWS], replaced(WINIT_LAWS, text[WINIT_LAWS], WINIT_EDITS)),
        (RESERVED_FIXTURE, text[RESERVED_FIXTURE], replaced(RESERVED_FIXTURE, text[RESERVED_FIXTURE], RESERVED_EDITS)),
        (LOCALE_LAWS, text[LOCALE_LAWS], replaced(LOCALE_LAWS, text[LOCALE_LAWS], LOCALE_EDITS)),
        (PANEL_LAWS, text[PANEL_LAWS], replaced(PANEL_LAWS, text[PANEL_LAWS], PANEL_EDITS)),
        (ICON_RENDER_LAWS, text[ICON_RENDER_LAWS], replaced(ICON_RENDER_LAWS, text[ICON_RENDER_LAWS], ICON_RENDER_EDITS)),
        (WIRING_LAWS, text[WIRING_LAWS], replaced(WIRING_LAWS, text[WIRING_LAWS], WIRING_EDITS)),
        (STANDALONE_LAWS, text[STANDALONE_LAWS], replaced(STANDALONE_LAWS, text[STANDALONE_LAWS], STANDALONE_EDITS)),
        (RECEIPT_LAWS, text[RECEIPT_LAWS], replaced(RECEIPT_LAWS, text[RECEIPT_LAWS], RECEIPT_EDITS)),
        (NATIVE_AX_FIXTURE, text[NATIVE_AX_FIXTURE], replaced(NATIVE_AX_FIXTURE, text[NATIVE_AX_FIXTURE], NATIVE_AX_FIXTURE_EDITS)),
        (LIFECYCLE_LAWS, text[LIFECYCLE_LAWS], replaced(LIFECYCLE_LAWS, text[LIFECYCLE_LAWS], LIFECYCLE_EDITS)),
        (NAVBAR_LAWS, text[NAVBAR_LAWS], replaced(NAVBAR_LAWS, text[NAVBAR_LAWS], NAVBAR_EDITS)),
        (ENGINE_SURFACE_LAWS, text[ENGINE_SURFACE_LAWS], replaced(ENGINE_SURFACE_LAWS, text[ENGINE_SURFACE_LAWS], ENGINE_SURFACE_EDITS)),
        (EXPORT_BATCH_LAWS, text[EXPORT_BATCH_LAWS], replaced(EXPORT_BATCH_LAWS, text[EXPORT_BATCH_LAWS], EXPORT_BATCH_EDITS)),
        (SETTINGS_LAWS, text[SETTINGS_LAWS], replaced(SETTINGS_LAWS, text[SETTINGS_LAWS], SETTINGS_EDITS)),
        (SHELL_INPUT_LAWS, text[SHELL_INPUT_LAWS], replaced(SHELL_INPUT_LAWS, text[SHELL_INPUT_LAWS], SHELL_INPUT_EDITS)),
        (UI_DRAW_LAWS, text[UI_DRAW_LAWS], replaced(UI_DRAW_LAWS, text[UI_DRAW_LAWS], UI_DRAW_EDITS)),
        (UI_PRESENT_LAWS, text[UI_PRESENT_LAWS], replaced(UI_PRESENT_LAWS, text[UI_PRESENT_LAWS], UI_PRESENT_EDITS)),
        (UI_PREPARED_LAWS, text[UI_PREPARED_LAWS], replaced(UI_PREPARED_LAWS, text[UI_PREPARED_LAWS], UI_PREPARED_EDITS)),
        (SLOT_BUDGET_FIXTURE, text[SLOT_BUDGET_FIXTURE], replaced(SLOT_BUDGET_FIXTURE, text[SLOT_BUDGET_FIXTURE], SLOT_BUDGET_EDITS)),
    ]


def main():
    if "--revert" in sys.argv:
        for path in FILES:
            backup = BACKUP / path.relative_to(ROOT)
            if not backup.exists():
                sys.exit(f"no backup for {path.relative_to(ROOT)}")
            shutil.copyfile(backup, path)
        print(f"REVERTED: {len(FILES)} files restored from backups")
        return
    write = "--write" in sys.argv
    glb_before = GLB.read_bytes()
    glb_patched = glb_after(glb_before)
    changed = sum(1 for left, right in zip(glb_before, glb_patched) if left != right)
    print(f"{GLB.name}: {len(glb_before)} bytes, {changed} bytes change (Adler-32 + IDAT CRC)")
    planned = plans()
    for path, before, after in planned:
        sys.stdout.writelines(difflib.unified_diff(before.splitlines(True), after.splitlines(True), f"{path.parent.name}/{path.name}", f"{path.parent.name}/{path.name} (patched)", n=1))
    if write:
        for path in FILES:
            backup = BACKUP / path.relative_to(ROOT)
            backup.parent.mkdir(parents=True, exist_ok=True)
            backup.write_bytes(path.read_bytes())
        GLB.write_bytes(glb_patched)
        for path, _, after in planned:
            path.write_text(after, encoding="utf-8")
    print(f"\n{'WRITTEN' if write else 'DRY RUN'}: {len(planned) + 1} files (crate: semio-framework-os-renderer-wgpu tests/fixtures; ui GLB fixture)")


if __name__ == "__main__":
    main()
