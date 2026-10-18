// usage: node fnbody.cjs <file> <function-start-line>: depth-1 statements of a function declaration body (start incl. JSDoc, end, kind, head)
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const fs = require("fs");
const [file, fl] = [process.argv[2], +process.argv[3]];
const text = fs.readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
const fn = sf.statements.find((s) => ts.isFunctionDeclaration(s) && L(s.getStart(sf)) <= fl && L(s.getEnd()) >= fl);
const stmts = fn.body.statements;
for (const s of stmts) {
  const a = L(s.getStart(sf, true)), b = L(s.getEnd());
  console.log(`${a}-${b}\t${b - a + 1}\t${ts.SyntaxKind[s.kind].replace("Declaration", "")}\t${s.getText(sf).split("\n")[0].slice(0, 110)}`);
}
