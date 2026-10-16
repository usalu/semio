import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import { readFileSync } from "node:fs";
const file = process.argv[2];
const text = readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true);
const lines = text.split("\n");
const regions = [];
lines.forEach((l, i) => { const m = l.match(/^\s*\/\/ ?#region (.*)$/); if (m) regions.push({ name: m[1], start: i + 1 }); const e = l.match(/^\s*\/\/ ?#endregion (.*)$/); if (e) { const r = [...regions].reverse().find((x) => x.name === e[1] && !x.end); if (r) r.end = i + 1; } });
const regionOf = (line) => { let best = null; for (const r of regions) if (r.end && line >= r.start && line <= r.end && (!best || r.start > best.start)) best = r; return best?.name ?? "-"; };
const tops = [];
for (const st of sf.statements) {
  const start = sf.getLineAndCharacterOfPosition(st.getStart()).line + 1, end = sf.getLineAndCharacterOfPosition(st.end).line + 1;
  const names = [];
  if (ts.isVariableStatement(st)) for (const d of st.declarationList.declarations) { if (ts.isIdentifier(d.name)) names.push(d.name.text); else d.name.elements?.forEach((e) => ts.isIdentifier(e.name) && names.push(e.name.text)); }
  else if ((ts.isFunctionDeclaration(st) || ts.isClassDeclaration(st) || ts.isInterfaceDeclaration(st) || ts.isTypeAliasDeclaration(st) || ts.isEnumDeclaration(st)) && st.name) names.push(st.name.text);
  const exported = !!st.modifiers?.some((m) => m.kind === ts.SyntaxKind.ExportKeyword);
  tops.push({ start, end, kind: ts.SyntaxKind[st.kind], names, exported, region: regionOf(start) });
}
const byName = new Map();
for (const t of tops) for (const n of t.names) byName.set(n, t);
const stat = {};
for (const t of tops) { const r = (stat[t.region] ??= { lines: 0, stmts: 0, names: 0, exported: 0, start: Infinity }); r.lines += t.end - t.start + 1; r.stmts++; r.names += t.names.length; r.exported += t.exported ? t.names.length : 0; r.start = Math.min(r.start, t.start); }
// references: identifiers in each statement referencing top-level names of other regions
const edges = {};
const visit = (node, t) => { if (ts.isIdentifier(node)) { const d = byName.get(node.text); if (d && d.region !== t.region) { (edges[t.region] ??= {})[d.region] ??= new Set(); edges[t.region][d.region].add(node.text); } } ts.forEachChild(node, (c) => visit(c, t)); };
for (const st of sf.statements) { const start = sf.getLineAndCharacterOfPosition(st.getStart()).line + 1; const t = tops.find((x) => x.start === start); if (t) visit(st, t); }
console.log("REGIONS");
for (const [r, s] of Object.entries(stat).sort((a, b) => a[1].start - b[1].start)) console.log(`${r}\tlines=${s.lines}\tstmts=${s.stmts}\tnames=${s.names}\texported=${s.exported}\tstart=${s.start}`);
console.log("EDGES (from -> to : count)");
for (const [f, tos] of Object.entries(edges)) for (const [to, names] of Object.entries(tos)) console.log(`${f} -> ${to} : ${names.size} ${[...names].slice(0, 6).join(",")}`);
