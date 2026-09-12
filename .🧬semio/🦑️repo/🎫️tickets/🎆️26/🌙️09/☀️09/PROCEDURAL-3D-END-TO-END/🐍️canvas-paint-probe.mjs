/** 🎨 Runtime proof for the flow node-graph canvas paint path (ticket 26/09/09, canvas-paint lane).
 *
 * Reads the PIXELS of the NodeGraph surface's own canvases — not a page screenshot — so "the graph is
 * drawn" is a measurement (distinct colours, ink coverage, ink bounding box) rather than an
 * impression, and the two failure shapes this lane closes are directly falsifiable:
 *   • the placeholder's solid black `fillRect` — counted as pure-black ink,
 *   • raw-world-coordinate placement — the ink bounding box against the camera's framing.
 * Then it hovers and clicks a node to prove host-side picking still round-trips after the paint change.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=<name> [SEMIO_PROBE_WEBGPU=1] bun 🐍️canvas-paint-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const webgpu = process.env.SEMIO_PROBE_WEBGPU === "1";
const settleMs = Number(process.env.SEMIO_PROBE_SECONDS ?? 100) * 1000;
const outDir = join(import.meta.dir, "🗑️generated", "canvas-paint", process.env.SEMIO_PROBE_OUT ?? (webgpu ? "webgpu" : "default"));
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: webgpu ? ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] : [] });
const page = await browser.newPage({ viewport: { width: Number(process.env.SEMIO_PROBE_WIDTH ?? 1440), height: Number(process.env.SEMIO_PROBE_HEIGHT ?? 900) } });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 1200)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 1200)}`));

await page.goto(url, { waitUntil: "domcontentloaded" });

const surface = '[data-surface-id="window:procedural-main"]';
const readInk = () =>
  page.evaluate((selector) => {
    const host = document.querySelector(selector);
    if (!host) return { error: "no-node-graph-host" };
    const canvases = [...host.querySelectorAll("canvas")];
    return canvases.map((canvas, index) => {
      const context = canvas.getContext("2d");
      if (!context) return { index, width: canvas.width, height: canvas.height, context: "not-2d" };
      const { data } = context.getImageData(0, 0, canvas.width, canvas.height);
      const colours = new Map();
      let ink = 0;
      let pureBlack = 0;
      let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
      // 🎨 The clear colour is whatever the first painted pixel is; ink is anything unlike it.
      const base = `${data[0]},${data[1]},${data[2]},${data[3]}`;
      for (let pixel = 0; pixel < data.length; pixel += 4) {
        const key = `${data[pixel]},${data[pixel + 1]},${data[pixel + 2]},${data[pixel + 3]}`;
        colours.set(key, (colours.get(key) ?? 0) + 1);
        if (data[pixel] === 0 && data[pixel + 1] === 0 && data[pixel + 2] === 0 && data[pixel + 3] === 255) pureBlack += 1;
        if (key === base || data[pixel + 3] === 0) continue;
        ink += 1;
        const at = pixel / 4;
        const x = at % canvas.width;
        const y = Math.floor(at / canvas.width);
        if (x < minX) minX = x;
        if (y < minY) minY = y;
        if (x > maxX) maxX = x;
        if (y > maxY) maxY = y;
      }
      const top = [...colours.entries()].sort((a, b) => b[1] - a[1]).slice(0, 6);
      return {
        index, width: canvas.width, height: canvas.height, clear: base,
        distinctColours: colours.size, inkPixels: ink, pureBlackPixels: pureBlack,
        inkBounds: ink === 0 ? null : { x: minX, y: minY, width: maxX - minX + 1, height: maxY - minY + 1 },
        topColours: top,
      };
    });
  }, surface);

await page.waitForTimeout(settleMs);
const settled = await readInk();
writeFileSync(join(outDir, "ink-settled.json"), JSON.stringify(settled, null, 2));
console.log("[DEBUG] ink settled", JSON.stringify(settled));
await page.screenshot({ path: join(outDir, "1-settled.png") });
const box = await page.locator(surface).first().boundingBox().catch(() => null);
if (box) await page.screenshot({ path: join(outDir, "2-surface.png"), clip: box });

//#region 🖱️HoverSelect
const interactions = [];
if (box) {
  const centre = { x: box.x + box.width / 2, y: box.y + box.height / 2 };
  for (const [label, point] of [
    ["centre", centre],
    ["upper-left-quadrant", { x: box.x + box.width * 0.3, y: box.y + box.height * 0.3 }],
    ["lower-right-quadrant", { x: box.x + box.width * 0.7, y: box.y + box.height * 0.6 }],
  ]) {
    const before = lines.length;
    await page.mouse.move(point.x, point.y);
    await page.waitForTimeout(900);
    const hoverLines = lines.slice(before).filter((line) => /interactionHover|pickTargets|hover/i.test(line)).length;
    const beforeClick = lines.length;
    await page.mouse.click(point.x, point.y);
    await page.waitForTimeout(1400);
    const selectLines = lines.slice(beforeClick).filter((line) => /interactionSelect/i.test(line)).length;
    interactions.push({ label, point, hoverLines, selectLines });
  }
  await page.screenshot({ path: join(outDir, "3-after-interaction.png"), clip: box });
  const afterInteraction = await readInk();
  writeFileSync(join(outDir, "ink-after-interaction.json"), JSON.stringify(afterInteraction, null, 2));
  console.log("[DEBUG] ink after interaction", JSON.stringify(afterInteraction));
}
writeFileSync(join(outDir, "interactions.json"), JSON.stringify(interactions, null, 2));
console.log("[DEBUG] interactions", JSON.stringify(interactions));
//#endregion 🖱️HoverSelect

//#region 🔭️ZoomToWholeGraph
// 📐️ The document's own camera (zoom 1.784) was authored for a full-width window; in today's split the
// Flow window is half that, so only part of the graph is framed. Wheel out over the canvas so the whole
// 7-node column is on screen — which also exercises the camera end to end, since the draw list carries
// the camera in every command's affine and nothing else in the replay knows about it.
if (box) {
  const centre = { x: box.x + box.width / 2, y: box.y + box.height / 2 };
  await page.mouse.move(centre.x, centre.y);
  for (let step = 0; step < 8; step += 1) {
    await page.mouse.wheel(0, 240);
    await page.waitForTimeout(700);
  }
  // ⏱️ The flow session answers a render behind whatever eval work is queued; measured at 10-30 s on a
  // cold boot, so a zoom needs a long settle before its repaint can be read off the canvas.
  await page.waitForTimeout(Number(process.env.SEMIO_PROBE_ZOOM_SETTLE_MS ?? 70000));
  await page.screenshot({ path: join(outDir, "4-zoomed-out.png"), clip: box });
  const zoomed = await readInk();
  writeFileSync(join(outDir, "ink-zoomed-out.json"), JSON.stringify(zoomed, null, 2));
  console.log("[DEBUG] ink zoomed out", JSON.stringify(zoomed));
}
//#endregion 🔭️ZoomToWholeGraph

//#region 📏️PixelStatisticsCheck
// 🧪️ The runtime half of the "no adapter -> the graph is still painted" law, as pixel STATISTICS
// rather than a golden image: resolution- and theme-independent, and it names the exact failure
// shapes. The 2D thresholds apply to the deviceless path — the path a viewer without WebGPU gets,
// and the one this lane found silently unpainted. A WebGPU canvas cannot be read back with
// `getImageData` at all (`getContext("2d")` is null on it by definition), so there the check is that
// the guest really took the GPU exit and never fell back onto an element it could not paint;
// `4-zoomed-out.png` is that path's picture. The node-bounds half of the law lives where it can be
// exact: `🌊️flow/🕸️wasm/🧪️tests/🎬️draw-list/{🦀️.rs,🟨️.js}`.
const failures = [];
const check = (law, held) => { if (!held) failures.push(law); };
for (const [label, reading] of [["settled", settled], ["zoomed-out", await readInk()]]) {
  const scene = Array.isArray(reading) ? reading[0] : undefined;
  if (!scene) { check(`${label}:scene-canvas-present`, false); continue; }
  if (scene.context === "not-2d") {
    check(`${label}:gpu-present-reported`, lines.some((line) => /flow surface created .*webgpu/.test(line)));
    check(`${label}:never-unpresentable`, !lines.some((line) => /cannot replay/.test(line)));
    continue;
  }
  const area = scene.width * scene.height;
  // 📐️ Measured on this window (483x814 = 393 162 px): 393 022 ink px at the document's own camera,
  // 78 702 zoomed out — a wide-open grid is legitimately sparser, since "ink" is everything unlike
  // the clear colour. The bar is a tenth of the surface: three orders of magnitude above the broken
  // reading this lane closed (0 px on a canvas that could not give a 2D context) and well above the
  // label overlay's own 311 px, while leaving the LOD room to breathe.
  check(`${label}:ink-order-of-magnitude(${scene.inkPixels}/${area})`, scene.inkPixels >= area * 0.1);
  check(`${label}:no-pure-black-placeholder(${scene.pureBlackPixels})`, scene.pureBlackPixels === 0);
  check(`${label}:distinct-colours(${scene.distinctColours})`, scene.distinctColours >= 100);
  check(`${label}:ink-spans-the-layout(${JSON.stringify(scene.inkBounds)})`, (scene.inkBounds?.width ?? 0) >= scene.width * 0.6 && (scene.inkBounds?.height ?? 0) >= scene.height * 0.6);
}
writeFileSync(join(outDir, "pixel-statistics-check.json"), JSON.stringify({ webgpu, failures }, null, 2));
console.log("[DEBUG] pixel statistics check", failures.length === 0 ? "PASSED" : `FAILED ${JSON.stringify(failures)}`);
//#endregion 📏️PixelStatisticsCheck

writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("DONE lines", lines.length, "webgpu", webgpu, "out", outDir);
await browser.close();
if (failures.length > 0) process.exit(1);
