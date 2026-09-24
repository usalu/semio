#!/usr/bin/env bun
/** 🔍️ S15 — opens Settings in `s` and dumps its controls (appearance/language/layout discovery). */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { awaitBeacon, dismissIntroduction, click } from "../../../☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️s6-all-kinds-sweep.mjs";
const baseUrl = process.argv[2] ?? "http://127.0.0.1:6540/";
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
await awaitBeacon(page, Date.now() + 300_000);
await dismissIntroduction(page);
await page.waitForTimeout(2_000);
const opened = await click(page, '[id="os.openSettings"], [data-slot="navbar"] [id*="settings" i], button:has-text("Settings")');
await page.waitForTimeout(3_000);
const dump = await page.evaluate(() => {
  const panel = [...document.querySelectorAll('[data-slot="panel"]')].find((el) => el instanceof HTMLElement && el.offsetParent !== null && /settings/iu.test(el.id));
  const root = panel ?? document.body;
  return {
    panelId: panel?.id ?? null,
    controls: [...root.querySelectorAll("button, select, input, [role='combobox'], [role='tab'], [role='switch'], [role='treeitem']")].filter((el) => el instanceof HTMLElement && el.offsetParent !== null).map((el) => `${el.tagName}#${el.id}[role=${el.getAttribute("role")}][aria=${el.getAttribute("aria-label")}] ${(el.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 40)}`).slice(0, 80),
    text: (root.textContent ?? "").replace(/\s+/gu, " ").slice(0, 800),
    storageKeys: Object.keys(localStorage).slice(0, 40),
  };
});
await page.screenshot({ path: fileURLToPath(new URL("./generated/s15-settings.png", import.meta.url)) });
await browser.close();
writeFileSync(fileURLToPath(new URL("./generated/s15-settings-dom.json", import.meta.url)), JSON.stringify({ opened, ...dump }, null, 1));
console.log(JSON.stringify({ opened, ...dump }, null, 1).slice(0, 5000));
