import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { PNG } from "pngjs";
import { repoPlaywrightBrowsersPath } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts";

const [surface, site, profile] = process.argv.slice(2);
if (!surface || !site || (profile && !["chromium", "windowed"].includes(profile)) || process.argv.length > 5) throw Error("map-gpu-probe <release surface module directory> <release Vite output> [chromium|windowed]");
const workspace = resolve(import.meta.dir, "../../../../../../..");
const play = join(workspace, "🏢️semio-tech/🎡️play");
const { neutral: policy } = JSON.parse(readFileSync(join(workspace, "🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🎛️map-policy/🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
policy.bands[0].gates.maxAdminLevel = 99;
policy.featureKeys = { class: "class", adminLevel: "admin_level", maritime: "maritime", label: ["name"] };
policy.layers = [["water", "waterArea"], ["landcover", "landCover"], ["boundary", "boundary"]].map(([name, role], drawRank) => ({ name, role, drawRank, labelRank: null }));
process.env.PLAYWRIGHT_BROWSERS_PATH = repoPlaywrightBrowsersPath(workspace);
const { chromium } = createRequire(join(play, "package.json"))("playwright") as typeof import("playwright");
const files = new Map([
  ["/framework_surface.js", Bun.file(join(surface, "framework_surface.js"))],
  ["/framework_surface_bg.wasm", Bun.file(join(surface, "framework_surface_bg.wasm"))],
]);
for (const file of files.values()) assert(await file.exists(), file.name);
const server = Bun.serve({
  hostname: "127.0.0.1",
  port: 0,
  fetch(request) {
    const path = new URL(request.url).pathname;
    const file = files.get(path) ?? (/^\/(?:osm|vt)\/\d+\/\d+\/\d+\.(?:png|pbf)$/u.test(path) ? Bun.file(join(site, path)) : undefined);
    if (path === "/") return new Response("<!doctype html><canvas width=256 height=256></canvas>", { headers: { "Content-Type": "text/html" } });
    if (path === "/favicon.ico") return new Response(null, { status: 204 });
    return file ? new Response(file) : new Response(null, { status: 404 });
  },
});
let browser: Awaited<ReturnType<typeof chromium.launch>> | undefined;
const timeout = setTimeout(() => { void browser?.close(); }, 45_000);
try {
  process.env.PLAYWRIGHT_BASE_URL = server.url.href;
  const { default: config } = await import(pathToFileURL(join(play, "🔨️modules/🧪️e2e/🎚️config/🟦️.ts")).href);
  const launch = config.projects[0].use.launchOptions;
  const args = process.env.PLAY_GPU_PROBE_DISABLE_COMPOSITING === "1" ? [...launch.args, "--disable-gpu-compositing"] : launch.args;
  const channel = profile === "chromium" ? profile : undefined;
  browser = await chromium.launch({ ...launch, args, channel, headless: profile === "windowed" ? false : launch.headless, timeout: 15_000 });
  const page = await browser.newPage();
  const errors: string[] = [];
  page.on("pageerror", error => errors.push(String(error)));
  page.on("console", entry => { if (entry.type() === "error") errors.push(entry.text()); });
  await page.goto(server.url.href);
  const capture = async () => PNG.sync.read(await page.screenshot({ clip: { x: 8, y: 8, width: 256, height: 256 }, timeout: 10_000 })).data;
  const before = await capture();
  console.log("[DEBUG] Browser captures a canvas before native map attachment.");
  const map = await page.evaluateHandle(async descriptor => {
    const module = await import(new URL("/framework_surface.js", location.href).href);
    await module.default();
    const session = new module.MapSession();
    await session.attachCanvas(document.querySelector("canvas"), 256, 256, 1);
    session.syncMapJson(descriptor);
    session.fitWorldCamera();
    session.setRenderMode("raster");
    return session;
  }, JSON.stringify({ policy, overlays: {} }));
  assert(await map.evaluate(session => session.gpuReady()));
  console.log("[DEBUG] MapSession attached to its WebGPU canvas.");
  const render = async (vector: boolean) => {
    const uploaded = await map.evaluate(async (session, vector) => {
      session.setRenderMode(vector ? "vector" : "raster");
      const tiles: { z: number; x: number; y: number }[] = JSON.parse(vector ? session.visibleVectorTilesJson() : session.visibleTilesJson());
      await Promise.all(tiles.map(async ({ z, x, y }) => {
        const response = await fetch(vector ? `/vt/${z}/${x}/${y}.pbf` : `/osm/${z}/${x}/${y}.png`);
        if (!response.ok) throw Error(`Missing visible tile ${z}/${x}/${y}`);
        const bytes = new Uint8Array(await response.arrayBuffer());
        if (vector) session.uploadVectorTile(z, x, y, bytes);
        else session.uploadTile(z, x, y, bytes);
      }));
      session.renderFrame();
      return tiles;
    }, vector);
    console.log(`[DEBUG] MapSession submitted ${vector ? "vector" : "raster"} draw with ${uploaded.length} uploaded tiles.`);
    assert(uploaded.length > 0);
    return capture();
  };
  const raster = await render(false);
  const vector = await render(true);
  const changed = (a: Buffer, b: Buffer) => {
    let count = 0;
    for (let i = 0; i < a.length; i += 4) if (a.readUInt32LE(i) !== b.readUInt32LE(i)) count++;
    return count;
  };
  const rasterPixels = changed(before, raster);
  const vectorPixels = changed(raster, vector);
  const colors = (pixels: Buffer) => {
    const seen = new Set<number>();
    for (let i = 0; i < pixels.length; i += 4) seen.add(pixels.readUInt32LE(i));
    return seen.size;
  };
  console.log(`[DEBUG] MapSession screenshot changed pixels ${JSON.stringify({ rasterPixels, vectorPixels, rasterColors: colors(raster), vectorColors: colors(vector), errors })}`);
  assert(rasterPixels > 100);
  assert(vectorPixels > 100);
  assert(colors(raster) > 1);
  assert(colors(vector) > 1);
  assert.deepEqual(errors, []);
} finally {
  clearTimeout(timeout);
  await browser?.close();
  await server.stop(true);
}
