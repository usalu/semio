/** 🧪️ Executes the fresh Draw descriptor in managed Chromium through actual browser module loading. */
import { strict as assert } from "node:assert";
import { join } from "node:path";
import { chromium } from "playwright";

const [moduleRoot, descriptor] = Bun.argv.slice(2);
const plugin = join(process.cwd(), "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle");
const { ACTOR_COMPONENT_EXPORTS } = await import(join(plugin, "🛂️descriptor/🟦️.ts"));
const assets = new Set<string>();
for await (const path of new Bun.Glob("{module,vendor}/**/*").scan({ cwd: moduleRoot, type: "file" })) assets.add(path);
const server = Bun.serve({
  hostname: "127.0.0.1",
  port: 0,
  fetch(request) {
    const path = decodeURIComponent(new URL(request.url).pathname.slice(1));
    if (path === "") return new Response("<!DOCTYPE html><html lang=en><title>Draw descriptor probe</title></html>", { headers: { "content-type": "text/html" } });
    if (!assets.has(path)) return new Response(null, { status: 404 });
    const mime = path.endsWith(".wasm") ? "application/wasm" : "text/javascript";
    return new Response(Bun.file(join(moduleRoot, path)), { headers: { "content-type": mime } });
  },
});
let browser;
try {
  browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  await page.goto(server.url.href);
  const value = await page.evaluate(async required => {
    const entry = new URL("/module/semio_hub_draw_component.js", location.href).href;
    const component = await import(entry);
    for (const [name, methods] of Object.entries(required)) {
      for (const method of methods) {
        if (typeof component[name]?.[method] !== "function") throw Error("Missing actor export " + name + "." + method);
      }
    }
    const grant = { maximumItems: 1n << 20n, maximumCopyBytes: 64n << 20n, maximumCapacityBytes: 64n << 20n, maximumReleaseBytes: 64n << 20n, maximumDepth: 64n };
    const fuel = 1n << 24n;
    const deadlineUs = 1n << 52n;
    const budget = { input: { operation: 1n, generation: 1n, epoch: 1n, work: { clockBinding: 1n, absoluteDeadlineUs: deadlineUs, fuel }, grant }, retainedPolicy: grant, fuel, deadlineUs, cancelled: false };
    const bytes = await component.describe.describe(budget);
    if (!(bytes instanceof Uint8Array) || bytes.length === 0 || bytes.length > 8 * 1024 * 1024) throw Error("Invalid descriptor byte extent");
    let binary = "";
    const chunk = 16384;
    for (let offset = 0; offset < bytes.length; offset += chunk) binary += String.fromCharCode(...bytes.subarray(offset, offset + chunk));
    return btoa(binary);
  }, ACTOR_COMPONENT_EXPORTS);
  const expected = await Bun.file(descriptor).bytes();
  assert(Buffer.from(expected).equals(Buffer.from(value, "base64")), "Chromium and Wasmtime returned different Draw descriptors");
  console.log({ browser: browser.version(), bytes: expected.byteLength, exports: Object.values(ACTOR_COMPONENT_EXPORTS).flat().length });
} finally {
  await browser?.close();
  await server.stop(true);
}
