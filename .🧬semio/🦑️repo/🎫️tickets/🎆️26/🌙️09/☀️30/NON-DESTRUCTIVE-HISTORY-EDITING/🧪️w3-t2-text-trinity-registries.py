#!/usr/bin/env python3
"""🗂️ W3-T2-TEXT (session 2): registers the four relative trinity rewriting leaves (`🧪️w3-t2-text-trinity-leaves.py`) in every
aggregate surface of `trinity.rewrite.rule` — Rust aggregate (+ the working-graph helpers the leaves share), TS union, JSON
Schema oneOf, GraphQL and protobuf aggregates, binary protocol records and tag registry, text grammar (.semio/.g4/.ebnf) and
opcode registry, retirement cursors and corpus, the oracle catalog and the mutate harness KINDS. Every edit is anchored on a
unique line and refuses to run twice. Run from the repo root."""
import json
from pathlib import Path

S = Path("✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any")
M = S / "🧬️schema/🧬️mutations"
LEAVES = [
    ("drag-working-nodes", "DragWorkingNodes", "drag_working_nodes", "✋️drag-working", 7),
    ("patch-working-nodes", "PatchWorkingNodes", "patch_working_nodes", "🩹️patch-working", 8),
    ("drag-rule-nodes", "DragRuleNodes", "drag_rule_nodes", "🫳️drag-rule", 9),
    ("set-rule-layout-points", "SetRuleLayoutPoints", "set_rule_layout_points", "📍️set-rule-layout", 10),
]


def edit(path: Path, pairs):
    text = path.read_text(encoding="utf-8")
    for old, new in pairs:
        if text.count(old) != 1:
            raise SystemExit(f"{path}: anchor not unique ({text.count(old)}): {old[:80]!r}")
        text = text.replace(old, new)
    path.write_text(text, encoding="utf-8")


aggregate = M / "🦀️.rs"
if "DragWorkingNodes(DragWorkingNodes)" in aggregate.read_text(encoding="utf-8"):
    raise SystemExit("already registered")

edit(aggregate, [
    ("pub use super::remove_rule_layout_point::{remove_rule_layout_point, RemoveRuleLayoutPoint};\n",
     "pub use super::remove_rule_layout_point::{remove_rule_layout_point, RemoveRuleLayoutPoint};\n"
     "pub use super::drag_working_nodes::{drag_working_nodes, DragWorkingNodes};\n"
     "pub use super::patch_working_nodes::{patch_working_nodes, PatchWorkingNodes};\n"
     "pub use super::drag_rule_nodes::{drag_rule_nodes, DragRuleNodes};\n"
     "pub use super::set_rule_layout_points::{set_rule_layout_points, RuleLayoutPlacement, SetRuleLayoutPoints};\n"),
    ("    RemoveRuleLayoutPoint(RemoveRuleLayoutPoint),\n}\n//#endregion 🔖️Aggregate\n",
     "    RemoveRuleLayoutPoint(RemoveRuleLayoutPoint),\n    DragWorkingNodes(DragWorkingNodes),\n    PatchWorkingNodes(PatchWorkingNodes),\n    DragRuleNodes(DragRuleNodes),\n    SetRuleLayoutPoints(SetRuleLayoutPoints),\n}\n//#endregion 🔖️Aggregate\n\n"
     "//#region 🕸️WorkingGraph\n"
     "/// 🕸️ The working (before) graph a rule is applied to, or `None` when its JSON does not decode.\n"
     "pub(crate) fn working_graph(json: &str) -> Option<semio_s_artifact_trinity_jack::JackSnapshot> {\n"
     "    semio_s_artifact_trinity_jack::JackSnapshot::from_json(json).ok()\n"
     "}\n\n"
     "/// 🕸️ `graph` with `nodes` and `edges` as the canonical host graph JSON, validated against the graph's own manifest; `None`\n"
     "/// when a node or edge kind is one the manifest does not declare.\n"
     "pub(crate) fn working_graph_json(graph: &semio_s_artifact_trinity_jack::JackSnapshot, nodes: Vec<semio_s_artifact_trinity_jack::Node>, edges: Vec<semio_s_artifact_trinity_jack::Edge>) -> Option<String> {\n"
     "    let scene = semio_s_artifact_trinity_jack::JackWorkingScene { nodes, edges };\n"
     "    let snapshot = semio_s_artifact_trinity_jack::JackSnapshot::with_content(graph.schema.clone(), graph.name.clone(), graph.manifest_id.clone(), graph.manifest.clone(), graph.camera.clone(), scene, graph.root_node_id.clone());\n"
     "    semio_s_artifact_trinity_jack::Graph::from_snapshot(snapshot).and_then(|graph| graph.host_snapshot_json()).ok()\n"
     "}\n\n"
     "/// 🔢️ A canvas offset as a label shows it: two decimals at most, trailing zeros dropped, `(en, de)`.\n"
     "pub(crate) fn offset_text(value: f64) -> (String, String) {\n"
     "    let rounded = (value * 100.0).round() / 100.0;\n"
     "    let text = format!(\"{:.2}\", if rounded == 0.0 { 0.0 } else { rounded });\n"
     "    let en = text.trim_end_matches('0').trim_end_matches('.').to_string();\n"
     "    let de = en.replace('.', \",\");\n"
     "    (en, de)\n"
     "}\n"
     "//#endregion 🕸️WorkingGraph\n"),
])

edit(M / "🟦️.ts", [
    ('import type { RemoveRuleLayoutPoint } from "./🗑️remove-rule-layout/🟦️.ts";\n',
     'import type { RemoveRuleLayoutPoint } from "./🗑️remove-rule-layout/🟦️.ts";\n'
     'import type { DragWorkingNodes } from "./✋️drag-working/🟦️.ts";\n'
     'import type { PatchWorkingNodes } from "./🩹️patch-working/🟦️.ts";\n'
     'import type { DragRuleNodes } from "./🫳️drag-rule/🟦️.ts";\n'
     'import type { SetRuleLayoutPoints } from "./📍️set-rule-layout/🟦️.ts";\n'),
    ('  | ({ mutation: "removeRuleLayoutPoint" } & RemoveRuleLayoutPoint);',
     '  | ({ mutation: "removeRuleLayoutPoint" } & RemoveRuleLayoutPoint)\n'
     '  | ({ mutation: "dragWorkingNodes" } & DragWorkingNodes)\n'
     '  | ({ mutation: "patchWorkingNodes" } & PatchWorkingNodes)\n'
     '  | ({ mutation: "dragRuleNodes" } & DragRuleNodes)\n'
     '  | ({ mutation: "setRuleLayoutPoints" } & SetRuleLayoutPoints);'),
])

catalog = M / "🔣️.json"
schema = json.loads(catalog.read_text(encoding="utf-8"))
schema["oneOf"].extend({"$ref": f"https://json.schemas.assets.semio-tech.com/s/trinity/rewriting/mutation/{kind}/schema.json"} for kind, *_ in LEAVES)
catalog.write_text(json.dumps(schema, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")

edit(M / "🔗️.graphql", [
    ("input RemoveRuleLayoutPointInput { key: String! }\n",
     "input RemoveRuleLayoutPointInput { key: String! }\n"
     "input DragWorkingNodesInput { targets: [String!]!, dx: Float!, dy: Float! }\n"
     "input PatchWorkingNodesInput { targets: [String!]!, field: String!, value: String! }\n"
     "input DragRuleNodesInput { targets: [String!]!, dx: Float!, dy: Float! }\n"
     "input RuleLayoutPlacementInput { key: String!, x: Float!, y: Float! }\n"
     "input SetRuleLayoutPointsInput { points: [RuleLayoutPlacementInput!]!, cleared: [String!]! }\n"),
    ("  removeRuleLayoutPoint: RemoveRuleLayoutPointInput\n}",
     "  removeRuleLayoutPoint: RemoveRuleLayoutPointInput\n  dragWorkingNodes: DragWorkingNodesInput\n  patchWorkingNodes: PatchWorkingNodesInput\n  dragRuleNodes: DragRuleNodesInput\n  setRuleLayoutPoints: SetRuleLayoutPointsInput\n}"),
])

edit(M / "🛰️.proto", [
    ("message RemoveRuleLayoutPoint { string key = 1; }\n",
     "message RemoveRuleLayoutPoint { string key = 1; }\n"
     "message DragWorkingNodes { repeated string targets = 1; double dx = 2; double dy = 3; }\n"
     "message PatchWorkingNodes { repeated string targets = 1; string field = 2; string value = 3; }\n"
     "message DragRuleNodes { repeated string targets = 1; double dx = 2; double dy = 3; }\n"
     "message RuleLayoutPlacement { string key = 1; double x = 2; double y = 3; }\n"
     "message SetRuleLayoutPoints { repeated RuleLayoutPlacement points = 1; repeated string cleared = 2; }\n"),
    ("    RemoveRuleLayoutPoint remove_rule_layout_point = 7;\n",
     "    RemoveRuleLayoutPoint remove_rule_layout_point = 7;\n    DragWorkingNodes drag_working_nodes = 8;\n    PatchWorkingNodes patch_working_nodes = 9;\n    DragRuleNodes drag_rule_nodes = 10;\n    SetRuleLayoutPoints set_rule_layout_points = 11;\n"),
])

edit(M / "💾️binary/📡️.protocol.semio", [
    ("record remove-rule-layout-point tag=6\nfield payload bytes\n",
     "record remove-rule-layout-point tag=6\nfield payload bytes\n" + "".join(f"record {kind} tag={tag}\nfield payload bytes\n" for kind, _, _, _, tag in LEAVES)),
])
edit(M / "💾️binary/🦀️.rs", [
    ('    ("RemoveRuleLayoutPoint", super::remove_rule_layout_point::binary::BINARY_TAG),\n',
     '    ("RemoveRuleLayoutPoint", super::remove_rule_layout_point::binary::BINARY_TAG),\n' + "".join(f'    ("{variant}", super::{snake}::binary::BINARY_TAG),\n' for _, variant, snake, _, _ in LEAVES)),
])
edit(M / "📝️text/🦀️.rs", [
    ('    ("RemoveRuleLayoutPoint", super::remove_rule_layout_point::text::TEXT_OPCODE),\n',
     '    ("RemoveRuleLayoutPoint", super::remove_rule_layout_point::text::TEXT_OPCODE),\n' + "".join(f'    ("{variant}", super::{snake}::text::TEXT_OPCODE),\n' for _, variant, snake, _, _ in LEAVES)),
])
edit(M / "📝️text/📖️.grammar.semio", [
    ("line = edit-before-fixture / edit-lhs / edit-rhs / change-parameter-binding / remove-parameter-binding / change-rule-layout-point / remove-rule-layout-point\n",
     "line = edit-before-fixture / edit-lhs / edit-rhs / change-parameter-binding / remove-parameter-binding / change-rule-layout-point / remove-rule-layout-point / drag-working-nodes / patch-working-nodes / drag-rule-nodes / set-rule-layout-points\n"),
    ('remove-rule-layout-point = "remove-rule-layout-point" SP key\n',
     'remove-rule-layout-point = "remove-rule-layout-point" SP key\n'
     'drag-working-nodes = "drag-working-nodes" SP targets SP number SP number\n'
     'patch-working-nodes = "patch-working-nodes" SP targets SP key SP value\n'
     'drag-rule-nodes = "drag-rule-nodes" SP targets SP number SP number\n'
     'set-rule-layout-points = "set-rule-layout-points" SP placement-table SP targets\n'),
    ('point-block = "{" NL number SP number "}"\n',
     'point-block = "{" NL number SP number "}"\nplacement-table = "{" *( NL key SP number SP number ) "}"\ntargets = "[" *( key [ SP ] ) "]"\n'),
])
edit(M / "📝️text/🅰️.g4", [
    ("line: editBeforeFixture | editLhs | editRhs | changeParameterBinding | removeParameterBinding | changeRuleLayoutPoint | removeRuleLayoutPoint ;\n",
     "line: editBeforeFixture | editLhs | editRhs | changeParameterBinding | removeParameterBinding | changeRuleLayoutPoint | removeRuleLayoutPoint | dragWorkingNodes | patchWorkingNodes | dragRuleNodes | setRuleLayoutPoints ;\n"),
    ("removeRuleLayoutPoint: 'remove-rule-layout-point' SP key ;\n",
     "removeRuleLayoutPoint: 'remove-rule-layout-point' SP key ;\n"
     "dragWorkingNodes: 'drag-working-nodes' SP targets SP number SP number ;\n"
     "patchWorkingNodes: 'patch-working-nodes' SP targets SP key SP value ;\n"
     "dragRuleNodes: 'drag-rule-nodes' SP targets SP number SP number ;\n"
     "setRuleLayoutPoints: 'set-rule-layout-points' SP placementTable SP targets ;\n"),
    ("pointBlock: '{' NL number SP number '}' ;\n",
     "pointBlock: '{' NL number SP number '}' ;\nplacementTable: '{' ( NL key SP number SP number )* '}' ;\ntargets: '[' ( key SP? )* ']' ;\n"),
])
edit(M / "📝️text/🔤️.ebnf", [
    ("     | remove rule layout point ;\n",
     "     | remove rule layout point\n     | drag working nodes\n     | patch working nodes\n     | drag rule nodes\n     | set rule layout points ;\n"),
    ("remove rule layout point = 'remove-rule-layout-point', space, key ;\n",
     "remove rule layout point = 'remove-rule-layout-point', space, key ;\n"
     "drag working nodes = 'drag-working-nodes', space, targets, space, number, space, number ;\n"
     "patch working nodes = 'patch-working-nodes', space, targets, space, key, space, value ;\n"
     "drag rule nodes = 'drag-rule-nodes', space, targets, space, number, space, number ;\n"
     "set rule layout points = 'set-rule-layout-points', space, placement table, space, targets ;\n"),
    ("point block = '{', newline, number, space, number, '}' ;\n",
     "point block = '{', newline, number, space, number, '}' ;\nplacement table = '{', { newline, key, space, number, space, number }, '}' ;\ntargets = '[', { key, [ space ] }, ']' ;\n"),
])

retirement = S / "🧬️schema/♻️retirement/🦀️.rs"
edit(retirement, [
    ("use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;\n",
     "use crate::standards::v1::subsets::any::schema::mutations::{RewriteRuleMutation, RuleLayoutPlacement};\n"),
    ("store::artifact_retire_struct!(LayoutPoint { x, y });\n",
     "store::artifact_retire_struct!(LayoutPoint { x, y });\nstore::artifact_retire_struct!(RuleLayoutPlacement { key, x, y });\n"),
    ("            Self::RemoveRuleLayoutPoint(value) => value.key.retirement(),\n",
     "            Self::RemoveRuleLayoutPoint(value) => value.key.retirement(),\n"
     "            Self::DragWorkingNodes(value) => (value.targets, (value.dx, value.dy)).retirement(),\n"
     "            Self::PatchWorkingNodes(value) => (value.targets, (value.field, value.value)).retirement(),\n"
     "            Self::DragRuleNodes(value) => (value.targets, (value.dx, value.dy)).retirement(),\n"
     "            Self::SetRuleLayoutPoints(value) => (value.points, value.cleared).retirement(),\n"),
])
corpus_path = S / "🧬️schema/♻️retirement/🧫️fixtures/🔣️.json"
edit(corpus_path, [
    ('    { "value": { "mutation": "removeRuleLayoutPoint", "key": "node" }, "bytes": 4 }\n',
     '    { "value": { "mutation": "removeRuleLayoutPoint", "key": "node" }, "bytes": 4 },\n'
     '    { "value": { "mutation": "dragWorkingNodes", "targets": ["a", "Ü"], "dx": 1.5, "dy": -2 }, "bytes": 19 },\n'
     '    { "value": { "mutation": "patchWorkingNodes", "targets": ["a"], "field": "name", "value": "😀" }, "bytes": 9 },\n'
     '    { "value": { "mutation": "dragRuleNodes", "targets": ["rhs-set-0"], "dx": 0, "dy": 80 }, "bytes": 25 },\n'
     '    { "value": { "mutation": "setRuleLayoutPoints", "points": [{ "key": "lhs-match", "x": 1, "y": 2 }], "cleared": ["node"] }, "bytes": 29 }\n'),
])
print("registries patched")
