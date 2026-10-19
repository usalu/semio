import { readFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const R = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/";
const v = (r.summary.violations as any[]).filter((x) => x.from.startsWith(R));
const cnt = (m: Map<string, number>, k: string) => m.set(k, (m.get(k) ?? 0) + 1);
const ncpr = new Map<string, number>(), ui = new Map<string, number>(), uiFrom = new Map<string,number>();
for (const x of v) {
  if (x.rule.name.startsWith("no-cross-package")) cnt(ncpr, x.rule.name);
  if (x.rule.name === "renderer-hosts-only-ui") { cnt(ui, x.to.split("/").slice(0, 6).join("/")); cnt(uiFrom, x.from.replace(R,"").split("/").slice(0,4).join("/")); }
}
console.log([...ncpr].sort((a, b) => b[1] - a[1]));
console.log([...ui].sort((a, b) => b[1] - a[1]).slice(0, 30));
console.log([...uiFrom].sort((a, b) => b[1] - a[1]).slice(0, 30));
