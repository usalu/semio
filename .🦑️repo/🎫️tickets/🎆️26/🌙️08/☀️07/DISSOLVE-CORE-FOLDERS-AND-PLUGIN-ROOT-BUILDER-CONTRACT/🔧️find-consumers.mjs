import { readdirSync, readFileSync, statSync } from "fs";
import { join, relative } from "path";

const ROOT = "/Users/ueli/Documents/semio";
const SKIP = new Set(["node_modules", "target", ".git", ".nx", "dist", "build", ".repo-cache", ".venv"]);

function walk(dir, acc=[]) {
  let entries;
  try { entries = readdirSync(dir); } catch { return acc; }
  for (const name of entries) {
    if (SKIP.has(name)) continue;
    const p = join(dir, name);
    let st;
    try { st = statSync(p); } catch { continue; }
    if (st.isDirectory()) walk(p, acc);
    else acc.push(p);
  }
  return acc;
}

const files = walk(ROOT);
const patterns = [
  { name: "semio-framework-core", re: /semio-framework-core/ },
  { name: "@semio-tech/framework-core", re: /@semio-tech\/framework-core/ },
  { name: "🧩core path", re: /🧩core|modules\/🧩core/ },
  { name: "framework-core package path", re: /framework-core/ },
];

for (const pat of patterns) {
  const hits = [];
  for (const f of files) {
    if (!/\.(rs|toml|ts|tsx|js|mjs|json|md|cjs)$/.test(f)) continue;
    // skip ticket folder noise and target-like
    if (f.includes("🎫️tickets") || f.includes("node_modules")) continue;
    let text;
    try { text = readFileSync(f, "utf8"); } catch { continue; }
    if (pat.re.test(text)) hits.push(relative(ROOT, f));
  }
  console.log(`\n=== ${pat.name}: ${hits.length} ===`);
  for (const h of hits.slice(0, 120)) console.log(h);
  if (hits.length > 120) console.log(`... +${hits.length-120} more`);
}
