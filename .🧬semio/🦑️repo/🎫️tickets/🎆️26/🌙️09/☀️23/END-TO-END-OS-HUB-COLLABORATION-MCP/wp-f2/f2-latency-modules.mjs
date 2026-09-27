#!/usr/bin/env bun
/** 🔬️ F2 — which served script URLs carry the wasm canvas session classes (exported classes with `renderFrame`) after the
 * latency gate's scenarios opened their programs; answers why the gate's paint hook found none. usage: bun f2-latency-modules.mjs <baseUrl> */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
const [baseUrl] = process.argv.slice(2);
const generated = fileURLToPath(new URL("./generated/", import.meta.url));
const sweep = await import("/Users/ueli/Documents/semio/.tmp-ticket-0918/🐍️s6-all-kinds-sweep.mjs");
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
page.setDefaultNavigationTimeout(300_000);
await page.goto(baseUrl, { waitUntil: "commit" });
await sweep.awaitBeacon(page, Date.now() + 300_000);
await sweep.dismissIntroduction(page);
await page.keyboard.press("Escape").catch(() => undefined);
for (const [pluginId, appId] of [["writer", "s.writer.writer@1/*#editor"], ["draw", "s.draw.drawing@1/*#editor"]]) {
  await page.evaluate(() => { if (document.activeElement instanceof HTMLElement) document.activeElement.blur(); });
  await page.keyboard.press("Meta+p");
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await input.waitFor({ state: "visible", timeout: 15_000 }).catch(() => undefined);
  await input.fill(/^s\.[^.]+\.([^@]+)@/u.exec(appId)[1]);
  await page.waitForTimeout(1_500);
  for (const id of [`spawn.${pluginId}.${appId}`, `spawn.${pluginId}`]) {
    const item = page.locator(`[data-slot="command-item"][data-command-item-id="${id}"]`).first();
    if ((await item.count()) > 0) { await item.click({ force: true }); break; }
  }
  await page.waitForTimeout(12_000);
}
const report = await page.evaluate(async () => {
  const urls = [...new Set(performance.getEntriesByType("resource").map((entry) => entry.name).filter((name) => /\.js(\?|$)/u.test(name)))];
  const rows = [];
  for (const url of urls) {
    const decoded = decodeURIComponent(url);
    if (!/rs|wasm|pkg|bindings|editor|surface|canvas/iu.test(decoded)) continue;
    let module;
    try { module = await import(url); } catch (error) { rows.push({ url: decoded.slice(0, 200), error: String(error).slice(0, 120) }); continue; }
    const classes = Object.entries(module).filter(([, value]) => typeof value === "function" && typeof value.prototype?.renderFrame === "function").map(([name]) => name);
    if (classes.length > 0) rows.push({ url: decoded.slice(0, 200), classes });
  }
  const wasm = performance.getEntriesByType("resource").map((entry) => decodeURIComponent(entry.name)).filter((name) => /\.wasm(\?|$)/u.test(name)).map((name) => name.slice(0, 200));
  return { rows, wasm, canvases: document.querySelectorAll("canvas").length, workers: performance.getEntriesByType("resource").filter((entry) => entry.initiatorType === "other" && /worker/iu.test(entry.name)).map((entry) => decodeURIComponent(entry.name).slice(0, 160)) };
});
writeFileSync(`${generated}f2-latency-modules.json`, JSON.stringify(report, null, 1));
console.log(JSON.stringify(report, null, 1).slice(0, 4000));
await browser.close();
