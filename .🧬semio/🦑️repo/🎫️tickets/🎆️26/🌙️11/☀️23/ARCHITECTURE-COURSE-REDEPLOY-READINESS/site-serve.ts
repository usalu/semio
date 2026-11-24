/** 🧊️ Serves the staged CDN directory of the course site like a static CDN on port 6062: files as they are, `index.html` for a directory, `404.html` with status 404 for anything else.
 * Usage (from the repository root): node <this file> */
import { existsSync, readFileSync, statSync } from "node:fs";
import { createServer } from "node:http";
import { extname, join, resolve, sep } from "node:path";

const pages = resolve(import.meta.dirname, "../../../../../../../🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/dist/pages/course");
const types: Readonly<Record<string, string>> = { ".html": "text/html; charset=utf-8", ".js": "text/javascript", ".css": "text/css", ".json": "application/json", ".webmanifest": "application/manifest+json", ".svg": "image/svg+xml", ".png": "image/png", ".ico": "image/x-icon", ".pdf": "application/pdf", ".txt": "text/plain; charset=utf-8", ".wasm": "application/wasm" };
createServer((request, response) => {
  const path = decodeURIComponent(new URL(request.url ?? "/", "http://localhost").pathname);
  let file = resolve(pages, "." + path);
  if (!file.startsWith(pages + sep) && file !== pages) file = join(pages, "404.html");
  if (existsSync(file) && statSync(file).isDirectory()) file = join(file, "index.html");
  const found = existsSync(file);
  if (!found) file = join(pages, "404.html");
  response.statusCode = found ? 200 : 404;
  response.setHeader("content-type", types[extname(file)] ?? "application/octet-stream");
  response.end(readFileSync(file));
}).listen(6062, "127.0.0.1", () => console.log("[site-serve] the staged CDN directory answers on http://127.0.0.1:6062"));
