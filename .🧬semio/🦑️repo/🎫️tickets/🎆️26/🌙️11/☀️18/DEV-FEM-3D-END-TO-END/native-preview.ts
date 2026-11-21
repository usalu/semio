import { createServer, request } from "node:http";
import { createReadStream, existsSync, mkdirSync, statSync } from "node:fs";
import { basename, join, resolve } from "node:path";
import { transpilePluginComponentAsync } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts";

/** 🔍️ Serves a freshly compiled FEM guest against the running playground on a separate loopback port. */
const [component] = process.argv.slice(2);
const staged = component === "--staged";
const port = Number(process.env.FEM_PREVIEW_PORT || (staged ? 6092 : 6091));
if (!staged && (!component || !existsSync(component))) throw Error("Pass the FEM component wasm from the native build.");
const workspace = resolve(import.meta.dirname, "../../../../../../..");
const modules = join(import.meta.dirname, "🗑️generated", "preview-" + port);
const fem = join(modules, "🏗️fem");
const base = "semio_hub_fem_component";
mkdirSync(fem, { recursive: true });
if (!staged) await transpilePluginComponentAsync(resolve(component), fem, base, {
  repoRoot: workspace, optimize: false, preview2VendorDir: join(modules, "🪞️vendor", "🤝️bytecode-alliance", "🪟️preview2-shim"),
});
const workerDiagnostics = `
const writeError = console.error.bind(console);
console.error = (...args) => {
  writeError(...args);
  const text = args.join(" ");
  if (/semantic failure|receipt-refusal|owes its enclosing/.test(text)) {
    void fetch("/__fem_guest_trace", {method:"POST",body:text.slice(0,4096)}).catch(()=>{});
  }
};
`;
createServer((incoming, response) => {
  if (incoming.url === "/__fem_guest_trace" && incoming.method === "POST") {
    let line = "";
    incoming.setEncoding("utf8").on("data", (chunk: string) => { if (line.length < 4096) line += chunk.slice(0, 4096 - line.length); });
    incoming.on("end", () => { console.log("[DEBUG] guest", line); response.writeHead(204).end(); });
    return;
  }
  const url = new URL(incoming.url ?? "/", "http://127.0.0.1:6091");
  const path = decodeURIComponent(url.pathname);
  const name = basename(path);
  if (!staged && path === "/🔌️plugin-modules/🏗️fem/" + name && name.startsWith(base + ".") && existsSync(join(fem, name))) {
    const file = join(fem, name);
    response.writeHead(200, { "content-type": name.endsWith(".wasm") ? "application/wasm" : "text/javascript", "content-length": statSync(file).size, "cache-control": "no-store", "cross-origin-resource-policy": "same-origin" });
    if (incoming.method === "HEAD") { response.end(); return; }
    const stream = createReadStream(file);
    response.once("close", () => stream.destroy());
    stream.on("error", () => response.destroy()).pipe(response);
    return;
  }
  const upstream = request({ hostname: "127.0.0.1", port: 6087, path: incoming.url, method: incoming.method, headers: { ...incoming.headers, host: "127.0.0.1:6087" } }, (reply) => {
    if (name === "🟨️shard-worker.js") {
      const headers = { ...reply.headers };
      delete headers["content-length"];
      delete headers.etag;
      response.writeHead(reply.statusCode ?? 502, headers);
      response.write(workerDiagnostics);
    } else {
      response.writeHead(reply.statusCode ?? 502, reply.headers);
    }
    reply.pipe(response);
    response.once("close", () => reply.destroy());
  });
  upstream.on("error", () => { if (!response.headersSent) response.writeHead(502); response.end(); });
  incoming.pipe(upstream);
}).listen(port, "127.0.0.1", () => console.log("[DEBUG] FEM preview listening on", port));
