/** 🔍️ Serves a built site while mapping plugin and extension module routes to the live staged modules directory, for boot probes before a site rebuild. Usage: node overlay-serve.mjs <site> <modules> <port> [module-folder transpiled-directory] */
import {createServer} from "node:http";
import {createReadStream, statSync} from "node:fs";
import {join, extname, posix} from "node:path";
const [site, modules, port, probeModule, transpiled] = process.argv.slice(2);
const types = {".html":"text/html",".js":"text/javascript",".mjs":"text/javascript",".css":"text/css",".json":"application/json",".wasm":"application/wasm",".svg":"image/svg+xml",".woff2":"font/woff2",".bin":"application/octet-stream",".semio":"application/octet-stream"};
const resolveFile = (url) => {
  const path = posix.normalize(decodeURIComponent(new URL(url, "http://x").pathname));
  const module = path.match(/^\/(🔌️plugin-modules|🧩️extension-modules)\/([^/]+)\/(.+)$/u);
  const candidates = module ? [join(modules, module[2], module[3]), join(site, path)] : [join(site, path)];
  if (module && module[2] === probeModule && transpiled) {
    const guest = /(?:\.wasm$|^semio_hub_.*_component\.js$|^🌉️bridge\.js$|^🟨️\.js$)/u.test(module[3]);
    if (guest || process.env.SEMIO_OVERLAY_EXACT_MODULE === "1") candidates.splice(0, candidates.length, join(transpiled, module[3]));
    else candidates.unshift(join(transpiled, module[3]));
  }
  for (const candidate of candidates) { try { const stat = statSync(candidate); if (stat.isFile()) return candidate; if (stat.isDirectory()) { const index = join(candidate, "index.html"); statSync(index); return index; } } catch {} }
  return undefined;
};
createServer((request, response) => {
  const file = resolveFile(request.url);
  if (!file) { response.writeHead(404); response.end(); return; }
  response.writeHead(200, {"content-type": types[extname(file)] ?? "application/octet-stream", "cache-control": "no-store"});
  if (request.method === "HEAD") { response.end(); return; }
  if (process.env.SEMIO_OVERLAY_FLOW_REQUESTS === "1" && request.method === "GET") {
    const path = new URL(request.url, "http://x").pathname;
    if (/^\/assets\/flow_core[-_][^/]+\.(js|wasm)$/u.test(path) && /Chrome\//u.test(request.headers["user-agent"] ?? "")) {
      response.once("finish", () => console.log("[DEBUG] Flow HTTP " + JSON.stringify({ path, file, status: response.statusCode })));
    }
  }
  createReadStream(file).pipe(response);
}).listen(Number(port), "127.0.0.1");
