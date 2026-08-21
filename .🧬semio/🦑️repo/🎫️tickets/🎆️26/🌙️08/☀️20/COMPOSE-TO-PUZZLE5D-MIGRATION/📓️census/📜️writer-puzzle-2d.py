#!/usr/bin/env python3
"""Writer for the handcrafted puzzle ◻2d mutation fixtures (ticket 26/08/20).

Every case's before/after/mutation/diff/outcome payload and every assertion string below is
hand-authored from that mutation's own 🔺️diff/🦀️component.rs. This file is only the transcriber
that lays the bytes down on disk; it contains no per-mutation logic.
"""
import copy
import json
import os
import textwrap

ROOT = "/Users/ueli/Documents/semio"
TREE = os.path.join(
    ROOT,
    "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
)

DIFF_FIELDS = [
    "artifact", "schema", "camera", "nodes", "edges", "meta", "selectedIds", "activeUtilityId",
    "cameraX", "cameraY", "cameraZoom", "selectionMethod", "gridSnapEnabled", "gridFactor",
    "suggestionOffset", "fillCount", "brushCandidateIndex", "brushCandidateSourceHandleId",
    "locale", "terminology", "lodModeByPaneJson", "engagementInputByPaneJson",
    "brushCandidatesJson", "nodeKindWeightsJson", "handleKindWeightsJson",
    "activeUtilityByWindowIdJson", "hoveredNodeId", "previewSeq",
]


def diff(**set_fields):
    out = {}
    for name in DIFF_FIELDS:
        out[name] = set_fields.get(name)
    return out


def nodes_delta(added=None, removed=None, patched=None, reordered=None):
    return {
        "added": added or [],
        "removed": removed or [],
        "patched": patched or [],
        "reordered": reordered,
    }


def edges_delta(added=None, removed=None, patched=None, reordered=None):
    return {
        "added": added or [],
        "removed": removed or [],
        "patched": patched or [],
        "reordered": reordered,
    }


def patch(entry_id, replacement):
    return [{"id": entry_id, "patch": {"replacement": replacement}}]


NODE_A = {
    "id": "node-a",
    "nodeKind": "node-kind-a",
    "shape": "circle",
    "x": 0.0,
    "y": 0.0,
    "radius": 10.0,
    "text": "Alpha",
    "iconKind": "icon-alpha",
    "anchor": "fixed",
    "handles": [
        {"id": "handle-1", "handleKind": "handle-kind-a", "angle": 0.0},
        {"id": "handle-spare", "handleKind": "handle-kind-a", "angle": 1.5},
    ],
}

NODE_B = {
    "id": "node-b",
    "nodeKind": "node-kind-b",
    "shape": "rectangle",
    "x": 40.0,
    "y": 20.0,
    "width": 30.0,
    "height": 12.0,
    "anchor": "fixed",
    "handles": [
        {"id": "handle-2", "handleKind": "handle-kind-b", "angle": 3.0},
    ],
}

EDGE_1 = {
    "id": "edge-1",
    "source": "handle-1",
    "target": "handle-2",
    "edgeKind": "edge-kind-a",
    "gap": 1.0,
    "shift": 0.0,
    "rise": 0.0,
    "rotation": 0.0,
    "turn": 0.0,
    "tilt": 0.0,
    "x": 0.0,
    "y": 0.0,
    "sourceTip": "none",
    "targetTip": "arrow",
}

COMPAT_AB = {
    "source": "handle-kind-a",
    "target": "handle-kind-b",
    "bidirectional": True,
    "important": False,
    "specificity": "handle",
}

BASE = {
    "schema": "puzzle.2d.fixture",
    "camera": {"x": 0.0, "y": 0.0, "zoom": 1.0},
    "nodes": [NODE_A, NODE_B],
    "edges": [EDGE_1],
    "meta": {"manifestId": "manifest-alpha", "kindCompatibility": [COMPAT_AB]},
}

APPLIED = {"status": "applied"}
APPLIED_NOOP = {
    "status": "applied",
    "messages": [{"level": "warn", "code": "mutation.no-op"}],
}


def base():
    return copy.deepcopy(BASE)


def with_node_a(**edits):
    node = copy.deepcopy(NODE_A)
    for key, value in edits.items():
        if value is None and key in node:
            del node[key]
        elif value is not None:
            node[key] = value
    return node


def with_edge_1(**edits):
    edge = copy.deepcopy(EDGE_1)
    for key, value in edits.items():
        if value is None and key in edge:
            del edge[key]
        elif value is not None:
            edge[key] = value
    return edge


def snapshot_with_node_a(node_a):
    snap = base()
    snap["nodes"][0] = node_a
    return snap


def snapshot_with_edge_1(edge_1):
    snap = base()
    snap["edges"][0] = edge_1
    return snap


CASES = []


def case(leaf, name, summary, mutation, diff_json, after, outcome, after_asserts, diff_asserts,
         before=None):
    CASES.append({
        "leaf": leaf,
        "name": name,
        "summary": summary,
        "mutation": mutation,
        "diff": diff_json,
        "before": before if before is not None else base(),
        "after": after,
        "outcome": outcome,
        "after_asserts": after_asserts,
        "diff_asserts": diff_asserts,
    })


# ─── 🌱create-node ────────────────────────────────────────────────────────────────────────────
NODE_C = {
    "id": "node-c",
    "nodeKind": "node-kind-c",
    "shape": "circle",
    "x": 80.0,
    "y": 0.0,
    "radius": 6.0,
    "text": "Gamma",
    "anchor": "fixed",
    "handles": [{"id": "handle-3", "handleKind": "handle-kind-a", "angle": 0.0}],
}
_after = base()
_after["nodes"].append(copy.deepcopy(NODE_C))
case(
    "🌱create-node", "appends-node-c",
    "A brand-new `node-c` is appended to `nodes`; `index: null` means the builder emits no "
    "`reordered` order at all, so the node lands at the end.",
    {"mutation": "createNode", "node": copy.deepcopy(NODE_C), "index": None},
    diff(nodes=nodes_delta(added=[copy.deepcopy(NODE_C)])),
    _after, APPLIED,
    [
        'assert_eq!(snapshot.nodes.len(), 3, "create-node/appends-node-c: node-c was not appended to the nodes collection");',
        'assert_eq!(snapshot.nodes[2].id, "node-c", "create-node/appends-node-c: a null index must append, never insert");',
        'assert_eq!(snapshot.edges, before().edges, "create-node/appends-node-c: creating a node must not touch any edge");',
    ],
    [
        'assert_eq!(committed["nodes"]["added"][0]["id"].as_str(), Some("node-c"), "create-node/appends-node-c: the diff must carry node-c in nodes.added");',
        'assert!(committed["nodes"]["reordered"].is_null(), "create-node/appends-node-c: a null index must leave reordered unset");',
        'assert!(committed["edges"].is_null(), "create-node/appends-node-c: create-node must never touch the edges delta");',
    ],
)

# ─── 🗑delete-node ────────────────────────────────────────────────────────────────────────────
_after = base()
_after["nodes"] = [copy.deepcopy(NODE_B)]
_after["edges"] = []
case(
    "🗑delete-node", "removes-node-a-and-severs-edge",
    "Deleting `node-a` cascades: `edge-1` hangs off `handle-1`, one of node-a's own handles, so the "
    "builder removes it from `edges` in the very same diff.",
    {"mutation": "deleteNode", "id": "node-a"},
    diff(nodes=nodes_delta(removed=["node-a"]), edges=edges_delta(removed=["edge-1"])),
    _after, APPLIED,
    [
        'assert!(!snapshot.nodes.iter().any(|node| node.id == "node-a"), "delete-node/removes-node-a-and-severs-edge: node-a survived the delete");',
        'assert!(snapshot.edges.is_empty(), "delete-node/removes-node-a-and-severs-edge: edge-1 hangs off handle-1 and must be severed with the node");',
    ],
    [
        'assert_eq!(committed["nodes"]["removed"][0].as_str(), Some("node-a"), "delete-node/removes-node-a-and-severs-edge: the diff must remove node-a by id");',
        'assert_eq!(committed["edges"]["removed"][0].as_str(), Some("edge-1"), "delete-node/removes-node-a-and-severs-edge: the severed edge must be a removal, not a rewrite");',
        'assert!(committed["nodes"]["patched"].as_array().map(Vec::is_empty).unwrap_or(false), "delete-node/removes-node-a-and-severs-edge: a delete must patch nothing");',
    ],
)

# ─── 📍move-node ──────────────────────────────────────────────────────────────────────────────
_node = with_node_a(x=5.0, y=7.0)
case(
    "📍move-node", "moves-node-a",
    "An absolute reposition of `node-a` to (5, 7). Only `x`/`y` change — the builder clones the "
    "node and rewrites exactly those two fields.",
    {"mutation": "moveNode", "id": "node-a", "newX": 5.0, "newY": 7.0},
    diff(nodes=nodes_delta(patched=patch("node-a", copy.deepcopy(_node)))),
    snapshot_with_node_a(_node), APPLIED,
    [
        'let node = snapshot.nodes.iter().find(|node| node.id == "node-a").expect("node-a survives its own move");',
        'assert_eq!((node.x, node.y), (5.0, 7.0), "move-node/moves-node-a: node-a did not land on the committed position");',
        'assert_eq!(node.radius, before().nodes[0].radius, "move-node/moves-node-a: a move must not resize the node");',
    ],
    [
        'assert_eq!(committed["nodes"]["patched"][0]["id"].as_str(), Some("node-a"), "move-node/moves-node-a: the diff must patch node-a and nothing else");',
        'assert_eq!(committed["nodes"]["patched"][0]["patch"]["replacement"]["x"].as_f64(), Some(5.0), "move-node/moves-node-a: the replacement must carry the new x");',
        'assert!(committed["edges"].is_null() && committed["meta"].is_null(), "move-node/moves-node-a: moving a node touches neither edges nor meta");',
    ],
)

# ─── 🧊replace-node-geometry ──────────────────────────────────────────────────────────────────
_node = with_node_a(shape="rectangle", radius=None, width=24.0, height=16.0)
case(
    "🧊replace-node-geometry", "circle-to-rectangle",
    "A whole-geometry swap on `node-a`: the builder writes all four of shape/radius/width/height "
    "from the payload, so the circle's `radius` is dropped as the rectangle's extent arrives.",
    {"mutation": "replaceNodeGeometry", "id": "node-a", "newShape": "rectangle",
     "newRadius": None, "newWidth": 24.0, "newHeight": 16.0},
    diff(nodes=nodes_delta(patched=patch("node-a", copy.deepcopy(_node)))),
    snapshot_with_node_a(_node), APPLIED,
    [
        'let node = snapshot.nodes.iter().find(|node| node.id == "node-a").expect("node-a survives its geometry swap");',
        'assert_eq!(node.shape.as_deref(), Some("rectangle"), "replace-node-geometry/circle-to-rectangle: node-a is still a circle");',
        'assert_eq!(node.radius, None, "replace-node-geometry/circle-to-rectangle: the circle radius must be cleared, not retained alongside the rectangle extent");',
        'assert_eq!((node.width, node.height), (Some(24.0), Some(16.0)), "replace-node-geometry/circle-to-rectangle: the rectangle extent is wrong");',
    ],
    [
        'assert_eq!(committed["nodes"]["patched"][0]["id"].as_str(), Some("node-a"), "replace-node-geometry/circle-to-rectangle: the diff must patch node-a");',
        'assert!(committed["nodes"]["patched"][0]["patch"]["replacement"].get("radius").is_none(), "replace-node-geometry/circle-to-rectangle: a cleared radius must be absent from the replacement, not null");',
        'assert!(committed["edges"].is_null(), "replace-node-geometry/circle-to-rectangle: geometry never touches edges");',
    ],
)

# ─── 🏗change-node-kind ───────────────────────────────────────────────────────────────────────
_node = with_node_a(nodeKind="node-kind-c")
case(
    "🏗change-node-kind", "reassigns-node-a-kind",
    "Repoints `node-a` at the `node-kind-c` catalog row. Only `nodeKind` moves; the node's pose, "
    "geometry and handles are carried through the replacement untouched.",
    {"mutation": "changeNodeKind", "id": "node-a", "newNodeKind": "node-kind-c"},
    diff(nodes=nodes_delta(patched=patch("node-a", copy.deepcopy(_node)))),
    snapshot_with_node_a(_node), APPLIED,
    [
        'let node = snapshot.nodes.iter().find(|node| node.id == "node-a").expect("node-a survives its kind change");',
        'assert_eq!(node.node_kind.as_deref(), Some("node-kind-c"), "change-node-kind/reassigns-node-a-kind: node-a still points at its old catalog row");',
        'assert_eq!(node.handles, before().nodes[0].handles, "change-node-kind/reassigns-node-a-kind: a kind change must not rebuild the handle list");',
    ],
    [
        'assert_eq!(committed["nodes"]["patched"][0]["patch"]["replacement"]["nodeKind"].as_str(), Some("node-kind-c"), "change-node-kind/reassigns-node-a-kind: the replacement must carry the new kind");',
        'assert!(committed["meta"].is_null(), "change-node-kind/reassigns-node-a-kind: pointing at a catalog row must not rewrite the catalog itself");',
    ],
)

# ─── ✏️edit-node-text ─────────────────────────────────────────────────────────────────────────
_node = with_node_a(text="Alpha Prime")
case(
    "✏️edit-node-text", "retitles-node-a",
    "Rewrites `node-a`'s authored display text. The payload is an `Option<String>`, so the builder "
    "assigns it wholesale — `null` would clear the text rather than leave it alone.",
    {"mutation": "editNodeText", "id": "node-a", "newText": "Alpha Prime"},
    diff(nodes=nodes_delta(patched=patch("node-a", copy.deepcopy(_node)))),
    snapshot_with_node_a(_node), APPLIED,
    [
        'let node = snapshot.nodes.iter().find(|node| node.id == "node-a").expect("node-a survives its retitle");',
        'assert_eq!(node.text.as_deref(), Some("Alpha Prime"), "edit-node-text/retitles-node-a: node-a kept its old text");',
        'assert_eq!(node.icon_kind, before().nodes[0].icon_kind, "edit-node-text/retitles-node-a: text and icon are separate fields");',
    ],
    [
        'assert_eq!(committed["nodes"]["patched"][0]["patch"]["replacement"]["text"].as_str(), Some("Alpha Prime"), "edit-node-text/retitles-node-a: the replacement must carry the new text");',
        'assert_eq!(committed["nodes"]["patched"].as_array().map(Vec::len), Some(1), "edit-node-text/retitles-node-a: exactly one node may be patched");',
    ],
)

# ─── 🎨change-node-icon ───────────────────────────────────────────────────────────────────────
_node = with_node_a(iconKind="icon-omega")
case(
    "🎨change-node-icon", "swaps-node-a-icon",
    "Swaps `node-a`'s `iconKind` to `icon-omega`. The builder rewrites that one presentation field "
    "and leaves the authored text alone.",
    {"mutation": "changeNodeIcon", "id": "node-a", "newIconKind": "icon-omega"},
    diff(nodes=nodes_delta(patched=patch("node-a", copy.deepcopy(_node)))),
    snapshot_with_node_a(_node), APPLIED,
    [
        'let node = snapshot.nodes.iter().find(|node| node.id == "node-a").expect("node-a survives its icon swap");',
        'assert_eq!(node.icon_kind.as_deref(), Some("icon-omega"), "change-node-icon/swaps-node-a-icon: node-a kept its old icon");',
        'assert_eq!(node.text, before().nodes[0].text, "change-node-icon/swaps-node-a-icon: an icon swap must not rewrite the text");',
    ],
    [
        'assert_eq!(committed["nodes"]["patched"][0]["patch"]["replacement"]["iconKind"].as_str(), Some("icon-omega"), "change-node-icon/swaps-node-a-icon: the replacement must carry the new icon kind");',
        'assert!(committed["edges"].is_null(), "change-node-icon/swaps-node-a-icon: an icon swap never touches edges");',
    ],
)

# ─── 📏scale-node ─────────────────────────────────────────────────────────────────────────────
_node = with_node_a(scale=2.0)
case(
    "📏scale-node", "doubles-node-a",
    "`node-a` carries no `scale` in the base snapshot; the builder writes `Some(2.0)`, so the field "
    "appears for the first time rather than being overwritten.",
    {"mutation": "scaleNode", "id": "node-a", "newScale": 2.0},
    diff(nodes=nodes_delta(patched=patch("node-a", copy.deepcopy(_node)))),
    snapshot_with_node_a(_node), APPLIED,
    [
        'let node = snapshot.nodes.iter().find(|node| node.id == "node-a").expect("node-a survives its scaling");',
        'assert_eq!(node.scale, Some(2.0), "scale-node/doubles-node-a: node-a did not take the committed scale factor");',
        'assert_eq!(node.radius, before().nodes[0].radius, "scale-node/doubles-node-a: scale is a separate field from the authored radius");',
    ],
    [
        'assert_eq!(committed["nodes"]["patched"][0]["patch"]["replacement"]["scale"].as_f64(), Some(2.0), "scale-node/doubles-node-a: the replacement must carry the new scale");',
        'assert!(committed["camera"].is_null(), "scale-node/doubles-node-a: scaling a node is not a camera zoom");',
    ],
)

# ─── 👁change-node-visible ────────────────────────────────────────────────────────────────────
_node = with_node_a(visible=False)
case(
    "👁change-node-visible", "hides-node-a",
    "Sets `node-a`'s tri-state `visible` flag to `Some(false)`. The base leaves it unset, so the "
    "field appears in the replacement.",
    {"mutation": "changeNodeVisible", "id": "node-a", "newVisible": False},
    diff(nodes=nodes_delta(patched=patch("node-a", copy.deepcopy(_node)))),
    snapshot_with_node_a(_node), APPLIED,
    [
        'let node = snapshot.nodes.iter().find(|node| node.id == "node-a").expect("node-a survives being hidden");',
        'assert_eq!(node.visible, Some(false), "change-node-visible/hides-node-a: node-a is still visible");',
        'assert_eq!(node.locked, before().nodes[0].locked, "change-node-visible/hides-node-a: hiding must not lock the node");',
    ],
    [
        'assert_eq!(committed["nodes"]["patched"][0]["patch"]["replacement"]["visible"].as_bool(), Some(false), "change-node-visible/hides-node-a: the replacement must carry visible=false");',
        'assert!(committed["edges"].is_null(), "change-node-visible/hides-node-a: hiding a node does not hide its edges");',
    ],
)

# ─── 🔒change-node-locked ─────────────────────────────────────────────────────────────────────
_node = with_node_a(locked=True)
case(
    "🔒change-node-locked", "locks-node-a",
    "Sets `node-a`'s tri-state `locked` flag to `Some(true)`. Locking is a document edit like any "
    "other here — it produces an ordinary node patch.",
    {"mutation": "changeNodeLocked", "id": "node-a", "newLocked": True},
    diff(nodes=nodes_delta(patched=patch("node-a", copy.deepcopy(_node)))),
    snapshot_with_node_a(_node), APPLIED,
    [
        'let node = snapshot.nodes.iter().find(|node| node.id == "node-a").expect("node-a survives being locked");',
        'assert_eq!(node.locked, Some(true), "change-node-locked/locks-node-a: node-a is still unlocked");',
        'assert_eq!(node.visible, before().nodes[0].visible, "change-node-locked/locks-node-a: locking must not change visibility");',
    ],
    [
        'assert_eq!(committed["nodes"]["patched"][0]["patch"]["replacement"]["locked"].as_bool(), Some(true), "change-node-locked/locks-node-a: the replacement must carry locked=true");',
        'assert_eq!(committed["nodes"]["removed"].as_array().map(Vec::is_empty), Some(true), "change-node-locked/locks-node-a: locking removes nothing");',
    ],
)

# ─── 🌟change-node-root ───────────────────────────────────────────────────────────────────────
_node = with_node_a(root=True)
case(
    "🌟change-node-root", "promotes-node-a-to-root",
    "Marks `node-a` as the layout seed by setting `root` to `Some(true)`. The builder writes only "
    "that flag — it does not demote any other node.",
    {"mutation": "changeNodeRoot", "id": "node-a", "newRoot": True},
    diff(nodes=nodes_delta(patched=patch("node-a", copy.deepcopy(_node)))),
    snapshot_with_node_a(_node), APPLIED,
    [
        'let node = snapshot.nodes.iter().find(|node| node.id == "node-a").expect("node-a survives its promotion");',
        'assert_eq!(node.root, Some(true), "change-node-root/promotes-node-a-to-root: node-a was not promoted");',
        'assert_eq!(snapshot.nodes[1], before().nodes[1], "change-node-root/promotes-node-a-to-root: promoting one node must not demote another");',
    ],
    [
        'assert_eq!(committed["nodes"]["patched"].as_array().map(Vec::len), Some(1), "change-node-root/promotes-node-a-to-root: only the promoted node may be patched");',
        'assert_eq!(committed["nodes"]["patched"][0]["patch"]["replacement"]["root"].as_bool(), Some(true), "change-node-root/promotes-node-a-to-root: the replacement must carry root=true");',
    ],
)

# ─── ⚓change-node-anchor ─────────────────────────────────────────────────────────────────────
_node = with_node_a(anchor="derived")
case(
    "⚓change-node-anchor", "fixed-to-derived",
    "Flips `node-a` from keeping its stored pose (`fixed`) to deriving it from its edges "
    "(`derived`). `anchor` is a plain enum with no `Option` wrapper, so it is always present.",
    {"mutation": "changeNodeAnchor", "id": "node-a", "newAnchor": "derived"},
    diff(nodes=nodes_delta(patched=patch("node-a", copy.deepcopy(_node)))),
    snapshot_with_node_a(_node), APPLIED,
    [
        'let node = snapshot.nodes.iter().find(|node| node.id == "node-a").expect("node-a survives its anchor flip");',
        'assert_eq!(node.anchor, crate::artifacts::puzzle2d::Puzzle2dNodeAnchor::Derived, "change-node-anchor/fixed-to-derived: node-a is still anchored fixed");',
        'assert_eq!((node.x, node.y), (before().nodes[0].x, before().nodes[0].y), "change-node-anchor/fixed-to-derived: flipping the anchor must not move the stored pose");',
    ],
    [
        'assert_eq!(committed["nodes"]["patched"][0]["patch"]["replacement"]["anchor"].as_str(), Some("derived"), "change-node-anchor/fixed-to-derived: the replacement must carry the derived anchor");',
        'assert!(committed["edges"].is_null(), "change-node-anchor/fixed-to-derived: the anchor flip must not re-solve the edges here");',
    ],
)

# ─── ➕add-node-handle ────────────────────────────────────────────────────────────────────────
HANDLE_3 = {"id": "handle-3", "handleKind": "handle-kind-b", "angle": 2.0}
_node_b = copy.deepcopy(NODE_B)
_node_b["handles"].append(copy.deepcopy(HANDLE_3))
_after = base()
_after["nodes"][1] = _node_b
case(
    "➕add-node-handle", "appends-handle-3-to-node-b",
    "Attaches a new rim port to `node-b`. A `null` index means the builder inserts at "
    "`handles.len()`, i.e. appends; the whole owner node is republished as one node patch.",
    {"mutation": "addNodeHandle", "nodeId": "node-b", "handle": copy.deepcopy(HANDLE_3),
     "index": None},
    diff(nodes=nodes_delta(patched=patch("node-b", copy.deepcopy(_node_b)))),
    _after, APPLIED,
    [
        'let node = snapshot.nodes.iter().find(|node| node.id == "node-b").expect("node-b survives gaining a handle");',
        'assert_eq!(node.handles.len(), 2, "add-node-handle/appends-handle-3-to-node-b: handle-3 was not attached");',
        'assert_eq!(node.handles[1].id, "handle-3", "add-node-handle/appends-handle-3-to-node-b: a null index must append the handle");',
        'assert_eq!(snapshot.nodes[0], before().nodes[0], "add-node-handle/appends-handle-3-to-node-b: only the owner node may change");',
    ],
    [
        'assert_eq!(committed["nodes"]["patched"][0]["id"].as_str(), Some("node-b"), "add-node-handle/appends-handle-3-to-node-b: the owner node is the patch target");',
        'assert_eq!(committed["nodes"]["patched"][0]["patch"]["replacement"]["handles"][1]["id"].as_str(), Some("handle-3"), "add-node-handle/appends-handle-3-to-node-b: the replacement must carry the appended handle");',
        'assert!(committed["edges"].is_null(), "add-node-handle/appends-handle-3-to-node-b: a fresh handle wires nothing up on its own");',
    ],
)

# ─── ➖remove-node-handle ─────────────────────────────────────────────────────────────────────
_node_b = copy.deepcopy(NODE_B)
_node_b["handles"] = []
_after = base()
_after["nodes"][1] = _node_b
_after["edges"] = []
case(
    "➖remove-node-handle", "removes-handle-2-and-severs-edge",
    "Detaching `handle-2` from `node-b` cascades: `edge-1` targets that handle, so the builder "
    "patches the owner node AND removes the dangling edge in the same diff.",
    {"mutation": "removeNodeHandle", "nodeId": "node-b", "handleId": "handle-2"},
    diff(nodes=nodes_delta(patched=patch("node-b", copy.deepcopy(_node_b))),
         edges=edges_delta(removed=["edge-1"])),
    _after, APPLIED,
    [
        'let node = snapshot.nodes.iter().find(|node| node.id == "node-b").expect("node-b survives losing a handle");',
        'assert!(node.handles.is_empty(), "remove-node-handle/removes-handle-2-and-severs-edge: handle-2 is still attached");',
        'assert!(snapshot.edges.is_empty(), "remove-node-handle/removes-handle-2-and-severs-edge: edge-1 targets handle-2 and must be severed");',
    ],
    [
        'assert_eq!(committed["nodes"]["patched"][0]["id"].as_str(), Some("node-b"), "remove-node-handle/removes-handle-2-and-severs-edge: the owner node is patched, not removed");',
        'assert_eq!(committed["edges"]["removed"][0].as_str(), Some("edge-1"), "remove-node-handle/removes-handle-2-and-severs-edge: the cascade must remove edge-1 by id");',
        'assert!(committed["nodes"]["removed"].as_array().map(Vec::is_empty).unwrap_or(false), "remove-node-handle/removes-handle-2-and-severs-edge: removing a handle must never remove its node");',
    ],
)

# ─── 🔌replace-node-handle ────────────────────────────────────────────────────────────────────
case(
    "🔌replace-node-handle", "rekind-handle-1-is-noop",
    "The builder clones the owner node and compares the clone against the original BEFORE writing "
    "the new handle, so its `next == *node` guard always fires: every `replace-node-handle` is a "
    "warned no-op with an empty diff. This fixture pins that actual behaviour, not the intent.",
    {"mutation": "replaceNodeHandle", "nodeId": "node-a", "handleId": "handle-1",
     "newHandle": {"id": "handle-1", "handleKind": "handle-kind-c", "angle": 0.0}},
    diff(),
    base(), APPLIED_NOOP,
    [
        'assert_eq!(snapshot, before(), "replace-node-handle/rekind-handle-1-is-noop: the builder\'s clone-then-compare guard fires first, so nothing may change");',
        'let node = snapshot.nodes.iter().find(|node| node.id == "node-a").expect("node-a is untouched");',
        'assert_eq!(node.handles[0].handle_kind.as_deref(), Some("handle-kind-a"), "replace-node-handle/rekind-handle-1-is-noop: handle-1 must keep its base kind");',
    ],
    [
        'assert!(committed["nodes"].is_null(), "replace-node-handle/rekind-handle-1-is-noop: the no-op guard must leave the nodes delta unset");',
        'assert!(committed.as_object().expect("the committed diff is a JSON object").values().all(serde_json::Value::is_null), "replace-node-handle/rekind-handle-1-is-noop: a no-op diff must carry no populated field at all");',
    ],
)

# ─── 🔗connect-handles ────────────────────────────────────────────────────────────────────────
EDGE_2 = {
    "id": "edge-2",
    "source": "handle-spare",
    "target": "handle-2",
    "edgeKind": "edge-kind-b",
    "gap": 0.0,
    "shift": 0.0,
    "rise": 0.0,
    "rotation": 0.0,
    "turn": 0.0,
    "tilt": 0.0,
    "x": 0.0,
    "y": 0.0,
}
_after = base()
_after["edges"].append(copy.deepcopy(EDGE_2))
case(
    "🔗connect-handles", "adds-second-edge",
    "Links `handle-spare` to `handle-2` as `edge-2`. The builder synthesises the edge from the "
    "payload and hard-codes `visible`/`locked` to `None` — a fresh link carries no overrides.",
    {"mutation": "connectHandles", "id": "edge-2", "source": "handle-spare", "target": "handle-2",
     "edgeKind": "edge-kind-b", "gap": 0.0, "shift": 0.0, "rise": 0.0, "rotation": 0.0,
     "turn": 0.0, "tilt": 0.0, "x": 0.0, "y": 0.0, "sourceTip": None, "targetTip": None},
    diff(edges=edges_delta(added=[copy.deepcopy(EDGE_2)])),
    _after, APPLIED,
    [
        'assert_eq!(snapshot.edges.len(), 2, "connect-handles/adds-second-edge: edge-2 was not appended");',
        'let edge = snapshot.edges.iter().find(|edge| edge.id == "edge-2").expect("edge-2 exists after connecting");',
        'assert_eq!((edge.source.as_str(), edge.target.as_str()), ("handle-spare", "handle-2"), "connect-handles/adds-second-edge: edge-2 joins the wrong handles");',
        'assert_eq!((edge.visible, edge.locked), (None, None), "connect-handles/adds-second-edge: a freshly connected edge must carry no presentation overrides");',
    ],
    [
        'assert_eq!(committed["edges"]["added"][0]["id"].as_str(), Some("edge-2"), "connect-handles/adds-second-edge: the diff must carry edge-2 in edges.added");',
        'assert!(committed["nodes"].is_null(), "connect-handles/adds-second-edge: connecting handles must not republish either node");',
    ],
)

# ─── ✂️disconnect-handles ────────────────────────────────────────────────────────────────────
_after = base()
_after["edges"] = []
case(
    "✂️disconnect-handles", "removes-edge-1",
    "Severs `edge-1`. The builder emits a real `edges.removed` entry — never a whole-snapshot "
    "capture — and leaves both endpoint nodes and their handles in place.",
    {"mutation": "disconnectHandles", "id": "edge-1"},
    diff(edges=edges_delta(removed=["edge-1"])),
    _after, APPLIED,
    [
        'assert!(snapshot.edges.is_empty(), "disconnect-handles/removes-edge-1: edge-1 survived the disconnect");',
        'assert_eq!(snapshot.nodes, before().nodes, "disconnect-handles/removes-edge-1: disconnecting must leave both endpoint nodes and their handles intact");',
    ],
    [
        'assert_eq!(committed["edges"]["removed"][0].as_str(), Some("edge-1"), "disconnect-handles/removes-edge-1: the diff must remove edge-1 by id");',
        'assert!(committed["nodes"].is_null(), "disconnect-handles/removes-edge-1: a disconnect must not touch the nodes delta");',
    ],
)

# ─── 🧮replace-edge-geometry ──────────────────────────────────────────────────────────────────
_edge = with_edge_1(gap=2.0, shift=1.0, x=3.0, y=4.0)
case(
    "🧮replace-edge-geometry", "repositions-edge-1",
    "A whole-pose swap on `edge-1`: the builder writes all eight connection parameters "
    "(gap/shift/rise/rotation/turn/tilt/x/y) from the payload in one gesture.",
    {"mutation": "replaceEdgeGeometry", "id": "edge-1", "newGap": 2.0, "newShift": 1.0,
     "newRise": 0.0, "newRotation": 0.0, "newTurn": 0.0, "newTilt": 0.0, "newX": 3.0, "newY": 4.0},
    diff(edges=edges_delta(patched=patch("edge-1", copy.deepcopy(_edge)))),
    snapshot_with_edge_1(_edge), APPLIED,
    [
        'let edge = snapshot.edges.iter().find(|edge| edge.id == "edge-1").expect("edge-1 survives its repose");',
        'assert_eq!((edge.gap, edge.shift), (2.0, 1.0), "replace-edge-geometry/repositions-edge-1: the connection offsets are wrong");',
        'assert_eq!((edge.x, edge.y), (3.0, 4.0), "replace-edge-geometry/repositions-edge-1: the diagram position is wrong");',
        'assert_eq!((edge.source.as_str(), edge.target.as_str()), ("handle-1", "handle-2"), "replace-edge-geometry/repositions-edge-1: a repose must not rewire the endpoints");',
    ],
    [
        'assert_eq!(committed["edges"]["patched"][0]["id"].as_str(), Some("edge-1"), "replace-edge-geometry/repositions-edge-1: the diff must patch edge-1");',
        'assert_eq!(committed["edges"]["patched"][0]["patch"]["replacement"]["gap"].as_f64(), Some(2.0), "replace-edge-geometry/repositions-edge-1: the replacement must carry the new gap");',
        'assert!(committed["nodes"].is_null(), "replace-edge-geometry/repositions-edge-1: an edge repose never republishes a node");',
    ],
)

# ─── 🏷change-edge-kind ───────────────────────────────────────────────────────────────────────
_edge = with_edge_1(edgeKind="edge-kind-c")
case(
    "🏷change-edge-kind", "rekinds-edge-1",
    "Repoints `edge-1` at the `edge-kind-c` catalog row. Only `edgeKind` moves — the tips and the "
    "eight connection parameters are carried through untouched.",
    {"mutation": "changeEdgeKind", "id": "edge-1", "newEdgeKind": "edge-kind-c"},
    diff(edges=edges_delta(patched=patch("edge-1", copy.deepcopy(_edge)))),
    snapshot_with_edge_1(_edge), APPLIED,
    [
        'let edge = snapshot.edges.iter().find(|edge| edge.id == "edge-1").expect("edge-1 survives its kind change");',
        'assert_eq!(edge.edge_kind.as_deref(), Some("edge-kind-c"), "change-edge-kind/rekinds-edge-1: edge-1 still points at its old catalog row");',
        'assert_eq!((edge.source_tip.clone(), edge.target_tip.clone()), (before().edges[0].source_tip.clone(), before().edges[0].target_tip.clone()), "change-edge-kind/rekinds-edge-1: a kind change must not redraw the tips");',
    ],
    [
        'assert_eq!(committed["edges"]["patched"][0]["patch"]["replacement"]["edgeKind"].as_str(), Some("edge-kind-c"), "change-edge-kind/rekinds-edge-1: the replacement must carry the new kind");',
        'assert!(committed["meta"].is_null(), "change-edge-kind/rekinds-edge-1: pointing at a catalog row must not rewrite the catalog itself");',
    ],
)

# ─── 🖇change-edge-tips ───────────────────────────────────────────────────────────────────────
_edge = with_edge_1(sourceTip="arrow", targetTip="none")
case(
    "🖇change-edge-tips", "swaps-edge-1-tips",
    "Writes BOTH tip fields of `edge-1` at once — the builder assigns `source_tip` and `target_tip` "
    "from the payload, so this fixture swaps the arrow from the target end to the source end.",
    {"mutation": "changeEdgeTips", "id": "edge-1", "newSourceTip": "arrow",
     "newTargetTip": "none"},
    diff(edges=edges_delta(patched=patch("edge-1", copy.deepcopy(_edge)))),
    snapshot_with_edge_1(_edge), APPLIED,
    [
        'let edge = snapshot.edges.iter().find(|edge| edge.id == "edge-1").expect("edge-1 survives its tip swap");',
        'assert_eq!(edge.source_tip.as_deref(), Some("arrow"), "change-edge-tips/swaps-edge-1-tips: the arrow did not move to the source end");',
        'assert_eq!(edge.target_tip.as_deref(), Some("none"), "change-edge-tips/swaps-edge-1-tips: the target end must lose its arrow");',
    ],
    [
        'assert_eq!(committed["edges"]["patched"][0]["patch"]["replacement"]["sourceTip"].as_str(), Some("arrow"), "change-edge-tips/swaps-edge-1-tips: the replacement must carry the new source tip");',
        'assert_eq!(committed["edges"]["patched"][0]["patch"]["replacement"]["targetTip"].as_str(), Some("none"), "change-edge-tips/swaps-edge-1-tips: the replacement must carry the new target tip");',
    ],
)

# ─── 👀change-edge-visible ────────────────────────────────────────────────────────────────────
_edge = with_edge_1(visible=False)
case(
    "👀change-edge-visible", "hides-edge-1",
    "Sets `edge-1`'s tri-state `visible` flag to `Some(false)`. The base leaves it unset, so the "
    "field appears in the replacement for the first time.",
    {"mutation": "changeEdgeVisible", "id": "edge-1", "newVisible": False},
    diff(edges=edges_delta(patched=patch("edge-1", copy.deepcopy(_edge)))),
    snapshot_with_edge_1(_edge), APPLIED,
    [
        'let edge = snapshot.edges.iter().find(|edge| edge.id == "edge-1").expect("edge-1 survives being hidden");',
        'assert_eq!(edge.visible, Some(false), "change-edge-visible/hides-edge-1: edge-1 is still visible");',
        'assert_eq!(snapshot.nodes, before().nodes, "change-edge-visible/hides-edge-1: hiding an edge must not hide its endpoint nodes");',
    ],
    [
        'assert_eq!(committed["edges"]["patched"][0]["patch"]["replacement"]["visible"].as_bool(), Some(false), "change-edge-visible/hides-edge-1: the replacement must carry visible=false");',
        'assert!(committed["nodes"].is_null(), "change-edge-visible/hides-edge-1: the nodes delta must stay unset");',
    ],
)

# ─── 🔐change-edge-locked ─────────────────────────────────────────────────────────────────────
_edge = with_edge_1(locked=True)
case(
    "🔐change-edge-locked", "locks-edge-1",
    "Sets `edge-1`'s tri-state `locked` flag to `Some(true)`, producing an ordinary edge patch — "
    "locking is a document edit here, not a presence or config lane change.",
    {"mutation": "changeEdgeLocked", "id": "edge-1", "newLocked": True},
    diff(edges=edges_delta(patched=patch("edge-1", copy.deepcopy(_edge)))),
    snapshot_with_edge_1(_edge), APPLIED,
    [
        'let edge = snapshot.edges.iter().find(|edge| edge.id == "edge-1").expect("edge-1 survives being locked");',
        'assert_eq!(edge.locked, Some(true), "change-edge-locked/locks-edge-1: edge-1 is still unlocked");',
        'assert_eq!(edge.visible, before().edges[0].visible, "change-edge-locked/locks-edge-1: locking must not change visibility");',
    ],
    [
        'assert_eq!(committed["edges"]["patched"][0]["patch"]["replacement"]["locked"].as_bool(), Some(true), "change-edge-locked/locks-edge-1: the replacement must carry locked=true");',
        'assert_eq!(committed["edges"]["removed"].as_array().map(Vec::is_empty), Some(true), "change-edge-locked/locks-edge-1: locking removes nothing");',
    ],
)

# ─── 🆔change-manifest-id ─────────────────────────────────────────────────────────────────────
_meta = copy.deepcopy(BASE["meta"])
_meta["manifestId"] = "manifest-beta"
_after = base()
_after["meta"] = copy.deepcopy(_meta)
case(
    "🆔change-manifest-id", "repoints-manifest",
    "`meta.manifestId` is a document-root singleton, so the builder has no missing-target branch: "
    "it clones the whole `meta` block, rewrites the one field, and publishes `meta` wholesale.",
    {"mutation": "changeManifestId", "newManifestId": "manifest-beta"},
    diff(meta=copy.deepcopy(_meta)),
    _after, APPLIED,
    [
        'assert_eq!(snapshot.meta.manifest_id.as_deref(), Some("manifest-beta"), "change-manifest-id/repoints-manifest: the manifest reference did not move");',
        'assert_eq!(snapshot.meta.kind_compatibility, before().meta.kind_compatibility, "change-manifest-id/repoints-manifest: republishing meta must carry the compatibility table through unchanged");',
    ],
    [
        'assert_eq!(committed["meta"]["manifestId"].as_str(), Some("manifest-beta"), "change-manifest-id/repoints-manifest: the diff must publish the new manifest id on meta");',
        'assert!(committed["nodes"].is_null() && committed["edges"].is_null(), "change-manifest-id/repoints-manifest: a meta edit must touch no collection delta");',
    ],
)

# ─── 🤝connect-kind-compatibility ─────────────────────────────────────────────────────────────
COMPAT_BC = {
    "source": "handle-kind-b",
    "target": "handle-kind-c",
    "bidirectional": False,
    "important": True,
    "specificity": "wire",
}
_meta = copy.deepcopy(BASE["meta"])
_meta["kindCompatibility"] = [copy.deepcopy(COMPAT_AB), copy.deepcopy(COMPAT_BC)]
_after = base()
_after["meta"] = copy.deepcopy(_meta)
case(
    "🤝connect-kind-compatibility", "adds-handle-kind-pair",
    "Appends a `handle-kind-b -> handle-kind-c` allowance to `meta.kindCompatibility`. The builder "
    "PUSHES the row (it never sorts or de-duplicates beyond the source/target pair check).",
    {"mutation": "connectKindCompatibility", "source": "handle-kind-b", "target": "handle-kind-c",
     "bidirectional": False, "important": True, "specificity": "wire"},
    diff(meta=copy.deepcopy(_meta)),
    _after, APPLIED,
    [
        'assert_eq!(snapshot.meta.kind_compatibility.len(), 2, "connect-kind-compatibility/adds-handle-kind-pair: the new allowance was not appended");',
        'let row = &snapshot.meta.kind_compatibility[1];',
        'assert_eq!((row.source.as_str(), row.target.as_str()), ("handle-kind-b", "handle-kind-c"), "connect-kind-compatibility/adds-handle-kind-pair: the row must be pushed at the end, in payload order");',
        'assert!(row.important && !row.bidirectional, "connect-kind-compatibility/adds-handle-kind-pair: the payload flags must be carried onto the row verbatim");',
    ],
    [
        'assert_eq!(committed["meta"]["kindCompatibility"].as_array().map(Vec::len), Some(2), "connect-kind-compatibility/adds-handle-kind-pair: the republished meta must carry both rows");',
        'assert_eq!(committed["meta"]["kindCompatibility"][1]["specificity"].as_str(), Some("wire"), "connect-kind-compatibility/adds-handle-kind-pair: the specificity must survive onto the new row");',
    ],
)

# ─── 💔disconnect-kind-compatibility ──────────────────────────────────────────────────────────
_meta = copy.deepcopy(BASE["meta"])
_meta["kindCompatibility"] = []
_after = base()
_after["meta"] = copy.deepcopy(_meta)
case(
    "💔disconnect-kind-compatibility", "removes-handle-kind-pair",
    "Revokes the `handle-kind-a -> handle-kind-b` allowance. The builder retains every row whose "
    "source/target pair does not match, then republishes the whole `meta` block.",
    {"mutation": "disconnectKindCompatibility", "source": "handle-kind-a",
     "target": "handle-kind-b"},
    diff(meta=copy.deepcopy(_meta)),
    _after, APPLIED,
    [
        'assert!(snapshot.meta.kind_compatibility.is_empty(), "disconnect-kind-compatibility/removes-handle-kind-pair: the allowance was not revoked");',
        'assert_eq!(snapshot.meta.manifest_id, before().meta.manifest_id, "disconnect-kind-compatibility/removes-handle-kind-pair: republishing meta must keep the manifest reference");',
        'assert_eq!(snapshot.edges, before().edges, "disconnect-kind-compatibility/removes-handle-kind-pair: revoking a kind allowance must not sever any existing edge");',
    ],
    [
        'assert_eq!(committed["meta"]["kindCompatibility"].as_array().map(Vec::is_empty), Some(true), "disconnect-kind-compatibility/removes-handle-kind-pair: the republished meta must carry an empty table");',
        'assert!(committed["edges"].is_null(), "disconnect-kind-compatibility/removes-handle-kind-pair: no edge cascade may appear in this diff");',
    ],
)

# ─── 📚replace-kind-catalogs ──────────────────────────────────────────────────────────────────
CATALOGS = {
    "nodes": [],
    "handles": [
        {
            "id": "handle-kind-a",
            "compatibleWith": ["handle-kind-b"],
            "description": "Alpha port",
            "icon": "circle",
            "color": "#3366ff",
            "defaultWireKind": "wire-kind-a",
        }
    ],
    "edges": [],
    "wires": [],
}
_meta = copy.deepcopy(BASE["meta"])
_meta["kindCatalogs"] = copy.deepcopy(CATALOGS)
_after = base()
_after["meta"] = copy.deepcopy(_meta)
case(
    "📚replace-kind-catalogs", "installs-handle-kind-catalog",
    "A whole-value swap of the typed catalog bundle: the base carries `None`, so the payload's "
    "nodes/handles/edges/wires bundle is installed as one manifest-import gesture on `meta`.",
    {"mutation": "replaceKindCatalogs", "newCatalogs": copy.deepcopy(CATALOGS)},
    diff(meta=copy.deepcopy(_meta)),
    _after, APPLIED,
    [
        'let catalogs = snapshot.meta.kind_catalogs.as_ref().expect("replace-kind-catalogs/installs-handle-kind-catalog: the catalog bundle was not installed");',
        'assert_eq!(catalogs.handles.len(), 1, "replace-kind-catalogs/installs-handle-kind-catalog: the handle-kind catalog is empty");',
        'assert_eq!(catalogs.handles[0].default_wire_kind, "wire-kind-a", "replace-kind-catalogs/installs-handle-kind-catalog: the catalog row did not survive the swap");',
        'assert_eq!(snapshot.meta.kind_compatibility, before().meta.kind_compatibility, "replace-kind-catalogs/installs-handle-kind-catalog: installing catalogs must not rewrite the compatibility table");',
    ],
    [
        'assert_eq!(committed["meta"]["kindCatalogs"]["handles"][0]["id"].as_str(), Some("handle-kind-a"), "replace-kind-catalogs/installs-handle-kind-catalog: the diff must publish the catalogs on meta");',
        'assert!(committed["nodes"].is_null(), "replace-kind-catalogs/installs-handle-kind-catalog: a catalog swap must not republish any document node");',
    ],
)


HEADER = '''//! 🧪️ `{leaf_slug}` fixture — `{case}`.
//!
{summary}
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::puzzle2d::mutations::{{apply_puzzle2d_mutation, inverse_puzzle2d_mutation}};
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> Puzzle2dSnapshot {{
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}}
fn expected_after() -> Puzzle2dSnapshot {{
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}}
fn mutation() -> Puzzle2dMutation {{
    serde_json::from_str(MUTATION).expect("mutation decodes")
}}
'''

BODY = '''
/// ▶️ The committed `{leaf_slug}` payload carries `before` to exactly the committed `after`, and
/// lands the change this case is named for.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {{
    let mut snapshot = before();
    apply_puzzle2d_mutation(&mut snapshot, &mutation()).expect("{leaf_slug} applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "{leaf_slug}/{case}: applied state differs from committed after-snapshot");
{after_asserts}
}}

/// ↩️ Applying `{leaf_slug}` then the inverse it derives from `before` restores `before` exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {{
    let base = before();
    let mutation = mutation();
    let inverse = inverse_puzzle2d_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_puzzle2d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {{
        apply_puzzle2d_mutation(&mut snapshot, step).expect("inverse step applies");
    }}
    assert_eq!(snapshot, base, "{leaf_slug}/{case}: inverse did not restore the before-snapshot");
}}

/// 🔣️ Both committed snapshots and the committed `{leaf_slug}` payload are already canonical:
/// decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {{
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {{
        let decoded: Puzzle2dSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "{leaf_slug}/{case}: committed {{label}} JSON is not canonical");
    }}
    let decoded_mutation = mutation();
    let reencoded = serde_json::to_value(&decoded_mutation).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "{leaf_slug}/{case}: committed mutation JSON is not canonical");
}}

/// 🎯️ The declared outcome matches what `{leaf_slug}` actually produces on this base.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {{
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_puzzle2d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {{
        "applied" => assert!(applied, "{leaf_slug}/{case}: declared applied but the mutation was rejected"),
        "rejected" => {{
            assert!(!applied, "{leaf_slug}/{case}: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "{leaf_slug}/{case}: rejected mutation must leave the snapshot untouched");
        }}
        other => panic!("{leaf_slug}/{case}: unknown outcome status {{other:?}}"),
    }}
{outcome_messages}}}

/// 🔺️ The sparse delta `{leaf_slug}` produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH collections and fields this mutation is
/// allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {{
    let base = before();
    let outcome = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "{leaf_slug}/{case}: produced diff differs from the committed 🔺️diff/🔣️component.json");
{diff_asserts}
}}

/// 🔣️ The committed `{leaf_slug}` diff is itself canonical and decodes to `Puzzle2dDiff`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {{
    let decoded: crate::artifacts::puzzle2d::diff::Puzzle2dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "{leaf_slug}/{case}: committed diff JSON is not canonical");
}}

/// 🩹 Applying the committed `{leaf_slug}` diff directly to `before` yields the committed `after` —
/// the diff is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {{
    let decoded: crate::artifacts::puzzle2d::diff::Puzzle2dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::puzzle2d::diff::Puzzle2dDiff as protocol::MutationDiff<Puzzle2dSnapshot>>::apply(&decoded, &before())
        .expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "{leaf_slug}/{case}: committed diff did not carry before to after");
}}
'''

NOOP_OUTCOME_CHECK = '''    let messages = outcome.get("messages").and_then(serde_json::Value::as_array).expect("{leaf_slug}/{case}: this case declares a warn no-op and must list it");
    assert_eq!(messages[0]["code"].as_str(), Some("mutation.no-op"), "{leaf_slug}/{case}: the declared message must be the no-op warning the builder raises");
'''


def write(path, payload):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(payload, indent=2, ensure_ascii=False) + "\n")


def main():
    mounts = []
    for entry in CASES:
        case_dir = os.path.join(TREE, entry["leaf"], "🧪️tests", entry["name"])
        write(os.path.join(case_dir, "📸️snapshot/⬅️before/🔣️component.json"), entry["before"])
        write(os.path.join(case_dir, "📸️snapshot/➡️after/🔣️component.json"), entry["after"])
        write(os.path.join(case_dir, "🦠️mutation/🔣️component.json"), entry["mutation"])
        write(os.path.join(case_dir, "🔺️diff/🔣️component.json"), entry["diff"])
        write(os.path.join(case_dir, "🎯️outcome/🔣️component.json"), entry["outcome"])

        leaf_slug = entry["leaf"]
        while leaf_slug and not (leaf_slug[0].isalpha()):
            leaf_slug = leaf_slug[1:]
        summary = "\n".join("//! " + line for line in textwrap.wrap(entry["summary"], 96))
        diff_asserts = ["    " + line for line in entry["diff_asserts"]]
        outcome_messages = ""
        if entry["outcome"].get("messages"):
            outcome_messages = NOOP_OUTCOME_CHECK.format(leaf_slug=leaf_slug, case=entry["name"])
        text = HEADER.format(leaf_slug=leaf_slug, case=entry["name"], summary=summary) + BODY.format(
            leaf_slug=leaf_slug,
            case=entry["name"],
            summary=summary,
            after_asserts="\n".join("    " + line for line in entry["after_asserts"]),
            diff_asserts="\n".join(diff_asserts),
            outcome_messages=outcome_messages,
        )
        with open(os.path.join(case_dir, "🦀️component.rs"), "w", encoding="utf-8") as handle:
            handle.write(text)
        mounts.append((entry["leaf"], entry["name"]))
    for leaf, name in mounts:
        print(f"{leaf}\t{name}")


if __name__ == "__main__":
    main()
