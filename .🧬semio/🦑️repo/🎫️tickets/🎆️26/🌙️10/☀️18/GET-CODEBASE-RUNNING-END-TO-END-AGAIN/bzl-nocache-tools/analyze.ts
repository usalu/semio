/** 🔬 Runs the cache-fix handler analysis over every authored uncached target: `bun analyze.ts` writes `🗑️generated/bzl-nocache/records.json`. */
import { writeFileSync } from "node:fs";
import { join } from "node:path";
process.env.CF_TARGETS ??= join(import.meta.dir, "../🗑️generated/bzl-nocache/targets.json");
const { classifyAll } = await import("../cache-fix-tools/handler/classify.ts");
const recs = classifyAll();
writeFileSync(join(import.meta.dir, "../🗑️generated/bzl-nocache/records.json"), JSON.stringify(recs));
console.log(recs.length, "records;", recs.filter((r) => r.flags.length).length, "flagged");
