import { readFile } from "node:fs/promises";
import { createServer } from "node:http";
import { extname, isAbsolute, relative, resolve, sep } from "node:path";
import { pathToFileURL } from "node:url";

const types = {
  ".css": "text/css", ".fbx": "application/octet-stream", ".glb": "model/gltf-binary", ".html": "text/html; charset=utf-8",
  ".ico": "image/x-icon", ".js": "text/javascript", ".json": "application/json", ".png": "image/png",
  ".svg": "image/svg+xml", ".txt": "text/plain", ".wasm": "application/wasm", ".webmanifest": "application/manifest+json",
  ".woff": "font/woff", ".woff2": "font/woff2",
};

/** 🗄️ Serves exact release files and a real 404 page over loopback, as a static host does. */
export async function serve(root, port = 0) {
  const directory = resolve(root);
  const missing = await readFile(resolve(directory, "404.html"));
  const server = createServer(async (request, response) => {
    try {
      const pathname = decodeURIComponent(new URL(request.url, "http://localhost").pathname);
      const file = resolve(directory, "." + (pathname === "/" ? "/index.html" : pathname));
      const path = relative(directory, file);
      if (isAbsolute(path) || path === ".." || path.startsWith(".." + sep)) {
        response.writeHead(400).end();
        return;
      }
      const content = await readFile(file).catch((error) => {
        if (!["ENOENT", "ENOTDIR", "EISDIR"].includes(error.code)) throw error;
        return undefined;
      });
      response.writeHead(content === undefined ? 404 : 200, {
        "Content-Type": types[content === undefined ? ".html" : extname(file)] ?? "application/octet-stream",
      }).end(content ?? missing);
    } catch (error) {
      response.writeHead(error instanceof URIError ? 400 : 500).end();
    }
  });
  await new Promise((ready, fail) => {
    server.once("error", fail);
    server.listen(port, "127.0.0.1", ready);
  });
  return { server, url: `http://127.0.0.1:${server.address().port}/` };
}

if (process.argv[1] && pathToFileURL(resolve(process.argv[1])).href === import.meta.url) {
  const [root, port] = process.argv.slice(2);
  if (!root) throw new Error("Usage: node serve.mjs <release-directory> [port]");
  console.log(`[DEBUG] Serving ${root} at ${(await serve(root, port === undefined ? 0 : Number(port))).url}`);
}
