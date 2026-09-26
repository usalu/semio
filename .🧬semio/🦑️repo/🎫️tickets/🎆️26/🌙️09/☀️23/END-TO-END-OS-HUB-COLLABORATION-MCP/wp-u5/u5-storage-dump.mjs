#!/usr/bin/env bun
/** 🗄️ U5 — lists what the shell keeps in browser storage after a short session on Home: the top-level keys of the
 * `semio.os.config` preference map (never values) and every other storage key. Usage: bun u5-storage-dump.mjs <baseUrl> */
import { chromium } from "playwright";
import { dismissIntroduction } from "../../../☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️s6-all-kinds-sweep.mjs";

const [baseUrl] = process.argv.slice(2);
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady !== undefined, undefined, { timeout: 300_000 });
await dismissIntroduction(page);
await page.waitForTimeout(3_000);
await page.locator('button:has-text("History")').first().click({ force: true }).catch(() => undefined);
await page.waitForTimeout(2_000);
console.log(JSON.stringify(await page.evaluate(() => {
  const describe = (raw) => { try { const value = JSON.parse(raw); return typeof value === "object" && value !== null ? Object.fromEntries(Object.entries(value).map(([key, entry]) => [key, typeof entry === "object" && entry !== null ? Object.keys(entry).slice(0, 20) : typeof entry])) : typeof value; } catch { return `raw:${raw.length}`; } };
  return { local: Object.fromEntries(Object.keys(localStorage).map((key) => [key, describe(localStorage.getItem(key) ?? "")])), session: Object.keys(sessionStorage) };
}), null, 1));
await browser.close();
