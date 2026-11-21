import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { chromium, firefox, webkit } from "playwright";
import png from "pngjs";

const [wasmPath, engine = "chromium"] = process.argv.slice(2);
assert(wasmPath, "Pass the built Surface WASM output directory.");
const browsers = { chromium, firefox, webkit };
assert(Object.hasOwn(browsers, engine), "Unknown Playwright browser");
const wasmDirectory = resolve(wasmPath);
const root = new URL("../../../../../../../", import.meta.url);
const fixture = JSON.parse(readFileSync(new URL("./🧪️canvas2d.json", import.meta.url), "utf8"));
const policyUrl = new URL("🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🎛️map-policy/🧪️tests/🧫️fixtures/🔣️.json", root);
const { neutral: policy } = JSON.parse(readFileSync(policyUrl, "utf8"));
policy.bands[0].gates.maxAdminLevel = Number.MAX_SAFE_INTEGER;
policy.rasterTileZMax = fixture.raster.tile[0];
policy.vectorTileZMax = fixture.vector.tile[0];
policy.layers = [{ name: "land", role: "landCover", drawRank: 0, labelRank: null }];
policy.overlays = [{
  id: "pins", geometry: "point", granularity: "pin", hitRadiusPx: 14, lineWidth: 2,
  pointRadius: fixture.pin.radius, pointStroke: 1, labelOffsetPx: 6, fill: "positionFill",
  stroke: "positionStroke", styleFills: [],
}];

const raster = new png.PNG({ width: 1, height: 1 });
raster.data.set(fixture.raster.rgba);
const rasterBytes = [...png.PNG.sync.write(raster)];
const packageFiles = new Map([
  ["/framework_surface.js", { contentType: "text/javascript", body: readFileSync(join(wasmDirectory, "framework_surface.js")) }],
  ["/framework_surface_bg.wasm", { contentType: "application/wasm", body: readFileSync(join(wasmDirectory, "framework_surface_bg.wasm")) }],
]);
const browser = await browsers[engine].launch({ headless: true });
try {
  const page = await browser.newPage();
  await page.addInitScript(() => {
    Object.defineProperty(navigator, "gpu", { configurable: true, value: undefined });
  });
  const origin = "http://localhost:6200";
  await page.route(`${origin}/**`, async route => {
    const path = new URL(route.request().url()).pathname;
    if (path === "/") {
      await route.fulfill({ contentType: "text/html", body: '<!doctype html><link rel="icon" href="data:,"><canvas></canvas>' });
    } else {
      await route.fulfill(packageFiles.get(path) ?? { status: 404 });
    }
  });
  await page.goto(origin);
  const result = await page.evaluate(async ({ fixture, policy, rasterBytes }) => {
    const { default: init, MapSession } = await import("/framework_surface.js");
    await init();
    const canvas = document.querySelector("canvas");
    const session = new MapSession();
    const { viewport, clear, raster, vector, pin, samples } = fixture;
    const descriptor = { policy, overlays: { pins: [{ id: pin.id, coordinates: [pin.coordinate] }] } };
    await session.attachCanvas(canvas, viewport, viewport, 1);
    const context = canvas.getContext("2d");
    if (!context || !session.rendererReady()) throw Error("Canvas2D unavailable");
    const prototype = CanvasRenderingContext2D.prototype;
    const putImageData = prototype.putImageData;
    let rasterCopies = 0;
    prototype.putImageData = function (...args) {
      rasterCopies++;
      return putImageData.apply(this, args);
    };
    try {
      session.syncMapJson(JSON.stringify(descriptor));
      session.setRenderMode("combined");
      session.setMapThemeJson(JSON.stringify({
        surfaceClear: clear, landFill: vector.rgba, positionFill: pin.rgba,
        positionStroke: pin.rgba, selectionStroke: pin.selection,
      }));
      session.fitWorldCamera();
      session.uploadTile(...raster.tile, Uint8Array.from(rasterBytes));
      session.uploadVectorTile(...vector.tile, Uint8Array.from(vector.bytes));
      const pick = JSON.parse(session.hitTestFeatureJson(...pin.center));
      const style = ([r,g,b,a]) => `fill="rgb(${r},${g},${b})" fill-opacity="${a / 255}"`;
      const rect = ([x, y, w, h], color) => `<rect x="${x}" y="${y}" width="${w}" height="${h}" ${style(color)}/>`;
      const circle = (r, color) => `<circle cx="${pin.center[0]}" cy="${pin.center[1]}" r="${r}" ${style(color)}/>`;
      const frames = [];
      for (const dpr of fixture.devicePixelRatios) {
        session.setSize(viewport, viewport, dpr);
        const size = Math.round(viewport * dpr);
        const oracleCanvas = document.createElement("canvas");
        oracleCanvas.width = size;
        oracleCanvas.height = size;
        const oracle = oracleCanvas.getContext("2d");
        for (const selected of [false, true]) {
          session.syncInteraction("pin", JSON.stringify(selected ? [pin.id] : []));
          session.renderFrame();
          const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 ${viewport} ${viewport}">` +
            rect([0, 0, viewport, viewport], clear) + rect(raster.svgRect, raster.rgba) +
            rect(vector.svgRect, vector.rgba) + (selected ? circle(pin.selectionRadius, pin.selection) : "") +
            circle(pin.radius, pin.rgba) + "</svg>";
          const url = URL.createObjectURL(new Blob([svg], { type: "image/svg+xml" }));
          try {
            const image = new Image();
            image.src = url;
            await image.decode();
            oracle.drawImage(image, 0, 0);
          } finally {
            URL.revokeObjectURL(url);
          }
          const values = Object.entries(samples).map(([name, [x, y]]) => {
            const pixel = [Math.floor(x * dpr), Math.floor(y * dpr), 1, 1];
            return { name, actual: [...context.getImageData(...pixel).data], expected: [...oracle.getImageData(...pixel).data] };
          });
          frames.push({ dpr, selected, width: canvas.width, height: canvas.height, values });
        }
      }
      const dpr = fixture.devicePixelRatios.at(-1);
      session.syncMapJson(JSON.stringify({ policy, overlays: { pins: [{ id: pin.id, coordinates: [pin.editedCoordinate] }] } }));
      session.renderFrame();
      const edited = JSON.parse(session.hitTestFeatureJson(...pin.editedCenter));
      const vacated = JSON.parse(session.hitTestFeatureJson(...pin.center));
      const editedPixel = [...context.getImageData(pin.editedCenter[0] * dpr, pin.editedCenter[1] * dpr, 1, 1).data];
      return { renderer: canvas.dataset.mapRenderer, pick, edited, vacated, editedPixel, rasterCopies, frames };
    } finally {
      prototype.putImageData = putImageData;
      session.free();
    }
  }, { fixture, policy, rasterBytes });
  assert.equal(result.renderer, "canvas2d");
  assert.equal(result.pick?.id, fixture.pin.id);
  assert.equal(result.edited?.id, fixture.pin.id);
  assert.equal(result.vacated, null);
  assert.deepEqual(result.editedPixel, fixture.pin.rgba);
  assert.equal(result.rasterCopies, 1, "Keep the decoded raster's browser backing buffer across frames.");
  for (const { dpr, selected, width, height, values } of result.frames) {
    assert.equal(width, Math.round(fixture.viewport * dpr));
    assert.equal(height, width);
    for (const { name, actual, expected } of values) {
      assert(actual.every((value, index) => Math.abs(value - expected[index]) <= fixture.maximumChannelDifference),
        `DPR ${dpr}, selected ${selected}, ${name}: saw ${actual}, SVG renderer expected ${expected}`);
    }
  }
  console.log(`${engine} Canvas2D matched ${result.frames.length} SVG frames, repainted and picked the edited overlay, and copied the raster once.`);
} finally {
  await browser.close();
}
