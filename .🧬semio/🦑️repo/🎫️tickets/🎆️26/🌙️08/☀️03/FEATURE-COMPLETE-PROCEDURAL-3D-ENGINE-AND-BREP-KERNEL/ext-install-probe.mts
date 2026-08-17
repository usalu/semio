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
await page.goto(`http://127.0.0.1:6018/?plugin=procedural3d&bust=${Date.now()}`, { waitUntil: "domcontentloaded", timeout: 240000 });
await page.waitForSelector(".semio-node-graph-host", { timeout: 240000 });
await page.waitForTimeout(8000);
const snap = await page.evaluate(() => {
  const graph = document.querySelector(".semio-node-graph-host");
  const world = document.querySelector(".semio-world-3d-host");
  return {
    statusJson: graph?.getAttribute("data-status-json")?.slice(0, 1000) ?? "",
    worldStatus: world?.getAttribute("data-status-json")?.slice(0, 1500) ?? "",
    meshes: world?.getAttribute("data-meshes-json")?.slice(0, 200) ?? "",
  };
});
const relevant = logs.filter((l) =>
  /contribution|extension|setContributions|invokeExtension|unknown kind|brep|math\.|Pending|plugin|error|fail/i.test(l),
).slice(-200);
await writeFile(path.join(ticketDir, "ext-install-probe.json"), JSON.stringify({ snap, relevant, logCount: logs.length }, null, 2));
console.log(JSON.stringify({ snap, relevant: relevant.slice(-40) }, null, 2));
await browser.close();
