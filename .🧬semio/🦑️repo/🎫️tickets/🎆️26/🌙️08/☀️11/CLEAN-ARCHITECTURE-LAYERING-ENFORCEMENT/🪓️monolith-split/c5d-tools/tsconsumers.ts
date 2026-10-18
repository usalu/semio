// 🔎 c5d: replicates the `directRelativeModules` of the plugin publication source ownership law for the listed consumers and prints the owners they import. usage: bun tsconsumers.ts
import {readFileSync} from "node:fs";
import {dirname, relative, resolve} from "node:path";
const ts = (await import("/Users/ueli/.bun/install/cache/typescript@5.9.3@@@1/lib/typescript.js")).default as typeof import("typescript");
const repo = "/Users/ueli/Documents/semio";
const fx = JSON.parse(readFileSync(`${repo}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/📣️plugin-publication-source-ownership/🔣️.json`, "utf8"));
const owners = new Set<string>(fx.owners.map((o: {path: string}) => o.path));
for (const [i, c] of fx.consumers.entries()) {
  const path = resolve(repo, c.path);
  const sf = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const mods = new Set<string>();
  const add = (e: ts.Expression | undefined) => { if (e && ts.isStringLiteral(e) && e.text.startsWith(".")) mods.add(relative(repo, resolve(dirname(path), e.text)).replaceAll("\\", "/")); };
  const visit = (n: ts.Node): void => { if (ts.isImportDeclaration(n)) add(n.moduleSpecifier); if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword) add(n.arguments[0]); ts.forEachChild(n, visit); };
  visit(sf);
  const actual = [...mods].filter(m => owners.has(m)).sort();
  const expected = [...c.owners].sort();
  console.log(i, JSON.stringify(actual) === JSON.stringify(expected) ? "OK" : "DIFF", JSON.stringify(actual.map(a => a.split("/").slice(-2).join("/"))), "expected", expected.length);
  if (process.argv.includes("--json") && i === 11) console.log("JSON11", JSON.stringify(actual));
}
