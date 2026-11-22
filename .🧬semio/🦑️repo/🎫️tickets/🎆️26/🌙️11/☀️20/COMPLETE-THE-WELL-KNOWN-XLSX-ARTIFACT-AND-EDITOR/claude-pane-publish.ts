import { createHash } from "node:crypto";
import { cpSync, existsSync, mkdirSync, readFileSync, readdirSync, renameSync, rmSync, statSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { dirname, join, relative } from "node:path";

/** 🛠️ [DEBUG] Native publication of the browser modules of boot components: jco transpile, host shim, preview2 import rewrite, descriptor probe, bridge, receipt and artifact manifest, mirroring the managed component driver stages. Usage: `bun claude-pane-publish.ts <pluginId> <component.wasm> [...]`. */
const root = "/Users/ueli/Documents/semio";
const plugin = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin");
const bundle = join(plugin, "🌐️browser-bundle");
const materialization = join(bundle, "🏗️materialization");
const modules = join(root, ".cache/b/p/m/browser/prototype/modules");
const { transpilePluginComponentModulesAsync, rewritePreview2ShimImports } = await import(join(materialization, "🟦️.ts"));
const { hostShimSource } = await import(join(materialization, "🪢️host-shim-source/🟦️.ts"));
const { pluginComponentBridgeSource } = await import(join(materialization, "🌉️component-bridge-source/🟦️.ts"));
const { PLUGIN_HOST_SHIM_FILE } = await import(join(materialization, "🧾️contract/🟦️.ts"));
const { PREVIEW2_VENDOR_RELATIVE } = await import(join(bundle, "🕸️imports/🟦️.ts"));
const { PLUGIN_DESCRIPTOR_PROBE_SOURCE, finalizePluginDescriptor } = await import(join(bundle, "🛂️descriptor/🟦️.ts"));
const { encodeComponentReceiptV1, parseComponentReceiptV1 } = await import(join(plugin, "🖨️describe/🏷️component-receipt/🟦️.ts"));
const { MODULE_BRIDGE_FILE } = await import("@semio-tech/plugin-registry/deployment");
const sessionPlugins = JSON.parse(readFileSync(join(root, ".cache/b/p/m/browser/prototype/sessions/stdio-xlsx/🔌️plugins.json"), "utf8")) as { pluginId: string; packageId: string; cratePath: string; packageName: string; wasmOut: string; directoryName: string }[];
const vendor = join(modules, PREVIEW2_VENDOR_RELATIVE);

function walk(directory: string, prefix = ""): string[] {
  return readdirSync(directory, { withFileTypes: true }).flatMap(entry => entry.isDirectory() ? walk(join(directory, entry.name), `${prefix}${entry.name}/`) : [`${prefix}${entry.name}`]).sort();
}

const args = process.argv.slice(2);
for (let index = 0; index < args.length; index += 2) {
  const [pluginId, wasm] = [args[index]!, args[index + 1]!];
  const row = sessionPlugins.find(candidate => candidate.pluginId === pluginId);
  if (!row) throw Error(`Unknown plugin ${pluginId}`);
  const base = row.packageName.replaceAll("-", "_"), final = join(modules, row.directoryName), staging = `${final}.claude-staging`;
  rmSync(staging, { recursive: true, force: true });
  mkdirSync(staging, { recursive: true });
  console.log(`[DEBUG] transpiling ${pluginId} (${statSync(wasm).size} bytes)`);
  await transpilePluginComponentModulesAsync(wasm, staging, base, { repoRoot: root, mode: "prototype", preview2VendorDir: vendor });
  writeFileSync(join(staging, PLUGIN_HOST_SHIM_FILE), hostShimSource());
  rewritePreview2ShimImports(join(staging, `${base}.js`), vendor);
  const probe = spawnSync("node", ["--experimental-wasm-jspi", "--input-type=module", "--eval", PLUGIN_DESCRIPTOR_PROBE_SOURCE, join(staging, `${base}.js`)], { cwd: root, encoding: "utf8", maxBuffer: 64 << 20 });
  if (probe.status !== 0) throw Error(`describe() probe failed (${probe.status}): ${probe.stderr.slice(-4000)}`);
  const semantic = Buffer.from(probe.stdout.trim(), "base64");
  const descriptor = finalizePluginDescriptor(semantic, pluginId);
  const wasmBytes = readFileSync(wasm);
  const receipt = parseComponentReceiptV1(encodeComponentReceiptV1({
    schema: "semio.plugin.component-receipt/v1", pluginId, packageId: row.packageId, profile: "prototype", target: "wasm32-wasip2",
    wasmSha256: createHash("sha256").update(wasmBytes).digest("hex"), wasmBytes: wasmBytes.byteLength,
    semanticDescriptorSha256: createHash("sha256").update(semantic).digest("hex"),
    toolchain: "rustc 1.99.0-nightly (9f36de775bc636c8e88c31a173c2bcb6995956a0)",
    sourceTreeDigest: createHash("sha256").update(`native:${row.cratePath}`).digest("hex"),
    provenanceSha256: createHash("sha256").update(`native-claude-pane:${wasmBytes.byteLength}`).digest("hex"),
  }));
  writeFileSync(join(staging, "🛂️.descriptor.semio"), descriptor.pack);
  writeFileSync(join(staging, "🔣️.json"), descriptor.json);
  writeFileSync(join(staging, MODULE_BRIDGE_FILE), pluginComponentBridgeSource(base, row.wasmOut));
  writeFileSync(join(staging, "🧾️receipt.json"), encodeComponentReceiptV1(receipt));
  const files = walk(staging).filter(name => name !== ".semio-artifact.json");
  writeFileSync(join(staging, ".semio-artifact.json"), JSON.stringify({ version: 1, owner: `${row.cratePath}/Cargo.toml:browser`, files }));
  rmSync(final, { recursive: true, force: true });
  renameSync(staging, final);
  console.log(`[DEBUG] published ${pluginId} -> ${relative(root, final)} (${files.length} files)`);
}
