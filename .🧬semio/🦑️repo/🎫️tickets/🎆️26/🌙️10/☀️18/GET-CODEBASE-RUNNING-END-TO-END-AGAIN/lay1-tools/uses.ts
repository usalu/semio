import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
const root = "/Users/ueli/Documents/semio";
const skip = new Set(["node_modules", "target", "pkg", "dist", "📤️dist", ".git", "🗑️generated", ".cache"]);
const roots = ["✏️s", "🧰️framework", "🌎️hub", "♻️mit-bestand", "🎓️teaching", ".🧬semio/🦑️repo/🔨️modules"];
const exts = /\.(ts|tsx|mts|cts|json|mjs|cjs|rs|go|md)$/u;
const words = process.argv.slice(2);
const re = new RegExp(`\\b(${words.join("|")})\\b`, "u");
const walk = (d: string) => {
  let es; try { es = readdirSync(d, { withFileTypes: true }); } catch { return; }
  for (const e of es) {
    if (skip.has(e.name)) continue;
    const p = join(d, e.name);
    if (e.isDirectory()) walk(p);
    else if (exts.test(e.name)) { try { const t = readFileSync(p, "utf8"); const m = re.exec(t); if (m) console.log(p.slice(root.length + 1), m[1]); } catch {} }
  }
};
for (const r of roots) walk(join(root, r));
