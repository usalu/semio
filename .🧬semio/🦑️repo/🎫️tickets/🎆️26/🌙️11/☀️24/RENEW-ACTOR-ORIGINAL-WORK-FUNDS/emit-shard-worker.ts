/** 🧵️ Writes the shard worker the current sources generate, for probes that route it in place of the staged one. */
import { writeFileSync } from "node:fs";
import { resolve } from "node:path";
const { shardWorkerSource } = await import(resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧵️shard-worker-source/🟦️.ts"));
writeFileSync(process.argv[2]!, shardWorkerSource());
