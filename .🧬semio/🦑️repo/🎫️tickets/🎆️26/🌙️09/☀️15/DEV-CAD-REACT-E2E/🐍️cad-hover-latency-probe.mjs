import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

// ⏱️ Measures hover latency in the served cad page: for each sample the mouse moves onto / off an
// object and the probe polls `data-guest-selection-json.hoveredId` on the pane's world host until it
// changes, recording ms from the move to the DOM change; console timings for `interactionHover` are
// kept alongside.
const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6020/?plugin=cad";
const bootSeconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 30);
const surface = process.env.SEMIO_PROBE_SURFACE ?? "window:cad-play-building";
const outDir = join(import.meta.dir, "🗑️generated", "hover-latency");
mkdirSync(outDir, { recursive: true });
const lines = [];
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const t0 = Date.now();
page.on("console", (msg) => lines.push(`${Date.now() - t0} ${msg.type()} ${msg.text().slice(0, 600)}`));
await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(bootSeconds * 1000);
const hovered = () => page.evaluate((s) => { try { return JSON.parse(document.querySelector(`[data-surface-id="${s}"]`)?.getAttribute("data-guest-selection-json") ?? "null")?.hoveredId ?? null; } catch { return null; } }, surface);
const rect = await page.evaluate((s) => { const r = document.querySelector(`[data-surface-id="${s}"]`).getBoundingClientRect(); return { x: r.x, y: r.y, w: r.width, h: r.height }; }, surface);
// 🎯 Find a spot that hovers an object and one that hovers nothing.
const spots = [[0.6, 0.5], [0.55, 0.45], [0.65, 0.55], [0.7, 0.5], [0.5, 0.5], [0.62, 0.6]];
let onSpot = null;
for (const [fx, fy] of spots) {
  await page.mouse.move(rect.x + rect.w * fx, rect.y + rect.h * fy);
  await page.waitForTimeout(2500);
  if (await hovered()) { onSpot = [fx, fy]; break; }
}
const offSpot = [0.9, 0.15];
const samples = [];
const measure = async (label, fx, fy, expectNull) => {
  const start = Date.now();
  await page.mouse.move(rect.x + rect.w * fx, rect.y + rect.h * fy);
  let value = await hovered();
  while ((expectNull ? value !== null : value === null) && Date.now() - start < 6000) {
    await page.waitForTimeout(10);
    value = await hovered();
  }
  const ms = Date.now() - start;
  samples.push({ label, ms, value, timedOut: ms >= 6000 });
  await page.waitForTimeout(400);
};
if (onSpot) {
  for (let i = 0; i < 8; i++) {
    await measure(`off→on #${i}`, onSpot[0], onSpot[1], false);
    await measure(`on→off #${i}`, offSpot[0], offSpot[1], true);
  }
  // 🌀 A sweep: 40 moves across the pane in 400 ms, then how long until the final hover state lands.
  const sweepStart = Date.now();
  for (let i = 0; i < 40; i++) { await page.mouse.move(rect.x + rect.w * (0.4 + 0.5 * i / 40), rect.y + rect.h * 0.5); await page.waitForTimeout(10); }
  await page.mouse.move(rect.x + rect.w * onSpot[0], rect.y + rect.h * onSpot[1]);
  let v = await hovered(); const s2 = Date.now();
  while (v === null && Date.now() - s2 < 8000) { await page.waitForTimeout(10); v = await hovered(); }
  samples.push({ label: "sweep→on", ms: Date.now() - s2, sweepTotalMs: Date.now() - sweepStart, value: v });
}
const hoverLog = lines.filter((l) => /interactionHover|command ingress crossed|performInvocation settled/.test(l)).slice(-40);
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "samples.json"), JSON.stringify({ onSpot, samples, hoverLog }, null, 2));
console.log("onSpot", JSON.stringify(onSpot));
for (const s of samples) console.log(JSON.stringify(s));
console.log("--- console tail");
for (const l of hoverLog.slice(-16)) console.log(l.slice(0, 300));
await browser.close();
