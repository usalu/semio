import { createHash } from "node:crypto";
import { cpSync, existsSync, mkdirSync, readdirSync, readFileSync, renameSync, rmSync, statSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { join } from "node:path";
import { discoverPluginBuildTargets } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts";
import { transpilePluginComponentModulesAsync, rewritePreview2ShimImports } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts";
import { PLUGIN_HOST_SHIM_FILE } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧾️contract/🟦️.ts";
import { hostShimSource } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🪢️host-shim-source/🟦️.ts";
import { pluginComponentBridgeSource } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts";
import { PLUGIN_DESCRIPTOR_PROBE_SOURCE, finalizePluginDescriptor } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🛂️descriptor/🟦️.ts";
import { MODULE_BRIDGE_FILE } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import { PREVIEW2_VENDOR_RELATIVE } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🕸️imports/🟦️.ts";
import { developmentRuntimeRoot, nextActivationReceipt, publishActivationReceipt, readActivationReceipt, ACTIVATION_RECEIPT_FILE } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts";
import { EXTENSION_INSTALL_META } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📥️installation/🟦️.ts";
import { MODULE_EXTENSION_ROUTE, MODULE_PLUGIN_ROUTE } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import { rewritePreview2ShimImportSource } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🕸️imports/🟦️.ts";
import { COMPONENT_RECEIPT_FILE, encodeComponentReceiptV1, rustToolchainIdentityV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🟦️.ts";

/** 🪵️ Native stager: materializes built sourcing-closure wasm into the prototype plugin-modules root exactly as the build driver's transpile, assemble, probe and publish stages do, without the repo graph.
 * Usage: `bun native-stage-modules.ts [pluginId…]` from the workspace root after `native-wasm-build.sh`.
 * @see ./🧩️native-activation-2026-11.md */

const root = process.cwd(), ticket = join(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/SOURCING-END-TO-END"), mode = "prototype" as const;
const modules = join(root, ".cache/b/p/m/browser", mode, "modules"), vendor = join(modules, PREVIEW2_VENDOR_RELATIVE);
const lanes = join(root, ".cache/b/r/m/u/l"), built = (base: string) => (existsSync(lanes) ? readdirSync(lanes) : []).map((lane) => join(lanes, lane, "t/wasm32-wasip2/prototype", `${base}.wasm`)).filter(existsSync).sort((a, b) => statSync(b).mtimeMs - statSync(a).mtimeMs)[0];
const wanted = new Set(process.argv.length > 2 ? process.argv.slice(2) : ["sourcing", "sourcing-module-beams", "sourcing-module-slabs", "sourcing-module-windows"]);
const owners = discoverPluginBuildTargets(root).filter((row) => wanted.has(row.pluginId));
const sha = (bytes: Uint8Array | string) => createHash("sha256").update(bytes).digest("hex");
const completed: { pluginId: string; artifactSha256: string }[] = [];
const environment = { ...process.env, SEMIO_BUILD_MODE: mode };
for (const owner of owners) {
  const base = owner.packageName.replaceAll("-", "_"), wasm = built(base), directory = owner.directoryName!;
  if (!wasm) throw Error(`missing built component ${base}.wasm in the repo cargo lanes under ${lanes}`);
  const work = join(ticket, "🗑️generated/native-stage", directory), out = join(work, "transpiled"), stage = join(modules, `.stage-${directory}`);
  rmSync(work, { recursive: true, force: true }); rmSync(stage, { recursive: true, force: true }); mkdirSync(work, { recursive: true }); mkdirSync(stage, { recursive: true });
  console.log(`== ${owner.pluginId} -> ${directory}`);
  await transpilePluginComponentModulesAsync(wasm, out, base, { repoRoot: root, mode, preview2VendorDir: vendor, environment });
  cpSync(out, stage, { recursive: true });
  writeFileSync(join(stage, PLUGIN_HOST_SHIM_FILE), hostShimSource());
  rewritePreview2ShimImports(join(stage, `${base}.js`), vendor);
  const probe = spawnSync("node", ["--experimental-wasm-jspi", "--input-type=module", "--eval", PLUGIN_DESCRIPTOR_PROBE_SOURCE, join(stage, `${base}.js`)], { cwd: root, env: environment, encoding: "utf8", maxBuffer: 1 << 28 });
  if (probe.status !== 0) throw Error(`describe() probe failed: ${probe.stderr}`);
  const semantic = Buffer.from(probe.stdout.trim(), "base64"), descriptor = finalizePluginDescriptor(semantic, owner.pluginId), bytes = readFileSync(wasm);
  const receipt = { schema: "semio.plugin.component-receipt/v1", pluginId: owner.pluginId, packageId: owner.packageId, profile: mode, target: "wasm32-wasip2", wasmSha256: sha(bytes), wasmBytes: bytes.length, semanticDescriptorSha256: sha(semantic), toolchain: rustToolchainIdentityV1(root), sourceTreeDigest: sha(`native-activation:source:${sha(bytes)}`), provenanceSha256: sha(`native-activation:provenance:${sha(bytes)}`) };
  writeFileSync(join(stage, "🛂️.descriptor.semio"), descriptor.pack); writeFileSync(join(stage, "🔣️.json"), descriptor.json);
  writeFileSync(join(stage, MODULE_BRIDGE_FILE), pluginComponentBridgeSource(base, owner.wasmOut)); writeFileSync(join(stage, COMPONENT_RECEIPT_FILE), encodeComponentReceiptV1(receipt as never));
  const committed = join(root, owner.cratePath.replace(/\/📦️packages\/.*$/u, ""), "🛂️.descriptor.semio");
  console.log(`  wasm ${bytes.length} sha ${receipt.wasmSha256}; descriptor ${descriptor.pack.length} bytes; committed ${existsSync(committed) ? (sha(readFileSync(committed)) === sha(descriptor.pack) ? "MATCH" : "DIFFERS") : "absent"}`);
  const finalDir = join(modules, directory); rmSync(finalDir, { recursive: true, force: true }); renameSync(stage, finalDir);
  if (owner.role === "extension") {
    const installed = join(developmentRuntimeRoot(root, "sourcing", mode, "react"), "extensions", directory);
    rmSync(installed, { recursive: true, force: true }); cpSync(finalDir, installed, { recursive: true });
    for (const name of readdirSync(installed)) if (name.endsWith(".js")) writeFileSync(join(installed, name), rewritePreview2ShimImportSource(readFileSync(join(finalDir, name), "utf8"), `../..${MODULE_PLUGIN_ROUTE}/${PREVIEW2_VENDOR_RELATIVE}/`));
    writeFileSync(join(installed, EXTENSION_INSTALL_META), JSON.stringify({ extensionId: owner.pluginId, directoryName: directory, version: "0.0.0-dev", label: owner.pluginId, extends: owner.extends ?? "", moduleUrl: `${MODULE_EXTENSION_ROUTE}/${directory}/${MODULE_BRIDGE_FILE}`, packageHash: sha(readFileSync(join(finalDir, MODULE_BRIDGE_FILE))), installedAt: Date.now() }) + "\n");
  }
  completed.push({ pluginId: owner.pluginId, artifactSha256: sha(readFileSync(join(finalDir, MODULE_BRIDGE_FILE))) });
  console.log(`  staged ${finalDir}: ${readdirSync(finalDir).map((n) => `${n}(${statSync(join(finalDir, n)).size})`).join(" ")}`);
}
const receiptDirectory = join(developmentRuntimeRoot(root, "sourcing", mode, "react"), "activation");
mkdirSync(receiptDirectory, { recursive: true });
const previous = existsSync(join(receiptDirectory, ACTIVATION_RECEIPT_FILE)) ? readActivationReceipt(receiptDirectory) : undefined;
publishActivationReceipt(receiptDirectory, nextActivationReceipt("sourcing", mode, [...(previous?.plugins.filter((row) => !completed.some((c) => c.pluginId === row.pluginId)) ?? []), ...completed], previous));
console.log(`activation receipt published in ${receiptDirectory}`);
