#!/usr/bin/env python3
"""🔌️ S3-TEXT (session 3): the two relative trinity rewriting wire leaves the shared node-graph record rows need on the working
graph (design §13.3 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, schema `📺️renderer/🧑‍🎨engine/🧬️schema/🔣️node-graph-edit-rows`):
`connect-working-ports {source, target, kind}` (a `connect` row) and `disconnect-working-edges {targets}` (a `disconnect` row and the
wires a `delete` row names). Writes every per-leaf surface (Rust payload/diff/inverse, text/binary identity, descriptor, payload
schema with full `x-semio-ui`, TS payload/diff/inverse mirrors, GraphQL, protobuf, quintet laws), the committed quintets (computed
here, independently of the Rust implementation), and registers both leaves in every aggregate surface (Rust aggregate + the
working-graph helpers they share, TS union, JSON Schema oneOf, GraphQL/protobuf aggregates, binary protocol + tag registry, text
opcode registry + .semio/.g4/.ebnf, retirement arm + corpus + TS retirement oracle, crate mounts, structural correspondence, oracle
catalog, the cross-language harness: Rust KINDS, Python second implementation, feature rows on the real Nakagin rule). Every edit
is anchored on a unique text; the script refuses to run twice. Run from the repo root."""
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
NAKAGIN_CONNECT = '{"mutation":"connectWorkingPorts","source":"6947a41b-8c6d-4291-bdd8-96cd535c78fc@4ba51a88-2a7f-4b78-b119-0f02eacdb702","target":"9fde3a12-8b39-42d9-850f-f0e8343caddd@4ba51a88-2a7f-4b78-b119-0f02eacdb702","kind":"Connection"}'
NAKAGIN_DISCONNECT = '{"mutation":"disconnectWorkingEdges","targets":["2jGlFQA9H2mvmjiNpnYG5Q","2aA4mq3Qj0XRbikPznzENT"]}'


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


def endpoint(key: str) -> str:
    node, separator, port = key.partition("@")
    return node if separator and node and port else key


ENDPOINT_UI = lambda en, de, den, dde, order: {
    "widget": "reference", "role": "target", "label": {"en": en, "de": de}, "description": {"en": den, "de": dde},
    "ref": {"kind": "port", "domain": "graph", "granularity": "port"}, "group": "wire", "order": order,
}

LEAVES = [
    {
        "dir": "🔌️connect-working", "emoji": "🔌️", "kind": "connect-working-ports", "variant": "ConnectWorkingPorts", "tag": 12, "module": "connect_working_ports",
        "display": "Connect Working Ports", "outcomes": ["applied", "no-op", "rejected"], "scenario": "🔌️connects", "test_module": "tests_connects_c_to_a",
        "scenario_note": "draws a Connection wire from port out of node c to port in of node a on the three-node chain; every node, both existing edges and every other member stay untouched",
        "description": "ONE edge of `kind` drawn on the working (before) graph from the `source` port to the `target` port (port keys `node@port`, a bare node id for a node-level endpoint), under the id `source->target`. An endpoint whose node the graph lacks is `mutation.target-missing`; a kind the graph's manifest does not declare is `mutation.target-mismatch`; a wire the graph already holds is `mutation.no-op`; a wire from an endpoint to itself is `mutation.invariant`.",
        "required": ["source", "target", "kind"],
        "properties": {
            "source": {"type": "string", "minLength": 1, "pattern": "\\S", "x-semio-ui": ENDPOINT_UI("Source Port", "Quellanschluss", "The port the wire leaves: `node@port`, or a node id for a node-level endpoint.", "Der Anschluss, von dem die Verbindung ausgeht: `Knoten@Anschluss` oder eine Knoten-Id für einen Endpunkt auf Knotenebene.", 10)},
            "target": {"type": "string", "minLength": 1, "pattern": "\\S", "x-semio-ui": ENDPOINT_UI("Target Port", "Zielanschluss", "The port the wire enters: `node@port`, or a node id for a node-level endpoint.", "Der Anschluss, in den die Verbindung mündet: `Knoten@Anschluss` oder eine Knoten-Id für einen Endpunkt auf Knotenebene.", 20)},
            "kind": {"type": "string", "minLength": 1, "pattern": "\\S", "x-semio-ui": {"widget": "text", "role": "value", "label": {"en": "Edge Kind", "de": "Kantenart"}, "description": {"en": "The edge kind of the graph's manifest the wire carries.", "de": "Die Kantenart aus dem Manifest des Graphen, die die Verbindung trägt."}, "group": "wire", "order": 30}},
        },
        "invariants": [{"id": "distinct-endpoints", "description": {"en": "A wire joins two different endpoints.", "de": "Eine Verbindung verbindet zwei verschiedene Endpunkte."}}],
        "ts": "export interface ConnectWorkingPorts {\n  source: string;\n  target: string;\n  kind: string;\n}\n",
        "graphql": "input ConnectWorkingPortsInput { source: String!, target: String!, kind: String! }",
        "proto": "message ConnectWorkingPorts { string source = 1; string target = 2; string kind = 3; }",
        "grammar": ("connect-working-ports = \"connect-working-ports\" SP key SP key SP key", "connectWorkingPorts: 'connect-working-ports' SP key SP key SP key ;", "connect working ports = 'connect-working-ports', space, key, space, key, space, key ;"),
        "payload": {"mutation": "connectWorkingPorts", "source": "c@out", "target": "a@in", "kind": "Connection"},
        "retirement": ("Self::ConnectWorkingPorts(value) => (value.source, (value.target, value.kind)).retirement(),", '{ "mutation": "connectWorkingPorts", "source": "a@out", "target": "Ü@in", "kind": "K" }', 11),
    },
    {
        "dir": "🪚️disconnect-working", "emoji": "🪚️", "kind": "disconnect-working-edges", "variant": "DisconnectWorkingEdges", "tag": 13, "module": "disconnect_working_edges",
        "display": "Disconnect Working Edges", "outcomes": ["applied", "rejected"], "scenario": "🪚️cuts", "test_module": "tests_cuts_edge_b_c",
        "scenario_note": "cuts the edge b→c of the three-node chain; every node, the edge a→b and every other member stay untouched",
        "description": "Edges of the working (before) graph cut by id. Edges the graph lacks are skipped (`mutation.partial`); none present is `mutation.target-missing`.",
        "required": ["targets"],
        "properties": {
            "targets": {"type": "array", "items": {"type": "string", "minLength": 1}, "minItems": 1, "uniqueItems": True, "x-semio-ui": {"widget": "reference", "role": "target", "label": {"en": "Edges", "de": "Kanten"}, "description": {"en": "Working-graph edges to cut; ones the graph lacks are skipped.", "de": "Zu trennende Kanten des Arbeitsgraphen; im Graphen fehlende werden übersprungen."}, "ref": {"kind": "edge", "domain": "graph", "granularity": "edge"}, "group": "target", "order": 10}},
        },
        "invariants": [],
        "ts": "export interface DisconnectWorkingEdges {\n  targets: string[];\n}\n",
        "graphql": "input DisconnectWorkingEdgesInput { targets: [String!]! }",
        "proto": "message DisconnectWorkingEdges { repeated string targets = 1; }",
        "grammar": ("disconnect-working-edges = \"disconnect-working-edges\" SP targets", "disconnectWorkingEdges: 'disconnect-working-edges' SP targets ;", "disconnect working edges = 'disconnect-working-edges', space, targets ;"),
        "payload": {"mutation": "disconnectWorkingEdges", "targets": ["e-bc"]},
        "retirement": ("Self::DisconnectWorkingEdges(value) => value.targets.retirement(),", '{ "mutation": "disconnectWorkingEdges", "targets": ["e", "Ü"] }', 3),
    },
]

RUST = {
"connect-working-ports": r'''//! 🔌️ Relative rewriting mutation — `ConnectWorkingPorts`: ONE edge of `kind` drawn on the working graph from the `source` port to
//! the `target` port (port keys `node@port`, a bare node id for a node-level endpoint). The wire's intent is the payload, so editing
//! it in history redraws it on whatever graph it replays on instead of writing a whole graph back. The node-graph `connect` row
//! yields it (design §13.3 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING); its exact undo is the base graph's `edit-before-fixture`.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::RewritingSnapshot;

//#region 🔖️Mutation
/// 🔌️ `connect-working-ports` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "connect-working-ports")]
pub struct ConnectWorkingPorts {
    pub source: String,
    pub target: String,
    pub kind: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn connect_working_ports(source: String, target: String, kind: String) -> RewriteRuleMutation {
    RewriteRuleMutation::ConnectWorkingPorts(ConnectWorkingPorts { source, target, kind })
}

impl ConnectWorkingPorts {
    /// 🛂️ Whether the payload is well-formed: two different non-blank endpoints and a non-blank edge kind.
    pub fn holds_invariants(&self) -> bool {
        [&self.source, &self.target, &self.kind].iter().all(|text| !text.trim().is_empty()) && self.source != self.target
    }

    /// 🆔️ The id the drawn edge carries — `source->target` — so the same wire always has the same id.
    pub fn edge_id(&self) -> String {
        format!("{}->{}", self.source, self.target)
    }
}

impl protocol::MutationKind<RewritingSnapshot, RewriteRuleMutation> for ConnectWorkingPorts {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "connect", entity: "working-ports", kind: "connect-working-ports", record: "ConnectedWorkingPorts" };

    fn diff(&self, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Connect two ports", "Zwei Anschlüsse verbinden")
    }
    fn target(&self) -> Vec<String> {
        vec![self.source.clone(), self.target.clone()]
    }
}
//#endregion 🔖️Mutation
''',
"disconnect-working-edges": r'''//! 🪚️ Relative rewriting mutation — `DisconnectWorkingEdges`: a set of working-graph edges cut by id. The cut wires are the
//! payload, so editing the cut in history removes those edges from whatever graph it replays on instead of writing a whole graph
//! back. The node-graph `disconnect` row and the wires a `delete` row names yield it (design §13.3 of ticket
//! 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING); its exact undo is the base graph's `edit-before-fixture`.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::RewritingSnapshot;

//#region 🔖️Mutation
/// 🪚️ `disconnect-working-edges` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "disconnect-working-edges")]
pub struct DisconnectWorkingEdges {
    pub targets: Vec<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn disconnect_working_edges(targets: Vec<String>) -> RewriteRuleMutation {
    RewriteRuleMutation::DisconnectWorkingEdges(DisconnectWorkingEdges { targets })
}

impl DisconnectWorkingEdges {
    /// 🛂️ Whether the payload is well-formed: at least one edge, none twice.
    pub fn holds_invariants(&self) -> bool {
        !self.targets.is_empty() && !self.targets.iter().enumerate().any(|(at, id)| self.targets[..at].contains(id))
    }
}

impl protocol::MutationKind<RewritingSnapshot, RewriteRuleMutation> for DisconnectWorkingEdges {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "disconnect", entity: "working-edges", kind: "disconnect-working-edges", record: "DisconnectedWorkingEdges" };

    fn diff(&self, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        match self.targets.len() {
            1 => protocol::LocalizedLabel::native("Disconnect 1 edge", "1 Kante trennen"),
            count => protocol::LocalizedLabel::native(&format!("Disconnect {count} edges"), &format!("{count} Kanten trennen")),
        }
    }
    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}
//#endregion 🔖️Mutation
''',
}

DIFF = {
"connect-working-ports": r'''//! 🔺️ Sparse diff builder for `ConnectWorkingPorts` — the wire appended to the working graph's edges once both endpoint nodes are
//! in it, the graph written back as compact JSON with sorted keys (`before_fixture_json`) once its manifest still declares every kind.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::RewritingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ConnectWorkingPorts, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
    let endpoints = vec![payload.source.clone(), payload.target.clone()];
    if !payload.holds_invariants() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a wire joins two different non-blank endpoints by a non-blank edge kind", endpoints);
    }
    let Some((json, missing, drawn)) = super::super::connect_working_graph_ports(&base.before_fixture_json, &payload.edge_id(), &payload.source, &payload.target, &payload.kind) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "the working graph does not decode, so it holds neither endpoint", endpoints);
    };
    if !missing.is_empty() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("the working graph holds no node {}", missing.join(", ")), missing);
    }
    if !drawn {
        return protocol::MutationOutcome::new(RewritingDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "the working graph already holds this wire").at(endpoints)]);
    }
    if !super::super::working_graph_is_valid(&json) {
        return protocol::MutationOutcome::error("mutation.target-mismatch", format!("the working graph's manifest declares no edge kind “{}”", payload.kind), endpoints);
    }
    protocol::MutationOutcome::new(RewritingDiff { before_fixture_json: Some(json), ..Default::default() })
}
//#endregion 🔖️Diff
''',
"disconnect-working-edges": r'''//! 🔺️ Sparse diff builder for `DisconnectWorkingEdges` — every addressed edge of the working graph removed, the graph written back
//! as compact JSON with sorted keys (`before_fixture_json`).
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::RewritingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DisconnectWorkingEdges, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
    if !payload.holds_invariants() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a cut names at least one edge, never one twice", payload.targets.clone());
    }
    let Some((json, missing)) = super::super::remove_working_graph_edges(&base.before_fixture_json, &payload.targets) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "the working graph does not decode, so it has none of the targets", payload.targets.clone());
    };
    if missing.len() == payload.targets.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("none of the {} target(s) is an edge of the working graph", payload.targets.len()), payload.targets.clone());
    }
    let partial: Vec<protocol::MutationMessage> =
        (!missing.is_empty()).then(|| protocol::MutationMessage::warning("mutation.partial", format!("{} of {} target(s) skipped (not in the working graph): {}", missing.len(), payload.targets.len(), missing.join(", "))).at(missing)).into_iter().collect();
    protocol::MutationOutcome::new(RewritingDiff { before_fixture_json: Some(json), ..Default::default() }).absorb_messages(partial)
}
//#endregion 🔖️Diff
''',
}

INVERSE = r'''//! ↩️ Inverse for `{variant}` — ONE `edit-before-fixture` putting the BASE working graph back; nothing when the {noun} changes nothing.
use crate::standards::v1::subsets::any::schema::mutations::{{edit_before_fixture, RewriteRuleMutation}};
use crate::RewritingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::{variant}, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {{
    match super::diff::diff(payload, base).diff().before_fixture_json.is_some() {{
        true => vec![edit_before_fixture(base.before_fixture_json.clone())],
        false => Vec::new(),
    }}
}}
//#endregion 🔖️Inverse
'''

TS_DIFF = {
"connect-working-ports": '''/** 🔺️ rewriting connect-working-ports/🔺️diff — mirror of the wire drawn on the working graph (`null`: an endpoint node is missing). */
import type { ConnectWorkingPorts } from "../🟦️.ts";

type WorkingEdge = { id: string; kind: string; source: string; target: string };

export function diff(payload: ConnectWorkingPorts, base: { readonly nodes: readonly { id: string }[]; readonly edges: readonly WorkingEdge[] }): { edges: WorkingEdge[] } | null {
  const node = (key: string): string => {
    const at = key.indexOf("@");
    return at > 0 && at < key.length - 1 ? key.slice(0, at) : key;
  };
  if (![payload.source, payload.target].every((key) => base.nodes.some((held) => held.id === node(key)))) return null;
  if (base.edges.some((edge) => edge.source === payload.source && edge.target === payload.target)) return { edges: [...base.edges] };
  return { edges: [...base.edges, { id: `${payload.source}->${payload.target}`, kind: payload.kind, source: payload.source, target: payload.target }] };
}
''',
"disconnect-working-edges": '''/** 🔺️ rewriting disconnect-working-edges/🔺️diff — mirror of the edge removal of the working graph. */
import type { DisconnectWorkingEdges } from "../🟦️.ts";

export function diff(payload: DisconnectWorkingEdges, base: { readonly edges: readonly { id: string }[] }): { edges: { id: string }[] } {
  return { edges: base.edges.filter((edge) => !payload.targets.includes(edge.id)) };
}
''',
}

TS_INVERSE = '''/** ↩️ rewriting {kind}/↩️inverse — mirror of the one-row exact undo. */
import type {{ {variant} }} from "../🟦️.ts";
import type {{ EditBeforeFixture }} from "../../🖼️edit-before-fixture/🟦️.ts";

export function inverse(_payload: {variant}, baseBeforeFixtureJson: string): EditBeforeFixture[] {{
  return [{{ newBeforeFixtureJson: baseBeforeFixtureJson }}];
}}
'''

TEST = r'''//! 🧪️ `{kind}` fixture — `{scenario}`: {note}.
//!
//! Source of truth is the committed JSON quintet (`🧫️fixtures/🧬️mutations/{dir}/{scenario}`), authored by
//! `T/🧪️s3-text-trinity-wire-leaves.py` independently of this implementation (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).

use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::RewritingSnapshot;
use crate::{{apply_rewrite_rule_mutation, inverse_rewrite_rule_mutation}};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{dir}/{scenario}/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{dir}/{scenario}/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{dir}/{scenario}/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{dir}/{scenario}/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{dir}/{scenario}/🎯️outcome/🔣️.json");

fn before() -> RewritingSnapshot {{
    pack::from_json_str(BEFORE).expect("before snapshot decodes")
}}
fn expected_after() -> RewritingSnapshot {{
    pack::from_json_str(AFTER).expect("after snapshot decodes")
}}
fn mutation() -> RewriteRuleMutation {{
    pack::from_json_str(MUTATION).expect("mutation decodes")
}}

/// ▶️ `{kind}` carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {{
    let mut snapshot = before();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation()).expect("{kind} applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "{kind}/{scenario}: applied state differs from the committed after-snapshot");
}}

/// ↩️ `{kind}` undoes with exactly ONE row of `EditBeforeFixture`, which restores `before` exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {{
    let base = before();
    let inverse = inverse_rewrite_rule_mutation(&base, &mutation());
    assert!(matches!(inverse.as_slice(), [RewriteRuleMutation::EditBeforeFixture(_)]), "{kind} undoes with one EditBeforeFixture row, got {{inverse:?}}");
    let mut snapshot = base.clone();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation()).expect("forward applies");
    for step in &inverse {{
        apply_rewrite_rule_mutation(&mut snapshot, step).expect("inverse step applies");
    }}
    assert_eq!(snapshot, base, "{kind}/{scenario}: the inverse did not restore the before-snapshot");
}}

/// 🔣️ The committed snapshots and payload are canonical: decode → encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {{
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {{
        let decoded: RewritingSnapshot = pack::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&decoded)).expect("snapshot encodes");
        assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(text).expect("snapshot reparses"), "{kind}/{scenario}: committed {{label}} JSON is not canonical");
    }}
    let reencoded = serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&mutation())).expect("mutation encodes");
    assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(MUTATION).expect("mutation reparses"), "{kind}/{scenario}: committed mutation JSON is not canonical");
}}

/// 🎯️ The declared outcome holds — an applied change with no diagnostics — and the fixture is bound to `{kind}`'s descriptor.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {{
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"));
    let produced = <RewriteRuleMutation as protocol::Mutation<RewritingSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "{kind}/{scenario} declares no diagnostics, but got {{:?}}", produced.messages());
    let semantics = <RewriteRuleMutation as protocol::SemanticMutation<RewritingSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("{verb}", "{entity}", "{kind}", "{record}"));
}}

/// 🔺️ The sparse delta is exactly the committed diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {{
    let outcome = <RewriteRuleMutation as protocol::Mutation<RewritingSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::from_str::<serde_json::Value>(&pack::to_json_string(outcome.diff())).expect("produced diff encodes");
    assert_eq!(produced, serde_json::from_str::<serde_json::Value>(DIFF).expect("committed diff decodes"), "{kind}/{scenario}: the produced diff differs from the committed one");
}}

/// 🩹️ Applying the committed diff to `before` yields the committed `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {{
    let decoded: RewritingDiff = pack::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <RewritingDiff as protocol::MutationDiff<RewritingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies");
    assert_eq!(produced, expected_after(), "{kind}/{scenario}: the committed diff did not carry before to after");
}}
'''

SEMANTICS = {"connect-working-ports": ("connect", "working-ports", "ConnectedWorkingPorts"), "disconnect-working-edges": ("disconnect", "working-edges", "DisconnectedWorkingEdges")}


def apply_working(document, mutation):
    """🐍️ The independent working-graph reading of both leaves (mirrors the harness's Python second implementation)."""
    result = copy.deepcopy(document)
    graph = json.loads(result["beforeFixtureJson"])
    if mutation["mutation"] == "connectWorkingPorts":
        graph.setdefault("edges", []).append({"id": f'{mutation["source"]}->{mutation["target"]}', "kind": mutation["kind"], "source": mutation["source"], "target": mutation["target"]})
    else:
        graph["edges"] = [edge for edge in graph.get("edges", []) if edge.get("id") not in mutation["targets"]]
    result["beforeFixtureJson"] = compact(graph)
    return result


aggregate = M / "🦀️.rs"
if "ConnectWorkingPorts(ConnectWorkingPorts)" in aggregate.read_text(encoding="utf-8"):
    raise SystemExit("already registered")

delete_base = json.loads((F / "✂️delete-working/✂️deletes/📸️snapshot/⬅️before/🔣️.json").read_text(encoding="utf-8"))
for leaf in LEAVES:
    kind, variant, root = leaf["kind"], leaf["variant"], M / leaf["dir"]
    write(root / "🦀️.rs", RUST[kind])
    write(root / "🔺️diff/🦀️.rs", DIFF[kind])
    write(root / "↩️inverse/🦀️.rs", INVERSE.format(variant=variant, noun="wire" if kind.startswith("connect") else "cut"))
    write(root / "🔺️diff/🟦️.ts", TS_DIFF[kind])
    write(root / "↩️inverse/🟦️.ts", TS_INVERSE.format(kind=kind, variant=variant))
    write(root / "💾️binary/🦀️.rs", f'//! 💾️ Direct binary-codec identity for {kind} / {variant}.\n\npub const BINARY_TAG: u8 = dsl::protocol_record::tag_u8(include_str!("../../💾️binary/📡️.protocol.semio"), "{kind}");\n')
    write(root / "📝️text/🦀️.rs", f'//! 📝️ Direct text-codec identity for {kind} / {variant}.\n\npub const TEXT_OPCODE: &str = "{kind}";\n')
    write(root / "🟦️.ts", f"/** {leaf['emoji']} Relative rewriting `{kind}` payload mirror of `{variant}`. */\n" + leaf["ts"])
    write(root / "🔗️.graphql", f"# {leaf['emoji']} Relative {kind} / {variant} payload.\n{leaf['graphql']}\n")
    write(root / "🛰️.proto", f'syntax = "proto3";\npackage semio.s.trinity.rewriting.mutation.{leaf["module"]};\n// {kind} / {variant}\n{leaf["proto"]}\n')
    write_json(root / "🔣️.json", {"schemaVersion": 1, "owner": f"{OWNER}/{leaf['dir']}", "semanticKind": kind, "displayName": leaf["display"], "emoji": leaf["emoji"], "aggregateVariant": variant, "payloadSchema": "🧬️schema/🔣️.json", "textOpcode": kind, "binaryTag": leaf["tag"], "invertibility": "explicit-mutation", "diffParticipation": "detect", "outcomeClasses": leaf["outcomes"], "composition": "atomic", "requiredLanguageSurfaces": SURFACES})
    schema = {"$schema": "http://json-schema.org/draft-07/schema#", "$id": SCHEMA_ID.format(kind=kind), "title": variant, "description": leaf["description"], "type": "object", "additionalProperties": False, "required": ["mutation", *leaf["required"]]}
    if leaf["invariants"]:
        schema["x-semio-invariant"] = leaf["invariants"]
    schema["properties"] = {"mutation": {"const": leaf["payload"]["mutation"]}, **leaf["properties"]}
    write_json(root / "🧬️schema/🔣️.json", schema)
    verb, entity, record = SEMANTICS[kind]
    write(root / f"🧪️tests/{leaf['scenario']}/🦀️.rs", TEST.format(kind=kind, scenario=leaf["scenario"], note=leaf["scenario_note"], dir=leaf["dir"], verb=verb, entity=entity, record=record))
    after = apply_working(delete_base, leaf["payload"])
    quintet = F / leaf["dir"] / leaf["scenario"]
    write_json(quintet / "📸️snapshot/⬅️before/🔣️.json", delete_base)
    write_json(quintet / "📸️snapshot/➡️after/🔣️.json", after)
    write_json(quintet / "🦠️mutation/🔣️.json", leaf["payload"])
    write_json(quintet / "🔺️diff/🔣️.json", {"beforeFixtureJson": after["beforeFixtureJson"], "lhsJson": None, "rhsJson": None, "parameterBindings": None, "ruleLayout": None})
    write_json(quintet / "🎯️outcome/🔣️.json", {"status": "applied"})

edit(aggregate, [
    ("pub use super::delete_working_nodes::{delete_working_nodes, DeleteWorkingNodes};\n",
     "pub use super::delete_working_nodes::{delete_working_nodes, DeleteWorkingNodes};\n"
     "pub use super::connect_working_ports::{connect_working_ports, ConnectWorkingPorts};\n"
     "pub use super::disconnect_working_edges::{disconnect_working_edges, DisconnectWorkingEdges};\n"),
    ("    DeleteWorkingNodes(DeleteWorkingNodes),\n}\n",
     "    DeleteWorkingNodes(DeleteWorkingNodes),\n    ConnectWorkingPorts(ConnectWorkingPorts),\n    DisconnectWorkingEdges(DisconnectWorkingEdges),\n}\n"),
    ("/// 🕸️ Whether `json` is a working graph whose every node and edge kind its own manifest declares.\n",
     "/// 🔌️ The working graph JSON `json` with ONE edge `id` of `kind` from the `source` to the `target` endpoint appended (an `edges`\n"
     "/// array is created when the graph has none), re-serialized as compact JSON with sorted keys, together with the endpoint nodes the\n"
     "/// graph lacks (a `node@port` endpoint names its node before the `@`) and whether the wire is new; a graph that lacks an endpoint\n"
     "/// or already holds the wire is answered unchanged. `None` when `json` is no object with a `nodes` array.\n"
     "pub(crate) fn connect_working_graph_ports(json: &str, id: &str, source: &str, target: &str, kind: &str) -> Option<(String, Vec<String>, bool)> {\n"
     "    let mut graph = pack::parse_json(json).ok()?;\n"
     "    let nodes = graph.get(\"nodes\")?.as_array()?;\n"
     "    let missing: Vec<String> = [source, target].iter().map(|key| semio_s_artifact_trinity_jack::port_node_id(key).unwrap_or(*key)).filter(|node| !nodes.iter().any(|held| held.get(\"id\").and_then(pack::JsonValue::as_str) == Some(*node))).map(str::to_string).collect();\n"
     "    let held = |edge: &pack::JsonValue| edge.get(\"source\").and_then(pack::JsonValue::as_str) == Some(source) && edge.get(\"target\").and_then(pack::JsonValue::as_str) == Some(target);\n"
     "    if !missing.is_empty() || graph.get(\"edges\").and_then(pack::JsonValue::as_array).is_some_and(|edges| edges.iter().any(held)) {\n"
     "        return Some((json.to_string(), missing, false));\n"
     "    }\n"
     "    let edge = pack::json_object([(\"id\".to_string(), pack::JsonValue::String(id.into())), (\"kind\".to_string(), pack::JsonValue::String(kind.into())), (\"source\".to_string(), pack::JsonValue::String(source.into())), (\"target\".to_string(), pack::JsonValue::String(target.into()))]);\n"
     "    match graph.get_mut(\"edges\").and_then(pack::JsonValue::as_array_mut) {\n"
     "        Some(edges) => edges.push(edge),\n"
     "        None => {\n"
     "            graph.as_object_mut()?.insert(\"edges\", pack::json_array([edge]));\n"
     "        }\n"
     "    }\n"
     "    Some((pack::json_to_string(&sorted_keys(graph)), Vec::new(), true))\n"
     "}\n\n"
     "/// 🪚️ The working graph JSON `json` without the edges `targets` names, re-serialized as compact JSON with sorted keys, together\n"
     "/// with the targets the graph lacks. `None` when `json` is no object with a `nodes` array.\n"
     "pub(crate) fn remove_working_graph_edges(json: &str, targets: &[String]) -> Option<(String, Vec<String>)> {\n"
     "    let mut graph = pack::parse_json(json).ok()?;\n"
     "    graph.get(\"nodes\")?.as_array()?;\n"
     "    let mut found: Vec<String> = Vec::new();\n"
     "    if let Some(edges) = graph.get_mut(\"edges\").and_then(pack::JsonValue::as_array_mut) {\n"
     "        found = edges.iter().filter_map(|edge| edge.get(\"id\").and_then(pack::JsonValue::as_str)).filter(|id| targets.iter().any(|target| target == id)).map(str::to_string).collect();\n"
     "        edges.retain(|edge| edge.get(\"id\").and_then(pack::JsonValue::as_str).is_none_or(|id| !found.iter().any(|gone| gone == id)));\n"
     "    }\n"
     "    let missing = targets.iter().filter(|target| !found.contains(target)).cloned().collect();\n"
     "    Some((pack::json_to_string(&sorted_keys(graph)), missing))\n"
     "}\n\n"
     "/// 🕸️ Whether `json` is a working graph whose every node and edge kind its own manifest declares.\n"),
])

edit(M / "🟦️.ts", [
    ('import type { DeleteWorkingNodes } from "./✂️delete-working/🟦️.ts";\n',
     'import type { DeleteWorkingNodes } from "./✂️delete-working/🟦️.ts";\nimport type { ConnectWorkingPorts } from "./🔌️connect-working/🟦️.ts";\nimport type { DisconnectWorkingEdges } from "./🪚️disconnect-working/🟦️.ts";\n'),
    ('  | ({ mutation: "deleteWorkingNodes" } & DeleteWorkingNodes);',
     '  | ({ mutation: "deleteWorkingNodes" } & DeleteWorkingNodes)\n  | ({ mutation: "connectWorkingPorts" } & ConnectWorkingPorts)\n  | ({ mutation: "disconnectWorkingEdges" } & DisconnectWorkingEdges);'),
])

edit_json(M / "🔣️.json", lambda schema: schema["oneOf"].extend({"$ref": SCHEMA_ID.format(kind=leaf["kind"])} for leaf in LEAVES))

edit(M / "🔗️.graphql", [
    ("input DeleteWorkingNodesInput { targets: [String!]! }\n", "input DeleteWorkingNodesInput { targets: [String!]! }\n" + "".join(leaf["graphql"] + "\n" for leaf in LEAVES)),
    ("  deleteWorkingNodes: DeleteWorkingNodesInput\n", "  deleteWorkingNodes: DeleteWorkingNodesInput\n  connectWorkingPorts: ConnectWorkingPortsInput\n  disconnectWorkingEdges: DisconnectWorkingEdgesInput\n"),
])

edit(M / "🛰️.proto", [
    ("message DeleteWorkingNodes { repeated string targets = 1; }\n", "message DeleteWorkingNodes { repeated string targets = 1; }\n" + "".join(leaf["proto"] + "\n" for leaf in LEAVES)),
    ("    DeleteWorkingNodes delete_working_nodes = 12;\n", "    DeleteWorkingNodes delete_working_nodes = 12;\n    ConnectWorkingPorts connect_working_ports = 13;\n    DisconnectWorkingEdges disconnect_working_edges = 14;\n"),
])

edit(M / "💾️binary/📡️.protocol.semio", [
    ("record delete-working-nodes tag=11\nfield payload bytes\n", "record delete-working-nodes tag=11\nfield payload bytes\nrecord connect-working-ports tag=12\nfield payload bytes\nrecord disconnect-working-edges tag=13\nfield payload bytes\n"),
])
edit(M / "💾️binary/🦀️.rs", [
    ('    ("DeleteWorkingNodes", super::delete_working_nodes::binary::BINARY_TAG),\n',
     '    ("DeleteWorkingNodes", super::delete_working_nodes::binary::BINARY_TAG),\n    ("ConnectWorkingPorts", super::connect_working_ports::binary::BINARY_TAG),\n    ("DisconnectWorkingEdges", super::disconnect_working_edges::binary::BINARY_TAG),\n'),
])
edit(M / "📝️text/🦀️.rs", [
    ('    ("DeleteWorkingNodes", super::delete_working_nodes::text::TEXT_OPCODE),\n',
     '    ("DeleteWorkingNodes", super::delete_working_nodes::text::TEXT_OPCODE),\n    ("ConnectWorkingPorts", super::connect_working_ports::text::TEXT_OPCODE),\n    ("DisconnectWorkingEdges", super::disconnect_working_edges::text::TEXT_OPCODE),\n'),
])
edit(M / "📝️text/📖️.grammar.semio", [
    (" / set-rule-layout-points / delete-working-nodes\n", " / set-rule-layout-points / delete-working-nodes / connect-working-ports / disconnect-working-edges\n"),
    ('delete-working-nodes = "delete-working-nodes" SP targets\n', 'delete-working-nodes = "delete-working-nodes" SP targets\n' + "".join(leaf["grammar"][0] + "\n" for leaf in LEAVES)),
])
edit(M / "📝️text/🅰️.g4", [
    (" | setRuleLayoutPoints | deleteWorkingNodes ;\n", " | setRuleLayoutPoints | deleteWorkingNodes | connectWorkingPorts | disconnectWorkingEdges ;\n"),
    ("deleteWorkingNodes: 'delete-working-nodes' SP targets ;\n", "deleteWorkingNodes: 'delete-working-nodes' SP targets ;\n" + "".join(leaf["grammar"][1] + "\n" for leaf in LEAVES)),
])
edit(M / "📝️text/🔤️.ebnf", [
    ("     | set rule layout points\n     | delete working nodes ;\n", "     | set rule layout points\n     | delete working nodes\n     | connect working ports\n     | disconnect working edges ;\n"),
    ("delete working nodes = 'delete-working-nodes', space, targets ;\n", "delete working nodes = 'delete-working-nodes', space, targets ;\n" + "".join(leaf["grammar"][2] + "\n" for leaf in LEAVES)),
])

edit(S / "🧬️schema/♻️retirement/🦀️.rs", [
    ("            Self::DeleteWorkingNodes(value) => value.targets.retirement(),\n",
     "            Self::DeleteWorkingNodes(value) => value.targets.retirement(),\n" + "".join(f"            {leaf['retirement'][0]}\n" for leaf in LEAVES)),
])
edit(S / "🧬️schema/♻️retirement/🧫️fixtures/🔣️.json", [
    ('    { "value": { "mutation": "deleteWorkingNodes", "targets": ["a", "Ü"] }, "bytes": 3 }\n',
     '    { "value": { "mutation": "deleteWorkingNodes", "targets": ["a", "Ü"] }, "bytes": 3 },\n'
     + ",\n".join(f'    {{ "value": {leaf["retirement"][1]}, "bytes": {leaf["retirement"][2]} }}' for leaf in LEAVES) + "\n"),
])
edit(S / "🧬️schema/♻️retirement/🧪️tests/🔬️document-retirement/🟦️.ts", [
    ('mutations: { type: "array", minItems: 12, maxItems: 12 }', 'mutations: { type: "array", minItems: 14, maxItems: 14 }'),
    ("      : value.targets !== undefined ? property(value.targets)",
     "      : value.source !== undefined ? text(value.source) + text(value.target) + text(value.kind)\n      : value.targets !== undefined ? property(value.targets)"),
])


def mount(leaf) -> str:
    base = f"🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{leaf['dir']}"
    return "\n".join([
        '                        #[path = "."]',
        f"                        pub mod {leaf['module']} {{",
        f'                            #[path = "{base}/🦀️.rs"]',
        "                            mod component;",
        "                            pub use component::*;",
        f'                            #[path = "{base}/💾️binary/🦀️.rs"]',
        "                            pub mod binary;",
        f'                            #[path = "{base}/🔺️diff/🦀️.rs"]',
        "                            pub mod diff;",
        f'                            #[path = "{base}/↩️inverse/🦀️.rs"]',
        "                            pub mod inverse;",
        "                            #[cfg(test)]",
        f'                            #[path = "{base}/🧪️tests/{leaf["scenario"]}/🦀️.rs"]',
        f"                            mod {leaf['test_module']};",
        f'                            #[path = "{base}/📝️text/🦀️.rs"]',
        "                            pub mod text;",
        "                        }",
    ]) + "\n"


edit(CRATE, [
    ('                            mod tests_deletes_node_a_and_its_edges;\n'
     '                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-working/📝️text/🦀️.rs"]\n'
     '                            pub mod text;\n'
     '                        }\n',
     '                            mod tests_deletes_node_a_and_its_edges;\n'
     '                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-working/📝️text/🦀️.rs"]\n'
     '                            pub mod text;\n'
     '                        }\n' + "".join(mount(leaf) for leaf in LEAVES)),
])

correspondence = M / "🧪️tests/🔬️structural-correspondence/🦀️.rs"
source = correspondence.read_text(encoding="utf-8")
start = source.index('    {\n        let kind = "delete-working-nodes";')
end = source.index("    }\n", source.index('"direct owner {directory} must correspond to the JSON catalog");', start)) + len("    }\n")
blocks = ""
for leaf in LEAVES:
    block = source[start:end].replace('"delete-working-nodes"', f'"{leaf["kind"]}"').replace('"DeleteWorkingNodes"', f'"{leaf["variant"]}"').replace("✂️delete-working", leaf["dir"]).replace("let binary_tag = 11;", f"let binary_tag = {leaf['tag']};")
    if f"let binary_tag = {leaf['tag']};" not in block or "✂️" in block:
        raise SystemExit("structural correspondence block did not retarget")
    blocks += block
edit(correspondence, [('    assert!(!catalog_kinds.contains(&"set-state")', blocks + '    assert!(!catalog_kinds.contains(&"set-state")')])


def catalog(value):
    oracle = value["oracles"][0]
    for old, new in [("all twelve typed mutations", "all fourteen typed mutations"), ("(the twelve verbs", "(the fourteen verbs"), ("the twelve committed", "the fourteen committed")]:
        if oracle["rationale"].count(old) != 1:
            raise SystemExit(f"rationale anchor {old!r}")
        oracle["rationale"] = oracle["rationale"].replace(old, new)
    mutation_catalog = value["mutationCatalogs"][0]
    template = next(entry for entry in value["mutationManifests"][0]["mutations"] if entry["id"] == "delete-working-nodes")
    for leaf in LEAVES:
        mutation_catalog["kinds"].append(leaf["kind"])
        scenario_id = {"connect-working-ports": "connects-c-to-a", "disconnect-working-edges": "cuts-edge-b-c"}[leaf["kind"]]
        mutation_catalog["vectors"].append({"mutationId": leaf["kind"], "sourceMutationDirectoryName": leaf["dir"], "mutationDirectoryName": leaf["dir"], "scenarios": [{"id": scenario_id, "directoryName": leaf["scenario"]}]})
        entry = json.loads(json.dumps(template))
        entry["id"] = leaf["kind"]
        entry["outcomes"] = leaf["outcomes"]
        entry["productionDispatch"] = {**entry["productionDispatch"], "operation": leaf["kind"], "variant": leaf["variant"]}
        value["mutationManifests"][0]["mutations"].append(entry)


edit_json(S / "🔮️oracles/🔣️.json", catalog)

edit(H / "🦀️.rs", [
    ("//! a second implementation of the rule document and all twelve typed mutations, written in Python",
     "//! a second implementation of the rule document and all fourteen typed mutations, written in Python"),
    ("Twelve verbs: three whole-value setters, a set/remove pair over each map, three relative working-graph edits (node drag, node field patch, node delete with its edges), a relative rule-node drag and its absolute layout placement.",
     "Fourteen verbs: three whole-value setters, a set/remove pair over each map, five relative working-graph edits (node drag, node field patch, node delete with its edges, wire draw, wire cut), a relative rule-node drag and its absolute layout placement."),
    ('"set-rule-layout-points", "delete-working-nodes"];', '"set-rule-layout-points", "delete-working-nodes", "connect-working-ports", "disconnect-working-edges"];'),
    ('            "edit-before-fixture" | "drag-working-nodes" | "patch-working-nodes" | "delete-working-nodes" => "beforeFixtureJson",',
     '            "edit-before-fixture" | "drag-working-nodes" | "patch-working-nodes" | "delete-working-nodes" | "connect-working-ports" | "disconnect-working-edges" => "beforeFixtureJson",'),
    ("make all twelve rows", "make all fourteen rows"),
    ("All twelve are accepting, so each", "All fourteen are accepting, so each"),
])

edit(H / "🐍️.py", [
    ("twelve of its typed mutations, in Python, serving as this case's differential oracle.", "fourteen of its typed mutations, in Python, serving as this case's differential oracle."),
    ("— the twelve verbs and their positional", "— the fourteen verbs and their positional"),
    ("`delete-working-nodes targets` and `drag-rule-nodes targets dx dy`", "`delete-working-nodes targets`, `connect-working-ports source target kind`, `disconnect-working-edges targets` and\n  `drag-rule-nodes targets dx dy`"),
    ("* the twelve committed `(before, mutation, after, outcome)`", "* the fourteen committed `(before, mutation, after, outcome)`"),
    ("All twelve are ACCEPTING, so unlike", "All fourteen are ACCEPTING, so unlike"),
    ('"set-rule-layout-points", "delete-working-nodes")\n"""🏷️ Every kind the catalog declares."""',
     '"set-rule-layout-points", "delete-working-nodes", "connect-working-ports", "disconnect-working-edges")\n"""🏷️ Every kind the catalog declares."""'),
    ('    "delete-working-nodes": "deleteWorkingNodes",\n}', '    "delete-working-nodes": "deleteWorkingNodes",\n    "connect-working-ports": "connectWorkingPorts",\n    "disconnect-working-edges": "disconnectWorkingEdges",\n}'),
    ('WORKING = ("drag-working-nodes", "patch-working-nodes", "delete-working-nodes")\n"""🕸️ The three relative working-graph verbs: they move, patch or delete nodes (a delete with every edge touching them) INSIDE the',
     'WORKING = ("drag-working-nodes", "patch-working-nodes", "delete-working-nodes", "connect-working-ports", "disconnect-working-edges")\n"""🕸️ The five relative working-graph verbs: they move, patch or delete nodes (a delete with every edge touching them), draw or cut\nwires INSIDE the'),
    ('''    elif kind in WORKING:
        graph = json.loads(result["beforeFixtureJson"])
        for node in graph["nodes"]:''', '''    elif kind == "connect-working-ports":
        graph = json.loads(result["beforeFixtureJson"])
        edges = graph.setdefault("edges", [])
        if not any(edge.get("source") == mutation["source"] and edge.get("target") == mutation["target"] for edge in edges):
            edges.append({"id": "%s->%s" % (mutation["source"], mutation["target"]), "kind": mutation["kind"], "source": mutation["source"], "target": mutation["target"]})
        result["beforeFixtureJson"] = compact(graph)
    elif kind == "disconnect-working-edges":
        graph = json.loads(result["beforeFixtureJson"])
        if "edges" in graph:
            graph["edges"] = [edge for edge in graph["edges"] if edge.get("id") not in mutation["targets"]]
        result["beforeFixtureJson"] = compact(graph)
    elif kind in WORKING:
        graph = json.loads(result["beforeFixtureJson"])
        for node in graph["nodes"]:'''),
    ("would make all twelve rows", "would make all fourteen rows"),
])

edit(H / "🥒️.feature", [
    ("rule document and all twelve typed mutations,", "rule document and all fourteen typed mutations,"),
    ("(the twelve verbs and their argument", "(the fourteen verbs and their argument"),
    ("from the twelve committed specification vectors.", "from the fourteen committed specification vectors."),
    ("sibling, all twelve of them are ACCEPTING,", "sibling, all fourteen of them are ACCEPTING,"),
    ("is the same twelve verbs against a rule whose", "is the same fourteen verbs against a rule whose"),
    ('''      | delete-working-nodes      | {"mutation":"deleteWorkingNodes","targets":["6947a41b-8c6d-4291-bdd8-96cd535c78fc","17d5dec8-87b2-44a9-84ff-93b7e7419bdd"]} |

  @id-inverse''', '''      | delete-working-nodes      | {"mutation":"deleteWorkingNodes","targets":["6947a41b-8c6d-4291-bdd8-96cd535c78fc","17d5dec8-87b2-44a9-84ff-93b7e7419bdd"]} |
      | connect-working-ports     | ''' + NAKAGIN_CONNECT + ''' |
      | disconnect-working-edges  | ''' + NAKAGIN_DISCONNECT + ''' |

  @id-inverse'''),
    ('''      | delete-working-nodes      | {"mutation":"deleteWorkingNodes","targets":["6947a41b-8c6d-4291-bdd8-96cd535c78fc","17d5dec8-87b2-44a9-84ff-93b7e7419bdd"]} |

  @id-spec-vector''', '''      | delete-working-nodes      | {"mutation":"deleteWorkingNodes","targets":["6947a41b-8c6d-4291-bdd8-96cd535c78fc","17d5dec8-87b2-44a9-84ff-93b7e7419bdd"]} |
      | connect-working-ports     | ''' + NAKAGIN_CONNECT + ''' |
      | disconnect-working-edges  | ''' + NAKAGIN_DISCONNECT + ''' |

  @id-spec-vector'''),
    ("      | delete-working-nodes      | ✂️delete-working  | ✂️deletes  |\n", "      | delete-working-nodes      | ✂️delete-working  | ✂️deletes  |\n      | connect-working-ports     | 🔌️connect-working | 🔌️connects |\n      | disconnect-working-edges  | 🪚️disconnect-working | 🪚️cuts |\n"),
])
print("connect-working-ports + disconnect-working-edges written and registered")
