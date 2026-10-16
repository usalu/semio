/** 🔭️ AST scan of every import edge that reaches the schema, validator or value module from outside its owned region without an export target. Usage: bun scan.ts <out.json> */
import { writeFileSync } from "node:fs";
import { loadPackages, listSourceFiles, scanEdges } from "./model.ts";
const out = process.argv[2];
const pkgs = loadPackages();
const files = listSourceFiles();
const edges = scanEdges(files, pkgs);
writeFileSync(out, JSON.stringify(edges));
const by = new Map<string, number>();
for (const e of edges) by.set(e.owner + (e.test ? ":test" : ":prod"), (by.get(e.owner + (e.test ? ":test" : ":prod")) ?? 0) + 1);
console.log(`files=${files.length} packages=${pkgs.length} edges=${edges.length}`, Object.fromEntries(by));
