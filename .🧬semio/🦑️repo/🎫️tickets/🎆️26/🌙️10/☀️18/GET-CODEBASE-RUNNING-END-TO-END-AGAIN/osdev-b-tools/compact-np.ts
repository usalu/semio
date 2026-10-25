import { readdirSync } from "node:fs";
import { join } from "node:path";
import { openRecordStore, RECORD_STORE_PACK_LIMIT } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🗃️records/🟦️.ts";

const np = process.argv[2]!;
let compacted = 0, before = 0, after = 0;
for (const name of readdirSync(np).filter((entry) => entry.endsWith(".records"))) {
  const directory = join(np, name);
  before += readdirSync(directory).length;
  const store = openRecordStore(directory);
  if (store.stats.packs > RECORD_STORE_PACK_LIMIT) { store.compact(); compacted++; }
  store.close();
  after += readdirSync(directory).length;
}
console.log(`[compact-np] compacted ${compacted} stores, entries ${before} -> ${after}`);
