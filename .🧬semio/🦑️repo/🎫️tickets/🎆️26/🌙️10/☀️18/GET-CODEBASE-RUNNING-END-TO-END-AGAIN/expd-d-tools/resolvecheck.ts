import { existsSync, readFileSync, statSync } from "node:fs";
import { dirname, join, normalize } from "node:path";
import { spawnSync } from "node:child_process";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
const root = "/Users/ueli/Documents/semio";
const scope = process.argv[2] ?? "🧰️framework/🛍️products/🦑️repo/";
const files = spawnSync("git", ["ls-files", "-co", "--exclude-standard", "-z", "--", scope], { cwd: root, maxBuffer: 1 << 28, encoding: "utf8" }).stdout.split("\0").filter((f) => /\.(?:[cm]?[jt]sx?)$/u.test(f) && !f.includes("/node_modules/") && existsSync(join(root, f)));
let bad = 0;
for (const from of files) {
  const text = readFileSync(join(root, from), "utf8"), sf = ts.createSourceFile(from, text, ts.ScriptTarget.Latest, true);
  const check = (spec: string) => { if (!spec.startsWith(".")) return; const clean = spec.split("?")[0]!; const to = normalize(join(dirname(from), clean)); try { if (!statSync(join(root, to)).isFile()) throw 0; } catch { bad++; console.log("UNRESOLVED", from.slice(-70), "->", spec.slice(-70)); } };
  const visit = (n: ts.Node) => {
    if ((ts.isImportDeclaration(n) || ts.isExportDeclaration(n)) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier)) check(n.moduleSpecifier.text);
    else if (ts.isCallExpression(n) && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0]) && (n.expression.kind === ts.SyntaxKind.ImportKeyword || (ts.isIdentifier(n.expression) && n.expression.text === "require"))) check(n.arguments[0].text);
    ts.forEachChild(n, visit);
  };
  visit(sf);
}
console.log("files", files.length, "unresolved", bad);
