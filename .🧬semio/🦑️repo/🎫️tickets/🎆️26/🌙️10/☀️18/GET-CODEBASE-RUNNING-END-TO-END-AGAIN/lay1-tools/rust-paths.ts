import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
const root = "/Users/ueli/Documents/semio";
const skip = new Set(["node_modules", "target", "pkg", "dist", "📤️dist", ".git", "🗑️generated", ".cache", "bazel-out", "temp", "🎫️tickets"]);
let checked = 0; const bad: string[] = [];
const walk = (d: string) => {
  let es; try { es = readdirSync(d, { withFileTypes: true }); } catch { return; }
  for (const e of es) {
    if (skip.has(e.name)) continue;
    const p = join(d, e.name);
    if (e.isDirectory()) walk(p);
    else if (e.name.endsWith(".rs")) {
      const t = readFileSync(p, "utf8");
      if (!t.includes("path") && !t.includes("include")) continue;
      for (const m of t.matchAll(/#\[path\s*=\s*"([^"]+)"\s*\]|include(?:_str|_bytes)?!\(\s*"([^"]+)"\s*\)|fixture_text!\(\s*"([^"]+)"\s*\)/gu)) {
        const rel = m[1] ?? m[2] ?? m[3]!;
        if (rel.includes("OUT_DIR") || rel.startsWith("$")) continue;
        checked++;
        const target = resolve(dirname(p), rel);
        if (!existsSync(target)) bad.push(`${p.slice(root.length + 1)} -> ${rel}`);
      }
    }
  }
};
for (const r of ["✏️s", "🧰️framework", "🌎️hub", "♻️mit-bestand", "🎓️teaching"]) walk(join(root, r));
import { writeFileSync } from "node:fs";
const out = process.argv[2];
if (out) writeFileSync(out, JSON.stringify(bad.sort(), null, 1));
console.log(`rust path/include literals checked ${checked}, missing ${bad.length}`);
