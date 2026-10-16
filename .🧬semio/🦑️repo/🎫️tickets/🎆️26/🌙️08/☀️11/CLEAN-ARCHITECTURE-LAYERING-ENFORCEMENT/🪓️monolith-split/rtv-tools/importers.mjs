import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
const [target, listFile] = process.argv.slice(2);
const files = readFileSync(listFile, "utf8").split("\n").filter(Boolean);
const norm = (p) => p.replace(/\.(ts|tsx)$/u, "");
const t = norm(target);
for (const f of files) {
  if (!/\.(ts|tsx|mjs|js)$/u.test(f)) continue;
  let src; try { src = readFileSync(f, "utf8"); } catch { continue; }
  if (!src.includes("🟦️")) continue;
  const sf = ts.createSourceFile(f, src, ts.ScriptTarget.Latest, true);
  const hit = [];
  const visit = (n) => {
    let spec = null;
    if ((ts.isImportDeclaration(n) || ts.isExportDeclaration(n)) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier)) spec = n.moduleSpecifier.text;
    else if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0] && ts.isStringLiteral(n.arguments[0])) spec = n.arguments[0].text;
    else if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal)) spec = n.argument.literal.text;
    if (spec && spec.startsWith(".") && norm(resolve(dirname(f), spec)) === t) hit.push(sf.getLineAndCharacterOfPosition(n.getStart()).line + 1 + ":" + n.getText().slice(0, 80).replace(/\n/g, " "));
    ts.forEachChild(n, visit);
  };
  visit(sf);
  if (hit.length) console.log(f, hit.join(" || "));
}
