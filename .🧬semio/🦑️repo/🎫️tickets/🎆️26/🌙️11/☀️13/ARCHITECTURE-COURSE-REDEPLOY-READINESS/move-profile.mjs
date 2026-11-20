/** 🔬️ Cost of one pointer move over the overview: handler time per move (capture start to the last window listener, microtasks included), long tasks and the top inclusive functions. Env: RATE, MOVES, SITE. */
import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/.cache/b/d/m/j/b87aa6c238609ac86876/node_modules/@playwright/test/");
const { chromium } = require("@playwright/test");
const rate = Number(process.env.RATE ?? 1), moves = Number(process.env.MOVES ?? 40), site = process.env.SITE ?? "http://127.0.0.1:6068";
const browser = await chromium.launch();
const context = await browser.newContext({ viewport: { width: 1280, height: 800 }, locale: "en" });
const page = await context.newPage();
const cdp = await context.newCDPSession(page);
await cdp.send("Profiler.enable");
await cdp.send("Profiler.setSamplingInterval", { interval: 100 });
await page.addInitScript(() => {
  window.__raf = [];
  const original = window.requestAnimationFrame.bind(window);
  window.requestAnimationFrame = (callback) => original((time) => { const t = performance.now(); callback(time); window.__raf.push({ name: callback.name || "(anon)", ms: performance.now() - t }); });
});
await page.goto(site + "/", { waitUntil: "commit" });
await page.locator('[data-slot="introduction-close"]').waitFor({ timeout: 240_000 });
await page.locator('[data-slot="introduction-close"]').click({ timeout: 240_000 });
await page.locator('[data-slot="introduction-info-box"]').waitFor({ state: "detached", timeout: 120_000 });
await page.waitForTimeout(3000 * rate);
if (rate !== 1) await cdp.send("Emulation.setCPUThrottlingRate", { rate });
await page.evaluate(() => {
  window.__m = { t0: 0, spans: [] };
  window.__raf.length = 0;
  window.addEventListener("pointermove", () => { window.__m.t0 = performance.now(); }, true);
  window.addEventListener("pointermove", () => { window.__m.spans.push(performance.now() - window.__m.t0); });
});
const calibrationMs = await page.evaluate(() => { const t = performance.now(); let x = 0; for (let i = 0; i < 5e7; i++) x += i % 7; return +(performance.now() - t).toFixed(0) + 0 * x; });
await cdp.send("Profiler.start");
for (let i = 0; i < moves; i++) {
  await page.mouse.move(200 + (i * 700) / moves, 300 + (i % 2) * 40);
  await page.waitForTimeout(30 * rate);
}
await page.waitForTimeout(500 * rate);
const { profile } = await cdp.send("Profiler.stop");
const spans = await page.evaluate(() => window.__m.spans);
const raf = await page.evaluate(() => window.__raf);
const stat = (name) => { const list = raf.filter((entry) => entry.name === name).map((entry) => entry.ms).sort((a, b) => a - b); return list.length === 0 ? null : { n: list.length, median: +list[Math.floor(list.length / 2)].toFixed(1), p90: +list[Math.floor(list.length * 0.9)].toFixed(1), max: +list.at(-1).toFixed(1) }; };
const frames = Object.fromEntries([...new Set(raf.map((entry) => entry.name))].map((name) => [name, stat(name)]));
spans.sort((a, b) => a - b);
const parent = new Map(); const byId = new Map(profile.nodes.map((n) => [n.id, n]));
for (const n of profile.nodes) for (const c of n.children ?? []) parent.set(c, n.id);
const incl = new Map();
const focus = process.env.FOCUS;
profile.samples.forEach((id, i) => { if (focus) { let inside = false; for (let at = id; at !== undefined; at = parent.get(at)) if (byId.get(at).callFrame.functionName === focus) inside = true; if (!inside) return; } const seen = new Set(); for (let at = id; at !== undefined; at = parent.get(at)) { const n = byId.get(at); const key = `${n.callFrame.functionName || "(anon)"} ${n.callFrame.url.split("/").pop().slice(0, 24)}:${n.callFrame.lineNumber}`; if (seen.has(key)) continue; seen.add(key); incl.set(key, (incl.get(key) ?? 0) + (profile.timeDeltas[i] ?? 0) / 1000); } });
const top = [...incl.entries()].filter(([k]) => !k.startsWith("(")).sort((a, b) => b[1] - a[1]).slice(0, 18).map(([k, v]) => `${(v / spans.length).toFixed(1)}ms/move ${k}`);
console.log(JSON.stringify({ calibrationMs, rate, moves: spans.length, rafMs: frames, handlerMs: { median: +spans[Math.floor(spans.length / 2)].toFixed(1), p90: +spans[Math.floor(spans.length * 0.9)].toFixed(1), max: +spans.at(-1).toFixed(1) }, top }, null, 1));
await browser.close();
