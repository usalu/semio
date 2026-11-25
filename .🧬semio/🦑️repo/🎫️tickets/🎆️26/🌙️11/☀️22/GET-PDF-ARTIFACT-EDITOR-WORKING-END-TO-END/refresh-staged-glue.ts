import { mkdir, readFile, writeFile } from "node:fs/promises";
import { join } from "node:path";

/** 🔄️ Rewrites the generated bridge, host shim and shard worker of the staged PDF module from the current sources without transpiling the component again, for a change that touches only the generated JavaScript.
 * Usage: `bun refresh-staged-glue.ts` from the workspace root.
 * @see ./🧮️pdf-browser-page-memory.md */

const root = process.cwd();
const modules = join(root, ".cache/b/p/m/browser/prototype/modules");
const stage = join(modules, "📘️stdio-pdf");
const components = ((JSON.parse(await readFile(join(stage, "🔣️.json"), "utf8")) as { components?: { id: string }[] }).components ?? []).map(row => ({ id: row.id, base: `${row.id.replaceAll("-", "_")}_component` }));
if (components.length === 0) throw new Error("the staged stdio-pdf descriptor declares no components: run stage-pdf-component.ts first");
const materialization = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization");
const { pluginComponentBridgeSource } = await import(join(materialization, "🌉️component-bridge-source/🟦️.ts"));
const { hostShimSource } = await import(join(materialization, "🪢️host-shim-source/🟦️.ts"));
const { shardWorkerSource } = await import(join(materialization, "🧵️shard-worker-source/🟦️.ts"));
const { SHARD_WORKER_FILE } = await import(join(materialization, "🧾️contract/🟦️.ts"));
const { MODULE_SHARD_DIRECTORY } = await import(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts"));
await writeFile(join(stage, "🌉️bridge.js"), pluginComponentBridgeSource(components[0]!.base, Object.fromEntries(components.map(component => [component.id, component.base]))));
await writeFile(join(stage, "🟨️.js"), hostShimSource());
await mkdir(join(modules, MODULE_SHARD_DIRECTORY), { recursive: true });
await writeFile(join(modules, MODULE_SHARD_DIRECTORY, SHARD_WORKER_FILE), shardWorkerSource());
console.log(`refreshed the bridge, host shim and shard worker of ${stage}`);
