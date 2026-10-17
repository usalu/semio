// usage: node lossless.cjs <label> <orig> <new...>   (import declarations removed; leading `export ` ignored; regions counted separately)
const fs = require("fs");
const ts = require("/Users/ueli/Documents/semio/node_modules/typescript");
const [label, orig, ...news] = process.argv.slice(2);
function load(p) {
  const text = fs.readFileSync(p, "utf8");
  const sf = ts.createSourceFile(p, text, ts.ScriptTarget.Latest, true, p.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  let out = text;
  const cuts = sf.statements.filter((s) => ts.isImportDeclaration(s)).map((s) => [s.getStart(sf), s.getEnd()]).sort((a, b) => b[0] - a[0]);
  let imports = 0;
  for (const [a, b] of cuts) { out = out.slice(0, a) + out.slice(b); imports++; }
  const code = [], region = [];
  for (const raw of out.split("\n")) {
    const l = raw.trim();
    if (!l) continue;
    if (/^\/\/\s*#(end)?region/.test(l)) region.push(l.replace(/^\/\/\s*/, "// "));
    else code.push(l.replace(/^export\s+(default\s+)?/, ""));
  }
  return { code, region, imports };
}
const count = (a) => { const m = new Map(); for (const x of a) m.set(x, (m.get(x) || 0) + 1); return m; };
const diff = (a, b) => { const out = []; for (const [k, v] of a) { const w = b.get(k) || 0; if (v > w) out.push([k, v - w]); } return out; };
const o = load(orig);
const n = { code: [], region: [], imports: 0 };
for (const p of news) { const r = load(p); n.code.push(...r.code); n.region.push(...r.region); n.imports += r.imports; }
for (const k of ["code", "region"]) {
  const co = count(o[k]), cn = count(n[k]);
  const lost = diff(co, cn), gained = diff(cn, co);
  console.log(`${label} [${k}] orig=${o[k].length} new=${n[k].length} lost=${lost.reduce((s, x) => s + x[1], 0)} added=${gained.reduce((s, x) => s + x[1], 0)}`);
  for (const [l, c] of lost.slice(0, 40)) console.log("   LOST  ", c, l.slice(0, 170));
  for (const [l, c] of gained.slice(0, 40)) console.log("   ADDED ", c, l.slice(0, 170));
}
console.log(`${label} [imports] orig=${o.imports} new=${n.imports}`);
