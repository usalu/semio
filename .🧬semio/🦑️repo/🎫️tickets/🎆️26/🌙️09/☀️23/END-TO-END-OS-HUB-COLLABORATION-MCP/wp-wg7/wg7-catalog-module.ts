/** 🪪️ WG7 (ticket-local) — materializes the EXACT component a trusted catalog published for one plugin into a
 * durable release plugin-module root (`.🧬semio/🌐hub/s11-wg7-catalog-modules`, preamble rule 15), with the product's own materialization steps (transpile, descriptor probe,
 * finalize, bridge), and copies the release root's support directories beside it. A wgpu browser serve pointed at this
 * root (`serve/wg7-serve.ts`) mounts the catalog's own bytes, so the shell's execution-target lease check (component
 * SHA-256 equality) admits the hub document.
 *
 * Usage (repo root): bun .tmp-ticket/wp-wg7/wg7-catalog-module.ts <catalogRoot> <pluginId> <crateName> [durableName]
 */
import { copyFileSync, cpSync, existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { createHash } from "node:crypto";
import { spawnSync } from "node:child_process";
import { hostShimSource, pluginComponentBridgeSource, PLUGIN_HOST_SHIM_FILE, PREVIEW2_VENDOR_RELATIVE, transpilePluginComponentAsync } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts";
import { finalizePluginDescriptor, PLUGIN_DESCRIPTOR_PROBE_SOURCE } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🛂️descriptor/🟦️.ts";
import { fileDigest } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📦️distribution/📋️inventory/🟦️.ts";
import { MODULE_BRIDGE_FILE, moduleDirectoryName } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import { pluginModulesRoot } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts";

const [catalogRoot, pluginId, crateName, durableName = "s11-wg7-catalog-modules"] = process.argv.slice(2);
if (!catalogRoot || !pluginId || !crateName) throw new Error("usage: wg7-catalog-module.ts <catalogRoot> <pluginId> <crateName> [durableName]");
const repo = "/Users/ueli/Documents/semio";
const current = JSON.parse(readFileSync(join(catalogRoot, "trusted-catalog", "current.json"), "utf8"));
const generation = join(catalogRoot, "trusted-catalog", "generations", current.generationId);
const catalog = JSON.parse(readFileSync(join(generation, "trusted-catalog.json"), "utf8"));
const entry = catalog.packages.find((row: { pluginId: string }) => row.pluginId === pluginId);
if (!entry) throw new Error(`catalog ${current.generationId} publishes no ${pluginId}`);
const component = join(generation, entry.component.path);
const sha = createHash("sha256").update(readFileSync(component)).digest("hex");
if (sha !== entry.component.sha256) throw new Error(`catalog component digest mismatch: ${sha} vs ${entry.component.sha256}`);

const shared = pluginModulesRoot("release");
const durable = join(repo, ".🧬semio", "🌐hub", durableName);
const root = join(durable, "release", "🔌️plugin-modules");
rmSync(join(durable, "release"), { recursive: true, force: true });
mkdirSync(root, { recursive: true });
for (const support of ["🪞️vendor", "🧵️shard"]) cpSync(join(shared, support), join(root, support), { recursive: true });
const output = join(root, moduleDirectoryName(pluginId)), componentBase = crateName + "_component";
mkdirSync(output, { recursive: true });
const staged = join(output, crateName + ".wasm");
copyFileSync(component, staged);
writeFileSync(join(output, PLUGIN_HOST_SHIM_FILE), hostShimSource());
console.log(`transpile ${pluginId} (catalog ${current.generationId.slice(0, 16)}, component ${sha.slice(0, 16)})`);
await transpilePluginComponentAsync(staged, output, componentBase, { repoRoot: repo, preview2VendorDir: join(root, PREVIEW2_VENDOR_RELATIVE), optimize: true });
const probe = spawnSync("node", ["--experimental-wasm-jspi", "--input-type=module", "--eval", PLUGIN_DESCRIPTOR_PROBE_SOURCE, join(output, componentBase + ".js")], { cwd: repo, maxBuffer: 12 * 1024 * 1024, encoding: "utf8" });
if (probe.status !== 0) throw new Error(`descriptor probe failed: ${probe.stderr}`);
const descriptor = finalizePluginDescriptor(Buffer.from(probe.stdout.trim(), "base64"), pluginId, await fileDigest(staged), await fileDigest(join(output, componentBase + ".core.wasm")));
rmSync(staged);
writeFileSync(join(output, "🛂️.descriptor.semio"), descriptor.pack);
writeFileSync(join(output, "🔣️.json"), descriptor.json);
writeFileSync(join(output, MODULE_BRIDGE_FILE), pluginComponentBridgeSource(componentBase, crateName + ".wasm"));
const served = JSON.parse(descriptor.json);
console.log(`materialized ${pluginId}: served wasmSha256 ${served.hashes.wasmSha256} == catalog ${entry.component.sha256}: ${served.hashes.wasmSha256 === entry.component.sha256} -> ${output}`);
if (!existsSync(join(root, "🪞️vendor", "🔤️guestslim-typst-fonts.bin"))) throw new Error("support copy lacks the typst font");
