#!/usr/bin/env python3
"""✂️ W3-T2-TEXT (session 2): registers the fifth relative trinity rewriting leaf, `delete-working-nodes` (written by
`🧪️w3-t2-text-trinity-leaves.py`, its quintet + laws by `🧪️w3-t2-text-trinity-fixtures.py`), in every aggregate surface of
`trinity.rewrite.rule` — Rust aggregate (+ the working-graph node removal it shares), TS union, JSON Schema oneOf, GraphQL and
protobuf aggregates, binary protocol record and tag registry, text grammar (.semio/.g4/.ebnf) and opcode registry, retirement
cursor and corpus (+ the TS retirement oracle), crate mounts, structural correspondence, the oracle catalog and the
cross-language harness (Rust subject KINDS, Python second implementation, feature rows on the real Nakagin rule). Every edit is
anchored on a unique text and the script refuses to run twice. Run from the repo root."""
import json
from pathlib import Path

S = Path("✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any")
M = S / "🧬️schema/🧬️mutations"
H = S / "🧪️tests/♻️mutate-rewrite-1"
CRATE = Path("✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🦀️.rs")
NAKAGIN_DELETE = '{"mutation":"deleteWorkingNodes","targets":["6947a41b-8c6d-4291-bdd8-96cd535c78fc","17d5dec8-87b2-44a9-84ff-93b7e7419bdd"]}'


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


aggregate = M / "🦀️.rs"
if "DeleteWorkingNodes(DeleteWorkingNodes)" in aggregate.read_text(encoding="utf-8"):
    raise SystemExit("already registered")

edit(aggregate, [
    ("pub use super::set_rule_layout_points::{set_rule_layout_points, RuleLayoutPlacement, SetRuleLayoutPoints};\n",
     "pub use super::set_rule_layout_points::{set_rule_layout_points, RuleLayoutPlacement, SetRuleLayoutPoints};\n"
     "pub use super::delete_working_nodes::{delete_working_nodes, DeleteWorkingNodes};\n"),
    ("    SetRuleLayoutPoints(SetRuleLayoutPoints),\n}\n",
     "    SetRuleLayoutPoints(SetRuleLayoutPoints),\n    DeleteWorkingNodes(DeleteWorkingNodes),\n}\n"),
    ("/// 🕸️ Whether `json` is a working graph whose every node and edge kind its own manifest declares.\n",
     "/// ✂️ The working graph JSON `json` without the nodes `targets` names and without every edge with an endpoint on one of them (a\n"
     "/// `node@port` endpoint names its node before the `@`), its `rootNodeId` cleared to `null` when it named a removed node, re-serialized as\n"
     "/// compact JSON with sorted keys, together with the targets the graph lacks. `None` when `json` is no object with a `nodes` array.\n"
     "pub(crate) fn remove_working_graph_nodes(json: &str, targets: &[String]) -> Option<(String, Vec<String>)> {\n"
     "    let mut graph = pack::parse_json(json).ok()?;\n"
     "    let nodes = graph.get_mut(\"nodes\")?.as_array_mut()?;\n"
     "    let found: Vec<String> = nodes.iter().filter_map(|node| node.get(\"id\").and_then(pack::JsonValue::as_str)).filter(|id| targets.iter().any(|target| target == id)).map(str::to_string).collect();\n"
     "    nodes.retain(|node| node.get(\"id\").and_then(pack::JsonValue::as_str).is_none_or(|id| !found.iter().any(|gone| gone == id)));\n"
     "    let removed = |edge: &pack::JsonValue| [\"source\", \"target\"].iter().any(|side| edge.get(side).and_then(pack::JsonValue::as_str).is_some_and(|key| found.iter().any(|gone| gone == semio_s_artifact_trinity_jack::port_node_id(key).unwrap_or(key))));\n"
     "    if let Some(edges) = graph.get_mut(\"edges\").and_then(pack::JsonValue::as_array_mut) {\n"
     "        edges.retain(|edge| !removed(edge));\n"
     "    }\n"
     "    if graph.get(\"rootNodeId\").and_then(pack::JsonValue::as_str).is_some_and(|root| found.iter().any(|gone| gone == root)) {\n"
     "        graph.as_object_mut()?.insert(\"rootNodeId\", pack::JsonValue::Null);\n"
     "    }\n"
     "    let missing = targets.iter().filter(|target| !found.contains(target)).cloned().collect();\n"
     "    Some((pack::json_to_string(&sorted_keys(graph)), missing))\n"
     "}\n\n"
     "/// 🕸️ Whether `json` is a working graph whose every node and edge kind its own manifest declares.\n"),
])

edit(M / "🟦️.ts", [
    ('import type { SetRuleLayoutPoints } from "./📍️set-rule-layout/🟦️.ts";\n',
     'import type { SetRuleLayoutPoints } from "./📍️set-rule-layout/🟦️.ts";\nimport type { DeleteWorkingNodes } from "./✂️delete-working/🟦️.ts";\n'),
    ('  | ({ mutation: "setRuleLayoutPoints" } & SetRuleLayoutPoints);',
     '  | ({ mutation: "setRuleLayoutPoints" } & SetRuleLayoutPoints)\n  | ({ mutation: "deleteWorkingNodes" } & DeleteWorkingNodes);'),
])

edit_json(M / "🔣️.json", lambda schema: schema["oneOf"].append({"$ref": "https://json.schemas.assets.semio-tech.com/s/trinity/rewriting/mutation/delete-working-nodes/schema.json"}))

edit(M / "🔗️.graphql", [
    ("input SetRuleLayoutPointsInput { points: [RuleLayoutPlacementInput!]!, cleared: [String!]! }\n",
     "input SetRuleLayoutPointsInput { points: [RuleLayoutPlacementInput!]!, cleared: [String!]! }\ninput DeleteWorkingNodesInput { targets: [String!]! }\n"),
    ("  setRuleLayoutPoints: SetRuleLayoutPointsInput\n", "  setRuleLayoutPoints: SetRuleLayoutPointsInput\n  deleteWorkingNodes: DeleteWorkingNodesInput\n"),
])

edit(M / "🛰️.proto", [
    ("message SetRuleLayoutPoints { repeated RuleLayoutPlacement points = 1; repeated string cleared = 2; }\n",
     "message SetRuleLayoutPoints { repeated RuleLayoutPlacement points = 1; repeated string cleared = 2; }\nmessage DeleteWorkingNodes { repeated string targets = 1; }\n"),
    ("    SetRuleLayoutPoints set_rule_layout_points = 11;\n", "    SetRuleLayoutPoints set_rule_layout_points = 11;\n    DeleteWorkingNodes delete_working_nodes = 12;\n"),
])

edit(M / "💾️binary/📡️.protocol.semio", [
    ("record set-rule-layout-points tag=10\nfield payload bytes\n", "record set-rule-layout-points tag=10\nfield payload bytes\nrecord delete-working-nodes tag=11\nfield payload bytes\n"),
])
edit(M / "💾️binary/🦀️.rs", [
    ('    ("SetRuleLayoutPoints", super::set_rule_layout_points::binary::BINARY_TAG),\n',
     '    ("SetRuleLayoutPoints", super::set_rule_layout_points::binary::BINARY_TAG),\n    ("DeleteWorkingNodes", super::delete_working_nodes::binary::BINARY_TAG),\n'),
])
edit(M / "📝️text/🦀️.rs", [
    ('    ("SetRuleLayoutPoints", super::set_rule_layout_points::text::TEXT_OPCODE),\n',
     '    ("SetRuleLayoutPoints", super::set_rule_layout_points::text::TEXT_OPCODE),\n    ("DeleteWorkingNodes", super::delete_working_nodes::text::TEXT_OPCODE),\n'),
])
edit(M / "📝️text/📖️.grammar.semio", [
    (" / drag-rule-nodes / set-rule-layout-points\n", " / drag-rule-nodes / set-rule-layout-points / delete-working-nodes\n"),
    ('set-rule-layout-points = "set-rule-layout-points" SP placement-table SP targets\n',
     'set-rule-layout-points = "set-rule-layout-points" SP placement-table SP targets\ndelete-working-nodes = "delete-working-nodes" SP targets\n'),
])
edit(M / "📝️text/🅰️.g4", [
    (" | dragRuleNodes | setRuleLayoutPoints ;\n", " | dragRuleNodes | setRuleLayoutPoints | deleteWorkingNodes ;\n"),
    ("setRuleLayoutPoints: 'set-rule-layout-points' SP placementTable SP targets ;\n",
     "setRuleLayoutPoints: 'set-rule-layout-points' SP placementTable SP targets ;\ndeleteWorkingNodes: 'delete-working-nodes' SP targets ;\n"),
])
edit(M / "📝️text/🔤️.ebnf", [
    ("     | set rule layout points ;\n", "     | set rule layout points\n     | delete working nodes ;\n"),
    ("set rule layout points = 'set-rule-layout-points', space, placement table, space, targets ;\n",
     "set rule layout points = 'set-rule-layout-points', space, placement table, space, targets ;\ndelete working nodes = 'delete-working-nodes', space, targets ;\n"),
])

edit(S / "🧬️schema/♻️retirement/🦀️.rs", [
    ("            Self::SetRuleLayoutPoints(value) => (value.points, value.cleared).retirement(),\n",
     "            Self::SetRuleLayoutPoints(value) => (value.points, value.cleared).retirement(),\n            Self::DeleteWorkingNodes(value) => value.targets.retirement(),\n"),
])
edit(S / "🧬️schema/♻️retirement/🧫️fixtures/🔣️.json", [
    ('    { "value": { "mutation": "setRuleLayoutPoints", "points": [{ "key": "lhs-match", "x": 1, "y": 2 }], "cleared": ["node"] }, "bytes": 29 }\n',
     '    { "value": { "mutation": "setRuleLayoutPoints", "points": [{ "key": "lhs-match", "x": 1, "y": 2 }], "cleared": ["node"] }, "bytes": 29 },\n'
     '    { "value": { "mutation": "deleteWorkingNodes", "targets": ["a", "Ü"] }, "bytes": 3 }\n'),
])
edit(S / "🧬️schema/♻️retirement/🧪️tests/🔬️document-retirement/🟦️.ts", [
    ('mutations: { type: "array", minItems: 11, maxItems: 11 }', 'mutations: { type: "array", minItems: 12, maxItems: 12 }'),
    ("      : value.targets !== undefined ? property(value.targets) + (value.field !== undefined ? text(value.field) + text(value.value) : 16)\n",
     "      : value.targets !== undefined ? property(value.targets) + (value.field !== undefined ? text(value.field) + text(value.value) : value.dx !== undefined ? 16 : 0)\n"),
])

mount = "\n".join([
    '                        #[path = "."]',
    "                        pub mod delete_working_nodes {",
    '                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-working/🦀️.rs"]',
    "                            mod component;",
    "                            pub use component::*;",
    '                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-working/💾️binary/🦀️.rs"]',
    "                            pub mod binary;",
    '                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-working/🔺️diff/🦀️.rs"]',
    "                            pub mod diff;",
    '                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-working/↩️inverse/🦀️.rs"]',
    "                            pub mod inverse;",
    "                            #[cfg(test)]",
    '                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-working/🧪️tests/✂️deletes/🦀️.rs"]',
    "                            mod tests_deletes_node_a_and_its_edges;",
    '                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-working/📝️text/🦀️.rs"]',
    "                            pub mod text;",
    "                        }",
]) + "\n"
edit(CRATE, [
    ('                            mod tests_places_one_rule_node_and_clears_one;\n'
     '                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️set-rule-layout/📝️text/🦀️.rs"]\n'
     '                            pub mod text;\n'
     '                        }\n',
     '                            mod tests_places_one_rule_node_and_clears_one;\n'
     '                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️set-rule-layout/📝️text/🦀️.rs"]\n'
     '                            pub mod text;\n'
     '                        }\n' + mount),
])

correspondence = M / "🧪️tests/🔬️structural-correspondence/🦀️.rs"
source = correspondence.read_text(encoding="utf-8")
start = source.index('    {\n        let kind = "drag-working-nodes";')
end = source.index("    }\n", source.index('"direct owner {directory} must correspond to the JSON catalog");', start)) + len("    }\n")
block = source[start:end].replace('"drag-working-nodes"', '"delete-working-nodes"').replace('"DragWorkingNodes"', '"DeleteWorkingNodes"').replace("✋️drag-working", "✂️delete-working").replace("let binary_tag = 7;", "let binary_tag = 11;")
if "let binary_tag = 11;" not in block or "✋️" in block:
    raise SystemExit("structural correspondence block did not retarget")
edit(correspondence, [('    assert!(!catalog_kinds.contains(&"set-state")', block + '    assert!(!catalog_kinds.contains(&"set-state")')])


def catalog(value):
    oracle = value["oracles"][0]
    for old, new in [("all eleven typed mutations", "all twelve typed mutations"), ("(the eleven verbs", "(the twelve verbs"), ("the eleven committed", "the twelve committed")]:
        if oracle["rationale"].count(old) != 1:
            raise SystemExit(f"rationale anchor {old!r}")
        oracle["rationale"] = oracle["rationale"].replace(old, new)
    mutation_catalog = value["mutationCatalogs"][0]
    mutation_catalog["kinds"].append("delete-working-nodes")
    mutation_catalog["vectors"].append({"mutationId": "delete-working-nodes", "sourceMutationDirectoryName": "✂️delete-working", "mutationDirectoryName": "✂️delete-working", "scenarios": [{"id": "deletes-node-a-and-its-edges", "directoryName": "✂️deletes"}]})
    template = next(entry for entry in value["mutationManifests"][0]["mutations"] if entry["id"] == "drag-working-nodes")
    entry = json.loads(json.dumps(template))
    entry["id"] = "delete-working-nodes"
    entry["outcomes"] = ["applied", "rejected"]
    entry["productionDispatch"] = {**entry["productionDispatch"], "operation": "delete-working-nodes", "variant": "DeleteWorkingNodes"}
    value["mutationManifests"][0]["mutations"].append(entry)


edit_json(S / "🔮️oracles/🔣️.json", catalog)

edit(H / "🦀️.rs", [
    ("//! a second implementation of the rule document and all eleven typed mutations, written in Python",
     "//! a second implementation of the rule document and all twelve typed mutations, written in Python"),
    ("Eleven verbs: three whole-value setters, a set/remove pair over each map, two relative working-graph edits (node drag, node field patch), a relative rule-node drag and its absolute layout placement.",
     "Twelve verbs: three whole-value setters, a set/remove pair over each map, three relative working-graph edits (node drag, node field patch, node delete with its edges), a relative rule-node drag and its absolute layout placement."),
    ('"drag-rule-nodes", "set-rule-layout-points"];', '"drag-rule-nodes", "set-rule-layout-points", "delete-working-nodes"];'),
    ('            "edit-before-fixture" | "drag-working-nodes" | "patch-working-nodes" => "beforeFixtureJson",',
     '            "edit-before-fixture" | "drag-working-nodes" | "patch-working-nodes" | "delete-working-nodes" => "beforeFixtureJson",'),
    ("make all eleven rows", "make all twelve rows"),
    ("All eleven are accepting, so each", "All twelve are accepting, so each"),
])

edit(H / "🐍️.py", [
    ("eleven of its typed mutations, in Python, serving as this case's differential oracle.", "twelve of its typed mutations, in Python, serving as this case's differential oracle."),
    ("— the eleven verbs and their positional", "— the twelve verbs and their positional"),
    ("  `remove-rule-layout-point key`.\n", "  `remove-rule-layout-point key`, the relative `drag-working-nodes targets dx dy`, `patch-working-nodes targets field\n  value`, `delete-working-nodes targets` and `drag-rule-nodes targets dx dy`, and the absolute\n  `set-rule-layout-points placement-table targets`.\n"),
    ("* the eleven committed `(before, mutation, after, outcome)`", "* the twelve committed `(before, mutation, after, outcome)`"),
    ("All eleven are ACCEPTING, so unlike", "All twelve are ACCEPTING, so unlike"),
    ('"drag-rule-nodes", "set-rule-layout-points")\n"""🏷️ Every kind the catalog declares."""',
     '"drag-rule-nodes", "set-rule-layout-points", "delete-working-nodes")\n"""🏷️ Every kind the catalog declares."""'),
    ('    "set-rule-layout-points": "setRuleLayoutPoints",\n}', '    "set-rule-layout-points": "setRuleLayoutPoints",\n    "delete-working-nodes": "deleteWorkingNodes",\n}'),
    ('WORKING = ("drag-working-nodes", "patch-working-nodes")\n"""🕸️ The two relative working-graph verbs: they edit node fields INSIDE the before-fixture document and write it back as compact',
     'WORKING = ("drag-working-nodes", "patch-working-nodes", "delete-working-nodes")\n"""🕸️ The three relative working-graph verbs: they move, patch or delete nodes (a delete with every edge touching them) INSIDE the\nbefore-fixture document and write it back as compact'),
    ('def rule_graph_slots(document):', 'def endpoint(key):\n    """🔌️ The node an edge endpoint names: a `node@port` key names its node before the `@`; any other key is the node itself."""\n    node, separator, port = key.partition("@")\n    return node if separator and node and port else key\n\n\ndef rule_graph_slots(document):'),
    ('''    elif kind in WORKING:
        graph = json.loads(result["beforeFixtureJson"])
        for node in graph["nodes"]:''', '''    elif kind == "delete-working-nodes":
        graph = json.loads(result["beforeFixtureJson"])
        gone = {node.get("id") for node in graph["nodes"] if node.get("id") in mutation["targets"]}
        graph["nodes"] = [node for node in graph["nodes"] if node.get("id") not in gone]
        if "edges" in graph:
            graph["edges"] = [edge for edge in graph["edges"] if endpoint(edge.get("source", "")) not in gone and endpoint(edge.get("target", "")) not in gone]
        if graph.get("rootNodeId") in gone:
            graph["rootNodeId"] = None
        result["beforeFixtureJson"] = compact(graph)
    elif kind in WORKING:
        graph = json.loads(result["beforeFixtureJson"])
        for node in graph["nodes"]:'''),
    ("would make all eleven rows", "would make all twelve rows"),
])

edit(H / "🥒️.feature", [
    ("rule document and all eleven typed mutations,", "rule document and all twelve typed mutations,"),
    ("(the eleven verbs and their argument", "(the twelve verbs and their argument"),
    ("from the eleven committed specification vectors.", "from the twelve committed specification vectors."),
    ("sibling, all eleven of them are ACCEPTING,", "sibling, all twelve of them are ACCEPTING,"),
    ("is the same eleven verbs against a rule whose", "is the same twelve verbs against a rule whose"),
    ('''      | set-rule-layout-points    | {"mutation":"setRuleLayoutPoints","points":[{"key":"lhs-match","x":15.5,"y":-4.0}],"cleared":["a"]} |

  @id-inverse''', '''      | set-rule-layout-points    | {"mutation":"setRuleLayoutPoints","points":[{"key":"lhs-match","x":15.5,"y":-4.0}],"cleared":["a"]} |
      | delete-working-nodes      | ''' + NAKAGIN_DELETE + ''' |

  @id-inverse'''),
    ('''      | set-rule-layout-points    | {"mutation":"setRuleLayoutPoints","points":[{"key":"lhs-match","x":15.5,"y":-4.0}],"cleared":["a"]} |

  @id-spec-vector''', '''      | set-rule-layout-points    | {"mutation":"setRuleLayoutPoints","points":[{"key":"lhs-match","x":15.5,"y":-4.0}],"cleared":["a"]} |
      | delete-working-nodes      | ''' + NAKAGIN_DELETE + ''' |

  @id-spec-vector'''),
    ("      | set-rule-layout-points    | 📍️set-rule-layout  | 📍️places  |\n", "      | set-rule-layout-points    | 📍️set-rule-layout  | 📍️places  |\n      | delete-working-nodes      | ✂️delete-working  | ✂️deletes  |\n"),
])
print("delete-working-nodes registered")
