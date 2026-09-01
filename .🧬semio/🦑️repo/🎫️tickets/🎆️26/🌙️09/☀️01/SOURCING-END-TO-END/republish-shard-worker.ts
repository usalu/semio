/** 🧵️ Rewrites `🔌️plugin-modules/_shard/🟨️shard-worker.js` from the current
 * `shardWorkerSource()` without running a full plugin build — the dev host only republishes it as a
 * side effect of `plugin <id>`, which is a ~7-minute cargo cycle. */
import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { shardWorkerSource } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🌐plugin-web-materialize.ts";

const out = join(process.cwd(), "🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules/_shard/🟨️shard-worker.js");
writeFileSync(out, shardWorkerSource());
console.log(`[DEBUG] republished ${out}`);
