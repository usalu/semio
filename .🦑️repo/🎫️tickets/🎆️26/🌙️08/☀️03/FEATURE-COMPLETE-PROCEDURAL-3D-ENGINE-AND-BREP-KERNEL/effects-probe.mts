#!/usr/bin/env bun
import { chromium } from "playwright";
import { writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
const ticketDir = path.dirname(fileURLToPath(import.meta.url));
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const logs: string[] = [];
page.on("console", (m) => logs.push(`[${m.type()}] ${m.text()}`));
page.on("pageerror", (e) => logs.push(`[pageerror] ${e.message}`));
await page.goto("http://127.0.0.1:6018/?plugin=procedural3d", { waitUntil: "domcontentloaded", timeout: 240000 });
await page.waitForSelector(".semio-node-graph-host[data-status-json]", { timeout: 240000 });
await page.waitForTimeout(15000);
const interesting = logs.filter((l) => /invokeExtension|flowEval|DispatchAction|Effects|extension handle|action failed|evaluate|pending|Effects|os-shell/i.test(l));
await writeFile(path.join(ticketDir, "effects-probe.json"), JSON.stringify({ interesting, allCount: logs.length, sample: logs.slice(0, 40), tail: logs.slice(-40) }, null, 2));
console.log(JSON.stringify(interesting, null, 2));
await browser.close();
