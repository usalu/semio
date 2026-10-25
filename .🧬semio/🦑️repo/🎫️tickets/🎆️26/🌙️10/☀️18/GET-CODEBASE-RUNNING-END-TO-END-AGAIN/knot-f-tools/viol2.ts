import { readFileSync } from "node:fs";
const g = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const rows = g.summary.violations as any[];
const key = (p: string) => { const s = p.split("/"); return s.slice(0, p.startsWith("🧰️framework/🛍️products") ? 4 : p.startsWith("🧰️framework/🔨️modules") ? 3 : 2).join("/"); };
const m = new Map<string, number>();
for (const v of rows) m.set(key(v.from), (m.get(key(v.from)) ?? 0) + 1);
for (const [k, n] of [...m].sort((a, b) => b[1] - a[1])) console.log(String(n).padStart(6), k);
if (process.argv[3]) for (const v of rows.filter((v) => v.from.startsWith(process.argv[3]!) || v.to.startsWith(process.argv[3]!))) console.log(v.rule.name, "|", v.from.replace("🧰️framework/", "F/"), "->", v.to.replace("🧰️framework/", "F/"));
