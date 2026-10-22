import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
read = lambda n: open(os.path.join(HERE, "..", n), encoding="utf8").read()
oracle = read("pylib.py") + "\n" + read("ptree.py") + "\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
cases = []
x = []
PLG = "✏s/🔌plugins/🔭p/🗿artifacts/"
DF = "stdio-artifacts/derived-facets"


def case(id_, rule, files, expect, args=None):
    row = {"id": id_, "rule": rule, "files": files, "expect": expect}
    if args is not None:
        row["args"] = args
    cases.append(row)


def b(i, kind):
    return [i, kind, "high", 0]


case("artifact-with-a-standards-directory-is-migrated", "policyArtifactIsMigrated", {PLG + "🔸a/🏅standards/x": x, PLG + "🔸b/⚙engine/x": x}, True, [PLG + "🔸a"])
case("artifact-without-a-standards-directory-is-not-migrated", "policyArtifactIsMigrated", {PLG + "🔸a/🏅standards/x": x, PLG + "🔸b/⚙engine/x": x}, False, [PLG + "🔸b"])
case("builders-are-derived-for-migrated-artifacts-only", "policyArtifactBuilderBreaches", {PLG + "🔸a/🏗builder/x": x, PLG + "🔸b/🏗builder/x": x, PLG + "🔸b/🏅standards/x": x, PLG + "🔸c/⚙engine/x": x}, [b("artifact-derived-builder-" + PLG + "🔸a", DF)])
DC = "stdio-artifacts/decomposer"
D = lambda a: PLG + a + "/🪓decomposer"
case("decomposers-need-a-trait-impl-and-a-ts-twin", "policyArtifactDecomposerBreaches", {
    PLG + "🔸a1/⚙engine/x": x, D("🔸a2") + "/x": x, D("🔸a3") + "/🦀.rs": ["pub struct D;"], D("🔸a3") + "/🟦.ts": x,
    D("🔸a4") + "/🦀.rs": ["impl ArtifactDecomposer for D {}"], D("🔸a4") + "/🟦.ts": x, D("🔸a5") + "/🦀.rs": ["// impl ArtifactDecomposer for D {}"], D("🔸a5") + "/🟦.ts": x,
    PLG + "🔸a6/🏅standards/x": x,
}, [b("%s-missing-%s" % (DC, D("🔸a1")), DC), b("%s-rs-%s" % (DC, D("🔸a2")), DC), b("%s-ts-%s" % (DC, D("🔸a2")), DC), b("%s-trait-%s" % (DC, D("🔸a3")), DC), b("%s-trait-%s" % (DC, D("🔸a5")), DC)])
SUB = lambda a, std="🔖1", s="✳any": "%s%s/🏅standards/%s/🪆subsets/%s" % (PLG, a, std, s)
derived_files = {
    SUB("🔸ok") + "/🧬schema/🦀.rs": ["derive_artifact_facets! {", "    construction: a,", "    analysis: b,", "}"],
    SUB("🔸bare") + "/x": x,
    PLG + "🔸explicit/🏗builder/x": x, SUB("🔸explicit") + "/🧬schema/🦀.rs": ["derive_artifact_facets! { construction: a, analysis: b, composition: c }"], SUB("🔸explicit") + "/🎹composer/x": x,
}
case("builders-derive-from-the-schema-macro", "policyArtifactBuilderMigratedBreaches", derived_files, [b("artifact-derived-builder-missing-" + SUB("🔸bare"), DF), b("artifact-derived-builder-explicit-" + PLG + "🔸explicit", DF)])
case("analyzers-derive-from-the-schema-macro", "policyArtifactAnalyzerBreaches", derived_files, [b("artifact-derived-analyzer-missing-" + SUB("🔸bare"), DF)])
case("composers-derive-from-the-schema-macro", "policyArtifactComposerBreaches", derived_files, [b("artifact-derived-composer-missing-" + SUB("🔸ok"), DF), b("artifact-derived-composer-missing-" + SUB("🔸bare"), DF), b("artifact-derived-composer-explicit-" + SUB("🔸explicit"), DF)])
case("the-catalog-needs-an-owner-table", "policyStdioCatalogBreaches", {"README.md": ["x"]}, [b("stdio-catalog-owner-table-missing", "stdio-artifacts/catalog")])
for c in cases:
    if isinstance(c["expect"], list):
        c["expect"] = jsort(c["expect"])
rules = ["policyArtifactIsMigrated", "policyArtifactBuilderBreaches", "policyArtifactDecomposerBreaches", "policyArtifactBuilderMigratedBreaches", "policyArtifactAnalyzerBreaches", "policyArtifactComposerBreaches", "policyStdioCatalogBreaches"]
schema = val_schema("Stdio Catalog Law Corpus", rules)
ts = val_ts(
    "🗂 The stdio-catalog laws (migration test, derived facets, decomposer shape, missing owner table) report exactly what their language-agnostic corpus names over synthetic plugin trees, agree with an independent Python model and reject a malformed corpus.",
    "import { policyArtifactAnalyzerBreaches, policyArtifactBuilderBreaches, policyArtifactBuilderMigratedBreaches, policyArtifactComposerBreaches, policyArtifactDecomposerBreaches, policyArtifactIsMigrated, policyStdioCatalogBreaches } from \"../../🟦.ts\";",
    "\n".join(["  policyArtifactIsMigrated: (root, art: string) => policyArtifactIsMigrated(root, art),"] + ["  %s: (root) => brs(%s(root))," % (r, r) for r in rules[1:]]),
)
ts = ts.replace("const rules:", SORT_TS + "\nconst rules:", 1)
folder, plen = write_test("⚖laws/🧩plugin-consistency/🚪artifact-io/🗂stdio-catalog", "🗂catalog", ts, {"version": 1, "cases": cases}, schema, oracle)
print(folder, plen, len(cases))
