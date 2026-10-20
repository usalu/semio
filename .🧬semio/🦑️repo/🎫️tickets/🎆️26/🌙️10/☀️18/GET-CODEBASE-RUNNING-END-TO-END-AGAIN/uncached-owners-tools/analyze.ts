/** 🔬 Runs the cache-fix handler analysis (call-graph walk, workspace-anchored writes, spawned processes) over the target list and writes one record per target to `🗑️generated/uncached-owners/records.json`. */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
process.env.CF_TARGETS ??= join(import.meta.dir, "../🗑️generated/uncached-owners/targets.json");
const { classifyAll } = await import("../cache-fix-tools/handler/classify.ts");
const recs = classifyAll();
const dir = join(import.meta.dir, "../🗑️generated/uncached-owners"); mkdirSync(dir, { recursive: true });
writeFileSync(join(dir, "records.json"), JSON.stringify(recs));
const stds: Record<string, number> = {}; for (const r of recs) stds[r.handler.std + "/" + r.handler.kind] = (stds[r.handler.std + "/" + r.handler.kind] ?? 0) + 1;
console.log(recs.length, "records;", recs.filter((r) => r.flags.length).length, "flagged", stds);
