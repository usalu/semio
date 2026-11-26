import { createServer } from "node:http";
import { createReadStream } from "node:fs";
import { join } from "node:path";
const base = join(import.meta.dirname, "🗑️generated/bridge-current/bindings-gate-debug");
createServer((request, response) => {
  const name = new URL(request.url, "http://localhost").pathname.slice(1);
  if (!/^semio-renderer-wgpu-component(?:_bg\.wasm|\.js)$/.test(name)) { response.writeHead(404).end(); return; }
  response.writeHead(200, { "access-control-allow-origin": "*", "content-type": name.endsWith(".wasm") ? "application/wasm" : "text/javascript" });
  createReadStream(join(base, name)).pipe(response);
}).listen(6121, "127.0.0.1");
