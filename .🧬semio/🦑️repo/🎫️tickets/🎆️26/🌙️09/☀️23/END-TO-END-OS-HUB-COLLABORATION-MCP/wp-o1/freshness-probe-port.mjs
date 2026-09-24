import { createServer } from "vite";
import { mkdtempSync, mkdirSync, writeFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
const sandbox = mkdtempSync(join(tmpdir(), "semio-fresh-probe-"));
mkdirSync(join(sandbox, "framework"), { recursive: true });
const target = join(sandbox, "framework/x.ts");
writeFileSync(target, "export const value = 0;\n");
const t0 = Date.now();
const server = await createServer({
  configFile: false, root: sandbox, logLevel: "info",
  cacheDir: join(sandbox, ".vite"),
  optimizeDeps: { noDiscovery: true, include: [] },
  server: { host: "127.0.0.1", port: 6211, strictPort: true, hmr: false, watch: null },
  plugins: [],
});
console.log("created", Date.now()-t0);
const timer = setTimeout(() => { console.error("LISTEN TIMEOUT"); process.exit(2); }, 10000);
await server.listen();
clearTimeout(timer);
const port = server.httpServer.address().port;
console.log("listening", port, Date.now()-t0);
const text = await fetch(`http://127.0.0.1:${port}/@fs${target}`).then(r => r.text());
console.log("body", text.slice(0,60));
await server.close();
rmSync(sandbox, { recursive: true, force: true });
