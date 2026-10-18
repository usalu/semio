// usage: node inner.cjs <file> <fn> <stmt-start-line>: depth-2 statements of the describe(...) call statement that starts at that line
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const fs = require("fs");
const [file, name, line] = process.argv.slice(2);
const text = fs.readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
const fn = sf.statements.find((s) => ts.isFunctionDeclaration(s) && s.name && s.name.text === name);
const st = fn.body.statements.find((s) => L(s.getStart(sf, true)) === +line || L(s.getStart(sf)) === +line);
const call = st.expression;
const cb = call.arguments[call.arguments.length - 1];
for (const s of cb.body.statements) {
  const a = L(s.getStart(sf, true)), b = L(s.getEnd());
  console.log(`${a}-${b}\t${b - a + 1}\t${ts.SyntaxKind[s.kind].replace("Declaration", "")}\t${s.getText(sf).split("\n")[0].slice(0, 130)}`);
}
