/** 🔗 Lists relative static imports of .ts/.mjs files under the given roots that resolve to no file. Usage: bun l22-broken-imports.mjs <root>... */
import { readdirSync, readFileSync, existsSync } from "node:fs";
import { join, dirname, resolve } from "node:path";
const skip = new Set(["node_modules", ".git", ".cache", "🗑️generated", "target", "dist", "pkg"]);
const files = [];
const walk = (d) => { for (const e of readdirSync(d, { withFileTypes: true })) { if (skip.has(e.name) || e.name.startsWith(".")) continue; const p = join(d, e.name); e.isDirectory() ? walk(p) : /\.(m?ts|mjs|tsx)$/.test(e.name) && files.push(p); } };
for (const r of process.argv.slice(2)) walk(resolve(r));
const re = /(?:import|export)\s[^"'`;]*?from\s*["'](\.{1,2}\/[^"']+)["']|import\(\s*["'](\.{1,2}\/[^"']+)["']\s*\)/g;
for (const f of files) {
  const src = readFileSync(f, "utf8");
  for (const m of src.matchAll(re)) { const s = m[1] ?? m[2]; const b = resolve(dirname(f), s.replace(/\?.*$/, "")); if (![b, ...[".ts", ".tsx", ".mjs", ".js", "/🟦️.ts", "/🟨️.mjs", "/index.ts"].map((x) => b + x), b.replace(/\.js$/, ".ts")].some(existsSync)) console.log(`${f}\t${s}`); }
}
