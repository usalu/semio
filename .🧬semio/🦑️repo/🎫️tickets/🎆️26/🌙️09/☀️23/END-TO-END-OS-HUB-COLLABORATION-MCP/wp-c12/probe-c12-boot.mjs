/** 🏠️ C12: one serve boots to Home (preamble rule 20 compile-atomic proof): loads the shell, waits for Home's create-space
 * button, reports page errors. usage: bun probe-c12-boot.mjs <url> */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
const url = process.argv[2];
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const errors = [];
try {
  const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
  page.on("pageerror", (error) => errors.push(String(error).slice(0, 300)));
  const started = Date.now();
  await page.goto(url, { waitUntil: "domcontentloaded", timeout: 180_000 });
  await page.locator('[data-ui-node-key="s-home-create-space"]').first().waitFor({ state: "attached", timeout: 240_000 });
  console.log(`BOOT HOME ok in ${Date.now() - started} ms; page errors ${errors.length} ${JSON.stringify(errors)}`);
} catch (error) {
  console.log(`BOOT FAIL ${String(error).split("\n")[0]}; page errors ${JSON.stringify(errors)}`);
} finally {
  await browser.close();
}
