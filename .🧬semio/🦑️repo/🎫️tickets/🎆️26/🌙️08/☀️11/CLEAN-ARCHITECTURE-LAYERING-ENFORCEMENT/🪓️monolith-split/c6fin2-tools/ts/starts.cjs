// usage: node starts.cjs <file> [describeLine]: gsplit-style statement start lines (JSDoc inclusive) with end line and names; region marker lines
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const fs = require("fs");
const file = process.argv[2]; const dl = process.argv[3] ? +process.argv[3] : 0;
const text = fs.readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
let stmts = sf.statements;
if (dl) { const d = stmts.find((s) => L(s.getStart(sf, true)) === dl || L(s.getStart(sf)) === dl); stmts = d.expression.arguments[d.expression.arguments.length - 1].body.statements; }
const rows = stmts.map((s) => ({ a: L(s.getStart(sf, true)), b: L(s.getEnd()), t: s.getText(sf).split("\n")[0].slice(0, 80) }));
const regs = text.split("\n").map((l, i) => (/^\s*\/\/\s*#(end)?region/.test(l) ? [i + 1, l.trim()] : null)).filter(Boolean);
const all = [...rows.map((r) => [r.a, `${r.a}-${r.b}\t${r.t}`]), ...regs.map(([n, l]) => [n - 0.5, `   ${n}\t${l}`])].sort((x, y) => x[0] - y[0]);
for (const [, l] of all) console.log(l);
