import { readFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const only = process.argv[3];
const R = ["🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/", "🧰️framework/📦️packages/🟦️typescript/🛍️products/💻️os/🔨️modules/📺️renderer/"];
for (const x of r.summary.violations as any[]) {
  if (!R.some(p => x.from.startsWith(p))) continue;
  if (only && !x.rule.name.startsWith(only)) continue;
  console.log(x.rule.name, "|", x.from, "->", Array.isArray(x.to) ? x.to.join(">") : x.to, x.cycle ? "CYCLE " + x.cycle.map((c: any) => c.name).join(" > ") : "");
}
