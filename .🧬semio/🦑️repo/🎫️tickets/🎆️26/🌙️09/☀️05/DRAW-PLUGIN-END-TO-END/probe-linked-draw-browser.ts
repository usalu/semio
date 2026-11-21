import { createHash } from "node:crypto";
import { mkdir } from "node:fs/promises";
import { join, resolve } from "node:path";
import { verifySemanticDescriptorPairBytesV1 } from "@semio-tech/plugin-registry/catalog-verification/descriptor-pair";

const [wasmPath, descriptorDirectory, outputDirectory, action] = process.argv.slice(2);
if (!wasmPath || !descriptorDirectory || !outputDirectory) {
  throw new Error("Expected linked Draw component, emitted descriptor directory, and generated output directory");
}
const repoRoot = process.cwd();
const browser = join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "🔌️plugin", "🌐️browser-bundle");
const materialization = join(browser, "🏗️materialization");
const { transpilePluginComponentModulesAsync, rewritePreview2ShimImports } = await import(join(materialization, "🟦️.ts"));
const { ensurePreview2ShimVendorAt } = await import(join(materialization, "🩹️preview2-shim-patches", "🟦️.ts"));
const { hostShimSource } = await import(join(materialization, "🪢️host-shim-source", "🟦️.ts"));
const { PLUGIN_HOST_SHIM_FILE } = await import(join(materialization, "🧾️contract", "🟦️.ts"));
const { PLUGIN_DESCRIPTOR_PROBE_SOURCE } = await import(join(browser, "🛂️descriptor", "🟦️.ts"));
const bounded = AbortSignal.timeout(300_000);
const output = resolve(outputDirectory);
const module = join(output, "module");
const vendor = join(output, "vendor");
const name = "semio_hub_draw_component";
const entry = join(module, name + ".js");
const expected = verifySemanticDescriptorPairBytesV1(
  await Bun.file(join(descriptorDirectory, "🔣️.json")).bytes(),
  await Bun.file(join(descriptorDirectory, "🛂️.descriptor.semio")).bytes(),
);
if (expected.pluginId !== "draw") throw new Error("Draw descriptor identity required");
await mkdir(module, { recursive: true });
ensurePreview2ShimVendorAt(vendor, repoRoot);
await transpilePluginComponentModulesAsync(resolve(wasmPath), module, name, { repoRoot, mode: "prototype", preview2VendorDir: vendor, signal: bounded });
rewritePreview2ShimImports(entry, vendor);
await Bun.write(join(module, PLUGIN_HOST_SHIM_FILE), hostShimSource());
if (action === "module-only") {
  console.log(JSON.stringify({ entry, pluginId: expected.pluginId }));
  process.exit(0);
}
const child = Bun.spawn([
  "repo", "tools", "exec", "--light", "node", "--",
  "--v8-pool-size=1", "--experimental-wasm-jspi", "--input-type=module", "--eval", PLUGIN_DESCRIPTOR_PROBE_SOURCE, entry,
], { cwd: repoRoot, stdin: "ignore", stdout: "pipe", stderr: "inherit", signal: bounded });
const [answer, code] = await Promise.all([new Response(child.stdout).text(), child.exited]);
if (code !== 0) throw new Error("Draw browser module descriptor probe exited " + code);
const base64 = answer.trim();
if (base64.length > 12 * 1024 * 1024 || !/^[A-Za-z0-9+/]+={0,2}$/.test(base64)) {
  throw new Error("Browser module answered no bounded base64 descriptor");
}
const semantic = Buffer.from(base64, "base64");
const semanticSha256 = createHash("sha256").update(semantic).digest("hex");
if (semanticSha256 !== expected.semanticSha256) {
  throw new Error("Browser module descriptor differs from the retained component descriptor");
}
await Bun.write(join(output, "semantic.bin"), semantic);
console.log(JSON.stringify({ entry, bytes: semantic.byteLength, semanticSha256 }));
