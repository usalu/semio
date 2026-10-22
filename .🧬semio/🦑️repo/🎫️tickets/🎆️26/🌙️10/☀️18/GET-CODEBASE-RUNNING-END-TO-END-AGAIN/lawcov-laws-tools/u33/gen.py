import sys, os, re, json
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
read = lambda n: open(os.path.join(HERE, "..", n), encoding="utf8").read()
oracle = read("pylib.py") + "\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
unit = real_unit("⚖laws/🧩plugin-consistency/🧭breaches")
code = open(os.path.join(HERE, "oracle.py"), encoding="utf8").read().split("cases = json.load")[0]
code = re.sub(r"^SOURCE = .*$", "", code, flags=re.M)
ns = {"re": re, "os": os, "json": json, "sys": sys, "SOURCE": os.path.join(unit, fix("🟦.ts"))}
exec(code, ns)
wired, imported = ns["wiring"](), ns["imports"]()
cases = [{"id": "policy-breaches-wiring", "rule": "wiring", "expect": wired}, {"id": "policy-breaches-import-sources", "rule": "imports", "expect": imported}]
schema = val_schema("Breaches Aggregate Wiring Corpus", ["wiring", "imports"])
extra = '''
test("every wired law is a distinct exported function of the module it is imported from", async () => {
  const names = (corpus.cases[0] as any).expect.map((row: any) => row.name);
  expect(new Set(names).size).toBe(names.length);
  const sources = (corpus.cases[1] as any).expect as Record<string, string>;
  for (const name of names) {
    expect(sources[name], `${name} is imported`).toBeTruthy();
    const loaded = (await import(resolve(import.meta.dir, "../..", sources[name]!))) as Record<string, unknown>;
    expect(typeof loaded[name], `${name} is exported as a function`).toBe("function");
  }
});
'''
ts_test = val_ts(
    "🧭 The aggregate breach list calls exactly the laws and the loop-scoped crate laws its language-agnostic corpus pins, in order, each one a distinct exported function of its import source, as read by an independent Python parse and the TypeScript compiler API.",
    'import { readFileSync } from "node:fs";\nimport ts from "typescript";',
    '  wiring: () => compilerWiring(),\n  imports: () => compilerImports(),',
    extra,
)
ts_test = ts_test.replace("const rules:", '''const unitSource = ts.createSourceFile("breaches.ts", readFileSync(resolve(import.meta.dir, "../../🟦.ts"), "utf8"), ts.ScriptTarget.Latest, true);
const compilerWiring = () => {
  const rows: { name: string; args: string; loop: boolean }[] = [];
  const visit = (node: ts.Node, inLoop: boolean): void => {
    if (ts.isCallExpression(node) && ts.isPropertyAccessExpression(node.expression) && node.expression.name.text === "push" && node.expression.expression.getText(unitSource) === "breaches") {
      const arg = node.arguments[0];
      if (arg && ts.isSpreadElement(arg) && ts.isCallExpression(arg.expression)) rows.push({ name: arg.expression.expression.getText(unitSource), args: arg.expression.arguments.map((a) => a.getText(unitSource)).join(", "), loop: inLoop });
    }
    ts.forEachChild(node, (child) => visit(child, inLoop || ts.isForOfStatement(node)));
  };
  visit(unitSource, false);
  return rows;
};
const compilerImports = () => {
  const out: Record<string, string> = {};
  unitSource.forEachChild((node) => {
    if (!ts.isImportDeclaration(node) || !node.importClause?.namedBindings || !ts.isNamedImports(node.importClause.namedBindings) || node.importClause.isTypeOnly) return;
    for (const element of node.importClause.namedBindings.elements) if (!element.isTypeOnly) out[element.name.text] = (node.moduleSpecifier as ts.StringLiteral).text;
  });
  return out;
};
const rules:''', 1)
folder, plen = write_test("⚖laws/🧩plugin-consistency/🧭breaches", "🧭wiring", ts_test, {"version": 1, "cases": cases}, schema, oracle)
print(folder, plen, len(wired), len(imported))
