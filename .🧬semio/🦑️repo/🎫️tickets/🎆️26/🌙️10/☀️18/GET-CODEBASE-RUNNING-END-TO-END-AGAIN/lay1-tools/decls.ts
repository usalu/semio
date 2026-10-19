import ts from "typescript";
import { readFileSync } from "node:fs";
for (const f of process.argv.slice(2)) {
  const sf = ts.createSourceFile(f, readFileSync(f, "utf8"), ts.ScriptTarget.Latest, true, f.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  console.log("##", f.split("/").slice(-3).join("/"), sf.statements.length);
  for (const s of sf.statements) {
    if (ts.isImportDeclaration(s)) continue;
    let names: string[] = [];
    if (ts.isVariableStatement(s)) names = s.declarationList.declarations.map((d) => d.name.getText());
    else if ((s as any).name?.getText) names = [(s as any).name.getText()];
    const exp = ts.canHaveModifiers(s) && ts.getModifiers(s)?.some((m) => m.kind === ts.SyntaxKind.ExportKeyword) ? "E" : " ";
    const lines = sf.getLineAndCharacterOfPosition(s.getEnd()).line - sf.getLineAndCharacterOfPosition(s.getStart()).line + 1;
    console.log(`${exp} ${ts.SyntaxKind[s.kind].replace("Declaration","")} ${names.join(",") || s.getText().slice(0, 50).replace(/\n/g, " ")} (${lines}l) @${sf.getLineAndCharacterOfPosition(s.getStart()).line + 1}`);
  }
}
