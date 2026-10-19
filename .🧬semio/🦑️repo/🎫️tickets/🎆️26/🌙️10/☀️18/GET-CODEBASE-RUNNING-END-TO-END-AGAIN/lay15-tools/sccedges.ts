import ts from "typescript";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
const R = "/Users/ueli/Documents/semio/";
const files = process.argv.slice(2).map(f => resolve(R, f));
const short = (f: string) => f.replace(R, "").replace(/[^\x00-\x7f]/g, "").split("/").slice(-4).join("/");
for (const f of files) {
  const sf = ts.createSourceFile(f, readFileSync(f, "utf8"), ts.ScriptTarget.Latest, true);
  const visit = (n: ts.Node): void => {
    let spec: string | undefined, names: string[] = [], tOnly = false;
    if (ts.isImportDeclaration(n) && ts.isStringLiteral(n.moduleSpecifier)) { spec = n.moduleSpecifier.text; tOnly = !!n.importClause?.isTypeOnly; const b = n.importClause?.namedBindings; if (b && ts.isNamedImports(b)) names = b.elements.map(e => (e.isTypeOnly ? "type " : "") + (e.propertyName ?? e.name).text); else if (n.importClause?.name) names = ["default"]; else if (b) names = ["*"]; }
    else if (ts.isExportDeclaration(n) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier)) { spec = n.moduleSpecifier.text; names = n.exportClause && ts.isNamedExports(n.exportClause) ? n.exportClause.elements.map(e => "re:" + (e.propertyName ?? e.name).text) : ["*"]; }
    else if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0])) { spec = n.arguments[0].text; names = ["dynamic"]; }
    if (spec && spec.startsWith(".")) {
      const t = resolve(dirname(f), spec);
      const hit = files.find(g => g === t || g === t + ".ts" || g.replace(/\.ts$/, "") === t);
      if (hit) console.log(short(f), "->", short(hit), (tOnly ? "TYPE " : "") + names.join(","));
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
}
