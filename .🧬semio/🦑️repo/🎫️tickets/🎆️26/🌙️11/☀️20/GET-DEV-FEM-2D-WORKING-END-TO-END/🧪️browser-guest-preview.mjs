import { createServer, request } from "node:http";
import { createHash } from "node:crypto";
import { createReadStream } from "node:fs";
import { mkdir, stat } from "node:fs/promises";
import { basename, join, resolve } from "node:path";

const componentInput = process.argv[2] ?? "6096";
const componentPort = /^\d+$/.test(componentInput) ? Number(componentInput) : undefined;
const appPort = Number(process.argv[3] ?? 6086);
const port = Number(process.argv[4] ?? 6095);
const base = "semio_hub_fem_component";
const bundle = componentPort === undefined ? await buildPreview(resolve(componentInput)) : undefined;

async function digest(file) {
  const hash = createHash("sha256");
  for await (const chunk of createReadStream(file)) hash.update(chunk);
  return hash.digest("hex");
}

async function buildPreview(component) {
  const linked = await digest(component);
  const output = join(import.meta.dirname, "🗑️generated", "guest-" + linked.slice(0, 12));
  const fem = join(output, "fem");
  const preview2 = join(output, "preview2");
  console.log("[DEBUG] Linked FEM component", component, linked);
  if (!process.argv.includes("--reuse-bundle")) {
    const workspace = resolve(import.meta.dirname, "../../../../../../..");
    const { transpilePluginComponentAsync } = await import("../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts");
    await mkdir(fem, { recursive: true });
    await transpilePluginComponentAsync(component, fem, base, {
      repoRoot: workspace,
      optimize: false,
      preview2VendorDir: preview2,
    });
  }
  if (linked !== await digest(component)) throw new Error("The linked component changed during preview transpilation");
  await stat(join(fem, base + ".js"));
  console.log("[DEBUG] FEM2D preview core SHA-256", await digest(join(fem, base + ".core.wasm")));
  return fem;
}
const workerDiagnostics = `
const reportGuestError = console.error.bind(console);
console.error = (...args) => {
  reportGuestError(...args);
  const message = args.join(" ");
  if (/FEM WIT poll semantic failure|FEM cleanup semantic failure|Command exchange|Manifest command|Store replacement/i.test(message)) {
    void fetch("/__fem2d_guest_trace", { method: "POST", body: message.slice(0, 4096) }).catch(() => {});
  }
};
`;

createServer(async (incoming, response) => {
  const url = new URL(incoming.url, `http://127.0.0.1:${port}`);
  const path = decodeURIComponent(url.pathname);
  if (path === "/__fem2d_guest_trace" && incoming.method === "POST") {
    let message = "";
    incoming.setEncoding("utf8");
    incoming.on("data", chunk => { if (message.length < 4096) message += chunk; });
    incoming.on("end", () => {
      console.log("[DEBUG] FEM2D preview guest", message.slice(0, 4096));
      response.writeHead(204).end();
    });
    return;
  }
  const name = basename(path);
  const component = path === "/🔌️plugin-modules/🏗️fem/" + name && name.startsWith(base + ".");
  const shim = path.match(/^\/🔌️plugin-modules\/preview2\/((?:[a-zA-Z0-9_-]+\/)*[a-zA-Z0-9_.-]+\.js)$/)?.[1];
  if (shim && bundle) url.pathname = "/🔌️plugin-modules/🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/" + shim;
  if (component && bundle) {
    const file = join(bundle, name);
    const { size } = await stat(file).catch(() => ({}));
    if (size === undefined) {
      response.writeHead(404).end();
      return;
    }
    response.writeHead(200, {
      "content-type": name.endsWith(".wasm") ? "application/wasm" : "text/javascript",
      "content-length": size,
      "cache-control": "no-store",
      "cross-origin-resource-policy": "same-origin",
    });
    console.log("[DEBUG] FEM2D preview module", path, 200, size);
    if (incoming.method === "HEAD") response.end();
    else {
      const stream = createReadStream(file);
      stream.on("error", () => response.destroy());
      response.on("close", () => stream.destroy());
      stream.pipe(response);
    }
    return;
  }
  const upstreamPort = component ? componentPort : appPort;
  const upstream = request({
    hostname: "127.0.0.1",
    port: upstreamPort,
    path: url.pathname + url.search,
    method: incoming.method,
    headers: { ...incoming.headers, host: `127.0.0.1:${upstreamPort}` },
  }, reply => {
    const headers = { ...reply.headers };
    const inspectWorker = path.endsWith("/🟨️shard-worker.js") && reply.statusCode === 200;
    if (inspectWorker) {
      delete headers["content-length"];
      delete headers.etag;
    }
    response.writeHead(reply.statusCode, headers);
    if (inspectWorker) response.write(workerDiagnostics);
    if (component) console.log("[DEBUG] FEM2D preview module", path, reply.statusCode, headers["content-length"]);
    reply.pipe(response);
    response.on("close", () => reply.destroy());
  });
  upstream.on("error", () => {
    if (!response.headersSent) response.writeHead(502);
    response.end();
  });
  incoming.pipe(upstream);
}).listen(port, "127.0.0.1", () => console.log("[DEBUG] FEM2D preview listening on", port));
