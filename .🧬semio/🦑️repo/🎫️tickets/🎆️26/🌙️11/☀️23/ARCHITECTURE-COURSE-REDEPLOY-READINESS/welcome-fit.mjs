/** 📏️ Measures on a served site whether the welcome box of the introduction fits the window, at several window sizes, and says its language and its text. Usage: node welcome-fit.mjs [origin] */
import { createRequire } from "node:module";
const require = createRequire("C:/semio/.cache/b/d/m/j/c642535c86e40717744c/node_modules/");
const { chromium } = require("playwright");
const origin = process.argv[2] ?? "http://127.0.0.1:6062";
const browser = await chromium.launch();
for (const [width, height] of [[1280, 800], [800, 600], [375, 667]]) {
  const page = await browser.newPage({ viewport: { width, height }, locale: "en-US" });
  await page.goto(origin + "/");
  const box = page.locator('[data-slot="introduction-info-box"]');
  await box.waitFor({ timeout: 120000 });
  await page.waitForTimeout(1500);
  console.log(JSON.stringify(await page.evaluate(() => { const box = document.querySelector('[data-slot="introduction-info-box"]'); const r = box.getBoundingClientRect(); const body = box.querySelector('[data-slot="introduction-body"]'); return { window: [innerWidth, innerHeight], lang: document.documentElement.lang, title: box.querySelector('[data-slot="introduction-info-box-title"]').textContent, box: [r.x, r.y, r.width, r.height].map(Math.round), inside: r.top >= 0 && r.left >= 0 && r.right <= innerWidth && r.bottom <= innerHeight, paragraphs: body.querySelectorAll("p").length, characters: body.textContent.length, pageScrolls: document.documentElement.scrollHeight > innerHeight }; })));
  await page.close();
}
await browser.close();
