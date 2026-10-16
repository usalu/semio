"""🖥️ Slice D follow-up, hosts and producers: the wgpu and React board hosts install the scene's placement naming, the
puzzle plugins publish theirs, and the os renderer drops its puzzle 2d names (`board2d_*` surface seams, `BOARD_*` event sets)."""
import re
from importlib import util

spec = util.spec_from_file_location("apply_board", __file__.rsplit("/", 1)[0] + "/apply-board.py")
board = util.module_from_spec(spec)
spec.loader.exec_module(board)
R = board.R
E = R + "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/"
W = E + "🧱️elements/⚙️EngineCanvas/"
H = E + "🧱️elements/🖥️Board2dHost/"
P = R + "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/"
P5 = R + "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/"

FAMILY = re.compile(r"\bpuzzle_board_(pointer_up_into|pointer_down_into|pointer_move_into|wheel_into|wheel|touch_pointer_up_into|touch_pointer_down_into|pointer_up|pointer_move|pointer_leave_into|pointer_leave|camera|touch_pointer_move|set_camera_silent|pointer_cancel_into|hover_into|yield_to_pinch_into|publish_camera_into|key_into|direct_pointer_into|flush_events_into)\b")
FAMILY_FILES = [
    E + "🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🖱️pointer-input/🦀️.rs",
    E + "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🧩️board2d-and-vfs/🦀️.rs",
    E + "🧱️elements/🎞️Scenes/🧪️tests/🧊️wgpu-standalone/🦀️.rs",
    W + "🎯️targets/🧊️wgpu/🎲️board2d/🦀️.rs",
    W + "🎯️targets/🧊️wgpu/🦀️.rs",
    W + "🧪️tests/🧊️wgpu-standalone/🦀️.rs",
    W + "🧪️tests/🔬️wgpu-board2d-engine/🦀️.rs",
    W + "🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs",
    E + "🎯️targets/🧊️wgpu/🧊️renderer/🧠️app-runtime/🦀️.rs",
    E + "🧪️tests/🛑️scene-pointer-cancellation/🟦️.ts",
]
EVENT_SET_FILES = [H + "🟦️.tsx", W + "🎯️targets/🧊️wgpu/🎲️board2d/🦀️.rs", W + "🧪️tests/🔬️wgpu-board2d-engine/🦀️.rs", P + "🧵️retained/🦀️.rs"]

WGPU_NAMING_TEST = '''
/// 🏷️ Law: the owning domain's placement naming reaches the wgpu board engine through the scene, so a brush placement on
/// the wgpu lane mints the domain's ids exactly like the domain's own boards; an unchanged naming re-applies nothing and an
/// absent one restores the engine's installed naming.
#[test]
fn the_scene_placement_naming_reaches_the_wgpu_board_engine() {
    let naming = r#"{"brushNodePrefix":"domain.brush.","brushEdgePrefix":"domain.brush.edge."}"#;
    let mut board = ui_wgpu::wgpu::Board2dScene::base(json!({ "schema": "board.ports.directed.v1", "nodes": [], "edges": [] }).to_string(), json!({ "x": 0.0, "y": 0.0, "zoom": 1.0 }).to_string(), true);
    board.placement_naming = Some(naming.into());
    let (mut host, mut cache) = (infinite_canvas::BoardHost::default(), BoardSyncCache::default());
    sync_board_engine(&mut host, &mut cache, &board, 800, 600);
    assert_eq!(host.brush_placement_naming().to_json(), naming, "the scene's naming is the one the wgpu host mints with");
    assert!(!sync_board_engine(&mut host, &mut cache, &board, 800, 600), "an unchanged scene re-applies nothing");
    board.placement_naming = None;
    sync_board_engine(&mut host, &mut cache, &board, 800, 600);
    assert_eq!(host.brush_placement_naming(), infinite_canvas::NEUTRAL_BOARD_NAMING.placement_naming(), "an absent naming restores the installed one");
}
'''

PLUGIN_LABEL_ASSERT = '''    assert_eq!(PUZZLE_2D_BOARD_NAMING.placement_naming().to_json(), r#"{"brushNodePrefix":"puzzle2d.brush.","brushEdgePrefix":"puzzle2d.brush.edge."}"#, "the scene wire form other hosts mint with");
'''

ENGINE_DOC_OLD = '''//! 🌉️ Externally reachable at `puzzle::editor::puzzle2d::engine::*` (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET moved this module here from the retired
//! `puzzle::apps::puzzle2d::engine::*`; the framework OS renderer's own call sites were the one real
//! cross-crate compile dependency on the old path and were repointed at this module in the same
//! ticket's W2-FIX lane) — the framework OS renderer
//! (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`)
//! holds a `BoardHost` and calls `board_host::puzzle_board_host()` directly, so this module and its
//! `board_host` child must both stay `pub`.
'''
ENGINE_DOC_NEW = '''//! 🌉️ Externally reachable at `puzzle::editor::puzzle2d::engine::*`. The framework OS renderer never links
//! this module: its wgpu `Board2d` surface holds a bare `BoardHost` and receives this plugin's brush naming as
//! scene data (`Board2dScene::placement_naming`, published from `board_host::PUZZLE_2D_BOARD_NAMING`). The
//! module and its `board_host` child stay `pub` for the plugin's own wasm session and the compositions that
//! re-export it.
'''

EDITS = {
    W + "🎯️targets/🧊️wgpu/🗂️engine-surface-registry/🦀️.rs": [
        ("    pub(super) glyph_catalogs_json: Option<String>,\n    pub(super) placement_compatibility_json: Option<String>,\n",
         "    pub(super) glyph_catalogs_json: Option<String>,\n"
         "    /// 🏷️ `Board2dScene.placement_naming` — the owning domain's brush id prefixes, so a placement on this lane mints the domain's ids.\n"
         "    pub(super) placement_naming: Option<String>,\n"
         "    pub(super) placement_compatibility_json: Option<String>,\n"),
    ],
    W + "🎯️targets/🧊️wgpu/🧩️engine-surface-attach/🦀️.rs": [
        ("        cache.glyph_catalogs_json = Some(board.glyph_catalogs_json.clone());\n        changed = true;\n    }\n",
         "        cache.glyph_catalogs_json = Some(board.glyph_catalogs_json.clone());\n        changed = true;\n    }\n"
         "    if cache.placement_naming != board.placement_naming {\n"
         "        let _ = host.set_placement_naming_json(board.placement_naming.as_deref());\n"
         "        cache.placement_naming = board.placement_naming.clone();\n    }\n"),
    ],
    W + "🧪️tests/🔬️wgpu-board2d-engine/🦀️.rs": [
        ('"/../../../../🧱️elements/🖥️Board2dHost/🧫️fixtures/🧫️board-event-coalescing/🔣️.json"', '"/../../../../🧱️elements/🖥️Board2dHost/🧪️tests/🧫️fixtures/🧫️board-event-coalescing/🔣️.json"'),
        ('"/../../../../🧱️elements/🖥️Board2dHost/🧫️fixtures/🧫️float32-decimal/🔣️.json"', '"/../../../../🧱️elements/🖥️Board2dHost/🧪️tests/🧫️fixtures/🧫️float32-decimal/🔣️.json"'),
    ],
    W + "🎯️targets/🧊️wgpu/🎲️board2d/🦀️.rs": [
        ("/// `🖥️Board2dHost/🧫️fixtures/🧫️board-event-coalescing/🔣️.json` pins both.", "/// `🖥️Board2dHost/🧪️tests/🧫️fixtures/🧫️board-event-coalescing/🔣️.json` pins both."),
        ("/// `🖥️Board2dHost/🧫️fixtures/🧫️board-event-coalescing/🔣️.json` names what each batch leaves.", "/// `🖥️Board2dHost/🧪️tests/🧫️fixtures/🧫️board-event-coalescing/🔣️.json` names what each batch leaves."),
    ],
    E + "🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧮️render-plan-validator/🦀️.rs": [
        ('        check_json_payload(&format!("{scene_label} board2d.placementCompatibility"), &board.placement_compatibility_json, limits)?;\n',
         '        check_json_payload(&format!("{scene_label} board2d.placementCompatibility"), &board.placement_compatibility_json, limits)?;\n'
         '        check_optional_json_payload(&format!("{scene_label} board2d.placementNaming"), &board.placement_naming, limits)?;\n'),
    ],
    H + "🟦️.tsx": [
        (" * `🧫️fixtures/🧫️board-event-coalescing/🔣️.json` pins this", " * `🧪️tests/🧫️fixtures/🧫️board-event-coalescing/🔣️.json` pins this"),
        ("    applyToSession(sessionRef.current, (session) => session.setAreaBrushExtent?.(size.width, size.height));\n  }, [sessionEpoch, scene?.areaBrushSize]);\n",
         "    applyToSession(sessionRef.current, (session) => session.setAreaBrushExtent?.(size.width, size.height));\n  }, [sessionEpoch, scene?.areaBrushSize]);\n\n"
         "  // 🏷️ The owning domain's brush placement naming, so this lane mints the same ids as the wgpu lane and the domain's own boards.\n"
         "  useEffect(() => {\n    if (!scene) return;\n"
         "    applyToSession(sessionRef.current, (session) => session.setPlacementNamingJson?.(scene.placementNaming ?? null));\n"
         "  }, [sessionEpoch, scene?.placementNaming]);\n"),
    ],
    E + "🧱️elements/🪪️WasmSessionLoader/🟦️.tsx": [
        ("  setAreaBrushExtent?(width: number, height: number): void;\n",
         "  setAreaBrushExtent?(width: number, height: number): void;\n"
         "  /** 🏷️ Installs (a string) or clears (`null`) the scene's brush placement prefixes in the board's wire form (`Board2dScene.placementNaming`). */\n"
         "  setPlacementNamingJson?(json: string | null): void;\n"),
    ],
    H + "🧪️tests/🧫️fixtures/🧫️board-event-coalescing/🔣️.json": [
        ('"rows": [{ "name": "brushPlace", "payload": { "nodeId": "puzzle2d.brush.1", "edgeId": "puzzle2d.brush.edge.1",',
         '"rows": [{ "name": "brushPlace", "payload": { "nodeId": "board.brush.1", "edgeId": "board.brush.edge.1",'),
        ('"events": [{ "name": "brushPlace", "payload": { "nodeId": "puzzle2d.brush.1", "edgeId": "puzzle2d.brush.edge.1",',
         '"events": [{ "name": "brushPlace", "payload": { "nodeId": "board.brush.1", "edgeId": "board.brush.edge.1",'),
    ],
    P + "🎭️modes/✏️edit/🦀️.rs": [
        ("use crate::editor::puzzle2d::engine::{BoardHost, BOARD_CAMERA_ZOOM_MAX, BOARD_CAMERA_ZOOM_MIN};\n",
         "use crate::editor::puzzle2d::engine::board_host::PUZZLE_2D_BOARD_NAMING;\nuse crate::editor::puzzle2d::engine::{BoardHost, BOARD_CAMERA_ZOOM_MAX, BOARD_CAMERA_ZOOM_MIN};\n"),
        ("        domain_id: Some(crate::editor::puzzle2d::snapshot::PUZZLE2D_INTERACTION_DOMAIN.into()),\n        suggestion_menu_json,\n",
         "        domain_id: Some(crate::editor::puzzle2d::snapshot::PUZZLE2D_INTERACTION_DOMAIN.into()),\n"
         "        placement_naming: Some(PUZZLE_2D_BOARD_NAMING.placement_naming().to_json()),\n"
         "        suggestion_menu_json,\n"),
    ],
    P + "🌉️wasm/🦀️.rs": [
        ("    #[wasm_bindgen(js_name = setAreaBrushExtent)]\n",
         "    /// 🏷️ Installs (`Some`) or clears the scene's brush placement prefixes in the board's wire form\n"
         "    /// (`Board2dScene.placementNaming`); `None` falls back to this session's installed puzzle naming.\n"
         "    #[wasm_bindgen(js_name = setPlacementNamingJson)]\n"
         "    pub fn set_placement_naming_json_wasm(&mut self, json: Option<String>) -> Result<(), JsValue> {\n"
         "        self.state.borrow_mut().host.set_placement_naming_json(json.as_deref()).map_err(|e| JsValue::from_str(&e.to_string()))\n"
         "    }\n\n"
         "    #[wasm_bindgen(js_name = setAreaBrushExtent)]\n"),
    ],
    P + "⚙️engine/🦀️.rs": [(ENGINE_DOC_OLD, ENGINE_DOC_NEW)],
    P + "⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs": [
        ("    assert!(std::ptr::eq(puzzle_board_host().naming(), &PUZZLE_2D_BOARD_NAMING));\n",
         PLUGIN_LABEL_ASSERT + "    assert!(std::ptr::eq(puzzle_board_host().naming(), &PUZZLE_2D_BOARD_NAMING));\n"),
    ],
    P5 + "👁️viewer/🎭️modes/👁️view/🪟️windows/◻️2d/🦀️.rs": [
        ("        area_brush_size: None,\n        domain_id: None,\n        suggestion_menu_json: None,\n",
         "        area_brush_size: None,\n        domain_id: None,\n        placement_naming: None,\n        suggestion_menu_json: None,\n"),
    ],
    P5 + "✏️editor/🎭️modes/✏️edit/🪟️windows/◻️2d/🦀️.rs": [
        ("        domain_id: Some(crate::editor::puzzle5d::PUZZLE5D_INTERACTION_DOMAIN.into()),\n        suggestion_menu_json: None,\n",
         "        domain_id: Some(crate::editor::puzzle5d::PUZZLE5D_INTERACTION_DOMAIN.into()),\n"
         "        placement_naming: Some(puzzle5d_board_placement_naming().to_json()),\n"
         "        suggestion_menu_json: None,\n"),
        ("use semio_framework_pack_json::{json, Value};\n", "use semio_framework_pack_json::{json, Value};\nuse semio_framework_os_infinite::BoardPlacementNaming;\n"),
        ("/// 🎬️ The board host's whole scene payload for one render",
         "/// 🪧️ The ids a 5d board's brush placements are minted under, owned by this plugin: 5d parts placed on the board have\n"
         "/// always carried the puzzle board's `puzzle2d.brush.<serial>` / `puzzle2d.brush.edge.<serial>` ids, kept byte-identical.\n"
         "pub fn puzzle5d_board_placement_naming() -> BoardPlacementNaming {\n"
         "    BoardPlacementNaming { brush_node_prefix: \"puzzle2d.brush.\".into(), brush_edge_prefix: \"puzzle2d.brush.edge.\".into() }\n"
         "}\n\n"
         "/// 🎬️ The board host's whole scene payload for one render"),
    ],
}


def apply_family():
    for path in FAMILY_FILES:
        text = open(path, encoding="utf-8").read()
        new, count = FAMILY.subn(lambda m: "board2d_" + m.group(1), text)
        assert count > 0, path
        assert "puzzle_board_host" not in new or "puzzle_board_host" in text
        open(path, "w", encoding="utf-8").write(new)
        print(f"renamed {count:3} in", path.replace(R, ""))
    for path in EVENT_SET_FILES:
        text = open(path, encoding="utf-8").read()
        new = text.replace("PUZZLE2D_TRANSIENT_EVENT_NAMES", "BOARD_TRANSIENT_EVENT_NAMES").replace("PUZZLE2D_FLUSH_NOW_EVENT_NAMES", "BOARD_FLUSH_NOW_EVENT_NAMES")
        assert new != text, path
        open(path, "w", encoding="utf-8").write(new)
        print("event sets in", path.replace(R, ""))


def apply_tests():
    path = W + "🧪️tests/🔬️wgpu-board2d-engine/🦀️.rs"
    text = open(path, encoding="utf-8").read()
    anchor = "/// 🎛️ Law: the rotate ring runs on wgpu as it runs on React"
    assert text.count(anchor) == 1
    open(path, "w", encoding="utf-8").write(text.replace(anchor, WGPU_NAMING_TEST.lstrip("\n") + "\n" + anchor))
    print("test added in", path.replace(R, ""))


NAMING_LITERAL = 'r#"{"brushNodePrefix":"puzzle2d.brush.","brushEdgePrefix":"puzzle2d.brush.edge."}"#'

TEST_2D = """
/// 🏷️ LAW: every board pane publishes the plugin's placement naming, so the os wgpu host and the React session mint the
/// same `puzzle2d.brush.<serial>` ids as the plugin's own boards.
#[test]
fn board_scene_publishes_the_plugin_placement_naming() {
    let snapshot: Value = semio_framework_pack_json::from_json_str(&crate::examples::puzzle2d::concrete_forest::SOURCE.document_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("concrete forest json");
    let envelope = Puzzle2dScene { board_snapshot: snapshot, runtime: Default::default(), active_utility: "select".into(), interaction: Puzzle2dInteractionSnapshot::default() };
    assert_eq!(edit::puzzle2d_board_scene("{}", &envelope, crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID).placement_naming.as_deref(), Some(NAMING));
}
""".replace("NAMING", NAMING_LITERAL)

TEST_5D = """
/// 🪧️ Law: the 5d board publishes its own placement naming on the scene, so the wgpu and React hosts mint the ids 5d parts
/// have always carried (`puzzle2d.brush.<serial>`), byte for byte.
#[test]
fn board_scene_publishes_the_plugin_placement_naming() {
    let scene = board_scene_with(crate::editor::puzzle5d::Puzzle5dInteractionSnapshot::default(), Puzzle5dRuntime::default());
    assert_eq!(scene.placement_naming.as_deref(), Some(NAMING));
}
""".replace("NAMING", NAMING_LITERAL)


def apply_plugin_tests():
    path = P + "🧪️tests/🔬️unit/🦀️.rs"
    text = open(path, encoding="utf-8").read()
    anchor = "/// 💡️ LAW: the context menu offers the popup on ONE selected handle"
    assert text.count(anchor) == 1
    open(path, "w", encoding="utf-8").write(text.replace(anchor, TEST_2D.lstrip("\n") + "\n" + anchor))
    path = P5 + "✏️editor/🎭️modes/✏️edit/🪟️windows/◻️2d/🧪️tests/🔬️unit/🦀️.rs"
    text = open(path, encoding="utf-8").read()
    open(path, "w", encoding="utf-8").write(text.rstrip("\n") + "\n" + TEST_5D)
    print("plugin scene tests added")


if __name__ == "__main__":
    board.apply(EDITS)
    apply_family()
    apply_tests()
    apply_plugin_tests()
