import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
read = lambda n: open(os.path.join(HERE, "..", n), encoding="utf8").read()
oracle = read("pylib.py") + "\n" + read("foundation.py") + "\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
cases = []
x = []
MM = "mutation-migration/"


def case(id_, rule, files, expect):
    cases.append({"id": id_, "rule": rule, "files": files, "expect": expect, "git": True})


def b(i, kind, line=0):
    return [i, MM + kind, "high", line]


case("crdt-vocabulary-is-forbidden", "policyNoCrdtVocabularyBreaches", {
    "a/x.rs": ["fn ok() {}", "let s = merge_strategy(1); let t = ConflictRule::A;", "use MergeStrategyKind;", "fn assert_crdt_x() {}", "// ResolutionPlanning mentions"], "a/y.ts": ["merge_strategy"], "a/z.rs": ["notmerge_strategy", "merge_concurrent_diffs"],
}, [b("no-crdt-vocabulary-a/x.rs-2", "no-crdt-vocabulary", 2), b("no-crdt-vocabulary-a/x.rs-3", "no-crdt-vocabulary", 3), b("no-crdt-vocabulary-a/x.rs-4", "no-crdt-vocabulary", 4), b("no-crdt-vocabulary-a/x.rs-5", "no-crdt-vocabulary", 5), b("no-crdt-vocabulary-a/z.rs-2", "no-crdt-vocabulary", 2)])
case("leaf-mutations-do-not-override-validate", "policyNoValidateOverrideBreaches", {
    "a/🧬mutations/🔹m/🦀.rs": ["impl X {", "    fn validate(&self) -> bool { true }", "}", "fn validate(&self, x: u8) {}", "fn validate(&mut self) {}"],
    "a/b/🦀.rs": ["fn validate(&self) {}", "impl CompositeMutationKind<D> for C {", "    fn other(&self) {}", "}"],
}, [b("no-validate-override-a/🧬mutations/🔹m/🦀.rs-2", "no-validate-override", 2), b("no-validate-override-a/🧬mutations/🔹m/🦀.rs-4", "no-validate-override", 4)])
cases.append({"id": "validate-overrides-inside-mutation-kind-impls-are-located-at-the-override", "rule": "policyNoValidateOverrideBreaches", "git": True, "files": {"a/b/🦀.rs": ["fn first() {}", "impl<T> MutationKind<D> for K {", "    fn validate(&self) {}", "}"]},
              "expect": [b("no-validate-override-a/b/🦀.rs-3", "no-validate-override", 3)],
              "defect": "🟦.ts:94 adds the override offset, which is relative to the impl body, to the offset of the impl header, so the breach points at the header line instead of the validate line"})
case("severity-hint-is-retired", "policySeverityInfoBreaches", {
    "a/x.rs": ["let s = Severity::Hint;", "let t = Severity.Hint;", 'let u = "hint"; // plain', 'let v = ("hint", Severity::Warning);', 'const level = "hint";'], "a/y.ts": ['const s = { severity: \'hint\' };', 'const hint = 1;'], "a/z.py": ["Severity.Hint"],
}, [b("severity-hint-a/x.rs-1", "no-severity-hint", 1), b("severity-hint-a/x.rs-2", "no-severity-hint", 2), b("severity-hint-literal-a/x.rs-4", "no-severity-hint", 4), b("severity-hint-literal-a/x.rs-5", "no-severity-hint", 5), b("severity-hint-literal-a/y.ts-1", "no-severity-hint", 1)])
full = ["LaissezFaire", "Normal", "Vigilant"]
case("merge-policy-spellings-match-on-every-surface", "policyMergePolicyParityBreaches", {
    "🧰framework/🔨modules/📡replication/🧾wire/🦀.rs": full, "🧰framework/🔨modules/🎠kernel/🔄merge-outcome/🟦.ts": ["laissezFaire", "normal", "vigilant"], "🧰framework/🔨modules/🖱ui/🧱elements/📚I18n/🟦.tsx": ["LaissezFaire", "Normal"],
}, [b("merge-policy-parity-🧰framework/🔨modules/🖱ui/🧱elements/📚I18n/🟦.tsx", "merge-policy-parity"), b("merge-policy-parity-🧰framework/🔨modules/🖱ui/🎯targets/⚛react/🌐i18n/🤝collaboration/🟦.ts", "merge-policy-parity")])
G = "🧰framework/🛍products/💻os/🔨modules/🗣dsl/✨derive/📦packages/🦀rust/🦀.rs"
O = "🧰framework/🛍products/💻os/🔨modules/🗣dsl/✨derive/🦀.rs"
mount = '#[path = "../../🦀.rs"] mod component;'
case("derive-glue-only-mounts-its-owner", "policyDeriveGlueMountBreaches", {O: ["pub fn expand_x() {}"], G: [mount]}, [])
case("derive-glue-must-not-expand", "policyDeriveGlueMountBreaches", {O: ["x"], G: [mount, "fn expand_helper() {}"]}, [b("derive-glue-mount-" + G, "derive-glue-mount")])
case("derive-glue-without-its-owner", "policyDeriveGlueMountBreaches", {G: [mount]}, [b("derive-glue-mount-" + G, "derive-glue-mount")])
rules = ["policyNoCrdtVocabularyBreaches", "policyNoValidateOverrideBreaches", "policySeverityInfoBreaches", "policyMergePolicyParityBreaches", "policyDeriveGlueMountBreaches"]
schema = std_schema("Merge Policy Law Corpus", rules)
ts = std_ts(
    "📐 The mutation merge-policy laws (no CRDT vocabulary, no validate override, no severity hint, merge-policy parity, derive glue) report exactly the breaches their language-agnostic corpus names over git-visible synthetic trees, agree with an independent Python model and reject a malformed corpus.",
    "import { policyDeriveGlueMountBreaches, policyMergePolicyParityBreaches, policyNoCrdtVocabularyBreaches, policyNoValidateOverrideBreaches, policySeverityInfoBreaches } from \"../../🟦.ts\";",
    "\n".join("  %s: (root) => %s(root)," % (r, r) for r in rules),
)
folder, plen = write_test("⚖laws/🧩plugin-consistency/🎯mutation-outcome/📐merge-policy-law", "📐merge-policy", ts, {"version": 1, "cases": cases}, schema, oracle)
print(folder, plen, len(cases))
