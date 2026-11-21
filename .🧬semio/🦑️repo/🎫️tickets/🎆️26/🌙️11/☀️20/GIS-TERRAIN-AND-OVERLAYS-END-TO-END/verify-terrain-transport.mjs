import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { chromium } from "playwright";
import { PNG } from "pngjs";
import ts from "typescript";

const source = readFileSync("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗺️WorldTerrainLayer/🟦️.tsx", "utf8");
const region = source.match(/\/\/#region TerrainTileRenderer([\s\S]+?)\/\/#endregion TerrainTileRenderer/)[1];
const renderer = ts.transpileModule(region, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
const fixture = JSON.parse(readFileSync("🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🧪️tests/🧫️fixtures/🏔️terrarium-tile/🔣️.json", "utf8"));

const image = new PNG({ width: 256, height: 256 });
for (let offset = 0; offset < image.data.length; offset += 4) image.data.set(fixture.pixel, offset);
const tile = PNG.sync.write(image);
const { data } = PNG.sync.read(tile);
assert.equal((data.readUInt16BE(0) + data[2] / 256 - 32768) * fixture.exaggeration, fixture.expectedMeshHeight);

const browser = await chromium.launch({ channel: "chrome", headless: true });
try {
  const page = await browser.newPage();
  let firstAttempts = 0;
  let releaseStale;
  const staleStarted = new Promise((resolve) => { releaseStale = resolve; });
  await page.route("http://terrain.test/**", async (route) => {
    const path = new URL(route.request().url()).pathname;
    if (path === "/") return route.fulfill({ contentType: "text/html", body: "<!doctype html><title>Terrain probe</title>" });
    if (path.endsWith("/11/1079/672.png") && ++firstAttempts === 1) return route.fulfill({ status: 503 });
    if (path.endsWith("/11/1080/672.png")) {
      releaseStale(route);
      return;
    }
    return route.fulfill({ contentType: "image/png", body: tile });
  });
  await page.goto("http://terrain.test/");
  await page.addScriptTag({ content: `
    const publishedPageUrl = (url) => url;
    const state = window.state = { uploaded: [], frames: [], freed: 0, aborted: [] };
    const nativeFetch = window.fetch.bind(window);
    window.fetch = (url, options) => {
      options.signal.addEventListener("abort", () => state.aborted.push(url), { once: true });
      return nativeFetch(url, options);
    };
    const createTerrainSession = async () => ({
      set_project_origin() {},
      set_exaggeration() {},
      visible_terrain_tiles_json(camera) { return camera; },
      upload_elevation_tile(z, x, y, bytes) {
        if (bytes[0] !== 137 || bytes[1] !== 80 || bytes[2] !== 78 || bytes[3] !== 71) return false;
        state.uploaded.push([z, x, y].join("/"));
        return true;
      },
      terrain_tile_mesh_json() { return "{}"; },
      evict_terrain_tile() {},
      free() { state.freed++; },
    });
    const geometryFromTerrainMesh = () => ({ dispose() {} });
    ${renderer}
    window.makeRenderer = (tileUrlTemplate) => new TerrainTileRenderer({ tileUrlTemplate }, (geometries) => state.frames.push([...geometries.keys()]));
  ` });

  const center = fixture.expectedCenter;
  await page.evaluate(async (row) => {
    const renderer = window.renderer = window.makeRenderer("http://terrain.test/dem/{z}/{x}/{y}.png");
    renderer.scheduleRefresh(JSON.stringify([row]));
    await renderer.init();
  }, center);
  await page.waitForFunction((key) => window.state.frames.at(-1)?.includes(key), center.key);
  assert.equal(firstAttempts, 2);
  await page.evaluate((row) => window.renderer.scheduleRefresh(JSON.stringify([{ ...row, x: 1080, key: "11/1080/672" }])), center);
  const staleRoute = await staleStarted;
  await page.evaluate((row) => window.renderer.scheduleRefresh(JSON.stringify([{ ...row, x: 1081, key: "11/1081/672" }])), center);
  await page.waitForFunction(() => window.state.frames.at(-1)?.includes("11/1081/672"));
  await staleRoute.abort().catch(() => {});
  const local = await page.evaluate(() => { window.renderer.dispose(); return window.state; });
  assert.deepEqual(local.uploaded, [center.key, "11/1081/672"]);
  assert.ok(local.aborted.some((url) => url.endsWith("/11/1080/672.png")));
  assert.equal(local.freed, 1);

  await page.evaluate(async (row) => {
    window.state.frames = [];
    const renderer = window.renderer = window.makeRenderer("https://s3.amazonaws.com/elevation-tiles-prod/terrarium/{z}/{x}/{y}.png");
    renderer.scheduleRefresh(JSON.stringify([row]));
    await renderer.init();
  }, center);
  await page.waitForFunction((key) => window.state.frames.at(-1)?.includes(key), center.key, { timeout: 15000 });
  await page.evaluate(() => window.renderer.dispose());
  console.log("[DEBUG] Chrome verified retry, camera cancellation, disposal, and cross-origin terrain PNG delivery");
} finally {
  await browser.close();
}
