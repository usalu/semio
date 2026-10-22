import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
read = lambda n: open(os.path.join(HERE, "..", n), encoding="utf8").read()
oracle = read("pylib.py") + "\n" + read("foundation.py") + "\n" + read("ptree.py") + "\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
cases = []
budget = json.load(open(os.path.join("/Users/ueli/Documents/semio", fix("🧰framework/🛍products/🦑repo/🔨modules/📚library/🔣taxonomy.json")), encoding="utf8"))["libWiringLineBudget"]
LIB = "✏s/🔌plugins/🔭p/📦packages/🦀rust/🦀.rs"
mk = lambda shape="taxonomy", pid="p": {"libRelPath": LIB, "shape": shape, "pluginId": pid, "ownerRel": "✏s/🔌plugins/🔭p", "dir": "d", "role": "plugin"}
LS = "policyTaxonomyLibShapeBreaches"
BS = "policyTaxonomyBarrelShapeBreaches"


def case(id_, rule, files, expect, crates=None):
    row = {"id": id_, "rule": rule, "files": files, "expect": expect}
    if crates is not None:
        row["crates"] = crates
    cases.append(row)


def fn(n, head="pub fn big() {", line="    let a = 1;"):
    return [head] + [line] * n + ["}"]


# the body slice starts at the opening brace, so a function with n inner lines counts n + 2 lines
inner_over, inner_exact = budget - 1, budget - 2
case("lib-wiring-over-the-line-budget", LS, {LIB: fn(inner_over)}, [["taxonomy-lib-shape-" + LIB, "taxonomy/lib-shape", "medium", 0]], [mk()])
case("lib-wiring-at-the-line-budget-is-clean", LS, {LIB: fn(inner_exact)}, [], [mk()])
case("lib-shape-of-a-non-plugin-crate-is-low", LS, {LIB: fn(inner_over)}, [["taxonomy-lib-shape-" + LIB, "taxonomy/lib-shape", "low", 0]], [mk(pid="")])
case("legacy-crates-are-not-measured", LS, {LIB: fn(inner_over)}, [], [mk("legacy")])
case("comments-and-blank-lines-do-not-count", LS, {LIB: fn(budget, line="    // comment") + [""] * 50}, [], [mk()])
case("nested-bodies-are-counted-once-through-their-impl", LS, {LIB: ["impl X {"] + fn(budget // 2 - 3, "    fn a() {", "        let a = 1;") + fn(budget // 2 - 3, "    fn b() {", "        let a = 1;") + ["}"]}, [], [mk()])
case("several-top-level-functions-add-up", LS, {LIB: fn(budget // 2) + fn(budget // 2, "pub fn second() {")}, [["taxonomy-lib-shape-" + LIB, "taxonomy/lib-shape", "medium", 0]], [mk()])
PKG = "🧰framework/🔨modules/🔸b/📦packages/🟦typescript/"
marker = ['{"name": "b", "semio": {"role": "library"}}']
over = ["const v%d = %d;" % (i, i) for i in range(budget + 1)]
case("barrel-with-too-much-logic", BS, {PKG + "package.json": marker, PKG + "🟦.ts": over}, [["taxonomy-barrel-shape-" + PKG + "package.json", "taxonomy/barrel-shape", "medium", 0]])
case("barrel-at-the-budget-is-clean", BS, {PKG + "package.json": marker, PKG + "🟦.ts": over[:budget]}, [])
wiring = ['export * from "./a";', 'export type { T } from "./b";', 'export { x, y } from "./c";', 'import d from "./d";', "// comment", "", "   "] * 40
case("wiring-lines-never-count", BS, {PKG + "package.json": marker, PKG + "🟦.ts": wiring}, [])
case("barrel-without-a-role-marker-is-not-a-package", BS, {PKG + "package.json": ['{"name": "b"}'], PKG + "🟦.ts": over}, [])

props = {"crates": {"type": "array", "items": {"type": "object", "additionalProperties": False, "required": ["libRelPath", "shape", "pluginId", "ownerRel", "dir", "role"], "properties": {k: {"type": "string"} for k in ("libRelPath", "shape", "pluginId", "ownerRel", "dir", "role")}}}}
rules = [LS, BS]
schema = std_schema("Lib And Barrel Shape Law Corpus", rules, props)
refs = "(row.crates ?? []).map((c: any) => ({ dir: c.dir, libRelPath: c.libRelPath, shape: c.shape, role: c.role, ownerRel: c.ownerRel, pluginId: c.pluginId }))"
ts = std_ts(
    "🧱 The lib-wiring and barrel-wiring laws measure authored logic against the taxonomy line budget exactly as their language-agnostic corpus says, agree with an independent Python model and reject a malformed corpus.",
    'import { policyTaxonomyBarrelShapeBreaches, policyTaxonomyLibShapeBreaches } from "../../🟦.ts";',
    "  %s: (root, row) => %s(root, %s),\n  %s: (root) => %s(root)," % (LS, LS, refs, BS, BS),
)
folder, plen = write_test("⚖laws/🧩plugin-consistency/🗺taxonomy-shape/🧱lib-and-barrel-shape", "🧱wiring", ts, {"version": 1, "cases": cases}, schema, oracle)
print(folder, plen, len(cases))
