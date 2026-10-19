import ts from "typescript";
import { readFileSync, existsSync } from "node:fs";
const ROOT = "/Users/ueli/Documents/semio/";
const spec = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const files = new Set<string>();
for (const rows of Object.values<any>(spec.byPackage)) for (const r of Object.values<any>(rows)) for (const i of r.importers) files.add(i);
let totalImports = 0, unusedImports = 0; const report: any[] = [];
for (const f of files) {
  const sf = ts.createSourceFile(f, readFileSync(ROOT + f, "utf8"), ts.ScriptTarget.Latest, true);
  const used = new Set<string>();
  const visit = (n: ts.Node): void => {
    if (ts.isImportDeclaration(n)) return;
    if (ts.isIdentifier(n)) used.add(n.text);
    if (ts.isExportSpecifier(n)) used.add((n.propertyName ?? n.name).text);
    ts.forEachChild(n, visit);
  };
  sf.forEachChild(visit);
  for (const st of sf.statements) {
    if (!ts.isImportDeclaration(st) || !ts.isStringLiteral(st.moduleSpecifier) || !st.moduleSpecifier.text.includes("🦑️repo")) continue;
    const c = st.importClause; if (!c) continue;
    const names: string[] = [];
    if (c.name) names.push(c.name.text);
    if (c.namedBindings) { if (ts.isNamespaceImport(c.namedBindings)) names.push(c.namedBindings.name.text); else for (const e of c.namedBindings.elements) names.push(e.name.text); }
    totalImports++;
    const un = names.filter((x) => !used.has(x));
    if (un.length === names.length) { unusedImports++; report.push({ f, spec: st.moduleSpecifier.text.split("/").slice(-3).join("/"), unused: un }); }
    else if (un.length) report.push({ f, spec: st.moduleSpecifier.text.split("/").slice(-3).join("/"), partial: un });
  }
}
console.log({ files: files.size, totalImports, fullyUnusedImports: unusedImports, partial: report.filter((r) => r.partial).length });
console.log(JSON.stringify(report.slice(0, 40), null, 0));
