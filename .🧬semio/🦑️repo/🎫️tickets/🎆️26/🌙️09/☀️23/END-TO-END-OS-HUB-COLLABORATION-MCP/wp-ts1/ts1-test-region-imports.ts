/** 🔎️ TS1: static imports that sit inside a source's `// #region 🧪️Tests` and whether production code outside that region uses each name. */
import ts from "typescript";
import { readFileSync } from "node:fs";
import { relative } from "node:path";
const repo = "/Users/ueli/Documents/semio";
const files = readFileSync(0, "utf8").split("\n").filter(Boolean);
for (const file of files) {
  const text = readFileSync(file, "utf8");
  const start = text.indexOf("// #region 🧪️Tests");
  if (start < 0) { console.log(`${relative(repo, file)}: no tests region`); continue; }
  const end = text.indexOf("// #endregion 🧪️Tests", start);
  const source = ts.createSourceFile(file, text, ts.ScriptTarget.ESNext, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  for (const statement of source.statements) {
    if (!ts.isImportDeclaration(statement) || statement.getStart() < start || statement.getStart() > end) continue;
    const names = statement.importClause?.namedBindings && ts.isNamedImports(statement.importClause.namedBindings) ? statement.importClause.namedBindings.elements.map((e) => e.name.text) : [];
    const used: string[] = [];
    const visit = (node: ts.Node): void => {
      if (ts.isIdentifier(node) && names.includes(node.text) && (node.getStart() < start || node.getStart() > end) && !ts.isImportSpecifier(node.parent)) used.push(node.text);
      ts.forEachChild(node, visit);
    };
    visit(source);
    console.log(`${relative(repo, file).replace(/^.*?(⚙️engine\/|🧩️extensions\/)/, "")}\t${(statement.moduleSpecifier as ts.StringLiteral).text.replace(/^(\.\.\/)+/, "…/")}\t${names.length} names\tprod-used: ${[...new Set(used)].join(",") || "-"}`);
  }
}
