import { chromium } from "playwright";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const out = join(dirname(fileURLToPath(import.meta.url)), "..", "🗑️generated", "ub");
const browser = await chromium.launch(process.env.UB_CHROME ? { executablePath: process.env.UB_CHROME } : {});
const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, locale: "en-US" });
page.on("console", (message) => console.log("console", message.type(), message.text().slice(0, 200)));
page.on("pageerror", (error) => console.log("pageerror", error.message.slice(0, 300)));
await page.goto(process.argv[2]);
await page.waitForTimeout(Number(process.argv[3] ?? 15000));
console.log((await page.evaluate(() => document.body.innerText)).slice(0, 600));
console.log(await page.evaluate(() => !!document.querySelector("[data-viewer]")));
await page.screenshot({ path: join(out, "probe.png") });
await browser.close();
