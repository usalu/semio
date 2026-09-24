#!/usr/bin/env bun
/** 🔍️ S15 — opens one program and activates each of its dock tabs with a real pointer click, reporting which window
 * elements mount. Usage: bun s15-stack-tab.mjs <baseUrl> <pluginId> <appId> */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { awaitBeacon, dismissIntroduction, windowIds } from "../../../☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️s6-all-kinds-sweep.mjs";
const [baseUrl, pluginId, appId] = process.argv.slice(2);
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const lines = [];
page.on("console", (m) => { if (m.type() === "error" || m.type() === "warning") lines.push(`${m.type()}: ${m.text()}`.slice(0, 300)); });
await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
await awaitBeacon(page, Date.now() + 300_000);
await dismissIntroduction(page);
await page.waitForTimeout(2_000);
await page.evaluate(() => document.body.focus());
await page.keyboard.press("Meta+p");
const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
await input.waitFor({ state: "visible", timeout: 15_000 });
await input.fill(/^s\.[^.]+\.([^@]+)@/u.exec(appId)?.[1] ?? pluginId);
await page.waitForTimeout(1_200);
await page.locator(`[data-slot="command-item"][data-command-item-id="spawn.${pluginId}.${appId}"]`).first().click();
await page.waitForTimeout(8_000);
const state = () => page.evaluate(() => ({
  windows: [...document.querySelectorAll('[data-slot="window"]')].map((el) => el.id),
  tabs: [...document.querySelectorAll('[data-slot="mode-dock-tab"]')].map((el) => ({ id: el.getAttribute("data-window-id"), active: el.getAttribute("data-active") ?? el.getAttribute("aria-selected") ?? el.getAttribute("data-state"), rect: (() => { const r = el.getBoundingClientRect(); return [Math.round(r.left), Math.round(r.top), Math.round(r.width), Math.round(r.height)]; })() })),
}));
const steps = [{ step: "opened", ...(await state()) }];
for (const tab of steps[0].tabs) {
  const [x, y, w, h] = tab.rect;
  await page.mouse.click(x + Math.min(20, w / 2), y + h / 2);
  await page.waitForTimeout(3_000);
  steps.push({ step: `clicked ${tab.id}`, ...(await state()) });
}
await page.screenshot({ path: fileURLToPath(new URL(`./generated/s15-stack-${pluginId}.png`, import.meta.url)) });
await browser.close();
writeFileSync(fileURLToPath(new URL(`./generated/s15-stack-${pluginId}.json`, import.meta.url)), JSON.stringify({ steps, lines: lines.slice(0, 20) }, null, 1));
console.log(JSON.stringify({ steps, lines: lines.slice(0, 8) }, null, 1));
