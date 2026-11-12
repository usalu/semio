import {createRequire} from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const {chromium} = require("playwright");
const browser = await chromium.launch({headless: true, args: ["--enable-logging=stderr", "--v=0"]});
const page = await browser.newPage({viewport: {width: 800, height: 600}});
await page.goto("data:text/html,<html><body>x</body></html>");
const outcome = await Promise.race([page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => resolve("fired")))), new Promise(resolve => setTimeout(() => resolve("TIMEOUT"), 5000))]);
console.log("[DEBUG]", outcome);
await browser.close().catch(() => {});
