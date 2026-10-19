import { readFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const roots = ["🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/", "🧰️framework/📦️packages/🟦️typescript/🛍️products/💻️os/🔨️modules/📺️renderer/"];
const by = new Map<string, number>();
let total = 0;
for (const m of r.modules) for (const d of m.dependencies ?? []) for (const rule of d.rules ?? []) {
  if (!roots.some(x => m.source.startsWith(x))) continue;
  const k = rule.name.replace(/-[a-z0-9-]+$/u, rule.name.startsWith("no-cross-package-relative") ? "" : "$&");
  const key = rule.name.startsWith("no-cross-package-relative") ? "no-cross-package-relative" : rule.name;
  by.set(key, (by.get(key) ?? 0) + 1); total++;
}
for (const m of r.modules) for (const rule of m.rules ?? []) {
  if (!roots.some(x => m.source.startsWith(x))) continue;
  by.set(rule.name + "(module)", (by.get(rule.name + "(module)") ?? 0) + 1); total++;
}
console.log(JSON.stringify(Object.fromEntries([...by].sort((a, b) => b[1] - a[1])), null, 1), total, r.summary.totalCruised);
