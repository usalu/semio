import { chromium } from "@playwright/test";

const baseURL = process.env.PLAYWRIGHT_BASE_URL ?? "http://127.0.0.1:4181";
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
page.on("console", (msg) => console.log(`[console.${msg.type()}]`, msg.text()));
page.on("pageerror", (err) => console.log("[pageerror]", err.message));
await page.goto(baseURL, { waitUntil: "networkidle", timeout: 180_000 });
const toggle = page.locator("#ui\\.panelToggle\\.workbench");
if (await toggle.isVisible()) await toggle.click();
await page.waitForTimeout(500);
console.log("[body text]", (await page.locator("body").innerText()).slice(0, 3000));
console.log("[has metabolism btn]", await page.getByText("Open metabolism fixture").isVisible());
console.log("[html length]", (await page.content()).length);
await browser.close();
