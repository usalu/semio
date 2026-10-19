import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import { dirname, join, resolve, basename } from "node:path";
const root = "/Users/ueli/Documents/semio";
const old = new Set<string>();
for (const f of ["renames1.txt", "renames2.txt"]) for (const l of readFileSync(f, "utf8").split("\n")) if (l) old.add(basename(l.split("|")[0]!));
old.add("🫀️core");
const skip = new Set(["node_modules", "target", "pkg", "dist", "📤️dist", ".git", "🗑️generated", ".cache", "bazel-out", "temp", "🎫️tickets"]);
const bad: string[] = [];
let n = 0;
const exts = ["", ".ts", ".tsx", ".js", ".mjs", ".cjs", ".json", ".mts"];
const walk = (d: string) => {
  let es; try { es = readdirSync(d, { withFileTypes: true }); } catch { return; }
  for (const e of es) {
    if (skip.has(e.name)) continue;
    const p = join(d, e.name);
    if (e.isDirectory()) walk(p);
    else if (/\.(ts|tsx|mts|js|mjs|cjs)$/u.test(e.name)) {
      const t = readFileSync(p, "utf8");
      for (const m of t.matchAll(/(?:from\s*|import\s*\(?\s*)["'](\.[^"']+)["']/gu)) {
        n++;
        const spec = m[1]!;
        const base = resolve(dirname(p), spec);
        if (exts.some((x) => existsSync(base + x) && (x !== "" || statSync(base).isFile())) || existsSync(join(base, "index.ts")) || existsSync(join(base, "🟦️.ts"))) continue;
        bad.push(`${p.slice(root.length + 1)} -> ${spec}`);
      }
    }
  }
};
for (const r of ["✏️s", "🧰️framework", "🌎️hub", "♻️mit-bestand", "🎓️teaching"]) walk(join(root, r));
console.log(`relative imports ${n}, unresolved ${bad.length}`);
const rel = bad.filter((b) => [...old].some((o) => b.includes(o)));
console.log(`unresolved mentioning a renamed name: ${rel.length}`);
for (const b of rel.slice(0, 30)) console.log("  ", b.slice(0, 250));
