import ts from "typescript";
import { readFileSync } from "node:fs";
import { join, dirname, resolve, relative } from "node:path";
import { scan } from "./scan.ts";
const ROOT = "/Users/ueli/Documents/semio";
const edges = scan(join(ROOT, process.argv[2] ?? "✏️s/🔌️plugins")).filter(e => !e.ok && !e.target.includes("💻️os"));
const byFile = new Map<string, typeof edges>();
for (const e of edges) { if (!byFile.has(e.from)) byFile.set(e.from, []); byFile.get(e.from)!.push(e); }
const out: any[] = [];
for (const [f, es] of byFile) {
  const txt = readFileSync(join(ROOT, f), "utf8");
  const sf = ts.createSourceFile(f, txt, ts.ScriptTarget.Latest, true);
  const specs = new Set(es.map(e => e.spec));
  const visit = (n: ts.Node) => {
    let spec: string | undefined; let names: string[] = []; let kind = "";
    if (ts.isImportDeclaration(n) && ts.isStringLiteral(n.moduleSpecifier)) {
      spec = n.moduleSpecifier.text; kind = "import";
      const c = n.importClause;
      if (c) {
        if (c.name) names.push("default:" + c.name.text);
        if (c.namedBindings) { if (ts.isNamespaceImport(c.namedBindings)) names.push("*:" + c.namedBindings.name.text); else for (const el of c.namedBindings.elements) names.push((el.isTypeOnly || c.isTypeOnly ? "type " : "") + (el.propertyName?.text ?? el.name.text)); }
      }
    } else if (ts.isExportDeclaration(n) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier)) {
      spec = n.moduleSpecifier.text; kind = "export";
      if (n.exportClause && ts.isNamedExports(n.exportClause)) names = n.exportClause.elements.map(el => el.propertyName?.text ?? el.name.text); else names = ["*"];
    } else if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0] && ts.isStringLiteral(n.arguments[0])) {
      spec = n.arguments[0].text; kind = "dynamic"; names = ["?"];
    }
    if (spec && specs.has(spec)) { const e = es.find(x => x.spec === spec)!; out.push({ from: f, spec, target: e.target, kind, names, bare: e.kind === "bare", pkg: e.pkg }); }
    ts.forEachChild(n, visit);
  };
  visit(sf);
}
console.log(JSON.stringify(out));
