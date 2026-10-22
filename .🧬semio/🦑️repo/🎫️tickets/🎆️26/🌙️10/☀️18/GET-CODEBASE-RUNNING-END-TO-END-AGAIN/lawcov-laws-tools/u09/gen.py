import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
read = lambda n: open(os.path.join(HERE, "..", n), encoding="utf8").read()
oracle = read("pylib.py") + "\nimport posixpath\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
cases = []
A, B = "✏s/🔌plugins/🔭alpha", "✏s/🔌plugins/🔭beta"
CA, CB = A + "/📦packages/🦀rust", B + "/📦packages/🦀rust"
marker = ["[package]", 'name = "x"', "[package.metadata.semio]", 'role = "plugin"']
crates = [{"dir": CA, "pluginId": "alpha"}, {"dir": CB, "pluginId": "beta"}, {"dir": "tools/x", "pluginId": ""}]


def case(id_, rule, files, expect, **kw):
    row = {"id": id_, "rule": rule, "files": files, "expect": expect}
    row.update(kw)
    cases.append(row)


def b(i, kind, prio, line=0):
    return [i, kind, prio, line]


case("stray-lockfiles-in-crate-directories", "policyCargoArtifactBreaches", {CA + "/Cargo.lock": [], CB + "/Cargo.toml": [], "tools/x/Cargo.lock": []},
     [b("stray-cargo-artifact-%s-Cargo.lock" % CA, "app-plugin/stray-cargo-artifact", "high"), b("stray-cargo-artifact-tools/x-Cargo.lock", "app-plugin/stray-cargo-artifact", "low")], crates=crates)
CK = "app-plugin/app-coupling"
case("plugin-crates-do-not-depend-on-each-other", "policyAppCouplingBreaches", {
    CA + "/Cargo.toml": marker + ["[dependencies]", 'beta_crate = { path = "../../../🔭beta/📦packages/🦀rust" }', 'flow = { path = "../../../🔭beta/🧱lib" }', 'odd = { path = "../../../🔭beta/🧱other" }',
                                  'self_dep = { path = "../../🧱x" }', 'ext = { path = "../../../../../🧰framework/x" }', 'v = "1"'],
    CB + "/Cargo.toml": marker, "tools/x/Cargo.toml": ["[dependencies]", 'tool_dep = { path = "../../✏s/🔌plugins/🔭beta/📦packages/🦀rust" }'],
}, [b("app-coupling-plugin-%s-beta_crate" % CA, CK, "medium"), b("app-coupling-domain-%s-odd" % CA, CK, "low"), b("app-coupling-plugin-tools/x-tool_dep", CK, "low")], crates=crates)
JK = "dsl-migration/no-json-fixture"
case("json-example-fixtures-are-forbidden", "policyJsonFixtureBreaches", {
    "a/example/one.json": [], "a/example/two.txt": [], "a/b/example/deep.json": [], "coda/x/example/ok.json": [], "a/examples/other.json": [], "node_modules/example/n.json": [],
}, [b("no-json-fixture-a/example/one.json", JK, "high"), b("no-json-fixture-a/b/example/deep.json", JK, "high")])
OK = "dsl-migration/ops-grammar"
case("ops-files-follow-the-op-log-grammar", "policyOpsGrammarBreaches", {
    "a/good.ops": ["@doc d", "@edit e", "  set x 1", "@checkpoint c", "@alternative a", "@active z"],
    "a/bad.ops": ["@doc d", "", "stray text", " one-space", "@unknown", "  ok", "   "],
}, [b("ops-grammar-blank-a/bad.ops-2", OK, "medium", 2), b("ops-grammar-line-a/bad.ops-3", OK, "medium", 3), b("ops-grammar-line-a/bad.ops-4", OK, "medium", 4), b("ops-grammar-line-a/bad.ops-5", OK, "medium", 5), b("ops-grammar-blank-a/bad.ops-7", OK, "medium", 7)])

props = {"crates": {"type": "array", "items": {"type": "object", "additionalProperties": False, "required": ["dir", "pluginId"], "properties": {"dir": {"type": "string"}, "pluginId": {"type": "string"}}}}}
rules = ["policyCargoArtifactBreaches", "policyAppCouplingBreaches", "policyJsonFixtureBreaches", "policyOpsGrammarBreaches"]
schema = std_schema("App Coupling Law Corpus", rules, props)
refs = "(row.crates ?? []).map((c: any) => ({ dir: c.dir, libRelPath: \"\", shape: \"taxonomy\", role: \"\", ownerRel: \"\", pluginId: c.pluginId }))"
ts = std_ts(
    "🧱 The app-coupling laws report exactly the stray-artifact, plugin-coupling, JSON-fixture and op-log grammar breaches their language-agnostic corpus names, agree with an independent Python model and reject a malformed corpus.",
    'import { policyAppCouplingBreaches, policyCargoArtifactBreaches, policyJsonFixtureBreaches, policyOpsGrammarBreaches } from "../../🟦.ts";',
    '''  policyCargoArtifactBreaches: (root, row) => policyCargoArtifactBreaches(root, %s),
  policyAppCouplingBreaches: (root, row) => policyAppCouplingBreaches(root, %s),
  policyJsonFixtureBreaches: (root) => policyJsonFixtureBreaches(root),
  policyOpsGrammarBreaches: (root) => policyOpsGrammarBreaches(root),''' % (refs, refs),
)
folder, plen = write_test("⚖laws/🧩plugin-consistency/🧱app-coupling", "🧱coupling", ts, {"version": 1, "cases": cases}, schema, oracle)
print(folder, plen, len(cases))
