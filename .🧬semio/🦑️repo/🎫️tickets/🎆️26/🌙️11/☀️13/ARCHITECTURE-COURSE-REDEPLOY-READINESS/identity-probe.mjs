/** 🔬️ Raw first input on the identity step under load: radio click at its position, then keystrokes at once, no actionability waits. Env: RATE, TRIALS, SITE. */
import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/.cache/b/d/m/j/b87aa6c238609ac86876/node_modules/@playwright/test/");
const { chromium } = require("@playwright/test");
const rate = Number(process.env.RATE ?? 30), trials = Number(process.env.TRIALS ?? 3), site = process.env.SITE ?? "http://127.0.0.1:6068";
const browser = await chromium.launch();
const dialog = 'div[role="dialog"]:has(input[type="radio"][value="anonymous"])';
for (let trial = 0; trial < trials; trial++) {
  const context = await browser.newContext({ viewport: { width: 1280, height: 800 }, locale: "en" });
  const page = await context.newPage();
  const cdp = await context.newCDPSession(page);
  await cdp.send("Emulation.setCPUThrottlingRate", { rate });
  await cdp.send("Network.enable");
  await cdp.send("Network.emulateNetworkConditions", { offline: false, latency: 150, downloadThroughput: (1.6 * 1024 * 1024) / 8, uploadThroughput: (750 * 1024) / 8 });
  const record = { trial, rate };
  try {
    await page.goto(site + "/", { waitUntil: "commit" });
    await page.locator('[data-slot="introduction-close"]').waitFor({ timeout: 240_000 });
    await page.locator('[data-slot="introduction-close"]').click({ timeout: 240_000 });
    await page.locator('[data-slot="introduction-info-box"]').waitFor({ state: "detached", timeout: 120_000 });
    await page.locator('[data-course-grid][data-grid-active] > [data-layered-overview] [data-layered-card="learner"] [data-overview-card-action="primary"]').click({ timeout: 240_000 });
    await page.locator(dialog).waitFor({ timeout: 120_000 });
    await page.evaluate((selector) => {
      const log = (window.__id = { removed: 0, values: [] });
      document.querySelector(selector).__mark = true;
      new MutationObserver(() => { const d = document.querySelector(selector); if (!d || !d.__mark) { log.removed++; if (d) d.__mark = true; } }).observe(document.body, { childList: true, subtree: true });
    }, dialog);
    const radio = await page.locator(dialog + ' input[type="radio"][value="name"]').boundingBox();
    const t0 = Date.now();
    await page.mouse.click(radio.x + radio.width / 2, radio.y + radio.height / 2);
    await page.keyboard.type("Ada", { delay: 0 });
    record.afterRawMs = Date.now() - t0;
    await page.waitForTimeout(4_000);
    record.radioChecked = await page.locator(dialog + ' input[type="radio"][value="name"]').isChecked();
    record.text = await page.locator(dialog + ' input[type="text"]').inputValue().catch(() => "NO-FIELD");
    record.focus = await page.evaluate(() => document.activeElement?.tagName + ":" + (document.activeElement?.getAttribute("type") ?? ""));
    record.remounts = (await page.evaluate(() => window.__id.removed));
  } catch (error) {
    record.error = String(error).split("\n")[0];
  }
  console.log(JSON.stringify(record));
  await context.close();
}
await browser.close();
