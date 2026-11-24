/** 🧪️ Native fallback publication of the FEM browser module: transpiles a fresh `semio_hub_fem_component.wasm` with the repo's own native
 * transpile and jco module rewrites and regenerates the finalized descriptor pair, bridge, host shim and receipt digest of the published module.
 * Usage: bun native-fem-materialize.ts <component.wasm> */
import { createHash } from "node:crypto";
import { cpSync, mkdirSync, readFileSync, readdirSync, rmSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const materialization = "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization";
const { rewritePreview2ShimImports } = await import(`${materialization}/🟦️.ts`);
const { nativeTranspileArgsV1, nativeTranspileBinaryV1, PLUGIN_COMPONENT_IMPORT_MAPS_V1 } = await import(`${materialization}/🦀️native-transpile/🟦️.ts`);
const { pluginComponentBridgeSource } = await import(`${materialization}/🌉️component-bridge-source/🟦️.ts`);
const { hostShimSource } = await import(`${materialization}/🪢️host-shim-source/🟦️.ts`);
const { finalizePluginDescriptor } = await import(`${materialization}/../🛂️descriptor/🟦️.ts`);
const { rewriteJcoAsyncResultLiftingAt, rewriteJcoComponentAssetUrlsAt } = await import(`${materialization}/🧬️jco-module-rewrites/🟦️.ts`);

const repoRoot = "C:/semio", base = "semio_hub_fem_component", artifact = process.argv[2]!;
const modules = join(repoRoot, ".cache/b/p/m/browser/prototype/modules"), published = join(modules, "🏗️fem");
const out = join(import.meta.dir, "🗑️generated", "fem-transpile");
const owner = join(repoRoot, "🌎️hub/🧩️compositions/🏗️fem/🧩️component");
rmSync(out, { recursive: true, force: true });
mkdirSync(out, { recursive: true });
const transpile = Bun.spawnSync([nativeTranspileBinaryV1(repoRoot, "prototype"), ...nativeTranspileArgsV1(artifact, out, base, PLUGIN_COMPONENT_IMPORT_MAPS_V1)], { stdout: "inherit", stderr: "inherit" });
if (transpile.exitCode !== 0) throw Error(`native transpile failed with ${transpile.exitCode}`);
for (const rewrite of [rewriteJcoAsyncResultLiftingAt, rewriteJcoComponentAssetUrlsAt]) rewrite(join(out, `${base}.js`));
const produced = readdirSync(out, { recursive: true }).map(String).filter(path => statSync(join(out, path)).isFile());
for (const file of readdirSync(published)) if (file.startsWith(`${base}.core`) && file.endsWith(".wasm")) rmSync(join(published, file));
rmSync(join(published, "interfaces"), { recursive: true, force: true });
cpSync(out, published, { recursive: true });
rewritePreview2ShimImports(join(published, `${base}.js`), join(modules, "🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim"));
const semantic = readFileSync(join(owner, "🛂️.descriptor.semio")), descriptor = finalizePluginDescriptor(semantic, "fem");
writeFileSync(join(published, "🛂️.descriptor.semio"), descriptor.pack);
writeFileSync(join(published, "🔣️.json"), descriptor.json);
writeFileSync(join(published, "🌉️bridge.js"), pluginComponentBridgeSource(base, `${base}.wasm`));
writeFileSync(join(published, "🟨️.js"), hostShimSource());
const bytes = readFileSync(artifact), receiptPath = join(published, "🧾️receipt.json"), receipt = JSON.parse(readFileSync(receiptPath, "utf8"));
receipt.wasmSha256 = createHash("sha256").update(bytes).digest("hex");
receipt.wasmBytes = bytes.byteLength;
receipt.semanticDescriptorSha256 = createHash("sha256").update(semantic).digest("hex");
writeFileSync(receiptPath, JSON.stringify(receipt, null, 2) + "\n");
const manifestPath = join(published, ".semio-artifact.json"), manifest = JSON.parse(readFileSync(manifestPath, "utf8"));
manifest.files = [...new Set([...manifest.files.filter((file: string) => !file.startsWith("interfaces/") && !(file.startsWith(`${base}.core`) && file.endsWith(".wasm"))), ...produced.map(path => path.replaceAll("\\", "/"))])].sort();
writeFileSync(manifestPath, JSON.stringify(manifest));
console.log(JSON.stringify({ produced: produced.length, wasmSha256: receipt.wasmSha256, wasmBytes: receipt.wasmBytes }));
