"""🧱️ S5-GRAPHS-WIRES — audit F3 wave for the shared graph child vocabulary (`s.stdio.semio@v1/graph`, `delete-node`).

`delete-node` declares `x-semio-inverse-rows: {"bounded": 1025}` and its undo is one row for the node plus one row per
severed edge, so a node with more than 1024 incident edges outgrows the declaration. The wave makes the leaf refuse that
degree with the frozen outcome code `mutation.target-referenced` (the outcome vocabulary is closed, `mutation.too-large`
is a store fault, not an outcome), keeps the undo empty for a refused delete, adds the bound + 1 law (Rust) and the same
refusal in the independent Python oracle.

The four targets live inside the puzzle activation closure (fleet rule 51), so the wave is prepared here and landed
under the landing lock:

    python3 🧪️s5-graphs-f3-stage.py --stage   # derive every `*.after.*` under 🗑️generated/s5-graphs-wires/f3/ (no tree write)
    python3 🧪️s5-graphs-f3-stage.py --land    # write the four targets; refuses when a target moved since staging
    python3 🧪️s5-graphs-f3-stage.py --revert  # put the four `*.before.*` copies back (rule 51: not green in time)

Fails closed: every target is an explicit path, every anchor must occur exactly once, and `--land` compares each live
file with its recorded `*.before.*` copy byte for byte before it writes anything.
"""

import pathlib
import sys

TICKET = pathlib.Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
STAGE = TICKET / "🗑️generated/s5-graphs-wires/f3"
GRAPH = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph"
LEAF = GRAPH / "🧬️schema/🧬️mutations/🗑️delete-node"
TARGETS = {
    "diff": (LEAF / "🔺️diff/🦀️.rs", "rs"),
    "inverse": (LEAF / "↩️inverse/🦀️.rs", "rs"),
    "unit": (GRAPH / "🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs", "rs"),
    "oracle": (GRAPH / "🧪️tests/🌳️mutate-semio-graph/🐍️.py", "py"),
}

UNIT_ANCHOR = """#[semio_framework_async_macros::async_test]
async fn delete_node_inverse_is_a_real_multi_mutation_cascade() {"""
UNIT_LAW = """/// 🧱️ LAW (audit F3): `delete-node` severs at most the edges its schema-declared inverse rows cover. AT the bound the delete
/// applies and its undo is exactly the declared rows, restoring the snapshot; ONE edge above it the delete is
/// `mutation.target-referenced`, changes nothing and has nothing to undo — the recorded rows never exceed the footprint the
/// store admitted from the declaration.
#[semio_framework_async_macros::async_test]
async fn delete_node_refuses_one_edge_above_its_declared_cascade_bound() {
    let hub = delete_node::DeleteNode { id: GraphNodeId::new("hub") };
    let declared = protocol::MutationLeaf::inverse_rows(&hub);
    let maximum = delete_node::diff::cascade_edges_maximum(&hub);
    assert_eq!(maximum + 1, declared, "the cascade bound is the declared inverse rows less the row that restores the node");
    let node = |id: &str| SemioGraphNode { id: GraphNodeId::new(id), kind: "k".into(), label: id.into(), position: SemioPoint2 { x: 0.0, y: 0.0 }, width: 0.0, height: 0.0, ports: vec![], properties: vec![] };
    let star = |degree: usize| SemioGraphSnapshot {
        nodes: vec![node("hub"), node("rim")],
        edges: (0..degree).map(|index| { let (source, target) = if index % 2 == 0 { ("hub", "rim") } else { ("rim", "hub") }; SemioGraphEdge { id: GraphEdgeId::new(format!("e{index}")), source: GraphNodeId::new(source), target: GraphNodeId::new(target), kind: "flow".into(), label: String::new(), source_port: None, target_port: None, properties: Vec::new() } }).collect(),
        ..Default::default()
    };
    let delete = SemioGraphMutation::DeleteNode(hub);
    assert_eq!(<SemioGraphMutation as Mutation<SemioGraphSnapshot>>::inverse_rows(&delete), declared, "the aggregate answers the leaf's declared rows");

    let at_bound = star(maximum);
    let applied = delete.diff(&at_bound);
    assert_eq!(applied.messages().iter().map(|message| message.code.0.as_str()).collect::<Vec<_>>(), vec!["mutation.cascade"], "at the bound the delete applies with its cascade note only");
    let undo = delete.inverse(&at_bound).expect("delete-node inverse at the bound");
    assert_eq!(undo.len(), declared, "at the bound the undo is exactly the declared rows");
    let mut restored = applied.diff().apply(&at_bound).expect("the delete at the bound applies");
    assert_eq!((restored.nodes.len(), restored.edges.len()), (1, 0), "the hub and every incident edge are gone");
    for back in &undo {
        restored = back.diff(&restored).diff().apply(&restored).expect("each undo row applies");
    }
    assert_eq!(restored, at_bound, "the undo at the bound restores the exact snapshot");

    let above = star(maximum + 1);
    let refused = delete.diff(&above);
    let messages = refused.messages();
    assert_eq!(messages.len(), 1, "one edge above the bound is exactly one refusal");
    assert_eq!((messages[0].code.0.as_str(), messages[0].level, messages[0].target.clone()), ("mutation.target-referenced", semio_framework_diagnostic::Severity::Error, vec!["hub".to_string()]), "the refusal names the node with the frozen outcome code");
    assert_eq!(refused.diff().apply(&above).expect("a refused delete carries the empty diff"), above, "a refused delete changes nothing");
    assert!(delete.inverse(&above).expect("delete-node inverse above the bound").is_empty(), "a refused delete has nothing to undo");
}

"""

ORACLE_CONSTANT_ANCHOR = """#: 🏷️ The externally tagged JSON name of each kebab-case kind, as the committed specification
#: vectors under `…/🧬️mutations/<kind>/🧪️tests/<fixture>/🦠️mutation/` spell it."""
ORACLE_CONSTANT = """#: 🧱️ The incident edges ONE `delete-node` severs at most. The committed leaf payload schema
#: `…/🧬️mutations/🗑️delete-node/🧬️schema/🔣️.json` declares `x-semio-inverse-rows: {"bounded": 1025}`: the undo is the
#: row that restores the node plus one row per severed edge, so the 1025th incident edge makes the delete a refusal
#: (`mutation.target-referenced`) that changes nothing and has nothing to undo.
DELETE_NODE_CASCADE_EDGES_MAXIMUM = 1024

"""
ORACLE_HELPER_ANCHOR = """def apply_mutation(document: dict, mutation: dict) -> dict:"""
ORACLE_HELPER = """def severed_edges(document: dict, node_id: dict, verb: str) -> list:
    \"\"\"✂️ `(index, edge)` of every edge a `delete-node` of `node_id` severs, in document order. More than
    `DELETE_NODE_CASCADE_EDGES_MAXIMUM` of them is a refusal, never a partial cascade.\"\"\"
    severed = [(at, edge) for at, edge in enumerate(document["edges"]) if edge["source"] == node_id or edge["target"] == node_id]
    if len(severed) > DELETE_NODE_CASCADE_EDGES_MAXIMUM:
        raise AssertionError("%s addresses node %r, which %d edges still reference — one delete severs at most %d" % (verb, node_id, len(severed), DELETE_NODE_CASCADE_EDGES_MAXIMUM))
    return severed


"""
ORACLE_APPLY_BEFORE = """    elif tag == "DeleteNode":
        node_at(result, args["id"], tag)
        result["nodes"] = [node for node in result["nodes"] if node["id"] != args["id"]]
        result["edges"] = [edge for edge in result["edges"] if edge["source"] != args["id"] and edge["target"] != args["id"]]
"""
ORACLE_APPLY_AFTER = """    elif tag == "DeleteNode":
        node_at(result, args["id"], tag)
        severed = {at for at, _ in severed_edges(result, args["id"], tag)}
        result["nodes"] = [node for node in result["nodes"] if node["id"] != args["id"]]
        result["edges"] = [edge for at, edge in enumerate(result["edges"]) if at not in severed]
"""
ORACLE_INVERSE_BEFORE = """        steps.extend(edge_mutation(edge, at) for at, edge in enumerate(document["edges"]) if edge["source"] == args["id"] or edge["target"] == args["id"])
"""
ORACLE_INVERSE_AFTER = """        steps.extend(edge_mutation(edge, at) for at, edge in severed_edges(document, args["id"], tag))
"""


def once(text: str, anchor: str, what: str) -> int:
    count = text.count(anchor)
    if count != 1:
        raise SystemExit("[f3] REFUSED: anchor %s occurs %d times, expected exactly 1" % (what, count))
    return text.index(anchor)


def insert_before(text: str, anchor: str, addition: str, what: str) -> str:
    at = once(text, anchor, what)
    return text[:at] + addition + text[at:]


def replace_once(text: str, before: str, after: str, what: str) -> str:
    once(text, before, what)
    return text.replace(before, after)


def derive() -> dict:
    """🧮️ Every target's intended content, derived from its recorded `*.before.*` copy."""
    before = {name: (STAGE / ("%s.before.%s" % (name, extension))).read_text() for name, (_, extension) in TARGETS.items()}
    unit = insert_before(before["unit"], UNIT_ANCHOR, UNIT_LAW, "unit law position")
    oracle = insert_before(before["oracle"], ORACLE_CONSTANT_ANCHOR, ORACLE_CONSTANT, "oracle constant position")
    oracle = insert_before(oracle, ORACLE_HELPER_ANCHOR, ORACLE_HELPER, "oracle helper position")
    oracle = replace_once(oracle, ORACLE_APPLY_BEFORE, ORACLE_APPLY_AFTER, "oracle DeleteNode apply arm")
    oracle = replace_once(oracle, ORACLE_INVERSE_BEFORE, ORACLE_INVERSE_AFTER, "oracle DeleteNode inverse arm")
    return {"diff": (STAGE / "diff.after.rs").read_text(), "inverse": (STAGE / "inverse.after.rs").read_text(), "unit": unit, "oracle": oracle}


def main() -> None:
    mode = sys.argv[1] if len(sys.argv) == 2 else ""
    if mode not in ("--stage", "--land", "--revert"):
        raise SystemExit("usage: 🧪️s5-graphs-f3-stage.py --stage | --land | --revert")
    if not STAGE.is_dir() or not GRAPH.is_dir():
        raise SystemExit("[f3] REFUSED: stage or graph root is missing")
    after = derive()
    if mode == "--stage":
        for name, (_, extension) in TARGETS.items():
            (STAGE / ("%s.after.%s" % (name, extension))).write_text(after[name])
        print("[f3] staged %d targets under %s" % (len(TARGETS), STAGE))
        return
    recorded = {name: (STAGE / ("%s.before.%s" % (name, extension))).read_text() for name, (_, extension) in TARGETS.items()}
    expected = recorded if mode == "--land" else after
    written = after if mode == "--land" else recorded
    moved = [name for name, (path, _) in TARGETS.items() if path.read_text() != expected[name]]
    if moved:
        raise SystemExit("[f3] REFUSED: %s changed in the tree since staging — re-stage from the live files" % ", ".join(moved))
    for name, (path, _) in TARGETS.items():
        path.write_text(written[name])
    print("[f3] %s %d targets" % ("landed" if mode == "--land" else "reverted", len(TARGETS)))


main()
