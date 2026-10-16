"""🏷️ Slice D board edits: neutral LOD names, the BoardNaming registration point and neutral docs in os-infinite."""
import sys

R = "/Users/ueli/Documents/semio/"
INF = R + "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/"
D = INF + "🎲️board/🔌️ports/➡️directed/"
B = D + "➕️normal/"

NAMING_BLOCK = '''
/// 🏷️ Domain naming a board consumer installs through [`BoardHost::set_naming`] and [`BoardFillSnapshotIngress::new`]:
/// the id prefixes of brush and fill placements, and the fill job's progress stage and stale-operation labels.
#[derive(Clone, Copy, Debug)]
pub struct BoardNaming {
    pub brush_node_prefix: &'static str,
    pub brush_edge_prefix: &'static str,
    pub fill_node_prefix: &'static str,
    pub fill_edge_prefix: &'static str,
    pub fill_stage_label: fn(BoardFillStage) -> &'static str,
    pub fill_stale_operation_fault: &'static str,
}

/// ⚪️ The board's own naming until a consumer installs one: `board.brush.<serial>`, `board.fill.<serial>` and bare stage ids.
pub static NEUTRAL_BOARD_NAMING: BoardNaming = BoardNaming {
    brush_node_prefix: "board.brush.",
    brush_edge_prefix: "board.brush.edge.",
    fill_node_prefix: "board.fill.",
    fill_edge_prefix: "board.fill.edge.",
    fill_stage_label: BoardFillStage::id,
    fill_stale_operation_fault: "stale-board-fill-operation",
};
'''

STAGE_MATCH_START = "    fn stage_label(&self) -> &'static str {\n        match self.stage() {\n"
STAGE_MATCH_END = '            BoardFillStage::Complete => "puzzle2d-fill-complete",\n        }\n    }\n'

NAMING_TEST = '''
#[cfg(test)]
/// 🧷️ A consumer's installed naming is what the board mints with: brush placements carry its prefixes and a fill
/// snapshot built under it hands the job its labels, while an untouched host keeps the neutral naming.
#[test]
fn installed_naming_owns_brush_ids_and_fill_labels() {
    fn probe_stage_label(stage: BoardFillStage) -> &'static str {
        if stage == BoardFillStage::Complete { "probe-complete" } else { "probe-busy" }
    }
    static PROBE_NAMING: BoardNaming = BoardNaming {
        brush_node_prefix: "probe.node.",
        brush_edge_prefix: "probe.edge.",
        fill_node_prefix: "probe.fill.",
        fill_edge_prefix: "probe.fill.edge.",
        fill_stage_label: probe_stage_label,
        fill_stale_operation_fault: "probe-stale",
    };
    let place = |naming: Option<&'static BoardNaming>| {
        let mut host = BoardHost::new();
        if let Some(naming) = naming {
            host.set_naming(naming);
        }
        host.set_size(800, 600, 1.0);
        host.set_active_utility("brush");
        host.set_suggestion_offset(40.0);
        host.set_brush_node_size(40.0);
        let catalogs = serde_json::json!({
            "handleKinds": [{ "id": "port", "name": "Port", "color": "#888" }],
            "nodeKinds": [{ "id": "brush.kind", "name": "Brush Kind", "handles": [{ "handleKind": "port", "angle": 3.141592653589793 }] }]
        });
        host.set_board_kind_catalogs_from_json(&catalogs.to_string()).expect("probe catalogs");
        let scene = SceneDescriptor {
            nodes: vec![NodeDescriptor { id: "a".into(), x: 0.0, y: 0.0, draggable: Some(true), selected: None, style: None, text: None, icon_kind: None, node_kind: Some("a.kind".into()), user_data: None, visible: None, locked: None, root: None, shape: Some("circle".into()), radius: Some(40.0), width: None, height: None, scale: None }].into(),
            handles: vec![HandleDescriptor { id: "a:h0".into(), node_id: "a".into(), angle: 0.0, radius: None, scale: None, selected: None, visible: None, locked: None, style: None, handle_kind: Some("port".into()), color: None, icon_kind: None, user_data: None }].into(),
            edges: vec![].into(),
            wires: vec![].into(),
            regions: vec![],
            selection_exit_highlight_ids: vec![],
        };
        host.sync_descriptor(&scene).expect("probe scene syncs");
        let handle = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let slot = host.world_to_screen(handle + (handle - Point::new(0.0, 0.0)));
        host.pointer_move_screen(slot.x, slot.y, false, false, false);
        let _ = host.drain_events_json();
        host.pointer_leave_screen(true);
        let events = host.drain_events_json();
        assert!(std::ptr::eq(host.naming(), naming.unwrap_or(&NEUTRAL_BOARD_NAMING)), "the host reports the naming it mints with");
        events
    };
    let neutral = place(None);
    assert!(neutral.contains("\\"board.brush.1\\"") && neutral.contains("\\"board.brush.edge.1\\""), "the untouched host mints neutral ids: {neutral}");
    let probe = place(Some(&PROBE_NAMING));
    assert!(probe.contains("\\"probe.node.1\\"") && probe.contains("\\"probe.edge.1\\""), "the installed naming prefixes brush placements: {probe}");
    assert!(!probe.contains("board.brush."), "no neutral id leaks past an installed naming: {probe}");
    let mut ingress = BoardFillSnapshotIngress::new(0.0, &PROBE_NAMING);
    let snapshot = ingress.take_snapshot().expect("empty probe snapshot");
    assert!(std::ptr::eq(snapshot.naming, &PROBE_NAMING), "the ingress hands its naming to the snapshot");
    let mut job = BoardFillJob::with_operation(snapshot, 0, semio_framework_job::Operation::new(semio_framework_job::OperationId(1), semio_framework_job::RevisionId(1), semio_framework_job::Generation(1), 1));
    assert!(std::ptr::eq(job.naming, &PROBE_NAMING), "the job keeps the snapshot's naming");
    assert_eq!((job.naming.fill_stage_label)(job.stage()), "probe-complete");
    close_probe_fill_job(&mut job);
}

#[cfg(test)]
fn close_probe_fill_job(job: &mut BoardFillJob) {
    use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep};
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    for turn in 0.. {
        assert!(turn < 64, "probe job close reaches terminal-empty");
        let copy = job.next_close_copy_byte_demand().expect("job copy demand");
        let exact = RetainedCloneGrant {
            maximum_items: 1,
            maximum_copy_bytes: copy,
            maximum_capacity_bytes: job.next_close_capacity_byte_demand(copy).expect("job capacity demand"),
            maximum_release_bytes: job.next_close_release_byte_demand().expect("job release demand"),
            maximum_depth: job.next_close_depth_demand().expect("job depth demand"),
        };
        match job.close_step(exact) {
            InteractiveJobCloseStep::Pending { .. } => {}
            InteractiveJobCloseStep::Complete { .. } => break,
            other => panic!("probe job close ended in {other:?}"),
        }
    }
    assert!(job.terminal_is_empty());
}
'''

EDITS = {
    B + "🦀️.rs": [
        ("const PUZZLE_2D_LODS: &[Lod; 6] = &[", "const BOARD_LODS: &[Lod; 6] = &["),
        ("const PUZZLE_2D_LOD_SCALE: LodScale = LodScale { lods: PUZZLE_2D_LODS };", "const BOARD_LOD_SCALE: LodScale = LodScale { lods: BOARD_LODS };"),
        ("pub fn puzzle_2d_lod_scale_json() -> String {\n    let rows: Vec<serde_json::Value> = PUZZLE_2D_LODS",
         "/// 🔭️ The board's six draw LOD bands ([`BoardDrawLod`]) as `[{id, name, description, maxZoom}]` JSON for LOD pickers.\npub fn board_lod_scale_json() -> String {\n    let rows: Vec<serde_json::Value> = BOARD_LODS"),
        ('    serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into())\n}\n\n/// 🎨️ Whether drawable style',
         '    serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into())\n}\n' + NAMING_BLOCK + '\n/// 🎨️ Whether drawable style'),
        ("    rules: BoardFillFixedPages<BoardFillRuleSnapshot, { BOARD_FILL_RULE_CAPACITY / BOARD_FILL_PAGE_ITEMS }>,\n    suggestion_offset: f64,\n}",
         "    rules: BoardFillFixedPages<BoardFillRuleSnapshot, { BOARD_FILL_RULE_CAPACITY / BOARD_FILL_PAGE_ITEMS }>,\n    suggestion_offset: f64,\n    naming: &'static BoardNaming,\n}"),
        ('"Puzzle2d fill fixed descriptor backing allocation"', '"Board fill fixed descriptor backing allocation"'),
        ('"Puzzle2d fill snapshot must transfer', '"Board fill snapshot must transfer'),
        ('"Puzzle2d fill placement must reach', '"Board fill placement must reach'),
        ('"Puzzle2d fill checkpoint must transfer', '"Board fill checkpoint must transfer'),
        ("/// 🧩️ Typed placement transferred to the Puzzle2d document applier without a JSON callback.", "/// 🧩️ Typed placement transferred to the owning domain's document applier without a JSON callback."),
        ("pub struct BoardFillCheckpoint {\n    operation: semio_framework_job::Operation,\n    state: Option<BoardFillJobState>,\n}",
         "pub struct BoardFillCheckpoint {\n    operation: semio_framework_job::Operation,\n    naming: &'static BoardNaming,\n    state: Option<BoardFillJobState>,\n}"),
        ("BoardFillJob { operation: self.operation, state: self.state.take(), checkpoint: None,", "BoardFillJob { operation: self.operation, naming: self.naming, state: self.state.take(), checkpoint: None,"),
        ("pub struct BoardFillJob {\n    operation: semio_framework_job::Operation,\n", "pub struct BoardFillJob {\n    operation: semio_framework_job::Operation,\n    naming: &'static BoardNaming,\n"),
        ("    brush_placement_serial: u64,\n    brush_node_kind_weights",
         "    brush_placement_serial: u64,\n    /// 📛️ Placement naming the owning domain installed ([`BoardHost::set_naming`]); [`NEUTRAL_BOARD_NAMING`] until then.\n    naming: &'static BoardNaming,\n    brush_node_kind_weights"),
    ],
    B + "🎨️style/🦀️.rs": [
        ("            brush_placement_serial: 0,\n", "            brush_placement_serial: 0,\n            naming: &NEUTRAL_BOARD_NAMING,\n"),
        ("PUZZLE_2D_LOD_SCALE.resolve_index", "BOARD_LOD_SCALE.resolve_index"),
        ('        let node_id = format!("puzzle2d.brush.{serial}");\n        let edge_id = format!("puzzle2d.brush.edge.{serial}");\n        let Some(reservation) = self.reserve_owned_event(',
         '        let node_id = format!("{}{serial}", self.naming.brush_node_prefix);\n        let edge_id = format!("{}{serial}", self.naming.brush_edge_prefix);\n        let Some(reservation) = self.reserve_owned_event('),
        ("        host.selection_options.select_handles = false;\n        host\n    }\n",
         "        host.selection_options.select_handles = false;\n        host\n    }\n\n"
         "    /// 🪪️ Installs the owning domain's placement naming: brush placements minted and fill snapshots captured\n"
         "    /// afterwards carry its id prefixes and stage labels.\n"
         "    pub fn set_naming(&mut self, naming: &'static BoardNaming) {\n        self.naming = naming;\n    }\n\n"
         "    /// 🆔️ The placement naming this host mints ids with.\n"
         "    pub fn naming(&self) -> &'static BoardNaming {\n        self.naming\n    }\n"),
    ],
    B + "🪣️fill-job/🦀️.rs": [
        ("BoardFillSnapshotCapture::new(self.suggestion_offset)", "BoardFillSnapshotCapture::new(self.suggestion_offset, self.naming)"),
        ("    pub(super) fn new(suggestion_offset: f64) -> Self {\n        Self {\n            snapshot: Some(BoardFillSnapshot { nodes: BoardFillFixedPages::new(), handles: BoardFillFixedPages::new(), kinds: BoardFillFixedPages::new(), rules: BoardFillFixedPages::new(), suggestion_offset }),",
         "    pub(super) fn new(suggestion_offset: f64, naming: &'static BoardNaming) -> Self {\n        Self {\n            snapshot: Some(BoardFillSnapshot { nodes: BoardFillFixedPages::new(), handles: BoardFillFixedPages::new(), kinds: BoardFillFixedPages::new(), rules: BoardFillFixedPages::new(), suggestion_offset, naming }),"),
        ("    pub fn new(suggestion_offset: f64) -> Self {\n        Self {\n            snapshot: Some(BoardFillSnapshot { nodes: BoardFillFixedPages::new(), handles: BoardFillFixedPages::new(), kinds: BoardFillFixedPages::new(), rules: BoardFillFixedPages::new(), suggestion_offset }),",
         "    pub fn new(suggestion_offset: f64, naming: &'static BoardNaming) -> Self {\n        Self {\n            snapshot: Some(BoardFillSnapshot { nodes: BoardFillFixedPages::new(), handles: BoardFillFixedPages::new(), kinds: BoardFillFixedPages::new(), rules: BoardFillFixedPages::new(), suggestion_offset, naming }),"),
        ("    pub fn with_operation(snapshot: BoardFillSnapshot, max_count: u32, operation: semio_framework_job::Operation) -> Self {\n        Self {\n            operation,\n",
         "    pub fn with_operation(snapshot: BoardFillSnapshot, max_count: u32, operation: semio_framework_job::Operation) -> Self {\n        Self {\n            operation,\n            naming: snapshot.naming,\n"),
        ("Ok(Self { operation, state: Some(state), checkpoint: None,", "Ok(Self { operation, naming: checkpoint.naming, state: Some(state), checkpoint: None,"),
        ("BoardFillCheckpoint { operation: self.operation, state: Some(state) }", "BoardFillCheckpoint { operation: self.operation, naming: self.naming, state: Some(state) }"),
        ('BoardFillText::try_indexed("puzzle2d.fill.", state.next_serial, "", None)', 'BoardFillText::try_indexed(state.snapshot.naming.fill_node_prefix, state.next_serial, "", None)'),
        ('BoardFillText::try_indexed("puzzle2d.fill.edge.", state.next_serial, "", None)', 'BoardFillText::try_indexed(state.snapshot.naming.fill_edge_prefix, state.next_serial, "", None)'),
        ('BoardFillText::try_indexed("puzzle2d.fill.", state.next_serial, ":h", Some(preview.target_handle_index))', 'BoardFillText::try_indexed(state.snapshot.naming.fill_node_prefix, state.next_serial, ":h", Some(preview.target_handle_index))'),
        ('BoardFillText::try_indexed("puzzle2d.fill.", serial, ":h", Some(state.accept_handle_cursor))', 'BoardFillText::try_indexed(state.snapshot.naming.fill_node_prefix, serial, ":h", Some(state.accept_handle_cursor))'),
        ('self.fault_outcome("stale-puzzle2d-fill-operation")', "self.fault_outcome(self.naming.fill_stale_operation_fault)"),
        ('"Puzzle2d fill capture must reach', '"Board fill capture must reach'),
        ('"Puzzle2d artifact fill ingress must reach', '"Board fill ingress must reach'),
        ('"Puzzle2d fill job must reach', '"Board fill job must reach'),
    ],
    B + "🧹️scene-delete-links/🦀️.rs": [
        ('        let node_id = format!("puzzle2d.brush.{serial}");\n        let edge_id = format!("puzzle2d.brush.edge.{serial}");\n        let events = [',
         '        let node_id = format!("{}{serial}", self.naming.brush_node_prefix);\n        let edge_id = format!("{}{serial}", self.naming.brush_edge_prefix);\n        let events = ['),
    ],
    B + "🧭️host/🦀️.rs": [
        ("may carry. Sized for the largest shipped puzzle 2d example: Nakagin", "may carry. Sized for the largest shipped board example: Nakagin"),
        ("gesture payload credits above: a 100-placement puzzle 2d fill of an 11-handle kind", "gesture payload credits above: a 100-placement fill of an 11-handle kind"),
    ],
    B + "🧪️tests/🔬️board-host-standalone/🦀️.rs": [
        ("        let mut ingress = BoardFillSnapshotIngress::new(0.0);", "        let mut ingress = BoardFillSnapshotIngress::new(0.0, &NEUTRAL_BOARD_NAMING);"),
        ("        let mut seed = BoardFillSnapshotIngress::new(0.0);", "        let mut seed = BoardFillSnapshotIngress::new(0.0, &NEUTRAL_BOARD_NAMING);"),
        ("||BoardFillSnapshotCapture::new(0.0));", "||BoardFillSnapshotCapture::new(0.0, &NEUTRAL_BOARD_NAMING));"),
        ("||BoardFillSnapshotIngress::new(0.0));", "||BoardFillSnapshotIngress::new(0.0, &NEUTRAL_BOARD_NAMING));"),
        ("credits: a hundred-placement puzzle 2d fill of an eleven-handle kind", "credits: a hundred-placement fill of an eleven-handle kind"),
        ("carries no `camera` key at all (puzzle 2d since its\n    /// `setCamera` became a View-kind verb).", "carries no `camera` key at all (a domain whose\n    /// `setCamera` is a View-kind verb)."),
        ("/// 🎯️ A puzzle 2d fixture's handle `radius` is the cap size", "/// 🎯️ A board handle's authored `radius` is the cap size"),
        ("fn puzzle2d_edge_starts_at_the_handle_cap() {", "fn board_edge_starts_at_the_handle_cap() {"),
    ],
    D + "🕸️dag/🦀️.rs": [
        ("/// 🎨️ Node body fill when painted; `None` means stroke/text only (puzzle 2d overview+).", "/// 🎨️ Node body fill when painted; `None` means stroke/text only (board overview+)."),
    ],
    D + "🕸️dag/🧪️tests/🔬️unit/🦀️.rs": [
        ("fn dag_draw_lod_maps_zoom_to_puzzle2d_bands() {", "fn dag_draw_lod_maps_zoom_to_board_bands() {"),
        ("fn dag_node_paint_fill_matches_puzzle2d_lod_chrome() {", "fn dag_node_paint_fill_matches_board_lod_chrome() {"),
    ],
    D + "🦀️.rs": [
        ("/// ⚙️ Puzzle 2d board engine alias.", "/// ⚙️ Directed port board engine alias."),
    ],
    INF + "🎲️board/🧬️schema/💡️inferences/📐️layout/⚛️force/🦀️.rs": [
        ("on a mindmap or puzzle 2d snapshot with node-id edges.", "on a board snapshot with node-id edges."),
    ],
    INF + "📖️stories/🎭️graph-wasm-canvas/🧪️.story.tsx": [
        ("no real WASM session, no puzzle-2d program.", "no real WASM session, no board program."),
    ],
}


def apply(edits):
    for path, pairs in edits.items():
        text = open(path, encoding="utf-8").read()
        for old, new in pairs:
            count = text.count(old)
            assert count == 1, (path, old[:80], count)
            text = text.replace(old, new)
        if path.endswith("🪣️fill-job/🦀️.rs"):
            assert text.count(STAGE_MATCH_START) == 1 and text.count(STAGE_MATCH_END) == 1
            start = text.index(STAGE_MATCH_START)
            end = text.index(STAGE_MATCH_END) + len(STAGE_MATCH_END)
            assert end > start and end - start < 3000
            text = text[:start] + "    fn stage_label(&self) -> &'static str {\n        (self.naming.fill_stage_label)(self.stage())\n    }\n" + text[end:]
        if path.endswith("🔬️board-host-standalone/🦀️.rs"):
            text = text.rstrip("\n") + "\n" + NAMING_TEST
        open(path, "w", encoding="utf-8").write(text)
        print("edited", path.replace(R, ""))


if __name__ == "__main__":
    apply(EDITS)
