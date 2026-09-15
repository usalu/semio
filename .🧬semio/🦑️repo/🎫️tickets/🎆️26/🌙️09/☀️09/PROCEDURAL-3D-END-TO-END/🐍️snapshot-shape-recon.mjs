/** 🔎️ Prints what the Flow node-graph surface publishes through `window.__semioFlowGraphProbe` — the
 * getters it exposes and the shape of the host snapshot behind them.
 *
 * 🩺️ Written when `flow-reorganize` went red with `moved: []` on 2026-09-15: the probe reads widget
 * positions out of that snapshot's `layout`, and a peer's repo-wide `fixture` → `host_document` /
 * `host_snapshot` rename had moved the getter (`fixtureJson` → `hostSnapshotJson`) AND left the scene
 * field behind it undefined, so the reader saw `null` and reported that nothing moved. The reorganize
 * action itself dispatches and settles (`📓️flow-scroll-render-perf-2026-09-15.md` §9).
 *
 * Usage: cd <ticket> && bun 🐍️snapshot-shape-recon.mjs
 */
import { chromium } from "playwright";
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
await page.goto("http://127.0.0.1:6022/?plugin=generation3d&example=hexagonal-mushroom-column", { waitUntil: "domcontentloaded" });
for (let i = 0; i < 60; i += 1) {
  await page.waitForTimeout(1000);
  const r = await page.evaluate(() => {
    const e = Object.values(window.__semioFlowGraphProbe ?? {})[0];
    if (!e) return null;
    const t = e.hostSnapshotJson?.();
    if (!t) return { getters: Object.keys(e), snapshot: String(t) };
    try { const p = JSON.parse(t); return { keys: Object.keys(p), widgets: (p.widgets ?? []).length, layout: p.layout ? Object.keys(p.layout).length : null }; } catch { return { raw: String(t).slice(0,150) }; }
  });
  if (r && (r.keys || r.getters)) { console.log("[DEBUG]", JSON.stringify(r)); break; }
}
await browser.close();
