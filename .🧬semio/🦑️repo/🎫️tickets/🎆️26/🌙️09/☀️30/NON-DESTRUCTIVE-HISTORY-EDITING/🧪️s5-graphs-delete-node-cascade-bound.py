"""🧱️ S5-GRAPHS-WIRES — the `delete-node` cascade-bound law (audit F3), run on the INDEPENDENT Python graph oracle.

The committed leaf payload schema is the single source of the bound: `x-semio-inverse-rows.bounded` is the undo's row
ceiling (the node's row plus one row per severed edge). This law reads that number from the schema and checks the oracle
(`🌳️mutate-semio-graph/🐍️.py`, written from the specification, never from the Rust subject) at the bound and one edge
above it. The Rust twin is `delete_node_refuses_one_edge_above_its_declared_cascade_bound` in the graph mutation unit
tests, which reads the same schema through the derived `MutationLeaf::inverse_rows`.

    python3 🧪️s5-graphs-delete-node-cascade-bound.py [--oracle <path to a 🐍️.py>]

`--oracle` checks a staged copy before it lands; the default is the oracle in the tree.
"""

import importlib.util
import json
import pathlib
import sys
import types

ROOT = pathlib.Path(__file__).resolve().parents[7]
GRAPH = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph"
SCHEMA = GRAPH / "🧬️schema/🧬️mutations/🗑️delete-node/🧬️schema/🔣️.json"
ORACLE = pathlib.Path(sys.argv[2]) if len(sys.argv) == 3 and sys.argv[1] == "--oracle" else GRAPH / "🧪️tests/🌳️mutate-semio-graph/🐍️.py"
if len(sys.argv) not in (1, 3) or not ORACLE.is_file() or not SCHEMA.is_file():
    raise SystemExit("usage: 🧪️s5-graphs-delete-node-cascade-bound.py [--oracle <path>] (oracle and leaf schema must exist)")

stub = types.ModuleType("semio_repo_test")
for name in ("Adapter", "Context", "Outcome", "digest", "patched_snapshot", "snapshot_patch_inverse"):
    setattr(stub, name, type(name, (), {}))
sys.modules["semio_repo_test"] = stub
spec = importlib.util.spec_from_file_location("graph_oracle", ORACLE)
oracle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(oracle)

declared = json.loads(SCHEMA.read_text())["x-semio-inverse-rows"]["bounded"]
maximum = declared - 1
word = {"bits": "0000000000000000"}


def node(identifier: str) -> dict:
    return {"id": {"value": identifier}, "kind": "k", "label": identifier, "position": {"x": dict(word), "y": dict(word)}, "width": dict(word), "height": dict(word), "ports": [], "properties": []}


def star(degree: int) -> dict:
    """⭐️ A hub and a rim joined by `degree` edges of alternating direction, so both endpoints of the hub are exercised."""
    edges = []
    for index in range(degree):
        source, target = ("hub", "rim") if index % 2 == 0 else ("rim", "hub")
        edges.append({"id": {"value": "e%d" % index}, "source": {"value": source}, "target": {"value": target}, "kind": "flow", "label": "", "properties": []})
    return {"schema": oracle.DOCUMENT_SCHEMA, "nodes": [node("hub"), node("rim")], "edges": edges}


delete = {"DeleteNode": {"id": {"value": "hub"}}}
passed, failed = 0, []


def law(name: str, check) -> None:
    global passed
    try:
        check()
        passed += 1
    except Exception as error:
        failed.append("%s: %s" % (name, str(error)[:300]))


def refuses(action) -> str:
    try:
        action()
    except AssertionError as refusal:
        return str(refusal)
    raise Exception("the oracle applied a delete it must refuse")


def the_oracle_bound_is_the_declared_rows_less_the_node_row() -> None:
    assert oracle.DELETE_NODE_CASCADE_EDGES_MAXIMUM == maximum, "oracle bound %r, schema declares %d rows" % (oracle.DELETE_NODE_CASCADE_EDGES_MAXIMUM, declared)


def at_the_bound_the_delete_applies_and_its_undo_is_the_declared_rows() -> None:
    document = star(maximum)
    mutated = oracle.apply_mutation(document, delete)
    assert [entry["id"]["value"] for entry in mutated["nodes"]] == ["rim"] and mutated["edges"] == [], "the hub and every incident edge must be gone"
    undo = oracle.inverse_mutation(document, delete)
    assert len(undo) == declared, "undo has %d rows, the schema declares %d" % (len(undo), declared)
    assert oracle.apply_all(mutated, undo) == document, "the undo did not restore the star"


def one_edge_above_the_bound_is_refused_and_nothing_changes() -> None:
    document = star(maximum + 1)
    untouched = json.dumps(document, sort_keys=True)
    for action in (lambda: oracle.apply_mutation(document, delete), lambda: oracle.inverse_mutation(document, delete)):
        message = refuses(action)
        assert str(maximum + 1) in message and str(maximum) in message, "the refusal must name the degree and the bound: %s" % message
    assert json.dumps(document, sort_keys=True) == untouched, "a refused delete must leave the document untouched"


def an_unrelated_node_of_the_dense_graph_still_deletes() -> None:
    document = star(maximum + 1)
    document["nodes"].append(node("lone"))
    mutated = oracle.apply_mutation(document, {"DeleteNode": {"id": {"value": "lone"}}})
    assert len(mutated["edges"]) == maximum + 1 and len(mutated["nodes"]) == 2, "the bound is per deleted node, not per graph"


for check in (the_oracle_bound_is_the_declared_rows_less_the_node_row, at_the_bound_the_delete_applies_and_its_undo_is_the_declared_rows, one_edge_above_the_bound_is_refused_and_nothing_changes, an_unrelated_node_of_the_dense_graph_still_deletes):
    law(check.__name__, check)
print("[delete-node-cascade-bound] oracle=%s declared=%d maximum=%d passed=%d failed=%d" % (ORACLE.name if ORACLE.parent.name != "f3" else "f3/" + ORACLE.name, declared, maximum, passed, len(failed)))
for entry in failed:
    print("  FAIL " + entry)
sys.exit(1 if failed else 0)
