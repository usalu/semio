// usage: node body.cjs <file> [describeLine] : list statements of a describe body (or top level) with start-end lines and first-line text
const ts = require("/Users/ueli/Documents/semio/node_modules/typescript");
const fs = require("fs");
const file = process.argv[2]; const dl = process.argv[3] ? +process.argv[3] : 0;
const text = fs.readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
const lead = (n) => { const f = n.getFullStart(); const m = text.slice(f, n.getStart(sf)).match(/^\s*/)[0].length; return L(f + m); };
let stmts = sf.statements;
if (dl) { const d = stmts.find((s) => lead(s) === dl || L(s.getStart(sf)) === dl); const c = d.expression.arguments[d.expression.arguments.length - 1]; stmts = c.body.statements; }
for (const s of stmts) console.log(`${lead(s)}-${L(s.getEnd())}\t${L(s.getEnd()) - lead(s) + 1}\t${s.getText(sf).split("\n")[0].slice(0, 110)}`);
