import { spawnSync } from "node:child_process";
import { cpSync, mkdirSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { builtPluginModulesRootIn } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts";
import { rewritePreview2ShimImports, transpilePluginComponentModulesAsync } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts";
import { hostShimSource } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🪢️host-shim-source/🟦️.ts";
import { monolithicComponentTable, pluginComponentBridgeSource } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts";
import { PLUGIN_HOST_SHIM_FILE } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧾️contract/🟦️.ts";
import { PLUGIN_DESCRIPTOR_PROBE_SOURCE, finalizePluginDescriptor } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🛂️descriptor/🟦️.ts";
import { MODULE_BRIDGE_FILE } from "@semio-tech/plugin-registry/deployment";
import { PREVIEW2_VENDOR_RELATIVE as VENDOR } from "@semio-tech/framework-plugin/browser-bundle/imports";

/** 🔬️ Materializes an isolated procedural browser module and probes its compiled guest descriptor. */
const wasm = resolve(process.argv[2]);
const output = join(import.meta.dir, "🗑️generated/bridge-current", process.argv[3] ?? "preview", "modules/🌀️procedural");
const vendor = join(dirname(output), VENDOR);
mkdirSync(output, { recursive: true });
cpSync(join(builtPluginModulesRootIn(process.cwd(), "prototype"), VENDOR), vendor, { recursive: true });
const base = "semio_hub_procedural_component";
await transpilePluginComponentModulesAsync(wasm, output, base, {
  repoRoot: process.cwd(), mode: "prototype", preview2VendorDir: vendor, environment: process.env,
});
rewritePreview2ShimImports(join(output, base + ".js"), vendor);
writeFileSync(join(output, PLUGIN_HOST_SHIM_FILE), hostShimSource());
const probe = spawnSync("node", ["--experimental-wasm-jspi", "--input-type=module", "--eval", PLUGIN_DESCRIPTOR_PROBE_SOURCE, join(output, base + ".js")], {
  encoding: "utf8", timeout: 300000, maxBuffer: 32 * 1024 * 1024,
});
if (probe.status !== 0) throw new Error(probe.error?.message ?? probe.stderr);
const descriptor = finalizePluginDescriptor(Buffer.from(probe.stdout.trim(), "base64"), "procedural");
writeFileSync(join(output, "🛂️.descriptor.semio"), descriptor.pack);
writeFileSync(join(output, "🔣️.json"), descriptor.json);
writeFileSync(join(output, MODULE_BRIDGE_FILE), pluginComponentBridgeSource(base, monolithicComponentTable(JSON.parse(descriptor.json), base)));
console.log("[DEBUG] Probed procedural runtime at", output);
