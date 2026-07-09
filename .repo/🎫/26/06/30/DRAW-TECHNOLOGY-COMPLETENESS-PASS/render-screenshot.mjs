import { chromium } from "playwright";
import { writeFileSync } from "node:fs";

const url = "http://127.0.0.1:6064/";
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
try {
  await page.goto(url, { waitUntil: "networkidle", timeout: 90_000 });
  const fixtureSelect = page.locator("#playground\\.navbar\\.fixture");
  if (await fixtureSelect.count()) {
    await fixtureSelect.selectOption("semio");
    await page.waitForTimeout(6000);
  }
  const shot = await page.locator(".bg-neutral-950").first().screenshot();
  writeFileSync(new URL("./render-screenshot.png", import.meta.url), shot);
  console.log("saved render-screenshot.png");
} finally {
  await browser.close();
}
