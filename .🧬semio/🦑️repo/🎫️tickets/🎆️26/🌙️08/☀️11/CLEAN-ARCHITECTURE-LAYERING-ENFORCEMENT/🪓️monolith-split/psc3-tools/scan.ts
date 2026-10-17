/** 🔭️ AST scan of every import edge the generated `no-cross-package-relative-*` rules flag. Usage: bun scan.ts <policy.json> <out.json> */
import { writeFileSync } from "node:fs";
import { loadPolicy, groupsOf, listSourceFiles, scanEdges } from "./model.ts";
const [policyPath, out] = process.argv.slice(2);
const groups = groupsOf(loadPolicy(policyPath));
const files = listSourceFiles();
const t0 = Date.now();
const edges = scanEdges(files, groups);
writeFileSync(out, JSON.stringify(edges));
const by = new Map<string, { all: number; prod: number }>();
for (const e of edges) { const v = by.get(e.sourceOwner) ?? { all: 0, prod: 0 }; v.all++; if (!e.test) v.prod++; by.set(e.sourceOwner, v); }
console.log(`files=${files.length} groups=${groups.length} edges=${edges.length} prod=${edges.filter(e => !e.test).length} ms=${Date.now() - t0}`);
for (const [k, v] of [...by].sort((a, b) => b[1].prod - a[1].prod).slice(0, 60)) console.log(String(v.prod).padStart(5), String(v.all).padStart(5), k);
