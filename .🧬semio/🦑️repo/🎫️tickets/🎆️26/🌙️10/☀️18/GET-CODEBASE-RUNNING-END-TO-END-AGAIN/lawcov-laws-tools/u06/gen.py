import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
read = lambda n: open(os.path.join(HERE, "..", n), encoding="utf8").read()
oracle = read("pylib.py") + "\n" + read("ptree.py") + "\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
cases = []
M, L, H = "medium", "low", "high"


def b(i, kind, prio, line=0):
    return [i, kind, prio, line]


def case(id_, rule, files, **kw):
    row = {"id": id_, "rule": rule, "files": files}
    row.update(kw)
    cases.append(row)


def sub(plugin="🔭p", art="🔸a", std="🔖1", name="✳any"):
    return "✏s/🔌plugins/%s/🗿artifacts/%s/🏅standards/%s/🪆subsets/%s" % (plugin, art, std, name)


S = sub()
case("subset-directories-follow-the-subset-naming-law", "policyListTopLevelSubsetDirs", {
    S + "/x": [], sub(name="✳fem-2d") + "/x": [], sub(name="✳Bad") + "/x": [], sub(name="plain") + "/x": [], sub(std="v1") + "/x": [], sub(art="🔸z", name="✳any") + "/x": [],
    "✏s/🔌plugins/🔭q/🗿artifacts/🔸a/📚examples/x": [], "✏s/🔌plugins/🔭q/🗿artifacts/🔸a/🏅standards/🔖2/📚stray/x": [],
}, expectList=[S, sub(name="✳fem-2d"), sub(art="🔸z")])
FT = "subset-conformance/facet-totality"
facet_files = {S + "/🧬schema/x": [], S + "/📚examples/x": []}
facet_expect = [b("subset-facet-missing-%s/%s" % (S, c), FT, M) for c in ("🚪io", "👁viewer", "✏editor")]
case("subset-facets-must-all-exist", "policySubsetFacetTotalityBreaches", facet_files, expect=facet_expect)
EF = "subset-conformance/engine-forbidden"
A1 = "✏s/🔌plugins/🔭p/🗿artifacts/🔸a"
engine_files = {A1 + "/⚙engine/x": [], A1 + "/🏅standards/🔖1/⚙engine/x": [], S + "/⚙engine/x": [], "✏s/🔌plugins/🔭p/🗿artifacts/🔸b/🏅standards/🔖1/🪆subsets/✳any/🧬schema/x": []}
case("engines-do-not-live-at-owner-levels", "policyArtifactEngineFacetForbiddenBreaches", engine_files, expect=[b("artifact-engine-forbidden-" + A1, EF, L), b("artifact-engine-forbidden-" + A1 + "/🏅standards/🔖1", EF, L), b("artifact-engine-forbidden-" + S, EF, L)])
EP = "subset-conformance/example-placement"
case("examples-belong-to-subsets", "policyExampleNotAtArtifactLevelBreaches", {A1 + "/📚examples/x": [], A1 + "/🏅standards/🔖1/📚examples/x": [], S + "/📚examples/x": []},
     expect=[b("artifact-examples-remaining-" + A1 + "/📚examples", EP, M), b("standard-examples-remaining-" + A1 + "/🏅standards/🔖1/📚examples", EP, M)])
PH = "subset-conformance/phantom-standards"
case("phantom-standards-directories", "policyPhantomStandardsBreaches", {A1 + "/🏅标准/x": [], A1 + "/🏅标准/🔖1/标准x/y": [], A1 + "/🏅standards/x": []},
     expect=[b("phantom-standards-" + A1 + "/🏅标准", PH, M), b("phantom-standards-" + A1 + "/🏅标准/🔖1/标准x", PH, M)])
TP = "subset-conformance/ts-parity"
S2 = sub(plugin="🔭q")
case("subset-ts-meta-stub-length", "policySubsetTsParityBreaches", {S + "/🧬schema/🟦.ts": ["l%d" % i for i in range(6)], S2 + "/🧬schema/🟦.ts": ["l%d" % i for i in range(7)]}, expect=[b("subset-ts-meta-stub-" + S, TP, M)])
GS = "subset-conformance/no-glue-shim-blocks"
G1, G2, G3 = ("✏s/🔌plugins/🔭%s/📦packages/🦀rust/🦀.rs" % n for n in "pqr")
case("glue-shim-blocks", "policyNoGlueShimBlocksBreaches", {G1: ["// Shims: keep pre-migration paths"], G2: ["// PRE-MIGRATION MODULE PATHS"], G3: ["// clean glue"]}, expect=[b("glue-shim-" + G1, GS, M), b("glue-shim-" + G2, GS, M)])
case("aggregate-runs-every-conformance-rule", "policySubsetConformanceBreaches", {**facet_files, **engine_files, G1: ["// Shims: keep pre-migration paths"], A1 + "/🏅标准/x": [], S + "/🧬schema/🟦.ts": ["l0"]},
     expect=[b("subset-facet-missing-%s/%s" % (S, c), FT, M) for c in ("🚪io", "👁viewer", "✏editor")] + [b("subset-facet-missing-%s/%s" % (sub(art="🔸b"), c), FT, M) for c in ("🚪io", "👁viewer", "✏editor", "📚examples")] + [b("artifact-engine-forbidden-" + A1, EF, L), b("artifact-engine-forbidden-" + A1 + "/🏅standards/🔖1", EF, L), b("artifact-engine-forbidden-" + S, EF, L),
            b("phantom-standards-" + A1 + "/🏅标准", PH, M), b("subset-ts-meta-stub-" + S, TP, M), b("glue-shim-" + G1, GS, M)])
SC = "taxonomy/surface-completeness"
sa = sub(art="🔸a")
win = lambda s, role, w: s + "/" + role + "/🎭modes/🔸m/🪟windows/" + w
case("surfaces-need-a-complete-window", "policySubsetSurfaceCompletenessBreaches", {
    sa + "/🧬schema/x": [],
    win(sa, "👁viewer", "🔸w") + "/🦀.rs": ["fn a() {}"], win(sa, "👁viewer", "🔸w") + "/🟦.ts": ["// SCAFFOLD"],
    sub(art="🔸b") + "/🧬schema/x": [], win(sub(art="🔸b"), "👁viewer", "🔸w") + "/🦀.rs": ["fn a() {}"], sub(art="🔸b") + "/✏editor/x": [],
    sub(art="🔸c") + "/📚examples/x": [],
}, expect=[
    b("subset-surface-missing-%s/✏editor" % sa, SC, M), b("subset-surface-scaffold-residue-%s/👁viewer" % sa, "taxonomy/surface-scaffold-residue", M),
    b("subset-surface-window-incomplete-%s/👁viewer" % sub(art="🔸b"), SC, M), b("subset-surface-window-incomplete-%s/✏editor" % sub(art="🔸b"), SC, M),
])
VP = "taxonomy/viewer-purity"
case("viewer-surfaces-do-not-mutate", "policyViewerPurityBreaches", {
    S + "/👁viewer/a.rs": ["doc.mutation(x);", "Emit::mutations(y);"], S + "/👁viewer/b/c.txt": ["see artifact_mutations and ::editor::run"], S + "/✏editor/d.rs": ["doc.mutation(x);"], S + "/👁viewer/clean.rs": ["fn ok() {}"],
}, expect=[b("viewer-purity-%s/👁viewer/a.rs-.mutation(" % S, VP, H), b("viewer-purity-%s/👁viewer/a.rs-Emit::mutations" % S, VP, H), b("viewer-purity-%s/👁viewer/b/c.txt-artifact_mutations" % S, VP, H), b("viewer-purity-%s/👁viewer/b/c.txt-::editor::" % S, VP, H)])
CS = "plugin-dependency/contributed-surface-target"
sA, sB, sC, sD = sub(plugin="🔭alpha"), sub(plugin="🔭beta"), sub(plugin="🔭gamma"), sub(plugin="🔭delta", art="🔸z")
case("contributed-surfaces-must-declare-their-owner", "policyContributedSurfaceTargetBreaches", {
    sA + "/🧬schema/x": [], sB + "/👁viewer/x": [], sC + "/✏editor/x": [], "✏s/🔌plugins/🔭gamma/🦀.rs": ['.depends_on("alpha")'], sD + "/👁viewer/x": [],
}, expect=[b("contributed-surface-target-undeclared-%s/👁viewer" % sB, CS, H), b("contributed-surface-unknown-owner-%s/👁viewer" % sD, CS, M)])
case("os-config-requires-its-schema", "policyOsConfigShapeBreaches", {"README.md": ["x"]}, expect=[b("os-config-shape-missing-schema", "taxonomy/os-config-shape", H)])

rules = ["policyListTopLevelSubsetDirs", "policySubsetFacetTotalityBreaches", "policyArtifactEngineFacetForbiddenBreaches", "policyExampleNotAtArtifactLevelBreaches", "policyPhantomStandardsBreaches", "policySubsetTsParityBreaches", "policyNoGlueShimBlocksBreaches", "policySubsetConformanceBreaches", "policySubsetSurfaceCompletenessBreaches", "policyViewerPurityBreaches", "policyContributedSurfaceTargetBreaches", "policyOsConfigShapeBreaches"]
schema = std_schema("Subset Conformance Law Corpus", rules)
ts = std_ts(
    "🪆 The subset-conformance laws list subsets and report exactly the breaches their language-agnostic corpus names over synthetic plugin trees, agree with an independent Python model and reject a malformed corpus.",
    "import { policyArtifactEngineFacetForbiddenBreaches, policyContributedSurfaceTargetBreaches, policyExampleNotAtArtifactLevelBreaches, policyListTopLevelSubsetDirs, policyNoGlueShimBlocksBreaches, policyOsConfigShapeBreaches, policyPhantomStandardsBreaches, policySubsetConformanceBreaches, policySubsetFacetTotalityBreaches, policySubsetSurfaceCompletenessBreaches, policySubsetTsParityBreaches, policyViewerPurityBreaches } from \"../../🟦.ts\";",
    "\n".join("  %s: (root) => %s(root)," % (r, r) for r in rules),
)
folder, plen = write_test("⚖laws/🧩plugin-consistency/🪆subset-conformance", "🪆conformance", ts, {"version": 1, "cases": cases}, schema, oracle)
print(folder, plen, len(cases))
