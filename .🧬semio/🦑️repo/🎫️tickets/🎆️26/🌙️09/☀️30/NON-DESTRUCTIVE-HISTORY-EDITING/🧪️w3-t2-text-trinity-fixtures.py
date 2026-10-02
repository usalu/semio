#!/usr/bin/env python3
"""🧫️ W3-T2-TEXT (session 2): authors, by hand, the committed quintets (before, mutation, after, diff, outcome) of the five
relative trinity rewriting leaves and the per-leaf Rust laws that replay them. Every expected value is computed here
independently of the Rust implementation: working-graph leaves rewrite the graph JSON as compact JSON with sorted keys
(`json.dumps(sort_keys=True, separators=(",", ":"), ensure_ascii=False)`; a delete also drops every edge with an endpoint on a
removed node, `node@port` naming its node before the `@`, and the root it removed), rule-node drags read the default slots the rule
editor draws (LHS match (0,0), WHERE (220,80); RHS rows create/merge/set/delete/parameter at y = 0/80/160/240/320, 220 apart).
Run from the repo root. Idempotent."""
import json
from pathlib import Path

S = Path("✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any")
FIXTURES = S / "🧫️fixtures/🧬️mutations"
MUTATIONS = S / "🧬️schema/🧬️mutations"


def compact(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


GRAPH = {"schema": "trinity.graph", "name": "pair", "manifestId": "nakagin", "nodes": [{"id": "a", "kind": "Piece", "name": "a", "x": 0.0, "y": 0.0}, {"id": "b", "kind": "Piece", "name": "b", "x": 100.0, "y": 40.0}], "edges": []}
LHS = compact({"pattern": {"leftVar": "a", "leftKind": "Piece"}, "whereClause": "a.tier = 0"})
RHS = compact({"set": [{"var": "a", "prop": "label", "value": "$label"}], "parameters": [{"name": "label", "kind": "string", "default": "core"}]})


def snapshot(graph_json, layout):
    return {"beforeFixtureJson": graph_json, "lhsJson": LHS, "rhsJson": RHS, "parameterBindings": {"label": "core"}, "ruleLayout": layout}


def diff(**members):
    return {"beforeFixtureJson": None, "lhsJson": None, "rhsJson": None, "parameterBindings": None, "ruleLayout": None, **members}


def entries(rows):
    return {"entries": [{"key": key, "precondition": precondition, "operation": operation} for key, precondition, operation in rows]}


def moved(graph, changes):
    result = json.loads(json.dumps(graph))
    for node in result["nodes"]:
        node.update(changes.get(node["id"], {}))
    return compact(result)


def deleted(graph, targets):
    result = json.loads(json.dumps(graph))
    gone = {node["id"] for node in result["nodes"] if node["id"] in targets}

    def endpoint(key):
        node, separator, port = key.partition("@")
        return node if separator and node and port else key

    result["nodes"] = [node for node in result["nodes"] if node["id"] not in gone]
    result["edges"] = [edge for edge in result["edges"] if endpoint(edge["source"]) not in gone and endpoint(edge["target"]) not in gone]
    if result.get("rootNodeId") in gone:
        result["rootNodeId"] = None
    return compact(result)


CHAIN = {"schema": "trinity.graph", "name": "chain", "manifestId": "nakagin", "nodes": [{"id": "a", "kind": "Piece", "name": "a", "x": 0.0, "y": 0.0}, {"id": "b", "kind": "Piece", "name": "b", "x": 100.0, "y": 40.0}, {"id": "c", "kind": "Piece", "name": "c", "x": 200.0, "y": 80.0}], "edges": [{"id": "e-ab", "kind": "Connection", "source": "a@out", "target": "b@in"}, {"id": "e-bc", "kind": "Connection", "source": "b@out", "target": "c@in"}], "rootNodeId": "a"}
BEFORE_JSON = json.dumps(GRAPH, separators=(",", ":"), ensure_ascii=False)
CHAIN_JSON = json.dumps(CHAIN, separators=(",", ":"), ensure_ascii=False)
DELETED = deleted(CHAIN, ["a"])
DRAGGED = moved(GRAPH, {"b": {"x": 120.0, "y": 30.0}})
RENAMED = moved(GRAPH, {"a": {"name": "Core"}})

CASES = [
    {
        "dir": "✋️drag-working", "case": "✋️moves", "kind": "drag-working-nodes", "variant": "DragWorkingNodes", "verb": "drag", "entity": "working-nodes", "record": "DraggedWorkingNodes",
        "note": "drags node b of the two-node working graph by (20, -10); node a and every other member stay untouched",
        "before": snapshot(BEFORE_JSON, {"lhs-match": {"x": 10.0, "y": 0.0}}),
        "mutation": {"mutation": "dragWorkingNodes", "targets": ["b"], "dx": 20.0, "dy": -10.0},
        "after": snapshot(DRAGGED, {"lhs-match": {"x": 10.0, "y": 0.0}}),
        "diff": diff(beforeFixtureJson=DRAGGED),
        "inverse": ("EditBeforeFixture", "new_before_fixture_json", "before.before_fixture_json"),
    },
    {
        "dir": "🩹️patch-working", "case": "🩹️renames", "kind": "patch-working-nodes", "variant": "PatchWorkingNodes", "verb": "patch", "entity": "working-nodes", "record": "PatchedWorkingNodes",
        "note": "renames node a of the two-node working graph to Core; node b and every other member stay untouched",
        "before": snapshot(BEFORE_JSON, {"lhs-match": {"x": 10.0, "y": 0.0}}),
        "mutation": {"mutation": "patchWorkingNodes", "targets": ["a"], "field": "name", "value": "Core"},
        "after": snapshot(RENAMED, {"lhs-match": {"x": 10.0, "y": 0.0}}),
        "diff": diff(beforeFixtureJson=RENAMED),
        "inverse": ("EditBeforeFixture", "new_before_fixture_json", "before.before_fixture_json"),
    },
    {
        "dir": "✂️delete-working", "case": "✂️deletes", "kind": "delete-working-nodes", "variant": "DeleteWorkingNodes", "verb": "delete", "entity": "working-nodes", "record": "DeletedWorkingNodes",
        "note": "deletes the root node a of the three-node chain with its edge a→b; nodes b and c, the edge b→c and every other member stay untouched",
        "before": snapshot(CHAIN_JSON, {"lhs-match": {"x": 10.0, "y": 0.0}}),
        "mutation": {"mutation": "deleteWorkingNodes", "targets": ["a"]},
        "after": snapshot(DELETED, {"lhs-match": {"x": 10.0, "y": 0.0}}),
        "diff": diff(beforeFixtureJson=DELETED),
        "inverse": ("EditBeforeFixture", "new_before_fixture_json", "before.before_fixture_json"),
    },
    {
        "dir": "🫳️drag-rule", "case": "🫳️moves", "kind": "drag-rule-nodes", "variant": "DragRuleNodes", "verb": "drag", "entity": "rule-nodes", "record": "DraggedRuleNodes",
        "note": "drags the LHS match (laid out at (10, 0)) and the RHS set clause (default slot (0, 160)) by (20, 40)",
        "before": snapshot(BEFORE_JSON, {"lhs-match": {"x": 10.0, "y": 0.0}}),
        "mutation": {"mutation": "dragRuleNodes", "targets": ["lhs-match", "rhs-set-0"], "dx": 20.0, "dy": 40.0},
        "after": snapshot(BEFORE_JSON, {"lhs-match": {"x": 30.0, "y": 40.0}, "rhs-set-0": {"x": 20.0, "y": 200.0}}),
        "diff": diff(ruleLayout=entries([("lhs-match", "any", {"kind": "set", "value": {"x": 30.0, "y": 40.0}}), ("rhs-set-0", "any", {"kind": "set", "value": {"x": 20.0, "y": 200.0}})])),
        "inverse": ("SetRuleLayoutPoints", None, None),
    },
    {
        "dir": "📍️set-rule-layout", "case": "📍️places", "kind": "set-rule-layout-points", "variant": "SetRuleLayoutPoints", "verb": "set", "entity": "rule-layout-points", "record": "SetRuleLayoutPoints",
        "note": "places the RHS parameter clause at (40, 300) and clears the LHS match back to its default slot",
        "before": snapshot(BEFORE_JSON, {"lhs-match": {"x": 10.0, "y": 0.0}}),
        "mutation": {"mutation": "setRuleLayoutPoints", "points": [{"key": "rhs-parameter-0", "x": 40.0, "y": 300.0}], "cleared": ["lhs-match"]},
        "after": snapshot(BEFORE_JSON, {"rhs-parameter-0": {"x": 40.0, "y": 300.0}}),
        "diff": diff(ruleLayout=entries([("lhs-match", "present", {"kind": "remove"}), ("rhs-parameter-0", "any", {"kind": "set", "value": {"x": 40.0, "y": 300.0}})])),
        "inverse": ("SetRuleLayoutPoints", None, None),
    },
]

TEST = r'''//! 🧪️ `{kind}` fixture — `{case}`: {note}.
//!
//! Source of truth is the committed JSON quintet (`🧫️fixtures/🧬️mutations/{dir}/{case}`), authored by
//! `T/🧪️w3-t2-text-trinity-fixtures.py` independently of this implementation (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).

use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::RewritingSnapshot;
use crate::{{apply_rewrite_rule_mutation, inverse_rewrite_rule_mutation}};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{dir}/{case}/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{dir}/{case}/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{dir}/{case}/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{dir}/{case}/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{dir}/{case}/🎯️outcome/🔣️.json");

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
    assert_eq!(snapshot, expected_after(), "{kind}/{case}: applied state differs from the committed after-snapshot");
}}

/// ↩️ `{kind}` undoes with exactly ONE row of `{inverse_variant}`, which restores `before` exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {{
    let base = before();
    let inverse = inverse_rewrite_rule_mutation(&base, &mutation());
    assert!(matches!(inverse.as_slice(), [RewriteRuleMutation::{inverse_variant}(_)]), "{kind} undoes with one {inverse_variant} row, got {{inverse:?}}");
    let mut snapshot = base.clone();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation()).expect("forward applies");
    for step in &inverse {{
        apply_rewrite_rule_mutation(&mut snapshot, step).expect("inverse step applies");
    }}
    assert_eq!(snapshot, base, "{kind}/{case}: the inverse did not restore the before-snapshot");
}}

/// 🔣️ The committed snapshots and payload are canonical: decode → encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {{
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {{
        let decoded: RewritingSnapshot = pack::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&decoded)).expect("snapshot encodes");
        assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(text).expect("snapshot reparses"), "{kind}/{case}: committed {{label}} JSON is not canonical");
    }}
    let reencoded = serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&mutation())).expect("mutation encodes");
    assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(MUTATION).expect("mutation reparses"), "{kind}/{case}: committed mutation JSON is not canonical");
}}

/// 🎯️ The declared outcome holds — an applied change with no diagnostics — and the fixture is bound to `{kind}`'s descriptor.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {{
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"));
    let produced = <RewriteRuleMutation as protocol::Mutation<RewritingSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "{kind}/{case} declares no diagnostics, but got {{:?}}", produced.messages());
    let semantics = <RewriteRuleMutation as protocol::SemanticMutation<RewritingSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("{verb}", "{entity}", "{kind}", "{record}"));
}}

/// 🔺️ The sparse delta is exactly the committed diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {{
    let outcome = <RewriteRuleMutation as protocol::Mutation<RewritingSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::from_str::<serde_json::Value>(&pack::to_json_string(outcome.diff())).expect("produced diff encodes");
    assert_eq!(produced, serde_json::from_str::<serde_json::Value>(DIFF).expect("committed diff decodes"), "{kind}/{case}: the produced diff differs from the committed one");
}}

/// 🩹️ Applying the committed diff to `before` yields the committed `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {{
    let decoded: RewritingDiff = pack::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <RewritingDiff as protocol::MutationDiff<RewritingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies");
    assert_eq!(produced, expected_after(), "{kind}/{case}: the committed diff did not carry before to after");
}}
'''


def write(path: Path, value) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(value if isinstance(value, str) else json.dumps(value, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


for case in CASES:
    root = FIXTURES / case["dir"] / case["case"]
    write(root / "📸️snapshot/⬅️before/🔣️.json", case["before"])
    write(root / "📸️snapshot/➡️after/🔣️.json", case["after"])
    write(root / "🦠️mutation/🔣️.json", case["mutation"])
    write(root / "🔺️diff/🔣️.json", case["diff"])
    write(root / "🎯️outcome/🔣️.json", {"status": "applied"})
    test = TEST.format(kind=case["kind"], case=case["case"], dir=case["dir"], note=case["note"], verb=case["verb"], entity=case["entity"], record=case["record"], inverse_variant=case["inverse"][0])
    write(MUTATIONS / case["dir"] / "🧪️tests" / case["case"] / "🦀️.rs", test)
    print(f"{case['dir']}/{case['case']}: quintet + laws written")
