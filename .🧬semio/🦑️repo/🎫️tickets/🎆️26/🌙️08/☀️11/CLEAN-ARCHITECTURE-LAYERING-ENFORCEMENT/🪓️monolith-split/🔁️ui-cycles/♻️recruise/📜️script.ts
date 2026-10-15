/** ♻️ Re-runs the dependency-cruiser over the UI module and prints strongly connected components plus no-circular counts per top-level folder.
 * Usage: bun 📜️script.ts <tag>   (run from the repository root)
 */
import { spawnSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";

const tag = process.argv[2] ?? "now";
const ticket = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split";
const generated = `${ticket}/🗑️generated`;
const ui = "🧰️framework/🔨️modules/🖱️ui/";
const cruise = spawnSync("node", ["node_modules/dependency-cruiser/bin/dependency-cruise.mjs", "🧰️framework/🔨️modules/🖱️ui", "--config", "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs", "--output-type", "json"], { maxBuffer: 1 << 30, timeout: 1_200_000 });
writeFileSync(`${generated}/ui-cycles-${tag}.json`, cruise.stdout);
const scc = spawnSync("bun", [`${ticket}/🔁️ui-cycles/📜️script.ts`, `${generated}/ui-cycles-${tag}.json`, ui], { maxBuffer: 1 << 28 });
writeFileSync(`${generated}/ui-scc-${tag}.txt`, scc.stdout);
for (const line of scc.stdout.toString().split("\n")) if (line.startsWith("###") || line.startsWith("TOTAL")) console.log(line);
const report = JSON.parse(readFileSync(`${generated}/ui-cycles-${tag}.json`, "utf8"));
const violations = report.summary.violations.filter((v: { rule: { name: string } }) => v.rule.name === "no-circular");
const byDir: Record<string, number> = {};
for (const v of violations) {
  if (!v.from.startsWith(ui)) continue;
  const segment = v.from.slice(ui.length).split("/")[0];
  byDir[segment] = (byDir[segment] ?? 0) + 1;
}
console.log(`no-circular violations total=${violations.length} ui-from=${Object.values(byDir).reduce((a, b) => a + b, 0)}`);
console.log(JSON.stringify(byDir));
