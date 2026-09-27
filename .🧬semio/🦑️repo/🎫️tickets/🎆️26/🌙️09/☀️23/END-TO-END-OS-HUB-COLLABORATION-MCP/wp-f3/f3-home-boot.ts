/** 🏠️ F3 — one page load of a serve: beacon, Home mounted, page errors. usage: bun f3-home-boot.ts <baseUrl> */
import { chromium } from "playwright";
import { awaitBeacon } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts";
const browser = await chromium.launch({ headless: true });
try {
  const page = await browser.newPage();
  const errors: string[] = [];
  page.on("pageerror", (error) => errors.push(error.message.slice(0, 200)));
  await page.goto(process.argv[2]!, { waitUntil: "commit", timeout: 300_000 });
  const beacon = await awaitBeacon(page, Date.now() + 300_000);
  const windows = await page.evaluate(() => document.querySelectorAll("[data-window-id]").length);
  console.log(JSON.stringify({ beacon, windows, errors }));
} finally {
  await browser.close();
}
