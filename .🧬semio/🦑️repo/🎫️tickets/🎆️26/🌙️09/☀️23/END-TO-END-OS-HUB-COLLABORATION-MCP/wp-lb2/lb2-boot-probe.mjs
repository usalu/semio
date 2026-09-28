/** 🥾️ LB2 boot probe (rule 20, copied from S18): one headless boot of a `s` React serve to Home; prints pageerrors + error lines.
 * usage: bun lb2-boot-probe.mjs <url> [locale] */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
const [url = "http://127.0.0.1:6630/", locale = "en-US"] = process.argv.slice(2);
const started = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--ignore-gpu-blocklist"] });
const lines = [];
try {
  const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 }, locale })).newPage();
  page.on("pageerror", (error) => lines.push(`pageerror ${String(error).slice(0, 600)}`));
  page.on("console", (message) => { if (message.type() === "error") lines.push(`console.error ${message.text().slice(0, 400)}`); });
  await page.goto(url, { waitUntil: "domcontentloaded", timeout: 240_000 });
  await page.locator('[data-ui-node-key="s-home-create-space"]').first().waitFor({ state: "attached", timeout: 240_000 });
  await page.waitForTimeout(4_000);
  const title = await page.title();
  console.log(`HOME ${Date.now() - started} ms title=${JSON.stringify(title)} lang=${await page.evaluate(() => document.documentElement.lang)}`);
} catch (error) {
  console.log(`FAIL ${Date.now() - started} ms ${String(error).slice(0, 400)}`);
} finally {
  console.log(`PAGEERRORS ${lines.filter((l) => l.startsWith("pageerror")).length}`);
  for (const line of lines.slice(-20)) console.log(line);
  await browser.close();
}
