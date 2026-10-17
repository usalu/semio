/** 🧭️ Resolves every static import specifier of the given files the way bun does from the file's directory. Usage: bun resolve-check.ts <file>... */
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
let bad = 0, total = 0;
for (const arg of process.argv.slice(2)) {
  const file = resolve(arg);
  const sf = ts.createSourceFile(file, readFileSync(file, "utf8"), ts.ScriptTarget.Latest, true);
  const specs: string[] = [];
  const visit = (n: ts.Node) => {
    if ((ts.isImportDeclaration(n) || ts.isExportDeclaration(n)) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier)) specs.push(n.moduleSpecifier.text);
    if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal)) specs.push(n.argument.literal.text);
    ts.forEachChild(n, visit);
  };
  visit(sf);
  for (const s of specs) {
    total++;
    try { Bun.resolveSync(s, dirname(file)); } catch (e) { bad++; console.log("UNRESOLVED", s, "in", arg.slice(-70)); }
  }
}
console.log({ total, bad });
