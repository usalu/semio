import { readFileSync, readdirSync, statSync } from "node:fs";
import { join, relative } from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const SKIP = new Set(["node_modules", "dist", "target", "pkg", ".cache", "🗑️generated", ".git", "storybook-static"]);
const roots = (process.env.ROOTS ?? "🧰️framework,✏️s,🌎️hub,♻️mit-bestand,.storybook,.vscode,.github").split(",");
const needles = process.argv.slice(2);
const hits: Record<string, string[]> = Object.fromEntries(needles.map(n => [n, []]));
const walk = (d: string) => {
  let es; try { es = readdirSync(d, { withFileTypes: true }); } catch { return; }
  for (const e of es) {
    if (SKIP.has(e.name)) continue;
    const p = join(d, e.name);
    if (e.isDirectory()) walk(p);
    else if (e.isFile()) {
      let s; try { s = statSync(p); } catch { continue; }
      if (s.size > 3_000_000) continue;
      if (/\.(png|jpg|jpeg|gif|wasm|ico|woff2?|ttf|otf|pdf|zip|gz|br|bin|rlib|rmeta|a|o|so|dylib|node|glb|ktx2|webp|mp4|webm)$/iu.test(e.name)) continue;
      let t: string; try { t = readFileSync(p, "utf8"); } catch { continue; }
      for (const n of needles) if (t.includes(n)) hits[n]!.push(relative(ROOT, p));
    }
  }
};
for (const r of roots) walk(join(ROOT, r));
for (const [n, fs] of Object.entries(hits)) { console.log("=== " + n + " (" + fs.length + ")"); for (const f of fs) console.log("  " + f); }
