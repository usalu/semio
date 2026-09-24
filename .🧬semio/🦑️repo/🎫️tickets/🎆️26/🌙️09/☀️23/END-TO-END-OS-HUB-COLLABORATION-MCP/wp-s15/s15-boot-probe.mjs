#!/usr/bin/env bun
/** 🔎️ S15 boot probe: boots the shell N times per locale and fails on ANY transient error frame. An init
 * script installs a MutationObserver on <html> before the app runs, so every change of the shell's boot
 * beacons (`data-semio-os-ready`, `data-semio-os-error`) and every `[role=alert]` text is recorded with its
 * timestamp — a 1 s error flash cannot fall between two samples.
 * usage: bun s15-boot-probe.mjs <shellUrl> <en,de> <boots-per-locale> <capture.json> */
import { writeFileSync } from "node:fs";
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
const [URL = "http://127.0.0.1:6540/", LOCALES = "en,de", BOOTS = "5", CAPTURE = "/Users/ueli/Documents/semio/.tmp-ticket/wp-s15/generated/s15-boot-probe.json"] = process.argv.slice(2);
const hook = () => {
  const t0 = performance.now();
  const frames = (window.__s15BootFrames = []);
  const record = (kind, value) => frames.push({ at: Math.round(performance.now() - t0), kind, value });
  const beacon = () => {
    const html = document.documentElement;
    return { ready: html?.getAttribute("data-semio-os-ready") ?? null, error: html?.getAttribute("data-semio-os-error") ?? null };
  };
  let last = "";
  const observe = () => {
    const now = beacon();
    const alerts = [...document.querySelectorAll("[role='alert']")].map((node) => node.textContent?.replace(/\s+/gu, " ").trim() ?? "").filter(Boolean).join(" | ");
    const key = JSON.stringify([now, alerts]);
    if (key === last) return;
    last = key;
    record("beacon", { ...now, alerts });
  };
  new MutationObserver(observe).observe(document, { subtree: true, childList: true, characterData: true, attributes: true, attributeFilter: ["data-semio-os-ready", "data-semio-os-error", "role"] });
  observe();
};
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--ignore-gpu-blocklist"] });
const runs = [];
for (const locale of LOCALES.split(",")) {
  for (let boot = 1; boot <= Number(BOOTS); boot += 1) {
    const context = await browser.newContext({ viewport: { width: 1600, height: 1000 }, locale: locale === "de" ? "de-DE" : "en-US" });
    await context.addInitScript(hook);
    const page = await context.newPage();
    const logs = [];
    const started = Date.now();
    page.on("console", (message) => { if (/error|warn/u.test(message.type())) logs.push(`${Date.now() - started}ms ${message.type()} ${message.text().slice(0, 400)}`); });
    await page.goto(URL, { waitUntil: "domcontentloaded", timeout: 180_000 });
    let readyAt = null;
    for (let tick = 0; tick < 360; tick += 1) {
      const ready = await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready"));
      if (ready && readyAt === null) readyAt = Date.now();
      if (readyAt !== null && Date.now() - readyAt > 8_000) break;
      await page.waitForTimeout(250);
    }
    const frames = await page.evaluate(() => window.__s15BootFrames);
    const errorFrames = frames.filter((frame) => frame.value.error || frame.value.alerts);
    runs.push({ locale, boot, ready: readyAt !== null, readyMs: readyAt === null ? null : readyAt - started, errorFrames, frames, console: logs.slice(-40) });
    process.stdout.write(`${locale}#${boot} ready=${readyAt !== null} errorFrames=${errorFrames.length}${errorFrames.length ? " " + JSON.stringify(errorFrames[0]).slice(0, 300) : ""}\n`);
    await context.close();
  }
}
await browser.close();
const failed = runs.filter((run) => !run.ready || run.errorFrames.length > 0);
writeFileSync(CAPTURE, JSON.stringify({ url: URL, boots: runs.length, failed: failed.length, runs }, null, 2));
console.log(`boots=${runs.length} failed=${failed.length}`);
process.exit(failed.length === 0 ? 0 : 1);
