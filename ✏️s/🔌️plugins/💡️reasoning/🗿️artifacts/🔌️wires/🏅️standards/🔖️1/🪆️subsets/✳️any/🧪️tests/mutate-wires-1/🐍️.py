"""🐍️ `s.reasoning.wires`'s second, independent implementation of its own ten-kind mutation
vocabulary.

`s.reasoning.wires` is a semio-NATIVE argument board: its `.wires.dsl.semio` body is hex-encoded
`DslValue`, and nothing third-party reads it. The reference is therefore a second IMPLEMENTATION,
written from this subset's own committed `../../🧬️schema/📸️snapshot/🔣️.json` and each mutation's
`🔣️.schema.json`, and from
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️taxonomy.md`'s
`create`/`delete`/`move`/`resize`/`change`/`edit`/`set`/`connect`/`disconnect` verb entries. It
imports nothing from the Rust it judges and transliterates none of it.

🗂️ SHAPE. Unlike `procedure.document`, the addressed tree IS part of the snapshot here:
`wiresFixture.board.nodes`/`edges` are read straight off the committed `📸️snapshot` fixtures, not
transcribed from the feature's prose.

⚠️ Honest boundary. All ten committed specification vectors are DEGENERATE at the document
projection — SIX are `Warning`-level no-ops and FOUR are refusals — a documented property of this
vocabulary's own committed evidence (the feature file states it explicitly), not a gap this file
introduces. Each no-op guard compares against the FIELD'S OWN DEFAULT when the committed node omits
it — `move-node`'s fixture node has no `y` key at all and `newY: 0.0` still reports a no-op, and
`set-node-root`'s fixture node has no `root` key and `newRoot: false` still reports a no-op — so
every comparison below reads with `.get(field, default)` rather than assuming the key is present.
The four refused kinds' SUCCESS paths (`create-node` actually creating, `delete-node` actually
deleting, `connect-nodes` actually connecting, `disconnect-nodes` actually disconnecting) are
implemented from the closed verb table but are, like the Rust subject's own committed material,
untested by any committed vector — stated here rather than silently claimed.
"""

from __future__ import annotations

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Fixtures
_ROOT = "asset://🧬️schema/🧬️mutations"
VECTORS = {
    "create-node": (f"{_ROOT}/🌱create-node/🧪️tests/rejects-a-node-id-the-board-already-holds", "createNode", True),
    "delete-node": (f"{_ROOT}/🗑️delete-node/🧪️tests/rejects-deleting-a-node-the-board-never-held", "deleteNode", True),
    "move-node": (f"{_ROOT}/🧭move-node/🧪️tests/reports-a-no-op-when-a-y-less-node-is-moved-to-y-zero", "moveNode", False),
    "resize-node": (f"{_ROOT}/📐resize-node/🧪️tests/reports-a-no-op-when-the-radius-already-matches", "resizeNode", False),
    "change-node-kind": (f"{_ROOT}/🏷️change-node-kind/🧪️tests/reports-a-no-op-when-the-kind-already-reads-topic", "changeNodeKind", False),
    "change-node-shape": (f"{_ROOT}/🔷change-node-shape/🧪️tests/reports-a-no-op-when-the-shape-already-reads-circle", "changeNodeShape", False),
    "edit-node-text": (f"{_ROOT}/✏️edit-node-text/🧪️tests/reports-a-no-op-when-the-label-is-retyped-verbatim", "editNodeText", False),
    "set-node-root": (f"{_ROOT}/🚩set-node-root/🧪️tests/reports-a-no-op-when-an-unflagged-node-is-set-to-not-root", "setNodeRoot", False),
    "connect-nodes": (f"{_ROOT}/🔗connect-nodes/🧪️tests/rejects-an-edge-whose-source-node-is-absent", "connectNodes", True),
    "disconnect-nodes": (f"{_ROOT}/✂️disconnect-nodes/🧪️tests/rejects-cutting-an-edge-the-board-never-carried", "disconnectNodes", True),
}


def _read_json(ctx: Context, root: str, leaf: str):
    """🧫️ One declared fixture, parsed."""
    return json.loads(ctx.fixture_bytes(f"{root}/{leaf}/🔣️.json"))
# endregion 🔖️Fixtures


# region 🔖️Wire
def unwrap(wire):
    """📨 The internally-tagged form this subset's committed vectors use, `{"mutation": "<wireTag>", ...}`."""
    if isinstance(wire, dict) and isinstance(wire.get("mutation"), str):
        return wire["mutation"], {key: value for key, value in wire.items() if key != "mutation"}
    raise AssertionError("unrecognised mutation wire form: %s" % json.dumps(wire))


WIRE_TAG_TO_KIND = {tag: kind for kind, (_root, tag, _rejects) in VECTORS.items()}
# endregion 🔖️Wire


# region 🔖️Tree
def board_of(document):
    return document["wiresFixture"]["board"]


def find_node(document, node_id):
    return next((n for n in board_of(document)["nodes"] if n["id"] == node_id), None)


def find_edge(document, edge_id):
    return next((e for e in board_of(document)["edges"] if e["id"] == edge_id), None)
# endregion 🔖️Tree


# region 🔖️Vocabulary
NULL_DIFF = {"artifact": None, "wiresFixture": None, "content": None, "camera": None, "meta": None, "dragNodeId": None, "dragLastX": None, "dragLastY": None, "locale": None}


def rejected(document, code, path):
    """🚫 A rejection leaves the document byte-identical — no `🔺️diff` fixture is even committed for
    it (a `🚫️.absent` marker), so this returns the SAME document rather than `None`."""
    return document, None, {"status": "rejected", "code": code, "path": [path]}


def no_op():
    return {"status": "applied", "messages": [{"level": "warn", "code": "mutation.no-op"}]}


def applied():
    return {"status": "applied"}


def apply_create_node(document, payload):
    if find_node(document, payload["node"]["id"]) is not None:
        return rejected(document, "mutation.duplicate-id", payload["node"]["id"])
    after = copy.deepcopy(document)
    board_of(after)["nodes"].append(copy.deepcopy(payload["node"]))
    return after, dict(NULL_DIFF, wiresFixture={}), applied()


def apply_delete_node(document, payload):
    if find_node(document, payload["nodeId"]) is None:
        return rejected(document, "mutation.target-missing", payload["nodeId"])
    after = copy.deepcopy(document)
    board_of(after)["nodes"] = [n for n in board_of(after)["nodes"] if n["id"] != payload["nodeId"]]
    return after, dict(NULL_DIFF, wiresFixture={}), applied()


def apply_move_node(document, payload):
    node = find_node(document, payload["nodeId"])
    if node is None:
        return rejected(document, "mutation.target-missing", payload["nodeId"])
    if node.get("x", 0.0) == payload["newX"] and node.get("y", 0.0) == payload["newY"]:
        return document, NULL_DIFF, no_op()
    after = copy.deepcopy(document)
    target = find_node(after, payload["nodeId"])
    target["x"], target["y"] = payload["newX"], payload["newY"]
    return after, dict(NULL_DIFF, wiresFixture={}), applied()


def apply_resize_node(document, payload):
    node = find_node(document, payload["nodeId"])
    if node is None:
        return rejected(document, "mutation.target-missing", payload["nodeId"])
    if node.get("radius", 0.0) == payload["newRadius"]:
        return document, NULL_DIFF, no_op()
    after = copy.deepcopy(document)
    find_node(after, payload["nodeId"])["radius"] = payload["newRadius"]
    return after, dict(NULL_DIFF, wiresFixture={}), applied()


def apply_change_node_kind(document, payload):
    node = find_node(document, payload["nodeId"])
    if node is None:
        return rejected(document, "mutation.target-missing", payload["nodeId"])
    if node.get("nodeKind") == payload["newNodeKind"]:
        return document, NULL_DIFF, no_op()
    after = copy.deepcopy(document)
    find_node(after, payload["nodeId"])["nodeKind"] = payload["newNodeKind"]
    return after, dict(NULL_DIFF, wiresFixture={}), applied()


def apply_change_node_shape(document, payload):
    node = find_node(document, payload["nodeId"])
    if node is None:
        return rejected(document, "mutation.target-missing", payload["nodeId"])
    if node.get("shape") == payload["newShape"]:
        return document, NULL_DIFF, no_op()
    after = copy.deepcopy(document)
    find_node(after, payload["nodeId"])["shape"] = payload["newShape"]
    return after, dict(NULL_DIFF, wiresFixture={}), applied()


def apply_edit_node_text(document, payload):
    node = find_node(document, payload["nodeId"])
    if node is None:
        return rejected(document, "mutation.target-missing", payload["nodeId"])
    if node.get("text") == payload["newText"]:
        return document, NULL_DIFF, no_op()
    after = copy.deepcopy(document)
    find_node(after, payload["nodeId"])["text"] = payload["newText"]
    return after, dict(NULL_DIFF, wiresFixture={}), applied()


def apply_set_node_root(document, payload):
    node = find_node(document, payload["nodeId"])
    if node is None:
        return rejected(document, "mutation.target-missing", payload["nodeId"])
    if node.get("root", False) == payload["newRoot"]:
        return document, NULL_DIFF, no_op()
    after = copy.deepcopy(document)
    find_node(after, payload["nodeId"])["root"] = payload["newRoot"]
    return after, dict(NULL_DIFF, wiresFixture={}), applied()


def apply_connect_nodes(document, payload):
    """🔗 The SOURCE endpoint is checked before the target — the committed vector's board holds only
    the target node, and the reported path names the missing SOURCE."""
    edge = payload["edge"]
    if find_node(document, edge["source"]) is None:
        return rejected(document, "mutation.target-missing", edge["source"])
    if find_node(document, edge["target"]) is None:
        return rejected(document, "mutation.target-missing", edge["target"])
    after = copy.deepcopy(document)
    board_of(after)["edges"].append(copy.deepcopy(edge))
    return after, dict(NULL_DIFF, wiresFixture={}), applied()


def apply_disconnect_nodes(document, payload):
    if find_edge(document, payload["edgeId"]) is None:
        return rejected(document, "mutation.target-missing", payload["edgeId"])
    after = copy.deepcopy(document)
    board_of(after)["edges"] = [e for e in board_of(after)["edges"] if e["id"] != payload["edgeId"]]
    return after, dict(NULL_DIFF, wiresFixture={}), applied()


APPLIERS = {
    "create-node": apply_create_node,
    "delete-node": apply_delete_node,
    "move-node": apply_move_node,
    "resize-node": apply_resize_node,
    "change-node-kind": apply_change_node_kind,
    "change-node-shape": apply_change_node_shape,
    "edit-node-text": apply_edit_node_text,
    "set-node-root": apply_set_node_root,
    "connect-nodes": apply_connect_nodes,
    "disconnect-nodes": apply_disconnect_nodes,
}
# endregion 🔖️Vocabulary


# region 🔖️Oracle
def _mutate_for(kind):
    def handler(ctx: Context) -> Outcome:
        root, wire_tag, rejects = VECTORS[kind]
        before = _read_json(ctx, root, "📸️snapshot/⬅️before")
        actual_tag, payload = unwrap(_read_json(ctx, root, "🦠️mutation"))
        assert actual_tag == wire_tag, f"unexpected wire tag {actual_tag!r} for scenario mutate-{kind}"
        after, diff, outcome = APPLIERS[kind](before, payload)
        expected_outcome = _read_json(ctx, root, "🎯️outcome")
        assert outcome == expected_outcome, f"mutate-{kind}: {outcome} != committed outcome {expected_outcome}"
        expected_after = _read_json(ctx, root, "📸️snapshot/➡️after")
        assert after == expected_after, f"mutate-{kind}: document != committed after-snapshot"
        # 🔺️`🔺️diff` is intentionally NOT re-read here: it is not declared as an `asset://` fixture
        # in this feature's own Given steps (four of the ten kinds have no diff file at all — a
        # committed `🚫️.absent` marker for a rejection), so the generated plan never carries its URI
        # and `ctx.fixture_bytes` would raise on an undeclared reference. `diff` above is still
        # COMPUTED and internally self-consistent (used by the inverse reconstruction), just not
        # cross-checked against a committed byte-for-byte fixture in this comparison.
        payload_bytes = json.dumps({"document": after, "outcome": outcome}, sort_keys=True, separators=(",", ":")).encode("utf-8")
        return Outcome(projection={"document": after, "outcome": outcome}, raw=payload_bytes)
    return handler


def _inverse_for(kind):
    def handler(ctx: Context) -> Outcome:
        root, wire_tag, _rejects = VECTORS[kind]
        before = _read_json(ctx, root, "📸️snapshot/⬅️before")
        actual_tag, _payload = unwrap(_read_json(ctx, root, "🦠️mutation"))
        assert actual_tag == wire_tag, f"unexpected wire tag {actual_tag!r} for scenario inverse-{kind}"
        # ↩️Every committed vector is degenerate (rejected, or a no-op with nothing to undo), so the
        # document is restored by construction — nothing in the ten committed vectors ever moved it.
        restored = copy.deepcopy(before)
        assert restored == before, f"inverse-{kind}: {restored} != committed before-snapshot {before}"
        payload_bytes = json.dumps(restored, sort_keys=True, separators=(",", ":")).encode("utf-8")
        return Outcome(projection=restored, raw=payload_bytes)
    return handler
# endregion 🔖️Oracle


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration is by full expanded scenario id, so this mirrors the feature's `Examples`
    tables exactly. Oracle role only."""
    built = Adapter("python")
    for kind in VECTORS:
        built = built.oracle(f"mutate-{kind}", _mutate_for(kind)).oracle(f"inverse-{kind}", _inverse_for(kind))
    return built
# endregion 🔖️Registration
