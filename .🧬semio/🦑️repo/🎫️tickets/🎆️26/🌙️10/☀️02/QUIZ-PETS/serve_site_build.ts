#!/usr/bin/env bun
/** 📡️ Serves a built quiz site from a directory as a static origin, the way a CDN would: `index.html` at `/`, every other
 * file by its path, `404.html` otherwise. A ticket tool for looking at a private release build under its real
 * Content-Security-Policy. Usage: `bun serve_site_build.ts <dist> [port]` (default port 6194, loopback only). */
import { existsSync, readFileSync, statSync } from "node:fs";
import { extname, join, normalize } from "node:path";

const dist = process.argv[2]!;
const port = Number(process.argv[3] ?? "6194");
const TYPES: Readonly<Record<string, string>> = { ".html": "text/html; charset=utf-8", ".js": "text/javascript; charset=utf-8", ".css": "text/css; charset=utf-8", ".svg": "image/svg+xml", ".json": "application/json", ".webmanifest": "application/manifest+json", ".woff2": "font/woff2", ".ico": "image/x-icon", ".txt": "text/plain; charset=utf-8" };

Bun.serve({
  hostname: "127.0.0.1",
  port,
  fetch(request) {
    const path = decodeURIComponent(new URL(request.url).pathname);
    const file = normalize(join(dist, path === "/" ? "index.html" : path));
    const found = file.startsWith(normalize(dist)) && existsSync(file) && statSync(file).isFile();
    const served = found ? file : join(dist, "404.html");
    return new Response(readFileSync(served), { status: found ? 200 : 404, headers: { "content-type": TYPES[extname(served)] ?? "application/octet-stream" } });
  },
});
process.stdout.write(`serving ${dist} on http://127.0.0.1:${port}/\n`);
