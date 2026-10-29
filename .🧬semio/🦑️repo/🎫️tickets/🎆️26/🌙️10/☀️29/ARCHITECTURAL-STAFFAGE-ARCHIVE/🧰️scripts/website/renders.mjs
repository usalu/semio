import { chromium } from "playwright";
const browser = await chromium.launch();
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 }, locale: "en-US" })).newPage();
let n = 0;
page.on("console", (m) => m.type() === "error" && /Maximum update/.test(m.text()) && n++);
await page.goto("http://localhost:6071/" + process.argv[2], { waitUntil: "domcontentloaded" });
await page.waitForTimeout(8000);
console.log("renders", JSON.stringify(await page.evaluate(() => window.__c)), "errors", n);
await browser.close();
