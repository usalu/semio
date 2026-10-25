import { readFileSync } from "node:fs";
const [graphPath, entriesPath, detail] = process.argv.slice(2) as [string, string, string?];
const g = JSON.parse(readFileSync(graphPath, "utf8")), entries = readFileSync(entriesPath, "utf8").split("\n").filter(Boolean);
const deps = new Map<string, string[]>();
for (const m of g.modules) deps.set(m.source, (m.dependencies ?? []).map((d: any) => d.resolved));
const seen = new Set<string>(), stack = entries.filter((e) => deps.has(e));
while (stack.length) { const s = stack.pop()!; if (seen.has(s)) continue; seen.add(s); for (const d of deps.get(s) ?? []) if (deps.has(d) && !seen.has(d)) stack.push(d); }
console.log("entries", entries.length, "closure modules", seen.size);
const rows = (g.summary.violations as any[]).filter((v) => seen.has(v.from));
console.log("violations in closure", rows.length);
const m = new Map<string, number>();
for (const v of rows) m.set(v.rule.name, (m.get(v.rule.name) ?? 0) + 1);
for (const [k, n] of [...m].sort((a, b) => b[1] - a[1])) console.log(String(n).padStart(5), k);
if (detail) for (const v of rows) console.log(v.rule.name, "|", v.from.replace("🧰️framework/", "F/"), "->", v.to.replace("🧰️framework/", "F/"));
