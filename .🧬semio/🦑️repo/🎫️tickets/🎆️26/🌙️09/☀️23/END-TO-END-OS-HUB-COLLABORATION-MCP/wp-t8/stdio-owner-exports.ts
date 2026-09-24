import { readFileSync } from "node:fs";
import ts from "typescript";
const root = "/Users/ueli/Documents/semio/";
const fixture = JSON.parse(readFileSync(root + "✏️s/🔌️plugins/🗄️stdio/🧫️fixtures/🏃️command-ownership/🔣️.json", "utf8"));
for (const owner of fixture.owners) {
  const source = ts.createSourceFile(owner.path, readFileSync(root + owner.path, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const names: string[] = [];
  for (const s of source.statements) {
    if (!(ts.canHaveModifiers(s) ? ts.getModifiers(s) : undefined)?.some((m) => m.kind === ts.SyntaxKind.ExportKeyword)) continue;
    if ((ts.isFunctionDeclaration(s) || ts.isClassDeclaration(s) || ts.isInterfaceDeclaration(s) || ts.isTypeAliasDeclaration(s)) && s.name) names.push(s.name.text);
    if (ts.isVariableStatement(s)) for (const d of s.declarationList.declarations) if (ts.isIdentifier(d.name)) names.push(d.name.text);
  }
  const extra = names.filter((n) => !owner.exports.includes(n)), missing = owner.exports.filter((n: string) => !names.includes(n));
  if (extra.length || missing.length) console.log(owner.id, { extra, missing });
}
