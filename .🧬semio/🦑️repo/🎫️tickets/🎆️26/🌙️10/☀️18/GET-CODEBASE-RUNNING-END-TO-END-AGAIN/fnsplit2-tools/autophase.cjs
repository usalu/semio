// usage: node autophase.cjs <file> <function> [maxLines=170]: proposes contiguous statement-run phases of about maxLines each (cuts only at statement starts; the final return stays) with the first and last statement heads
const ts = require(process.env.TYPESCRIPT_PATH || "/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const fs = require("fs");
const [file, name, max = "170"] = process.argv.slice(2);
const text = fs.readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
let fn = sf.statements.find((s) => ts.isFunctionDeclaration(s) && s.name && s.name.text === name);
if (!fn) { const vs = sf.statements.find((s) => ts.isVariableStatement(s) && s.declarationList.declarations.some((d) => ts.isIdentifier(d.name) && d.name.text === name)); const d = vs && vs.declarationList.declarations.find((x) => ts.isIdentifier(x.name) && x.name.text === name); fn = d && d.initializer && ts.isArrowFunction(d.initializer) ? d.initializer : null; }
const st = [...fn.body.statements];
const last = st[st.length - 1];
const body = ts.isReturnStatement(last) ? st.slice(0, -1) : st;
const rows = body.map((s) => ({ a: L(s.getStart(sf)), b: L(s.getEnd()), head: s.getText(sf).split("\n")[0].slice(0, 70) }));
const phases = [];
let cur = [];
let size = 0;
for (const r of rows) {
  const len = r.b - r.a + 1;
  if (cur.length && size + len > +max) { phases.push(cur); cur = []; size = 0; }
  cur.push(r); size += len;
}
if (cur.length) phases.push(cur);
phases.forEach((p, i) => console.log(`phase ${i + 1}: from ${p[0].a} to ${p[p.length - 1].a} (${p[p.length - 1].b - p[0].a + 1} lines, ${p.length} stmts)  first: ${p[0].head}  | last: ${p[p.length - 1].head}`));
