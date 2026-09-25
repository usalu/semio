import { chromium } from "/home/user/semio/node_modules/playwright/index.mjs";
const args = process.argv.slice(2);
const browser = await chromium.launch({ executablePath: "/opt/pw-browsers/chromium-1194/chrome-linux/chrome", args });
const page = await browser.newPage();
await page.goto("http://127.0.0.1:5174/favicon.ico");
const info = await page.evaluate(async () => { if (!navigator.gpu) return "no navigator.gpu"; const a = await navigator.gpu.requestAdapter(); return a ? `adapter ok` : "no adapter"; });
console.log(args.join(" "), "=>", info);
await browser.close();
