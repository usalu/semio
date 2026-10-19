// usage: node verify-text2.cjs <original.ts|tsx> <function> <treeDir> (all 🟦️.ts[x] files below treeDir) : every depth-1 statement of the function (and every depth-1 statement of every top-level describe callback) must occur verbatim (modulo indentation, relative-import depth and an `export ` prefix) in exactly one new file; module-level statements outside the function too
const fs = require("fs");
const ts = require(process.env.TYPESCRIPT_PATH || "/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const path = require("path");
const [origFile, fname, dirArg] = process.argv.slice(2);
const listFile = "/dev/null";
const text = fs.readFileSync(origFile, "utf8");
const sf = ts.createSourceFile(origFile, text, ts.ScriptTarget.Latest, true, origFile.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const norm = (t) => t.split("\n").map((l) => l.trim()).filter((l) => l).join("\n").replace(/(\.\.\/)+/g, "../").replace(/^export /gm, "").replace(/^export\s+(async\s+)?/gm, "$1");
const files = [];
const walk = (d) => { for (const e of fs.readdirSync(d, { withFileTypes: true })) { const p = path.join(d, e.name); if (e.isDirectory()) walk(p); else if (/🟦️\.tsx?$/.test(e.name)) files.push(p); } };
walk(dirArg.startsWith("/") ? dirArg : "/Users/ueli/Documents/semio/" + dirArg);
const bodies = files.map((f) => ({ f, t: norm(fs.readFileSync(f, "utf8")) }));
const fn = sf.statements.find((s) => ts.isFunctionDeclaration(s) && s.name?.text === fname);
const items = [];
for (const s of fn.body.statements) {
  items.push(s);
  if (ts.isExpressionStatement(s) && ts.isCallExpression(s.expression) && s.expression.expression.getText(sf) === "describe") {
    const cb = s.expression.arguments[s.expression.arguments.length - 1];
    if (cb && (ts.isArrowFunction(cb) || ts.isFunctionExpression(cb)) && ts.isBlock(cb.body)) for (const i of cb.body.statements) items.push(i);
  }
}
for (const s of sf.statements) if (s !== fn && !ts.isImportDeclaration(s)) items.push(s);
let bad = 0, total = 0;
for (const s of items) {
  const isDescribe = ts.isExpressionStatement(s) && ts.isCallExpression(s.expression) && /^describe$/.test(s.expression.expression.getText(sf));
  const t = norm(s.getText(sf));
  total++;
  if (isDescribe) {
    const cb = s.expression.arguments[s.expression.arguments.length - 1];
    const title = s.expression.arguments[0].getText(sf);
    const head = norm(`${s.expression.expression.getText(sf)}(${title}`);
    const hits = bodies.filter((b) => b.t.includes(head));
    if (hits.length !== 1) { bad++; console.log(`DESCRIBE HEAD hits=${hits.length}: ${head.slice(0, 80)}`); }
    continue;
  }
  const hits = bodies.filter((b) => b.t.includes(t));
  if (hits.length !== 1) { bad++; console.log(`HITS=${hits.length} line ${sf.getLineAndCharacterOfPosition(s.getStart(sf)).line + 1}: ${t.slice(0, 100).replace(/\n/g, " | ")}`); }
}
console.log(`statements checked=${total} problems=${bad} ${bad ? "FAIL" : "TEXT LOSSLESS OK"}`);
process.exit(bad ? 1 : 0);
