import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { shardWorkerSource } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧵️shard-worker-source/🟦️.ts";

/** 🧵️ Rewrites the staged `🧵️shard/🟨️shard-worker.js` of the prototype plugin modules from the current `shardWorkerSource()` without a full plugin build: a freshly staged `🌉️bridge.js` fails boot with `actor-core-module.loader-required` against an older worker.
 * Usage: `bun republish-shard-modules.ts` from the workspace root.
 * @see ./native-stage-modules.ts */
const out = join(process.cwd(), ".cache/b/p/m/browser/prototype/modules/🧵️shard/🟨️shard-worker.js");
writeFileSync(out, shardWorkerSource());
console.log(`republished ${out}`);
