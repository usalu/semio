#!/usr/bin/env bun
import { chromium } from "playwright";
import { writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
const ticketDir = path.dirname(fileURLToPath(import.meta.url));
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.goto(`http://127.0.0.1:6018/?plugin=procedural3d&bust=${Date.now()}`, { waitUntil: "domcontentloaded", timeout: 240000 });
await page.waitForSelector(".semio-node-graph-host[data-status-json]", { timeout: 240000 });
await page.waitForTimeout(4000);
await page.locator("#playground\\.navbar\\.fixture\\.trigger").click();
await page.waitForTimeout(1000);
const dump = await page.evaluate(() => {
  const items = Array.from(document.querySelectorAll("[role='option'], [data-radix-collection-item], [cmdk-item], li, [data-value]")).map((el) => ({
    tag: el.tagName,
    role: el.getAttribute("role"),
    value: el.getAttribute("data-value"),
    text: (el.textContent || "").trim().slice(0, 120),
    id: el.id,
  }));
  return items.filter((i) => i.text.length > 0).slice(0, 100);
});
await writeFile(path.join(ticketDir, "fixture-options-dump.json"), JSON.stringify(dump, null, 2));
console.log(JSON.stringify(dump, null, 2));
await browser.close();
