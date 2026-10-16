// usage: node lossless2.mjs <original.txt> <report-out.txt> <new-file>...
// Line multiset original vs union of new files, ignoring import declarations, a leading `export ` / `pub(super) ` visibility change, and the leading docstring block of every file (original and new).
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import fs from "node:fs";
const [orig, outFile, ...news] = process.argv.slice(2);
const stripImports = (file, text) => {
  const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, /x(\.txt)?$/.test(file) ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  let out = "", pos = 0;
  for (const s of sf.statements) if (ts.isImportDeclaration(s) || (ts.isExportDeclaration(s) && s.moduleSpecifier)) { out += text.slice(pos, s.getFullStart()); pos = s.end; }
  return out + text.slice(pos);
};
const dropLeadDoc = (t) => {
  const lines = t.split("\n"); let i = 0;
  while (i < lines.length && lines[i].trim() === "") i++;
  if (lines[i]?.trim().startsWith("/**") ) { while (i < lines.length && !lines[i].includes("*/")) i++; return lines.slice(i + 1).join("\n"); }
  if (lines[i]?.trim().startsWith("// #region") && lines[i + 1]?.trim().startsWith("/**")) { i++; while (i < lines.length && !lines[i].includes("*/")) i++; return lines.slice(i + 1).join("\n"); }
  return t;
};
const norm = (t) => t.split("\n").map((l) => l.trim().replace(/^export (default )?/, "")).filter(Boolean);
const origText = fs.readFileSync(orig, "utf8");
const origLines = norm(stripImports(orig.replace(/\.txt$/, ""), dropLeadDoc(origText)));
const newLines = news.flatMap((f) => norm(stripImports(f, dropLeadDoc(fs.readFileSync(f, "utf8")))));
const count = (arr) => { const m = new Map(); for (const l of arr) m.set(l, (m.get(l) ?? 0) + 1); return m; };
const a = count(origLines), b = count(newLines);
const missing = [], extra = [];
for (const [l, n] of a) { const k = b.get(l) ?? 0; if (k < n) missing.push([l, n - k]); }
for (const [l, n] of b) { const k = a.get(l) ?? 0; if (k < n) extra.push([l, n - k]); }
const sum = (xs) => xs.reduce((s, [, n]) => s + n, 0);
const report = [`original lines=${origLines.length} new lines=${newLines.length} files=${news.length}`, `MISSING: ${sum(missing)}`, ...missing.map(([l, n]) => `  -${n} ${l.slice(0, 200)}`), `EXTRA: ${sum(extra)}`, ...extra.map(([l, n]) => `  +${n} ${l.slice(0, 200)}`)].join("\n");
fs.writeFileSync(outFile, report + "\n");
console.log(report.split("\n").slice(0, 3).join(" | "));
