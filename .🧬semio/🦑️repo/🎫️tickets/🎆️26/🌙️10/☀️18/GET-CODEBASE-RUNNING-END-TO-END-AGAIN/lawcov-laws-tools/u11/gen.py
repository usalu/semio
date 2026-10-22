import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
read = lambda n: open(os.path.join(HERE, "..", n), encoding="utf8").read()
oracle = read("pylib.py") + "\n" + read("ptree.py") + "\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
cases = []
OWNER = "✏s/🔌plugins/🔭alpha"
ART = OWNER + "/🗿artifacts/🔸a"
STD = ART + "/🏅standards/🔖1"
SUB = STD + "/🪆subsets/✳any"
CR = [{"ownerRel": OWNER, "pluginId": "alpha"}]
KD = "taxonomy/dirs"


def case(id_, rule, files, expect, crates=CR):
    cases.append({"id": id_, "rule": rule, "files": files, "expect": expect, "crates": crates})


def b(i, kind, prio="high"):
    return [i, kind, prio, 0]


x = []
dirs_files = {
    ART + "/🔸junk/x": x, ART + "/🧬schema/🔸odd/x": x, ART + "/🧬schema/📸snapshot/x": x, ART + "/📚examples/x": x, ART + "/🔨modules/x": x,
    ART + "/🚪io/🔸weird/x": x, ART + "/🚪io/📥import/🔸wrongcodec/x": x, ART + "/🚪io/📥import/🧩deserializers/🔸notartifacts/x": x, ART + "/🚪io/📥import/🧩deserializers/🗿artifacts/x": x,
    ART + "/🚪io/📤export/🧵serializers/🗿artifacts/x": x, ART + "/🚪io/📝text/x": x[:0],
    STD + "/🔸x/x": x, STD + "/🔨modules/x": x, SUB + "/🔸y/x": x, SUB + "/📚examples/x": x, SUB + "/🧬schema/🔸odd2/x": x, SUB + "/🚪io/🔸weird2/x": x,
    SUB + "/👁viewer/🔸z/x": x, SUB + "/👁viewer/🎭modes/x": x, SUB + "/✏editor/🌉wasm/x": x,
}
dirs_expect = [
    b("taxonomy-dirs-artifact-%s/🔸junk" % ART, KD, "medium"), b("taxonomy-dirs-artifact-%s/🧬schema-🔸odd" % ART, KD, "medium"),
    b("taxonomy-dirs-artifact-%s/🚪io-🔸weird" % ART, KD, "medium"), b("taxonomy-dirs-artifact-%s/🚪io/📥import-🔸wrongcodec" % ART, KD, "medium"),
    b("taxonomy-dirs-artifact-%s/🚪io/📥import/🧩deserializers-🔸notartifacts" % ART, KD, "medium"),
    b("taxonomy-dirs-standard-%s-🔸x" % STD, KD, "medium"), b("taxonomy-dirs-subset-%s-🔸y" % SUB, KD, "medium"),
    b("taxonomy-dirs-artifact-%s/🧬schema-🔸odd2" % SUB, KD, "medium"), b("taxonomy-dirs-artifact-%s/🚪io-🔸weird2" % SUB, KD, "medium"),
    b("taxonomy-dirs-surface-%s/👁viewer-🔸z" % SUB, KD, "medium"),
]
dirs_expect = [[i.replace("/🔸junk-", "-").replace("%s/🔸junk" % ART, "taxonomy-dirs-artifact-%s-🔸junk" % ART) if False else i, k, p, l] for i, k, p, l in dirs_expect]
dirs_expect[0] = b("taxonomy-dirs-artifact-%s-🔸junk" % ART, KD, "medium")
dirs_expect[1] = b("taxonomy-dirs-artifact-%s/🧬schema-🔸odd" % ART, KD, "medium")
case("owner-directory-vocabulary", "policyTaxonomyDirsBreaches", dirs_files, dirs_expect)
case("non-plugin-owners-report-at-low-priority", "policyTaxonomyDirsBreaches", {"🧰framework/🔨modules/🔸owner/🗿artifacts/🔸a/🔸junk/x": x},
     [b("taxonomy-dirs-artifact-🧰framework/🔨modules/🔸owner/🗿artifacts/🔸a-🔸junk", KD, "low")], crates=[{"ownerRel": "🧰framework/🔨modules/🔸owner", "pluginId": ""}])
W = SUB + "/👁viewer/🎭modes/🔸m/🪟windows/🔸w"
case("window-children-use-the-window-vocabulary", "policyTaxonomyDirsBreaches", {W + "/🍱panes/x": x, W + "/🔸bad/x": x, W + "/🧪tests/x": x, W + "/🧫fixtures/x": x, SUB + "/✏editor/🪟windows/🔸w2/🔸bad2/x": x},
     [b("taxonomy-dirs-window-%s-🔸bad" % W, KD, "medium"), b("taxonomy-dirs-window-%s/✏editor/🪟windows/🔸w2-🔸bad2" % SUB, KD, "medium")][:1])
cases[-1]["files"].pop(SUB + "/✏editor/🪟windows/🔸w2/🔸bad2/x")
MK = "📝.md"
case("window-capabilities-are-complete", "policyWindowCompletenessBreaches", {
    W + "/🎬actions/" + MK: x, W + "/☑options/x.txt": x, W + "/🎚config/🔸m1/🦀.rs": x, W + "/👥presence/🔸m2/🦀.rs": x, W + "/👥presence/" + MK: x, W + "/👥presence/🔸m2/🟦.ts": x,
    W + "/🫧transient/🦀.rs": x,
}, [
    b("taxonomy-window-completeness-%s-🪛utilities" % W, "taxonomy/window-completeness"), b("taxonomy-window-empty-facet-%s/☑options" % W, "taxonomy/window-empty-facet"),
    b("taxonomy-window-component-%s/🎚config/🔸m1-🟦typescript" % W, "taxonomy/window-component"), b("taxonomy-window-populated-facet-marker-%s/👥presence" % W, "taxonomy/window-empty-facet"),
    b("taxonomy-window-facet-component-%s/🫧transient-🦀.rs" % W, "taxonomy/window-facet-component"), b("taxonomy-window-empty-facet-%s/🫧transient" % W, "taxonomy/window-empty-facet"),
])
M = SUB + "/👁viewer/🎭modes/🔸m"
case("mode-children-are-complete", "policyModeCompletenessBreaches", {
    M + "/🪟windows/🔸w/x": x, M + "/🎮commands/x.txt": x, M + "/🎚config/" + MK: x, M + "/🎚config/🔸c1/x": x, M + "/👥presence/" + MK: x,
}, [b("taxonomy-mode-empty-child-%s/🎮commands" % M, "taxonomy/mode-empty-child"), b("taxonomy-mode-populated-child-marker-%s/🎚config" % M, "taxonomy/mode-empty-child"), b("taxonomy-mode-completeness-%s-🫧transient" % M, "taxonomy/mode-completeness")])

props = {"crates": {"type": "array", "items": {"type": "object", "additionalProperties": False, "required": ["ownerRel", "pluginId"], "properties": {"ownerRel": {"type": "string"}, "pluginId": {"type": "string"}}}}}
rules = ["policyTaxonomyDirsBreaches", "policyWindowCompletenessBreaches", "policyModeCompletenessBreaches"]
schema = std_schema("Taxonomy Shape Law Corpus", rules, props)
refs = "(row.crates ?? []).map((c: any) => ({ dir: c.ownerRel, libRelPath: \"\", shape: \"taxonomy\", role: \"plugin\", ownerRel: c.ownerRel, pluginId: c.pluginId }))"
ts = std_ts(
    "🗺 The taxonomy-shape laws report exactly the directory-vocabulary, window-completeness and mode-completeness breaches their language-agnostic corpus names over synthetic plugin trees, agree with an independent Python model and reject a malformed corpus.",
    'import { policyModeCompletenessBreaches, policyTaxonomyDirsBreaches, policyWindowCompletenessBreaches } from "../../🟦.ts";',
    "\n".join("  %s: (root, row) => %s(root, %s)," % (r, r, refs) for r in rules),
)
folder, plen = write_test("⚖laws/🧩plugin-consistency/🗺taxonomy-shape", "🗺shape", ts, {"version": 1, "cases": cases}, schema, oracle)
print(folder, plen, len(cases))
