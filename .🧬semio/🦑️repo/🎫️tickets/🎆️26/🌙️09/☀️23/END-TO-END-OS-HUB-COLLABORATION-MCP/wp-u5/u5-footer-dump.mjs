import { chromium } from "playwright";
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
await page.goto(process.argv[2], { waitUntil: "commit" });
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady !== undefined, undefined, { timeout: 300_000 });
await page.waitForTimeout(3000);
console.log(JSON.stringify(await page.evaluate(() => [...document.querySelectorAll('[id="ui.footer"] button')].map((b) => ({ id: b.id, text: b.textContent, label: b.getAttribute("aria-label"), slot: b.getAttribute("data-slot"), pressed: b.getAttribute("aria-pressed") }))), null, 1));
await browser.close();
