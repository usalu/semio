"""🪧️ Slice D plugin edits: the puzzle 2d plugin declares and installs its board naming and reads the neutral LOD table."""
import sys

sys.path.insert(0, __file__.rsplit("/", 1)[0])
from importlib import util

spec = util.spec_from_file_location("apply_board", __file__.rsplit("/", 1)[0] + "/apply-board.py")
board = util.module_from_spec(spec)
spec.loader.exec_module(board)

R = board.R
P = R + "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/"
P5 = R + "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/"

STAGES = [
    ("ResetSources", "reset-sources"), ("PrepareSources", "prepare-sources"), ("SelectTarget", "select-target"),
    ("ResetCandidates", "reset-candidates"), ("PrepareCandidates", "prepare-candidates"), ("ScanCompatibility", "scan-compatibility"),
    ("SelectCandidate", "select-candidate"), ("ConstructPreview", "construct-preview"), ("ScanHostCollision", "scan-host-collision"),
    ("ScanVirtualCollision", "scan-virtual-collision"), ("AcceptCandidate", "accept-candidate"), ("AcceptNodeId", "accept-node-id"),
    ("AcceptEdgeId", "accept-edge-id"), ("AcceptEdgeKind", "accept-edge-kind"), ("AcceptNodeKind", "accept-node-kind"),
    ("AcceptSourceHandle", "accept-source-handle"), ("AcceptTargetHandle", "accept-target-handle"), ("AcceptIcon", "accept-icon"),
    ("AcceptVirtualNode", "accept-virtual-node"), ("AcceptSourceConnection", "accept-source-connection"), ("AcceptHandles", "accept-handles"),
    ("AcceptHandleId", "accept-handle-id"), ("AcceptHandleVirtual", "accept-handle-virtual"), ("AcceptHandlePublish", "accept-handle-publish"),
    ("PublishPlanPrefix", "publish-prefix"), ("Complete", "complete"),
]

STAGE_ARMS = "".join(f'        BoardFillStage::{variant} => "puzzle2d-fill-{label}",\n' for variant, label in STAGES)

NAMING = '''use crate::editor::puzzle2d::engine::icons::puzzle_themed_icon_lookup;
use crate::editor::puzzle2d::engine::{BoardFillStage, BoardHost, BoardNaming};

/// 🪧️ The ids this plugin's boards mint and the labels its fill job reports: persisted document ids
/// (`puzzle2d.brush.<serial>`, `puzzle2d.fill.<serial>`) and job progress stages, owned by the puzzle 2d plugin and
/// installed on every puzzle board ([`BoardHost::set_naming`]) and fill ingress (`BoardFillSnapshotIngress::new`).
pub static PUZZLE_2D_BOARD_NAMING: BoardNaming = BoardNaming {
    brush_node_prefix: "puzzle2d.brush.",
    brush_edge_prefix: "puzzle2d.brush.edge.",
    fill_node_prefix: "puzzle2d.fill.",
    fill_edge_prefix: "puzzle2d.fill.edge.",
    fill_stage_label: puzzle_2d_fill_stage_label,
    fill_stale_operation_fault: "stale-puzzle2d-fill-operation",
};

/// 🚦️ The job progress stage a puzzle board's fill reports for one board fill stage.
fn puzzle_2d_fill_stage_label(stage: BoardFillStage) -> &'static str {
    match stage {
''' + STAGE_ARMS + '''    }
}

/// 🎲️ A `BoardHost` for the directed port graph, painting icons from this plugin's metabolism table.
pub fn puzzle_board_host() -> BoardHost {
    let mut h = BoardHost::new();
    h.icon_paint_cache.themed_icon_lookup = puzzle_themed_icon_lookup;
    h.set_naming(&PUZZLE_2D_BOARD_NAMING);
    h
}

/// 🎲️ The undirected ("normal") variant of [`puzzle_board_host`].
pub fn puzzle_board_host_normal() -> BoardHost {
    let mut h = BoardHost::new_normal();
    h.icon_paint_cache.themed_icon_lookup = puzzle_themed_icon_lookup;
    h.set_naming(&PUZZLE_2D_BOARD_NAMING);
    h
}
'''

OLD_HEAD = '''use crate::editor::puzzle2d::engine::icons::puzzle_themed_icon_lookup;
use crate::editor::puzzle2d::engine::BoardHost;

/// 🎲️ A `BoardHost` for the directed port graph, painting icons from this plugin's metabolism table.
pub fn puzzle_board_host() -> BoardHost {
    let mut h = BoardHost::new();
    h.icon_paint_cache.themed_icon_lookup = puzzle_themed_icon_lookup;
    h
}

/// 🎲️ The undirected ("normal") variant of [`puzzle_board_host`].
pub fn puzzle_board_host_normal() -> BoardHost {
    let mut h = BoardHost::new_normal();
    h.icon_paint_cache.themed_icon_lookup = puzzle_themed_icon_lookup;
    h
}
'''

LABEL_TEST = '''
/// 🎫️ The puzzle board naming is the plugin's wire vocabulary, byte for byte: every fill stage keeps its
/// `puzzle2d-fill-*` progress label, placements keep their `puzzle2d.brush.` / `puzzle2d.fill.` prefixes, and both
/// board constructors install it.
#[test]
fn puzzle_board_naming_declares_the_plugin_wire_vocabulary() {
    use crate::editor::puzzle2d::engine::board_host::{puzzle_board_host, puzzle_board_host_normal, PUZZLE_2D_BOARD_NAMING};
    use crate::editor::puzzle2d::engine::BoardFillStage;
    let labels = [
''' + "".join(f'        (BoardFillStage::{variant}, "puzzle2d-fill-{label}"),\n' for variant, label in STAGES) + '''    ];
    for (stage, label) in labels {
        assert_eq!((PUZZLE_2D_BOARD_NAMING.fill_stage_label)(stage), label);
    }
    assert_eq!(
        (PUZZLE_2D_BOARD_NAMING.brush_node_prefix, PUZZLE_2D_BOARD_NAMING.brush_edge_prefix, PUZZLE_2D_BOARD_NAMING.fill_node_prefix, PUZZLE_2D_BOARD_NAMING.fill_edge_prefix, PUZZLE_2D_BOARD_NAMING.fill_stale_operation_fault),
        ("puzzle2d.brush.", "puzzle2d.brush.edge.", "puzzle2d.fill.", "puzzle2d.fill.edge.", "stale-puzzle2d-fill-operation")
    );
    assert!(std::ptr::eq(puzzle_board_host().naming(), &PUZZLE_2D_BOARD_NAMING));
    assert!(std::ptr::eq(puzzle_board_host_normal().naming(), &PUZZLE_2D_BOARD_NAMING));
}
'''

FILL_TEST = '''
    /// 🔏️ A puzzle board mints its fill placements under the plugin's `puzzle2d.fill.` naming; the same scene on a
    /// bare board mints the neutral `board.fill.` ids, so the prefix is the plugin's declaration, not the board's.
    #[test]
    fn puzzle_board_naming_owns_fill_placement_ids() {
        use crate::editor::puzzle2d::engine::board_host::PUZZLE_2D_BOARD_NAMING;
        let operation = Operation::new(semio_framework_job::OperationId(42), semio_framework_job::RevisionId(9), semio_framework_job::Generation(1), 42);
        let mut puzzle = frontier_fill_host();
        puzzle.set_naming(&PUZZLE_2D_BOARD_NAMING);
        let (puzzle_placements, _, _) = run_fill_job(&puzzle, 1, operation, 1);
        let (neutral_placements, _, _) = run_fill_job(&frontier_fill_host(), 1, operation, 1);
        assert!(!puzzle_placements.is_empty() && !neutral_placements.is_empty(), "the frontier scene admits a fill placement");
        assert!(puzzle_placements.iter().all(|placement| placement.target.starts_with("puzzle2d.fill.")), "{puzzle_placements:?}");
        assert!(neutral_placements.iter().all(|placement| placement.target.starts_with("board.fill.")), "{neutral_placements:?}");
    }
'''

EDITS = {
    P + "⚙️engine/🎲️board-host/🦀️.rs": [(OLD_HEAD, NAMING)],
    P + "🧵️retained/🦀️.rs": [("    let host = RefCell::new(BoardHost::default());", "    let host = RefCell::new(puzzle_board_host());")],
    P + "⏳️precompute/🪣️fill/🦀️.rs": [
        ("/// 🆔️ Prefix of the node ids the board engine mints for fill placements (`puzzle2d.fill.<serial>`).\npub(crate) const FILL_RUN_NODE_ID_PREFIX: &str = \"puzzle2d.fill.\";\n", ""),
        ("id.strip_prefix(FILL_RUN_NODE_ID_PREFIX)", "id.strip_prefix(PUZZLE_2D_BOARD_NAMING.fill_node_prefix)"),
        ("ingress: Some(BoardFillSnapshotIngress::new(suggestion_offset)),", "ingress: Some(BoardFillSnapshotIngress::new(suggestion_offset, &PUZZLE_2D_BOARD_NAMING)),"),
        ("use crate::editor::puzzle2d::snapshot::Puzzle2dPlaySnapshot;\n", "use crate::editor::puzzle2d::engine::board_host::PUZZLE_2D_BOARD_NAMING;\nuse crate::editor::puzzle2d::snapshot::Puzzle2dPlaySnapshot;\n"),
    ],
    P + "🌉️wasm/🦀️.rs": [
        ("handle_position_on_rectangle, puzzle_2d_lod_scale_json,", "handle_position_on_rectangle, board_lod_scale_json,"),
        ("        puzzle_2d_lod_scale_json()\n", "        board_lod_scale_json()\n"),
    ],
    P + "🎭️modes/✏️edit/☑️options/🔭️lod/🦀️.rs": [("crate::editor::puzzle2d::engine::puzzle_2d_lod_scale_json()", "crate::editor::puzzle2d::engine::board_lod_scale_json()")],
    P + "🎮️commands/🎛️runtime/📊️lod-scale-json/🦀️.rs": [("crate::editor::puzzle2d::engine::puzzle_2d_lod_scale_json()", "crate::editor::puzzle2d::engine::board_lod_scale_json()")],
    P5 + "🎭️modes/✏️edit/🪟️windows/◻️2d/☑️options/🔭️lod/🦀️.rs": [("semio_framework_os_infinite::puzzle_2d_lod_scale_json()", "semio_framework_os_infinite::board_lod_scale_json()")],
}


def apply_tests():
    path = P + "⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs"
    text = open(path, encoding="utf-8").read()
    open(path, "w", encoding="utf-8").write(text.rstrip("\n") + "\n" + LABEL_TEST)
    print("edited", path.replace(R, ""))
    path = P + "⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs"
    text = open(path, encoding="utf-8").read()
    anchor = "    #[test]\n    fn board_host_brush_fill_frontier_deterministic_and_collision_limited() {\n"
    assert text.count(anchor) == 1
    text = text.replace(anchor, FILL_TEST.lstrip("\n") + "\n" + anchor)
    open(path, "w", encoding="utf-8").write(text)
    print("edited", path.replace(R, ""))


if __name__ == "__main__":
    board.apply(EDITS)
    apply_tests()
