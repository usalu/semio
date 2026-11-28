/** Publishes only the generated shard worker (no wasm). */
import { writeFileSync } from "node:fs";
import { join } from "node:path";
const R = "/Users/ueli/Documents/semio";
const F = `${R}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization`;
const { shardWorkerSource } = await import(`${F}/🧵️shard-worker-source/🟦️.ts`);
const { SHARD_WORKER_FILE } = await import(`${F}/🧾️contract/🟦️.ts`);
writeFileSync(join(R, ".cache/b/p/m/browser/prototype/modules/🧵️shard", SHARD_WORKER_FILE), shardWorkerSource());
console.log("published shard worker", SHARD_WORKER_FILE);
