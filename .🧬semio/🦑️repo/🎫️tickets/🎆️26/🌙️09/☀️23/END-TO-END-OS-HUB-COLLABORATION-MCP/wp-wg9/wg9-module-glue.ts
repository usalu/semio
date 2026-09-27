/** 🧬️ WG9 (ticket-local, from WG7's s12-module-glue): rewrites the shard worker of WG9's durable catalog-module roots from the CURRENT
 * generator (`shardWorkerSource`). `wg7-catalog-module.ts` copies `🧵️shard` from the shared release root, whose worker dates from
 * 2026-09-25 07:54 and has no `codec` case, so the browser component codec (pack-schema-hash, the identity every document socket dials
 * with) failed and the wasm32 shell never dialled the hub (run s13a). The served component bytes stay the catalog's.
 * Usage: bun wg9-module-glue.ts [--check] <durable root name…> */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { SHARD_WORKER_FILE, shardWorkerSource } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts";
const names = process.argv.slice(2).filter((arg) => !arg.startsWith("--"));
for (const name of names) {
  const path = join(`/Users/ueli/Documents/semio/.🧬semio/🌐hub/${name}/release/🔌️plugin-modules`, "🧵️shard", SHARD_WORKER_FILE);
  const before = readFileSync(path, "utf8");
  const content = shardWorkerSource();
  console.log(`${name}: ${before === content ? "fresh" : "stale"} (${before.length} -> ${content.length} chars, codec case ${content.includes('case "codec"')})`);
  if (!process.argv.includes("--check")) writeFileSync(path, content);
}
