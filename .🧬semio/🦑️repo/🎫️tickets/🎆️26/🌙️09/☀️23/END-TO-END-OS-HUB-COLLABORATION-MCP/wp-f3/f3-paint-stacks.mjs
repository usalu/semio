#!/usr/bin/env bun
/** 🔬️ F3 (copy of wp-f2/f2-paint-stacks.mjs) — who calls `renderFrame` while typing in a text editor: opens the program, types, and aggregates the call stacks of
 * every wasm canvas session `renderFrame` during the typing. usage: bun f2-paint-stacks.mjs <baseUrl> <pluginId> <appId> <windowSuffix> */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
const [baseUrl, pluginId, appId, windowSuffix] = process.argv.slice(2);
const generated = fileURLToPath(new URL("./generated/", import.meta.url));
const sweep = await import("/Users/ueli/Documents/semio/.tmp-ticket-0918/🐍️s6-all-kinds-sweep.mjs");
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist"] });
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
page.setDefaultNavigationTimeout(300_000);
await page.addInitScript(() => performance.setResourceTimingBufferSize(100_000));
await page.goto(baseUrl, { waitUntil: "commit" });
await sweep.awaitBeacon(page, Date.now() + 300_000);
await sweep.dismissIntroduction(page);
await page.keyboard.press("Escape").catch(() => undefined);
await page.keyboard.press("Meta+p");
const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
await input.waitFor({ state: "visible", timeout: 15_000 });
await input.fill(/^s\.[^.]+\.([^@]+)@/u.exec(appId)[1]);
await page.waitForTimeout(1_500);
for (const id of [`spawn.${pluginId}.${appId}`, `spawn.${pluginId}`]) {
  const item = page.locator(`[data-slot="command-item"][data-command-item-id="${id}"]`).first();
  if ((await item.count()) > 0) { await item.click({ force: true }); break; }
}
const target = page.locator(`[id$="${windowSuffix}"] [data-slot="window-body"] .semio-text-editor-host canvas`).first();
await target.waitFor({ state: "visible", timeout: 60_000 });
const box = await target.boundingBox();
await page.mouse.click(box.x + box.width * 0.5, box.y + box.height * 0.4);
await page.waitForTimeout(800);
await page.keyboard.press("End");
const hooked = await page.evaluate(async () => {
  const state = { armed: false, stacks: new Map(), calls: 0, times: [], log: [], lastKey: null };
  addEventListener("keydown", (event) => { if (state.armed) { state.lastKey = `${Math.round(performance.now())} key ${event.key}`; state.log.push(state.lastKey); } }, { capture: true });
  Object.defineProperty(window, "__f2Paint", { value: state });
  const urls = [...new Set(performance.getEntriesByType("resource").map((entry) => entry.name).filter((name) => /\.js(\?|$)/u.test(name) && /\/pkg\/|bindings\//u.test(decodeURIComponent(name))))];
  const names = [];
  for (const url of urls) {
    let module;
    try { module = await import(url); } catch { continue; }
    for (const [name, value] of Object.entries(module)) {
      const prototype = value?.prototype;
      if (typeof value !== "function" || typeof prototype?.renderFrame !== "function" || prototype.__f2Hooked) continue;
      const original = prototype.renderFrame;
      prototype.renderFrame = function (...args) {
        if (state.armed) {
          state.calls += 1;
          state.times.push(Math.round(performance.now()));
          state.log.push(Math.round(performance.now()) + " " + (state.lastKey ?? ""));
          const stack = (new Error().stack ?? "").split("\n").slice(2, 12).map((line) => line.trim().replace(/https?:\/\/[^/]+/u, "").replace(/\?[^:)]*/u, "")).map((line) => decodeURIComponent(line).replace(/^at /u, "").slice(-110)).join(" <- ");
          state.stacks.set(stack, (state.stacks.get(stack) ?? 0) + 1);
        }
        return original.apply(this, args);
      };
      prototype.__f2Hooked = true;
      names.push(name);
    }
  }
  return names;
});
await page.waitForTimeout(1_500);
await page.evaluate(() => { window.__f2Paint.armed = true; });
const keys = 12;
for (let index = 0; index < keys; index++) {
  await page.keyboard.press(index % 2 === 0 ? "a" : "Backspace");
  await page.waitForTimeout(180);
}
await page.waitForTimeout(1_000);
const result = await page.evaluate(() => { const state = window.__f2Paint; state.armed = false; return { calls: state.calls, times: state.times, log: state.log, stacks: [...state.stacks.entries()].sort((a, b) => b[1] - a[1]) }; });
const report = { appId, hooked, keys, log: result.log, perKey: +(result.calls / keys).toFixed(2), gapsMs: result.times.slice(1).map((time, index) => time - result.times[index]), stacks: result.stacks };
writeFileSync(`${generated}f3-paint-stacks-${pluginId}.json`, JSON.stringify(report, null, 1));
console.log(JSON.stringify({ hooked, calls: result.calls, perKey: report.perKey, gaps: report.gapsMs.slice(0, 40) }));
for (const [stack, count] of result.stacks.slice(0, 12)) console.log(count, stack);
await browser.close();
