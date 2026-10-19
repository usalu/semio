import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
const SKIP = new Set(["node_modules", "dist", "target", "pkg", ".cache", "🗑️generated", ".git", "📤️dist"]);
const syms = new Map<string, number>();
const walk = (d: string) => { let es; try { es = readdirSync(d, { withFileTypes: true }); } catch { return; } for (const e of es) { if (SKIP.has(e.name)) continue; const p = join(d, e.name); if (e.isDirectory()) walk(p); else if (e.name === "🛰️.proto") { const t = readFileSync(p, "utf8"); for (const m of t.matchAll(/semio\.os\.store\.[A-Za-z_.]+/g)) syms.set(m[0], (syms.get(m[0]) ?? 0) + 1); } } };
for (const r of ["✏️s", "🧰️framework", "🌎️hub"]) walk(join("/Users/ueli/Documents/semio", r));
console.log([...syms].sort((a, b) => b[1] - a[1]).map(([k, v]) => v + " " + k).join("\n"));
