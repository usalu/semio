/** 🔬️ Flow ABI pump recon: boots the procedural 3d React editor, waits for the node-graph board, and
 * reads how the flow surface's own wasm bridge behaved — how many frames it presented, how long each
 * present took, and (when the pump is instrumented) how many ABI messages one operation cost.
 *
 * 🩺️ This is the probe that found the real ceiling behind "scrolling in the flow takes seconds to
 * render": the board's own present was taking 200–450 ms idle and over 2.5 s under a gesture, and the
 * pump was draining 68 000 ABI messages per second on an IDLE shell, 99.9 % of them
 * `FLOW_EVENT_PROGRESS` — one per BYTE of every operation's payload
 * (`📓️flow-scroll-render-perf-2026-09-15.md` §3).
 *
 * 🧪️ The message census is not always armed: it needs three temporary counters inside
 * `🌊️flow/🫀️core/🕸️bindings/🖥️host/🟨️.js`'s `pump`, published on `globalThis.__flowPumpStats` —
 * `{steps, polls, messages, kinds, ops, progressOps}` incremented in `step`, at the `pollExact` call
 * and in `start`. Without them the run still reports the paint spans, which is the number the gate
 * moves. Add the counters when a throughput regression is suspected; take them out afterwards, since
 * a counter in the pump is itself pump cost.
 *
 * Usage: cd <ticket> && bun 🐍️flow-pump-recon.mjs
 */
import { chromium } from "playwright";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6022/?plugin=generation3d&example=hexagonal-mushroom-column";
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
const logs = [];
page.on("console", (message) => {
  const text = message.text();
  if (text.includes("flow surface created") || text.includes("node-graph")) logs.push(text.slice(0, 200));
});
await page.goto(url, { waitUntil: "domcontentloaded" });

/** 📐️ The node-graph board's rect, once the example's own graph is installed — never the fallback. */
const boardRect = () =>
  page.evaluate(() => {
    const entry = Object.values(window.__semioFlowGraphProbe ?? {})[0];
    const rect = entry?.rect?.();
    let widgets = 0;
    try { widgets = (JSON.parse(entry?.hostSnapshotJson?.() ?? "{}").widgets ?? []).length; } catch { widgets = 0; }
    return rect && widgets >= 3 ? { ...rect, widgets } : null;
  });

let rect = null;
for (let second = 0; second < 90; second += 1) {
  await page.waitForTimeout(1000);
  rect = await boardRect();
  if (rect) break;
}
console.log("[DEBUG] rect", JSON.stringify(rect));
console.log(logs.join("\n"));
if (!rect) {
  await browser.close();
  process.exit(1);
}

const pumpStats = () => page.evaluate(() => ({ ...(globalThis.__flowPumpStats ?? {}) }));
const numericDelta = (before, after) => JSON.stringify(Object.fromEntries(Object.keys(after).filter((key) => typeof after[key] === "number").map((key) => [key, Math.round((after[key] - before[key]) * 10) / 10])));

await page.waitForTimeout(2000);
const idleBefore = await pumpStats();
await page.waitForTimeout(2000);
const idleAfter = await pumpStats();
console.log("[DEBUG] idle 2s delta", numericDelta(idleBefore, idleAfter));
console.log("[DEBUG] idle kinds", JSON.stringify(idleAfter.kinds ?? null), "ops", JSON.stringify(idleAfter.ops ?? null), "progressOps", JSON.stringify(idleAfter.progressOps ?? null));

const cx = Math.round(rect.x + rect.width / 2);
const cy = Math.round(rect.y + rect.height / 2);
await page.mouse.move(cx, cy);
const wheelBefore = await pumpStats();
const startedAt = Date.now();
for (let tick = 0; tick < 30; tick += 1) {
  await page.mouse.wheel(0, -120);
  await page.waitForTimeout(16);
}
await page.waitForTimeout(1500);
const wheelAfter = await pumpStats();
console.log("[DEBUG] wheel window wall", Date.now() - startedAt, "delta", numericDelta(wheelBefore, wheelAfter));
console.log("[DEBUG] wheel kinds", JSON.stringify(wheelAfter.kinds ?? null));

const paints = await page.evaluate(() =>
  performance
    .getEntriesByType("measure")
    .filter((entry) => entry.name === "semio.hop.surface.paint")
    .map((entry) => ({ atMs: Math.round(entry.startTime), durationMs: Math.round(entry.duration) })),
);
console.log("[DEBUG] paints", paints.length, "last", JSON.stringify(paints.slice(-8)));
await browser.close();
