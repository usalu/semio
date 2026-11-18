/** 🐢️ Throttled first-visit repro: Skip click and identity step without retries. Env: RATE (CPU slowdown), TRIALS, SITE, MODE (stable|raw). */
import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/.cache/b/d/m/j/b87aa6c238609ac86876/node_modules/@playwright/test/");
const { chromium } = require("@playwright/test");
const rate = Number(process.env.RATE ?? 30), trials = Number(process.env.TRIALS ?? 5), site = process.env.SITE ?? "http://127.0.0.1:6068", mode = process.env.MODE ?? "stable";
const browser = await chromium.launch();
const intro = (page) => page.locator('[data-slot="introduction-info-box"]');
const identity = (page) => page.getByRole("dialog").filter({ has: page.locator('input[type="radio"][value="anonymous"]') });
const results = [];
for (let trial = 0; trial < trials; trial++) {
  const context = await browser.newContext({ viewport: { width: 1280, height: 800 }, locale: "en" });
  const page = await context.newPage();
  const cdp = await context.newCDPSession(page);
  await cdp.send("Emulation.setCPUThrottlingRate", { rate });
  await cdp.send("Network.enable");
  await cdp.send("Network.emulateNetworkConditions", { offline: false, latency: 150, downloadThroughput: (1.6 * 1024 * 1024) / 8, uploadThroughput: (750 * 1024) / 8 });
  const record = { trial, rate, mode };
  try {
    await page.goto(site + "/", { waitUntil: "commit" });
    await intro(page).waitFor({ state: "visible", timeout: 240_000 });
    const t0 = Date.now();
    const close = intro(page).locator('[data-slot="introduction-close"]');
    if (mode === "raw") {
      const box = await close.boundingBox();
      await page.mouse.click(box.x + box.width / 2, box.y + box.height / 2);
    } else await close.click({ timeout: 60_000 });
    record.skipClickMs = Date.now() - t0;
    await intro(page).waitFor({ state: "detached", timeout: 30_000 }).then(() => (record.skip = "ok"), () => (record.skip = "LOST"));
    if (record.skip === "ok") {
      const learner = page.locator('[data-course-grid][data-grid-active] > [data-layered-overview] [data-layered-card="learner"] [data-overview-card-action="primary"]');
      await learner.click({ timeout: 120_000 });
      await identity(page).waitFor({ state: "visible", timeout: 30_000 }).then(() => (record.dialog = "ok"), () => (record.dialog = "MISSING"));
      if (record.dialog === "ok") {
        const form = identity(page);
        const kind = form.locator('input[type="radio"][value="name"]');
        await kind.click({ timeout: 60_000 });
        record.radio = await kind.isChecked();
        const text = form.locator('input[type="text"]');
        await text.waitFor({ timeout: 30_000 });
        await text.fill("Load Tester", { timeout: 60_000 });
        await page.waitForTimeout(3_000);
        record.value = await text.inputValue().catch(() => "GONE");
        record.radioAfter = await kind.isChecked().catch(() => "GONE");
      }
    }
  } catch (error) {
    record.error = String(error).split("\n")[0];
  }
  console.log(JSON.stringify(record));
  results.push(record);
  await context.close();
}
await browser.close();
