import fs from "node:fs";
import path from "node:path";
import { chromium } from "@playwright/test";

const generated = path.resolve(import.meta.dirname, "🗑️generated");
fs.mkdirSync(generated, { recursive: true });
const browser = await chromium.launch({ headless: true, channel: "msedge" });
try {
  const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, locale: "en" });
  const errors = [];
  page.on("pageerror", error => errors.push(String(error)));
  await page.goto("http://127.0.0.1:6067/", { waitUntil: "networkidle" });
  await page.waitForTimeout(5000);
  console.log("[DEBUG] " + (await page.locator("body").innerText()).slice(0,9000));
  console.log("[DEBUG] errors", errors);
  await page.screenshot({ path: path.join(generated, "browser.png"), fullPage: true });
} finally { await browser.close(); }
