import { readFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const OS = "🧰️framework/🛍️products/💻️os/";
const rend = "/📺️renderer/";
const m: Record<string, number> = {};
for (const x of r.summary.violations as any[]) if (x.from.startsWith(OS) && !x.from.includes(rend) && !x.to.startsWith(OS)) m[x.rule.name] = (m[x.rule.name] ?? 0) + 1;
console.log(Object.entries(m).sort((a, b) => b[1] - a[1]).map(([k, c]) => c + " " + k).join("\n"));
