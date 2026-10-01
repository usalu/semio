#!/usr/bin/env python3
"""🧫️ W1-F authoring tool: writes the fifteen selection-transform fixture quintets, their Rust leaf
tests, and refreshes the fixture catalog hashes. The transform arithmetic mirrors the Rust leaves
operation for operation (IEEE doubles, same evaluation order), so the committed after-snapshots are
exactly what the Rust diff builders must reproduce."""
import copy
import hashlib
import json
import math
import os

SUBSET = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any"
FIXTURES = os.path.join(SUBSET, "🧫️fixtures", "🧬️mutations")
LEAVES = os.path.join(SUBSET, "🧬️schema", "🧬️mutations")
QUARTER = math.pi / 2

BOARD = {
    "camera": {"x": 0.0, "y": 0.0, "zoom": 1.0},
    "edges": [{"edgeKind": "edge-kind-a", "gap": 1.0, "id": "edge-1", "rise": 0.0, "rotation": 0.0, "shift": 0.0, "source": "handle-1", "sourceTip": "none", "target": "handle-2", "targetTip": "arrow", "tilt": 0.0, "turn": 0.0, "x": 0.0, "y": 0.0}],
    "meta": {"kindCompatibility": [{"bidirectional": True, "important": False, "source": "handle-kind-a", "specificity": "handle", "target": "handle-kind-b"}], "manifestId": "manifest-alpha"},
    "nodes": [
        {"anchor": "fixed", "handles": [{"angle": 0.0, "handleKind": "handle-kind-a", "id": "handle-1"}, {"angle": 1.5, "handleKind": "handle-kind-a", "id": "handle-spare"}], "iconKind": "icon-alpha", "id": "node-a", "nodeKind": "node-kind-a", "radius": 10.0, "shape": "circle", "text": "Alpha", "x": 0.0, "y": 0.0},
        {"anchor": "fixed", "handles": [{"angle": 3.0, "handleKind": "handle-kind-b", "id": "handle-2"}], "height": 12.0, "id": "node-b", "nodeKind": "node-kind-b", "shape": "rectangle", "width": 30.0, "x": 40.0, "y": 20.0},
        {"anchor": "fixed", "handles": [{"angle": 0.5, "handleKind": "handle-kind-b", "id": "handle-3"}], "id": "node-c", "locked": True, "nodeKind": "node-kind-b", "radius": 8.0, "shape": "circle", "x": 80.0, "y": 0.0},
    ],
    "schema": "puzzle.2d.fixture",
    "targetRegions": [
        {"height": 60.5, "hidden": False, "id": "region-1", "locked": False, "width": 80.5, "x": -12.5, "y": -12.5},
        {"height": 20.0, "hidden": False, "id": "region-2", "locked": True, "width": 20.0, "x": 60.0, "y": -10.0},
    ],
}


def drag(dx, dy):
    return (lambda node: dict(node, x=node["x"] + dx, y=node["y"] + dy)), (lambda region: dict(region, x=region["x"] + dx, y=region["y"] + dy)), dx == 0.0 and dy == 0.0


def rotate(cx, cy, radians):
    sin, cos = math.sin(radians), math.cos(radians)

    def node(entry):
        turned = dict(entry, x=cx + (entry["x"] - cx) * cos - (entry["y"] - cy) * sin, y=cy + (entry["x"] - cx) * sin + (entry["y"] - cy) * cos)
        turned["handles"] = [dict(handle, angle=handle["angle"] + radians) for handle in entry["handles"]]
        return turned

    return node, None, radians == 0.0


def scale(cx, cy, factor):
    node = lambda entry: dict(entry, x=cx + (entry["x"] - cx) * factor, y=cy + (entry["y"] - cy) * factor)
    region = lambda entry: dict(entry, x=cx + (entry["x"] - cx) * factor, y=cy + (entry["y"] - cy) * factor, width=entry["width"] * factor, height=entry["height"] * factor)
    return node, region, factor == 1.0


def outcome_of(board, targets, transform):
    """🎯️ The same classification and outcome rules as `puzzle2d_selection_diff`."""
    node_fn, region_fn, identity = transform
    missing, locked, fixed, survivors = [], [], [], []
    for identifier in targets:
        if identifier in survivors or identifier in missing or identifier in locked or identifier in fixed:
            continue
        node = next((entry for entry in board["nodes"] if entry["id"] == identifier), None)
        region = next((entry for entry in board.get("targetRegions", []) if entry["id"] == identifier), None)
        if node is not None:
            (survivors if not node.get("locked", False) else locked).append(identifier)
        elif region is not None:
            if region["locked"]:
                locked.append(identifier)
            elif region_fn is None:
                fixed.append(identifier)
            else:
                survivors.append(identifier)
        else:
            missing.append(identifier)
    if not survivors:
        return None, {"status": "rejected", "code": "mutation.target-missing", "path": list(targets)}
    messages = [{"code": "mutation.partial", "level": "warning", "target": ids} for ids in (missing, locked, fixed) if ids]
    after = copy.deepcopy(board)
    patched_nodes, patched_regions = [], []
    if not identity:
        for at, node in enumerate(after["nodes"]):
            if node["id"] in survivors:
                moved = node_fn(node)
                if moved != node:
                    after["nodes"][at] = moved
                    patched_nodes.append({"id": node["id"], "patch": {"replacement": moved}})
        if region_fn is not None:
            for at, region in enumerate(after.get("targetRegions", [])):
                if region["id"] in survivors:
                    moved = region_fn(region)
                    if moved != region:
                        after["targetRegions"][at] = moved
                        patched_regions.append({"id": region["id"], "patch": {"replacement": moved}})
    delta = lambda patched: {"added": [], "patched": patched, "removed": [], "reordered": None} if patched else None
    diff = {"artifact": None, "camera": None, "edges": None, "meta": None, "nodes": delta(patched_nodes), "schema": None, "targetRegions": delta(patched_regions)}
    if not patched_nodes and not patched_regions:
        return (after, diff), {"status": "no-op", "messages": messages + [{"code": "mutation.no-op", "level": "warning", "target": list(targets)}]}
    return (after, diff), dict({"status": "applied"}, **({"messages": messages} if messages else {}))


CASES = [
    ("✋️drag-selection", "✋️drags-two-nodes", {"mutation": "dragSelection", "targets": ["node-a", "node-b"], "dx": 5.0, "dy": -2.5}, "Drags `node-a` and `node-b` by (5, -2.5): both positions move by the one offset, read off the base; nothing else on the board changes."),
    ("✋️drag-selection", "🎯️drags-node-and-region", {"mutation": "dragSelection", "targets": ["node-a", "region-1"], "dx": 10.0, "dy": 4.0}, "One drag over a node AND a target region (ids classified by document membership): `node-a` and `region-1`'s corner both move by (10, 4)."),
    ("✋️drag-selection", "⚠️skips-locked-ghost", {"mutation": "dragSelection", "targets": ["node-b", "node-c", "region-2", "node-ghost"], "dx": 2.0, "dy": 3.0}, "`node-b` moves by (2, 3); the absent `node-ghost` and the locked `node-c` and `region-2` are skipped with one Warning-level `mutation.partial` per reason."),
    ("✋️drag-selection", "🚫️rejects-ghosts", {"mutation": "dragSelection", "targets": ["node-ghost", "region-ghost"], "dx": 5.0, "dy": 5.0}, "Every target is absent: Error-level `mutation.target-missing`, nothing moves."),
    ("✋️drag-selection", "⏸️keeps-a-zero-offset", {"mutation": "dragSelection", "targets": ["node-a"], "dx": 0.0, "dy": 0.0}, "A zero offset is a Warning-level `mutation.no-op`: the default diff, nothing to undo."),
    ("🔄️rotate-selection", "🔄️turns-two-nodes", {"mutation": "rotateSelection", "targets": ["node-a", "node-b"], "pivotX": 20.0, "pivotY": 10.0, "angle": QUARTER}, "A quarter turn of `node-a` and `node-b` about (20, 10): both orbit the pivot and every one of their handle angles turns by the same π/2, so the edge between them keeps its geometry."),
    ("🔄️rotate-selection", "🎯️skips-the-region", {"mutation": "rotateSelection", "targets": ["node-b", "region-1"], "pivotX": 20.0, "pivotY": 10.0, "angle": QUARTER}, "A quarter turn over `node-b` and `region-1`: the node orbits the pivot, the axis-aligned target region is skipped with a Warning-level `mutation.partial`."),
    ("🔄️rotate-selection", "⚠️skips-locked-ghost", {"mutation": "rotateSelection", "targets": ["node-a", "node-c", "node-ghost"], "pivotX": 0.0, "pivotY": 0.0, "angle": QUARTER}, "A quarter turn about `node-a`'s own position keeps it in place and turns only its handles; the absent `node-ghost` and the locked `node-c` are skipped as `mutation.partial`."),
    ("🔄️rotate-selection", "🚫️rejects-ghosts", {"mutation": "rotateSelection", "targets": ["node-ghost"], "pivotX": 0.0, "pivotY": 0.0, "angle": QUARTER}, "The only target is absent: Error-level `mutation.target-missing`, nothing turns."),
    ("🔄️rotate-selection", "⏸️keeps-a-zero-angle", {"mutation": "rotateSelection", "targets": ["node-a"], "pivotX": 20.0, "pivotY": 10.0, "angle": 0.0}, "A zero angle is a Warning-level `mutation.no-op`: the default diff, nothing to undo."),
    ("🔍️scale-selection", "🔍️doubles-two-nodes", {"mutation": "scaleSelection", "targets": ["node-a", "node-b"], "pivotX": 20.0, "pivotY": 10.0, "factor": 2.0}, "Factor 2 about (20, 10): `node-a` and `node-b` spread away from the pivot while their own sizes stay."),
    ("🔍️scale-selection", "🎯️halves-node-region", {"mutation": "scaleSelection", "targets": ["node-a", "region-1"], "pivotX": 20.0, "pivotY": 10.0, "factor": 0.5}, "Factor 0.5 about (20, 10): `node-a` closes in on the pivot and `region-1` scales its corner AND its extent."),
    ("🔍️scale-selection", "⚠️skips-locked-ghost", {"mutation": "scaleSelection", "targets": ["node-b", "node-c", "region-2", "node-ghost"], "pivotX": 20.0, "pivotY": 10.0, "factor": 2.0}, "`node-b` spreads from the pivot; the absent `node-ghost` and the locked `node-c` and `region-2` are skipped with one `mutation.partial` per reason."),
    ("🔍️scale-selection", "🚫️rejects-ghosts", {"mutation": "scaleSelection", "targets": ["node-ghost", "region-ghost"], "pivotX": 0.0, "pivotY": 0.0, "factor": 2.0}, "Every target is absent: Error-level `mutation.target-missing`, nothing scales."),
    ("🔍️scale-selection", "⏸️keeps-a-unit-factor", {"mutation": "scaleSelection", "targets": ["node-a"], "pivotX": 20.0, "pivotY": 10.0, "factor": 1.0}, "A unit factor is a Warning-level `mutation.no-op`: the default diff, nothing to undo."),
]


TEMPLATE_CATALOG = {"edges": [], "handles": [], "nodes": [{"abstract": False, "attributes": [], "authors": [], "baseKinds": [], "description": "Capsule", "handles": [{"angle": 0.0, "description": "Door", "icon": "door", "id": "template-door", "label": "Door", "name": "door", "t": 1.5}], "icon": "capsule", "id": "kind-capsule", "image": "", "label": "Capsule", "name": "capsule", "representations": [], "unit": "mm"}], "wires": []}

INVARIANTS = [
    ("✋️drag-selection", "🧱️no-targets", {"mutation": "dragSelection", "targets": [], "dx": 1.0, "dy": 1.0}, [], "An empty target set is what the schema's `minItems: 1` forbids: a Fatal `mutation.invariant`, nothing moves."),
    ("🔄️rotate-selection", "🧱️repeated-targets", {"mutation": "rotateSelection", "targets": ["node-a", "node-b", "node-a"], "pivotX": 0.0, "pivotY": 0.0, "angle": QUARTER}, ["node-a", "node-b", "node-a"], "A target named twice is what the schema's `uniqueItems` forbids: a Fatal `mutation.invariant`, nothing turns."),
    ("🔍️scale-selection", "🧱️zero-factor", {"mutation": "scaleSelection", "targets": ["node-a"], "pivotX": 20.0, "pivotY": 10.0, "factor": 0.0}, ["node-a"], "A zero factor would collapse the selection onto the pivot; the schema's `exclusiveMinimum: 0` forbids it: a Fatal `mutation.invariant`."),
    ("🔍️scale-selection", "⛔️negative-factor", {"mutation": "scaleSelection", "targets": ["node-a"], "pivotX": 20.0, "pivotY": 10.0, "factor": -1.0}, ["node-a"], "A negative factor would mirror the selection through the pivot; the schema's `exclusiveMinimum: 0` forbids it: a Fatal `mutation.invariant`."),
    ("📏scale-node", "🧱️zero-scale", {"mutation": "scaleNode", "id": "node-a", "newScale": 0.0}, ["node-a"], "A zero node scale is what the schema's `exclusiveMinimum: 0` forbids: a Fatal `mutation.invariant`, the node keeps its size."),
    ("🧊replace-node-geometry", "🧱️negative-radius", {"mutation": "replaceNodeGeometry", "id": "node-a", "newShape": "circle", "newRadius": -4.0, "newWidth": None, "newHeight": None}, ["node-a"], "A negative radius is what the schema's `exclusiveMinimum: 0` forbids: a Fatal `mutation.invariant`, the node keeps its extent."),
    ("🌱create-node", "🧱️zero-width-node", {"mutation": "createNode", "node": {"anchor": "fixed", "handles": [], "height": 5.0, "id": "node-d", "shape": "rectangle", "width": 0.0, "x": 1.0, "y": 2.0}, "index": None}, ["node-d"], "A rectangle of zero width is what the node record's `exclusiveMinimum: 0` forbids: a Fatal `mutation.invariant`, no node is added."),
    ("➕add-node-handle", "🧱️zero-radius-handle", {"mutation": "addNodeHandle", "nodeId": "node-a", "handle": {"angle": 0.5, "id": "handle-new", "radius": 0.0}, "index": None}, ["node-a", "handle-new"], "A handle of zero radius is what the handle record's `exclusiveMinimum: 0` forbids: a Fatal `mutation.invariant`, no handle is added."),
    ("🔌replace-node-handle", "🧱️negative-scale", {"mutation": "replaceNodeHandle", "nodeId": "node-a", "handleId": "handle-1", "newHandle": {"angle": 0.0, "handleKind": "handle-kind-a", "id": "handle-1", "scale": -1.0}}, ["node-a", "handle-1"], "A negative handle scale is what the handle record's `exclusiveMinimum: 0` forbids: a Fatal `mutation.invariant`, the handle stays."),
    ("📚replace-kind-catalogs", "🧱️off-rim-template", {"mutation": "replaceKindCatalogs", "newCatalogs": TEMPLATE_CATALOG}, [], "A handle template whose rim parameter `t` is 1.5 lies off the node outline; the schema bounds `t` to 0..1: a Fatal `mutation.invariant`, no catalogue is installed."),
]


def transform_of(payload):
    if payload["mutation"] == "dragSelection":
        return drag(payload["dx"], payload["dy"])
    if payload["mutation"] == "rotateSelection":
        return rotate(payload["pivotX"], payload["pivotY"], payload["angle"])
    return scale(payload["pivotX"], payload["pivotY"], payload["factor"])


def dump(path, value):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        if value is None:
            return
        handle.write(json.dumps(value, indent=2, ensure_ascii=False, sort_keys=True) + "\n")


def kind_of(leaf):
    return next(leaf[at:] for at, character in enumerate(leaf) if character.isascii() and character.isalpha())


def slug_of(case):
    return case.split("️", 1)[1]


RUST_HEADER = '''//! 🧪️ `{kind}` fixture — `{case}`.
//!
//! {description}
//!
//! Source of truth is the committed JSON quintet under `🧫️fixtures/🧬️mutations/{leaf}/{case}/`
//! (contract D1); the board is the synthetic selection board shared by every selection-transform vector.

use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{{apply_puzzle2d_mutation, inverse_puzzle2d_mutation}};
use crate::Puzzle2dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{leaf}/{case}/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{leaf}/{case}/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{leaf}/{case}/🦠️mutation/🔣️.json");
{diff_const}
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{leaf}/{case}/🎯️outcome/🔣️.json");

fn before() -> Puzzle2dSnapshot {{
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}}
fn expected_after() -> Puzzle2dSnapshot {{
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}}
fn mutation() -> Puzzle2dMutation {{
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}}
fn outcome() -> serde_json::Value {{
    serde_json::from_str(OUTCOME).expect("outcome decodes")
}}

/// 🗣️ `(level, code, target)` of every message `{kind}` raises on the committed base.
fn produced_messages() -> Vec<(protocol::Severity, String, Vec<String>)> {{
    let produced = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &before());
    produced.messages().iter().map(|message| (message.level, message.code.0.clone(), message.target.clone())).collect()
}}

/// 🔣️ Both committed snapshots and the committed `{kind}` payload are already canonical.
#[test]
fn committed_json_is_canonical() {{
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {{
        let decoded: Puzzle2dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "{kind}/{slug}: committed {{label}} JSON is not canonical");
    }}
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "{kind}/{slug}: committed mutation JSON is not canonical");
}}
'''

DECLARED_MESSAGES = '''
/// 📜️ `(level, code, target)` of every message the committed outcome declares.
fn declared_messages() -> Vec<(protocol::Severity, String, Vec<String>)> {{
    let level = |text: &str| match text {{
        "info" => protocol::Severity::Info,
        "warning" => protocol::Severity::Warning,
        "error" => protocol::Severity::Error,
        "fatal" => protocol::Severity::Fatal,
        other => panic!("{kind}/{slug}: unknown message level {{other:?}}"),
    }};
    let strings = |value: &serde_json::Value| value.as_array().expect("an array of strings").iter().map(|entry| entry.as_str().expect("a string").to_string()).collect::<Vec<_>>();
    outcome().get("messages").and_then(serde_json::Value::as_array).map_or_else(Vec::new, |messages| {{
        messages.iter().map(|message| (level(message["level"].as_str().expect("a level")), message["code"].as_str().expect("a code").to_string(), strings(&message["target"]))).collect()
    }})
}}

/// 🔺️ The sparse delta `{kind}` produces is exactly the committed diff: WHICH records it patches, and
/// every patched record whole.
#[test]
fn produces_committed_diff() {{
    let outcome = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "{kind}/{slug}: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert!(committed["edges"].is_null() && committed["meta"].is_null() && committed["camera"].is_null(), "{kind}/{slug}: a selection transform touches neither edges, meta nor camera");
}}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after`.
#[test]
fn committed_diff_applies_to_after() {{
    let decoded: Puzzle2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <Puzzle2dDiff as protocol::MutationDiff<Puzzle2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "{kind}/{slug}: committed diff did not carry before to after");
}}
'''

APPLIED = '''
/// ▶️ The committed payload carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {{
    let mut snapshot = before();
    apply_puzzle2d_mutation(&mut snapshot, &mutation()).expect("{kind} applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "{kind}/{slug}: applied state differs from committed after-snapshot");
    assert_ne!(snapshot, before(), "{kind}/{slug}: an applied vector must move the board");
}}

/// ↩️ Applying the payload then the inverse it derives from `before` restores `before` EXACTLY — the
/// inverse is absolute setters read off the base, never a negated parameter.
#[test]
fn inverse_restores_before() {{
    let base = before();
    let mutation = mutation();
    let inverse = inverse_puzzle2d_mutation(&base, &mutation);
    assert!(!inverse.is_empty(), "{kind}/{slug}: a moving vector must have something to undo");
    let mut snapshot = base.clone();
    apply_puzzle2d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in inverse.iter().rev() {{
        apply_puzzle2d_mutation(&mut snapshot, step).expect("inverse step applies");
    }}
    assert_eq!(snapshot, base, "{kind}/{slug}: inverse did not restore the before-snapshot");
}}

/// 🎯️ The declared outcome — `applied`, with exactly the declared warnings — is what `{kind}` emits.
#[test]
fn declared_outcome_holds() {{
    assert_eq!(outcome()["status"].as_str(), Some("applied"), "{kind}/{slug} declares an applied outcome");
    assert_eq!(produced_messages(), declared_messages(), "{kind}/{slug}: the produced messages differ from the declared ones");
}}

/// 🧾️ The committed diff is itself canonical and decodes to `Puzzle2dDiff`.
#[test]
fn committed_diff_is_canonical() {{
    let decoded: Puzzle2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "{kind}/{slug}: committed diff JSON is not canonical");
}}
'''

NO_OP = '''
/// ⏸️ A no-op still applies cleanly and leaves the board byte-identical.
#[test]
fn applies_to_committed_after() {{
    let mut snapshot = before();
    apply_puzzle2d_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "{kind}/{slug}: applied state differs from committed after-snapshot");
    assert_eq!(expected_after(), before(), "{kind}/{slug}: a no-op vector's two committed snapshots must be identical");
}}

/// 🎯️ The declared no-op is exactly what `{kind}` emits: a Warning-level `mutation.no-op` and the default diff.
#[test]
fn declared_outcome_holds() {{
    assert_eq!(outcome()["status"].as_str(), Some("no-op"), "{kind}/{slug} declares a no-op outcome");
    assert_eq!(produced_messages(), declared_messages(), "{kind}/{slug}: the produced messages differ from the declared ones");
    let produced = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &Puzzle2dDiff::default(), "{kind}/{slug}: a no-op answers the default diff");
}}

/// ↩️ Nothing moved, so nothing is undone.
#[test]
fn inverse_is_empty() {{
    assert!(inverse_puzzle2d_mutation(&before(), &mutation()).is_empty(), "{kind}/{slug}: a no-op must yield no inverse step");
}}
'''

INVARIANT = '''
/// ▶️ A refused `{kind}` still applies cleanly — its diff is the default one — and leaves the board at
/// the committed `after`, which is the committed `before`.
#[test]
fn refusal_leaves_the_document_at_the_committed_after() {{
    let mut snapshot = before();
    apply_puzzle2d_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "{kind}/{slug}: applied state differs from committed after-snapshot");
    assert_eq!(expected_after(), before(), "{kind}/{slug}: a rejected vector's two committed snapshots must be identical");
}}

/// 🧱️ The payload breaks a hard bound of its own schema, so the refusal is exactly one Fatal
/// `mutation.invariant` addressing the declared path, with the default diff.
#[test]
fn the_invariant_is_the_declared_refusal() {{
    assert!(DIFF_ABSENT.is_empty(), "{kind}/{slug}: the D6 sentinel 🔺️diff/🚫️.absent must stay empty");
    let produced = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &Puzzle2dDiff::default(), "{kind}/{slug}: a Fatal outcome carries the default diff");
    let outcome = outcome();
    assert_eq!(outcome["status"].as_str(), Some("rejected"), "{kind}/{slug} declares a rejected outcome");
    assert_eq!(outcome["code"].as_str(), Some("mutation.invariant"), "{kind}/{slug} declares the invariant refusal");
    let path: Vec<String> = outcome["path"].as_array().expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(produced_messages(), vec![(protocol::Severity::Fatal, "mutation.invariant".to_string(), path)], "{kind}/{slug}: the refusal differs from the declared one");
}}

/// 🌐️ The refusal does not depend on the board: the empty board refuses the same payload the same way.
#[test]
fn the_invariant_is_independent_of_the_base() {{
    let produced = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &Puzzle2dSnapshot::default());
    assert_eq!(produced.messages().iter().map(|message| (message.level, message.code.0.as_str())).collect::<Vec<_>>(), vec![(protocol::Severity::Fatal, "mutation.invariant")], "{kind}/{slug}: an invariant is a property of the payload alone");
}}
'''

REJECTED = '''
/// ▶️ A refused `{kind}` still applies cleanly — its diff is the default one — and leaves the board at
/// the committed `after`, which is the committed `before`.
#[test]
fn rejection_leaves_the_document_at_the_committed_after() {{
    let mut snapshot = before();
    apply_puzzle2d_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "{kind}/{slug}: applied state differs from committed after-snapshot");
    assert_eq!(expected_after(), before(), "{kind}/{slug}: a rejected vector's two committed snapshots must be identical");
}}

/// 🚨️ The refusal is exactly the declared one: default diff, one Error-level message, its code and
/// every target the payload named.
#[test]
fn the_refusal_is_the_declared_one() {{
    assert!(DIFF_ABSENT.is_empty(), "{kind}/{slug}: the D6 sentinel 🔺️diff/🚫️.absent must stay empty");
    let produced = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &Puzzle2dDiff::default(), "{kind}/{slug}: a refusing diff builder answers the default diff");
    let outcome = outcome();
    assert_eq!(outcome["status"].as_str(), Some("rejected"), "{kind}/{slug} declares a rejected outcome");
    let path: Vec<String> = outcome["path"].as_array().expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(produced_messages(), vec![(protocol::Severity::Error, outcome["code"].as_str().expect("a code").to_string(), path)], "{kind}/{slug}: the refusal differs from the declared one");
}}

/// ↩️ Nothing moved, so nothing is undone.
#[test]
fn inverse_of_a_refusal_is_empty() {{
    assert!(inverse_puzzle2d_mutation(&before(), &mutation()).is_empty(), "{kind}/{slug}: a refusal must yield no inverse step");
}}
'''


def rust_test(leaf, case, description, status, invariant=False):
    kind, slug = kind_of(leaf), slug_of(case)
    fields = {"kind": kind, "slug": slug, "leaf": leaf, "case": case, "description": description}
    if invariant:
        fields["diff_const"] = 'const DIFF_ABSENT: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/%s/%s/🔺️diff/🚫️.absent");' % (leaf, case)
        return (RUST_HEADER.format(**fields) + INVARIANT.format(**fields)).replace("mutations::{apply_puzzle2d_mutation, inverse_puzzle2d_mutation};", "mutations::apply_puzzle2d_mutation;")
    if status == "rejected":
        fields["diff_const"] = 'const DIFF_ABSENT: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/%s/%s/🔺️diff/🚫️.absent");' % (leaf, case)
        return RUST_HEADER.format(**fields) + REJECTED.format(**fields)
    fields["diff_const"] = 'const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/%s/%s/🔺️diff/🔣️.json");' % (leaf, case)
    body = APPLIED if status == "applied" else NO_OP
    return RUST_HEADER.format(**fields) + DECLARED_MESSAGES.format(**fields) + body.format(**fields)


def main():
    written = []
    for leaf, case, payload, description in CASES:
        directory = os.path.join(FIXTURES, leaf, case)
        board = copy.deepcopy(BOARD)
        result, outcome = outcome_of(board, payload["targets"], transform_of(payload))
        dump(os.path.join(directory, "📸️snapshot", "⬅️before", "🔣️.json"), board)
        dump(os.path.join(directory, "🦠️mutation", "🔣️.json"), payload)
        dump(os.path.join(directory, "🎯️outcome", "🔣️.json"), outcome)
        if result is None:
            dump(os.path.join(directory, "📸️snapshot", "➡️after", "🔣️.json"), board)
            dump(os.path.join(directory, "🔺️diff", "🚫️.absent"), None)
        else:
            after, diff = result
            dump(os.path.join(directory, "📸️snapshot", "➡️after", "🔣️.json"), after)
            dump(os.path.join(directory, "🔺️diff", "🔣️.json"), diff)
        test = os.path.join(LEAVES, leaf, "🧪️tests", case, "🦀️.rs")
        os.makedirs(os.path.dirname(test), exist_ok=True)
        with open(test, "w", encoding="utf-8") as handle:
            handle.write(rust_test(leaf, case, description, outcome["status"]))
        written.append((leaf, case, outcome["status"]))
    for leaf, case, payload, path, description in INVARIANTS:
        directory = os.path.join(FIXTURES, leaf, case)
        board = copy.deepcopy(BOARD)
        for side in ("⬅️before", "➡️after"):
            dump(os.path.join(directory, "📸️snapshot", side, "🔣️.json"), board)
        dump(os.path.join(directory, "🦠️mutation", "🔣️.json"), payload)
        dump(os.path.join(directory, "🎯️outcome", "🔣️.json"), {"status": "rejected", "code": "mutation.invariant", "path": path})
        dump(os.path.join(directory, "🔺️diff", "🚫️.absent"), None)
        test = os.path.join(LEAVES, leaf, "🧪️tests", case, "🦀️.rs")
        os.makedirs(os.path.dirname(test), exist_ok=True)
        with open(test, "w", encoding="utf-8") as handle:
            handle.write(rust_test(leaf, case, description, "rejected", invariant=True))
        written.append((leaf, case, "invariant"))
    catalog_path = os.path.join(FIXTURES, "🔣️.json")
    catalog = json.load(open(catalog_path, encoding="utf-8"))
    files = []
    for directory, _dirs, names in os.walk(FIXTURES):
        for name in names:
            relative = os.path.relpath(os.path.join(directory, name), FIXTURES)
            if relative == "🔣️.json":
                continue
            files.append({"path": relative, "sha256": hashlib.sha256(open(os.path.join(directory, name), "rb").read()).hexdigest()})
    catalog["files"] = sorted(files, key=lambda entry: entry["path"])
    with open(catalog_path, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(catalog, indent=2, ensure_ascii=False) + "\n")
    for row in written:
        print(*row)
    print("catalog files", len(catalog["files"]))


if __name__ == "__main__":
    main()
