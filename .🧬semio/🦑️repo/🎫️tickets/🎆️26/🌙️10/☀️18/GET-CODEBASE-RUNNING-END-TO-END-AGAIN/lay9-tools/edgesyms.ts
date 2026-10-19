import ts from "typescript";
import { readFileSync } from "node:fs";
const report = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const only = process.argv[3]; // substring filter of 'to'
const members = process.argv[4] ? new Set<string>(JSON.parse(readFileSync(process.argv[4], "utf8"))) : null;
const R = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/";
const ROOT = "/Users/ueli/Documents/semio/";
const s = (p: string) => p.replace(R, "").replace(/\/🟦️\.(tsx?)$/, "/#");
for (const m of report.modules) {
  if (members ? !members.has(m.source) : !m.source.startsWith(R)) continue;
  const deps = m.dependencies.filter((d: any) => d.resolved && (only ? d.resolved.includes(only) : true) && (!members || members.has(d.resolved)));
  if (!deps.length) continue;
  const sf = ts.createSourceFile(m.source, readFileSync(ROOT + m.source, "utf8"), ts.ScriptTarget.Latest, true, m.source.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const bySpec = new Map<string, string[]>();
  const add = (spec: string, v: string) => bySpec.set(spec, [...(bySpec.get(spec) ?? []), v]);
  const visit = (n: ts.Node): void => {
    if (ts.isImportDeclaration(n) && ts.isStringLiteral(n.moduleSpecifier)) {
      const c = n.importClause; const names: string[] = [];
      if (c?.isTypeOnly) names.push("(type-only)");
      if (c?.name) names.push("default:" + c.name.text);
      if (c?.namedBindings) { if (ts.isNamespaceImport(c.namedBindings)) names.push("* as " + c.namedBindings.name.text); else for (const e of c.namedBindings.elements) names.push((e.isTypeOnly ? "type " : "") + (e.propertyName ?? e.name).text); }
      add(n.moduleSpecifier.text, names.join(",") || "(side-effect)");
    } else if (ts.isExportDeclaration(n) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier)) {
      add(n.moduleSpecifier.text, "export " + (n.isTypeOnly ? "type " : "") + (n.exportClause && ts.isNamedExports(n.exportClause) ? n.exportClause.elements.map((e) => e.name.text).join(",") : "*"));
    } else if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0])) add(n.arguments[0].text, "dynamic()");
    else if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal)) add(n.argument.literal.text, "import(type)." + (n.qualifier?.getText() ?? "*"));
    ts.forEachChild(n, visit);
  };
  visit(sf);
  for (const d of deps) console.log(s(m.source), "->", s(d.resolved), "::", (bySpec.get(d.module) ?? ["?"]).join(" | "));
}
