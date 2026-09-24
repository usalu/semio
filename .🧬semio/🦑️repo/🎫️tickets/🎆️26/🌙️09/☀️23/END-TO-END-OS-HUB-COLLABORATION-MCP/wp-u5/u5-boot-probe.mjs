#!/usr/bin/env bun
/** 🔭️ U5 — quick boot probe: beacon, device attribute, footer badges, console faults, screenshot. */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const baseUrl = process.argv[2] ?? "http://127.0.0.1:6580/";
const width = Number(process.argv[3] ?? 1440);
const height = Number(process.argv[4] ?? 900);
const tag = process.argv[5] ?? `boot-${width}x${height}`;
const out = (name) => fileURLToPath(new URL(`./generated/${name}`, import.meta.url));
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const context = await browser.newContext({ viewport: { width, height }, hasTouch: width < 1024, isMobile: width < 768 });
const page = await context.newPage();
const lines = [];
page.on("console", (m) => lines.push(`${m.type()}: ${m.text()}`.slice(0, 400)));
page.on("pageerror", (e) => lines.push(`pageerror: ${String(e)}`.slice(0, 400)));
const started = Date.now();
await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
let beacon = null;
while (Date.now() - started < 300_000) {
  beacon = await page.evaluate(() => {
    const d = document.documentElement.dataset;
    return d.semioOsReady !== undefined ? `ready:${d.semioOsReady}` : d.semioOsError !== undefined ? `error:${d.semioOsError}` : null;
  });
  if (beacon) break;
  await page.waitForTimeout(1000);
}
await page.waitForTimeout(4000);
const state = await page.evaluate(() => ({
  device: [...document.querySelectorAll("[data-ui-device]")].map((el) => el.getAttribute("data-ui-device")),
  hub: document.querySelector("[data-semio-hub-connection]")?.getAttribute("data-semio-hub-connection") ?? null,
  hubLabel: document.querySelector("[data-semio-hub-connection]")?.getAttribute("aria-label") ?? null,
  approvals: document.querySelector("[data-semio-agent-approvals-waiting]")?.getAttribute("data-semio-agent-approvals-waiting") ?? null,
  windows: [...document.querySelectorAll("[data-window-id]")].map((el) => el.getAttribute("data-window-id")),
  scrollWidth: document.documentElement.scrollWidth,
  clientWidth: document.documentElement.clientWidth,
  lang: document.documentElement.lang,
}));
await page.screenshot({ path: out(`u5-${tag}.png`) });
writeFileSync(out(`u5-${tag}.json`), JSON.stringify({ baseUrl, width, height, ms: Date.now() - started, beacon, state, console: lines.slice(-80) }, null, 1));
console.log(JSON.stringify({ beacon, state, faults: lines.filter((l) => /error|pageerror/i.test(l)).length }));
await browser.close();
