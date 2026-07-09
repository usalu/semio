import { chromium } from "playwright";

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1600, height: 900 } });
const errors: string[] = [];
page.on("console", (m) => {
  if (m.type() === "error") errors.push(m.text());
});
page.on("pageerror", (e) => errors.push(e.message));
await page.goto("http://127.0.0.1:6027/", { waitUntil: "load", timeout: 120_000 });
await page.waitForTimeout(8000);
const text = await page.locator("body").innerText();
const html = await page.content();
console.log("[DEBUG] errors", errors.slice(0, 30));
console.log("[DEBUG] text", text.slice(0, 800));
console.log("[DEBUG] hasShelf", html.includes("board-play-fixture-shelf"));
console.log("[DEBUG] hasHandlesBtn", (await page.locator('button[title^="Redraw handles"]').count()) > 0);
await browser.close();
