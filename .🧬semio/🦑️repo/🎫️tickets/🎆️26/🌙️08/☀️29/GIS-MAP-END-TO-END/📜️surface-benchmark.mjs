import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { chromium, devices, firefox, webkit } from "playwright";

const [bundle, rasterFile, vectorFile, tilePath] = process.argv.slice(2);
assert(bundle && rasterFile && vectorFile && /^\d+\/\d+\/\d+$/.test(tilePath ?? ""));
const tile = tilePath.split("/").map(Number);
const packages = {
  "/framework_surface.js": ["text/javascript", readFileSync(join(bundle, "framework_surface.js"))],
  "/framework_surface_bg.wasm": ["application/wasm", readFileSync(join(bundle, "framework_surface_bg.wasm"))],
};
const raster = readFileSync(rasterFile).toString("base64");
const vector = readFileSync(vectorFile).toString("base64");
const root = new URL("../../../../../../../", import.meta.url);
const contract = new URL("🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🎛️map-policy/🧪️tests/🧫️fixtures/🔣️.json", root);
const { neutral: policy } = JSON.parse(readFileSync(contract, "utf8"));
const { devices: profiles } = JSON.parse(readFileSync(new URL("./🧪️devices.json", import.meta.url), "utf8"));
policy.bands[0].gates.maxAdminLevel = Number.MAX_SAFE_INTEGER;
policy.rasterTileZMax = policy.vectorTileZMax = tile[0];
policy.layers = [
  ["landcover", "landCover"], ["water", "waterArea"], ["building", "building"], ["transportation", "road"],
].map(([name, role], drawRank) => ({ name, role, drawRank, labelRank: null }));

const origin = "http://localhost:6200";
for (const [engine, browserType] of Object.entries({ chromium, firefox, webkit })) {
  const browser = await browserType.launch();
  try {
    for (const { name } of profiles.filter(({ name }) => devices[name].defaultBrowserType === engine)) {
      const { defaultBrowserType, ...device } = devices[name];
      const page = await browser.newPage(device);
      try {
        await page.addInitScript(() => Object.defineProperty(navigator, "gpu", { configurable: true, value: undefined }));
        await page.route(`${origin}/**`, route => {
          const path = new URL(route.request().url()).pathname;
          if (path === "/") {
            return route.fulfill({ contentType: "text/html", body: '<!doctype html><meta name="viewport" content="width=device-width,initial-scale=1"><canvas></canvas>' });
          }
          const file = packages[path];
          return route.fulfill(file ? { contentType: file[0], body: file[1] } : { status: 404 });
        });
        await page.goto(origin);
        const measurements = await page.evaluate(async ({ policy, raster, vector, tile }) => {
          const { default: init, MapSession } = await import("/framework_surface.js");
          await init();
          const canvas = document.querySelector("canvas");
          const width = window.innerWidth, height = window.innerHeight, dpr = window.devicePixelRatio;
          canvas.style.width = `${width}px`;
          canvas.style.height = `${height}px`;
          const map = new MapSession();
          const decode = data => Uint8Array.from(atob(data), value => value.charCodeAt(0));
          try {
            await map.attachCanvas(canvas, width, height, dpr);
            map.syncMapJson(JSON.stringify({ policy, overlays: {} }));
            map.setRenderMode("combined");
            const n = 2 ** tile[0], x = 2 * (tile[1] + 0.5) / n - 1, y = 2 * (tile[2] + 0.5) / n - 1;
            const zoom = Math.max(width, height) * n * 0.65;
            map.setCamera(x, y, zoom);
            const rasterStart = performance.now();
            map.uploadTile(...tile, decode(raster));
            const vectorStart = performance.now();
            map.uploadVectorTile(...tile, decode(vector));
            const drawStart = performance.now();
            map.renderFrame();
            const firstDraw = performance.now() - drawStart;
            const elapsed = [];
            for (let i = 0; i < 60; i++) {
              await new Promise(requestAnimationFrame);
              map.setCamera(x + Math.sin(i / 10) * 20 / zoom, y, zoom);
              const started = performance.now();
              map.renderFrame();
              elapsed.push(performance.now() - started);
            }
            const context = canvas.getContext("2d");
            const pixels = context.getImageData(0, 0, canvas.width, canvas.height).data;
            const colors = new Set();
            for (let i = 0; i < pixels.length; i += 4 * 8191) colors.add(pixels.slice(i, i + 4).join(","));
            elapsed.sort((a, b) => a - b);
            const isSample = row => row.z === tile[0] && row.x === tile[1] && row.y === tile[2];
            return {
              viewport: [width, height, dpr], backend: canvas.dataset.mapRenderer, distinctColors: colors.size,
              heldRaster: map.hasTile(...tile), heldVector: map.hasVectorTile(...tile),
              visibleRaster: JSON.parse(map.visibleTilesJson()).some(isSample),
              visibleVector: JSON.parse(map.visibleVectorTilesJson()).some(isSample),
              rasterUploadMs: vectorStart - rasterStart, vectorUploadMs: drawStart - vectorStart, firstDrawMs: firstDraw,
              panDrawMedianMs: elapsed[29], panDrawP95Ms: elapsed[56],
            };
          } finally {
            map.free();
          }
        }, { policy, raster, vector, tile });
        assert.equal(measurements.backend, "canvas2d");
        assert(measurements.heldRaster && measurements.heldVector && measurements.visibleRaster && measurements.visibleVector && measurements.distinctColors > 20,
          JSON.stringify({ engine, device: name, ...measurements }));
        console.log(JSON.stringify({ engine, device: name, ...measurements }));
      } finally {
        await page.close();
      }
    }
  } finally {
    await browser.close();
  }
}
