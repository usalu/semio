import {createRequire} from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const {chromium} = require("playwright");
const variants = {default: []};
for (const [name, args] of Object.entries(variants)) {
  const browser = await chromium.launch({headless: true, args});
  const page = await browser.newPage({viewport: {width: 1600, height: 900}});
  await page.goto("data:text/html,<html><body>x</body></html>");
  const state = await page.evaluate(() => [document.visibilityState, document.hasFocus()]);
  const outcome = await Promise.race([page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => resolve("fired")))), new Promise(resolve => setTimeout(() => resolve("TIMEOUT"), 6000))]);
  console.log("[DEBUG]", name, state, outcome);
  await browser.close().catch(() => {});
}
