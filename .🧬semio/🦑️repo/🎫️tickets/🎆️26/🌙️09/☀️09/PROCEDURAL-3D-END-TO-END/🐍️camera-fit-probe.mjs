/** 📷️ Runtime proof for the node-graph opening camera and node captions (ticket 26/09/09).
 *
 * Two measurements, neither an impression:
 *   • CAPTIONS — every `fillText` the label overlay issues is recorded (string, anchor, font px) by
 *     an init script that wraps `CanvasRenderingContext2D.prototype.fillText` before the app loads.
 *     "the titles are full words" is then the recorded strings, and the boot defect (`E` for the
 *     `extrude` node) is a single-glyph caption in that list.
 *   • FRAMING — a caption's anchor is inside the overlay canvas exactly when its node is on screen,
 *     so "all seven nodes framed" is: every widget the graph holds has a caption anchored inside the
 *     surface. The `[DEBUG] node-graph fit on open` console line says the decision itself fired.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=<name> [SEMIO_PROBE_WEBGPU=1] bun 🐍️camera-fit-probe.mjs
 * Launch args are copied from 🐍️canvas-paint-probe.mjs so the two runs are comparable.
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6048/?plugin=generation3d";
const webgpu = process.env.SEMIO_PROBE_WEBGPU === "1";
const settleMs = Number(process.env.SEMIO_PROBE_SECONDS ?? 110) * 1000;
const outDir = join(import.meta.dir, "🗑️generated", "camera-fit", process.env.SEMIO_PROBE_OUT ?? (webgpu ? "webgpu" : "default"));
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: webgpu ? ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] : [] });
const page = await browser.newPage({ viewport: { width: Number(process.env.SEMIO_PROBE_WIDTH ?? 1440), height: Number(process.env.SEMIO_PROBE_HEIGHT ?? 900) } });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 1200)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 1200)}`));

//#region 🖋️FillTextRecorder
await page.addInitScript(() => {
  const record = [];
  globalThis.__semioFillText = record;
  const original = CanvasRenderingContext2D.prototype.fillText;
  CanvasRenderingContext2D.prototype.fillText = function patched(text, x, y, ...rest) {
    try {
      record.push({ text: String(text), x, y, font: this.font, canvasWidth: this.canvas.width, canvasHeight: this.canvas.height, at: Date.now() });
      if (record.length > 4000) record.splice(0, 2000);
    } catch {
      /* recording must never break the paint */
    }
    return original.call(this, text, x, y, ...rest);
  };
});
//#endregion 🖋️FillTextRecorder

await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(settleMs);

const surface = '[data-surface-id="window:procedural-main"]';

//#region 🎨️ExamplePick
// 🔀️ A fresh browser profile boots the DEFAULT flow document (two widgets). The seven-widget graph
// this lane is about is the bundled `Hexagonal Mushroom Column` example, picked through the navbar
// combobox — which is also the example-switch half of the camera law.
const pick = process.env.SEMIO_PROBE_PICK ?? "Hexagonal Mushroom Column";
let picked = null;
if (pick) {
  try {
    const before = (await page.evaluate(() => globalThis.__semioFillText ?? [])).length;
    const combo = page.locator('[role="combobox"]').first();
    await combo.click({ timeout: 8000 });
    await page.waitForTimeout(600);
    await page.locator('[role="option"]').filter({ hasText: pick }).first().click({ timeout: 8000 });
    await page.waitForTimeout(Number(process.env.SEMIO_PROBE_PICK_SETTLE_MS ?? 90000));
    picked = { pick, fillTextBefore: before, fillTextAfter: (await page.evaluate(() => globalThis.__semioFillText ?? [])).length };
  } catch (error) {
    picked = { pick, error: String(error).slice(0, 300) };
  }
}
console.log("[DEBUG] example pick", JSON.stringify(picked));
//#endregion 🎨️ExamplePick
//#region 🖼️FitGraphAction
// 🖼️ The `Fit graph` action itself: pan the camera far away with the wheel, then reach the control by
// KEYBOARD (Tab to it, Enter) and by click, and check the graph is framed again — and that the fitted
// camera is persisted through `nodeGraphViewport` exactly like a gesture.
let fitAction = null;
if (process.env.SEMIO_PROBE_FIT_CLICK === "1") {
  const surfaceBox = await page.locator(surface).first().boundingBox().catch(() => null);
  const button = page.getByRole("button", { name: process.env.SEMIO_PROBE_FIT_LABEL ?? "Fit graph" }).first();
  const visible = await button.isVisible().catch(() => false);
  if (surfaceBox && visible) {
    await page.mouse.move(surfaceBox.x + surfaceBox.width / 2, surfaceBox.y + surfaceBox.height / 2);
    for (let step = 0; step < 12; step += 1) {
      await page.mouse.wheel(0, -240);
      await page.waitForTimeout(250);
    }
    await page.waitForTimeout(6000);
    const awayCaptions = (await page.evaluate(() => globalThis.__semioFillText ?? [])).slice(-12).map((row) => row.text);
    const mark = lines.length;
    await button.focus();
    const focused = await page.evaluate(() => document.activeElement?.textContent ?? null);
    await page.keyboard.press("Enter");
    await page.waitForTimeout(12000);
    fitAction = {
      awayCaptions,
      keyboardFocused: focused,
      viewportDispatches: lines.slice(mark).filter((line) => /nodeGraphViewport/.test(line)).length,
      afterFit: (await page.evaluate(() => globalThis.__semioFillText ?? [])).slice(-12).map((row) => row.text),
    };
    await page.screenshot({ path: join(outDir, "3-after-fit-action.png"), clip: surfaceBox });
  } else {
    fitAction = { error: `fit control ${visible ? "found" : "not found"}, surface ${surfaceBox ? "found" : "missing"}` };
  }
}
console.log("[DEBUG] fit action", JSON.stringify(fitAction));
//#endregion 🖼️FitGraphAction

const box = await page.locator(surface).first().boundingBox().catch(() => null);
await page.screenshot({ path: join(outDir, "1-settled.png") });
if (box) await page.screenshot({ path: join(outDir, "2-surface.png"), clip: box });

const drawn = await page.evaluate(() => globalThis.__semioFillText ?? []);
// 🕒️ Only the LAST paint pass matters: every repaint re-records the same strings.
const lastCanvasSize = drawn.length > 0 ? { width: drawn.at(-1).canvasWidth, height: drawn.at(-1).canvasHeight } : null;
const lastPassStart = (() => {
  if (drawn.length === 0) return 0;
  const tail = drawn.at(-1).at;
  for (let index = drawn.length - 1; index > 0; index -= 1) if (tail - drawn[index].at > 500) return index + 1;
  return 0;
})();
const lastPass = drawn.slice(lastPassStart);
const captions = lastPass.filter((row) => !row.text.startsWith("!"));
const ports = lastPass.filter((row) => row.text.startsWith("!"));
const singleGlyph = captions.filter((row) => [...row.text.trim()].length <= 1);
const offScreen = lastPass.filter((row) => !lastCanvasSize || row.x < 0 || row.y < 0 || row.x > lastCanvasSize.width || row.y > lastCanvasSize.height);
const report = {
  webgpu,
  picked,
  fitAction,
  url,
  canvas: lastCanvasSize,
  totalFillTextCalls: drawn.length,
  lastPass: lastPass.map((row) => ({ text: row.text, x: Math.round(row.x), y: Math.round(row.y), font: row.font })),
  allPasses: drawn.map((row) => ({ text: row.text, x: Math.round(row.x), y: Math.round(row.y), at: row.at })),
  captions: captions.map((row) => row.text),
  portLabels: [...new Set(ports.map((row) => row.text))],
  singleGlyphCaptions: singleGlyph.map((row) => row.text),
  offScreenAnchors: offScreen.length,
  fitOnOpen: lines.filter((line) => /node-graph fit on open|node-graph refit after graph change/.test(line)),
  presentLines: lines.filter((line) => /flow surface created/.test(line)),
};
writeFileSync(join(outDir, "captions.json"), JSON.stringify(report, null, 2));
console.log("[DEBUG] captions", JSON.stringify(report.captions));
console.log("[DEBUG] port labels", JSON.stringify(report.portLabels));
console.log("[DEBUG] single-glyph captions", JSON.stringify(report.singleGlyphCaptions));
console.log("[DEBUG] fit on open", JSON.stringify(report.fitOnOpen));

//#region 📏️Checks
const failures = [];
const check = (law, held) => { if (!held) failures.push(law); };
// 🕸️ The bundled `hexagonal-mushroom-column` holds 7 widgets; the preview widget is uncaptioned by
// `node_label_text`, so six captions is the whole graph.
check(`captions-drawn(${report.captions.length})`, report.captions.length >= 6);
check(`no-single-glyph-caption(${JSON.stringify(report.singleGlyphCaptions)})`, report.singleGlyphCaptions.length === 0);
check(`every-caption-on-screen(${report.offScreenAnchors} off)`, report.offScreenAnchors === 0);
check("fit-decision-logged", report.fitOnOpen.length > 0);
writeFileSync(join(outDir, "camera-fit-check.json"), JSON.stringify({ webgpu, failures }, null, 2));
console.log("[DEBUG] camera fit check", failures.length === 0 ? "PASSED" : `FAILED ${JSON.stringify(failures)}`);
//#endregion 📏️Checks

writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("DONE lines", lines.length, "webgpu", webgpu, "out", outDir);
await browser.close();
if (failures.length > 0) process.exit(1);
