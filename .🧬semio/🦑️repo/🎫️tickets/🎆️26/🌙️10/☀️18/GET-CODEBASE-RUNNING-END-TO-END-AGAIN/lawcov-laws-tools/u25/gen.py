import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
read = lambda n: open(os.path.join(HERE, "..", n), encoding="utf8").read()
oracle = read("pylib.py") + "\n" + read("ptree.py") + "\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
ST = "✏s/🔌plugins/🗄stdio/🗿artifacts"
x = []
SUB = ST + "/🔸foo/🏅standards/🔖1/🪆subsets/✳any"
VM = "stdio-artifacts/vcs-machinery-ban"
DA = "stdio-artifacts/diff-algebra"
cases = []


def b(i, kind, prio="high"):
    return [i, kind, prio, 0]


cases.append({"id": "stdio-codecs-use-no-vcs-collection-machinery", "rule": "policyStdioVcsMachineryBanBreaches", "files": {
    ST + "/🔸a/x.rs": ["use vcs::CollectionDiff;"], ST + "/🔸a/y.rs": ["// Patchable is mentioned in a comment", 'let s = "Identified";'], ST + "/🔸b/z.rs": ["impl Identified for Z {}"], ST + "/🔸b/w.rs": ["struct MyIdentified;", "type Patchables = u8;"],
    "✏s/🔌plugins/🔭p/o.rs": ["use vcs::CollectionMutation;"],
}, "expect": [b("stdio-vcs-machinery-ban-" + ST + "/🔸a/x.rs", VM), b("stdio-vcs-machinery-ban-" + ST + "/🔸b/z.rs", VM)]})
cases.append({"id": "snapshot-owning-diff-facets-implement-the-diff-algebra", "rule": "policyDiffAlgebraBreaches", "files": {
    SUB + "/🧬schema/📸snapshot/x": x, SUB + "/🧬schema/🔺diff/🦀.rs": ["pub struct D;"],
    ST + "/🔸ok/🏅standards/🔖1/🪆subsets/✳any/🧬schema/📸snapshot/x": x, ST + "/🔸ok/🏅standards/🔖1/🪆subsets/✳any/🧬schema/🔺diff/🦀.rs": ["impl<T> DiffAlgebra for D<T> {}"],
    ST + "/🔸cm/🏅standards/🔖1/🪆subsets/✳any/🧬schema/📸snapshot/x": x, ST + "/🔸cm/🏅standards/🔖1/🪆subsets/✳any/🧬schema/🔺diff/🦀.rs": ["// impl DiffAlgebra for D {}"],
    ST + "/🔸nodiff/🏅standards/🔖1/🪆subsets/✳any/🧬schema/📸snapshot/x": x,
    ST + "/🔸ns/🏅standards/🔖1/🪆subsets/✳any/🧬schema/🔺diff/🦀.rs": ["pub struct D;"],
}, "expect": [b("diff-algebra-missing-" + SUB + "/🧬schema/🔺diff/🦀.rs", DA, "medium"), b("diff-algebra-missing-" + ST + "/🔸cm/🏅standards/🔖1/🪆subsets/✳any/🧬schema/🔺diff/🦀.rs", DA, "medium")]})
cases.append({"id": "sniff-functions-must-read-their-input", "rule": "policySniffRealityBreaches", "files": {
    "a/x.rs": ["fn sniff(_bytes: &[u8]) -> bool { true }"], "a/y.rs": ["fn sniff(bytes: &[u8]) -> bool { bytes.len() > 3 }"], "a/z.rs": ["fn sniff (_x: u8) {}"], "a/w.rs": ["fn other(_x: u8) {}"],
}, "expect": [["sniff-reality-a/x.rs", "artifact-io/sniff-reality", "high", 0]]})

machinery_rows = [c for c in cases if c["rule"] != "policySniffRealityBreaches"]
parent_rows = [c for c in cases if c["rule"] == "policySniffRealityBreaches"]
parent_rows.append({"id": "aggregate-delegates-to-the-machinery-and-diff-algebra-laws", "rule": "policySchemaOverhaulS2Breaches", "kindFilter": "stdio-artifacts/vcs-machinery-ban", "files": machinery_rows[0]["files"], "expect": machinery_rows[0]["expect"]})
parent_rows.append({"id": "aggregate-also-runs-the-diff-algebra-law", "rule": "policySchemaOverhaulS2Breaches", "kindFilter": "stdio-artifacts/diff-algebra", "files": machinery_rows[1]["files"], "expect": machinery_rows[1]["expect"]})
schema_m = std_schema("Machinery Bans Law Corpus", ["policyStdioVcsMachineryBanBreaches", "policyDiffAlgebraBreaches"])
ts_m = std_ts(
    "🚫 The stdio machinery-ban and diff-algebra laws report exactly the breaches their language-agnostic corpus names, agree with an independent Python model and reject a malformed corpus.",
    'import { policyDiffAlgebraBreaches, policyStdioVcsMachineryBanBreaches } from "../../🟦.ts";',
    "  policyStdioVcsMachineryBanBreaches: (root) => policyStdioVcsMachineryBanBreaches(root),\n  policyDiffAlgebraBreaches: (root) => policyDiffAlgebraBreaches(root),",
)
print(write_test("⚖laws/🧩plugin-consistency/🧬schema-overhaul-s2/🚫machinery-bans", "🚫machinery", ts_m, {"version": 1, "cases": machinery_rows}, schema_m, oracle)[0])
schema_p = std_schema("Schema Overhaul S2 Law Corpus", ["policySniffRealityBreaches", "policySchemaOverhaulS2Breaches"])
ts_p = std_ts(
    "🧬 The schema-overhaul S2 laws report exactly the unused-sniff-parameter breaches their language-agnostic corpus names and delegate to the machinery-ban and diff-algebra laws, agree with an independent Python model and reject a malformed corpus.",
    'import { policySchemaOverhaulS2Breaches, policySniffRealityBreaches } from "../../🟦.ts";',
    "  policySniffRealityBreaches: (root) => policySniffRealityBreaches(root),\n  policySchemaOverhaulS2Breaches: (root) => policySchemaOverhaulS2Breaches(root),",
)
print(write_test("⚖laws/🧩plugin-consistency/🧬schema-overhaul-s2", "🧬overhaul-s2", ts_p, {"version": 1, "cases": parent_rows}, schema_p, oracle)[0])
