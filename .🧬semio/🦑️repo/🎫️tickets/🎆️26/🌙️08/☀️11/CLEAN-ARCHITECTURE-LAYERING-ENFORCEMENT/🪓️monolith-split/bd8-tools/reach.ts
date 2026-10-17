import { listModuleTree } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import { readFileSync, existsSync } from "node:fs";
import { resolve } from "node:path";
import { execSync } from "node:child_process";
const plugin = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const roots: [string, string][] = JSON.parse(readFileSync("/private/tmp/claude-501/scratch/plugin-roots.json", "utf8"));
const reached = new Set<string>();
for (const [, p] of roots) { if (p.startsWith("🧷️companions") || p.startsWith("🧪️tests/")) continue; for (const f of listModuleTree(resolve(plugin, p))) reached.add(f); }
const all = execSync("git -c core.fsmonitor=false ls-files --cached --others --exclude-standard -- '*.rs'", { cwd: plugin, encoding: "utf8", maxBuffer: 1 << 28 }).split("\n").filter(f => f && existsSync(resolve(plugin, f)) && !f.includes("🧪️tests") && !f.startsWith("🧷️companions"));
const missing = all.filter(f => !reached.has(resolve(plugin, f)));
console.log(all.length, reached.size, missing.length);
for (const m of missing) console.log(m);
