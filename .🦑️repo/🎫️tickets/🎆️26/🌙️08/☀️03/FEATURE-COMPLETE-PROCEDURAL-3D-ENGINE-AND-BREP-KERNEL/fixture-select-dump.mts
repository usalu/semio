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
await page.waitForTimeout(5000);
const dump = await page.evaluate(() => {
  const selects = Array.from(document.querySelectorAll("select")).map((s) => ({
    id: s.id,
    name: s.name,
    options: Array.from(s.options).map((o) => ({ value: o.value, text: o.text })),
  }));
  const fixtureish = Array.from(document.querySelectorAll("[id*='fixture'], [data-id*='fixture'], [class*='fixture'], [class*='example']")).map((el) => ({
    id: el.id,
    tag: el.tagName,
    className: typeof (el as any).className === "string" ? (el as any).className.slice(0, 120) : "",
    text: (el.textContent || "").trim().slice(0, 120),
  }));
  return { selects, fixtureish };
});
await writeFile(path.join(ticketDir, "fixture-select-dump.json"), JSON.stringify(dump, null, 2));
console.log(JSON.stringify(dump, null, 2));
await browser.close();
