import { cp, mkdir, readFile, rm, writeFile } from "node:fs/promises";
import { createHash } from "node:crypto";
import { join } from "node:path";
import { existsSync } from "node:fs";
import { Buffer } from "node:buffer";

/** 📦️ Stages a built PDF plugin component for the native dev host with the browser-bundle pipeline's own transpile steps (native transpile, async-result and asset-URL rewrites, preview2 shim relocation) and refreshes the staged semantic descriptor (`🛂️.descriptor.semio`, `🔣️.json`) from the module's own `describe()`, so the served manifest (fault notices, actions) is the built guest's.
 * Usage: `bun stage-pdf-component.ts <component.wasm> <work-dir>` from the workspace root.
 * @see ./🌐️pdf-browser-journey.md */

const root = process.cwd();
const [component, work] = process.argv.slice(2) as [string, string];
const base = "semio_hub_stdio_pdf_component";
const modules = join(root, ".cache/b/p/m/browser/prototype/modules");
const stage = join(modules, "📘️stdio-pdf");
const materialization = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization");
const { transpilePluginComponentModulesAsync, rewritePreview2ShimImports } = await import(join(materialization, "🟦️.ts"));
const { pluginComponentBridgeSource } = await import(join(materialization, "🌉️component-bridge-source/🟦️.ts"));
const { hostShimSource } = await import(join(materialization, "🪢️host-shim-source/🟦️.ts"));
if (!existsSync(stage)) throw new Error(`no staged module at ${stage}: attach the products image first (sh mount-products.sh)`);
const stripped = join(work, "component.wasm");
const out = join(work, "out");
await rm(work, { recursive: true, force: true });
await mkdir(out, { recursive: true });
const proc = Bun.spawnSync(["node", join(import.meta.dir, "strip-component-names.mjs"), component, stripped], { stdout: "inherit", stderr: "inherit" });
if (proc.exitCode !== 0) throw new Error("strip failed");
await transpilePluginComponentModulesAsync(stripped, out, base, { repoRoot: root, mode: "prototype", environment: process.env, preview2VendorDir: join(modules, "🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim") });
for (const file of [`${base}.core.wasm`, `${base}.js`, `${base}.d.ts`]) await cp(join(out, file), join(stage, file));
rewritePreview2ShimImports(join(stage, `${base}.js`), join(modules, "🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim"));
await cp(join(out, "interfaces"), join(stage, "interfaces"), { recursive: true });
await writeFile(join(stage, "🌉️bridge.js"), pluginComponentBridgeSource(base, `${base}.core.wasm`));
await writeFile(join(stage, "🟨️.js"), hostShimSource());
const { PLUGIN_DESCRIPTOR_PROBE_SOURCE, finalizePluginDescriptor } = await import(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🛂️descriptor/🟦️.ts"));
const probe = Bun.spawnSync(["node", "--experimental-wasm-jspi", "--input-type=module", "--eval", PLUGIN_DESCRIPTOR_PROBE_SOURCE, join(stage, `${base}.js`)], { cwd: root, stdout: "pipe", stderr: "inherit", maxBuffer: 64 * 1024 * 1024 });
if (probe.exitCode !== 0) throw new Error("describe() probe failed");
const descriptor = finalizePluginDescriptor(Buffer.from(probe.stdout.toString().trim(), "base64"), JSON.parse(await readFile(join(stage, "🔣️.json"), "utf8")).manifest.pluginId);
await writeFile(join(stage, "🛂️.descriptor.semio"), descriptor.pack);
await writeFile(join(stage, "🔣️.json"), descriptor.json);
const bytes = await readFile(stripped);
const receiptPath = join(stage, "🧾️receipt.json");
const receipt = JSON.parse(await readFile(receiptPath, "utf8"));
receipt.wasmSha256 = createHash("sha256").update(bytes).digest("hex");
receipt.wasmBytes = bytes.byteLength;
receipt.semanticDescriptorSha256 = createHash("sha256").update(descriptor.pack).digest("hex");
await writeFile(receiptPath, `${JSON.stringify(receipt, null, 2)}\n`);
console.log(`staged ${stage} (${bytes.byteLength} bytes, sha256 ${receipt.wasmSha256})`);
