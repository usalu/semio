/** 🧪️ Exercises the route probe against the versioned asset shape emitted by the component compiler. */
import assert from "node:assert/strict";
import { execFile } from "node:child_process";
import { createHash } from "node:crypto";
import { createServer } from "node:http";
import { once } from "node:events";
import { promisify } from "node:util";
import { fileURLToPath } from "node:url";

const run = promisify(execFile);
const wasm = Buffer.from([0, 97, 115, 109, 1, 0, 0, 0]);
const assets = new Map([
  ["/", '<script id="semio-plugin-catalog" type="application/json">{"pluginId":"layout","moduleUrl":"/guest/bridge.js?v=layout-current"}</script>'],
  ["/guest/bridge.js?v=layout-current", 'const component = new URL("./layout.js", import.meta.url);'],
  ["/guest/layout.js?v=layout-current", 'const program = fetchCompile(__semioVersionedComponentAssetUrl("./layout.core.wasm"));'],
  ["/guest/layout.core.wasm?v=layout-current", wasm],
  ["/renderer-modules/wgpu/semio-renderer-wgpu-component.js", "export {};"],
  ["/renderer-modules/wgpu/semio-renderer-wgpu-component_bg.wasm", wasm],
]);
const server = createServer((request, response) => {
  const asset = assets.get(request.url);
  if (asset === undefined) {
    response.writeHead(404).end();
    return;
  }
  response.setHeader("cross-origin-opener-policy", "same-origin");
  response.setHeader("cross-origin-embedder-policy", "credentialless");
  response.end(asset);
});
server.listen(0, "127.0.0.1");
await once(server, "listening");
try {
  const entrypoint = fileURLToPath(new URL("./host-route-probe.mjs", import.meta.url));
  const { stdout } = await run(process.execPath, [entrypoint, "http://127.0.0.1:" + server.address().port]);
  const report = JSON.parse(stdout);
  assert.equal(report.kind, "live-host-route-probe");
  assert.equal(report.assets.length, assets.size);
  const paths = report.assets.map(asset => new URL(asset.url).pathname + new URL(asset.url).search);
  assert.deepEqual(new Set(paths), new Set(assets.keys()));
  report.assets.forEach((asset, index) => {
    const expected = assets.get(paths[index]);
    assert.equal(asset.bytes, Buffer.byteLength(expected));
    assert.equal(asset.sha256, createHash("sha256").update(expected).digest("hex"));
  });
  console.log("[DEBUG] Route probe verified the versioned Wasm and renderer requests");
} finally {
  server.close();
}
