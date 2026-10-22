import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
read = lambda n: open(os.path.join(HERE, "..", n), encoding="utf8").read()
oracle = read("pylib.py") + "\nimport posixpath\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
CL = "🧰framework/🛍products/🦑repo/🔨modules/📚library/🏗build/📥source/🛗zero-touch/🗂closures/🔣.json"
cases = []


def roster(closures, seams=()):
    return [json.dumps({"version": 1, "closures": closures, "seams": [{"module": s, "reason": "post-install"} for s in seams]})]


def r(closure, kind, file, spec):
    return ["zero-touch-%s-%s-%s-%s" % (closure, kind, file, spec), "repo/zero-touch-closure", "high", False, file]


def case(id_, files, expect, workspaces=()):
    files = {"package.json": [json.dumps({"workspaces": list(workspaces)})], **files}
    cases.append({"id": id_, "rule": "policyZeroTouchClosureBreaches", "files": files, "expect": jsort(expect)})


E1 = lambda mode="full": {"id": "c1", "entry": "e.ts", "mode": mode, "reason": "x"}
case("external-packages-are-refused-through-the-relative-closure", {
    CL: roster([E1()]), "e.ts": ['import fs from "node:fs";', 'import p from "path";', 'import left from "left-pad";', 'import x from "./b";'], "b.ts": ['const l = require("lodash");', 'export * from "./c.js";'], "c.js": ['import "chalk";'],
}, [r("c1", "external-package", "e.ts", "left-pad"), r("c1", "external-package", "b.ts", "lodash"), r("c1", "external-package", "c.js", "chalk")])
case("a-missing-closure-entry-is-reported", {CL: roster([E1()])}, [r("c1", "missing-entry", "e.ts", "e.ts")])
case("seam-modules-may-only-be-imported-dynamically", {
    CL: roster([E1()], ["seam.ts"]), "e.ts": ['import s from "./seam";', 'const d = await import("./seam");'], "seam.ts": ['import "never-scanned";'],
}, [r("c1", "seam-static", "e.ts", "./seam")])
case("a-missing-seam-module-is-reported", {CL: roster([E1()], ["gone.ts"]), "e.ts": ["export {};"]}, [r("seams", "missing-entry", "gone.ts", "gone.ts")])
case("static-closures-ignore-dynamic-and-require-imports", {CL: roster([E1("static")]), "e.ts": ['import a from "static-dep";', 'const b = await import("dynamic-dep");', 'const c = require("required-dep");']}, [r("c1", "external-package", "e.ts", "static-dep")])
case("workspace-package-specifiers-must-be-relative", {
    CL: roster([E1()]), "e.ts": ['import a from "@x/p";', 'import b from "@x/p/sub";'], "pkgs/p/package.json": ['{"name": "@x/p", "exports": {".": "./index.ts"}}'], "pkgs/p/index.ts": ["export {};"],
}, [r("c1", "workspace-specifier", "e.ts", "@x/p"), r("c1", "workspace-specifier", "e.ts", "@x/p/sub")], workspaces=["pkgs/p"])
rules = ["policyZeroTouchClosureBreaches"]
schema = val_schema("Zero Touch Closure Law Corpus", rules)
ts = val_ts(
    "🛗 The zero-touch closure law reports exactly the external, workspace-specifier, seam and missing-entry findings its language-agnostic corpus names over synthetic import closures, and agrees with an independent Python model of the import scan.",
    'import { policyZeroTouchClosureBreaches } from "../../🟦.ts";',
    '  policyZeroTouchClosureBreaches: (root) => policyZeroTouchClosureBreaches(root).map((b) => [b.id, b.kind, b.priority ?? "", b.autofixable ?? null, b.scope]).sort((a, b) => { const p = JSON.stringify(a), q = JSON.stringify(b); return p < q ? -1 : p > q ? 1 : 0; }),',
)
folder, plen = write_test("⚖laws/🛗zero-touch-closure", "🛗closure", ts, {"version": 1, "cases": cases}, schema, oracle)
print(folder, plen, len(cases))
