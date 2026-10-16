import fs from "node:fs";
import path from "node:path";
import { analyze } from "./analyze.mjs";
const [cfgPath, orig, ...extra] = process.argv.slice(2);
const cfg = (await import(cfgPath)).default;
process.env.SPLIT_SOURCE = path.resolve(orig);
const a = analyze(cfg.file);
const P = path.dirname(cfg.file);
const files = [];
const walk = (d) => { for (const e of fs.readdirSync(d, { withFileTypes: true })) { if (e.isDirectory()) { if (!["🧪️tests", "🧫️fixtures", "node_modules", "📖️stories", "🎯️targets", "🧬️schema"].includes(e.name)) walk(path.join(d, e.name)); } else if (/^🟦️\.tsx?$/.test(e.name)) files.push(path.join(d, e.name)); } };
for (const m of cfg.modules) { const b = path.join(P, m.dir, "🟦️"); files.push(fs.existsSync(b + ".ts") ? b + ".ts" : b + ".tsx"); }
if (fs.existsSync(cfg.file)) files.push(cfg.file);
files.push(...extra);
const norm = (s) => s.replace(/[ \t]+$/gm, "").replace(/^[ \t]*\/\/\s*#(end)?region.*$/gm, "");
const texts = files.map((f) => norm(fs.readFileSync(f, "utf8")));
let ok = 0; const missing = [], dup = [];
for (const u of a.units) {
  if (u.exportDecl) continue;
  const t = norm(u.stmt.getText(a.sf)).replace(/^export /, "");
  const stripExport = (x) => x.replace(/(^|\n)export (?=(async function|function|const|let|class|interface|type|enum|abstract))/g, "$1");
  const n = texts.reduce((c, x) => c + stripExport(x).split(t).length - 1, 0);
  if (n === 1) ok++; else if (n === 0) missing.push(u.names.join(",") || "<stmt>@" + a.lineOf(u.start)); else dup.push(u.names.join(",") + "x" + n);
}
console.log("units", a.units.length, "non-export-decl", a.units.filter((u) => !u.exportDecl).length, "exactly-once", ok, "missing", missing, "dup", dup);
