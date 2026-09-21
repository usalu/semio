#!/usr/bin/env bun
/** 🛰️ RB1 §4 — serve the PRODUCTION `s` bundle the way a deployment would: a plain static file
 * server over `dist/build-s-react-release`, with no Vite, no dev server, no HMR and no plugin
 * pipeline behind it. If the bundle needs anything the dev server used to do for it, it fails here.
 *
 * Three things a naive static server gets wrong for this bundle, all handled:
 *   · `.wasm` must be `application/wasm` or `WebAssembly.instantiateStreaming` refuses it;
 *   · the shell is an SPA — unknown paths fall back to `index.html`, but asset-looking paths 404
 *     honestly instead, so a missing chunk is visible rather than served as HTML;
 *   · cross-origin isolation headers, because the shell's workers use SharedArrayBuffer.
 *
 * Usage: bun 🐍️rb1-serve-release-bundle.ts <root> <port>
 */
import { existsSync, statSync } from "node:fs";
import { extname, join, normalize, resolve } from "node:path";

const [rootArg, portArg] = process.argv.slice(2);
if (!rootArg || !portArg) throw new Error("usage: 🐍️rb1-serve-release-bundle.ts <root> <port>");
const root = resolve(rootArg);
const port = Number(portArg);
if (!existsSync(join(root, "index.html"))) throw new Error(`no index.html under ${root} — the bundle was not built`);

const TYPES: Record<string, string> = {
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".mjs": "text/javascript; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".json": "application/json; charset=utf-8",
  ".wasm": "application/wasm",
  ".svg": "image/svg+xml",
  ".png": "image/png",
  ".jpg": "image/jpeg",
  ".webp": "image/webp",
  ".woff2": "font/woff2",
  ".woff": "font/woff",
  ".ttf": "font/ttf",
  ".map": "application/json; charset=utf-8",
  ".wgsl": "text/plain; charset=utf-8",
  ".glb": "model/gltf-binary",
};

/** 🧭️ A path that looks like an asset must 404 rather than fall back to `index.html`, so a missing
 * chunk shows up as a missing chunk instead of as a parse error on unexpected HTML. */
const ASSET = /\.[a-z0-9]{2,6}$/i;

const served = { requests: 0, notFound: [] as string[] };

Bun.serve({
  port,
  hostname: "0.0.0.0",
  async fetch(request) {
    served.requests += 1;
    const url = new URL(request.url);
    const relative = normalize(decodeURIComponent(url.pathname)).replace(/^(\.\.[/\\])+/, "").replace(/^[/\\]+/, "");
    const candidate = join(root, relative);
    const headers: Record<string, string> = {
      "cross-origin-opener-policy": "same-origin",
      "cross-origin-embedder-policy": "require-corp",
      "cross-origin-resource-policy": "cross-origin",
      "cache-control": "no-store",
    };
    if (candidate.startsWith(root) && existsSync(candidate) && statSync(candidate).isFile()) {
      const type = TYPES[extname(candidate).toLowerCase()];
      return new Response(Bun.file(candidate), { headers: type ? { ...headers, "content-type": type } : headers });
    }
    if (ASSET.test(relative)) {
      served.notFound.push(relative);
      return new Response(`not found: ${relative}`, { status: 404, headers });
    }
    return new Response(Bun.file(join(root, "index.html")), { headers: { ...headers, "content-type": TYPES[".html"]! } });
  },
});

console.log(`[rb1-serve] root ${root}`);
console.log(`[rb1-serve] listening on http://0.0.0.0:${port}`);
setInterval(() => {
  if (served.notFound.length) {
    console.log(`[rb1-serve] ${served.requests} requests, ${served.notFound.length} 404(s): ${[...new Set(served.notFound)].slice(0, 20).join(", ")}`);
    served.notFound.length = 0;
  }
}, 5000);
