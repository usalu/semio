import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
read = lambda n: open(os.path.join(HERE, "..", n), encoding="utf8").read()
oracle = read("pylib.py") + "\nimport posixpath\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
REG = "📋build-owners.json"
cases = []


def case(id_, files, expect):
    cases.append({"id": id_, "rule": "policyBuildOwnerRegistryBreaches", "files": files, "expect": jsort(expect), "git": True})


def row(kind, key, scope, auto):
    return ["build-owner-registry-%s-%s" % (kind, key), "repo/build-owner-registry", "high", 0 if False else 0, auto, scope]


# a row is [id, kind, priority, autofixable, scope]
def r(kind, key, scope, auto):
    return ["build-owner-registry-%s-%s" % (kind, key), "repo/build-owner-registry", "high", auto, scope]


case("a-missing-registry-is-invalid", {"README.md": ["x"]}, [r("registry-invalid", REG, REG, False)])
case("an-unparseable-registry-is-invalid", {REG: ["{not json"]}, [r("registry-invalid", REG, REG, False)])
reg = '{"version":1,"owners":[{"name":"x","root":"a","manifest":"a/Cargo.toml","packages":[]},{"name":"y","root":"b","manifest":"b/Cargo.toml","packages":[]}],"catalogProviders":[{"ownerRoot":"p","sourcePath":"p/s.ts","consumerRoots":["q"]}]}'
case("authored-and-derived-findings-differ-in-autofixability", {
    REG: [reg], "b/Cargo.toml": ["[package]", 'name = "zzz"'], "c/Cargo.toml": ["[package]", 'name = "c"'],
}, [r("missing-root", "x", "a", True), r("name-mismatch", "y", "b/Cargo.toml", True), r("provider-owner-root", "p", "p", False), r("provider-source", "p/s.ts", "p/s.ts", False), r("provider-consumer-root", "q", "q", False), r("cargo-without-row", "c/Cargo.toml", "c/Cargo.toml", True)])
case("a-consistent-registry-is-clean", {REG: ['{"version":1,"owners":[{"name":"x","root":"a","manifest":"a/Cargo.toml","packages":[]}],"catalogProviders":[]}'], "a/Cargo.toml": ["[package]", 'name = "x"']}, [])
rules = ["policyBuildOwnerRegistryBreaches"]
schema = val_schema("Build Owner Registry Law Corpus", rules)
ts = val_ts(
    "📋 The build-owner registry law maps every registry finding to the breach id, priority, scope and autofixability its language-agnostic corpus names, and agrees with an independent Python model of the finding rules.",
    'import { policyBuildOwnerRegistryBreaches } from "../../🟦.ts";',
    '  policyBuildOwnerRegistryBreaches: (root) => policyBuildOwnerRegistryBreaches(root).map((b) => [b.id, b.kind, b.priority ?? "", b.autofixable ?? null, b.scope]).sort((a, b) => { const p = JSON.stringify(a), q = JSON.stringify(b); return p < q ? -1 : p > q ? 1 : 0; }),',
)
oracle = oracle.replace('return sorted([r[:5] + [r[5]] for r in rows]', 'return sorted([[r[0], r[1], r[2], r[4], r[5]] for r in rows]')
folder, plen = write_test("⚖laws/📋build-owner-registry", "📋registry", ts, {"version": 1, "cases": cases}, schema, oracle)
print(folder, plen, len(cases))
