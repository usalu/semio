/** 🗄️ Static CDN stand-in for the staffage release folder: exact files only, `404.html` with status 404 for misses, no SPA fallback. Usage: `node serve.mjs <dist/release> <port>`. */
import { createServer } from "node:http";
import { readFile, stat } from "node:fs/promises";
import { extname, join, normalize } from "node:path";

const [root, port] = process.argv.slice(2);
const types = { ".html": "text/html; charset=utf-8", ".js": "text/javascript", ".css": "text/css", ".svg": "image/svg+xml", ".json": "application/json", ".webmanifest": "application/manifest+json", ".ico": "image/x-icon", ".woff2": "font/woff2", ".wasm": "application/wasm", ".txt": "text/plain" };
createServer(async (request, response) => {
  const path = normalize(decodeURIComponent(new URL(request.url, "http://x").pathname)).replace(/^\/+/, "");
  const file = join(root, path === "" ? "index.html" : path);
  const found = await stat(file).then((s) => s.isFile(), () => false);
  if (!found) console.log(`404 ${request.url}`);
  response.writeHead(found ? 200 : 404, { "content-type": types[extname(found ? file : ".html")] ?? "application/octet-stream" });
  response.end(await readFile(found ? file : join(root, "404.html")));
}).listen(Number(port), "127.0.0.1", () => console.log(`serving ${root} on http://127.0.0.1:${port}/`));
