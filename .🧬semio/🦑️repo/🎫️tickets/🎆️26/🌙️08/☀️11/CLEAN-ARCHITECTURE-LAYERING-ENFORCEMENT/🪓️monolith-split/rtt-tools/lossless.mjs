import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import fs from "node:fs";
import path from "node:path";
import { pathToFileURL } from "node:url";
const [cfgPath, which] = process.argv.slice(2);
const cfg = (await import(pathToFileURL(path.resolve(cfgPath)).href)).default;
const strip = (file) => {
  const text = fs.readFileSync(file, "utf8");
  const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  let out = "";
  let pos = 0;
  for (const s of sf.statements) if (ts.isImportDeclaration(s)) { out += text.slice(pos, s.getStart()); pos = s.end; }
  out += text.slice(pos);
  return out;
};
const lines = (t) => t.split("\n").map((l) => l.trim()).filter((l) => l && !/^\/\*\*? .*\*\/$/.test(l) || false);
const norm = (t) => t.split("\n").map((l) => l.trim()).filter(Boolean);
const orig = norm(strip(cfg.source));
const dirs = [...cfg.topics.map((t) => t.folder), cfg.helperFolder, ...(cfg.registrar ? [cfg.registrar.folder] : [])];
const newFiles = dirs.map((d) => path.join(cfg.testsDir, d, `🟦️${cfg.ext}`));
const news = newFiles.flatMap((f) => norm(strip(f)));
const count = (arr) => { const m = new Map(); for (const l of arr) m.set(l, (m.get(l) ?? 0) + 1); return m; };
const a = count(orig), b = count(news);
const missing = [], extra = [];
for (const [l, n] of a) { const k = b.get(l) ?? 0; if (k < n) missing.push([l, n - k]); }
for (const [l, n] of b) { const k = a.get(l) ?? 0; if (k < n) extra.push([l, n - k]); }
console.log(`orig non-import lines=${orig.length} new=${news.length}`);
console.log(`MISSING from new (${missing.reduce((s, [, n]) => s + n, 0)} lines, ${missing.length} distinct):`);
for (const [l, n] of missing.slice(0, +(process.argv[4] ?? 60))) console.log(`  -${n} ${l.slice(0, 160)}`);
console.log(`EXTRA in new (${extra.reduce((s, [, n]) => s + n, 0)} lines, ${extra.length} distinct):`);
for (const [l, n] of extra.slice(0, +(process.argv[4] ?? 60))) console.log(`  +${n} ${l.slice(0, 160)}`);
