// usage: node findtop.cjs <file> <fnStartLine> : start line of the top-level statement that contains the function starting at that line (or "none")
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const fs = require("fs");
const [file, line] = process.argv.slice(2);
const sf = ts.createSourceFile(file, fs.readFileSync(file, "utf8"), ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
let hit = null;
const find = (n) => { if (hit) return; if (ts.isFunctionLike(n) && n.body && L(n.getStart(sf)) === +line) hit = n; else ts.forEachChild(n, find); };
find(sf);
if (!hit) { console.log("none"); process.exit(0); }
let p = hit; while (p.parent && p.parent !== sf) p = p.parent;
if (!ts.isExpressionStatement(p)) { console.log("none"); process.exit(0); }
const isDesc = (st) => ts.isExpressionStatement(st) && ts.isCallExpression(st.expression) && /^describe\b/.test(st.expression.expression.getText(sf));
const descs = sf.statements.filter(isDesc);
console.log(L(p.getStart(sf)) + " " + Math.max(0, descs.indexOf(p)));
