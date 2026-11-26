import { strict as assert } from "node:assert";
import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import { join } from "node:path";
import { createServer } from "node:http";
import { once } from "node:events";
import { installedProceduralVite } from "./installed-procedural-vite.ts";
import { developmentRuntimeRoot } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts";

const scratch = await mkdtemp(join(import.meta.dir, "🗑️generated", "host-test-"));
const paths = "/🔌️plugin-modules/🌀️procedural/";
const file = "semio_hub_procedural_component.core.wasm";
const bytes = Uint8Array.of(0, 97, 115, 109, 1, 0, 0, 0);
try {
  for (const renderer of ["react", "wgpu"] as const) {
    const root = join(developmentRuntimeRoot(scratch, "generation3d", "prototype", renderer), "components", "🌀️procedural");
    await mkdir(root, { recursive: true });
    await writeFile(join(root, file), bytes);
    await writeFile(join(root, ".semio-artifact.json"), JSON.stringify({ version: 1, owner: "development-component:procedural", files: [file] }));
    const server = createServer();
    const plugin = installedProceduralVite(renderer, scratch);
    assert.equal(typeof plugin.configureServer, "function");
    (plugin.configureServer as Function)({ middlewares: { use: (handle: Function) => server.on("request", (req, res) => handle(req, res, () => res.writeHead(418).end())) } });
    server.listen(0, "127.0.0.1");
    await once(server, "listening");
    try {
      const address = server.address();
      assert.ok(address && typeof address === "object");
      const url = "http://127.0.0.1:" + address.port;
      const response = await fetch(url + paths + file);
      assert.equal(response.status, 200);
      assert.equal(response.headers.get("content-type"), "application/wasm");
      assert.deepEqual(new Uint8Array(await response.arrayBuffer()), bytes);
      const head = await fetch(url + paths + file, { method: "HEAD" });
      assert.equal(head.headers.get("content-length"), String(bytes.length));
      assert.equal((await head.arrayBuffer()).byteLength, 0);
      assert.equal((await fetch(url + paths + file, { method: "POST" })).status, 405);
      assert.equal((await fetch(url + paths + ".semio-artifact.json")).status, 404);
      assert.equal((await fetch(url + "/index.html")).status, 418);
    } finally { server.closeAllConnections(); await new Promise<void>(resolve => server.close(() => resolve())); }
  }
} finally { await rm(scratch, { recursive: true, force: true }); }
console.log("[DEBUG] Verified installed module GET, HEAD, inventory and delegation on both native development hosts");
