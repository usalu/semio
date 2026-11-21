/** 🧪 Streams the isolated browser experiment through an ephemeral loopback port. */
import { once } from "node:events";
import { createReadStream, readFileSync, statSync } from "node:fs";
import { createServer } from "node:http";
import { join } from "node:path";

export async function usePickerExperiment(context, path, origin) {
  if (!path) return;
  const contentLength = statSync(path).size;
  const server = createServer((request, response) => {
    if (request.method !== "GET" || request.url !== "/note-picker-experiment.wasm") {
      response.writeHead(404).end();
      return;
    }
    response.writeHead(200, {
      "Access-Control-Allow-Origin": origin,
      "Cache-Control": "no-store",
      "Content-Length": contentLength,
      "Content-Type": "application/wasm",
      "Cross-Origin-Resource-Policy": "cross-origin",
    });
    const stream = createReadStream(path);
    response.on("close", () => stream.destroy());
    stream.on("error", () => response.destroy());
    stream.pipe(response);
  });
  server.listen(0, "127.0.0.1");
  await once(server, "listening");
  server.unref();
  context.once("close", () => server.close());
  await context.route(url => decodeURIComponent(url.pathname).endsWith("/🗒️note/semio_hub_note_component.core.wasm"), route => {
    console.log("[DEBUG] Requesting an isolated Note core from the local test server");
    return route.continue({url: `http://127.0.0.1:${server.address().port}/note-picker-experiment.wasm`});
  });
}

/** 📦️ Loads an unpublished linked guest with the binding generated alongside its core. */
export async function useLinkedCapture(context, directory, origin) {
  if (!directory) return;
  const base = "semio_hub_note_component";
  const source = readFileSync(join(directory, `${base}.js`), "utf8");
  if (source.includes("from '@bytecodealliance/preview2-shim/")) throw Error("The isolated Note bindings need browser-relative preview2 imports");
  await context.route(url => decodeURIComponent(url.pathname).endsWith(`/🗒️note/${base}.js`), route => {
    return route.fulfill({body: source, contentType: "text/javascript"});
  });
  await usePickerExperiment(context, join(directory, `${base}.core.wasm`), origin);
}
