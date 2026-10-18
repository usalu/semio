// usage: node verify-seq.cjs <spec.json> <original.ts> : proves the split root expands (phase by phase) to exactly the original depth-1 statement sequence
const fs = require("fs");
const path = require("path");
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const spec = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const origText = fs.readFileSync(process.argv[3], "utf8");
const parse = (f, t) => ts.createSourceFile(f, t ?? fs.readFileSync(f, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const fnOf = (sf, name) => sf.statements.find((s) => ts.isFunctionDeclaration(s) && s.name && s.name.text === name);
const osf = parse(spec.file, origText);
const ofn = fnOf(osf, spec.function);
const renames = new Map((spec.helpers?.items ?? []).map((h) => [h.line, h.as]));
const L = (sf, p) => sf.getLineAndCharacterOfPosition(p).line + 1;
const renameMap = new Map();
for (const s of ofn.body.statements) {
  const a = L(osf, s.getStart(osf));
  if (renames.has(a)) for (const d of s.declarationList.declarations) renameMap.set(d.name.text, renames.get(a));
}
const norm = (sf, node, ren) => {
  const edits = [];
  const visit = (n) => { if (ts.isIdentifier(n) && ren.has(n.text)) { const p = n.parent; if (!(ts.isPropertyAccessExpression(p) && p.name === n) && !(ts.isPropertyAssignment(p) && p.name === n)) edits.push({ s: n.getStart(sf) - node.getStart(sf), e: n.getEnd() - node.getStart(sf), t: ren.get(n.text) }); } ts.forEachChild(n, visit); };
  visit(node);
  let t = node.getText(sf);
  for (const e of edits.sort((x, y) => y.s - x.s)) t = t.slice(0, e.s) + e.t + t.slice(e.e);
  return t;
};
const origStmts = [];
const origHelpers = [];
for (const s of ofn.body.statements) {
  const a = L(osf, s.getStart(osf));
  if (renames.has(a)) origHelpers.push(norm(osf, s, renameMap));
  else origStmts.push(norm(osf, s, renameMap));
}
const rootSf = parse(spec.file);
const rfn = fnOf(rootSf, spec.function);
const childPath = (dir) => path.join(spec.childRoot || path.dirname(spec.file), dir, "🟦️.ts");
const phaseFile = new Map();
for (const f of spec.files) for (const p of f.phases) phaseFile.set(p.name, childPath(f.dir));
const calleeOf = (s) => {
  let e = null;
  if (ts.isExpressionStatement(s)) e = s.expression;
  else if (ts.isVariableStatement(s) && s.declarationList.declarations.length === 1) e = s.declarationList.declarations[0].initializer;
  if (e && ts.isAwaitExpression(e)) e = e.expression;
  if (e && ts.isCallExpression(e) && ts.isIdentifier(e.expression) && phaseFile.has(e.expression.text)) return e.expression.text;
  return null;
};
const expanded = [];
const trace = [];
for (const s of rfn.body.statements) {
  const callee = calleeOf(s);
  if (!callee) { expanded.push(norm(rootSf, s, new Map())); trace.push("root:" + s.getText(rootSf).slice(0, 30)); continue; }
  const csf = parse(phaseFile.get(callee));
  const cfn = fnOf(csf, callee);
  const body = [...cfn.body.statements];
  const last = body[body.length - 1];
  if (last && ts.isReturnStatement(last)) body.pop();
  for (const b of body) expanded.push(norm(csf, b, new Map()));
  trace.push(`${callee}(${body.length})`);
}
let bad = 0;
if (expanded.length !== origStmts.length) { console.log(`COUNT MISMATCH expanded=${expanded.length} original=${origStmts.length}`); bad++; }
for (let i = 0; i < Math.min(expanded.length, origStmts.length); i++) if (expanded[i] !== origStmts[i]) { console.log(`DIFF at statement ${i}:\n--- orig\n${origStmts[i].slice(0, 300)}\n--- new\n${expanded[i].slice(0, 300)}`); bad++; if (bad > 5) break; }
const helperSf = parse(childPath(spec.helpers.dir));
const helperNew = helperSf.statements.filter(ts.isVariableStatement).map((s) => {
  const t = s.getText(helperSf).replace(/^export /, "");
  return t.split("\n").map((l) => l).join("\n");
});
const dedent = (t) => t.split("\n").map((l, i) => (i === 0 ? l : l.startsWith("  ") ? l.slice(2) : l)).join("\n");
origHelpers.forEach((h, i) => {
  const want = dedent(h);
  const got = helperNew[i];
  if (want !== got) { console.log(`HELPER DIFF ${i}\n--- want\n${want.slice(0, 200)}\n--- got\n${(got ?? "").slice(0, 200)}`); bad++; }
});
console.log(`statements original=${ofn.body.statements.length} (helpers ${origHelpers.length}) expanded=${expanded.length} phases=${trace.filter((t) => !t.startsWith("root:")).length} root-kept=${trace.filter((t) => t.startsWith("root:")).length} ${bad ? "FAIL" : "LOSSLESS OK"}`);
process.exit(bad ? 1 : 0);
