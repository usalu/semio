#!/usr/bin/env bun
import { chromium } from "playwright";
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
page.setDefaultTimeout(90_000);
const errors = [];
page.on("pageerror", (e) => errors.push(String(e)));
await page.goto("http://127.0.0.1:6020/", { waitUntil: "domcontentloaded" });
await page.waitForFunction(() => document.querySelectorAll("#root *").length > 20);
const deadline = Date.now() + 60_000;
let report;
while (Date.now() < deadline) {
  report = await page.evaluate(() => ({
    loading: /Loading plugins/i.test(document.body.innerText),
    unreachable: /unreachable/i.test(document.body.innerText),
    canvases: document.querySelectorAll("canvas").length,
    title: document.title,
  }));
  console.log(`[DEBUG] ${JSON.stringify(report)}`);
  if (!report.loading && report.canvases >= 4 && !report.unreachable) break;
  await page.waitForTimeout(2000);
}
console.log(`[DEBUG] errors=${JSON.stringify(errors)}`);
if (report.canvases < 4 || report.unreachable || report.loading || errors.length) {
  console.error("[DEBUG] FAIL");
  process.exit(1);
}
console.log("[DEBUG] PASS");
await browser.close();
