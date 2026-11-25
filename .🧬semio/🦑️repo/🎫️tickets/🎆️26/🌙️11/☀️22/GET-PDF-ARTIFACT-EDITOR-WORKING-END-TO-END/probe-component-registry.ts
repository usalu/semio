import { join } from "node:path";

/** 🔎️ Lists the component source owners registry discovery finds for `stdio-pdf`, to show whether the shard component crates join the plugin registry beside the hub component.
 * Usage: `bun probe-component-registry.ts` from the workspace root. */
const root = process.cwd();
const { generateComponentSourceRegistry } = await import(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts"));
const owners = generateComponentSourceRegistry(root) as { pluginId: string; cratePath: string; wasmOut: string; role: string }[];
for (const owner of owners.filter(row => row.pluginId.includes("pdf") || row.cratePath.includes("stdio/🧩️extensions/📘️pdf"))) console.log(owner.pluginId, owner.role, owner.wasmOut, owner.cratePath.slice(-90));
console.log("owners", owners.length, "distinct plugin ids", new Set(owners.map(row => row.pluginId)).size);
