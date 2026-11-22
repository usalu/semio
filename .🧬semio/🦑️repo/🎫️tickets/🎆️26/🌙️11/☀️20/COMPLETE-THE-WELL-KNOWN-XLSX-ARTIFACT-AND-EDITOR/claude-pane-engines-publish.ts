import { copyFileSync, mkdirSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
/** 🛠️ [DEBUG] Publishes the natively built wasm-pack web packages into the managed engine product directories. */
const root = "/Users/ueli/Documents/semio";
const { frameworkOsEngineDeliveriesV1 } = await import(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/📥️artifacts/🕸️engines/🟦️.ts"));
const out = join(import.meta.dir, "🗑️generated/claude-pane/native-wasm");
const base: Record<string, string> = { surface: "framework_surface", editor: "framework_editor", graph: "framework_os_node_graph" };
for (const d of frameworkOsEngineDeliveriesV1(join(root, ".cache/b/p/m"), { version: 1, mode: "prototype", facet: "renderer" })) {
  rmSync(d.root, { recursive: true, force: true });
  mkdirSync(d.root, { recursive: true });
  for (const f of d.files) copyFileSync(join(out, `out-${base[d.id]}`, f), join(d.root, f));
  writeFileSync(join(d.root, ".semio-artifact.json"), JSON.stringify({ version: 1, owner: d.request.owner, files: [...d.files].sort() }));
  console.log(`[DEBUG] published ${d.id} -> ${d.root}`);
}
