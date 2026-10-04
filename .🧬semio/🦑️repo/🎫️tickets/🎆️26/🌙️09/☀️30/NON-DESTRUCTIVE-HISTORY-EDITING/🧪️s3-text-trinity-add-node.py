#!/usr/bin/env python3
"""➕️ S3-TEXT (session 3, audit `📓️audit-s3-tools.md` T1/T2): restores trinity rewriting's add-node gesture as an intent leaf and
makes the working-graph wire/node inverses relative.

* NEW relative leaf `➕️add-working` `add-working-node {id, kind, name, x, y}` (tag 14) with every per-leaf surface (Rust payload,
  diff, inverse, text/binary identity, descriptor, payload schema with full `x-semio-ui` and hard length bounds, TS payload/diff/
  inverse mirrors, GraphQL, protobuf, quintet laws), its committed quintet `➕️adds` (computed here, independently of the Rust
  implementation) and every aggregate registration (Rust aggregate + the working-graph helpers, TS union, JSON Schema oneOf,
  GraphQL/protobuf aggregates, binary protocol + tag registry, text opcode registry + .semio/.g4/.ebnf, retirement arm + corpus +
  TS retirement oracle, crate mounts, structural correspondence, oracle catalog, the cross-language harness: Rust KINDS, Python
  second implementation, feature rows on the real Nakagin rule).
* T2: `connect-working-ports` and `add-working-node` undo with ONE exact RELATIVE row (`disconnect-working-edges` of the drawn wire,
  `delete-working-nodes` of the added node) whenever the base working graph is already in the canonical form the working-graph
  leaves write (sorted keys, compact), else with the base graph's `edit-before-fixture` (the only exact undo of a non-canonical
  string); `connect-working-ports` / `disconnect-working-edges` gain hard `maxLength` bounds (ids 512, kind 256) enforced by
  `holds_invariants` too.
Every edit is anchored on a unique text; the script refuses to run twice. Run from the repo root."""
import copy
import json
from pathlib import Path

S = Path("✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any")
M = S / "🧬️schema/🧬️mutations"
F = S / "🧫️fixtures/🧬️mutations"
H = S / "🧪️tests/♻️mutate-rewrite-1"
CRATE = Path("✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🦀️.rs")
OWNER = "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
SCHEMA_ID = "https://json.schemas.assets.semio-tech.com/s/trinity/rewriting/mutation/{kind}/schema.json"
SURFACES = ["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]
NAKAGIN_ADD = '{"mutation":"addWorkingNode","id":"n180","kind":"Piece","name":"n180","x":120.0,"y":-40.0}'
ID_BYTES, KIND_BYTES = 512, 256


def write(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def write_json(path: Path, value) -> None:
    write(path, json.dumps(value, ensure_ascii=False, indent=2) + "\n")


def edit(path: Path, pairs) -> None:
    text = path.read_text(encoding="utf-8")
    for old, new in pairs:
        if text.count(old) != 1:
            raise SystemExit(f"{path}: anchor not unique ({text.count(old)}): {old[:90]!r}")
        text = text.replace(old, new)
    path.write_text(text, encoding="utf-8")


def edit_json(path: Path, change) -> None:
    value = json.loads(path.read_text(encoding="utf-8"))
    change(value)
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def compact(value) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


aggregate = M / "🦀️.rs"
if "AddWorkingNode(AddWorkingNode)" in aggregate.read_text(encoding="utf-8"):
    raise SystemExit("already registered")

ADD = M / "➕️add-working"
write(ADD / "🦀️.rs", r'''//! ➕️ Relative rewriting mutation — `AddWorkingNode`: ONE node of `kind` named `name` added to the working graph at canvas
//! position (`x`, `y`) under the id `id`. The node's intent is the payload, so editing the add in history re-places, re-names or
//! re-kinds it on whatever graph it replays on instead of writing a whole graph back. The guest's own add-node verb
//! (`addWorkingNode`, design §13.3 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING: adding a node is never a `nodeGraphEdit` row)
//! yields it; its undo is ONE `delete-working-nodes` of the added node on a canonical base.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::RewritingSnapshot;

//#region 🔖️Mutation
/// ➕️ `add-working-node` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "add-working-node")]
pub struct AddWorkingNode {
    pub id: String,
    pub kind: String,
    pub name: String,
    pub x: f64,
    pub y: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn add_working_node(id: String, kind: String, name: String, x: f64, y: f64) -> RewriteRuleMutation {
    RewriteRuleMutation::AddWorkingNode(AddWorkingNode { id, kind, name, x, y })
}

impl AddWorkingNode {
    /// 🛂️ Whether the payload is well-formed: a non-blank id, kind and name within their byte bounds and a finite position.
    pub fn holds_invariants(&self) -> bool {
        [(&self.id, super::super::WORKING_ID_MAXIMUM_BYTES), (&self.kind, super::super::WORKING_KIND_MAXIMUM_BYTES), (&self.name, super::super::WORKING_ID_MAXIMUM_BYTES)]
            .iter()
            .all(|(text, bound)| !text.trim().is_empty() && text.len() <= *bound)
            && self.x.is_finite()
            && self.y.is_finite()
    }
}

impl protocol::MutationKind<RewritingSnapshot, RewriteRuleMutation> for AddWorkingNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "working-node", kind: "add-working-node", record: "AddedWorkingNode" };

    fn diff(&self, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Add node “{}”", self.name), &format!("Knoten „{}“ hinzufügen", self.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
''')
write(ADD / "🔺️diff/🦀️.rs", r'''//! 🔺️ Sparse diff builder for `AddWorkingNode` — the node appended to the working graph's nodes once its id is free, the graph
//! written back as compact JSON with sorted keys (`before_fixture_json`) once its manifest still declares every kind.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::RewritingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::AddWorkingNode, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
    let target = vec![payload.id.clone()];
    if !payload.holds_invariants() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "an added node has a non-blank id, kind and name within their bounds and a finite position", target);
    }
    let Some((json, added)) = super::super::add_working_graph_node(&base.before_fixture_json, &payload.id, &payload.kind, &payload.name, payload.x, payload.y) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "the working graph does not decode, so no node can be added to it", target);
    };
    if !added {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("the working graph already holds a node “{}”", payload.id), target);
    }
    if !super::super::working_graph_is_valid(&json) {
        return protocol::MutationOutcome::error("mutation.target-mismatch", format!("the working graph's manifest declares no node kind “{}”", payload.kind), target);
    }
    protocol::MutationOutcome::new(RewritingDiff { before_fixture_json: Some(json), ..Default::default() })
}
//#endregion 🔖️Diff
''')
write(ADD / "↩️inverse/🦀️.rs", r'''//! ↩️ Inverse for `AddWorkingNode` — ONE relative `delete-working-nodes` of the added node on a canonical base (the form every
//! working-graph leaf writes), else ONE `edit-before-fixture` putting the base string back; nothing when the add changes nothing.
use crate::standards::v1::subsets::any::schema::mutations::{delete_working_nodes, edit_before_fixture, RewriteRuleMutation};
use crate::RewritingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddWorkingNode, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
    match (super::diff::diff(payload, base).diff().before_fixture_json.is_some(), super::super::working_graph_is_canonical(&base.before_fixture_json)) {
        (false, _) => Vec::new(),
        (true, true) => vec![delete_working_nodes(vec![payload.id.clone()])],
        (true, false) => vec![edit_before_fixture(base.before_fixture_json.clone())],
    }
}
//#endregion 🔖️Inverse
''')
write(ADD / "🔺️diff/🟦️.ts", '''/** 🔺️ rewriting add-working-node/🔺️diff — mirror of the node appended to the working graph (`null`: the id is taken). */
import type { AddWorkingNode } from "../🟦️.ts";

type WorkingNode = { id: string; kind: string; name: string; x: number; y: number };

export function diff(payload: AddWorkingNode, base: { readonly nodes: readonly WorkingNode[] }): { nodes: WorkingNode[] } | null {
  if (base.nodes.some((held) => held.id === payload.id)) return null;
  return { nodes: [...base.nodes, { id: payload.id, kind: payload.kind, name: payload.name, x: payload.x, y: payload.y }] };
}
''')
write(ADD / "↩️inverse/🟦️.ts", '''/** ↩️ rewriting add-working-node/↩️inverse — mirror of the one-row exact undo (relative on a canonical base). */
import type { AddWorkingNode } from "../🟦️.ts";
import type { DeleteWorkingNodes } from "../../✂️delete-working/🟦️.ts";
import type { EditBeforeFixture } from "../../🖼️edit-before-fixture/🟦️.ts";

export function inverse(payload: AddWorkingNode, baseBeforeFixtureJson: string, canonical: boolean): (DeleteWorkingNodes | EditBeforeFixture)[] {
  return canonical ? [{ targets: [payload.id] }] : [{ newBeforeFixtureJson: baseBeforeFixtureJson }];
}
''')
write(ADD / "💾️binary/🦀️.rs", '//! 💾️ Direct binary-codec identity for add-working-node / AddWorkingNode.\n\npub const BINARY_TAG: u8 = dsl::protocol_record::tag_u8(include_str!("../../💾️binary/📡️.protocol.semio"), "add-working-node");\n')
write(ADD / "📝️text/🦀️.rs", '//! 📝️ Direct text-codec identity for add-working-node / AddWorkingNode.\n\npub const TEXT_OPCODE: &str = "add-working-node";\n')
write(ADD / "🟦️.ts", "/** ➕️ Relative rewriting `add-working-node` payload mirror of `AddWorkingNode`. */\nexport interface AddWorkingNode {\n  id: string;\n  kind: string;\n  name: string;\n  x: number;\n  y: number;\n}\n")
write(ADD / "🔗️.graphql", "# ➕️ Relative add-working-node / AddWorkingNode payload.\ninput AddWorkingNodeInput { id: String!, kind: String!, name: String!, x: Float!, y: Float! }\n")
write(ADD / "🛰️.proto", 'syntax = "proto3";\npackage semio.s.trinity.rewriting.mutation.add_working_node;\n// add-working-node / AddWorkingNode\nmessage AddWorkingNode { string id = 1; string kind = 2; string name = 3; double x = 4; double y = 5; }\n')
write_json(ADD / "🔣️.json", {"schemaVersion": 1, "owner": f"{OWNER}/➕️add-working", "semanticKind": "add-working-node", "displayName": "Add Working Node", "emoji": "➕️", "aggregateVariant": "AddWorkingNode", "payloadSchema": "🧬️schema/🔣️.json", "textOpcode": "add-working-node", "binaryTag": 14, "invertibility": "explicit-mutation", "diffParticipation": "detect", "outcomeClasses": ["applied", "rejected"], "composition": "atomic", "requiredLanguageSurfaces": SURFACES})
COORDINATE_UI = lambda axis, order: {"widget": "stepper", "role": "value", "label": {"en": f"Position {axis.upper()}", "de": f"Position {axis.upper()}"}, "description": {"en": f"{'Horizontal' if axis == 'x' else 'Vertical'} canvas position of the node.", "de": f"{'Horizontale' if axis == 'x' else 'Vertikale'} Leinwandposition des Knotens."}, "step": 1, "precision": 2, "unit": "px", "group": "position", "order": order}
write_json(ADD / "🧬️schema/🔣️.json", {
    "$schema": "http://json-schema.org/draft-07/schema#", "$id": SCHEMA_ID.format(kind="add-working-node"), "title": "AddWorkingNode",
    "description": "ONE node of `kind` named `name` added to the working (before) graph at canvas position (`x`, `y`) under the id `id`. An id the graph already holds is `mutation.duplicate-id`; a kind the graph's manifest does not declare is `mutation.target-mismatch`.",
    "type": "object", "additionalProperties": False, "required": ["mutation", "id", "kind", "name", "x", "y"],
    "properties": {
        "mutation": {"const": "addWorkingNode"},
        "id": {"type": "string", "minLength": 1, "maxLength": ID_BYTES, "pattern": "\\S", "x-semio-ui": {"widget": "text", "role": "target", "label": {"en": "Node Id", "de": "Knoten-Id"}, "description": {"en": "The id the added node carries; another node of the graph must not hold it.", "de": "Die Id des hinzugefügten Knotens; kein anderer Knoten des Graphen darf sie tragen."}, "ref": {"kind": "node", "domain": "graph", "granularity": "node"}, "group": "node", "order": 10}},
        "kind": {"type": "string", "minLength": 1, "maxLength": KIND_BYTES, "pattern": "\\S", "x-semio-ui": {"widget": "text", "role": "value", "label": {"en": "Node Kind", "de": "Knotenart"}, "description": {"en": "The node kind of the graph's manifest the node carries.", "de": "Die Knotenart aus dem Manifest des Graphen, die der Knoten trägt."}, "group": "node", "order": 20}},
        "name": {"type": "string", "minLength": 1, "maxLength": ID_BYTES, "pattern": "\\S", "x-semio-ui": {"widget": "text", "role": "value", "label": {"en": "Name", "de": "Name"}, "description": {"en": "The name the node shows.", "de": "Der Name, den der Knoten zeigt."}, "group": "node", "order": 30}},
        "x": {"type": "number", "x-semio-ui": COORDINATE_UI("x", 40)},
        "y": {"type": "number", "x-semio-ui": COORDINATE_UI("y", 50)},
    },
})
write(ADD / "🧪️tests/➕️adds/🦀️.rs", (M / "🔌️connect-working/🧪️tests/🔌️connects/🦀️.rs").read_text(encoding="utf-8")
      .replace("`connect-working-ports` fixture — `🔌️connects`: draws a Connection wire from port out of node c to port in of node a on the three-node chain; every node, both existing edges and every other member stay untouched.",
               "`add-working-node` fixture — `➕️adds`: adds the Piece node d at (300, 120) to the three-node chain; every existing node, both edges and every other member stay untouched.")
      .replace("🔌️connect-working/🔌️connects", "➕️add-working/➕️adds").replace("connect-working-ports", "add-working-node").replace("🔌️connects", "➕️adds")
      .replace('("connect", "working-ports", "add-working-node", "ConnectedWorkingPorts")', '("add", "working-node", "add-working-node", "AddedWorkingNode")'))

delete_base = json.loads((F / "✂️delete-working/✂️deletes/📸️snapshot/⬅️before/🔣️.json").read_text(encoding="utf-8"))
payload = {"mutation": "addWorkingNode", "id": "d", "kind": "Piece", "name": "d", "x": 300.0, "y": 120.0}
after = copy.deepcopy(delete_base)
graph = json.loads(after["beforeFixtureJson"])
graph["nodes"].append({"id": "d", "kind": "Piece", "name": "d", "x": 300.0, "y": 120.0})
after["beforeFixtureJson"] = compact(graph)
quintet = F / "➕️add-working" / "➕️adds"
write_json(quintet / "📸️snapshot/⬅️before/🔣️.json", delete_base)
write_json(quintet / "📸️snapshot/➡️after/🔣️.json", after)
write_json(quintet / "🦠️mutation/🔣️.json", payload)
write_json(quintet / "🔺️diff/🔣️.json", {"beforeFixtureJson": after["beforeFixtureJson"], "lhsJson": None, "rhsJson": None, "parameterBindings": None, "ruleLayout": None})
write_json(quintet / "🎯️outcome/🔣️.json", {"status": "applied"})

write(M / "🔌️connect-working/↩️inverse/🦀️.rs", r'''//! ↩️ Inverse for `ConnectWorkingPorts` — ONE relative `disconnect-working-edges` of the drawn wire on a canonical base (the form
//! every working-graph leaf writes), else ONE `edit-before-fixture` putting the base string back; nothing when the wire changes nothing.
use crate::standards::v1::subsets::any::schema::mutations::{disconnect_working_edges, edit_before_fixture, RewriteRuleMutation};
use crate::RewritingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ConnectWorkingPorts, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
    match (super::diff::diff(payload, base).diff().before_fixture_json.is_some(), super::super::working_graph_is_canonical(&base.before_fixture_json)) {
        (false, _) => Vec::new(),
        (true, true) => vec![disconnect_working_edges(vec![payload.edge_id()])],
        (true, false) => vec![edit_before_fixture(base.before_fixture_json.clone())],
    }
}
//#endregion 🔖️Inverse
''')
write(M / "🔌️connect-working/↩️inverse/🟦️.ts", '''/** ↩️ rewriting connect-working-ports/↩️inverse — mirror of the one-row exact undo (relative on a canonical base). */
import type { ConnectWorkingPorts } from "../🟦️.ts";
import type { DisconnectWorkingEdges } from "../../🪚️disconnect-working/🟦️.ts";
import type { EditBeforeFixture } from "../../🖼️edit-before-fixture/🟦️.ts";

export function inverse(payload: ConnectWorkingPorts, baseBeforeFixtureJson: string, canonical: boolean): (DisconnectWorkingEdges | EditBeforeFixture)[] {
  return canonical ? [{ targets: [`${payload.source}->${payload.target}`] }] : [{ newBeforeFixtureJson: baseBeforeFixtureJson }];
}
''')
edit(M / "🔌️connect-working/🦀️.rs", [
    ("    /// 🛂️ Whether the payload is well-formed: two different non-blank endpoints and a non-blank edge kind.\n    pub fn holds_invariants(&self) -> bool {\n        [&self.source, &self.target, &self.kind].iter().all(|text| !text.trim().is_empty()) && self.source != self.target\n    }",
     "    /// 🛂️ Whether the payload is well-formed: two different non-blank endpoints and a non-blank edge kind within their byte bounds.\n    pub fn holds_invariants(&self) -> bool {\n        [(&self.source, super::super::WORKING_ID_MAXIMUM_BYTES), (&self.target, super::super::WORKING_ID_MAXIMUM_BYTES), (&self.kind, super::super::WORKING_KIND_MAXIMUM_BYTES)].iter().all(|(text, bound)| !text.trim().is_empty() && text.len() <= *bound)\n            && self.source != self.target\n    }"),
])
edit(M / "🪚️disconnect-working/🦀️.rs", [
    ("    /// 🛂️ Whether the payload is well-formed: at least one edge, none twice.\n    pub fn holds_invariants(&self) -> bool {\n        !self.targets.is_empty() && !self.targets.iter().enumerate().any(|(at, id)| self.targets[..at].contains(id))\n    }",
     "    /// 🛂️ Whether the payload is well-formed: at least one edge id within its byte bound, none twice.\n    pub fn holds_invariants(&self) -> bool {\n        !self.targets.is_empty() && self.targets.iter().all(|id| id.len() <= super::super::WORKING_ID_MAXIMUM_BYTES) && !self.targets.iter().enumerate().any(|(at, id)| self.targets[..at].contains(id))\n    }"),
])


def bound_connect(schema):
    for field, bound in (("source", ID_BYTES), ("target", ID_BYTES), ("kind", KIND_BYTES)):
        schema["properties"][field]["maxLength"] = bound


def bound_disconnect(schema):
    schema["properties"]["targets"]["items"]["maxLength"] = ID_BYTES


edit_json(M / "🔌️connect-working/🧬️schema/🔣️.json", bound_connect)
edit_json(M / "🪚️disconnect-working/🧬️schema/🔣️.json", bound_disconnect)

edit(aggregate, [
    ("pub use super::disconnect_working_edges::{disconnect_working_edges, DisconnectWorkingEdges};\n",
     "pub use super::disconnect_working_edges::{disconnect_working_edges, DisconnectWorkingEdges};\npub use super::add_working_node::{add_working_node, AddWorkingNode};\n"),
    ("    DisconnectWorkingEdges(DisconnectWorkingEdges),\n}\n", "    DisconnectWorkingEdges(DisconnectWorkingEdges),\n    AddWorkingNode(AddWorkingNode),\n}\n"),
    ("//#region 🕸️WorkingGraph\n",
     "//#region 🕸️WorkingGraph\n"
     "/// 📏️ The byte bounds of a working-graph id or name and of a node or edge kind the working-graph leaves carry.\n"
     "pub(crate) const WORKING_ID_MAXIMUM_BYTES: usize = 512;\n"
     "pub(crate) const WORKING_KIND_MAXIMUM_BYTES: usize = 256;\n\n"
     "/// 🧾️ Whether the working graph JSON `json` is already in the canonical form every working-graph leaf writes (compact, sorted\n"
     "/// keys): only then does a relative leaf undo it exactly, string for string.\n"
     "pub(crate) fn working_graph_is_canonical(json: &str) -> bool {\n"
     "    semio_framework_pack_json::parse(json, semio_framework_pack_json::JsonMemberPolicy::Reject).is_ok_and(|graph| semio_framework_pack_json::to_string(&sorted_keys(graph)) == json)\n"
     "}\n\n"
     "/// ➕️ The working graph JSON `json` with ONE node `{id, kind, name, x, y}` appended, re-serialized as compact JSON with sorted keys,\n"
     "/// together with whether the id was free (a graph that already holds it is answered unchanged). `None` when `json` is no object\n"
     "/// with a `nodes` array.\n"
     "pub(crate) fn add_working_graph_node(json: &str, id: &str, kind: &str, name: &str, x: f64, y: f64) -> Option<(String, bool)> {\n"
     "    let mut graph = semio_framework_pack_json::parse(json, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()?;\n"
     "    let nodes = graph.get_mut(\"nodes\")?.as_array_mut()?;\n"
     "    if nodes.iter().any(|node| node.get(\"id\").and_then(semio_framework_pack_json::Value::as_str) == Some(id)) {\n"
     "        return Some((json.to_string(), false));\n"
     "    }\n"
     "    nodes.push(semio_framework_pack_json::object([(\"id\".to_string(), semio_framework_pack_json::Value::String(id.into())), (\"kind\".to_string(), semio_framework_pack_json::Value::String(kind.into())), (\"name\".to_string(), semio_framework_pack_json::Value::String(name.into())), (\"x\".to_string(), semio_framework_pack_json::Value::from(x)), (\"y\".to_string(), semio_framework_pack_json::Value::from(y))]));\n"
     "    Some((semio_framework_pack_json::to_string(&sorted_keys(graph)), true))\n"
     "}\n\n"),
])

edit(M / "🟦️.ts", [
    ('import type { DisconnectWorkingEdges } from "./🪚️disconnect-working/🟦️.ts";\n', 'import type { DisconnectWorkingEdges } from "./🪚️disconnect-working/🟦️.ts";\nimport type { AddWorkingNode } from "./➕️add-working/🟦️.ts";\n'),
    ('  | ({ mutation: "disconnectWorkingEdges" } & DisconnectWorkingEdges);', '  | ({ mutation: "disconnectWorkingEdges" } & DisconnectWorkingEdges)\n  | ({ mutation: "addWorkingNode" } & AddWorkingNode);'),
])
edit_json(M / "🔣️.json", lambda schema: schema["oneOf"].append({"$ref": SCHEMA_ID.format(kind="add-working-node")}))
edit(M / "🔗️.graphql", [
    ("input DisconnectWorkingEdgesInput { targets: [String!]! }\n", "input DisconnectWorkingEdgesInput { targets: [String!]! }\ninput AddWorkingNodeInput { id: String!, kind: String!, name: String!, x: Float!, y: Float! }\n"),
    ("  disconnectWorkingEdges: DisconnectWorkingEdgesInput\n", "  disconnectWorkingEdges: DisconnectWorkingEdgesInput\n  addWorkingNode: AddWorkingNodeInput\n"),
])
edit(M / "🛰️.proto", [
    ("message DisconnectWorkingEdges { repeated string targets = 1; }\n", "message DisconnectWorkingEdges { repeated string targets = 1; }\nmessage AddWorkingNode { string id = 1; string kind = 2; string name = 3; double x = 4; double y = 5; }\n"),
    ("    DisconnectWorkingEdges disconnect_working_edges = 14;\n", "    DisconnectWorkingEdges disconnect_working_edges = 14;\n    AddWorkingNode add_working_node = 15;\n"),
])
edit(M / "💾️binary/📡️.protocol.semio", [("record disconnect-working-edges tag=13\nfield payload bytes\n", "record disconnect-working-edges tag=13\nfield payload bytes\nrecord add-working-node tag=14\nfield payload bytes\n")])
edit(M / "💾️binary/🦀️.rs", [('    ("DisconnectWorkingEdges", super::disconnect_working_edges::binary::BINARY_TAG),\n', '    ("DisconnectWorkingEdges", super::disconnect_working_edges::binary::BINARY_TAG),\n    ("AddWorkingNode", super::add_working_node::binary::BINARY_TAG),\n')])
edit(M / "📝️text/🦀️.rs", [('    ("DisconnectWorkingEdges", super::disconnect_working_edges::text::TEXT_OPCODE),\n', '    ("DisconnectWorkingEdges", super::disconnect_working_edges::text::TEXT_OPCODE),\n    ("AddWorkingNode", super::add_working_node::text::TEXT_OPCODE),\n')])
edit(M / "📝️text/📖️.grammar.semio", [
    (" / connect-working-ports / disconnect-working-edges\n", " / connect-working-ports / disconnect-working-edges / add-working-node\n"),
    ('disconnect-working-edges = "disconnect-working-edges" SP targets\n', 'disconnect-working-edges = "disconnect-working-edges" SP targets\nadd-working-node = "add-working-node" SP key SP key SP value SP number SP number\n'),
])
edit(M / "📝️text/🅰️.g4", [
    (" | connectWorkingPorts | disconnectWorkingEdges ;\n", " | connectWorkingPorts | disconnectWorkingEdges | addWorkingNode ;\n"),
    ("disconnectWorkingEdges: 'disconnect-working-edges' SP targets ;\n", "disconnectWorkingEdges: 'disconnect-working-edges' SP targets ;\naddWorkingNode: 'add-working-node' SP key SP key SP value SP number SP number ;\n"),
])
edit(M / "📝️text/🔤️.ebnf", [
    ("     | connect working ports\n     | disconnect working edges ;\n", "     | connect working ports\n     | disconnect working edges\n     | add working node ;\n"),
    ("disconnect working edges = 'disconnect-working-edges', space, targets ;\n", "disconnect working edges = 'disconnect-working-edges', space, targets ;\nadd working node = 'add-working-node', space, key, space, key, space, value, space, number, space, number ;\n"),
])

edit(S / "🧬️schema/♻️retirement/🦀️.rs", [
    ("            Self::DisconnectWorkingEdges(value) => value.targets.retirement(),\n",
     "            Self::DisconnectWorkingEdges(value) => value.targets.retirement(),\n            Self::AddWorkingNode(value) => (value.id, (value.kind, (value.name, (value.x, value.y)))).retirement(),\n"),
])
edit(S / "🧬️schema/♻️retirement/🧫️fixtures/🔣️.json", [
    ('    { "value": { "mutation": "disconnectWorkingEdges", "targets": ["e", "Ü"] }, "bytes": 3 }\n',
     '    { "value": { "mutation": "disconnectWorkingEdges", "targets": ["e", "Ü"] }, "bytes": 3 },\n    { "value": { "mutation": "addWorkingNode", "id": "n", "kind": "Ü", "name": "ab", "x": 1, "y": 2 }, "bytes": 21 }\n'),
])
edit(S / "🧬️schema/♻️retirement/🧪️tests/🔬️document-retirement/🟦️.ts", [
    ('mutations: { type: "array", minItems: 14, maxItems: 14 }', 'mutations: { type: "array", minItems: 15, maxItems: 15 }'),
    ("      : value.source !== undefined ? text(value.source) + text(value.target) + text(value.kind)\n",
     "      : value.source !== undefined ? text(value.source) + text(value.target) + text(value.kind)\n      : value.id !== undefined ? text(value.id) + text(value.kind) + text(value.name) + 16\n"),
])

base = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️add-working"
mount = "\n".join([
    '                        #[path = "."]', "                        pub mod add_working_node {",
    f'                            #[path = "{base}/🦀️.rs"]', "                            mod component;", "                            pub use component::*;",
    f'                            #[path = "{base}/💾️binary/🦀️.rs"]', "                            pub mod binary;",
    f'                            #[path = "{base}/🔺️diff/🦀️.rs"]', "                            pub mod diff;",
    f'                            #[path = "{base}/↩️inverse/🦀️.rs"]', "                            pub mod inverse;",
    "                            #[cfg(test)]", f'                            #[path = "{base}/🧪️tests/➕️adds/🦀️.rs"]', "                            mod tests_adds_node_d;",
    f'                            #[path = "{base}/📝️text/🦀️.rs"]', "                            pub mod text;", "                        }",
]) + "\n"
edit(CRATE, [
    ('                            mod tests_cuts_edge_b_c;\n'
     '                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪚️disconnect-working/📝️text/🦀️.rs"]\n'
     '                            pub mod text;\n'
     '                        }\n',
     '                            mod tests_cuts_edge_b_c;\n'
     '                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪚️disconnect-working/📝️text/🦀️.rs"]\n'
     '                            pub mod text;\n'
     '                        }\n' + mount),
])

correspondence = M / "🧪️tests/🔬️structural-correspondence/🦀️.rs"
source = correspondence.read_text(encoding="utf-8")
start = source.index('    {\n        let kind = "delete-working-nodes";')
end = source.index("    }\n", source.index('"direct owner {directory} must correspond to the JSON catalog");', start)) + len("    }\n")
block = source[start:end].replace('"delete-working-nodes"', '"add-working-node"').replace('"DeleteWorkingNodes"', '"AddWorkingNode"').replace("✂️delete-working", "➕️add-working").replace("let binary_tag = 11;", "let binary_tag = 14;")
if "let binary_tag = 14;" not in block or "✂️" in block:
    raise SystemExit("structural correspondence block did not retarget")
edit(correspondence, [('    assert!(!catalog_kinds.contains(&"set-state")', block + '    assert!(!catalog_kinds.contains(&"set-state")')])


def catalog(value):
    oracle = value["oracles"][0]
    for old, new in [("all fourteen typed mutations", "all fifteen typed mutations"), ("(the fourteen verbs", "(the fifteen verbs"), ("the fourteen committed", "the fifteen committed")]:
        if oracle["rationale"].count(old) != 1:
            raise SystemExit(f"rationale anchor {old!r}")
        oracle["rationale"] = oracle["rationale"].replace(old, new)
    mutation_catalog = value["mutationCatalogs"][0]
    mutation_catalog["kinds"].append("add-working-node")
    mutation_catalog["vectors"].append({"mutationId": "add-working-node", "sourceMutationDirectoryName": "➕️add-working", "mutationDirectoryName": "➕️add-working", "scenarios": [{"id": "adds-node-d", "directoryName": "➕️adds"}]})
    template = next(entry for entry in value["mutationManifests"][0]["mutations"] if entry["id"] == "delete-working-nodes")
    entry = json.loads(json.dumps(template))
    entry["id"] = "add-working-node"
    entry["outcomes"] = ["applied", "rejected"]
    entry["productionDispatch"] = {**entry["productionDispatch"], "operation": "add-working-node", "variant": "AddWorkingNode"}
    value["mutationManifests"][0]["mutations"].append(entry)


edit_json(S / "🔮️oracles/🔣️.json", catalog)

edit(H / "🦀️.rs", [
    ("all fourteen typed mutations, written in Python", "all fifteen typed mutations, written in Python"),
    ("Fourteen verbs: three whole-value setters, a set/remove pair over each map, five relative working-graph edits (node drag, node field patch, node delete with its edges, wire draw, wire cut),",
     "Fifteen verbs: three whole-value setters, a set/remove pair over each map, six relative working-graph edits (node drag, node field patch, node delete with its edges, wire draw, wire cut, node add),"),
    ('"connect-working-ports", "disconnect-working-edges"];', '"connect-working-ports", "disconnect-working-edges", "add-working-node"];'),
    ('| "connect-working-ports" | "disconnect-working-edges" => "beforeFixtureJson",', '| "connect-working-ports" | "disconnect-working-edges" | "add-working-node" => "beforeFixtureJson",'),
    ("make all fourteen rows", "make all fifteen rows"),
    ("All fourteen are accepting, so each", "All fifteen are accepting, so each"),
])
edit(H / "🐍️.py", [
    ("fourteen of its typed mutations, in Python, serving as this case's differential oracle.", "fifteen of its typed mutations, in Python, serving as this case's differential oracle."),
    ("— the fourteen verbs and their positional", "— the fifteen verbs and their positional"),
    ("`disconnect-working-edges targets` and\n  `drag-rule-nodes targets dx dy`", "`disconnect-working-edges targets`,\n  `add-working-node id kind name x y` and `drag-rule-nodes targets dx dy`"),
    ("* the fourteen committed `(before, mutation, after, outcome)`", "* the fifteen committed `(before, mutation, after, outcome)`"),
    ("All fourteen are ACCEPTING, so unlike", "All fifteen are ACCEPTING, so unlike"),
    ('"connect-working-ports", "disconnect-working-edges")\n"""🏷️ Every kind the catalog declares."""', '"connect-working-ports", "disconnect-working-edges", "add-working-node")\n"""🏷️ Every kind the catalog declares."""'),
    ('    "disconnect-working-edges": "disconnectWorkingEdges",\n}', '    "disconnect-working-edges": "disconnectWorkingEdges",\n    "add-working-node": "addWorkingNode",\n}'),
    ('"connect-working-ports", "disconnect-working-edges")\n"""🕸️ The five relative working-graph verbs: they move, patch or delete nodes (a delete with every edge touching them), draw or cut\nwires INSIDE the',
     '"connect-working-ports", "disconnect-working-edges", "add-working-node")\n"""🕸️ The six relative working-graph verbs: they move, patch, add or delete nodes (a delete with every edge touching them), draw or\ncut wires INSIDE the'),
    ('''    elif kind == "disconnect-working-edges":''', '''    elif kind == "add-working-node":
        graph = json.loads(result["beforeFixtureJson"])
        if not any(node.get("id") == mutation["id"] for node in graph["nodes"]):
            graph["nodes"].append({"id": mutation["id"], "kind": mutation["kind"], "name": mutation["name"], "x": float(mutation["x"]), "y": float(mutation["y"])})
        result["beforeFixtureJson"] = compact(graph)
    elif kind == "disconnect-working-edges":'''),
    ("would make all fourteen rows", "would make all fifteen rows"),
])
edit(H / "🥒️.feature", [
    ("rule document and all fourteen typed mutations,", "rule document and all fifteen typed mutations,"),
    ("(the fourteen verbs and their argument", "(the fifteen verbs and their argument"),
    ("from the fourteen committed specification vectors.", "from the fifteen committed specification vectors."),
    ("sibling, all fourteen of them are ACCEPTING,", "sibling, all fifteen of them are ACCEPTING,"),
    ("is the same fourteen verbs against a rule whose", "is the same fifteen verbs against a rule whose"),
    ('''"targets":["2jGlFQA9H2mvmjiNpnYG5Q","2aA4mq3Qj0XRbikPznzENT"]} |

  @id-inverse''', '''"targets":["2jGlFQA9H2mvmjiNpnYG5Q","2aA4mq3Qj0XRbikPznzENT"]} |
      | add-working-node          | ''' + NAKAGIN_ADD + ''' |

  @id-inverse'''),
    ('''"targets":["2jGlFQA9H2mvmjiNpnYG5Q","2aA4mq3Qj0XRbikPznzENT"]} |

  @id-spec-vector''', '''"targets":["2jGlFQA9H2mvmjiNpnYG5Q","2aA4mq3Qj0XRbikPznzENT"]} |
      | add-working-node          | ''' + NAKAGIN_ADD + ''' |

  @id-spec-vector'''),
    ("      | disconnect-working-edges  | 🪚️disconnect-working | 🪚️cuts |\n", "      | disconnect-working-edges  | 🪚️disconnect-working | 🪚️cuts |\n      | add-working-node          | ➕️add-working | ➕️adds |\n"),
])
print("add-working-node written and registered; connect/add inverses relative on a canonical base; wire leaves bounded")
