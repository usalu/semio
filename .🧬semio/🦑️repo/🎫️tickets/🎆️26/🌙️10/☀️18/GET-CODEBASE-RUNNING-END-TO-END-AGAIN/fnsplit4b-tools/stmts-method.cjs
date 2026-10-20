// usage: node stmts-method.cjs <file> <methodName>: depth-1 statements of a class method body
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const fs = require("fs");
const [file, name] = process.argv.slice(2);
const text = fs.readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
let m = null;
const visit = (n) => { if (ts.isMethodDeclaration(n) && n.name.getText(sf) === name && !m) m = n; ts.forEachChild(n, visit); };
visit(sf);
for (const s of m.body.statements) console.log(`${L(s.getStart(sf))}-${L(s.getEnd())}\t${L(s.getEnd()) - L(s.getStart(sf)) + 1}\t${ts.SyntaxKind[s.kind].replace("Declaration", "")}\t${s.getText(sf).split("\n")[0].slice(0, 110)}`);
