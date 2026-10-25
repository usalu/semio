import { existsSync, readFileSync } from "node:fs";
import { dirname, join, normalize } from "node:path";
import { spawnSync } from "node:child_process";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
const root = "/Users/ueli/Documents/semio";
const target = process.argv[2]!;
const files = spawnSync("git", ["ls-files", "-co", "--exclude-standard", "-z"], { cwd: root, maxBuffer: 1 << 28, encoding: "utf8" }).stdout.split("\0").filter((f) => /\.(?:[cm]?[jt]sx?|d\.mts)$/u.test(f) && !f.includes("/node_modules/") && !f.startsWith(".🧬semio/") && existsSync(join(root, f)));
const base = target.split("/").slice(-2).join("/");
const byName = new Map<string, string[]>();
for (const from of files) {
  const t = readFileSync(join(root, from), "utf8");
  if (!t.includes(target.split("/").pop()!.split(".")[0]!) && !t.includes("from")) continue;
  const sf = ts.createSourceFile(from, t, ts.ScriptTarget.Latest, true);
  const hit = (spec: string) => spec.startsWith(".") && normalize(join(dirname(from), spec.split("?")[0]!)) === target;
  const visit = (n: ts.Node) => {
    if ((ts.isImportDeclaration(n) || ts.isExportDeclaration(n)) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier) && hit(n.moduleSpecifier.text)) {
      let names: string[] = [];
      if (ts.isImportDeclaration(n)) { const c = n.importClause; if (c?.name) names.push("default"); if (c?.namedBindings) names.push(...(ts.isNamespaceImport(c.namedBindings) ? ["*"] : c.namedBindings.elements.map((e) => (e.propertyName ?? e.name).text))); if (!c) names.push("(side-effect)"); }
      else names = n.exportClause && ts.isNamedExports(n.exportClause) ? n.exportClause.elements.map((e) => (e.propertyName ?? e.name).text) : ["* (export-star)"];
      for (const nm of names) (byName.get(nm) ?? byName.set(nm, []).get(nm)!).push(from);
    } else if (ts.isCallExpression(n) && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0]) && hit(n.arguments[0].text)) {
      const nm = "(dynamic/require)"; (byName.get(nm) ?? byName.set(nm, []).get(nm)!).push(from);
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
}
const strip = (p: string) => p.replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/", "L/");
for (const [n, f] of [...byName].sort()) console.log(n, f.length, f.slice(0, 40).map(strip).join(" | "));
