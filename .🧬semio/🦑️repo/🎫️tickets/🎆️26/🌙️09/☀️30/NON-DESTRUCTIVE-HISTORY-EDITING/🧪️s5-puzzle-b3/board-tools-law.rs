//! 🧱️ Laws of the board tools (`🧫️fixtures/🧫️board-tools`): every board gesture that is not a selection transform —
//! an area painted, a region grip dragged, a brush stamped, a wire dropped, a wire cut, a delete — reaches the app as
//! its release and is ONE tool transaction of `<appId>#<kind>` holding exactly its leaves; a release that changes
//! nothing and a flush of transient rows alone leave zero trace. Driven through the registered app, one
//! `applyBoardEvents` dispatch per flush; `🐍️.py` beside this file re-derives the corpus independently.

use super::*;
use crate::editor::puzzle2d::unit_tests::context::*;
use semio_framework::kernel::HistoryEntry;
use semio_framework_plugin::InvocationResult;

const BOARD_TOOLS_CORPUS: &str = include_str!("../../🧫️fixtures/🧫️board-tools/🔣️.json");

/// 🧫️ The corpus, its schema tag checked.
fn corpus() -> Value {
    let corpus: Value = serde_json::from_str(BOARD_TOOLS_CORPUS).expect("the board-tools corpus parses");
    assert_eq!(corpus["schema"], "s.puzzle2d.board-tools.v1");
    corpus
}

/// 📬️ One flush of `rows` from the overview window.
fn flush(app: &mut Puzzle2dApp, rows: &Value) -> InvocationResult {
    dispatch(app, "applyBoardEvents", Some(&json!({ "eventsJson": rows.to_string() })), Some(overview::WINDOW_KIND_ID)).expect("applyBoardEvents")
}

/// 🧾️ The applied history rows one dispatch upserted that carry document ops.
fn edit_rows(result: &InvocationResult) -> Vec<HistoryEntry> {
    result.history_patch.as_ref().map(|patch| patch.upserts.iter().filter(|entry| entry.applied && !entry.op_lines.is_empty()).cloned().collect()).unwrap_or_default()
}

/// 🔢️ How many rows `fixture` holds under `key`.
fn count(fixture: &Value, key: &str) -> u64 {
    fixture.get(key).and_then(Value::as_array).map_or(0, |rows| rows.len() as u64)
}

/// 🎯️ `rows` with every `$region` id replaced by the id of the board's first target region.
fn addressed(rows: &Value, fixture: &Value) -> Value {
    let region = fixture.get("targetRegions").and_then(Value::as_array).and_then(|regions| regions.first()).and_then(|region| region.get("id")).cloned().unwrap_or(Value::Null);
    let mut rows = rows.clone();
    for row in rows.as_array_mut().into_iter().flatten() {
        if row["payload"]["id"] == "$region" {
            row["payload"]["id"] = region.clone();
        }
    }
    rows
}

/// 🧱️ LAW (design §22.32 a): every corpus case — one gesture is ONE transaction of its tool, a release that changes
/// nothing commits nothing, and the document after the flush holds what the corpus states.
#[test]
fn every_board_tool_release_is_one_transaction_of_its_tool() {
    let corpus = corpus();
    for case in corpus["cases"].as_array().expect("cases") {
        let name = case["name"].as_str().expect("case name");
        let mut app = select_tool_history_tests::seeded_app(&corpus["board"]);
        for rows in case["setup"].as_array().into_iter().flatten() {
            let rows = addressed(rows, &fixture_of(&app));
            assert_eq!(committed_edits(&flush(&mut app, &rows)), 1, "{name}: every setup flush is one edit");
        }
        let before = fixture_of(&app);
        let result = flush(&mut app, &addressed(&case["rows"], &before));
        let expect = &case["expect"];
        let rows = edit_rows(&result);
        assert_eq!(rows.len() as u64, expect["edits"].as_u64().expect("edits"), "{name}: history rows");
        assert_eq!(committed_edits(&result) as u64, expect["edits"].as_u64().expect("edits"), "{name}: committed edits");
        let after = fixture_of(&app);
        for key in ["nodes", "edges", "targetRegions"] {
            assert_eq!(count(&after, key), expect[key].as_u64().expect("count"), "{name}: {key}");
        }
        match expect["tool"].as_str() {
            Some(tool) => {
                let transaction = rows[0].transaction.as_ref().unwrap_or_else(|| panic!("{name}: the row is keyed by its tool transaction"));
                assert!(transaction.id.starts_with("tx-"), "{name}: {transaction:?}");
                assert_eq!(transaction.tool, format!("{}#{tool}", corpus["appTool"].as_str().expect("appTool")), "{name}: the tool that authored the row");
            }
            None => {
                assert_eq!(after, before, "{name}: zero trace leaves the document as it was");
                assert!(result.mutations.is_empty(), "{name}: no inline mutation either");
            }
        }
        if let Some(region) = expect.get("region") {
            for key in ["x", "y", "width", "height"] {
                assert_eq!(after["targetRegions"][0][key].as_f64(), region[key].as_f64(), "{name}: region {key}");
            }
        }
        close_app(&mut app);
    }
}

/// 🫥️ LAW (F24): a flush of transient kinds alone — a hover, a selection, a candidate page, nothing — upserts no
/// history row at all, however a host batches it: the guest never turns a flush without a document change into an edit.
#[test]
fn a_flush_of_transient_rows_upserts_no_history_row() {
    let corpus = corpus();
    let mut app = select_tool_history_tests::seeded_app(&corpus["board"]);
    for case in corpus["cases"].as_array().expect("cases").iter().filter(|case| case["transient"] == true) {
        let result = flush(&mut app, &case["rows"]);
        let upserts: Vec<(u64, bool, usize)> = result.history_patch.as_ref().map(|patch| patch.upserts.iter().map(|entry| (entry.seq, entry.applied, entry.op_lines.len())).collect()).unwrap_or_default();
        assert!(upserts.is_empty(), "{}: a transient flush is no history row: {upserts:?}", case["name"]);
        assert!(result.mutations.is_empty(), "{}: and no inline mutation", case["name"]);
    }
    close_app(&mut app);
}

/// 🗺️ LAW (F16): the `vortex` domain declares an app-supplied topology that names every id the board paints, in board
/// order — each node with its handles under it, each edge, each target region — which is what `selectAll` enumerates
/// and what keeps a selection inside the document.
#[test]
fn the_interaction_topology_names_every_board_entity() {
    let mut board = corpus()["board"].clone();
    puzzle2d_push_target_region(&mut board, 0.5, 0.5, 10.5, 10.5);
    puzzle2d_push_edge(&mut board, json!({ "id": "edge-left-mid", "source": "left:v0", "target": "mid:v0" }));
    let snapshot = Puzzle2dPlaySnapshot::new(board);
    let region = snapshot.typed().target_regions.first().map(|region| region.id.clone()).expect("the pushed region decodes");
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let config = Puzzle2dConfig::default();
    let topology = Puzzle2dPlayApp::interaction_topology(&doc, &ConfigView { snapshot: &config, window: None }).expect("the topology builds");
    let named: Vec<(&str, &str, Option<&str>)> = topology.domains[PUZZLE2D_INTERACTION_DOMAIN].ordered.iter().map(|node| (node.id.as_str(), node.granularity.as_str(), node.parent.as_deref())).collect();
    assert_eq!(named, vec![("left", "node", None), ("left:v0", "handle", Some("left")), ("mid", "node", None), ("mid:v0", "handle", Some("mid")), ("pin", "node", None), ("edge-left-mid", "edge", None), (region.as_str(), "node", None)]);
    assert_eq!(puzzle2d_interaction_definition().hierarchy, HierarchyProvider::Topology, "the domain declares that the app supplies it");
}
