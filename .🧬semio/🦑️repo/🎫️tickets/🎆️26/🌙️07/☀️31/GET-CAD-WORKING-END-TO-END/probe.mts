#!/usr/bin/env bun
/** Deeper CAD boot probe: wait past Loading plugins, capture console/network. */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { join } from "node:path";

const baseUrl = process.env.CAD_URL ?? "http://127.0.0.1:6020/";
const outDir = import.meta.dir;

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
const pageErrors: string[] = [];
const consoleLogs: string[] = [];
const failedRequests: { url: string; status?: number; failure?: string }[] = [];

page.on("pageerror", (err) => pageErrors.push(String(err)));
page.on("console", (msg) => consoleLogs.push(`[${msg.type()}] ${msg.text()}`));
page.on("requestfailed", (req) => failedRequests.push({ url: req.url(), failure: req.failure()?.errorText }));
page.on("response", (res) => {
  if (res.status() >= 400) failedRequests.push({ url: res.url(), status: res.status() });
});

console.log(`[DEBUG] navigating to ${baseUrl}`);
await page.goto(baseUrl, { waitUntil: "domcontentloaded", timeout: 120_000 });
await page.waitForFunction(() => document.querySelectorAll("#root *").length > 20, { timeout: 90_000 });

// Wait up to 60s for Loading plugins to clear OR for canvases / ui-path
const deadline = Date.now() + 60_000;
let last = "";
while (Date.now() < deadline) {
  const state = await page.evaluate(() => {
    const text = document.body.innerText;
    return {
      text: text.slice(0, 1500),
      loading: /Loading plugins/i.test(text),
      rootKids: document.querySelectorAll("#root *").length,
      uiPaths: document.querySelectorAll("[data-ui-path]").length,
      canvases: document.querySelectorAll("canvas").length,
      wasmHints: [...document.querySelectorAll("script")].map((s) => s.src).filter((s) => /wasm|plugin|cad/i.test(s)).slice(0, 20),
    };
  });
  last = JSON.stringify(state);
  console.log(`[DEBUG] tick loading=${state.loading} root=${state.rootKids} ui=${state.uiPaths} canvas=${state.canvases}`);
  if (!state.loading && (state.uiPaths > 0 || state.canvases > 0)) break;
  if (!state.loading && state.rootKids > 200) break;
  await page.waitForTimeout(2000);
}

await page.screenshot({ path: join(outDir, "probe.png"), fullPage: true });
const html = await page.content();
writeFileSync(join(outDir, "probe.html"), html);
writeFileSync(
  join(outDir, "probe-report.json"),
  JSON.stringify({ last: JSON.parse(last || "{}"), pageErrors, consoleLogs, failedRequests }, null, 2),
);

console.log(`[DEBUG] pageErrors=${JSON.stringify(pageErrors)}`);
console.log(`[DEBUG] failedRequests=${JSON.stringify(failedRequests.slice(0, 40), null, 2)}`);
console.log(`[DEBUG] consoleLogs (last 80):\n${consoleLogs.slice(-80).join("\n")}`);
await browser.close();
