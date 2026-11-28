import { cp, mkdir, readdir, readFile, rm, writeFile } from "node:fs/promises";
import { createHash } from "node:crypto";
import { join } from "node:path";
import { existsSync } from "node:fs";
import { Buffer } from "node:buffer";
import { isDeepStrictEqual } from "node:util";

/** 📦️ Stages the built components of the PDF plugin for the native dev host with the browser-bundle pipeline's own transpile steps (native transpile into an instantiation factory, async-result and factory WASI-import rewrites, preview2 shim relocation, every extracted core module), one factory per component the plugin descriptor declares (`PackageDescriptor.components`), beside one bridge that imports only the component an activation names. The staged descriptor is the whole plugin's, the pair the native `committed_descriptor` test described from `semio_hub_stdio_pdf::plugin()` (checked as a pack and its JSON, not against the catalog's 4 MiB commit bound, which that 4.5 MB pair exceeds until the action tables are shared); every component's own `describe()` must be exactly that descriptor restricted to its apps, so no component ships a declaration the plugin descriptor does not make. Refreshes the shard worker the bridge drives.
 * Usage: `bun stage-pdf-component.ts <wasm directory> <work directory> <described directory>` from the workspace root; the wasm directory holds `<id with _>_component.wasm` for every declared component id.
 * @see ./🌐️pdf-browser-journey.md */

const root = process.cwd();
const [wasmDirectory, work, described] = process.argv.slice(2) as [string, string, string];
const modules = join(root, ".cache/b/p/m/browser/prototype/modules");
const stage = join(modules, "📘️stdio-pdf");
const vendor = join(modules, "🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim");
const materialization = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization");
const { transpilePluginComponentModulesAsync, rewritePreview2ShimImports } = await import(join(materialization, "🟦️.ts"));
const { pluginComponentBridgeSource } = await import(join(materialization, "🌉️component-bridge-source/🟦️.ts"));
const { hostShimSource } = await import(join(materialization, "🪢️host-shim-source/🟦️.ts"));
const { PLUGIN_DESCRIPTOR_PROBE_SOURCE, finalizePluginDescriptor } = await import(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🛂️descriptor/🟦️.ts"));
if (!existsSync(stage)) throw new Error(`no staged module at ${stage}: attach the products image first (sh mount-products.sh)`);

const pairJson = await readFile(join(described, "🔣️.json")), pairPack = await readFile(join(described, "🛂️.descriptor.semio"));
const pairFinal = finalizePluginDescriptor(pairPack, "stdio-pdf");
const plugin = JSON.parse(pairFinal.json) as { components?: { id: string; apps: string[] }[]; manifest: { pluginId: string; apps: { id: string }[] } };
if (!isDeepStrictEqual(JSON.parse(pairJson.toString("utf8")), plugin)) throw new Error("the described stdio-pdf JSON is not its pack form");
if (!plugin.components?.length) throw new Error("the described stdio-pdf descriptor declares no components");
const components = plugin.components.map(row => ({ ...row, base: `${row.id.replaceAll("-", "_")}_component` }));

const { resolveManifestActionSets } = await import(join(root, "🧰️framework/🔨️modules/🛂️manifest/🧺️action-sets/🟦️.ts"));
/** 🧺️ The apps as a host resolves them (plugin action sets joined into each roster, exactly as the descriptor loader does), each roster ordered by action id: a shard factors its own apps' shared rows, the plugin descriptor factors all of them, and both resolve to the same rosters. */
const resolvedApps = (value: typeof plugin): { id: string; actions?: { id: string }[] }[] => (resolveManifestActionSets(value.manifest as never) as typeof plugin["manifest"]).apps.map(app => ({ ...app, actions: [...((app as { actions?: { id: string }[] }).actions ?? [])].sort((left, right) => left.id < right.id ? -1 : left.id > right.id ? 1 : 0) }));

/** 🔎️ The first JSON path where two descriptor values differ. */
function firstDifference(left: unknown, right: unknown, path = "$"): string | null {
  if (isDeepStrictEqual(left, right)) return null;
  if (left && right && typeof left === "object" && typeof right === "object") {
    for (const key of new Set([...Object.keys(left), ...Object.keys(right)])) {
      const found = firstDifference((left as Record<string, unknown>)[key], (right as Record<string, unknown>)[key], `${path}.${key}`);
      if (found) return found;
    }
  }
  return `${path}: ${JSON.stringify(left)?.slice(0, 160)} != ${JSON.stringify(right)?.slice(0, 160)}`;
}

await rm(work, { recursive: true, force: true });
for (const name of await readdir(stage)) if (/\.core\d*\.wasm$/u.test(name) || (/_component\.(js|d\.ts)$/u.test(name))) await rm(join(stage, name));
await writeFile(join(stage, "🟨️.js"), hostShimSource());
const staged: { id: string; base: string; bytes: number; sha256: string }[] = [];
for (const component of components) {
  const directory = join(work, component.id), stripped = join(directory, "component.wasm"), out = join(directory, "out");
  await mkdir(out, { recursive: true });
  const proc = Bun.spawnSync(["node", join(import.meta.dir, "strip-component-names.mjs"), join(wasmDirectory, `${component.base}.wasm`), stripped], { stdout: "inherit", stderr: "inherit" });
  if (proc.exitCode !== 0) throw new Error(`strip failed: ${component.id}`);
  await transpilePluginComponentModulesAsync(stripped, out, component.base, { repoRoot: root, mode: "prototype", environment: process.env, preview2VendorDir: vendor });
  for (const file of (await readdir(out)).filter(name => name === `${component.base}.js` || name === `${component.base}.d.ts` || new RegExp(`^${component.base}\\.core\\d*\\.wasm$`, "u").test(name))) await cp(join(out, file), join(stage, file));
  rewritePreview2ShimImports(join(stage, `${component.base}.js`), vendor);
  if (staged.length === 0) await cp(join(out, "interfaces"), join(stage, "interfaces"), { recursive: true });
  const probe = Bun.spawnSync(["node", "--experimental-wasm-jspi", "--input-type=module", "--eval", PLUGIN_DESCRIPTOR_PROBE_SOURCE, join(stage, `${component.base}.js`)], { cwd: root, stdout: "pipe", stderr: "inherit", maxBuffer: 64 * 1024 * 1024 });
  if (probe.exitCode !== 0) throw new Error(`describe() probe failed: ${component.id}`);
  const own = JSON.parse(finalizePluginDescriptor(Buffer.from(probe.stdout.toString().trim(), "base64"), plugin.manifest.pluginId).json) as typeof plugin;
  const expected = { ...plugin, components: [{ id: component.id, apps: component.apps }], manifest: { ...plugin.manifest, apps: plugin.manifest.apps.filter(app => component.apps.includes(app.id)) } };
  const identity = (value: typeof plugin) => ({ ...Object.fromEntries(["descriptorVersion", "packageId", "role", "activationEvents", "capabilityRequests", "execution", "executionProtocol", "quotas"].map(key => [key, (value as Record<string, unknown>)[key]])), manifest: Object.fromEntries(["pluginId", "label", "version", "dependencies", "hostedArtifactKinds"].map(key => [key, (value.manifest as Record<string, unknown>)[key]])) });
  const manifestDifference = firstDifference(resolvedApps(own), resolvedApps(expected), "$.manifest.apps") ?? firstDifference(own.components, expected.components, "$.components") ?? firstDifference(identity(own), identity(expected), "$");
  if (manifestDifference) throw new Error(`component ${component.id} describes apps the plugin descriptor does not: ${manifestDifference}`);
  const bytes = await readFile(stripped);
  staged.push({ id: component.id, base: component.base, bytes: bytes.byteLength, sha256: createHash("sha256").update(bytes).digest("hex") });
}
await writeFile(join(stage, "🌉️bridge.js"), pluginComponentBridgeSource(components[0]!.base, Object.fromEntries(components.map(component => [component.id, component.base]))));
const { MODULE_SHARD_DIRECTORY } = await import(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts"));
const { SHARD_WORKER_FILE } = await import(join(materialization, "🧾️contract/🟦️.ts"));
const { shardWorkerSource } = await import(join(materialization, "🧵️shard-worker-source/🟦️.ts"));
await mkdir(join(modules, MODULE_SHARD_DIRECTORY), { recursive: true });
await writeFile(join(modules, MODULE_SHARD_DIRECTORY, SHARD_WORKER_FILE), shardWorkerSource());
const descriptor = finalizePluginDescriptor(pairPack, plugin.manifest.pluginId);
await writeFile(join(stage, "🛂️.descriptor.semio"), descriptor.pack);
await writeFile(join(stage, "🔣️.json"), descriptor.json);
const receiptPath = join(stage, "🧾️receipt.json");
const receipt = JSON.parse(await readFile(receiptPath, "utf8"));
receipt.wasmSha256 = staged[0]!.sha256;
receipt.wasmBytes = staged[0]!.bytes;
receipt.semanticDescriptorSha256 = createHash("sha256").update(descriptor.pack).digest("hex");
await writeFile(receiptPath, `${JSON.stringify(receipt, null, 2)}\n`);
const manifestPath = join(stage, ".semio-artifact.json");
const manifest = JSON.parse(await readFile(manifestPath, "utf8"));
const present = await readdir(stage);
manifest.files = [...new Set([...manifest.files.filter((name: string) => !/\.core\d*\.wasm$/u.test(name) && !/_component\.(js|d\.ts)$/u.test(name)), ...present.filter(name => components.some(component => name === `${component.base}.js` || name === `${component.base}.d.ts` || new RegExp(`^${component.base}\\.core\\d*\\.wasm$`, "u").test(name)))])].filter(name => existsSync(join(stage, name))).sort();
await writeFile(manifestPath, JSON.stringify(manifest));
for (const row of staged) console.log(`staged ${row.id}: ${row.base} (${row.bytes} bytes, sha256 ${row.sha256})`);
console.log(`descriptor ${descriptor.json.length} JSON bytes, ${descriptor.pack.byteLength} pack bytes, ${components.length} components`);
