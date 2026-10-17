const ts = require("/Users/ueli/Documents/semio/node_modules/typescript");
const fs = require("fs");
const file = process.argv[2];
const text = fs.readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
sf.statements.forEach((s, i) => {
  let n = "";
  if (ts.isVariableStatement(s)) n = s.declarationList.declarations.map((d) => d.name.getText(sf)).join(",");
  else if (s.name) n = s.name.getText(sf);
  else if (ts.isImportDeclaration(s)) { if (process.argv[3] !== "imports") return; n = s.getText(sf).slice(0, 80).replace(/\n/g, " "); }
  else n = s.getText(sf).slice(0, 60).replace(/\n/g, " ");
  const a = L(s.getStart(sf, true)), b = L(s.getEnd());
  console.log(`${i}\t${a}-${b}\t${b - a + 1}\t${ts.SyntaxKind[s.kind].replace("Declaration", "")}\t${n}`);
});
