// usage: node lossless3.mjs <original.txt> <out-prefix> <new-file>...
// Line multiset of the original vs the union of the new files (trimmed, non-empty; import declarations removed from both; docstring-only lines ignored on the new side).
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import fs from "node:fs";
const [orig, outPrefix, ...news] = process.argv.slice(2);
const strip = (file, text) => {
  const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, file.endsWith("x") || file.includes(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  let out = "", pos = 0;
  for (const s of sf.statements) if (ts.isImportDeclaration(s)) { out += text.slice(pos, s.getStart(sf)); pos = s.end; }
  return out + text.slice(pos);
};
const norm = (t) => t.split("\n").map((l) => l.trim()).filter(Boolean);
const origLines = norm(strip(orig.replace(/\.txt$/, ""), fs.readFileSync(orig, "utf8")));
const newLines = news.flatMap((f) => norm(strip(f, fs.readFileSync(f, "utf8"))));
const count = (arr) => { const m = new Map(); for (const l of arr) m.set(l, (m.get(l) ?? 0) + 1); return m; };
const a = count(origLines), b = count(newLines);
const missing = [], extra = [];
for (const [l, n] of a) { const k = b.get(l) ?? 0; if (k < n) missing.push([l, n - k]); }
for (const [l, n] of b) { const k = a.get(l) ?? 0; if (k < n) extra.push([l, n - k]); }
const sum = (xs) => xs.reduce((s, [, n]) => s + n, 0);
const report = [`original non-import lines=${origLines.length} new non-import lines=${newLines.length}`, `MISSING from new: ${sum(missing)} lines, ${missing.length} distinct`, ...missing.map(([l, n]) => `  -${n} ${l.slice(0, 200)}`), `EXTRA in new: ${sum(extra)} lines, ${extra.length} distinct`, ...extra.map(([l, n]) => `  +${n} ${l.slice(0, 200)}`)].join("\n");
fs.writeFileSync(outPrefix + ".txt", report + "\n");
console.log(report.split("\n").slice(0, 3).join("\n"));
