"""🧲️ Design §22.13 landing wave (S5-PUZZLE): a recorded proximity `connect-handles` states its precondition.

Applies the puzzle 2d half as exact, anchored text hunks, all or nothing: every anchor must resolve exactly once (or its
replacement must already be present) before any file is written. `--check` writes nothing and prints the pending hunks;
`--stage <dir>` writes the result into a mirror tree under `<dir>` instead of the repo (a dry run of the non-Rust parts).
The framework half (`mutation.precondition-drifted` in the outcome vocabulary) lands first, by hand, under the fleet lock.

Run: `python3 <this file> [--check | --stage <dir>]`.
"""

import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[7]
P2 = "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d"
ANY = f"{P2}/🏅️standards/🔖️1/🪆️subsets/✳️any"
MUTATIONS = f"{ANY}/🧬️schema/🧬️mutations"
LEAF = f"{MUTATIONS}/🪢️connect-handles"
EDITOR = f"{ANY}/✏️editor"
SELECT = f"{EDITOR}/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖱️select/🦀️.rs"

HUNKS = []


def hunk(path, old, new):
    HUNKS.append((path, old, new))


# 🧬️ Schema first: the leaf payload states the new input and its control.
hunk(
    f"{LEAF}/🧬️schema/🔣️.json",
    """        "group": "appearance",
        "order": 140
      }
    }
  }
}""",
    """        "group": "appearance",
        "order": 140
      }
    },
    "tolerance": {
      "type": [
        "number",
        "null"
      ],
      "minimum": 0,
      "x-semio-ui": {
        "widget": "stepper",
        "role": "value",
        "label": {
          "en": "Proximity tolerance",
          "de": "Näherungstoleranz"
        },
        "description": {
          "en": "Largest distance between the two handles this connection was recorded under. Empty for a connection that states no proximity.",
          "de": "Größter Abstand der beiden Anschlüsse, unter dem diese Verbindung aufgezeichnet wurde. Leer bei einer Verbindung ohne Näherungsbedingung."
        },
        "step": 1,
        "precision": 2,
        "softMin": 0,
        "softMax": 96,
        "group": "precondition",
        "order": 150
      }
    }
  }
}""",
)

hunk(
    f"{MUTATIONS}/🟦️.ts",
    """  sourceTip: string | null;
  targetTip: string | null;
}

/** ✂️ `disconnect-handles` payload. */""",
    """  sourceTip: string | null;
  targetTip: string | null;
  tolerance?: number | null;
}

/** ✂️ `disconnect-handles` payload. */""",
)

hunk(
    f"{MUTATIONS}/📝️text/📖️.grammar.semio",
    'connect-handles = "connect-handles" SP id SP id SP id SP edge-fields\n',
    'connect-handles = "connect-handles" SP id SP id SP id SP edge-fields SP number-opt\n',
)
hunk(
    f"{MUTATIONS}/📝️text/🔤️.ebnf",
    "connect handles = 'connect-handles', space, id, space, id, space, id, space, edge fields ;\n",
    "connect handles = 'connect-handles', space, id, space, id, space, id, space, edge fields, space, number opt ;\n",
)
hunk(
    f"{MUTATIONS}/📝️text/🅰️.g4",
    "connectHandles: 'connect-handles' SP id SP id SP id SP edgeFields ;\n",
    "connectHandles: 'connect-handles' SP id SP id SP id SP edgeFields SP numberOpt ;\n",
)

# 🦀️ The leaf: payload, builders, diff.
hunk(
    f"{LEAF}/🦀️.rs",
    """//! initial connection-parameterization payload included (rule 4: `connect-<nouns>{endpoints,
//! payload}`).
""",
    """//! initial connection-parameterization payload included (rule 4: `connect-<nouns>{endpoints,
//! payload}`). A connection a drop records from proximity also states its precondition, the `tolerance` its two
//! handles lay within: replayed on a base where they no longer do, it still connects and reports
//! `mutation.precondition-drifted` (design
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §22.13).
""",
)
hunk(
    f"{LEAF}/🦀️.rs",
    """/// connection-parameter payload (`edge_kind`/`gap`/`shift`/`rise`/`rotation`/`turn`/`tilt`/`x`/`y`/
/// `source_tip`/`target_tip`).
""",
    """/// connection-parameter payload (`edge_kind`/`gap`/`shift`/`rise`/`rotation`/`turn`/`tilt`/`x`/`y`/
/// `source_tip`/`target_tip`), and the proximity `tolerance` a drop recorded it under (`None`: no precondition).
""",
)
hunk(
    f"{LEAF}/🦀️.rs",
    """    pub source_tip: Option<String>,
    pub target_tip: Option<String>,
}
""",
    """    pub source_tip: Option<String>,
    pub target_tip: Option<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub tolerance: Option<f64>,
}
""",
)
hunk(
    f"{LEAF}/🦀️.rs",
    """    Puzzle2dMutation::ConnectHandles(ConnectHandles { id, source, target, edge_kind, gap, shift, rise, rotation, turn, tilt, x, y, source_tip, target_tip })
}
""",
    """    Puzzle2dMutation::ConnectHandles(ConnectHandles { id, source, target, edge_kind, gap, shift, rise, rotation, turn, tilt, x, y, source_tip, target_tip, tolerance: None })
}

/// 🧲️ Builder — the connection a drop records from proximity: the default geometry and the `tolerance` its two
/// handles lay within when it was recorded.
pub fn connect_handles_in_proximity(id: String, source: String, target: String, tolerance: f64) -> Puzzle2dMutation {
    Puzzle2dMutation::ConnectHandles(ConnectHandles { id, source, target, edge_kind: None, gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 0.0, source_tip: None, target_tip: None, tolerance: Some(tolerance) })
}
""",
)

hunk(
    f"{LEAF}/🔺️diff/🦀️.rs",
    """//! 🔺️ Sparse diff builder for `ConnectHandles` — a real append-only insert (never a
//! whole-snapshot capture). No-op when the id already exists in `base`.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle2dDiff, Puzzle2dEdgesDelta};
use crate::{Puzzle2dEdge, Puzzle2dSnapshot};
use crate::standards::v1::subsets::any::schema::mutations::puzzle2d_finite;
""",
    """//! 🔺️ Sparse diff builder for `ConnectHandles` — a real append-only insert (never a
//! whole-snapshot capture). No-op when the id already exists in `base`. A payload that states a `tolerance` connects
//! on any base and warns `mutation.precondition-drifted` where its two handles are no longer within it.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle2dDiff, Puzzle2dEdgesDelta};
use crate::{Puzzle2dEdge, Puzzle2dSnapshot};
use crate::standards::v1::subsets::any::schema::mutations::{puzzle2d_finite, puzzle2d_handle_distance};
""",
)
hunk(
    f"{LEAF}/🔺️diff/🦀️.rs",
    """    if let Err(reason) = puzzle2d_finite(&[("gap", payload.gap), ("shift", payload.shift), ("rise", payload.rise), ("rotation", payload.rotation), ("turn", payload.turn), ("tilt", payload.tilt), ("x", payload.x), ("y", payload.y)]) {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, vec![payload.id.clone()]);
    }
""",
    """    if let Err(reason) = puzzle2d_finite(&[("gap", payload.gap), ("shift", payload.shift), ("rise", payload.rise), ("rotation", payload.rotation), ("turn", payload.turn), ("tilt", payload.tilt), ("x", payload.x), ("y", payload.y), ("tolerance", payload.tolerance.unwrap_or(0.0))]) {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, vec![payload.id.clone()]);
    }
    if payload.tolerance.is_some_and(|tolerance| tolerance < 0.0) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "tolerance must not be negative", vec![payload.id.clone()]);
    }
""",
)
hunk(
    f"{LEAF}/🔺️diff/🦀️.rs",
    """    protocol::MutationOutcome::new(Puzzle2dDiff { edges: Some(Puzzle2dEdgesDelta { added: vec![edge], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
""",
    """    protocol::MutationOutcome::new(Puzzle2dDiff { edges: Some(Puzzle2dEdgesDelta { added: vec![edge], ..Default::default() }), ..Default::default() }).absorb_messages(drift(payload, base))
}

/// 🧲️ The warning of a recorded proximity that no longer holds on `base`: the payload states a `tolerance`, and its two
/// handles lie farther apart than it — or one of them is on no node. Names both handles.
fn drift(payload: &super::ConnectHandles, base: &Puzzle2dSnapshot) -> Option<protocol::MutationMessage> {
    let tolerance = payload.tolerance?;
    let reason = match puzzle2d_handle_distance(base, &payload.source, &payload.target) {
        Some(distance) if distance <= tolerance => return None,
        Some(distance) => format!("handles {:?} and {:?} are {distance} apart, beyond the tolerance {tolerance} their connection was recorded under", payload.source, payload.target),
        None => format!("handles {:?} and {:?} are not both on a node, so the tolerance {tolerance} their connection was recorded under cannot hold", payload.source, payload.target),
    };
    Some(protocol::MutationMessage::warning(protocol::OutcomeCode::PreconditionDrifted, reason).at(vec![payload.source.clone(), payload.target.clone()]))
}
//#endregion 🔖️Diff
""",
)

# 🧮️ The aggregate: the typed rim geometry the diff and the select tool share, and the new builder's export.
hunk(
    f"{MUTATIONS}/🦀️.rs",
    "pub use super::connect_handles::{connect_handles, ConnectHandles};\n",
    "pub use super::connect_handles::{connect_handles, connect_handles_in_proximity, ConnectHandles};\n",
)
hunk(
    f"{MUTATIONS}/🦀️.rs",
    """//#endregion 🔖️Invariants

//#region 🔖️SnapshotDelta
""",
    """//#endregion 🔖️Invariants

//#region 🔖️HandleGeometry
/// 📍️ The board position of the handle `handle_id` on `document`: its node's rim point at the handle's angle, the
/// same rim geometry the board engine draws with and the editor's proximity search measures
/// (`puzzle2d_handle_world_position`) — a circle's east-zero angle on the node's radius, a rectangle's north-zero
/// angle on its outline. `None` when no node carries the handle.
pub fn puzzle2d_handle_position(document: &Puzzle2dSnapshot, handle_id: &str) -> Option<(f64, f64)> {
    let (node, handle) = document.nodes.iter().find_map(|node| node.handles.iter().find(|handle| handle.id == handle_id).map(|handle| (node, handle)))?;
    let centre = semio_framework_geometry::Point::new(node.x, node.y);
    let point = if node.shape.as_deref() == Some("rectangle") {
        semio_framework_graph::drawing::routing::handle_position_on_rectangle(centre, node.width.unwrap_or(48.0), node.height.unwrap_or(48.0), handle.angle)
    } else {
        semio_framework_graph::drawing::routing::handle_position_on_circle(centre, node.radius.unwrap_or(24.0), handle.angle)
    };
    Some((point.x, point.y))
}

/// 📏️ The board distance between the handles `source` and `target` of `document` — what a recorded proximity
/// tolerance is measured against. `None` when either handle is on no node.
pub fn puzzle2d_handle_distance(document: &Puzzle2dSnapshot, source: &str, target: &str) -> Option<f64> {
    let ((source_x, source_y), (target_x, target_y)) = (puzzle2d_handle_position(document, source)?, puzzle2d_handle_position(document, target)?);
    Some((target_x - source_x).hypot(target_y - source_y))
}
//#endregion 🔖️HandleGeometry

//#region 🔖️SnapshotDelta
""",
)

# 📦️ The rim geometry's point type is a framework geometry type: the dependency the graph crate already pulls in.
hunk(
    f"{P2}/📦️packages/🦀️rust/Cargo.toml",
    'component-app-assembly = ["semio-framework-plugin/component-guest", "dep:machine", "dep:semio-framework-async", "dep:semio-framework-geometry", "dep:semio-framework-os-infinite",',
    'component-app-assembly = ["semio-framework-plugin/component-guest", "dep:machine", "dep:semio-framework-async", "dep:semio-framework-os-infinite",',
)
hunk(
    f"{P2}/📦️packages/🦀️rust/Cargo.toml",
    "semio-framework-geometry = { workspace = true, optional = true }\n",
    "semio-framework-geometry = { workspace = true }\n",
)

# 🛠️ The select tool: every connection a drop lands carries the tolerance it was recorded under.
hunk(
    SELECT,
    """//! and leaves as ONE `ToolTransaction`: the parametric `drag-`/`rotate-`/`scale-selection` leaf plus the
//! `connect-handles` its drop lands, targets and edge ids literal. Tool state is never history; the yielded
""",
    """//! and leaves as ONE `ToolTransaction`: the parametric `drag-`/`rotate-`/`scale-selection` leaf plus the
//! `connect-handles` its drop lands, targets and edge ids literal, each stating the proximity tolerance it was
//! recorded under (design §22.13). Tool state is never history; the yielded
""",
)
hunk(
    SELECT,
    "use crate::standards::v1::subsets::any::schema::mutations::{apply_puzzle2d_mutation, connect_handles, drag_selection, rotate_selection, scale_selection, Puzzle2dMutation};\n",
    "use crate::standards::v1::subsets::any::schema::mutations::{apply_puzzle2d_mutation, connect_handles_in_proximity, drag_selection, puzzle2d_handle_distance, rotate_selection, scale_selection, Puzzle2dMutation};\n",
)
hunk(
    SELECT,
    """/// search over the moved state — until the gesture's [`PUZZLE2D_PROXIMITY_GESTURE_MAX`] budget is spent. Every
/// edge id is minted HERE, deterministically from its pair and the document, so a replay never mints again.
""",
    """/// search over the moved state — until the gesture's [`PUZZLE2D_PROXIMITY_GESTURE_MAX`] budget is spent. Every
/// edge id is minted HERE, deterministically from its pair and the document, so a replay never mints again. Every
/// connection states the tolerance it was recorded under: `radius`, widened to the two handles' distance on the
/// moved state where the board paired them from farther away — the precondition a replay checks instead of searching
/// again.
""",
)
hunk(
    SELECT,
    """            let id = puzzle2d_minted_edge_id(&state, &source, &target);
            let connect = connect_handles(id.clone(), source, target, None, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None, None);
""",
    """            let id = puzzle2d_minted_edge_id(&state, &source, &target);
            let tolerance = puzzle2d_handle_distance(&state, &source, &target).map_or(radius.max(0.0), |distance| distance.max(radius));
            let connect = connect_handles_in_proximity(id.clone(), source, target, tolerance);
""",
)

# 🧪️ Laws: the leaf on its own, then the tool's yields.
hunk(
    f"{MUTATIONS}/🧪️tests/🔬️unit/🦀️.rs",
    """#[test]
fn delete_node_severs_and_reconnects_edges() {
""",
    """/// 🧲️ LAW (design §22.13): a connection that states the tolerance it was recorded under connects on ANY base. Within
/// the tolerance it reports nothing; once a handle moved away, or is on no node, it still adds its edge and warns
/// `mutation.precondition-drifted` naming both handles; a negative or non-finite tolerance is what the schema forbids.
#[test]
fn a_recorded_proximity_connect_warns_once_its_handles_drift_apart() {
    use crate::{Puzzle2dHandle, Puzzle2dNode};
    let node = |id: &str, x: f64, handle: &str, angle: f64| Puzzle2dNode { id: id.into(), x, radius: Some(24.0), handles: vec![Puzzle2dHandle { id: handle.into(), angle, ..Default::default() }], ..Default::default() };
    let mut base = empty_puzzle2d_snapshot();
    base.nodes = vec![node("a", 0.0, "ha", 0.0), node("b", 56.0, "hb", std::f64::consts::PI)];
    assert_eq!(puzzle2d_handle_position(&base, "ha"), Some((24.0, 0.0)), "a circle's handle sits on the rim at its east-zero angle");
    let apart = puzzle2d_handle_distance(&base, "ha", "hb").expect("both handles are on a node");
    assert!((apart - 8.0).abs() < 1e-9, "{apart}");
    assert_eq!(puzzle2d_handle_distance(&base, "ha", "ghost"), None);
    let recorded = connect_handles_in_proximity("e1".into(), "ha".into(), "hb".into(), 12.0);
    let near = recorded.diff(&base);
    assert!(near.messages().is_empty(), "within the tolerance nothing is reported: {:?}", near.messages());
    ::semio_framework_async::poll::resolve_ready(assert_mutation_inverse_law(&base, &recorded));
    let mut moved = base.clone();
    moved.nodes[1].x = 300.0;
    let mut gone = base.clone();
    gone.nodes.remove(1);
    for (state, what) in [(&moved, "moved away"), (&gone, "on no node")] {
        let outcome = recorded.diff(state);
        let reported: Vec<(semio_framework_diagnostic::Severity, &str, Vec<String>)> = outcome.messages().iter().map(|message| (message.level, message.code.0.as_str(), message.target.clone())).collect();
        assert_eq!(reported, vec![(semio_framework_diagnostic::Severity::Warning, "mutation.precondition-drifted", vec!["ha".to_string(), "hb".to_string()])], "{what}");
        assert!(outcome.messages()[0].message.contains("\\"ha\\"") && outcome.messages()[0].message.contains("\\"hb\\""), "{what}: the words name both handles: {}", outcome.messages()[0].message);
        let connected = MutationDiff::<Puzzle2dSnapshot>::apply(outcome.diff(), state).expect("a drifted connection still applies");
        assert!(connected.edges.iter().any(|edge| edge.id == "e1" && edge.source == "ha" && edge.target == "hb"), "{what}: the edge is there");
    }
    let unconditional = connect_handles("e1".into(), "ha".into(), "hb".into(), None, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None, None);
    assert!(unconditional.diff(&moved).messages().is_empty(), "a connection that states no tolerance has no precondition");
    for forbidden in [-1.0, f64::NAN, f64::INFINITY] {
        let outcome = connect_handles_in_proximity("e1".into(), "ha".into(), "hb".into(), forbidden).diff(&base);
        ::semio_framework_async::poll::resolve_ready(assert_fatal_never_applies(&outcome));
        assert_eq!(outcome.messages()[0].code.0, "mutation.invariant", "tolerance {forbidden}");
    }
    let line = <Puzzle2dMutation as protocol::OpText>::print_op(&recorded);
    assert_eq!(<Puzzle2dMutation as protocol::OpText>::parse_op(&line).expect("the text form parses"), recorded, "{line}");
    let bytes = <Puzzle2dMutation as protocol::OpBinary>::encode_op(&recorded).expect("the binary form encodes");
    assert_eq!(<Puzzle2dMutation as protocol::OpBinary>::decode_op(&bytes).expect("the binary form decodes"), recorded);
    let value = semio_framework_value::ToValue::to_value(&unconditional);
    assert!(serde_json::Value::from(value).get("tolerance").is_none(), "a connection with no precondition states none");
}

#[test]
fn delete_node_severs_and_reconnects_edges() {
""",
)
hunk(
    f"{EDITOR}/🧪️tests/🧪️select-tool/🦀️.rs",
    """    let Puzzle2dMutation::ConnectHandles(ConnectHandles { id, source, target, .. }) = &yields[1].1 else { panic!("the drop lands a connection: {:?}", yields[1].1) };
    assert_eq!((id.as_str(), source.as_str(), target.as_str()), ("edge-left:v0-right:v0", "left:v0", "right:v0"), "the stationary peer is the source");
""",
    """    let Puzzle2dMutation::ConnectHandles(ConnectHandles { id, source, target, tolerance, .. }) = &yields[1].1 else { panic!("the drop lands a connection: {:?}", yields[1].1) };
    assert_eq!((id.as_str(), source.as_str(), target.as_str()), ("edge-left:v0-right:v0", "left:v0", "right:v0"), "the stationary peer is the source");
    assert_eq!(*tolerance, Some(12.0), "the connection states the radius its search found it within");
""",
)
hunk(
    f"{EDITOR}/🧪️tests/🧪️select-tool/🦀️.rs",
    """    assert_eq!(yields.iter().map(|(key, _)| key.as_str()).collect::<Vec<_>>(), vec!["selection:0", "connect:edge-right:v0-left:v0-2"], "the recorded pair connects as recorded, past the id the document already holds; a pair naming a missing handle is dropped");
""",
    """    assert_eq!(yields.iter().map(|(key, _)| key.as_str()).collect::<Vec<_>>(), vec!["selection:0", "connect:edge-right:v0-left:v0-2"], "the recorded pair connects as recorded, past the id the document already holds; a pair naming a missing handle is dropped");
    let Puzzle2dMutation::ConnectHandles(ConnectHandles { tolerance, .. }) = &yields[1].1 else { panic!("the recorded pair lands a connection: {:?}", yields[1].1) };
    let mut dropped = base.clone();
    apply_puzzle2d_mutation(&mut dropped, &yields[0].1).expect("the drag applies");
    assert_eq!(*tolerance, crate::standards::v1::subsets::any::schema::mutations::puzzle2d_handle_distance(&dropped, "right:v0", "left:v0"), "a pair the board recorded from farther than the radius states its recorded distance");
    assert!(yields[1].1.diff(&dropped).messages().is_empty(), "replayed where it was recorded, the connection reports nothing");
""",
)

# 🧫️ The runtime corpus: its schema, the scenario, the Rust law that drives it and the Python oracle that re-folds it.
CORPUS_SCHEMA = f"{EDITOR}/🧬️schema/🔣️history-edit-runtime/🔣️.json"
CORPUS = f"{EDITOR}/🧫️fixtures/🧫️history-edit-runtime/🔣️.json"
LAW = f"{EDITOR}/🧪️tests/🧪️history-edit-runtime/🦀️.rs"
ORACLE = f"{EDITOR}/🧪️tests/🧪️history-edit-runtime/🐍️.py"

hunk(
    CORPUS_SCHEMA,
    """      "properties": { "mutation": { "enum": ["dragSelection", "createNode", "changeNodeLocked"] } }""",
    """      "properties": { "mutation": { "enum": ["dragSelection", "createNode", "changeNodeLocked", "connectHandles"] } }""",
)
hunk(
    CORPUS_SCHEMA,
    """        "nodes": { "type": "object", "minProperties": 1, "additionalProperties": { "$ref": "#/definitions/Point" } },
        "absent": { "$ref": "#/definitions/Ids" }
      }
    },
""",
    """        "nodes": { "type": "object", "minProperties": 1, "additionalProperties": { "$ref": "#/definitions/Point" } },
        "absent": { "$ref": "#/definitions/Ids" },
        "edges": { "$ref": "#/definitions/Ids" },
        "outcomes": { "type": "array", "items": { "$ref": "#/definitions/Folded" } }
      }
    },
    "Folded": {
      "type": "object",
      "additionalProperties": false,
      "required": ["index", "code", "target"],
      "properties": {
        "index": { "type": "integer", "minimum": 0 },
        "code": { "type": "string", "pattern": "^mutation\\\\." },
        "target": { "$ref": "#/definitions/Ids" }
      }
    },
""",
)
hunk(
    CORPUS_SCHEMA,
    """        "row": { "type": "integer", "minimum": 0 },
        "worst": { "$ref": "#/definitions/Severity" },
""",
    """        "row": { "type": "integer", "minimum": 0 },
        "mutation": { "type": "integer", "minimum": 0 },
        "worst": { "$ref": "#/definitions/Severity" },
""",
)
hunk(
    CORPUS_SCHEMA,
    """        "head": { "type": "string" },
""",
    """        "head": { "type": "string" },
        "preview": { "type": "string" },
""",
)
hunk(
    CORPUS_SCHEMA,
    """            { "const": "nextProblem" },
            { "type": "object", "additionalProperties": false, "required": ["row"], "properties": { "row": { "type": "integer", "minimum": 0 } } }
""",
    """            { "const": "nextProblem" },
            { "type": "object", "additionalProperties": false, "required": ["row"], "properties": { "row": { "type": "integer", "minimum": 0 }, "mutation": { "type": "integer", "minimum": 0 } } }
""",
)
hunk(
    CORPUS_SCHEMA,
    """        "id": { "type": "string", "pattern": "^[a-z0-9-]+$" },
        "log":""",
    """        "id": { "type": "string", "pattern": "^[a-z0-9-]+$" },
        "board": { "$ref": "#/properties/board" },
        "log":""",
)

hunk(
    CORPUS,
    "One board; each scenario authors `log` (one edit per leaf, oldest first; `row` indices address these edits), edits it",
    "One board, unless a scenario states its own; each scenario authors `log` (the leaves of its edits, oldest first; `row` addresses an edit and `mutation` a leaf inside a tool transaction's edit), edits it",
)
hunk(
    CORPUS,
    "`heads` name documents: the board with `log` folded after replacing or dropping the leaves `edits` names; the Rust law",
    "`heads` name documents: the board with `log` folded after replacing or dropping the leaves `edits` names, with the `edges` it holds and the `outcomes` that fold reports (log index, code, target); a step's `head` is the committed document and its `preview` the document a review paints; the Rust law",
)
hunk(
    CORPUS,
    """        { "redo": null, "expect": { "head": "fixed", "alternative": "Fixed" } }
      ]
    }
  ]
}""",
    """        { "redo": null, "expect": { "head": "fixed", "alternative": "Fixed" } }
      ]
    },
    {
      "id": "a-drifted-proximity-connect-warns-and-is-withdrawn",
      "board": {
        "schema": "puzzle.2d.fixture",
        "nodes": [
          { "id": "left", "shape": "circle", "x": -200.0, "y": 0.0, "radius": 24.0, "text": "Left", "handles": [{ "id": "left-east", "angle": 0.0 }] },
          { "id": "mid", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": "Middle", "handles": [{ "id": "mid-west", "angle": 3.141592653589793 }] }
        ],
        "edges": []
      },
      "log": [
        { "mutation": "dragSelection", "targets": ["left"], "dx": 144.0, "dy": 0.0 },
        { "mutation": "connectHandles", "id": "edge-mid-west-left-east", "source": "mid-west", "target": "left-east", "edgeKind": null, "gap": 0.0, "shift": 0.0, "rise": 0.0, "rotation": 0.0, "turn": 0.0, "tilt": 0.0, "x": 0.0, "y": 0.0, "sourceTip": null, "targetTip": null, "tolerance": 12.0 }
      ],
      "heads": {
        "connected": { "edits": [], "nodes": { "left": { "x": -56.0, "y": 0.0 }, "mid": { "x": 0.0, "y": 0.0 } }, "edges": ["edge-mid-west-left-east"], "outcomes": [] },
        "drifted": {
          "edits": [{ "index": 0, "leaf": { "mutation": "dragSelection", "targets": ["left"], "dx": -100.0, "dy": 0.0 } }],
          "nodes": { "left": { "x": -300.0, "y": 0.0 }, "mid": { "x": 0.0, "y": 0.0 } },
          "edges": ["edge-mid-west-left-east"],
          "outcomes": [{ "index": 1, "code": "mutation.precondition-drifted", "target": ["mid-west", "left-east"] }]
        },
        "apart": {
          "edits": [{ "index": 0, "leaf": { "mutation": "dragSelection", "targets": ["left"], "dx": -100.0, "dy": 0.0 } }, { "index": 1, "withdrawn": true }],
          "nodes": { "left": { "x": -300.0, "y": 0.0 }, "mid": { "x": 0.0, "y": 0.0 } },
          "edges": [],
          "outcomes": []
        }
      },
      "steps": [
        { "translate": { "select": ["left"], "dx": 1.0, "dy": 0.0, "step": 144.0 }, "expect": { "rows": 1, "head": "connected" } },
        { "begin": { "row": 0 }, "expect": { "stage": "editing" } },
        { "input": { "path": "/dx", "value": -100.0 } },
        { "accept": null, "expect": { "stage": "reviewing", "review": "ready", "blocking": false, "worst": "warning", "acceptedCount": 1, "preview": "drifted", "outcomes": [{ "row": 0, "mutation": 1, "worst": "warning", "code": "mutation.precondition-drifted", "target": ["mid-west", "left-east"], "introduced": true }], "words": { "row": 0, "en": "Precondition drifted", "de": "Vorbedingung nicht mehr erfüllt" } } },
        { "begin": { "row": 0, "mutation": 1 }, "expect": { "stage": "editing", "acceptedCount": 1 } },
        { "withdraw": null },
        { "accept": null, "expect": { "stage": "reviewing", "review": "ready", "blocking": false, "acceptedCount": 2, "preview": "apart" } },
        { "finalize": "overwrite", "expect": { "stage": null, "head": "apart", "historyEdits": [{ "en": "History edited — overwrite: 2 mutations", "de": "Verlauf bearbeitet — überschrieben: 2 Mutationen" }] } },
        { "undo": null, "expect": { "head": "connected" } },
        { "redo": null, "expect": { "head": "apart" } }
      ]
    }
  ]
}""",
)

hunk(
    LAW,
    """//! - Every head equals a fresh app folding the edited log.
""",
    """//! - A drag whose drop recorded a proximity connection is edited far away: the connection still lands, carries a NEW
//!   `mutation.precondition-drifted` Warning that names both handles, the review paints it, and withdrawing the connection
//!   leaves a clean review (design §22.13).
//! - Every head equals a fresh app folding the edited log, and that fold reports exactly the outcomes the head states.
""",
)
hunk(
    LAW,
    """/// ✏️ The scenario's log with `head`'s edits applied: a replaced leaf takes its slot, a withdrawn one is dropped.
fn edited_log(scenario: &Value, head: &Value) -> Vec<Value> {
    let edits = head["edits"].as_array().expect("edits");
    scenario["log"].as_array().expect("log").iter().enumerate().filter_map(|(index, logged)| match edits.iter().find(|edit| edit["index"].as_u64() == Some(index as u64)) {
        Some(edit) => edit.get("leaf").cloned(),
        None => Some(logged.clone()),
    }).collect()
}

/// 🎯️ The app's head is the named corpus head: its nodes where the corpus places them, its absent ids absent, and every
/// node record exactly what a fresh app folding the head's edited log holds.
fn check_head(app: &mut Puzzle2dApp, corpus: &Value, scenario: &Value, name: &str, what: &str) {
    let head = &scenario["heads"][name];
    let fixture = fixture_of(app);
    let nodes = fixture_nodes(&fixture);
    for (id, expected) in head["nodes"].as_object().expect("head nodes") {
        let node = nodes.iter().find(|node| node["id"] == id.as_str()).unwrap_or_else(|| panic!("{what}: {id} is in the {name} head"));
        assert_eq!((node["x"].as_f64(), node["y"].as_f64(), node["locked"].as_bool().unwrap_or(false)), (expected["x"].as_f64(), expected["y"].as_f64(), expected["locked"].as_bool().unwrap_or(false)), "{what}: {id} in the {name} head");
    }
    for id in head["absent"].as_array().into_iter().flatten() {
        assert!(nodes.iter().all(|node| node["id"] != *id), "{what}: {id} is absent from the {name} head");
    }
    let mut fresh = seeded_app(&corpus["board"]);
    for logged in edited_log(scenario, head) {
        block_on(fresh.ingest_operations_text(&leaf(&logged).print_op())).unwrap_or_else(|fault| panic!("{what}: the edited log folds: {fault:?}"));
    }
    assert_eq!(nodes, fixture_nodes(&fixture_of(&fresh)), "{what}: the {name} head equals a fresh fold of its edited log");
    close_app(&mut fresh);
}
""",
    """/// 🎲️ The board a scenario runs on: its own, else the corpus board.
fn board<'a>(corpus: &'a Value, scenario: &'a Value) -> &'a Value {
    scenario.get("board").unwrap_or(&corpus["board"])
}

/// ✏️ The scenario's log with `head`'s edits applied, every leaf with its index in the log: a replaced leaf takes its
/// slot, a withdrawn one is dropped.
fn edited_log(scenario: &Value, head: &Value) -> Vec<(usize, Value)> {
    let edits = head["edits"].as_array().expect("edits");
    scenario["log"].as_array().expect("log").iter().enumerate().filter_map(|(index, logged)| match edits.iter().find(|edit| edit["index"].as_u64() == Some(index as u64)) {
        Some(edit) => edit.get("leaf").cloned().map(|leaf| (index, leaf)),
        None => Some((index, logged.clone())),
    }).collect()
}

/// 📍️ `document` places the named corpus head: its nodes where the corpus puts them, its absent ids absent and, where
/// the head states them, exactly its edges.
fn check_placement(document: &Value, head: &Value, name: &str, what: &str) {
    let nodes = fixture_nodes(document);
    for (id, expected) in head["nodes"].as_object().expect("head nodes") {
        let node = nodes.iter().find(|node| node["id"] == id.as_str()).unwrap_or_else(|| panic!("{what}: {id} is in the {name} head"));
        assert_eq!((node["x"].as_f64(), node["y"].as_f64(), node["locked"].as_bool().unwrap_or(false)), (expected["x"].as_f64(), expected["y"].as_f64(), expected["locked"].as_bool().unwrap_or(false)), "{what}: {id} in the {name} head");
    }
    for id in head["absent"].as_array().into_iter().flatten() {
        assert!(nodes.iter().all(|node| node["id"] != *id), "{what}: {id} is absent from the {name} head");
    }
    if let Some(edges) = head.get("edges") {
        let held: std::collections::BTreeSet<&str> = fixture_edges(document).iter().filter_map(|edge| edge["id"].as_str()).collect();
        assert_eq!(held, edges.as_array().expect("head edges").iter().filter_map(Value::as_str).collect(), "{what}: the edges of the {name} head");
    }
}

/// 🌱️ The document a fresh app holds after folding the named head's edited log over the scenario's board. Where the head
/// states `outcomes`, that fold reported exactly those messages: the leaf's index in the log, the code and the target.
fn folded_head(corpus: &Value, scenario: &Value, name: &str, what: &str) -> Value {
    let head = &scenario["heads"][name];
    let mut fresh = seeded_app(board(corpus, scenario));
    let seed = edits(&mut fresh).len();
    let log = edited_log(scenario, head);
    for (_, logged) in &log {
        block_on(fresh.ingest_operations_text(&leaf(logged).print_op())).unwrap_or_else(|fault| panic!("{what}: the edited log folds: {fault:?}"));
    }
    if let Some(outcomes) = head.get("outcomes") {
        let rows = edits(&mut fresh).split_off(seed);
        assert_eq!(rows.len(), log.len(), "{what}: every leaf of the {name} head's edited log is one edit");
        let reported: Vec<Value> = rows.iter().zip(&log).flat_map(|(row, (index, _))| row.mutations.iter().flat_map(|mutation| &mutation.messages).map(|message| json!({ "index": index, "code": message.code, "target": message.target })).collect::<Vec<_>>()).collect();
        assert_eq!(&Value::Array(reported), outcomes, "{what}: the outcomes the {name} head's edited log folds to");
    }
    let document = fixture_of(&fresh);
    close_app(&mut fresh);
    document
}

/// 🎯️ The app's head is the named corpus head: placed as the corpus states, and every node and edge record exactly what a
/// fresh app folding the head's edited log holds.
fn check_head(app: &mut Puzzle2dApp, corpus: &Value, scenario: &Value, name: &str, what: &str) {
    let document = fixture_of(app);
    check_placement(&document, &scenario["heads"][name], name, what);
    let folded = folded_head(corpus, scenario, name, what);
    assert_eq!(fixture_nodes(&document), fixture_nodes(&folded), "{what}: the {name} head equals a fresh fold of its edited log");
    assert_eq!(fixture_edges(&document), fixture_edges(&folded), "{what}: the edges of the {name} head equal a fresh fold of its edited log");
}

/// 🖼️ The review paints the named corpus head: placed as the corpus states, every node where a fresh app folding the
/// head's edited log holds it.
fn check_preview(app: &mut Puzzle2dApp, corpus: &Value, scenario: &Value, name: &str, what: &str) {
    let painted = painted_fixture(app);
    check_placement(&painted, &scenario["heads"][name], name, what);
    let folded = folded_head(corpus, scenario, name, what);
    let places = |document: &Value| fixture_nodes(document).iter().map(|node| (node["id"].clone(), node["x"].as_f64(), node["y"].as_f64())).collect::<Vec<_>>();
    assert_eq!(places(&painted), places(&folded), "{what}: the painted {name} preview places every node where a fresh fold of its edited log does");
}
""",
)
hunk(
    LAW,
    """                _ => edits(app)[seed + spec["row"].as_u64().expect("row") as usize].mutations[0].mutation_id.clone(),
""",
    """                _ => edits(app)[seed + spec["row"].as_u64().expect("row") as usize].mutations[spec["mutation"].as_u64().unwrap_or(0) as usize].mutation_id.clone(),
""",
)
hunk(
    LAW,
    """        let mutation = serde_json::to_value(row.mutations.first().expect("the row's mutation")).expect("mutation row serializes");
""",
    """        let mutation = serde_json::to_value(row.mutations.get(outcome["mutation"].as_u64().unwrap_or(0) as usize).expect("the row's mutation")).expect("mutation row serializes");
""",
)
hunk(
    LAW,
    """    if let Some(name) = expect["head"].as_str() {
        check_head(app, corpus, scenario, name, what);
    }
""",
    """    if let Some(name) = expect["head"].as_str() {
        check_head(app, corpus, scenario, name, what);
    }
    if let Some(name) = expect["preview"].as_str() {
        check_preview(app, corpus, scenario, name, what);
    }
""",
)
hunk(
    LAW,
    """/// - the named head, which equals a fresh fold of its edited log.
""",
    """/// - the named head, which equals a fresh fold of its edited log, and the head a review paints.
""",
)
hunk(
    LAW,
    """        let id = scenario["id"].as_str().expect("scenario id");
        let mut app = seeded_app(&corpus["board"]);
""",
    """        let id = scenario["id"].as_str().expect("scenario id");
        let mut app = seeded_app(board(&corpus, scenario));
""",
)

ORACLE_TEXT = '''"""🐍️ Independent oracle of the history-edit runtime corpus (`🧫️fixtures/🧫️history-edit-runtime/🔣️.json`).

Validates the corpus against its JSON Schema and every leaf (authored, ingested, and edited) against its puzzle 2d payload
schema with `jsonschema`. It then folds each named head with shapely, using the select-tool oracle's fold of the selection
leaves (`🧪️select-tool-history/🐍️.py`, which was written from the leaf schemas and design §8, not from the Rust). Each head
is the scenario's board with its `log` folded, after the leaves its `edits` name are replaced or dropped. The fold must give
the head's nodes, must not contain its absent ids, must hold exactly its `edges` and must report exactly its `outcomes`.

A `connectHandles` leaf adds its edge. One that states a `tolerance` is a recorded proximity (design §22.13): where its two
handles are farther apart than the tolerance on the document it folds onto, or one is on no node, the fold reports
`mutation.precondition-drifted` for both handles. A handle sits on its node's rim: a circle's east-zero angle on the radius,
a rectangle's north-zero angle on its outline.

It also checks that every head a step names exists, that every `row` a step names is an authored edit, and that a review
step's drift outcomes name the handles its previewed head folds to.

Exits non-zero on the first disagreement. An optional argument names another corpus, for negative controls.
"""

import importlib.util
import json
import math
import pathlib
import sys

import jsonschema
from shapely import affinity
from shapely.geometry import LineString, Point, box

EDITOR = pathlib.Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("select_tool_history_oracle", EDITOR / "🧪️tests" / "🧪️select-tool-history" / "🐍️.py")
SELECT_TOOL = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(SELECT_TOOL)
TOLERANCE = 1e-9
CONNECT = "connectHandles"
DRIFTED = "mutation.precondition-drifted"


def load(path):
    return json.loads(path.read_text(encoding="utf-8"))


def validator(kind, cache={}):
    if kind != CONNECT:
        return SELECT_TOOL.leaf_validator(kind)
    if kind not in cache:
        cache[kind] = jsonschema.Draft7Validator(load(SELECT_TOOL.MUTATIONS / "🪢️connect-handles" / "🧬️schema" / "🔣️.json"))
    return cache[kind]


def edited_log(scenario, head):
    edits = {edit["index"]: edit for edit in head["edits"]}
    for index, leaf in enumerate(scenario["log"]):
        edit = edits.get(index)
        if edit is None:
            yield index, leaf
        elif "leaf" in edit:
            yield index, edit["leaf"]


def leaves(scenario):
    yield from scenario["log"]
    for head in scenario["heads"].values():
        yield from (edit["leaf"] for edit in head["edits"] if "leaf" in edit)
    yield from (step["ingest"] for step in scenario["steps"] if "ingest" in step)


def rim(node, handle):
    centre = Point(node["x"], node["y"])
    if node.get("shape") != "rectangle":
        return affinity.rotate(Point(centre.x + node.get("radius", 24.0), centre.y), handle["angle"], origin=centre, use_radians=True)
    half_width, half_height = node.get("width", 48.0) / 2.0, node.get("height", 48.0) / 2.0
    reach = 2.0 * math.hypot(half_width, half_height)
    ray = LineString([centre, (centre.x - math.sin(handle["angle"]) * reach, centre.y - math.cos(handle["angle"]) * reach)])
    return ray.intersection(box(centre.x - half_width, centre.y - half_height, centre.x + half_width, centre.y + half_height).exterior)


def fold(board, log):
    edges, outcomes, selection = [edge["id"] for edge in board["edges"]], [], []
    for index, leaf in log:
        if leaf["mutation"] != CONNECT:
            selection.append(leaf)
            continue
        if leaf["id"] in edges:
            continue
        handles = {handle["id"]: (node, handle) for node in SELECT_TOOL.fold(board, selection).values() for handle in node.get("handles", [])}
        ends = [handles.get(leaf[end]) for end in ("source", "target")]
        tolerance = leaf.get("tolerance")
        if tolerance is not None and (None in ends or rim(*ends[0]).distance(rim(*ends[1])) > tolerance):
            outcomes.append({"index": index, "code": DRIFTED, "target": [leaf["source"], leaf["target"]]})
        edges.append(leaf["id"])
    return SELECT_TOOL.fold(board, selection), edges, outcomes


def main():
    corpus = load(pathlib.Path(sys.argv[1]) if len(sys.argv) > 1 else EDITOR / "🧫️fixtures" / "🧫️history-edit-runtime" / "🔣️.json")
    jsonschema.Draft7Validator(load(EDITOR / "🧬️schema" / "🔣️history-edit-runtime" / "🔣️.json")).validate(corpus)
    checked, drifted = 0, 0
    for scenario in corpus["scenarios"]:
        name = scenario["id"]
        for leaf in leaves(scenario):
            validator(leaf["mutation"]).validate(leaf)
        folded = {head_name: fold(scenario.get("board", corpus["board"]), list(edited_log(scenario, head))) for head_name, head in scenario["heads"].items()}
        for step in scenario["steps"]:
            expect = step.get("expect", {})
            for key in ("head", "preview"):
                if key in expect and expect[key] not in scenario["heads"]:
                    sys.exit(f"{name}: a step names the unknown head {expect[key]}")
            rows = [outcome["row"] for outcome in expect.get("outcomes", [])] + [expect[key]["row"] for key in ("nextProblem", "words") if key in expect]
            if isinstance(step.get("begin"), dict):
                rows.append(step["begin"]["row"])
            if any(row >= len(scenario["log"]) for row in rows):
                sys.exit(f"{name}: a step names a row beyond the {len(scenario['log'])} authored edits")
            reviewed = [outcome["target"] for outcome in expect.get("outcomes", []) if outcome["code"] == DRIFTED]
            if "preview" in expect and reviewed != [outcome["target"] for outcome in folded[expect["preview"]][2]]:
                sys.exit(f"{name}: a review reports the drift of {reviewed}, its previewed head {expect['preview']} folds to {folded[expect['preview']][2]}")
        for head_name, head in scenario["heads"].items():
            nodes, edges, outcomes = folded[head_name]
            for node_id, expected in head["nodes"].items():
                node = nodes.get(node_id)
                if node is None:
                    sys.exit(f"{name}/{head_name}: {node_id} is missing from the folded head")
                if not (math.isclose(node["x"], expected["x"], abs_tol=TOLERANCE) and math.isclose(node["y"], expected["y"], abs_tol=TOLERANCE)):
                    sys.exit(f"{name}/{head_name}: {node_id} folds to ({node['x']}, {node['y']}), the corpus says ({expected['x']}, {expected['y']})")
                if node.get("locked", False) != expected.get("locked", False):
                    sys.exit(f"{name}/{head_name}: {node_id} lock folds to {node.get('locked', False)}")
                checked += 1
            if any(node_id in nodes for node_id in head.get("absent", [])):
                sys.exit(f"{name}/{head_name}: an absent node is in the folded head")
            if "edges" in head and sorted(edges) != sorted(head["edges"]):
                sys.exit(f"{name}/{head_name}: the fold holds the edges {sorted(edges)}, the corpus says {sorted(head['edges'])}")
            if "outcomes" in head:
                if any(outcome["code"] != DRIFTED for outcome in head["outcomes"]):
                    sys.exit(f"{name}/{head_name}: this oracle folds only {DRIFTED} outcomes")
                if outcomes != head["outcomes"]:
                    sys.exit(f"{name}/{head_name}: the fold reports {outcomes}, the corpus says {head['outcomes']}")
                drifted += len(outcomes)
    print(f"history-edit-runtime oracle: {len(corpus['scenarios'])} scenarios, {checked} head nodes agree, {drifted} drift outcomes agree")


if __name__ == "__main__":
    main()
'''


ORACLE_BASE = 'nodes = SELECT_TOOL.fold(corpus["board"], list(edited_log(scenario, head)))'


def main():
    check = "--check" in sys.argv[1:]
    contents, pending = {}, 0
    oracle = (ROOT / ORACLE).read_text(encoding="utf-8")
    if oracle != ORACLE_TEXT:
        if oracle.count(ORACLE_BASE) != 1:
            sys.exit(f"{ORACLE}: not the oracle this wave was written against")
        pending += 1
        if check:
            print(f"pending: {ORACLE}: whole file")
    contents[ORACLE] = ORACLE_TEXT
    for path, old, new in HUNKS:
        text = contents.get(path)
        if text is None:
            text = (ROOT / path).read_text(encoding="utf-8")
        if new in text:
            contents[path] = text
            continue
        count = text.count(old)
        if count != 1:
            sys.exit(f"{path}: anchor resolves {count} times, expected 1:\n{old[:200]}")
        contents[path] = text.replace(old, new)
        pending += 1
        if check:
            print(f"pending: {path}: {old.splitlines()[0][:100]}")
    if check:
        print(f"{pending} pending of {len(HUNKS) + 1} changes in {len(contents)} files")
        return
    stage = next((pathlib.Path(argument) for flag, argument in zip(sys.argv[1:], sys.argv[2:]) if flag == "--stage"), None)
    for path, text in contents.items():
        target = (stage or ROOT) / path
        if stage is not None:
            target.parent.mkdir(parents=True, exist_ok=True)
        if not target.exists() or target.read_text(encoding="utf-8") != text:
            target.write_text(text, encoding="utf-8")
    print(f"applied {pending} of {len(HUNKS) + 1} changes in {len(contents)} files to {stage or ROOT}")


if __name__ == "__main__":
    main()
