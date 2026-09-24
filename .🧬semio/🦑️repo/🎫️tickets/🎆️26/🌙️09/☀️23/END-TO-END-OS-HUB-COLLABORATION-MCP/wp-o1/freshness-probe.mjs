import { createServer } from "vite";
import { mkdtempSync, mkdirSync, writeFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const sandbox = mkdtempSync(join(tmpdir(), "semio-fresh-probe-"));
mkdirSync(join(sandbox, "framework"), { recursive: true });
const target = join(sandbox, "framework/x.ts");
writeFileSync(target, "export const value = 0;\n");
console.log("creating server...");
const t0 = Date.now();
const server = await createServer({
  configFile: false,
  root: sandbox,
  logLevel: "silent",
  cacheDir: join(sandbox, ".vite"),
  optimizeDeps: { noDiscovery: true, include: [] },
  server: { host: "127.0.0.1", port: 0, hmr: false, watch: null },
  plugins: [],
});
console.log("listen...", Date.now()-t0);
await server.listen();
const port = server.httpServer.address().port;
console.log("port", port, "fetch...", Date.now()-t0);
const text = await fetch(`http://127.0.0.1:${port}/@fs${target}`).then(r => r.text());
console.log("got", text.slice(0,80), "ms", Date.now()-t0);
await server.close();
rmSync(sandbox, { recursive: true, force: true });
console.log("done");
