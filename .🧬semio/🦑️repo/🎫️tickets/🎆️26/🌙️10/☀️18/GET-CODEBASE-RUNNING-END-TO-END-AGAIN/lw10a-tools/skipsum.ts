import { readFileSync } from "node:fs";
const p = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const by = new Map<string, { n: number; names: Set<string>; star: boolean }>();
for (const s of p.skipped) { const r = by.get(s.to) ?? { n: 0, names: new Set(), star: false }; r.n++; if (s.names === "*") r.star = true; else for (const x of s.names ?? []) r.names.add(x); by.set(s.to, r); }
for (const [t, r] of [...by].sort((a, b) => b[1].n - a[1].n)) console.log(r.n, t.replace("🧰️framework/🔨️modules/🏃️process/", ""), r.star ? "*" : "", [...r.names].join(",").slice(0, 160));
console.log(by.size);
