// usage: node stmts.cjs <file> <function> [describeLine]: statement start lines exactly as ksplit/psplit see them (getStart without comments)
const ts = require(process.env.TYPESCRIPT_PATH || "/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const fs = require("fs");
const [file, name, dl] = process.argv.slice(2);
const text = fs.readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
let fn = sf.statements.find((s) => ts.isFunctionDeclaration(s) && s.name && s.name.text === name);
if (!fn) { const vs = sf.statements.find((s) => ts.isVariableStatement(s) && s.declarationList.declarations.some((d) => ts.isIdentifier(d.name) && d.name.text === name)); const d = vs && vs.declarationList.declarations.find((x) => ts.isIdentifier(x.name) && x.name.text === name); fn = d && d.initializer && ts.isArrowFunction(d.initializer) ? d.initializer : null; }
let list = fn.body.statements;
if (dl) {
  const st = list.find((s) => L(s.getStart(sf)) === +dl);
  const call = st.expression;
  list = call.arguments[call.arguments.length - 1].body.statements;
}
for (const s of list) console.log(`${L(s.getStart(sf))}-${L(s.getEnd())}\t${L(s.getEnd()) - L(s.getStart(sf)) + 1}\t${ts.SyntaxKind[s.kind].replace("Declaration", "")}\t${s.getText(sf).split("\n")[0].slice(0, 110)}`);
