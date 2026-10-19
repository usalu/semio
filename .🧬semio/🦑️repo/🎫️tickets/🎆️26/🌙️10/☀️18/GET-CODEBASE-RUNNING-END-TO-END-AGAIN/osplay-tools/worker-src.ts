
import { shardWorkerSource } from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧵️shard-worker-source/🟦️.ts";
{ const s = shardWorkerSource(); require("node:fs").writeFileSync(process.env.OUT!, s); console.log("[DEBUG] len", s.length, (s.match(/^import .*$/gm) ?? []).slice(0, 8)); }
