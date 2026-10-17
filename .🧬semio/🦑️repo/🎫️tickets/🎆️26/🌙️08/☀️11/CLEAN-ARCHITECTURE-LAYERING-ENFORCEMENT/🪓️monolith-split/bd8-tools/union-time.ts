import { readModuleTreeSource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
const plugin = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const roots: [string, string][] = JSON.parse(readFileSync("/private/tmp/claude-501/scratch/plugin-roots.json", "utf8"));
const t0 = performance.now();
let total = 0, n = 0;
for (const [name, p] of roots) {
  if (p.startsWith("🧷️companions") || p.startsWith("🧪️tests/") || name.includes("host-test") || p.startsWith("🖥️host/🧪️")) continue;
  total += readModuleTreeSource(resolve(plugin, p)).length; n++;
}
console.log(n, total, Math.round(performance.now() - t0), "ms");
