/** 🔬️ Samples the introduction box of the dev page at a phone size and prints the distinct geometries it takes: more than one means it never settles. Usage: node intro-probe.mjs [width] [height] */
import { createRequire } from "node:module";
const require = createRequire("C:/semio/.cache/b/d/m/j/c642535c86e40717744c/node_modules/");
const { chromium } = require("playwright");
const [width = "375", height = "560"] = process.argv.slice(2);
const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 375, height: 812 }, isMobile: true, hasTouch: true });
await page.goto("http://127.0.0.1:6061/");
await page.waitForSelector('[data-slot="introduction-info-box"]', { timeout: 120000 });
await page.setViewportSize({ width: Number(width), height: Number(height) });
await page.waitForTimeout(3000);
const seen = await page.evaluate(async () => {
  const states = new Map();
  for (let i = 0; i < 120; i++) {
    const box = document.querySelector('[data-slot="introduction-info-box"]');
    const skip = box?.querySelector('[data-slot="introduction-close"]');
    const r = (e) => { const b = e.getBoundingClientRect(); return [b.x, b.y, b.width, b.height].map((v) => Math.round(v * 10) / 10).join(","); };
    const key = box ? `box ${r(box)} style(${box.style.top},${box.style.left},${box.style.width}) skip ${skip ? r(skip) : "-"}` : "none";
    states.set(key, (states.get(key) ?? 0) + 1);
    await new Promise((done) => requestAnimationFrame(done));
  }
  return [...states];
});
console.log(JSON.stringify(seen, null, 1));
await browser.close();
