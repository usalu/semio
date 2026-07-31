#!/usr/bin/env node
import { chromium } from "playwright";

const baseUrl = process.env.S_STUDIO_URL ?? "http://127.0.0.1:6068/";

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
const errors = [];
page.on("pageerror", (e) => errors.push(String(e)));
page.on("console", (m) => {
  if (m.type() === "error") errors.push(m.text());
});

await page.goto(baseUrl, { waitUntil: "networkidle", timeout: 120000 });

for (let i = 0; i < 60; i++) {
  const n = await page.locator("#root *").count();
  const text = await page.locator("body").innerText();
  console.log(`t=${i}s children=${n} text=${JSON.stringify(text.slice(0, 120))}`);
  if (i > 5 && n === 0 && errors.length) break;
  await page.waitForTimeout(1000);
}

console.log("errors:", errors.slice(0, 10));
await browser.close();
