import { readFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import { load } from "./graph.mjs";
const [origFile, childRoot, ...rest] = process.argv.slice(2);
const flags = new Set(rest);
const orig = readFileSync(origFile, "utf8");
const files = [];
const walk = (d) => { for (const e of readdirSync(d)) { const p = join(d, e); if (statSync(p).isDirectory()) walk(p); else if (/^🟦️\.tsx?$/u.test(e)) files.push(p); } };
if (childRoot.endsWith(".mjs")) { const cfg = (await import(childRoot)).default; for (const m of cfg.modules) { const f = m.isParent ? process.env.RTV_PARENT_FILE : join(process.env.RTV_PARENT_DIR, m.folder, "🟦️" + (cfg.ext ?? ".ts")); files.push(f); } } else walk(childRoot);
const norm = (l) => l.replace(/\s+$/u, "");
const multi = (arr) => { const m = new Map(); for (const l of arr) m.set(l, (m.get(l) ?? 0) + 1); return m; };
const origLines = orig.split("\n").map(norm).filter((l) => l.trim() !== "");
const childLines = files.flatMap((f) => readFileSync(f, "utf8").split("\n").map(norm).filter((l) => l.trim() !== ""));
const A = multi(origLines), B = multi(childLines);
const missing = [], extra = [];
for (const [l, n] of A) { const m = B.get(l) ?? 0; if (m < n) missing.push([l, n - m]); }
for (const [l, n] of B) { const m = A.get(l) ?? 0; if (m < n) extra.push([l, n - m]); }
const isImportLine = (l) => /^\s*import\b/u.test(l);
const isMarker = (l) => /^\s*\/\/ ?#(end)?region /u.test(l);
const missingNonImport = missing.filter(([l]) => !isImportLine(l) && !isMarker(l));
const extraNonImport = extra.filter(([l]) => !isImportLine(l) && !isMarker(l));
console.log(`original non-blank lines ${origLines.length}; children ${childLines.length} in ${files.length} files`);
console.log(`missing from children: ${missing.reduce((a, [, n]) => a + n, 0)} (imports ${missing.filter(([l]) => isImportLine(l)).reduce((a, [, n]) => a + n, 0)}, markers ${missing.filter(([l]) => isMarker(l)).reduce((a, [, n]) => a + n, 0)}, other ${missingNonImport.reduce((a, [, n]) => a + n, 0)})`);
for (const [l, n] of missingNonImport) console.log("  - ", n, l.slice(0, 200));
console.log(`extra in children: ${extra.reduce((a, [, n]) => a + n, 0)} (imports ${extra.filter(([l]) => isImportLine(l)).reduce((a, [, n]) => a + n, 0)}, markers ${extra.filter(([l]) => isMarker(l)).reduce((a, [, n]) => a + n, 0)}, other ${extraNonImport.reduce((a, [, n]) => a + n, 0)})`);
for (const [l, n] of extraNonImport) console.log("  + ", n, l.slice(0, 200));
if (flags.has("markers")) { for (const [l, n] of missing.filter(([l]) => isMarker(l))) console.log("  marker missing", n, l); for (const [l, n] of extra.filter(([l]) => isMarker(l))) console.log("  marker extra", n, l); }
// statement-level
const { tops } = load(origFile);
const strip = (s) => s.replace(/^(\s*(?:\/\*[\s\S]*?\*\/\s*|\/\/[^\n]*\n\s*)*)export (?=\S)/u, "$1");
const origStmts = tops.filter((t) => !t.isImport).map((t) => strip(orig.slice(t.spanPos, t.st.end)));
const childStmts = [];
for (const f of files) { const { tops: ct, text } = load(f); for (const t of ct) if (!t.isImport) childStmts.push(strip(text.slice(t.spanPos, t.st.end))); }
const SA = multi(origStmts), SB = multi(childStmts);
let bad = 0;
for (const [s, n] of SA) if ((SB.get(s) ?? 0) !== n) { bad++; console.log("STATEMENT MISMATCH orig", n, "child", SB.get(s) ?? 0, s.slice(0, 120).replace(/\n/g, "\\n")); }
for (const [s, n] of SB) if (!SA.has(s)) { bad++; console.log("STATEMENT EXTRA in child", s.slice(0, 120).replace(/\n/g, "\\n")); }
console.log(`statements: orig ${origStmts.length} children ${childStmts.length} mismatches ${bad}`);
