/** 🔬️ Slice B3a2 — is the gis2d **tiled-map lane** actually rendering, or only mounted?
 *
 * The map paints into a canvas with no data attributes, so "the surface exists" and "the surface has
 * pixels" are different claims. This records every `/osm` tile-proxy request the shell makes, samples
 * the canvas for non-uniform pixel content, and reports the `window:gis2d-main` surface geometry.
 */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { writeFileSync } from "node:fs";

const PORT = process.env.SEMIO_PROBE_PORT ?? "6040";
const VARIANT = process.env.SEMIO_PROBE_VARIANT ?? "gis2d";
const OUT = `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated/b3a2-${VARIANT}-tile-lane.txt`;
const lines = [];
const tiles = [];
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("response", (r) => { if (/\/(osm|dem)\//.test(r.url())) tiles.push(`${r.status()} ${r.url().slice(0, 120)}`); });

await page.goto(`http://127.0.0.1:${PORT}/?plugin=${VARIANT}`, { waitUntil: "domcontentloaded", timeout: 120_000 });
for (let i = 0; i < 90; i++) { await page.waitForTimeout(1000); if (await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready"))) break; }
await page.waitForTimeout(15_000);

lines.push("## tile-proxy responses");
lines.push(`count=${tiles.length}`);
lines.push(...tiles.slice(0, 20));
lines.push(`statuses=${JSON.stringify([...new Set(tiles.map((t) => t.split(" ")[0]))])}`);

lines.push("\n## surface + canvas pixels");
lines.push(JSON.stringify(await page.evaluate(() => {
  const surface = document.querySelector('[data-surface-id]');
  const canvas = document.querySelector("canvas");
  const box = surface?.getBoundingClientRect();
  let sample = null;
  if (canvas instanceof HTMLCanvasElement) {
    // 🎨️ A WebGL/WebGPU canvas answers a blank 2d context, so read the composited pixels instead:
    // count distinct colours over a coarse grid. A blank surface answers 1.
    try {
      const gl = canvas.getContext("webgl2") ?? canvas.getContext("webgl");
      if (gl !== null) {
        const pixels = new Uint8Array(canvas.width * canvas.height * 4);
        gl.readPixels(0, 0, canvas.width, canvas.height, gl.RGBA, gl.UNSIGNED_BYTE, pixels);
        const colours = new Set();
        for (let i = 0; i < pixels.length; i += 4 * 977) colours.add(`${pixels[i]},${pixels[i + 1]},${pixels[i + 2]}`);
        sample = { via: "webgl.readPixels", distinctColours: colours.size, first: [...colours].slice(0, 6) };
      }
    } catch (error) { sample = { via: "webgl.readPixels", error: String(error).slice(0, 200) }; }
  }
  return {
    surfaceId: surface?.getAttribute("data-surface-id") ?? null,
    surfaceBox: box === undefined ? null : { w: Math.round(box.width), h: Math.round(box.height) },
    canvas: canvas instanceof HTMLCanvasElement ? { w: canvas.width, h: canvas.height } : null,
    sample,
  };
}), null, 1));

lines.push("\n## screenshot");
const shot = `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated/b3a2-${VARIANT}-tile-lane.png`;
await page.screenshot({ path: shot });
lines.push(shot);

writeFileSync(OUT, lines.join("\n"));
console.log(lines.join("\n"));
await browser.close();
