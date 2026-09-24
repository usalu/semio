#!/usr/bin/env bun
/** ⌨️ S15 — why Tab does not move focus on Home: records every keydown's defaultPrevented + the listener target. */
import { chromium } from "playwright";
import { awaitBeacon, dismissIntroduction } from "../../../☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️s6-all-kinds-sweep.mjs";
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.addInitScript(() => {
  const log = [];
  Object.defineProperty(window, "__s15Keys", { value: log });
  window.addEventListener("keydown", (event) => { if (event.key === "Tab") queueMicrotask(() => log.push({ phase: "window-capture", prevented: event.defaultPrevented, target: event.target?.tagName })); }, true);
  window.addEventListener("keydown", (event) => { if (event.key === "Tab") log.push({ phase: "window-bubble", prevented: event.defaultPrevented, target: event.target?.tagName }); });
});
await page.goto(process.argv[2] ?? "http://127.0.0.1:6540/", { waitUntil: "commit", timeout: 300_000 });
await awaitBeacon(page, Date.now() + 300_000);
await dismissIntroduction(page);
await page.waitForTimeout(2_500);
const tabbables = await page.evaluate(() => [...document.querySelectorAll("button, a[href], input, select, textarea, [tabindex]")].filter((el) => el instanceof HTMLElement && el.offsetParent !== null && el.tabIndex >= 0).length);
await page.mouse.click(700, 450);
const seq = [];
for (let i = 0; i < 6; i++) {
  await page.keyboard.press("Tab");
  await page.waitForTimeout(150);
  seq.push(await page.evaluate(() => `${document.activeElement?.tagName}#${document.activeElement?.id}`));
}
console.log(JSON.stringify({ tabbables, seq, keys: await page.evaluate(() => window.__s15Keys) }, null, 1));
await browser.close();
