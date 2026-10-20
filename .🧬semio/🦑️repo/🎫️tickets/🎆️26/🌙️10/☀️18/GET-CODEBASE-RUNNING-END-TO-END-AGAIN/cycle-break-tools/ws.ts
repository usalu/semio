import { readFileSync } from "node:fs";
import { resolve } from "node:path";
const g = JSON.parse(readFileSync(resolve(import.meta.dir, "../🗑️generated/cycle-break-x/graph.json"), "utf8")).edges as any[];
const who = process.argv[2]!;
const out = new Map<string, any>(), inn = new Map<string, any>();
for (const e of g) { if (e.from === who) { const o = out.get(e.to) ?? out.set(e.to, { rel: 0, bare: 0, decl: 0 }).get(e.to); o[e.kind]++; } if (e.to === who) { const o = inn.get(e.from) ?? inn.set(e.from, { rel: 0, bare: 0, decl: 0 }).get(e.from); o[e.kind]++; } }
const f = (m: Map<string, any>) => [...m].sort((a, b) => b[1].rel + b[1].bare - a[1].rel - a[1].bare).map(([k, v]) => `  ${k} rel:${v.rel} bare:${v.bare} decl:${v.decl}`).join("\n");
console.log("OUT of", who + "\n" + f(out) + "\nIN\n" + f(inn));
