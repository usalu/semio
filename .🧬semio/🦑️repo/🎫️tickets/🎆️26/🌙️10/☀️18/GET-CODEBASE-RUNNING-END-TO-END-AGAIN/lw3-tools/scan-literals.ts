import { readdirSync, readFileSync, statSync } from "node:fs";
import { dirname, join, resolve, relative } from "node:path";
const ROOT = "/Users/ueli/Documents/semio/";
const PL = ROOT + "✏️s/🔌️plugins/";
const skip = new Set(["node_modules", "target", "dist", ".git", "pkg", "🗑️generated"]);
const files: string[] = [];
const walk = (d: string) => { for (const e of readdirSync(d, { withFileTypes: true })) { if (skip.has(e.name)) continue; const p = join(d, e.name); if (e.isDirectory()) walk(p); else if (/\.(tsx?|mts|cts|js|mjs|cjs|json|jsonc|rs|toml)$/u.test(e.name) && !/(^|\/)(package|tsconfig)\.json$/u.test(p)) files.push(p); } };
for (const r of process.argv.slice(2)) walk(ROOT + r);
const plugin = (p: string) => p.startsWith(PL) ? p.slice(PL.length).split("/")[0]! : null;
const hits: Record<string, string[]> = {};
const lit = /["'`]((?:\.\.?\/)[^"'`\n]*|[^"'`\n]*✏️s\/🔌️plugins\/[^"'`\n]*)["'`]/gu;
for (const f of files) {
  const from = plugin(f); const src = readFileSync(f, "utf8");
  for (const m of src.matchAll(lit)) {
    const s = m[1]!;
    let target: string;
    if (s.includes("✏️s/🔌️plugins/")) target = ROOT + s.slice(s.indexOf("✏️s/🔌️plugins/"));
    else target = resolve(dirname(f), s);
    const to = plugin(target);
    if (to && to !== from) (hits[`${from ?? relative(ROOT, f).split("/").slice(0, 2).join("/")} -> ${to}`] ??= []).push(`${relative(ROOT, f)} :: ${s.slice(0, 160)}`);
  }
}
for (const [k, v] of Object.entries(hits).sort()) { console.log(v.length, k); for (const x of v.slice(0, 4)) console.log("   ", x); }
console.log("scanned", files.length);
