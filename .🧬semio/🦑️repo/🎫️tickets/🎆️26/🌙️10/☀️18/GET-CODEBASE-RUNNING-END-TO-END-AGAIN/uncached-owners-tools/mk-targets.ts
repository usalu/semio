/** 🎯 Writes the target list (project, root, targets) of every authored `cache: false` target without a recorded decision, the input the handler analysis reads (`CF_TARGETS`). */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const G = join(import.meta.dir, "../🗑️generated/uncached-owners");
const rows = JSON.parse(readFileSync(join(G, "rows.json"), "utf8")).filter((r: any) => !r.newCache && !r.newDecision);
const by = new Map<string, { project: string; root: string; targets: string[] }>();
for (const r of rows) { const e = by.get(r.manifest) ?? { project: r.project, root: r.root, targets: [] }; e.targets.push(r.target); by.set(r.manifest, e); }
writeFileSync(join(G, "targets.json"), JSON.stringify([...by.values()]));
console.log(rows.length, "targets in", by.size, "manifests");
