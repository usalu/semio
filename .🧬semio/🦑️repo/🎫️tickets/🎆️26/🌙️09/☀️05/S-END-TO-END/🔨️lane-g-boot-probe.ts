#!/usr/bin/env bun
/** @emoji 🩺️ Lane G served-boot probe: loads the React `s` shell, waits for the readiness beacon
 * (`document.documentElement.dataset.semioOsReady === "s"`) and prints every console line, so the
 * shard-liveness changes can be judged on real runtime evidence instead of unit tests alone. */
const port = process.argv[2] ?? "6076";
const waitMs = Number(process.argv[3] ?? "180000");
const { chromium } = await import("playwright");
const browser = await chromium.launch({ args: ["--enable-features=WebAssemblyJavaScriptPromiseIntegration"] });
const page = await browser.newPage();
page.on("console", (message) => console.log(`[console:${message.type()}] ${message.text()}`));
page.on("pageerror", (error) => console.log(`[pageerror] ${error.message}`));
page.on("requestfailed", (request) => console.log(`[requestfailed] ${request.url()} ${request.failure()?.errorText ?? ""}`));
await page.goto(`http://127.0.0.1:${port}/`, { waitUntil: "domcontentloaded", timeout: 120_000 });
let ready = false;
try {
  await page.waitForFunction(() => document.documentElement.dataset.semioOsReady === "s", undefined, { timeout: waitMs, polling: 1000 });
  ready = true;
} catch {}
const beacon = await page.evaluate(() => document.documentElement.dataset.semioOsReady ?? null);
const bodyText = await page.evaluate(() => document.body.innerText.slice(0, 400));
console.log(`[probe] readyBeacon=${JSON.stringify(beacon)} reached=${ready}`);
console.log(`[probe] bodyText=${JSON.stringify(bodyText)}`);
await browser.close();
process.exit(ready ? 0 : 1);
