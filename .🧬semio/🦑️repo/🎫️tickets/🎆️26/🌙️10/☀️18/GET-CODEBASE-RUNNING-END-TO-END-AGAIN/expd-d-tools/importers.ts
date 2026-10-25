import { existsSync, readFileSync, statSync } from "node:fs";
import { dirname, join, normalize } from "node:path";
import { spawnSync } from "node:child_process";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
const root = "/Users/ueli/Documents/semio";
const [target, ...names] = process.argv.slice(2) as string[];
const set = new Set(names);
const files = spawnSync("git", ["ls-files", "-co", "--exclude-standard", "-z"], { cwd: root, maxBuffer: 1 << 28, encoding: "utf8" }).stdout.split("\0").filter((f) => /\.(?:[cm]?[jt]sx?)$/u.test(f) && !f.includes("/node_modules/") && !f.startsWith(".🧬semio/") && existsSync(join(root, f)));
let n = 0;
for (const from of files) {
  const text = readFileSync(join(root, from), "utf8"); if (!text.includes("🕸️graph") && !text.includes(target.split("/").slice(-2, -1)[0]!)) continue;
  const sf = ts.createSourceFile(from, text, ts.ScriptTarget.Latest, true);
  for (const st of sf.statements) {
    const lit = (ts.isImportDeclaration(st) || ts.isExportDeclaration(st)) && st.moduleSpecifier && ts.isStringLiteral(st.moduleSpecifier) ? st.moduleSpecifier.text : null;
    if (!lit || !lit.startsWith(".") || normalize(join(dirname(from), lit)) !== target) continue;
    const got: string[] = [];
    if (ts.isImportDeclaration(st) && st.importClause?.namedBindings && ts.isNamedImports(st.importClause.namedBindings)) for (const e of st.importClause.namedBindings.elements) got.push((e.propertyName ?? e.name).text);
    else if (ts.isExportDeclaration(st) && st.exportClause && ts.isNamedExports(st.exportClause)) for (const e of st.exportClause.elements) got.push((e.propertyName ?? e.name).text);
    else got.push("*");
    const hit = got.filter((g) => set.has(g) || g === "*");
    if (hit.length) { n++; console.log(from.slice(-80), hit.join(","), got.length === hit.length ? "(all)" : "(mixed)"); }
  }
}
console.log("importers", n);
