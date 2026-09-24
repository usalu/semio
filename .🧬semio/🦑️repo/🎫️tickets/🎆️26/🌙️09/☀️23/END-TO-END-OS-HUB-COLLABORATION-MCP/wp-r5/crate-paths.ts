import { readFileSync, readdirSync, statSync, existsSync } from "node:fs";
import { join } from "node:path";
const root = "/Users/ueli/Documents/semio";
const out: string[] = [];
const skip = new Set(["node_modules", "target", ".git", "🧫️fixtures", "🖼️assets", "dist"]);
const walk = (dir: string) => {
  let entries; try { entries = readdirSync(dir, { withFileTypes: true }); } catch { return; }
  if (entries.some((e) => e.name === "🥒️.feature") && existsSync(join(dir, "🦀️.rs"))) {
    const text = readFileSync(join(dir, "🦀️.rs"), "utf8");
    const mods = new Set([...text.matchAll(/\bmod\s+([a-z_0-9]+)/g)].map((m) => m[1]));
    const bad = new Set([...text.matchAll(/\bcrate::([a-z_0-9]+)/g)].map((m) => m[1]).filter((m) => !mods.has(m)));
    if (bad.size) out.push(`${dir.slice(root.length + 1)} :: ${[...bad].join(",")}`);
  }
  for (const e of entries) if (e.isDirectory() && !skip.has(e.name) && !e.name.startsWith(".")) walk(join(dir, e.name));
};
for (const top of ["✏️s", "🧰️framework", "💻️os"]) if (existsSync(join(root, top))) walk(join(root, top));
console.log(out.join("\n"));
console.log(out.length);
