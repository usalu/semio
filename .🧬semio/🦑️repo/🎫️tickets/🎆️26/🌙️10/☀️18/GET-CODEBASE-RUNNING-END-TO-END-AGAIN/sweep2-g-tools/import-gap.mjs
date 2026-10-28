// sweep2-g: relative static imports of the .mjs/.ts files of a key list that resolve to a workspace file outside the list (the closure inference's blind spots). usage: bun import-gap.mjs <key-list> [filter-regex]. Streams, RSS guard.
import {readFileSync, existsSync} from "node:fs";
import {dirname, join, normalize} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const [list, filter = "."] = process.argv.slice(2);
const files = readFileSync(list, "utf8").split("\n").filter(Boolean), set = new Set(files), pattern = new RegExp(filter, "u");
const re = /(?:^|[^\w.$])(?:import|export)\b[^"'`;]*?\bfrom\s*["']([^"']+)["']|(?:^|[^\w.$])import\s*\(\s*["']([^"']+)["']\s*\)|(?:^|[^\w.$])import\s*["']([^"']+)["']/gu;
const gaps = new Map();
for (const file of files) {
  if (!/\.(mjs|ts|tsx|js|cjs)$/u.test(file) || !pattern.test(file)) continue;
  let text; try { text = readFileSync(join(ROOT, file), "utf8"); } catch { continue; }
  for (const m of text.matchAll(re)) {
    if (/^(?:import|export)\s+type\b/u.test(m[0].replace(/^[^\w]+/u, ""))) continue;
    const spec = m[1] ?? m[2] ?? m[3];
    if (!spec || !spec.startsWith(".")) continue;
    const target = normalize(join(dirname(file), spec));
    if (!set.has(target) && existsSync(join(ROOT, target))) { const rows = gaps.get(target) ?? []; rows.push(file); gaps.set(target, rows); }
  }
  if (process.memoryUsage().rss > 500e6) { console.error("RSS above 500 MB"); process.exit(3); }
}
for (const [target, importers] of [...gaps].sort()) console.log(target, "<=", importers.slice(0, 2).join(" , "), importers.length > 2 ? `(+${importers.length - 2})` : "");
console.error(`${gaps.size} gaps`);
