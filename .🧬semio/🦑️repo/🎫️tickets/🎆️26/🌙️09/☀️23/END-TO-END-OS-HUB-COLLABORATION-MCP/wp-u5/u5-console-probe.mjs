#!/usr/bin/env bun
/** 🩺️ U5 — spawns one app from the palette and prints every console warning/error the shell logs for it.
 * Usage: bun u5-console-probe.mjs <baseUrl> <pluginId> <appId|-> [seconds] */
import { chromium } from "playwright";
import { dismissIntroduction, openPalette } from "../../../☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️s6-all-kinds-sweep.mjs";

const [baseUrl, pluginId, appId, seconds = "15"] = process.argv.slice(2);
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
const t0 = Date.now();
page.on("console", (m) => { if (m.type() === "error" || m.type() === "warning" || m.text().includes("[DEBUG] u5")) console.log(`${Date.now() - t0} ${m.type()}: ${m.text().slice(0, 1500)}`); });
page.on("worker", (worker) => worker.on("console", (m) => { const text = m.text(); if (/error|fault|refus|fail|panic|\[DEBUG\]/iu.test(text)) console.log(`${Date.now() - t0} worker ${m.type()}: ${text.slice(0, 1200)}`); }));
page.on("pageerror", (e) => console.log(`${Date.now() - t0} pageerror: ${String(e).slice(0, 1500)}`));
await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady !== undefined, undefined, { timeout: 300_000 });
await dismissIntroduction(page);
await page.waitForTimeout(2_000);
console.log(`${Date.now() - t0} --- spawn`);
await openPalette(page);
await page.locator("[role='dialog'] [data-slot='command-input']").first().fill(/^s\.[^.]+\.([^@]+)@/u.exec(appId)?.[1] ?? pluginId);
const item = page.locator(`[data-slot="command-item"][data-command-item-id="${appId === "-" ? `spawn.${pluginId}` : `spawn.${pluginId}.${appId}`}"]`).first();
await item.waitFor({ state: "visible", timeout: 20_000 });
await item.click({ force: true });
for (let second = 0; second < Number(seconds); second += 3) {
  await page.waitForTimeout(3_000);
  const text = await page.evaluate(() => [...document.querySelectorAll("[data-window-id]")].map((element) => `${element.getAttribute("data-window-id")}: ${(element.textContent ?? "").replace(/\s+/gu, " ").slice(0, 300)}`));
  console.log(`${Date.now() - t0} windows ${JSON.stringify(text)}`);
}
if (process.env.U5_ARM_TOOL) {
  await page.locator('[id="framework.category.tool"]').first().click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(1_500);
  console.log(`${Date.now() - t0} tools ${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll('[id^="tool."]')].map((element) => `${element.id}:${element.getAttribute("aria-pressed")}`)))}`);
}
await page.locator(`[data-tab-id="framework.panel.toolRun"]`).first().click({ force: true }).catch(() => undefined);
await page.waitForTimeout(3_000);
if (process.env.U5_START) {
  await page.locator('[data-ui-node-key="framework.toolRun.ready.toolRunStart"]').first().click({ force: true }).catch((error) => console.log(`start click failed ${error}`));
  await page.waitForTimeout(Number(process.env.U5_START) * 1000);
}
console.log(JSON.stringify(await page.evaluate(() => [...document.querySelectorAll('[data-slot="panel"][data-panel-visible="true"]')].map((element) => ({ anchor: element.getAttribute("data-anchor"), tab: element.getAttribute("data-active-tab-id"), text: (element.querySelector('[data-slot="panel-content"]')?.textContent ?? "").replace(/\s+/gu, " ").slice(0, 400), keys: [...element.querySelectorAll("[data-ui-node-key]")].map((node) => node.getAttribute("data-ui-node-key")).slice(0, 30) })))));
if (0) console.log(await page.evaluate(() => { const panel = [...document.querySelectorAll('[data-slot="panel"][data-panel-visible="true"]')].map((element) => `${element.getAttribute("data-anchor")} ${element.getAttribute("data-active-tab-id")}: ${(element.querySelector(`[data-slot="panel-content"]`)?.innerHTML ?? "").replace(/class="[^"]*"|style="[^"]*"/gu, "").slice(0, 2500)}`); return panel.join("\n---\n"); }));
console.log((await page.evaluate(() => document.body.innerText)).split("\n").filter((line) => /missing=|refused|fault|error/iu.test(line)).join("\n").slice(0, 3000));
await page.screenshot({ path: (await import("node:url")).fileURLToPath(new URL("./generated/u5-console-probe.png", import.meta.url)) });
await browser.close();
