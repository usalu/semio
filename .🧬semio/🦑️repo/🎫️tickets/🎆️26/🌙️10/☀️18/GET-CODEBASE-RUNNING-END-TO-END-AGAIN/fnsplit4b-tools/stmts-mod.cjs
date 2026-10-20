// usage: node stmts-mod.cjs <file> <describeLine>: depth-1 statements of a module-level describe callback
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const fs = require("fs");
const [file, dl] = process.argv.slice(2);
const text = fs.readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
const st = sf.statements.find((s) => L(s.getStart(sf)) === +dl);
let call = st.expression; if (ts.isAwaitExpression(call)) call = call.expression;
for (const s of call.arguments[call.arguments.length - 1].body.statements) console.log(`${L(s.getStart(sf))}-${L(s.getEnd())}\t${L(s.getEnd()) - L(s.getStart(sf)) + 1}\t${ts.SyntaxKind[s.kind].replace("Declaration", "")}\t${s.getText(sf).split("\n")[0].slice(0, 110)}`);
