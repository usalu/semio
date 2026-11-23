import { chromium } from "playwright";
const [id, expr] = process.argv.slice(2);
const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 1280, height: 720 } });
page.on("pageerror", e => console.log("[DEBUG] pageerror", e.message));
await page.goto(`http://127.0.0.1:6090/iframe.html?id=${id}&viewMode=story`, { waitUntil: "load" });
await page.waitForSelector("#storybook-root > *", { timeout: 60000 });
console.log(JSON.stringify(await page.evaluate(expr), null, 1));
await browser.close();
