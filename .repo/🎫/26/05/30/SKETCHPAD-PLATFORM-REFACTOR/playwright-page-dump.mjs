import { chromium } from "@playwright/test";

const baseURL = process.env.PLAYWRIGHT_BASE_URL ?? "http://127.0.0.1:4181";
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
page.on("console", (msg) => console.log(`[console.${msg.type()}]`, msg.text()));
page.on("pageerror", (err) => console.log("[pageerror]", err.message));
await page.goto(baseURL, { waitUntil: "networkidle", timeout: 180_000 });
console.log("[body text]", (await page.locator("body").innerText()).slice(0, 2000));
console.log("[html length]", (await page.content()).length);
await browser.close();
