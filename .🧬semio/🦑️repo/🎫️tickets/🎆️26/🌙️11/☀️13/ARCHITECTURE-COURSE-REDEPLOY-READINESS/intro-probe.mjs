/** 🔬️ Records how long the introduction's Skip button keeps moving after it first shows, and where an immediate click lands. Env: RATE, TRIALS, SITE. */
import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/.cache/b/d/m/j/b87aa6c238609ac86876/node_modules/@playwright/test/");
const { chromium } = require("@playwright/test");
const rate = Number(process.env.RATE ?? 30), trials = Number(process.env.TRIALS ?? 3), site = process.env.SITE ?? "http://127.0.0.1:6068";
const browser = await chromium.launch();
for (let trial = 0; trial < trials; trial++) {
  const context = await browser.newContext({ viewport: { width: 1280, height: 800 }, locale: "en" });
  const page = await context.newPage();
  await page.addInitScript(() => {
    const log = (window.__probe = { rects: [], events: [], first: 0 });
    const read = () => {
      const b = document.querySelector('[data-slot="introduction-close"]');
      if (b) {
        const r = b.getBoundingClientRect();
        if (!log.first) log.first = performance.now();
        const last = log.rects.at(-1);
        if (!last || last.x !== r.x || last.y !== r.y) log.rects.push({ t: Math.round(performance.now() - log.first), x: r.x, y: r.y, w: r.width, h: r.height, id: b.__id ??= Math.random().toString(36).slice(2, 6) });
      }
      requestAnimationFrame(read);
    };
    requestAnimationFrame(read);
    for (const type of ["pointerdown", "pointerup", "click"]) document.addEventListener(type, (e) => log.events.push({ type, t: Math.round(performance.now() - log.first), x: e.clientX, y: e.clientY, target: (e.target.closest?.("[data-slot]")?.dataset.slot) ?? e.target.tagName }), true);
  });
  const cdp = await context.newCDPSession(page);
  await cdp.send("Emulation.setCPUThrottlingRate", { rate });
  await cdp.send("Network.enable");
  await cdp.send("Network.emulateNetworkConditions", { offline: false, latency: 150, downloadThroughput: (1.6 * 1024 * 1024) / 8, uploadThroughput: (750 * 1024) / 8 });
  await page.goto(site + "/", { waitUntil: "commit" });
  await page.waitForFunction(() => window.__probe?.first > 0, null, { timeout: 240_000, polling: 50 });
  const box = await page.evaluate(() => { const r = document.querySelector('[data-slot="introduction-close"]').getBoundingClientRect(); return { x: r.x, y: r.y, w: r.width, h: r.height }; });
  await page.mouse.click(box.x + box.w / 2, box.y + box.h / 2);
  await page.waitForTimeout(12_000 * Math.max(1, rate / 20));
  const out = await page.evaluate(() => ({ ...window.__probe, gone: document.querySelector('[data-slot="introduction-info-box"]') === null }));
  console.log(JSON.stringify({ trial, rate, clickAt: box, rects: out.rects.slice(0, 12), moves: out.rects.length, events: out.events, gone: out.gone }));
  await context.close();
}
await browser.close();
