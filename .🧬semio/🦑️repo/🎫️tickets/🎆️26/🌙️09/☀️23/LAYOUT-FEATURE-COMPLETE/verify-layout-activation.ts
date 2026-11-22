import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { createReadStream } from "node:fs";
import { readFile, stat } from "node:fs/promises";
import { basename, dirname, join, relative, resolve, sep } from "node:path";
import { pathToFileURL } from "node:url";
import { artifactFiles } from "@semio-tech/framework-plugin/browser-bundle/distribution/inventory";
import { PREVIEW2_VENDOR_RELATIVE } from "@semio-tech/framework-plugin/browser-bundle/imports";
import { FONT_ASSET } from "@semio-tech/framework-canvas-ts/fonts";
import { componentOwnerRootV1, MODULE_SHARD_DIRECTORY } from "@semio-tech/plugin-registry/deployment";
import { componentSourceContentHash, readStagedSourceContentHash } from "@semio-tech/framework-plugin/build/freshness";

/** 🔐️ Recomputes the activated inventory hash from owned published files. */
async function digest(files: ReadonlyMap<string, string>, indexed: boolean) {
  const hash = createHash("sha256");
  let bytes = 0;
  for (const name of [...files.keys()].sort()) {
    const file = files.get(name)!;
    const before = await stat(file);
    if (indexed) hash.update(JSON.stringify([name.replaceAll("\\", "/"), before.size]) + "\n");
    let read = 0;
    for await (const chunk of createReadStream(file)) {
      hash.update(chunk);
      read += chunk.length;
    }
    const after = await stat(file);
    assert.equal(read, before.size, file);
    assert.deepEqual([after.ino, after.size, after.mtimeMs], [before.ino, before.size, before.mtimeMs], file);
    bytes += read;
  }
  return { sha256: hash.digest("hex"), bytes };
}

const [receiptFile, modulesDirectory, fontDirectory, rendererDirectory, routeEvidenceFile, ownerManifestFile] = process.argv.slice(2).map(path => resolve(path));
assert.ok(receiptFile && modulesDirectory && fontDirectory && rendererDirectory && routeEvidenceFile, "Expected the owned activation, module, font, renderer and HTTP paths");
const receiptPath = resolve(receiptFile);
const receiptBytes = await readFile(receiptPath);
const receipt = JSON.parse(receiptBytes.toString("utf8"));
assert.equal(receipt.schema, "semio.dev.activation/v1");
assert.equal(receipt.variant, "layout");
assert.equal(receipt.profile, "prototype");
assert.deepEqual(receipt.plugins.map(({ pluginId }) => pluginId), ["layout"]);
const sourceContentSha256 = receipt.plugins[0].sourceContentSha256;
assert.match(sourceContentSha256, /^[0-9a-f]{64}$/u);
if (ownerManifestFile) {
  const runtime = dirname(dirname(receiptPath));
  const marker = join(runtime, "🔖️", createHash("sha256").update("layout").digest("hex").slice(0, 24));
  assert.equal(readStagedSourceContentHash(marker), sourceContentSha256);
  const cratePath = relative(process.cwd(), dirname(ownerManifestFile)).split(sep).join("/");
  assert.equal(await componentSourceContentHash(resolve(componentOwnerRootV1(cratePath))), sourceContentSha256);
}

const support = new Map<string, string>();
for (const directory of [PREVIEW2_VENDOR_RELATIVE, MODULE_SHARD_DIRECTORY]) {
  for (const [name, path] of artifactFiles(join(modulesDirectory, directory))) support.set(join(directory, name), path);
}
support.set(FONT_ASSET, join(fontDirectory, FONT_ASSET));
const module = join(modulesDirectory, "📏️layout");
const supportDigest = await digest(support, true);
const actorDigest = await digest(artifactFiles(module), true);
const activationHash = createHash("sha256").update(supportDigest.sha256 + actorDigest.sha256).digest("hex");
assert.equal(activationHash, receipt.plugins[0].artifactSha256);
assert.deepEqual(await readFile(receiptPath), receiptBytes, "Activation changed while hashing");

const report = JSON.parse(await readFile(routeEvidenceFile, "utf8"));
assert.equal(report.layout, "layout");
const documentUrl = new URL(report.assets[0].url);
const documentResponse = await fetch(documentUrl, { redirect: "error", signal: AbortSignal.timeout(15_000) });
assert.equal(documentResponse.status, 200);
const catalogScript = (await documentResponse.text()).match(/<script\b(?=[^>]*\bid=["']semio-plugin-catalog["'])[^>]*>([\s\S]*?)<\/script>/i);
assert.ok(catalogScript, "Serving catalog required");
const { composeSpecificOsCatalogV1 } = await import(pathToFileURL(resolve("✏️s/🧑‍💻dev/🧩️catalog/🟦️.ts")).href);
const projected = composeSpecificOsCatalogV1(new URL("/", documentUrl).href, {
  maxBytes: 2_097_152,
  maxRows: 512,
  maxEdges: 4_096,
  maxWork: 65_536,
  deadlineMs: performance.now() + 30_000,
  now: () => performance.now(),
  cancelled: () => false,
  progress: () => {},
}).rows;
assert.deepEqual(JSON.parse(catalogScript[1]), JSON.parse(JSON.stringify(projected)), "live WGPU catalog projection");

const assets = [
  [module, "semio_hub_layout_component.core.wasm"],
  [rendererDirectory, "semio-renderer-wgpu-component_bg.wasm"],
] as const;
for (const [directory, name] of assets) {
  const responses = report.assets.filter(asset => basename(decodeURIComponent(new URL(asset.url).pathname)) === name);
  assert.equal(responses.length, 1, name);
  const { bytes, sha256 } = await digest(new Map([[name, join(directory, name)]]), false);
  assert.equal(responses[0].bytes, bytes, name);
  assert.equal(responses[0].sha256, sha256, name);
}
console.log(JSON.stringify({ activationHash, sourceContentSha256: receipt.plugins[0].sourceContentSha256, supportDigest, actorDigest, comparedWasmResponses: assets.length, currentCatalog: true }));
