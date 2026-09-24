#!/usr/bin/env bun
/** 🔍️ S15 — opens ONE app-qualified program from Home by the palette chord and dumps what the shell did:
 * window elements, dock tabs, console lines, notices. Usage: bun s15-open-diagnose.mjs <baseUrl> <pluginId> <appId> */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { awaitBeacon, dismissIntroduction, windowIds } from "../../../☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️s6-all-kinds-sweep.mjs";

const [baseUrl, pluginId, appId] = process.argv.slice(2);
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const lines = [];
page.on("console", (m) => lines.push(`${m.type()}: ${m.text()}`.slice(0, 500)));
page.on("pageerror", (e) => lines.push(`pageerror: ${String(e)}`.slice(0, 500)));
await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
const beacon = await awaitBeacon(page, Date.now() + 300_000);
await dismissIntroduction(page);
await page.waitForTimeout(3_000);
const cursor = lines.length;
await page.evaluate(() => document.body.focus());
await page.keyboard.press("Meta+p");
const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
await input.waitFor({ state: "visible", timeout: 15_000 });
await input.fill(/^s\.[^.]+\.([^@]+)@/u.exec(appId)?.[1] ?? pluginId);
await page.waitForTimeout(1_500);
const items = await page.evaluate(() => [...document.querySelectorAll('[data-slot="command-item"]')].map((el) => el.getAttribute("data-command-item-id")).slice(0, 20));
await page.locator(`[data-slot="command-item"][data-command-item-id="spawn.${pluginId}.${appId}"]`).first().click({ timeout: 10_000 }).catch((error) => lines.push(`probe: click failed ${String(error).slice(0, 200)}`));
const samples = [];
for (let i = 0; i < 12; i += 1) {
  await page.waitForTimeout(5_000);
  samples.push(await page.evaluate(() => ({
    windows: [...document.querySelectorAll('[data-slot="window"]')].map((el) => el.id),
    tabs: [...document.querySelectorAll('[data-slot="mode-dock-tab"]')].map((el) => `${el.getAttribute("data-window-id")}|${(el.textContent ?? "").trim().slice(0, 30)}`),
    notices: [...document.querySelectorAll("[data-semio-transient-notice]")].map((el) => (el.textContent ?? "").trim().slice(0, 120)),
    breadcrumb: (document.querySelector('[data-slot="navbar"]')?.textContent ?? "").replace(/\s+/gu, " ").slice(0, 160),
  })));
}
await page.screenshot({ path: fileURLToPath(new URL(`./generated/s15-open-${pluginId}-${appId.replace(/[^A-Za-z0-9]+/gu, "-")}.png`, import.meta.url)) });
await browser.close();
const out = fileURLToPath(new URL(`./generated/s15-open-${pluginId}-${appId.replace(/[^A-Za-z0-9]+/gu, "-")}.json`, import.meta.url));
writeFileSync(out, JSON.stringify({ beacon, items, samples, console: lines.slice(cursor).filter((line) => !/agent-bridge|typed-operation slots|Failed to load resource/u.test(line)).slice(0, 80) }, null, 1));
console.log(out);
