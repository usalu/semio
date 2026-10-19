import { readFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const v = r.summary.violations as any[];
const fam = (n: string) => n.replace(/^(no-cross-package-relative|no-cross-plugin|plugin-no-extension-or-artifact|no-cross-technology)-.*/, "$1");
const by: Record<string, number> = {};
for (const x of v) by[fam(x.rule.name)] = (by[fam(x.rule.name)] ?? 0) + 1;
console.log("total", v.length, "modules", r.summary.totalCruised);
console.log(by);
const area = (p: string) => p.split("/").slice(0, p.startsWith("✏️s/🔌️plugins/") ? 3 : 2).join("/");
const edges: Record<string, number> = {};
for (const x of v) { const k = `${fam(x.rule.name)} ${area(x.from)} -> ${area(x.to)}`; edges[k] = (edges[k] ?? 0) + 1; }
for (const [k, n] of Object.entries(edges).sort((a, b) => b[1] - a[1])) console.log(n, k);
