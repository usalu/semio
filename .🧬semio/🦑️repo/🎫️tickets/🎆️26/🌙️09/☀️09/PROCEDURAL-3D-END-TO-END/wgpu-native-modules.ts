import { spawnSync } from "node:child_process";
import { cpSync, existsSync, mkdirSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { builtPluginModulesRootIn } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts";
import { rewritePreview2ShimImports, transpilePluginComponentModulesAsync } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts";
import { hostShimSource } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🪢️host-shim-source/🟦️.ts";
import { pluginComponentBridgeSource } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts";
import { PLUGIN_HOST_SHIM_FILE } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧾️contract/🟦️.ts";
import { PLUGIN_DESCRIPTOR_PROBE_SOURCE, finalizePluginDescriptor } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🛂️descriptor/🟦️.ts";
import { MODULE_BRIDGE_FILE } from "@semio-tech/plugin-registry/deployment";
import { PREVIEW2_VENDOR_RELATIVE as VENDOR } from "@semio-tech/framework-plugin/browser-bundle/imports";

/** 🧩️ Graph-free twin of the component driver's transpile, assemble, probe and publish stages: stages each natively built wasm32-wasip2 component of `generation3d`
 * as one jco module into the prototype plugin-modules root (host shim, preview2 imports, `describe()` descriptor, bridge).
 * Usage: bun wgpu-native-modules.ts [pluginId...] — default is the whole closure. */
const workspace = process.cwd(), mode = "prototype" as const, root = builtPluginModulesRootIn(workspace, mode);
const target = join(workspace, ".cache/g3d-native/tp2/wasm32-wasip2/debug"), sessions = join(root, "../sessions/generation3d/🔌️plugins.json");
const rows = JSON.parse(readFileSync(sessions, "utf8")) as { pluginId: string; packageId: string; packageName: string; wasmOut: string; directoryName: string }[];
const closure = ["flow", "flow-extension-bim", "flow-extension-brep", "flow-extension-dictionary", "flow-extension-draw", "flow-extension-list", "flow-extension-logic", "flow-extension-math", "flow-extension-primitive", "flow-extension-text", "procedural"];
const wanted = process.argv.length > 2 ? process.argv.slice(2) : closure;
for (const pluginId of wanted) {
  const row = rows.find(r => r.pluginId === pluginId); if (!row) throw new Error("unknown plugin " + pluginId);
  const wasm = join(target, row.wasmOut); if (!existsSync(wasm)) { console.log("[g3d-native] skip " + pluginId + ": " + wasm + " missing"); continue; }
  const base = row.packageName.replaceAll("-", "_"), final = join(root, row.directoryName), staging = final + ".g3d-staging", scratch = join(workspace, ".cache/g3d-native/stage", pluginId);
  rmSync(staging, { recursive: true, force: true }); rmSync(scratch, { recursive: true, force: true }); mkdirSync(join(scratch, "transpiled"), { recursive: true });
  const started = Date.now();
  await transpilePluginComponentModulesAsync(wasm, join(scratch, "transpiled"), base, { repoRoot: workspace, mode, preview2VendorDir: join(root, VENDOR) });
  cpSync(join(scratch, "transpiled"), staging, { recursive: true });
  writeFileSync(join(staging, PLUGIN_HOST_SHIM_FILE), hostShimSource());
  rewritePreview2ShimImports(join(staging, base + ".js"), join(root, VENDOR));
  const probe = spawnSync("node", ["--experimental-wasm-jspi", "--input-type=module", "--eval", PLUGIN_DESCRIPTOR_PROBE_SOURCE, join(staging, base + ".js")], { cwd: workspace, encoding: "utf8", maxBuffer: 64 * 1024 * 1024, timeout: 300000 });
  if (probe.status !== 0) throw new Error(pluginId + ": describe() probe failed (" + probe.status + "): " + probe.stderr.slice(-2000));
  const descriptor = finalizePluginDescriptor(Buffer.from(probe.stdout.trim(), "base64"), pluginId);
  writeFileSync(join(staging, "🛂️.descriptor.semio"), descriptor.pack); writeFileSync(join(staging, "🔣️.json"), descriptor.json);
  writeFileSync(join(staging, MODULE_BRIDGE_FILE), pluginComponentBridgeSource(base, row.wasmOut));
  rmSync(final, { recursive: true, force: true }); renameSync(staging, final);
  console.log("[g3d-native] staged " + pluginId + " -> " + final + " in " + (Date.now() - started) + " ms");
}
