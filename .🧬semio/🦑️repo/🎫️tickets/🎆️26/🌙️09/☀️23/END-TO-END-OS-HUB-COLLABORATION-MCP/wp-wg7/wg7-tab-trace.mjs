/** ⌨️ WG7 S12-4 — traces what one Tab press does to focus in the wasm32 wgpu shell: every focusin/focusout (element key or tag),
 * mirror repaints (childList mutations of the mirror root) and the active element sampled every 20 ms for 1.5 s.
 * Usage: SEMIO_PROBE_URL=http://127.0.0.1:6552/?plugin=note node wg7-tab-trace.mjs → generated/tab-trace.json */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
const SHELL = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6552/?plugin=note";
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 }, locale: "en-US" })).newPage();
await page.goto(SHELL, { waitUntil: "domcontentloaded" });
await page.waitForFunction(() => typeof globalThis.semioWgpuIntrospection?.dumpAccessibility === "function", null, { timeout: 180_000 });
await page.waitForTimeout(12_000);
await page.evaluate(() => {
  const name = (element) => element instanceof HTMLElement ? (element.dataset.nodeKey ?? `<${element.tagName.toLowerCase()}${element.id ? "#" + element.id : ""}>`) : String(element);
  const log = (globalThis.__wg7TabTrace = []);
  const t0 = performance.now();
  document.addEventListener("focusin", (event) => log.push({ t: Math.round(performance.now() - t0), kind: "focusin", target: name(event.target) }), true);
  document.addEventListener("focusout", (event) => log.push({ t: Math.round(performance.now() - t0), kind: "focusout", target: name(event.target), next: name(event.relatedTarget) }), true);
  document.addEventListener("keydown", (event) => log.push({ t: Math.round(performance.now() - t0), kind: "keydown", key: event.key, target: name(event.target), prevented: event.defaultPrevented }), false);
  const mirror = document.getElementById("semio-wgpu-accessibility");
  new MutationObserver(() => log.push({ t: Math.round(performance.now() - t0), kind: "repaint", nodes: mirror.dataset.nodeCount, active: name(document.activeElement) })).observe(mirror, { childList: true });
  const sample = () => log.push({ t: Math.round(performance.now() - t0), kind: "sample", active: name(document.activeElement) });
  globalThis.__wg7Sample = sample;
});
const first = await page.evaluate(() => [...document.querySelectorAll("#semio-wgpu-accessibility [data-focusable]")].slice(0, 3).map((element) => ({ key: element.dataset.nodeKey, tabIndex: element.tabIndex, inert: element.closest("[inert]") !== null, hidden: element.closest("[aria-hidden=true]") !== null, display: getComputedStyle(element).display, visibility: getComputedStyle(element).visibility })));
const order = await page.evaluate(() => {
  const canvas = document.getElementById("semio-wgpu-canvas");
  const mirror = document.getElementById("semio-wgpu-accessibility");
  return { canvasBeforeMirror: Boolean(canvas.compareDocumentPosition(mirror) & Node.DOCUMENT_POSITION_FOLLOWING), mirrorParent: mirror.parentElement?.id ?? mirror.parentElement?.tagName, canvasParent: canvas.parentElement?.id ?? canvas.parentElement?.tagName };
});
await page.locator("#semio-wgpu-canvas").focus();
for (let press = 0; press < 3; press += 1) {
  await page.keyboard.press("Tab");
  for (let tick = 0; tick < 25; tick += 1) {
    await page.evaluate(() => globalThis.__wg7Sample());
    await page.waitForTimeout(20);
  }
}
const direct = await page.evaluate(() => {
  const target = document.querySelector("#semio-wgpu-accessibility [data-focusable]");
  target?.focus();
  return document.activeElement?.dataset?.nodeKey ?? document.activeElement?.tagName;
});
const trace = await page.evaluate(() => globalThis.__wg7TabTrace);
writeFileSync("generated/tab-trace.json", JSON.stringify({ first, order, direct, trace }, null, 1));
console.log(JSON.stringify({ first, order, direct, events: trace.filter((row) => row.kind !== "sample").slice(0, 30), samples: [...new Set(trace.filter((row) => row.kind === "sample").map((row) => row.active))] }));
await browser.close();
