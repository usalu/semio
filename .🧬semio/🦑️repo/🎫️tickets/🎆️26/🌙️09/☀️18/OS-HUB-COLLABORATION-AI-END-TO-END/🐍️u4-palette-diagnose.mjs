#!/usr/bin/env bun
/** 🔎️ U4 diagnose: which palette rows the live `s` shell offers for a query (spawn ids for block 2d). */
import { chromium } from "playwright";
import { awaitBeacon, dismissIntroduction, openPalette } from "./🐍️s6-all-kinds-sweep.mjs";
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.goto(process.argv[2] ?? "http://127.0.0.1:6400/", { waitUntil: "commit", timeout: 300_000 });
console.log(await awaitBeacon(page, Date.now() + 300_000));
await dismissIntroduction(page);
await page.waitForTimeout(4_000);
await openPalette(page);
await page.locator("[role='dialog'] [data-slot='command-input']").first().fill(process.argv[3] ?? "block");
await page.waitForTimeout(2_000);
console.log(JSON.stringify(await page.evaluate(() => [...document.querySelectorAll('[data-slot="command-item"]')].map((item) => `${item.getAttribute("data-command-item-id")} | ${(item.textContent ?? "").trim().slice(0, 60)}`).slice(0, 30)), null, 1));
await browser.close();
