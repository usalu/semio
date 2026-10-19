// usage: node stmtmove.cjs <file> <functionName> <newOrder: comma list of old statement indexes (0-based) or "a-b" ranges> [APPLY]
// Reorders the depth-1 statements of a function (leading comments travel with their statement).
const fs = require("fs");
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const [file, fname, orderArg, mode] = process.argv.slice(2);
const text = fs.readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
let fn = null;
const find = (n) => { if (fn) return; if (ts.isFunctionDeclaration(n) && n.name && n.name.text === fname) { fn = n; return; } ts.forEachChild(n, find); };
find(sf);
const stmts = [...fn.body.statements];
const order = orderArg.split(",").flatMap((x) => { const m = /^(\d+)-(\d+)$/.exec(x); return m ? Array.from({ length: +m[2] - +m[1] + 1 }, (_, i) => +m[1] + i) : [+x]; });
if (order.length !== stmts.length || new Set(order).size !== stmts.length) throw new Error(`order must be a permutation of 0..${stmts.length - 1}`);
const startOf = (i) => { const lead = ts.getLeadingCommentRanges(text, stmts[i].getFullStart()) || []; const first = lead.length ? lead[0].pos : stmts[i].getStart(sf); return text.lastIndexOf("\n", first) + 1; };
const chunks = stmts.map((s, i) => text.slice(startOf(i), s.getEnd()));
const prefixEnd = startOf(0), suffixStart = stmts[stmts.length - 1].getEnd();
const out = text.slice(0, prefixEnd) + order.map((i) => chunks[i]).join("\n") + text.slice(suffixStart);
if (mode === "APPLY") { fs.writeFileSync(file, out); console.log("APPLIED"); } else { process.stdout.write(out); }
