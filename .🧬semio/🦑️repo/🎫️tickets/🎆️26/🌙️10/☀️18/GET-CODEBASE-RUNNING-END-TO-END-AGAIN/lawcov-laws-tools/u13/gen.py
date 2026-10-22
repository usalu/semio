import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
read = lambda n: open(os.path.join(HERE, "..", n), encoding="utf8").read()
oracle = read("pylib.py") + "\n" + read("ptree.py") + "\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
cases = []
OWN = "✏s/🔌plugins/🔭alpha"
CR = [{"ownerRel": OWN, "pluginId": "alpha"}]
ART = OWN + "/🗿artifacts/🔸a"
EX = ART + "/📚examples"
K = "taxonomy/semio-examples"
x = []
nonempty = ["x"]


def case(id_, rule, files, expect, crates=CR):
    cases.append({"id": id_, "rule": rule, "files": files, "expect": expect, "crates": crates})


def b(i, kind, prio="medium"):
    return [i, kind, prio, 0]


def complete(rel):
    return {rel + "/🦀.rs": x, rel + "/🟦.ts": x, rel + "/🖼assets/data.json": nonempty, rel + "/🧪tests/🦀.rs": x, rel + "/🧪tests/🟦.ts": x}


case("complete-example-sets-are-clean", "policySemioArtifactExamplesBreaches", {**complete(EX + "/🎬demo-one")}, [])
S = "policySemioArtifactExamplesBreaches"
case("plugin-root-examples-and-missing-artifact-examples", S, {OWN + "/📚examples/x": x, ART + "/🦀.rs": x, OWN + "/🗿artifacts/🔸b/📚examples/🎬ok/🦀.rs": x},
     [b("semio-examples-plugin-root-" + OWN, K, "high"), b("semio-examples-missing-" + ART + "/📚examples", K)] if False else
     [b("semio-examples-plugin-root-" + OWN, K, "high"), b("semio-examples-missing-" + ART + "/📚examples", K)] + [b("semio-examples-" + k + "-" + OWN + "/🗿artifacts/🔸b/📚examples/🎬ok", K) for k in ("leaf-ts", "assets", "tests")])
bad = EX + "/♻reuse"
case("example-unit-requirements", S, {
    bad + "/🦀.rs": x, bad + "/🎒packs/x": x, bad + "/🖼assets/empty.json": x, bad + "/🖼assets/full.json": nonempty,
    EX + "/noemoji/🦀.rs": x, EX + "/🎬Bad_Slug/🦀.rs": x,
    EX + "/🎬no-tests/🦀.rs": x, EX + "/🎬no-tests/🟦.ts": x, EX + "/🎬no-tests/🖼assets/a": nonempty,
    EX + "/🎬half-tests/🦀.rs": x, EX + "/🎬half-tests/🟦.ts": x, EX + "/🎬half-tests/🖼assets/a": nonempty, EX + "/🎬half-tests/🧪tests/🦀.rs": x,
}, [
    b("semio-examples-slug-" + bad, K), b("semio-examples-plural-%s-🎒packs" % bad, K), b("semio-examples-leaf-ts-" + bad, K), b("semio-examples-zero-byte-asset-%s-empty.json" % bad, K), b("semio-examples-tests-" + bad, K),
    b("semio-examples-slug-" + EX + "/noemoji", K), b("semio-examples-leaf-ts-" + EX + "/noemoji", K), b("semio-examples-assets-" + EX + "/noemoji", K), b("semio-examples-tests-" + EX + "/noemoji", K),
    b("semio-examples-slug-" + EX + "/🎬Bad_Slug", K), b("semio-examples-leaf-ts-" + EX + "/🎬Bad_Slug", K), b("semio-examples-assets-" + EX + "/🎬Bad_Slug", K), b("semio-examples-tests-" + EX + "/🎬Bad_Slug", K),
    b("semio-examples-tests-" + EX + "/🎬no-tests", K), b("semio-examples-test-ts-" + EX + "/🎬half-tests", K),
])
SUB = ART + "/🏅standards/🔖1/🪆subsets/✳any"
SUB2 = ART + "/🏅standards/🔖1/🪆subsets/✳fem"
case("surface-examples-are-validated-and-may-be-absent", S, {
    EX + "/🎬ok/🦀.rs": x, SUB + "/👁viewer/📚examples/🎬v/🦀.rs": x, SUB + "/✏editor/🎭modes/x": x, SUB2 + "/👁viewer/📚examples/readme.txt": x,
}, [b("semio-examples-leaf-ts-" + EX + "/🎬ok", K), b("semio-examples-assets-" + EX + "/🎬ok", K), b("semio-examples-tests-" + EX + "/🎬ok", K),
    b("semio-examples-leaf-ts-" + SUB + "/👁viewer/📚examples/🎬v", K), b("semio-examples-assets-" + SUB + "/👁viewer/📚examples/🎬v", K), b("semio-examples-tests-" + SUB + "/👁viewer/📚examples/🎬v", K),
    b("semio-examples-surface-empty-" + SUB2 + "/👁viewer/📚examples", K)])
case("non-plugin-owner-reports-at-low-priority", S, {"🧰framework/🔨modules/🔸o/🗿artifacts/🔸a/🦀.rs": x}, [b("semio-examples-missing-🧰framework/🔨modules/🔸o/🗿artifacts/🔸a/📚examples", K, "low")], crates=[{"ownerRel": "🧰framework/🔨modules/🔸o", "pluginId": ""}])
CF = "taxonomy/component-file"
C = "policyComponentFileBreaches"
case("component-leaves-are-named-by-the-taxonomy", C, {
    ART + "/⚙engine/🦀.rs": x, ART + "/⚙engine/helper.rs": x,
    ART + "/🧬schema/a.rs": x, ART + "/🧬schema/b.ts": x,
    ART + "/🚪io/🦀.rs": x, ART + "/🚪io/🟦.ts": x,
    ART + "/🔸other/odd.rs": x,
    ART + "/📚examples/🎬e/odd.rs": x, ART + "/📚examples/🎬e/odd.ts": x, ART + "/📚examples/🎬f/🦀.rs": x, ART + "/📚examples/🎬f/🟦.ts": x,
}, [b("component-file-" + ART + "/🧬schema", CF), b("component-file-ts-" + ART + "/🧬schema", CF),
    b("component-file-example-" + ART + "/📚examples/🎬e", CF), b("component-file-example-ts-" + ART + "/📚examples/🎬e", CF)])
props = {"crates": {"type": "array", "items": {"type": "object", "additionalProperties": False, "required": ["ownerRel", "pluginId"], "properties": {"ownerRel": {"type": "string"}, "pluginId": {"type": "string"}}}}}
rules = [S, C]
schema = std_schema("Semio Examples Law Corpus", rules, props)
refs = "(row.crates ?? []).map((c: any) => ({ dir: c.ownerRel, libRelPath: \"\", shape: \"taxonomy\", role: \"plugin\", ownerRel: c.ownerRel, pluginId: c.pluginId }))"
ts = std_ts(
    "📚 The semio-examples and component-file laws report exactly the breaches their language-agnostic corpus names over synthetic artifact trees, agree with an independent Python model and reject a malformed corpus.",
    'import { policyComponentFileBreaches, policySemioArtifactExamplesBreaches } from "../../🟦.ts";',
    "\n".join("  %s: (root, row) => %s(root, %s)," % (r, r, refs) for r in rules),
)
folder, plen = write_test("⚖laws/🧩plugin-consistency/🗺taxonomy-shape/📚examples", "📚examples", ts, {"version": 1, "cases": cases}, schema, oracle)
print(folder, plen, len(cases))
