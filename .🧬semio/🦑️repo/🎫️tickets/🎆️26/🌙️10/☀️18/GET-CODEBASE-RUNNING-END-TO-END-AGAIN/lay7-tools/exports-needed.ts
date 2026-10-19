import ts from "typescript";
import { readFileSync, writeFileSync, existsSync } from "node:fs";
const ROOT = "/Users/ueli/Documents/semio/";
const [, , reportPath, outPath] = process.argv;
const OS = "🧰️framework/🛍️products/💻️os/";
const rend = OS + "🔨️modules/📺️renderer/";
const report = JSON.parse(readFileSync(reportPath!, "utf8"));
const mods = new Map<string, any>(report.modules.map((m: any) => [m.source, m]));
const LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const slug = (rel: string): string => rel.replace(/[^\x00-\x7f]/gu, "").replace(/\/+/gu, "/").replace(/\.(ts|mjs|cjs)$/u, "").replace(/(^|\/)\.json$/u, "").replace(/\.json$/u, "").replace(/(^|\/)tests(?=\/|$)/gu, "$1testing").replace(/^\/|\/$/gu, "");
type Row = { targetFile: string; proposedSubpath: string; symbols: string[] | "*"; importers: string[] };
const out: Record<string, Record<string, Row>> = {};
const namesFor = (file: string, spec: string): string[] | "*" => {
  const sf = ts.createSourceFile(file, readFileSync(ROOT + file, "utf8"), ts.ScriptTarget.Latest, true);
  const names = new Set<string>(); let star = false, found = false;
  const visit = (n: ts.Node): void => {
    if (ts.isImportDeclaration(n) && ts.isStringLiteral(n.moduleSpecifier) && n.moduleSpecifier.text === spec) {
      found = true; const c = n.importClause;
      if (!c) star = true; else { if (c.name) names.add("default"); if (c.namedBindings) { if (ts.isNamespaceImport(c.namedBindings)) star = true; else for (const e of c.namedBindings.elements) names.add((e.propertyName ?? e.name).text); } }
    } else if (ts.isExportDeclaration(n) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier) && n.moduleSpecifier.text === spec) {
      found = true; if (!n.exportClause || ts.isNamespaceExport(n.exportClause)) star = true; else for (const e of n.exportClause.elements) names.add((e.propertyName ?? e.name).text);
    } else if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0]) && n.arguments[0].text === spec) { found = true; star = true; }
    else if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal) && n.argument.literal.text === spec) { found = true; star = true; }
    ts.forEachChild(n, visit);
  };
  visit(sf);
  return star || !found ? "*" : [...names].sort();
};
const seen = new Set<string>();
for (const x of report.summary.violations as any[]) {
  if (!x.rule.name.startsWith("no-cross-package-relative-") || x.from.startsWith(rend) || !x.from.startsWith(OS) || !x.to.startsWith("🧰️framework/🛍️products/🦑️repo/")) continue;
  if (!existsSync(ROOT + x.from)) continue;
  const dep = mods.get(x.from)?.dependencies.find((d: any) => d.resolved === x.to);
  if (!dep) continue;
  const pkg = x.rule.name.replace("no-cross-package-relative-", "");
  if (seen.has(`${pkg}|${x.from}|${x.to}`)) continue; seen.add(`${pkg}|${x.from}|${x.to}`);
  const rel = x.to.startsWith(LIB) ? x.to.slice(LIB.length) : x.to.slice("🧰️framework/🛍️products/🦑️repo/".length);
  const key = pkg;
  const row = ((out[key] ??= {})[x.to] ??= { targetFile: x.to, proposedSubpath: "./" + (slug(rel) || "index"), symbols: [], importers: [] });
  const names = namesFor(x.from, dep.module);
  if (names === "*" || row.symbols === "*") row.symbols = "*"; else row.symbols = [...new Set([...row.symbols as string[], ...names])].sort();
  row.importers.push(x.from);
}
const collisions: Record<string, string[]> = {};
for (const [pkg, rows] of Object.entries(out)) { const m: Record<string, string[]> = {}; for (const r of Object.values(rows)) (m[r.proposedSubpath] ??= []).push(r.targetFile); for (const [s, t] of Object.entries(m)) if (t.length > 1) collisions[pkg + " " + s] = t; }
const summary = Object.entries(out).map(([pkg, rows]) => ({ package: pkg, newExportEntries: Object.keys(rows).length, importerEdges: Object.values(rows).reduce((n, r) => n + r.importers.length, 0) }));
writeFileSync(outPath!, JSON.stringify({ summary, collisions, byPackage: out }, null, 1));
console.log(summary, "collisions", Object.keys(collisions).length);
