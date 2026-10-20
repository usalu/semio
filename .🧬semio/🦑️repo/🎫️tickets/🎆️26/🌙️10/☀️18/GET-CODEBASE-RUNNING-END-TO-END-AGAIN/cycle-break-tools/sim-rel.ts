import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { build } from "./sim.ts";
const plan = JSON.parse(readFileSync(resolve(import.meta.dir, process.argv[2]!), "utf8"));
const names = new Set(plan.map((c: any) => c.name));
const es = build(plan);
const rel = es.filter(e => e.kind === "rel" && (names.has(e.to) || names.has(e.from)));
const byTo = new Map<string, { edges: number; targets: Set<string>; files: Set<string> }>();
for (const e of rel) { const r = byTo.get(e.to) ?? byTo.set(e.to, { edges: 0, targets: new Set(), files: new Set() }).get(e.to)!; r.edges++; r.targets.add(e.target); r.files.add(e.file); }
console.log("relative cross-package edges touching carves", rel.length);
for (const [k, v] of [...byTo].filter(([k]) => names.has(k)).sort((a, b) => b[1].edges - a[1].edges)) console.log(String(v.edges).padStart(5), "edges", String(v.targets.size).padStart(4), "targets", String(v.files.size).padStart(4), "files ->", k);
const bareTo = es.filter(e => e.kind === "bare" && names.has(e.to));
console.log("bare edges into carves", bareTo.length);
