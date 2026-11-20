/** 🔬️ Probe of the root card grid's cells against the equal thirds of the layered-home spec (port 6070). */
import { createRequire } from "node:module";
const require = createRequire(import.meta.url);
const { chromium } = require("/Users/ueli/Documents/semio/.cache/b/d/m/j/b87aa6c238609ac86876/node_modules/@playwright/test");
const browser = await chromium.launch();
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 }, locale: "en-GB" })).newPage();
await page.goto("http://127.0.0.1:6070/");
await page.locator('[data-slot="introduction-close"]').click();
await page.locator('[data-layered-card="learner"] [data-overview-card-action="primary"]').click();
const dialog = page.getByRole("dialog").filter({ has: page.locator('input[type="radio"][value="anonymous"]') });
await dialog.locator('button[type="submit"], button').last().click().catch(() => {});
await page.mouse.move(1, 1);
await page.waitForTimeout(3000);
const out = await page.evaluate(() => {
  const root = document.querySelector("[data-course-grid][data-grid-active] > [data-layered-overview]");
  const f = root.getBoundingClientRect();
  const cards = [...root.querySelectorAll(":scope > [data-layered-overlay] [data-layered-card]")].map((c, i) => { const r = c.querySelector("[data-overview-card]").getBoundingClientRect(); const cell = c.closest("[data-card-grid-cell]"); return { id: c.getAttribute("data-layered-card"), top: Math.round(r.top - f.top), bottom: Math.round(r.bottom - f.top), h: Math.round(r.height), lift: cell?.style.getPropertyValue("--card-stack-lift") ?? c.style.getPropertyValue("--card-stack-lift"), cellTop: cell ? Math.round(cell.getBoundingClientRect().top - f.top) : null }; });
  return { frame: [f.width, f.height], overlay: getComputedStyle(root.querySelector("[data-layered-overlay]")).gridTemplateRows, cards };
});
console.log(JSON.stringify(out, null, 1));
await browser.close();
