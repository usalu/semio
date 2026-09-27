/** 🌅️ C13 (preamble rule 20): one `serve s react dev` boot to Home — pageerror-free — after a host TS edit.
 * usage: bun probe-c13-boot.mjs <url> */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
const url = process.argv[2] ?? "http://127.0.0.1:6670/";
const browser = await chromium.launch({ headless: true });
const page = await (await browser.newContext({ locale: "en-US" })).newPage();
const errors = [];
page.on("pageerror", (error) => errors.push(String(error).slice(0, 400)));
page.on("console", (message) => { if (message.type() === "error") errors.push(`console: ${message.text().slice(0, 300)}`); });
const started = Date.now();
await page.goto(url, { waitUntil: "domcontentloaded", timeout: 180_000 });
const home = await page.locator('[data-ui-node-key="s-home-create-space"]').first().waitFor({ state: "attached", timeout: 180_000 }).then(() => true, () => false);
await page.waitForTimeout(3_000);
console.log(JSON.stringify({ url, home, ms: Date.now() - started, pageErrors: errors.filter((line) => !line.startsWith("console:")), consoleErrors: errors.filter((line) => line.startsWith("console:")).slice(0, 8) }, null, 1));
await browser.close();
