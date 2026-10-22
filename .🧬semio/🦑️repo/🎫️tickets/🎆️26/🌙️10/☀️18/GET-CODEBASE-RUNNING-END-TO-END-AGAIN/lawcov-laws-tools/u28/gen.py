import sys, os, re
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
read = lambda n: open(os.path.join(HERE, "..", n), encoding="utf8").read()
oracle = read("pylib.py") + "\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
VOCAB = {"codes": [{"code": "mutation.target-missing", "level": "error"}, {"code": "mutation.no-op", "level": "warning"}, {"code": "mutation.clamped", "level": "warning"}], "applyPattern": "^mutation\\.apply\\.[a-z-]+$", "applyLevel": "error"}
K = "mutation-migration/message-code"
cases = []


def case(id_, path, lines, expect):
    cases.append({"id": id_, "rule": "policyOutcomeCodeFileBreaches", "path": path, "source": lines, "vocabulary": VOCAB, "expect": expect})


def mc(path, n, code):
    return ["mutation-message-code-%s-%d-%s" % (path, n, code), K, "high", n]


def at(lines, n, needle):
    return len("\n".join(lines[: n - 1])) + (1 if n > 1 else 0) + lines[n - 1].index(needle)


P1 = "a/🧬mutations/🔹m/🔺diff/🦀.rs"
rs = [
    "fn build() -> MutationOutcome<D> {",
    '    let ok = MutationOutcome::error("mutation.target-missing");',
    '    let bad_level = MutationOutcome::fatal("mutation.target-missing");',
    '    let unknown = MutationMessage::info("mutation.not-a-code");',
    '    out.warning("mutation.clamped");',
    '    out.info("mutation.target-missing");',
    '    let l = "mutation.leaf-literal";',
    '    MutationMessage::warn("x");',
    "}",
    'fn other() { let c = x.warning("mutation.zzz"); y.warn(1); }',
    'let e = MutationApplyError::new("mutation.apply.bad-thing");',
    'let f = MutationApplyError { code: "nope" };',
    '    "warn" => Severity::Warning,',
]
case("rust-leaf-sources-use-the-frozen-vocabulary", P1, rs, [
    mc(P1, 3, "mutation.target-missing"), mc(P1, 4, "mutation.not-a-code"), mc(P1, 6, "mutation.target-missing"), mc(P1, 7, "mutation.leaf-literal"), mc(P1, 10, "mutation.zzz"), mc(P1, 12, "nope"),
    ["mutation-message-retired-builder-%s-8-%d" % (P1, at(rs, 8, "MutationMessage::warn")), K, "high", 8], ["mutation-message-level-alias-%s-13" % P1, K, "high", 13],
])
P2 = "a/b/x.rs"
case("rust-outside-leaf-folders-skips-namespace-literals", P2, ['let l = "mutation.leaf-literal";', 'let b = MutationMessage::error("mutation.nope");'], [mc(P2, 2, "mutation.nope")])
P3 = "a/🧬mutations/🔹m/🟦.ts"
ts = ['refuse("warn", "mutation.no-op");', 'refuse("warning", "mutation.no-op");', 'const a = { level: "error", code: "mutation.target-missing" };', 'const b = { code: "mutation.zzz", level: "info" };', 'const c = "mutation.leaf-ts";']
case("typescript-sources-use-the-frozen-vocabulary", P3, ts, [mc(P3, 1, "mutation.no-op"), mc(P3, 4, "mutation.zzz"), mc(P3, 5, "mutation.leaf-ts")])
P4 = "a/🧪tests/t.py"
case("python-tests-use-the-frozen-vocabulary", P4, ['t1 = ("error", "mutation.target-missing")', 't2 = ("warn", "mutation.no-op")', 'x = "mutation.bogus-py"'], [mc(P4, 2, "mutation.no-op"), mc(P4, 3, "mutation.bogus-py")])
case("python-outside-tests-is-not-scanned", "a/b.py", ['x = "mutation.bogus-py"'], [])
P5 = "a/x.feature"
feat = ["Then outcome has mutation.target-missing", "And mutation.unknown-token appears", "See mutation.fixture.json for details", "level: warning, code: mutation.target-missing"]
case("feature-files-use-the-frozen-vocabulary", P5, feat, [mc(P5, 2, "mutation.unknown-token"), mc(P5, 4, "mutation.target-missing")])
P6 = "a/🎯outcome/🔣.json"
js = ["{", '  "code": "mutation.target-missing",', '  "level": "warning",', '  "messages": [', '    { "code": "mutation.zzz", "level": "error" },', '    { "code": "mutation.no-op", "level": "warning" }', "  ]", "}"]
case("outcome-json-documents-use-the-frozen-vocabulary", P6, js, [mc(P6, 2, "mutation.target-missing"), mc(P6, 5, "mutation.zzz")])
case("unparseable-json-is-ignored", P6, ["{broken"], [])
case("empty-sources-are-clean", "a/x.rs", [], [])
props = {"path": {"type": "string"}, "source": {"type": "array", "items": {"type": "string"}}, "vocabulary": {"type": "object", "additionalProperties": False, "required": ["codes", "applyPattern", "applyLevel"], "properties": {"codes": {"type": "array", "items": {"type": "object", "additionalProperties": False, "required": ["code", "level"], "properties": {"code": {"type": "string"}, "level": {"type": "string"}}}}, "applyPattern": {"type": "string"}, "applyLevel": {"type": "string"}}}}
schema = pure_schema("Mutation Outcome Law Corpus", ["policyOutcomeCodeFileBreaches"], props, ["path", "source", "vocabulary"])
ts_test = pure_ts(
    "🎯 The mutation-outcome code law reports exactly the vocabulary breaches its language-agnostic corpus names across Rust, TypeScript, Python, feature and JSON sources, agrees with an independent Python model and rejects a malformed corpus.",
    'import { policyOutcomeCodeFileBreaches } from "../../🟦.ts";',
    "  policyOutcomeCodeFileBreaches: (row) => policyOutcomeCodeFileBreaches({ codes: new Map(row.vocabulary.codes.map((c: any) => [c.code, c.level])), applyPattern: new RegExp(row.vocabulary.applyPattern), applyLevel: row.vocabulary.applyLevel }, row.path, row.source.length ? row.source.join(\"\\n\") + \"\\n\" : \"\"),",
)
folder, plen = write_test("⚖laws/🧩plugin-consistency/🎯mutation-outcome", "🎯outcome", ts_test, {"version": 1, "cases": cases}, schema, oracle)
print(folder, plen, len(cases))
