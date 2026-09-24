/** 🔎️ C10 probe: dumps the hub sign-in workspace's buttons (both locales). */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
for (const locale of ["en-US", "de-DE"]) {
  const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 }, locale })).newPage();
  await page.goto(process.argv[2], { waitUntil: "domcontentloaded", timeout: 120000 });
  await page.locator('[data-semio-hub-sign-in=""]').first().waitFor({ state: "attached", timeout: 120000 });
  await page.locator('[data-semio-hub-sign-in=""]').first().click();
  await page.locator("[data-semio-hub-workspace]").waitFor({ state: "visible", timeout: 30000 });
  console.log(locale, JSON.stringify(await page.locator("[data-semio-hub-workspace] button").evaluateAll((els) => els.map((el) => `${el.type}|${el.id}|${el.getAttribute("aria-label")}|${(el.textContent ?? "").trim().slice(0, 30)}|visible=${el.offsetParent !== null}`))));
}
await browser.close();
