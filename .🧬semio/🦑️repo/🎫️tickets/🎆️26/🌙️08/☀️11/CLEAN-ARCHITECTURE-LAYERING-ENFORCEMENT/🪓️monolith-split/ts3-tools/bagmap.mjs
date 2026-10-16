// usage: node bagmap.mjs <host.ts> <registrar-fn-prefix-regex>  -> prints for each registerTestsN call the names and their import module specifiers in the host file
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import fs from "node:fs";
const [file, re] = process.argv.slice(2);
const text = fs.readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const importOf = new Map();
for (const s of sf.statements) if (ts.isImportDeclaration(s) && s.importClause) {
  const c = s.importClause;
  if (c.name) importOf.set(c.name.text, { spec: s.moduleSpecifier.text, kind: "default" });
  if (c.namedBindings && ts.isNamedImports(c.namedBindings)) for (const el of c.namedBindings.elements) importOf.set(el.name.text, { spec: s.moduleSpecifier.text, imported: (el.propertyName ?? el.name).text, typeOnly: c.isTypeOnly || el.isTypeOnly });
}
const re2 = new RegExp(re);
const visit = (n) => {
  if (ts.isCallExpression(n) && ts.isIdentifier(n.expression) && re2.test(n.expression.text)) {
    const bag = n.arguments[1];
    console.log("== " + n.expression.text);
    if (bag && ts.isObjectLiteralExpression(bag)) for (const p of bag.properties) {
      const name = ts.isShorthandPropertyAssignment(p) ? p.name.text : p.name.getText(sf);
      const m = importOf.get(name);
      console.log(`  ${name}\t${m ? m.spec + (m.imported && m.imported !== name ? " as " + m.imported : "") : "<not imported at top level>"}`);
    }
  }
  ts.forEachChild(n, visit);
};
visit(sf);
