import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
const seen = new Set<string>(); const missing: string[] = [];
const re = /(?:from\s*|import\s*\(\s*|import\s+)["'](\.[^"']+)["']/g;
function visit(file: string) {
  if (seen.has(file)) return; seen.add(file);
  const text = readFileSync(file, "utf8");
  for (const m of text.matchAll(re)) {
    const target = resolve(dirname(file), m[1]!);
    const ext = [".ts", ".tsx", ".json", ".mjs", ""].find((e) => existsSync(target + e) && !(e === "" && !/\.\w+$/.test(target)));
    if (ext === undefined) { missing.push(`${file} -> ${m[1]}`); continue; }
    const real = target + ext;
    if (/\.(ts|tsx|mjs)$/.test(real) && !real.includes("node_modules")) visit(real);
  }
}
for (const f of process.argv.slice(2)) visit(resolve(f));
console.log(`visited ${seen.size} files, ${missing.length} dangling`);
for (const m of missing) console.log(m);
