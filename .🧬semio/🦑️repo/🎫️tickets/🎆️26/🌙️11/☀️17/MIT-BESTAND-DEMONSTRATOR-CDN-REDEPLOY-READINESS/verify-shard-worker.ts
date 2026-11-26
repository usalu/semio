import { readFileSync } from "node:fs";
import { gzipCoreSuffix } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📦️distribution/🗜️cores/🟦️.ts";
import { shardWorkerPublicationV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧵️shard-worker-source/🟦️.ts";

/** 🧵 Checks that the served shard uses the current compressed-core runtime. */
export function verifyShardWorker(source: string): void {
  if (shardWorkerPublicationV1(source) !== "current" || !source.includes(gzipCoreSuffix) || !source.includes("DecompressionStream")) {
    throw new Error("Served shard worker does not match the current gzip-aware release worker");
  }
}

if (import.meta.main) verifyShardWorker(readFileSync(process.argv[2]!, "utf8"));
