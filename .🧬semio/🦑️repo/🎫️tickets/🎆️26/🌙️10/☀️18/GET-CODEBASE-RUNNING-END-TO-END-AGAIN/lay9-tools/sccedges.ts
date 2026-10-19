import ts from "typescript";
import { readFileSync } from "node:fs";
const q = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const idx = Number(process.argv[3] ?? 0);
const scc = new Set<string>(q.sccs[idx]);
const P = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/";
const s = (p: string) => p.replace(P, "").replace("🧑‍🎨engine/🧱️elements/", "").replace(/\/🟦️\.(tsx?)$/, "/#");
const byFrom = new Map<string, any[]>();
for (const e of q.edges) if (scc.has(e.from) && scc.has(e.to)) byFrom.set(e.from, [...(byFrom.get(e.from) ?? []), e]);
for (const [from, es] of byFrom) {
  const sf = ts.createSourceFile(from, readFileSync(from, "utf8"), ts.ScriptTarget.Latest, true, from.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const syms = new Map<string, string[]>();
  const visit = (n: ts.Node): void => {
    let spec: string | undefined, label = "";
    if (ts.isImportDeclaration(n) && ts.isStringLiteral(n.moduleSpecifier)) { spec = n.moduleSpecifier.text; const c = n.importClause; const names: string[] = []; if (c?.name) names.push("default"); if (c?.namedBindings) { if (ts.isNamespaceImport(c.namedBindings)) names.push("*"); else for (const e of c.namedBindings.elements) names.push((c.isTypeOnly || e.isTypeOnly ? "t:" : "") + (e.propertyName ?? e.name).text); } label = names.join(","); }
    else if (ts.isExportDeclaration(n) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier)) { spec = n.moduleSpecifier.text; label = "export " + (n.exportClause && ts.isNamedExports(n.exportClause) ? n.exportClause.elements.map((e) => (n.isTypeOnly ? "t:" : "") + e.name.text).join(",") : "*"); }
    else if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal)) { spec = n.argument.literal.text; label = "import(type)." + (n.qualifier?.getText() ?? ""); }
    else if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0])) { spec = n.arguments[0].text; label = "dyn"; }
    if (spec) syms.set(spec, [...(syms.get(spec) ?? []), label]);
    ts.forEachChild(n, visit);
  };
  visit(sf);
  const { dirname, resolve } = require("node:path");
  for (const e of es) {
    const lab = [...syms].filter(([sp]) => sp.startsWith(".") && [ "", ".ts", ".tsx", "/🟦️.ts", "/🟦️.tsx"].some((x) => resolve(dirname(from), sp) + x === e.to)).map(([, l]) => l.join(" ; ")).join(" | ");
    console.log(e.typeOnly ? "T" : "R", s(from), "->", s(e.to), "::", lab);
  }
}
