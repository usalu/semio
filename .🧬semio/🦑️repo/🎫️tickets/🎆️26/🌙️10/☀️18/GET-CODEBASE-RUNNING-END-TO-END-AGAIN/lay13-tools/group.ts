import { readFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const own = (p: string): string => { const s = p.split("/"); if (s[0]==="🧰️framework" && s[1]==="🛍️products") return s.slice(0,3).join("/")+(s[2]==="💻️os"?"/"+s[3]+"/"+(s[4]??""):""); if (s[0]==="✏️s") return s.slice(0,3).join("/"); return s.slice(0,2).join("/"); };
const t: Record<string, Record<string, number>> = {};
for (const v of r.violations) { const k = v.rule; const o = own(v.from) + " -> " + own(v.to); ((t[k] ??= {})[o] = (t[k][o] ?? 0) + 1); }
for (const [k, m] of Object.entries(t)) { const tot = Object.values(m).reduce((a, b) => a + b, 0); console.log(`\n## ${k} ${tot}`); for (const [o, n] of Object.entries(m).sort((a, b) => b[1] - a[1]).slice(0, +(process.argv[3] ?? 12))) console.log(`  ${n} ${o}`); }
