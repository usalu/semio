import fs from "node:fs";
import path from "node:path";
import ts from "typescript";
const ROOT = "/Users/ueli/Documents/semio/";
const ELEMENTS = ROOT + "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements";
export function walk(dir, out = []) {
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    if (entry.name === "node_modules" || entry.name === "🗑️generated") continue;
    const full = path.join(dir, entry.name);
    if (entry.isDirectory()) { if (entry.name === "🧪️tests") continue; walk(full, out); }
    else if (/\.(ts|tsx)$/.test(entry.name) && !entry.name.endsWith(".d.ts")) out.push(full);
  }
  return out;
}
if (import.meta.main) {
  const files = walk(ELEMENTS);
  const forms = new Map();
  const names = new Map();
  let importing = 0;
  for (const file of files) {
    const text = fs.readFileSync(file, "utf8");
    if (!text.includes('"react"') && !text.includes("'react'")) continue;
    const source = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
    let hit = false;
    const note = (kind) => forms.set(kind, (forms.get(kind) ?? 0) + 1);
    const visit = (node) => {
      if (ts.isImportDeclaration(node) && ts.isStringLiteral(node.moduleSpecifier) && node.moduleSpecifier.text === "react") {
        hit = true;
        const clause = node.importClause;
        if (!clause) note("side-effect");
        else {
          if (clause.isTypeOnly) note("type-only");
          if (clause.name) note("default");
          if (clause.namedBindings && ts.isNamespaceImport(clause.namedBindings)) note("namespace");
          if (clause.namedBindings && ts.isNamedImports(clause.namedBindings)) for (const el of clause.namedBindings.elements) {
            const key = (el.isTypeOnly || clause.isTypeOnly ? "type " : "") + (el.propertyName ?? el.name).text;
            names.set(key, (names.get(key) ?? 0) + 1);
            if (el.propertyName) note("renamed");
          }
        }
      } else if (ts.isExportDeclaration(node) && node.moduleSpecifier && ts.isStringLiteral(node.moduleSpecifier) && node.moduleSpecifier.text === "react") { hit = true; note("re-export"); }
      else if (ts.isImportTypeNode(node) && ts.isLiteralTypeNode(node.argument) && ts.isStringLiteral(node.argument.literal) && node.argument.literal.text === "react") { hit = true; note("import-type"); }
      else if (ts.isCallExpression(node) && node.expression.kind === ts.SyntaxKind.ImportKeyword && node.arguments[0] && ts.isStringLiteral(node.arguments[0]) && node.arguments[0].text === "react") { hit = true; note("dynamic-import"); }
      else if (ts.isCallExpression(node) && ts.isIdentifier(node.expression) && node.expression.text === "require" && node.arguments[0] && ts.isStringLiteral(node.arguments[0]) && node.arguments[0].text === "react") { hit = true; note("require"); }
      ts.forEachChild(node, visit);
    };
    visit(source);
    if (hit) importing++;
  }
  console.log(JSON.stringify({ files: files.length, importing, forms: Object.fromEntries(forms) }));
  console.log([...names.entries()].sort((a, b) => b[1] - a[1]).map(([n, c]) => `${n}:${c}`).join(" "));
}
