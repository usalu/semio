#!/usr/bin/env bun
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { join } from "node:path";

const baseUrl = "http://127.0.0.1:6020/";
const outDir = import.meta.dir;
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
page.setDefaultTimeout(120_000);
const pageErrors = [];
const consoleErrors = [];
const failed = [];
page.on("pageerror", (e) => pageErrors.push(String(e)));
page.on("console", (m) => { if (m.type() === "error") consoleErrors.push(m.text()); });
page.on("response", (r) => { if (r.status() >= 400) failed.push({ url: r.url(), status: r.status() }); });

console.log(`[DEBUG] navigating to ${baseUrl}`);
await page.goto(baseUrl, { waitUntil: "domcontentloaded", timeout: 120_000 });
await page.waitForFunction(() => document.querySelectorAll("#root *").length > 20, { timeout: 120_000 });

const deadline = Date.now() + 90_000;
let report = { loading: true, unreachable: false, rootKids: 0, canvases: 0, title: "", body: "" };
while (Date.now() < deadline) {
  report = await page.evaluate(() => ({
    loading: /Loading plugins/i.test(document.body.innerText),
    unreachable: /unreachable/i.test(document.body.innerText),
    rootKids: document.querySelectorAll("#root *").length,
    canvases: document.querySelectorAll("canvas").length,
    title: document.title,
    body: document.body.innerText.slice(0, 800),
  }));
  console.log(`[DEBUG] loading=${report.loading} unreachable=${report.unreachable} root=${report.rootKids} canvas=${report.canvases}`);
  if (!report.loading && report.canvases >= 4 && !report.unreachable) break;
  await page.waitForTimeout(2000);
}

await page.screenshot({ path: join(outDir, "smoke-final.png"), fullPage: true });
writeFileSync(join(outDir, "smoke-final.json"), JSON.stringify({ report, pageErrors, consoleErrors, failed }, null, 2));
console.log(`[DEBUG] title=${report.title}`);
console.log(`[DEBUG] body=${JSON.stringify(report.body)}`);
console.log(`[DEBUG] pageErrors=${JSON.stringify(pageErrors)}`);
console.log(`[DEBUG] failed=${JSON.stringify(failed)}`);
console.log(`[DEBUG] consoleErrors=${JSON.stringify(consoleErrors.slice(0, 20))}`);

const ignorable = (m) => /WebGL|WebGPU|GPU stall|ReadPixels/i.test(m);
const critical = pageErrors.filter((m) => !ignorable(m));
const criticalFailed = failed.filter((f) => !/favicon/i.test(f.url));
if (critical.length || report.unreachable || report.canvases < 4 || report.loading || criticalFailed.length) {
  console.error("[DEBUG] CAD smoke FAIL");
  await browser.close();
  process.exit(1);
}
console.log("[DEBUG] CAD smoke PASS");
await browser.close();
