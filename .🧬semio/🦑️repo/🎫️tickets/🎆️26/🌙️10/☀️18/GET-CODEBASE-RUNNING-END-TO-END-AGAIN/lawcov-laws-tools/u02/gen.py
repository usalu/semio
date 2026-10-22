import sys, os, re
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
oracle = open(os.path.join(HERE, "..", "pylib.py"), encoding="utf8").read() + "\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
SLUG = {"R8": "leaf-mutable-base", "R9": "leaf-applies-diff", "R10": "leaf-between", "R11": "diff-derived-inverse", "R12": "base-clone-diff", "R13": "whole-state-diff", "R14": "restore-inverse", "R15": "inverse-sum-law-untested", "R16": "outcome-apply-to", "R17": "host-apply", "R18": "row-emitter"}
cases = []


def marked(path, lines):
    out = []
    for i, text in enumerate(lines, 1):
        m = re.search(r"//\s*((?:@R\d+\s*)+)$", text)
        for rule in (re.findall(r"@(R\d+)", m.group(1)) if m else []):
            out.append(["diff-only-%s-%s-%d" % (rule.lower(), path, i), "diff-only-mutation/" + SLUG[rule], "high", i])
    return out


def file_case(id_, rule, path, lines, oracle_=True, group=None, expect=None):
    row = {"id": id_, "rule": rule, "path": path, "source": lines, "expect": marked(path, lines) if expect is None else expect}
    if group:
        row["group"] = group
    if not oracle_:
        row["oracle"] = False
    cases.append(row)


M = "🔸area/🧬mutations/🔹leaf/🦀.rs"
FILE = "policyDiffOnlyFileBreaches"
file_case("every-rule-under-the-mutations-facet", FILE, M, [
    "use crate::apply;",
    "impl MutationKind<Doc> for Add {",
    "    type Diff = Doc; // @R13",
    "    fn diff(&self, base: &Doc, scratch: &mut Vec<u8>) -> Out { // @R8",
    "        let mut copy = base.clone(); // @R12",
    "        let next = state_after(base); // @R10",
    "        apply_diff(&d); // @R9",
    "        out.apply(1); // @R9",
    "        Out::new()",
    "    }",
    "    fn inverse(&self, base: &Doc) -> Out {",
    "        let n = negative(x); // @R11",
    "        let m = d.negate(); // @R11",
    "        let k = diff(a); // @R11",
    "        let j = flip_inverse(y);",
    "        let q = flip_inverse(outcome); // @R11",
    "        Out::RestoreAll // @R14",
    "    }",
    "}",
])
file_case("whole-snapshot-diff-impl-and-diff-machinery", FILE, "🔸area/🧬mutations/🔹leaf/🔺diff/🦀.rs", [
    "impl MutationDiff<Doc> for Doc { // @R13",
    "    fn apply(&self, base: &Doc) -> Doc { base.clone() }",
    "    fn absorb(&mut self, other: &Doc) {}",
    "    fn helper_between() {}",
    "}",
])
file_case("plugin-artifact-leaf-scans-logic-impls-and-diffish-names", FILE, "✏s/🔌plugins/🔭p/🗿artifacts/🔸k/⚙engine/🦀.rs", [
    "impl MutationKind<Doc> for Move {",
    "    fn inverse(&self, base: &Doc) -> Out {",
    "        let x = state_after(base); // @R10 @R11",
    "        Out::empty()",
    "    }",
    "    fn unrelated(&self) { thing.apply(2); } // @R9",
    "}",
    "fn helper_diff(a: &mut Doc) {} // @R8",
    "fn plain(a: &mut Doc) { a.apply(1); }",
])
file_case("outcome-apply-to-is-gone", FILE, "🔸area/🧱host/🦀.rs", [
    "fn run(o: Outcome) { let n = o.apply_to(&base); } // @R16",
    "impl MutationOutcome<D> {",
    "    fn apply_to(&self) {} // @R16",
    "}",
])
file_case("non-leaf-source-without-signals-is-clean", FILE, "🔸area/🧱host/🦀.rs", ["fn run() { let x = 1; x.apply(2); }"])
file_case("masking-hides-strings-comments-and-test-items", FILE, M, [
    "fn build() {",
    '    let a = "state_after(x)";',
    "    // state_after(y)",
    '    let r = r#"out.apply(1)"#;',
    "    let c = '\"';",
    "    let z = state_after(q); // @R10",
    "}",
    "#[cfg(test)]",
    "mod tests {",
    "    fn t() { out.apply(1); state_after(1); }",
    "}",
])
EM = "policyRowEmitterFileBreaches"
file_case("row-emitter-banned-calls", EM, M, [
    "fn build() {",
    "    let m = commit_built(rows); // @R18",
    "    let n = mesh.bevel_job(args); // @R18",
    '    let s = "equals_base";',
    "    // sparse_rows(x)",
    "    let k = Row::produced(a); // @R18",
    "}",
    "#[cfg(test)]",
    "mod tests {",
    "    fn t() { commit_built(rows); }",
    "}",
])
file_case("row-emitter-journal-directory", EM, "🔸area/🥽mesh/📝journal/🦀.rs", ["fn go() { commit_built(x); } // @R18"])
file_case("row-emitter-ignores-tests-and-other-directories", EM, "🔸area/🧬mutations/🧪tests/🦀.rs", ["fn go() { commit_built(x); }"])
file_case("row-emitter-other-directory-is-ignored", EM, "🔸area/🧱host/🦀.rs", ["fn go() { commit_built(x); }"])
HA = "policyHostApplyFileBreaches"
file_case("host-apply-snapshot-escape-and-before-after-differencing", HA, "🔸area/✏editor/🦀.rs", [
    "fn go(s: &Doc) {",
    "    let c = snapshot_mut(s); // @R17",
    "    let d = make_diff(&before, &after); // @R17",
    "}",
    "fn compute_diff(before: &Doc, after: &Doc) -> D { D } // @R17",
])
file_case("host-apply-framework-skips-before-after-calls", HA, "🧰framework/🛍products/💻os/🔨modules/🔸area/🦀.rs", [
    "fn go() { let d = make_diff(&before, &after); }",
    "fn c(a: &Doc) { snapshot_mut(a); } // @R17",
])
file_case("host-apply-scope-skips-excluded-directories", HA, "🔸area/✏editor/🧪tests/🦀.rs", ["fn go(s: &Doc) { snapshot_mut(s); }"])
file_case("host-apply-typescript-ref-assignment-from-local-apply", HA, "🔸app/shell.tsx", [
    "const next = applyLocal(doc);",
    "documentSnapshotRef.current = next; // @R17",
    "documentSnapshotRef.current = fromServer;",
])
file_case("host-apply-mutable-parameter-write-without-central-applier", HA, "🔸area/🌉apply/🦀.rs", [
    "fn put(doc: &mut DocSnapshot) {",
    "    doc.rows.push(1); // @R17",
    "}",
    "fn put_ok(doc: &mut DocSnapshot, d: D) {",
    "    *doc = apply_diff(d);",
    "}",
], oracle_=False)
file_case("host-apply-writer-called-only-from-the-diff-machinery-is-exempt", HA, "🔸area/🌉apply/🦀.rs", [
    "fn put(doc: &mut DocSnapshot) {",
    "    doc.rows.push(1);",
    "}",
], oracle_=False, group={"🔸area/🌉apply/🔹b/🦀.rs": ["impl MutationDiff<D> for E {", "    fn apply(&self) { put(&mut s); }", "}"]}, expect=[])
file_case("host-apply-typescript-whole-archive-put-outside-an-import-path", HA, "🔸app/handlers.ts", [
    'const save = () => { put({ kind: "localDocumentArchive", bytes }); }; // @R17',
    'const importDoc = () => { admitArchive(x); put({ kind: "localDocumentArchive", bytes }); };',
], oracle_=False)


def tree_case(id_, tree, expect):
    cases.append({"id": id_, "rule": "policyInverseSumLawUntestedLeafBreaches", "tree": tree, "expect": expect})


def r15(path):
    return ["diff-only-r15-%s-1" % path, "diff-only-mutation/inverse-sum-law-untested", "high", 1]


tree_case("inverse-sum-law-diff-leaf-without-test", {"🔸area/🧬mutations/🔹add/🔺diff/🦀.rs": ["impl MutationDiff<X> for X {}"]}, [r15("🔸area/🧬mutations/🔹add/🔺diff/🦀.rs")])
tree_case("inverse-sum-law-tested-leaf-is-clean", {
    "🔸area/🧬mutations/🔹add/🔺diff/🦀.rs": ["impl MutationDiff<X> for X {}"],
    "🔸area/🧬mutations/🔹add/🧪tests/🔬t/🦀.rs": ["fn t() { assert_mutation_inverse_sum_law(a); }"],
}, [])
tree_case("inverse-sum-law-composite-plan-leaf-is-exempt", {
    "🔸area/🧬mutations/🔹plan1/🧩plan/🦀.rs": ["fn p() {}"],
    "🔸area/🧬mutations/🔹plan1/🔺diff/🦀.rs": ["fn d() {}"],
}, [])
tree_case("inverse-sum-law-payload-named-by-a-family-test", {
    "🔸area/🧬mutations/🔹move/🦀.rs": ["impl MutationKind<Doc> for Move {}"],
    "🔸area/🧬mutations/🔹other/🦀.rs": ["impl MutationKind<Doc> for Other {}"],
    "🔸area/🧪tests/🔬law/🦀.rs": ["fn law() { assert_mutation_inverse_sum_law(Move::new()); }", "fn unrelated() { other(); }"],
}, [r15("🔸area/🧬mutations/🔹other/🦀.rs")])
tree_case("inverse-sum-law-reached-through-a-helper", {
    "🔸area/🧬mutations/🔹swap/🦀.rs": ["impl MutationKind<Doc> for Swap {}"],
    "🔸area/🧪tests/🔬law/🦀.rs": ["fn law() { check(); }", "fn check() { assert_mutation_inverse_sum_law(Swap::new()); }"],
}, [])
tree_case("inverse-sum-law-slug-literal-names-the-leaf", {
    "🔸area/🧬mutations/🔹rotate-all/🔺diff/🦀.rs": ["impl MutationDiff<X> for X {}"],
    "🔸area/🧪tests/🔬law/🦀.rs": ['fn law() { assert_mutation_inverse_sum_law("rotate-all"); }'],
}, [])

props = {
    "path": {"type": "string"},
    "source": {"type": "array", "items": {"type": "string"}},
    "tree": {"type": "object", "additionalProperties": {"type": "array", "items": {"type": "string"}}},
    "group": {"type": "object", "additionalProperties": {"type": "array", "items": {"type": "string"}}},
}
rules = ["policyDiffOnlyFileBreaches", "policyRowEmitterFileBreaches", "policyHostApplyFileBreaches", "policyInverseSumLawUntestedLeafBreaches"]
schema = pure_schema("Diff Only Law Corpus", rules, props, [])
ts = pure_ts(
    "🧪 The diff-only mutation law reports exactly the breaches its language-agnostic Rust and TypeScript corpus marks, agrees with an independent Python port on every modelled row, and rejects a malformed corpus.",
    'import { policyDiffOnlyFileBreaches, policyHostApplyFileBreaches, policyInverseSumLawUntestedLeafBreaches, policyRowEmitterFileBreaches } from "../../🟦.ts";',
    '''  policyDiffOnlyFileBreaches: (row) => policyDiffOnlyFileBreaches(row.path, text(row.source), group(row)),
  policyRowEmitterFileBreaches: (row) => policyRowEmitterFileBreaches(row.path, text(row.source)),
  policyHostApplyFileBreaches: (row) => policyHostApplyFileBreaches(row.path, text(row.source), group(row)),
  policyInverseSumLawUntestedLeafBreaches: (row) => policyInverseSumLawUntestedLeafBreaches(Object.keys(row.tree), (path) => text(row.tree[path])),''',
)
ts = ts.replace("const rules:", 'const text = (lines: readonly string[]) => lines.join("\\n") + "\\n";\nconst group = (row: any) => (row.group ? () => new Map(Object.entries(row.group as Record<string, string[]>).map(([path, lines]) => [path, text(lines)] as const)) : undefined);\nconst rules:', 1)
oracle = oracle + ""
oracle = oracle.replace('cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]\noutput({c["id"]: run(c) for c in cases if c.get("oracle") is not False})\n', 'cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]\noutput({c["id"]: run(c) for c in cases if c.get("oracle") is not False})\n')
folder, plen = write_test("⚖laws/🧩plugin-consistency/🎯mutation-outcome/🧪diff-only-law", "🧪diff-only", ts, {"version": 1, "cases": cases}, schema, oracle)
print(folder, plen, len(cases))
