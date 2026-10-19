import { readFileSync, writeFileSync, readdirSync, statSync } from "node:fs";
import { join, relative } from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const SKIP = new Set(["node_modules", "dist", "target", "pkg", ".cache", "🗑️generated", ".git", "storybook-static", "📤️dist"]);
const SKIP_FILES = new Set(["package.json", "bun.lock", "📓️schema-catalog.md", "🗂️schema-document-index.json"]);
const roots = (process.env.ROOTS ?? "🧰️framework,✏️s,🌎️hub,♻️mit-bestand,.storybook").split(",");
const rules: [string, string][] = JSON.parse(process.env.RULES!);
const exclude = (process.env.EXCLUDE ?? "").split(",").filter(Boolean);
const dry = process.env.DRY === "1";
let changed = 0; const touched: string[] = [];
const walk = (d: string) => {
  let es; try { es = readdirSync(d, { withFileTypes: true }); } catch { return; }
  for (const e of es) {
    if (SKIP.has(e.name)) continue;
    const p = join(d, e.name);
    if (e.isDirectory()) walk(p);
    else if (e.isFile() && !SKIP_FILES.has(e.name)) {
      if (/\.(png|jpg|jpeg|gif|wasm|ico|woff2?|ttf|otf|pdf|zip|gz|br|bin|rlib|rmeta|a|o|so|dylib|node|glb|ktx2|webp|mp4|webm)$/iu.test(e.name)) continue;
      const rel = relative(ROOT, p);
      if (exclude.some(x => rel.includes(x))) continue;
      let s; try { s = statSync(p); } catch { continue; }
      if (s.size > 3_000_000) continue;
      let t: string; try { t = readFileSync(p, "utf8"); } catch { continue; }
      let u = t;
      for (const [a, b] of rules) u = u.split(a).join(b);
      if (u !== t) { changed++; touched.push(rel); if (!dry) writeFileSync(p, u); }
    }
  }
};
for (const r of roots) walk(join(ROOT, r));
console.log((dry ? "would change " : "changed ") + changed);
writeFileSync(process.env.OUT ?? "/dev/null", touched.join("\n") + "\n");
