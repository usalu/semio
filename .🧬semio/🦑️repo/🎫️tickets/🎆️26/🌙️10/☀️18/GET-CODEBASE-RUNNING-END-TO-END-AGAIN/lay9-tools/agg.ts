import { readFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const R = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/";
const fam = (n: string) => n.replace(/^(no-cross-package-relative|no-cross-technology|no-cross-plugin|plugin-no-extension-or-artifact)-.+$/u, "$1");
const by: Record<string, number> = {};
let cyc = 0, cycSet = new Set<string>();
for (const x of r.summary.violations as any[]) {
  if (!x.from.startsWith(R)) continue;
  const f = fam(x.rule.name);
  by[f] = (by[f] ?? 0) + 1;
}
console.log(JSON.stringify({ totalCruised: r.summary.totalCruised, by, total: Object.values(by).reduce((a, b) => a + b, 0) }, null, 1));
