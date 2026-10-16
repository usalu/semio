"""🔎️ Slice D definition scan: every field, method and free item the relocation added or re-signed is reachable from each use site."""
import os
import re
import sys

ROOT = sys.argv[1] if len(sys.argv) > 1 else "/Users/ueli/Documents/semio"
B = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/"
P = "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/"
P5 = "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/"
MODULES = {
    B + "🦀️.rs": ("infinite", "normal"),
    B + "🧭️host/🦀️.rs": ("infinite", "normal::host"),
    B + "🧪️tests/🔬️board-host-standalone/🦀️.rs": ("infinite", "normal::host"),
    B + "🎨️style/🦀️.rs": ("infinite", "normal::style"),
    B + "🪣️fill-job/🦀️.rs": ("infinite", "normal::fill_job"),
    B + "🎛️selection-events/🦀️.rs": ("infinite", "normal::selection_events"),
    B + "🧹️scene-delete-links/🦀️.rs": ("infinite", "normal::scene_delete_links"),
    B + "🖱️pointer-gestures/🦀️.rs": ("infinite", "normal::pointer_gestures"),
    P + "⚙️engine/🎲️board-host/🦀️.rs": ("puzzle2d", "engine::board_host"),
    P + "⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs": ("puzzle2d", "engine::board_host::unit_tests"),
    P + "⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs": ("puzzle2d", "engine::brush::tests"),
    P + "⏳️precompute/🪣️fill/🦀️.rs": ("puzzle2d", "precompute::fill"),
    P + "🧵️retained/🦀️.rs": ("puzzle2d", "retained"),
    P + "🌉️wasm/🦀️.rs": ("puzzle2d", "wasm"),
    P + "🎭️modes/✏️edit/☑️options/🔭️lod/🦀️.rs": ("puzzle2d", "options::lod"),
    P + "🎮️commands/🎛️runtime/📊️lod-scale-json/🦀️.rs": ("puzzle2d", "commands::lod_scale_json"),
    P5 + "🎭️modes/✏️edit/🪟️windows/◻️2d/☑️options/🔭️lod/🦀️.rs": ("puzzle5d", "options::lod"),
}
E = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/"
W = E + "🧱️elements/⚙️EngineCanvas/"
PLANE = "🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🖼️plane/🦀️.rs"
MODULES.update({
    W + "🎯️targets/🧊️wgpu/🗂️engine-surface-registry/🦀️.rs": ("renderer", "engine_canvas::wgpu::registry"),
    W + "🎯️targets/🧊️wgpu/🧩️engine-surface-attach/🦀️.rs": ("renderer", "engine_canvas::wgpu::attach"),
    W + "🧪️tests/🔬️wgpu-board2d-engine/🦀️.rs": ("renderer", "engine_canvas::wgpu::board2d_engine_tests"),
    E + "🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧮️render-plan-validator/🦀️.rs": ("renderer", "interpreter::render_plan_validator"),
    PLANE: ("plane", "plane"),
    "🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🔬️plane-unit/🦀️.rs": ("plane", "plane::unit_tests"),
    P + "🎭️modes/✏️edit/🦀️.rs": ("puzzle2d", "modes::edit"),
    P + "🧪️tests/🔬️unit/🦀️.rs": ("puzzle2d", "editor::tests"),
    P5 + "🎭️modes/✏️edit/🪟️windows/◻️2d/🦀️.rs": ("puzzle5d", "windows::board2d"),
    P5 + "🎭️modes/✏️edit/🪟️windows/◻️2d/🧪️tests/🔬️unit/🦀️.rs": ("puzzle5d", "windows::board2d::tests"),
    "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/◻️2d/🦀️.rs": ("puzzle5d", "viewer::board2d"),
})
MEMBERS = [
    ("field", "BoardHost.placement_naming", r"^\s*(pub(?:\([^)]*\))?\s+)?placement_naming: Option<BoardPlacementNaming>", r"\bself\.placement_naming\b", B + "🦀️.rs"),
    ("vfield", "BoardSyncCache.placement_naming", r"^\s*(pub(?:\([^)]*\))?\s+)?placement_naming: Option<String>", r"\bcache\.placement_naming\b", W + "🎯️targets/🧊️wgpu/🗂️engine-surface-registry/🦀️.rs"),
    ("vfield", "Board2dScene.placement_naming", r"^\s*(pub(?:\([^)]*\))?\s+)?placement_naming: Option<String>", r"\b(?:board|scene|probe)\.placement_naming\b|^\s*placement_naming: (?:Some|None)", PLANE),
    ("item", "BoardPlacementNaming", r"^\s*(pub(?:\([^)]*\))?\s+)?struct BoardPlacementNaming\b", r"\bBoardPlacementNaming\b"),
    ("assoc", "BoardPlacementNaming::from_json", r"^\s*(pub(?:\([^)]*\))?\s+)?fn from_json\(json: &str\) -> Result<Self, NormalPortError>", r"BoardPlacementNaming::from_json\b"),
    ("method", "to_json", r"^\s*(pub(?:\([^)]*\))?\s+)?fn to_json\(&self\) -> String", r"\.to_json\(\)", B + "🦀️.rs"),
    ("method", "placement_naming()", r"^\s*(pub(?:\([^)]*\))?\s+)?fn placement_naming\(&self\) -> BoardPlacementNaming", r"\.placement_naming\(\)", B + "🦀️.rs"),
    ("method", "set_placement_naming_json", r"^\s*(pub(?:\([^)]*\))?\s+)?fn set_placement_naming_json\b", r"\.set_placement_naming_json\("),
    ("method", "brush_placement_naming", r"^\s*(pub(?:\([^)]*\))?\s+)?fn brush_placement_naming\b", r"\.brush_placement_naming\("),
    ("method", "brush_node_prefix()", r"^\s*(pub(?:\([^)]*\))?\s+)?fn brush_node_prefix\(&self\)", r"\.brush_node_prefix\(\)"),
    ("method", "brush_edge_prefix()", r"^\s*(pub(?:\([^)]*\))?\s+)?fn brush_edge_prefix\(&self\)", r"\.brush_edge_prefix\(\)"),
    ("item", "puzzle5d_board_placement_naming", r"^\s*(pub(?:\([^)]*\))?\s+)?fn puzzle5d_board_placement_naming\b", r"\bpuzzle5d_board_placement_naming\("),

    ("field", "naming", r"^\s*(pub(?:\([^)]*\))?\s+)?naming:\s*&'static BoardNaming", r"(?<![\w.])(?:self|snapshot|checkpoint|job|state\.snapshot)\.naming\b(?!\()"),
    ("method", "set_naming", r"^\s*(pub(?:\([^)]*\))?\s+)?fn set_naming\b", r"\.set_naming\("),
    ("method", "naming", r"^\s*(pub(?:\([^)]*\))?\s+)?fn naming\(", r"\.naming\(\)"),
    ("method", "stage_label", r"^\s*(pub(?:\([^)]*\))?\s+)?fn stage_label\b", r"\.stage_label\(\)"),
    ("assoc", "BoardFillSnapshotCapture::new", r"^\s*(pub(?:\([^)]*\))?\s+)?fn new\(suggestion_offset: f64, naming", r"BoardFillSnapshotCapture::new\("),
    ("item", "BoardNaming", r"^\s*(pub(?:\([^)]*\))?\s+)?struct BoardNaming\b", r"\bBoardNaming\b"),
    ("item", "NEUTRAL_BOARD_NAMING", r"^\s*(pub(?:\([^)]*\))?\s+)?static NEUTRAL_BOARD_NAMING\b", r"\bNEUTRAL_BOARD_NAMING\b"),
    ("item", "board_lod_scale_json", r"^\s*(pub(?:\([^)]*\))?\s+)?fn board_lod_scale_json\b", r"\bboard_lod_scale_json\(\)"),
    ("item", "BOARD_LOD_SCALE", r"^\s*(pub(?:\([^)]*\))?\s+)?const BOARD_LOD_SCALE\b", r"\bBOARD_LOD_SCALE\b"),
    ("item", "PUZZLE_2D_BOARD_NAMING", r"^\s*(pub(?:\([^)]*\))?\s+)?static PUZZLE_2D_BOARD_NAMING\b", r"\bPUZZLE_2D_BOARD_NAMING\b"),
    ("item", "puzzle_2d_fill_stage_label", r"^\s*(pub(?:\([^)]*\))?\s+)?fn puzzle_2d_fill_stage_label\b", r"\bpuzzle_2d_fill_stage_label\b"),
]
FIELDS = ["brush_node_prefix", "brush_edge_prefix", "fill_node_prefix", "fill_edge_prefix", "fill_stage_label", "fill_stale_operation_fault"]


def strip_comments(text):
    return "\n".join(re.sub(r"//.*$", "", line) for line in text.split("\n"))


def visible(vis, def_mod, use_mod):
    def_crate, def_path = def_mod
    use_crate, use_path = use_mod
    if vis is None:
        return use_crate == def_crate and (use_path == def_path or use_path.startswith(def_path + "::"))
    vis = vis.strip()
    if vis == "pub":
        return True
    if vis == "pub(crate)":
        return use_crate == def_crate
    if vis == "pub(super)":
        parent = def_path.rsplit("::", 1)[0] if "::" in def_path else ""
        return use_crate == def_crate and (not parent or use_path == parent or use_path.startswith(parent + "::"))
    return use_crate == def_crate


def scan():
    texts = {path: strip_comments(open(os.path.join(ROOT, path), encoding="utf-8").read()) for path in MODULES}
    findings, rows = [], []
    for member in MEMBERS:
        kind, name, def_rx, use_rx = member[:4]
        def_file = member[4] if len(member) > 4 else None
        defs = [(path, m.group(1)) for path, text in texts.items() for m in re.finditer(def_rx, text, re.M) if def_file in (None, path)]
        if not defs:
            findings.append(f"MISSING DEFINITION {name}")
            continue
        def_path, vis = defs[0]
        if kind == "field":
            vis = None
        uses = sorted({path for path, text in texts.items() if re.search(use_rx, text, re.M) and path != def_path and (kind != "field" or MODULES[path][0] == MODULES[def_path][0])})
        for use in uses:
            ok = visible(vis, MODULES[def_path], MODULES[use])
            rows.append(f"{name:28} {str(vis):12} def={MODULES[def_path][1]:22} use={MODULES[use][0]}::{MODULES[use][1]:32} {'ok' if ok else 'UNREACHABLE'}")
            if not ok:
                findings.append(f"UNREACHABLE {name} ({vis}) defined in {MODULES[def_path]} used in {MODULES[use]}")
    struct = re.search(r"pub struct BoardNaming \{(.*?)\n\}", texts[B + "🦀️.rs"], re.S)
    for field in FIELDS:
        if not re.search(rf"^\s*pub {field}:", struct.group(1), re.M):
            findings.append(f"PRIVATE BoardNaming.{field} is read by the puzzle 2d plugin")
    return rows, findings


if __name__ == "__main__":
    rows, findings = scan()
    print("\n".join(rows))
    print("findings:", findings or "none")
