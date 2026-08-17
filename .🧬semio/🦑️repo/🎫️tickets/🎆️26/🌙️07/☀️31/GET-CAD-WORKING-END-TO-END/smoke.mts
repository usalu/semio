#!/usr/bin/env bun
/** Temporary CAD react playground smoke against a running Vite on :6020. */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { join } from "node:path";

const baseUrl = process.env.CAD_URL ?? "http://127.0.0.1:6020/";
const outDir = import.meta.dir;

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
const pageErrors: string[] = [];
const consoleErrors: string[] = [];
page.on("pageerror", (err) => pageErrors.push(String(err)));
page.on("console", (msg) => {
  if (msg.type() === "error") consoleErrors.push(msg.text());
});

console.log(`[DEBUG] navigating to ${baseUrl}`);
await page.goto(baseUrl, { waitUntil: "domcontentloaded", timeout: 120_000 });

try {
  await page.waitForFunction(() => document.querySelectorAll("#root *").length > 20, { timeout: 90_000 });
} catch (e) {
  const shot = join(outDir, "smoke-fail.png");
  await page.screenshot({ path: shot, fullPage: true });
  const html = await page.content();
  writeFileSync(join(outDir, "smoke-fail.html"), html);
  console.error(`[DEBUG] BOOT-TIMEOUT: react #root never populated — ${e}`);
  console.error(`[DEBUG] pageErrors=${JSON.stringify(pageErrors)}`);
  console.error(`[DEBUG] consoleErrors=${JSON.stringify(consoleErrors)}`);
  await browser.close();
  process.exit(1);
}

const report = await page.evaluate(() => {
  const rootKids = document.querySelectorAll("#root *").length;
  const uiPaths = document.querySelectorAll("[data-ui-path]").length;
  const canvases = document.querySelectorAll("canvas").length;
  const bodyText = document.body.innerText.slice(0, 2000);
  const title = document.title;
  return { rootKids, uiPaths, canvases, bodyText, title };
});

await page.screenshot({ path: join(outDir, "smoke.png"), fullPage: true });
writeFileSync(join(outDir, "smoke-report.json"), JSON.stringify({ report, pageErrors, consoleErrors }, null, 2));

console.log(`[DEBUG] title=${report.title}`);
console.log(`[DEBUG] rootKids=${report.rootKids} uiPaths=${report.uiPaths} canvases=${report.canvases}`);
console.log(`[DEBUG] bodyPreview=${JSON.stringify(report.bodyText.slice(0, 400))}`);
console.log(`[DEBUG] pageErrors=${JSON.stringify(pageErrors)}`);
console.log(`[DEBUG] consoleErrors=${JSON.stringify(consoleErrors)}`);

const ignorable = (m: string) => /WebGL|WebGPU|NoCompatibleDevice|THREE\.WebGLRenderer/i.test(m);
const critical = pageErrors.filter((m) => !ignorable(m));
if (critical.length > 0) {
  console.error(`[DEBUG] FAIL critical page errors`);
  await browser.close();
  process.exit(1);
}
if (report.rootKids <= 20) {
  console.error(`[DEBUG] FAIL sparse root`);
  await browser.close();
  process.exit(1);
}
if (report.uiPaths === 0) {
  console.error(`[DEBUG] FAIL no data-ui-path nodes`);
  await browser.close();
  process.exit(1);
}

console.log(`[DEBUG] CAD smoke PASS`);
await browser.close();
