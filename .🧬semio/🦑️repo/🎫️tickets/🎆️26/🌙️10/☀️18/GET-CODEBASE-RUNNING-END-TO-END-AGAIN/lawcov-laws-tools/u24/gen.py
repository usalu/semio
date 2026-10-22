import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
read = lambda n: open(os.path.join(HERE, "..", n), encoding="utf8").read()
oracle = read("pylib.py") + "\n" + read("ptree.py") + "\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
ST = "✏s/🔌plugins/🗄stdio/🗿artifacts/🔸foo/🏅standards/🔖1/🪆subsets/✳any"
FACET = ST + "/🧬schema/📸snapshot/"
RULE = "policyFacetMirrorDriftBreaches"
cases = []


def consistent():
    return {
        FACET + "🦀.rs": ["pub struct Node {", "    pub node_id: u32,", "    pub label: String,", "}"],
        FACET + "🟦.ts": ["export interface Node {", "  nodeId: number;", "  label: string;", "}"],
        FACET + "🔗.graphql": ["type Node {", "  nodeId: Int", "  label: String", "}"],
        FACET + "🔣.json": ['{"properties": {"nodeId": {}, "label": {}}}'],
        FACET + "🛰.proto": ["message Node {", "  uint32 nodeId = 1;", "  string label = 2;", "}"],
    }


def case(id_, files, drift):
    cases.append({"id": id_, "rule": RULE, "files": files, "expect": [["facet-mirror-drift-" + FACET + "🦀.rs", "stdio-artifacts/facet-mirror-drift", "low", 0]] if drift else []})


case("mirrored-facets-are-clean", consistent(), False)
f = consistent()
f.pop(FACET + "🔗.graphql")
case("a-missing-sibling-mirror-drifts", f, True)
f = consistent()
f[FACET + "🟦.ts"] = ["export interface Node {", "  nodeId: number;", "  label: string;", "  unrelatedThing: string;", "}"]
case("an-extra-sibling-field-drifts", f, True)
f = consistent()
f[FACET + "🔣.json"] = ["{not json"]
case("an-unparseable-json-mirror-drifts", f, True)
f = consistent()
f[FACET + "🛰.proto"] = ["message Node {", "  string label = 2;", "}"]
case("a-field-missing-from-the-mirror-drifts", f, True)
f = consistent()
f[FACET + "🟦.ts"] = ["export interface Node {", "  nodeIdentifier: number;", "  label: string;", "}"]
case("a-longer-name-containing-the-field-satisfies-the-mirror", f, False)
f = {k: v for k, v in consistent().items()}
f[FACET + "🦀.rs"] = ['#[serde(tag = "kind")]', "pub enum Op {", "    Add { x: u8 },", "    Remove(u8),", "}"]
f[FACET + "🟦.ts"] = ["export type Op = { kind: string; x: number; add: number; remove: number };"]
f[FACET + "🔗.graphql"] = ["type Op {", "  x: Int", "  kind: String", "}"]
f[FACET + "🔣.json"] = ['{"properties": {"x": {}, "kind": {}, "schema": {}}}']
f[FACET + "🛰.proto"] = ["message Op {", "  uint32 x = 1;", "  string kind = 2;", "}"]
case("serde-tags-and-variants-are-known-names", f, False)
f = consistent()
f[FACET + "🦀.rs"] = ["// pub struct Old { ghost_field: u8 }", 'const S: &str = "hidden_field: u8";', "pub struct Node {", "    pub node_id: u32,", "    pub label: String,", "}"]
case("comments-and-strings-hide-fields", f, False)
cases.append({"id": "subsets-without-a-snapshot-schema-are-not-checked", "rule": RULE, "files": {ST + "/🧬schema/🔺diff/🦀.rs": ["pub struct X { pub a_field: u8 }"]}, "expect": []})
schema = std_schema("Facet Mirror Drift Law Corpus", [RULE])
ts = std_ts(
    "🪞 The facet-mirror-drift law reports exactly the drifting Rust facets its language-agnostic corpus names (missing, extra, unparseable and absent mirror fields across TypeScript, GraphQL, JSON and Protobuf), agrees with an independent Python model and rejects a malformed corpus.",
    'import { policyFacetMirrorDriftBreaches } from "../../🟦.ts";',
    "  policyFacetMirrorDriftBreaches: (root) => policyFacetMirrorDriftBreaches(root),",
)
folder, plen = write_test("⚖laws/🧩plugin-consistency/🧬schema-overhaul-s2/🪞facet-mirror-drift", "🪞drift", ts, {"version": 1, "cases": cases}, schema, oracle)
print(folder, plen, len(cases))
