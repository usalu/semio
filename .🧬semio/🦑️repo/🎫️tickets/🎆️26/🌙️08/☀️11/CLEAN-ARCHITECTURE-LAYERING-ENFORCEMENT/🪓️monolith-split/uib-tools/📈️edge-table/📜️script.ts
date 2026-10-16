/** 📈 Classifies `from → to` edge lists (rule scan output) by owner area, tooling kind and target cluster and prints markdown tables.
 * Usage: bun 📜️script.ts <before.txt> <after.txt>
 */
import { readFileSync } from "node:fs";

const [beforePath, afterPath] = process.argv.slice(2) as [string, string];
const read = (p: string) => readFileSync(p, "utf8").split("\n").filter(Boolean);
const UI = "🧰️framework/🔨️modules/🖱️ui/";
const FW = "🧰️framework/🔨️modules/";
function area(from: string): string {
  const rel = from.slice(UI.length);
  if (rel.startsWith("🧬️contract/")) return "peer: ui-contract";
  if (rel.includes("🎯️targets/🧊️wgpu")) return "peer: ui wgpu";
  if (rel.startsWith("🪟️viewport/📦️packages/🦀️rust/📜️script.ts")) return "parent: viewport cargo script";
  if (/📜️script\.ts$/.test(rel)) return "tooling: 📜️script.ts";
  if (/(^|\/)🏗️build(er)?\//.test(rel) || /🧪️tests\/🎚️config\/🟦️\.ts$/.test(rel)) return "tooling: build/vite/vitest config";
  if (/(^|\/)🧪️tests\//.test(rel) || /🧪️\.story\.tsx$/.test(rel) || /\.test\.ts$/.test(rel)) return "tests and stories";
  if (/^🎚️axes\/(🏃️execution|📤️publication|🌱️bootstrap)\//.test(rel)) return "tooling: generator classes";
  return "production";
}
function cluster(to: string): string {
  if (to === "🧰️framework/📦️packages/🟦️typescript/🟦️.ts") return "root barrel @semio-tech/framework";
  if (to.startsWith(`${FW}🖼️assets/`)) return "assets";
  if (to.startsWith(`${FW}🕹️interaction/`)) return "interaction";
  if (to.startsWith(`${FW}🧬️schema/✅️validator/`)) return "schema validator";
  if (to.startsWith(`${FW}🛂️manifest/`)) return "manifest";
  if (to.startsWith(`${FW}🏃️process/`) || to.includes("🏃️process/")) return "process";
  return "other";
}
const table = (lines: string[]) => {
  const rows = new Map<string, number>();
  for (const l of lines) { const [from, to] = l.split(" → ") as [string, string]; const key = `${area(from)}\t${cluster(to)}`; rows.set(key, (rows.get(key) ?? 0) + 1); }
  return rows;
};
const before = table(read(beforePath)), after = table(read(afterPath));
const keys = [...new Set([...before.keys(), ...after.keys()])].sort();
console.log("| Area | Target cluster | Before | After |\n| --- | --- | ---: | ---: |");
for (const k of keys) { const [a, c] = k.split("\t"); console.log(`| ${a} | ${c} | ${before.get(k) ?? 0} | ${after.get(k) ?? 0} |`); }
const sum = (m: Map<string, number>, pred: (a: string) => boolean) => [...m].filter(([k]) => pred(k.split("\t")[0]!)).reduce((n, [, v]) => n + v, 0);
const groups: [string, (a: string) => boolean][] = [["production", a => a === "production"], ["tooling (all kinds)", a => a.startsWith("tooling")], ["tests and stories", a => a === "tests and stories"], ["peer or parent owned", a => a.startsWith("peer") || a.startsWith("parent")]];
console.log("\n| Group | Before | After |\n| --- | ---: | ---: |");
for (const [name, pred] of groups) console.log(`| ${name} | ${sum(before, pred)} | ${sum(after, pred)} |`);
console.log(`| total | ${read(beforePath).length} | ${read(afterPath).length} |`);
