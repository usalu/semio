/** 🗺️ Ticket-local static harness for live MapSession verification.
 *
 * Serves the prebuilt `framework_surface` wasm pkg, the repo's cached OSM/OpenFreeMap tiles and the
 * driver page, so the tiled-map engine can be exercised in a real browser without the gis plugin,
 * storybook, or any part of the app currently broken by concurrent refactors.
 */
// 🔤️ `URL.pathname` percent-encodes the emoji path segments this repo uses, which `Bun.file` will not
// find on disk — decode before touching the filesystem.
const ROOT = decodeURIComponent(new URL("../../../../../../../../", import.meta.url).pathname);
const HERE = decodeURIComponent(new URL(".", import.meta.url).pathname);
const PKG = `${ROOT}🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/pkg`;
const TILES = `${ROOT}.🧬semio/🗺️map`;
const PORT = Number(process.env.MAP_HARNESS_PORT ?? "6210");

const TYPES: Record<string, string> = { ".html": "text/html", ".js": "text/javascript", ".wasm": "application/wasm", ".png": "image/png", ".pbf": "application/x-protobuf", ".json": "application/json" };

const contentType = (path: string): string => TYPES[path.slice(path.lastIndexOf("."))] ?? "application/octet-stream";

async function serveFile(path: string): Promise<Response> {
  const file = Bun.file(path);
  if (!(await file.exists())) return new Response("not found", { status: 404 });
  return new Response(file, { headers: { "content-type": contentType(path), "cache-control": "no-store" } });
}

Bun.serve({
  port: PORT,
  idleTimeout: 240,
  async fetch(request) {
    const { pathname } = new URL(request.url);
    if (pathname === "/" || pathname === "/index.html") return serveFile(`${HERE}index.html`);
    if (pathname.startsWith("/pkg/")) return serveFile(`${PKG}/${pathname.slice(5)}`);
    if (pathname.startsWith("/osm/")) return serveFile(`${TILES}/osm-tiles/${pathname.slice(5)}`);
    if (pathname.startsWith("/vt/")) return serveFile(`${TILES}/openfreemap-vt/${pathname.slice(4)}`);
    return new Response("not found", { status: 404 });
  },
});

console.log(`[map-harness] listening on http://localhost:${PORT}`);
