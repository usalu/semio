import { readFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const OS = "🧰️framework/🛍️products/💻️os/";
const rend = OS + "🔨️modules/📺️renderer/";
const fam = (n: string) => n.replace(/^(no-cross-package-relative|no-cross-technology|no-cross-plugin|plugin-no-extension-or-artifact)-.+$/u, "$1");
const v = r.summary.violations as any[];
const all: Record<string, number> = {}, wp6: Record<string, number> = {}, cyc = { all: 0, nonRenderer: 0, renderer: 0 };
for (const x of v) {
  const f = fam(x.rule.name);
  all[f] = (all[f] ?? 0) + 1;
  const isRend = x.from.startsWith(rend);
  if (f === "no-circular") { cyc.all++; isRend ? cyc.renderer++ : cyc.nonRenderer++; }
  if (!isRend && x.to.startsWith(OS)) wp6[f] = (wp6[f] ?? 0) + 1;
}
console.log(JSON.stringify({ totalCruised: r.summary.totalCruised, all, wp6, wp6total: Object.values(wp6).reduce((a, b) => a + b, 0), cyc }, null, 1));
