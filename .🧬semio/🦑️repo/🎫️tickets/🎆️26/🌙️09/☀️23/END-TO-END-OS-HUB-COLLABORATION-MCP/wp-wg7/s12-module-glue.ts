/** 🧬️ WG7 (ticket-local): rewrites the generated glue of the durable catalog-module root (`s11-wg7-catalog-modules/release`) from the
 * current generators — the note bridge (`pluginComponentBridgeSource`, now with the `codec` arm) and the shard worker (`shardWorkerSource`,
 * now with the `codec` case). The transpiled component and its descriptor are untouched (the served bytes stay the catalog's).
 * Usage: bun .tmp-ticket/wp-wg7/s12-module-glue.ts [--check] [--root=<durable module root name>] */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { pluginComponentBridgeSource, SHARD_WORKER_FILE, shardWorkerSource } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts";
import { MODULE_BRIDGE_FILE } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
const root = `/Users/ueli/Documents/semio/.🧬semio/🌐hub/${process.argv.find((arg) => arg.startsWith("--root="))?.slice(7) ?? "s11-wg7-catalog-modules"}/release/🔌️plugin-modules`;
const files = [
  { path: join(root, "🗒️note", MODULE_BRIDGE_FILE), content: pluginComponentBridgeSource("semio_s_plugin_note_component", "semio_s_plugin_note.wasm") },
  { path: join(root, "🧵️shard", SHARD_WORKER_FILE), content: shardWorkerSource() },
];
for (const file of files) {
  const before = readFileSync(file.path, "utf8");
  console.log(`${file.path}: ${before === file.content ? "fresh" : "stale"} (${before.length} -> ${file.content.length} chars)`);
  if (!process.argv.includes("--check")) writeFileSync(file.path, file.content);
}
