import { chromium } from "playwright";
import { strict as assert } from "node:assert";
import { createServer } from "node:http";
import { existsSync } from "node:fs";
import { join } from "node:path";

const probe = process.argv[2] === "adapter";
const server = probe ? createServer((_, response) => response.end("<!doctype html><title>WebGPU</title>")) : undefined;
if (server) await new Promise(resolve => server.listen(0, "127.0.0.1", resolve));
const local = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";
const browser = await chromium.launch({
  executablePath: process.env.CHROME_BIN || (existsSync(local) ? local : undefined),
  args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--enable-experimental-webassembly-features", ...(process.platform === "darwin" ? ["--use-angle=metal"] : [])],
});
try {
  const locale = process.argv[3] || "en";
  const page = await browser.newPage({ viewport: { width: 1280, height: 800 }, locale: locale === "en" ? "en-US" : "de-DE" });
  page.on("pageerror", error => console.error("[DEBUG] Page error", error.message));
  page.on("console", message => { if (message.type() === "error") console.error("[DEBUG] Browser error", message.text()); });
  page.on("requestfailed", request => console.error("[DEBUG] Resource unavailable", request.failure()?.errorText, request.url()));
  page.on("response", response => { if (response.status() >= 400) console.error("[DEBUG] HTTP", response.status(), response.url()); });
  const role = process.argv[4] || (locale === "en" ? "editor" : "viewer");
  const axes = new URLSearchParams({ locale, plugin:"bim", role, mode:role === "editor" ? "edit" : "view", app:"s.bim.model@1/*#"+role, example:"house" });
  const address = new URL(probe ? "http://127.0.0.1:" + server.address().port : process.env.SEMIO_WGPU_URL || "http://127.0.0.1:6402/");
  if (!probe) for (const [name, value] of axes) address.searchParams.set(name, value);
  const document = await page.goto(address.href, { waitUntil: "domcontentloaded" });
  assert.ok(document?.ok(), `shell HTTP status ${document?.status()}`);
  const graphics = await page.evaluate(async () => {
    const adapter = await navigator.gpu?.requestAdapter();
    if (!adapter) throw Error("WebGPU adapter unavailable");
    const device = await adapter.requestDevice();
    const result = { vendor: adapter.info.vendor, architecture: adapter.info.architecture, features: [...device.features] };
    device.destroy();
    return result;
  });
  console.log(JSON.stringify({ browser: await browser.version(), locale, graphics }));
  if (!probe) {
    await page.waitForFunction(() => document.querySelector("[data-semio-os-ready],[data-semio-os-error]"), null, { timeout: 240000 });
    const shell = page.locator("html");
    assert.equal(await shell.getAttribute("data-semio-os-error"), null, (await page.locator("body").innerText()).slice(0, 1000));
    assert.equal(await shell.getAttribute("data-semio-os-ready"), "bim");
    assert.equal(await page.locator("#root").getAttribute("lang"), locale);
    await page.waitForFunction(async () => {
      const state = window.semioWgpuIntrospection && JSON.parse(await window.semioWgpuIntrospection.dumpChrome());
      return state && state.generation > 0 && state.hits?.length;
    }, null, { timeout: 120000 });
    const state = await page.evaluate(async () => JSON.parse(await window.semioWgpuIntrospection.dumpChrome()));
    console.log(JSON.stringify({ generation: state.generation, controls: state.hits.map(hit => hit.controlId), surfaces: state.surfaces }));
    const chrome = async () => JSON.parse(await page.evaluate(() => window.semioWgpuIntrospection.dumpChrome()));
    const skip = state.hits.find(hit => hit.controlId === "ui.introduction.skip");
    if (skip) {
      const [left, top, width, height] = skip.rect;
      await page.mouse.click(left + width / 2, top + height / 2);
      await page.waitForFunction(async () => !JSON.parse(await window.semioWgpuIntrospection.dumpChrome()).hits.some(hit => hit.controlId === "ui.introduction.skip"));
    }
    const accepted = await page.waitForFunction(async () => {
      const mirror = document.querySelector("#semio-wgpu-accessibility");
      const ids = [...new Set([...mirror?.querySelectorAll("[data-window]") || []].map(element => element.dataset.window))];
      for (const id of ids) {
        const accepted = JSON.parse(await window.semioWgpuIntrospection.dumpAccessibility(id)).windows?.find(surface => surface.windowId === id);
        if (accepted?.nodes?.length && /bim|storey|wall|geschoss|wand/i.test(JSON.stringify(accepted))) {
          return {windowId:id,generation:accepted.windowGeneration,keys:accepted.nodes.map(node=>node.key).slice(0,24)};
        }
      }
      return false;
    }, null, {timeout:240000});
    console.log(JSON.stringify({role,example:"house",accepted:await accepted.jsonValue()}));
    const mesh = await page.waitForFunction(async () => {
      const dump = JSON.parse(await window.semioWgpuIntrospection.dumpMeshStats());
      const surface = dump.surfaces?.find(row => row.meshes?.length > 1 && row.instances?.length > 1 && row.liveCamera && row.bboxMin && row.bboxMax);
      return surface && {
        surfaceId: surface.surfaceId, meshes: surface.meshes.length, instances: surface.instances.length,
        bounds: [surface.bboxMin, surface.bboxMax], roles: surface.roles, rect: surface.rect,
      };
    }, null, {timeout:240000});
    const world = await mesh.jsonValue();
    console.log(JSON.stringify({ world }));
    const image = await page.screenshot({ path: join(import.meta.dirname, "🗑️generated", "bim-wgpu-"+locale+".png") });
    const paint = await page.evaluate(async ({ encoded, rect }) => {
      const decoded = Uint8Array.from(atob(encoded), value => value.charCodeAt(0));
      const bitmap = await createImageBitmap(new Blob([decoded], { type: "image/png" }));
      const canvas = new OffscreenCanvas(96, 64);
      const context = canvas.getContext("2d");
      const [x, y, width, height] = rect;
      const sx = bitmap.width / innerWidth;
      const sy = bitmap.height / innerHeight;
      context.drawImage(bitmap, (x + width * 0.10) * sx, (y + height * 0.10) * sy, width * 0.80 * sx, height * 0.80 * sy, 0, 0, 96, 64);
      const pixels = context.getImageData(0, 0, 96, 64).data;
      const colors = new Set();
      let opaque = 0;
      for (let i = 0; i < pixels.length; i += 4) {
        if (pixels[i + 3] === 255) opaque++;
        colors.add((pixels[i] << 16) | (pixels[i + 1] << 8) | pixels[i + 2]);
      }
      return { distinctCentralPixels: colors.size, opaque };
    }, { encoded: image.toString("base64"), rect: world.rect });
    assert.ok(paint.distinctCentralPixels > 8 && paint.opaque > 0, `shell paint was blank: ${JSON.stringify(paint)}`);
    console.log(JSON.stringify({ paint }));
    await page.locator("canvas:visible").first().focus();
    await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
    await page.waitForFunction(async () => {
      const hits = JSON.parse(await window.semioWgpuIntrospection.dumpChrome()).hits;
      return hits.some(hit => hit.controlId === "ui.search.input") && hits.some(hit => hit.controlId?.startsWith("ui.search.item."));
    });
    const opened = await chrome();
    await page.keyboard.press("Escape");
    await page.waitForFunction(async () => !JSON.parse(await window.semioWgpuIntrospection.dumpChrome()).hits.some(hit => hit.controlId === "ui.search.input"));
    console.log(JSON.stringify({ paletteEntries: opened.hits.filter(hit => hit.controlId?.startsWith("ui.search.item.")).length, dismissed: true }));
  }
} finally {
  await browser.close();
  if (server) server.close();
}
