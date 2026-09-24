#!/usr/bin/env bun
/** 🚫️ S15 — a REFUSED kind inside `s`: every staged module of one plugin answers 404 (a guest that cannot be
 * instantiated on this device), the kind is opened from Home through the palette, and the shell must answer with a
 * visible, localized refusal notice (never a silent drop). Usage: bun s15-refused.mjs <baseUrl> <pluginDir> <pluginId> <locale> */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { awaitBeacon, dismissIntroduction, seatLocale, windowIds } from "../../../☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️s6-all-kinds-sweep.mjs";
const [baseUrl, pluginDir, pluginId, locale] = process.argv.slice(2);
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
const blocked = `/🔌️plugin-modules/${pluginDir}/`;
await context.route((url) => decodeURIComponent(url.pathname).includes(blocked) && !decodeURIComponent(url.pathname).endsWith("🔣️.json"), (route) => route.fulfill({ status: 404, body: "not staged on this device" }));
const page = await context.newPage();
await page.addInitScript(() => {
  const seen = [];
  Object.defineProperty(window, "__s15Notices", { value: seen });
  new MutationObserver(() => {
    for (const el of document.querySelectorAll("[data-semio-transient-notice]")) {
      const text = (el.firstChild?.textContent ?? el.textContent ?? "").trim();
      if (!seen.some((row) => row.text === text)) seen.push({ code: el.getAttribute("data-notice-code"), role: el.getAttribute("role") ?? el.closest("[role]")?.getAttribute("role") ?? null, text, lang: document.documentElement.lang });
    }
  }).observe(document, { subtree: true, childList: true, characterData: true });
});
const errors = [];
page.on("pageerror", (error) => errors.push(String(error).slice(0, 200)));
await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
const beacon = await awaitBeacon(page, Date.now() + 300_000);
await dismissIntroduction(page);
await page.waitForTimeout(2_000);
const seated = await seatLocale(page, locale);
await page.keyboard.press("Escape");
const before = await windowIds(page);
await page.evaluate(() => document.body.focus());
await page.keyboard.press("Meta+p");
const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
await input.waitFor({ state: "visible", timeout: 15_000 });
await input.fill(pluginId);
await page.waitForTimeout(1_500);
await page.locator(`[data-slot="command-item"][data-command-item-id="spawn.${pluginId}"]`).first().click();
await page.waitForTimeout(20_000);
const result = { beacon, seated, lang: await page.evaluate(() => document.documentElement.lang), opened: (await windowIds(page)).filter((id) => !before.includes(id)), notices: await page.evaluate(() => window.__s15Notices), pageErrors: errors };
await page.screenshot({ path: fileURLToPath(new URL(`./generated/s15-refused-${pluginId}-${locale}.png`, import.meta.url)) });
await browser.close();
writeFileSync(fileURLToPath(new URL(`./generated/s15-refused-${pluginId}-${locale}.json`, import.meta.url)), JSON.stringify(result, null, 1));
console.log(JSON.stringify(result, null, 1));
