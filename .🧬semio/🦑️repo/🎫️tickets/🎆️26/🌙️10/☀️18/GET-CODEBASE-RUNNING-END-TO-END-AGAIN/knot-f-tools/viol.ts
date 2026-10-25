import { readFileSync } from "node:fs";
const g = JSON.parse(readFileSync(process.argv[2]!, "utf8")), scopes = (process.argv[3] ?? "").split(",").filter(Boolean);
const rows = g.summary.violations as any[];
const byRule = new Map<string, number>();
for (const v of rows) byRule.set(v.rule.name, (byRule.get(v.rule.name) ?? 0) + 1);
console.log("total", rows.length, "modules", g.modules.length);
for (const [k, n] of [...byRule].sort((a, b) => b[1] - a[1]).slice(0, 25)) console.log(String(n).padStart(6), k);
for (const s of scopes) {
  const sel = rows.filter((v) => v.from.includes(s) || v.to.includes(s));
  console.log("--- scope", s, sel.length);
  const m = new Map<string, number>();
  for (const v of sel) m.set(v.rule.name, (m.get(v.rule.name) ?? 0) + 1);
  for (const [k, n] of [...m].sort((a, b) => b[1] - a[1])) console.log(String(n).padStart(6), k);
}
