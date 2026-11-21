import assert from "node:assert/strict";
import { mkdir, mkdtemp, writeFile } from "node:fs/promises";
import { dirname, join, resolve } from "node:path";
import { createServer } from "vite";
import { resolveAssetDeliveryModeV1 } from "@semio-tech/assets/resolver/dispatch";
import { frameworkOsViteArtifactPathsV1 } from "@semio-tech/framework-os-dev/builder/vite/artifacts";
import { FONT_ASSET } from "@semio-tech/framework-canvas-ts/fonts";
import { repositoryCachePartitionRootDataV1 } from "@semio-tech/repo-lib/storage";

const { issueRepositoryScriptProvidersV1 } = await import(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers/🟦️.ts"));
issueRepositoryScriptProvidersV1(process.env);
const { createWgpuBrowserConfig } = await import(resolve("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/🟦️.ts"));
const fixture = await mkdtemp(resolve(import.meta.dir, "🗑️generated/isolation-"));
const dirs = Object.fromEntries(["root", "compilerRoot", "moduleRoot", "extensionRoot", "bootRoot", "libraryRoot", "workerRoot"].map(name => [name, join(fixture, name)]));
const assets = frameworkOsViteArtifactPathsV1(repositoryCachePartitionRootDataV1(fixture, "products"), "prototype");
const reloadFile = join(fixture, "activation", "receipt.json");
const authored = await Bun.file(join(import.meta.dir, "wgpu-isolation-cases.json")).json();
const content = new Map([
  [reloadFile, "{}"],
  [join(dirs.root, "🌐️.html"), "<!doctype html><title>WGPU isolation fixture</title>"],
  [join(dirs.compilerRoot, "semio-renderer-wgpu-component.js"), "export {};\n"],
  [join(dirs.compilerRoot, "semio-renderer-wgpu-component_bg.wasm"), Buffer.from([0, 97, 115, 109, 1, 0, 0, 0])],
  [join(assets.fonts, FONT_ASSET), "isolation fixture"],
  ...[dirs.bootRoot, dirs.libraryRoot, dirs.workerRoot].map(dir => [join(dir, "🟨️.js"), "export {};\n"]),
]);
for (const [path, bytes] of content) {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, bytes);
}
const config = createWgpuBrowserConfig({
  workspace: fixture,
  ...dirs,
  profile: "prototype",
  catalog: { version: 1, targets: [], hosts: [], playgrounds: [] },
  variant: "layout",
  reloadFile,
  assets: [],
  assetServeMode: resolveAssetDeliveryModeV1(undefined),
});
const server = await createServer({ ...config, configFile: false, logLevel: "silent", server: { ...config.server, host: "127.0.0.1", port: 0, hmr: false, watch: null } });
try {
  await server.listen();
  const port = server.httpServer?.address()?.port;
  assert.ok(port);
  const observations = [];
  for (const request of authored.requests) {
    const response = await fetch(new URL(request.path, `http://127.0.0.1:${port}`), {signal: AbortSignal.timeout(5000)});
    assert.equal(response.status, request.status, request.path);
    observations.push([request.path, ...Object.keys(authored.headers).map(key => response.headers.get(key))]);
    await response.arrayBuffer();
  }
  assert.deepEqual(observations, authored.requests.map(row => [row.path, ...Object.values(authored.headers)]));
  console.log(`${observations.length} real Vite responses carry the WGPU isolation policy`);
} finally {
  await server.close();
}
