process.env.SEMIO_PLUGIN_CATALOG ??= "/Users/ueli/Documents/semio/✏️s/🧑‍💻dev/🧩️catalog/🤖️generated";
import {createHash} from "node:crypto";
import {cpSync, existsSync, mkdirSync, readFileSync, readdirSync, renameSync, rmSync, statSync, writeFileSync} from "node:fs";
import {join, relative, resolve} from "node:path";
import {spawnSync} from "node:child_process";
import {transpilePluginComponentModulesAsync, rewritePreview2ShimImports} from "@semio-tech/framework-plugin/browser-bundle/materialization";
import {hostShimSource} from "@semio-tech/framework-plugin/browser-bundle/materialization/host-shim-source";
import {pluginComponentBridgeSource} from "@semio-tech/framework-plugin/browser-bundle/materialization/component-bridge-source";
import {PLUGIN_HOST_SHIM_FILE} from "@semio-tech/framework-plugin/browser-bundle/materialization/contract";
import {PLUGIN_DESCRIPTOR_PROBE_SOURCE, finalizePluginDescriptor} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🛂️descriptor/🟦️.ts";
import {encodeComponentReceiptV1} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🟦️.ts";
import {componentModuleOwnerV1, MODULE_BRIDGE_FILE} from "@semio-tech/plugin-registry/deployment";
import {PREVIEW2_VENDOR_RELATIVE} from "@semio-tech/framework-plugin/browser-bundle/imports";
import {stripWasmDebugSectionsV1} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🪶️strip-debug/🟦️.ts";

const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const modulesRoot = join(repoRoot, ".cache/b/p/m/browser/prototype/modules");
const catalog = JSON.parse(readFileSync(join(repoRoot, "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🔌️plugins.json"), "utf8")) as {pluginId: string; packageId: string; cratePath: string; packageName: string; wasmOut: string; directoryName: string}[];
const work = join(import.meta.dir, "🗑️generated/claude-full-s/work");
const layoutReceipt = join(modulesRoot, "📏️layout/🧾️receipt.json");
const toolchain = existsSync(layoutReceipt) ? JSON.parse(readFileSync(layoutReceipt, "utf8")).toolchain as string : (() => {
  const probed = spawnSync("rustc", ["-vV"], {encoding: "utf8"});
  const commit = probed.stdout.match(/^commit-hash: ([0-9a-f]+)$/m)?.[1];
  const release = probed.stdout.match(/^release: (.+)$/m)?.[1];
  if (probed.status !== 0 || !commit || !release) throw new Error(`rustc -vV did not name a toolchain: ${probed.stderr}`);
  return `rustc ${release} (${commit})`;
})();
const sha = (b: Uint8Array | string) => createHash("sha256").update(b).digest("hex");

function files(dir: string, base = dir): string[] {
  return readdirSync(dir, {withFileTypes: true}).flatMap(e => e.isDirectory() ? files(join(dir, e.name), base) : [relative(base, join(dir, e.name)).replaceAll("\\", "/")]).sort();
}

for (const pair of process.argv.slice(2)) {
  const [pluginId, wasm] = pair.split("=") as [string, string];
  const entry = catalog.find(e => e.pluginId === pluginId);
  if (!entry || !existsSync(wasm)) throw new Error(`Unknown plugin or missing wasm: ${pair}`);
  const base = entry.packageName.replaceAll("-", "_"), stage = join(work, pluginId), module = join(stage, "module");
  if (existsSync(join(module, ".semio-artifact.json"))) { console.log(`[DEBUG] ${pluginId}: reusing staged module`); }
  else {
  rmSync(stage, {recursive: true, force: true});
  mkdirSync(stage, {recursive: true});
  console.log(`[DEBUG] ${pluginId}: transpile ${wasm}`);
  await transpilePluginComponentModulesAsync(wasm, module, base, {repoRoot, mode: "prototype", preview2VendorDir: join(modulesRoot, PREVIEW2_VENDOR_RELATIVE)} as never);
  writeFileSync(join(module, PLUGIN_HOST_SHIM_FILE), hostShimSource());
  rewritePreview2ShimImports(join(module, `${base}.js`), join(stage, PREVIEW2_VENDOR_RELATIVE));
  cpSync(join(modulesRoot, PREVIEW2_VENDOR_RELATIVE), join(stage, PREVIEW2_VENDOR_RELATIVE), {recursive: true});
  console.log(`[DEBUG] ${pluginId}: probe describe()`);
  const probe = spawnSync("node", ["--experimental-wasm-jspi", "--input-type=module", "--eval", PLUGIN_DESCRIPTOR_PROBE_SOURCE, join(module, `${base}.js`)], {cwd: repoRoot, encoding: "utf8", maxBuffer: 64 << 20});
  if (probe.status !== 0) throw new Error(`describe probe failed: ${probe.stderr.slice(-4000)}`);
  const semantic = Buffer.from(probe.stdout.trim(), "base64");
  const descriptor = finalizePluginDescriptor(semantic, pluginId);
  const wasmBytes = readFileSync(wasm);
  const receiptValue = {schema: "semio.plugin.component-receipt/v1", pluginId, packageId: entry.packageId, profile: "prototype", target: "wasm32-wasip2", wasmSha256: sha(wasmBytes), wasmBytes: wasmBytes.byteLength, semanticDescriptorSha256: sha(semantic), toolchain, sourceTreeDigest: sha("claude-fulls:source:" + pluginId + sha(wasmBytes)), provenanceSha256: sha("claude-fulls:provenance:" + pluginId + sha(wasmBytes))};
  const receipt = (() => { try { return encodeComponentReceiptV1(receiptValue as never); } catch { return `${JSON.stringify(receiptValue, null, 2)}\n`; } })();
  writeFileSync(join(module, "🛂️.descriptor.semio"), descriptor.pack);
  writeFileSync(join(module, "🔣️.json"), descriptor.json);
  writeFileSync(join(module, MODULE_BRIDGE_FILE), pluginComponentBridgeSource(base, entry.wasmOut));
  writeFileSync(join(module, "🧾️receipt.json"), receipt);
  writeFileSync(join(module, ".semio-artifact.json"), JSON.stringify({version: 1, owner: componentModuleOwnerV1(entry.cratePath), files: files(module).filter(f => f !== ".semio-artifact.json")}) + "\n");
  }
  const destination = join(modulesRoot, entry.directoryName), incoming = destination + ".stage";
  rmSync(incoming, {recursive: true, force: true});
  cpSync(module, incoming, {recursive: true});
  rmSync(destination, {recursive: true, force: true});
  renameSync(incoming, destination);
  console.log(`[DEBUG] ${pluginId}: published ${destination} (${files(destination).length} files)`);
}
