import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
read = lambda n: open(os.path.join(HERE, "..", n), encoding="utf8").read()
oracle = read("pylib.py") + "\n" + read("foundation.py") + "\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
cases = []
x = []


def case(id_, rule, files, expect):
    cases.append({"id": id_, "rule": rule, "files": files, "expect": expect})


def b(i, kind, prio, line=0):
    return [i, kind, prio, line]


DK = "dissolved-kernels/"
D = "policyDissolvedKernelsBreaches"
STORE = "🧰framework/🔨modules/◻2d/🗄store/🦀.rs"
dk_files = {
    "r/one.rs": ["pub struct S {", "    pub mesh: HalfedgeMesh,", "}"], "r/two.rs": ["static CACHE: Mutex<EngineCache> = X;"], "r/three.rs": ["fn f(mesh: HalfedgeMesh) {", "    let local: HalfedgeMesh = x;", "}"],
    "r/four.rs": ["fn g(", "    mesh: &HalfedgeMesh,", ") {}"], STORE: ["pub struct S { pub store: DrawingStore }", "", "struct T {", "    store: DrawingStore,", "}"],
    "🧰framework/🔨modules/🧊3d/📐brep/⚙engine/🖥host/🦀.rs": ["fn nothing() {}"],
    "e/cache.rs": ["let c = EngineCache::new();"], "🧰framework/🔨modules/◻2d/🧮compute/c.rs": ["let c = EngineCache::new();"], "🧰framework/🛍products/💻os/🔨modules/🔌plugin/p.rs": ["let c = EngineCache::new();"],
    "🧰framework/🔨modules/◻2d/🧮compute/../x.rs": ["let c = EngineCache::new();"],
    "m/🧬mutations/🟤set-snapshot/🦀.rs": x, "m/🧬mutations/🟤set-snapshot/sub/b.rs": x, "m/other/🟤set-snapshot/🦀.rs": x,
}
dk_files.pop("🧰framework/🔨modules/◻2d/🧮compute/../x.rs")
case("dissolved-representations-caches-and-whole-document-replaces", D, dk_files, [
    b("dissolved-rep-escape-r/one.rs", DK + "rep-escape", "medium"), b("dissolved-rep-escape-r/two.rs", DK + "rep-escape", "medium"), b("dissolved-rep-escape-r/four.rs", DK + "rep-escape", "medium"),
    b("dissolved-rep-escape-stale-🧰framework/🔨modules/🧊3d/📐brep/⚙engine/🖥host/🦀.rs", DK + "rep-escape", "low"),
    b("dissolved-engine-cache-scope-e/cache.rs", DK + "engine-cache-scope", "medium"),
    b("dissolved-whole-document-replace-m/🧬mutations/🟤set-snapshot", DK + "whole-document-replace", "medium"),
])
CK = "composition/"
kind_lines = [
    'ArtifactKindSpec { id: "s.alpha.beta" }', 'ArtifactKindSpec { id: "S.Alpha.beta" }', 'ArtifactKindSpec { id: "s.a.b.c" }', 'ArtifactKindSpec { id: "s.-a.b" }',
    '// ArtifactKindSpec { id: "bad" }', 'let url = "http://x"; ArtifactKindSpec { id: "bad2" }', 'ArtifactKindSpec { id: "s.a--b.c" }', 'ArtifactKindSpec{id:"s.ok-1.ok-2"}', "pub struct ArtifactKindSpec {",
]
case("artifact-kinds-are-s-dot-plugin-dot-artifact", "policyCanonicalArtifactKindBreaches", {"k/specs.rs": kind_lines, "k/plain.rs": ['id: "bad"']},
     [b("canonical-artifact-kind-k/specs.rs-%d" % n, CK + "canonical-artifact-kind", "medium", n) for n in (2, 3, 4, 7)])
SCH = "✏s/🔌plugins/🔭p/🗿artifacts/🔸a/🧬schema/"
ts_lines = [" * @child kind=s.a.b", " * @child kind=bad", " * @child kind=S.a.b */"]
js_lines = ['{"x-semio-child": "s.x.y"}', '{"x-semio-child": "S.x"}']
gq_lines = ['# @child(kind: "bad")', 'type A @child(kind: "s.a.b")', 'type B @child(kind: "bad")']
case("child-kinds-in-schema-sources", "policyCanonicalChildKindBreaches", {
    SCH + "a.ts": ts_lines, SCH + "b.json": js_lines, SCH + "c.graphql": gq_lines, SCH + "d.proto": ["// @child kind=Bad"], SCH + "🧪tests/e.ts": ["@child kind=bad"], "✏s/🔌plugins/🔭p/other/f.ts": ["@child kind=bad"],
}, [b("canonical-child-kind-%sa.ts-2" % SCH, CK + "canonical-child-kind", "high", 2), b("canonical-child-kind-%sa.ts-3" % SCH, CK + "canonical-child-kind", "high", 3), b("canonical-child-kind-%sb.json-2" % SCH, CK + "canonical-child-kind", "high", 2),
    b("canonical-child-kind-%sc.graphql-3" % SCH, CK + "canonical-child-kind", "high", 3), b("canonical-child-kind-%sd.proto-1" % SCH, CK + "canonical-child-kind", "high", 1)])
RP = "✏s/🔌plugins/🔭p/🗿artifacts/🔸a/x.rs"
redef_lines = ["pub struct SemioTextSnapshot {}", "// struct SemioAudioSnapshot {}", "use stdio::SemioImageSnapshot;", "    pub(crate) enum SemioFlowSnapshot { A }", "struct SemioTableSnapshotExtra;"]
case("neutral-snapshot-types-are-not-redefined-by-plugins", "policyDissolvedKindRedefinitionBreaches", {RP: redef_lines, "✏s/🔌plugins/🗄stdio/y.rs": ["pub struct SemioTextSnapshot {}"], "other/z.rs": ["pub struct SemioTextSnapshot {}"]},
     [b("dissolved-kind-redefinition-%s-SemioTextSnapshot" % RP, CK + "dissolved-kind-redefinition", "high", 1), b("dissolved-kind-redefinition-%s-SemioFlowSnapshot" % RP, CK + "dissolved-kind-redefinition", "high", 4)])
case("composition-runs-the-three-kind-rules", "policyCompositionBreaches", {"k/specs.rs": ['ArtifactKindSpec { id: "bad" }'], RP: ["pub struct SemioTextSnapshot {}"], SCH + "a.ts": ["@child kind=bad"]},
     [b("canonical-artifact-kind-k/specs.rs-1", CK + "canonical-artifact-kind", "medium", 1), b("canonical-child-kind-%sa.ts-1" % SCH, CK + "canonical-child-kind", "high", 1), b("dissolved-kind-redefinition-%s-SemioTextSnapshot" % RP, CK + "dissolved-kind-redefinition", "high", 1)])

rules = ["policyDissolvedKernelsBreaches", "policyCanonicalArtifactKindBreaches", "policyCanonicalChildKindBreaches", "policyDissolvedKindRedefinitionBreaches", "policyCompositionBreaches"]
schema = std_schema("Dissolved Kernels Law Corpus", rules)
ts = std_ts(
    "🧊 The dissolved-kernel and composition laws report exactly the representation-escape, cache-scope, kind-grammar and redefinition breaches their language-agnostic corpus names, agree with an independent Python model and reject a malformed corpus.",
    "import { policyCanonicalArtifactKindBreaches, policyCanonicalChildKindBreaches, policyCompositionBreaches, policyDissolvedKernelsBreaches, policyDissolvedKindRedefinitionBreaches } from \"../../🟦.ts\";",
    "\n".join("  %s: (root) => %s(root)," % (r, r) for r in rules),
)
folder, plen = write_test("⚖laws/🧩plugin-consistency/🧊dissolved-kernels", "🧊kernels", ts, {"version": 1, "cases": cases}, schema, oracle)
print(folder, plen, len(cases))
