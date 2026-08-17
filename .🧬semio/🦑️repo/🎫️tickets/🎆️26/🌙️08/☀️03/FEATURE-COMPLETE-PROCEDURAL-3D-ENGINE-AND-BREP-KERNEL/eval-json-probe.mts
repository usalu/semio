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
await page.goto("http://127.0.0.1:6018/?plugin=procedural3d", { waitUntil: "domcontentloaded", timeout: 240000 });
await page.waitForSelector(".semio-node-graph-host[data-status-json]", { timeout: 240000 });
// wait all ok
for (let i = 0; i < 60; i++) {
  const ok = await page.evaluate(() => {
    const status = JSON.parse(document.querySelector(".semio-node-graph-host")?.getAttribute("data-status-json") || "{}");
    const entries = Object.values(status) as Array<{ status?: string }>;
    return entries.length > 0 && entries.every((e) => e.status === "ok");
  });
  if (ok) break;
  await page.waitForTimeout(500);
}
await page.waitForTimeout(2000);
const dump = await page.evaluate(() => {
  const world = document.querySelector(".semio-world-3d-host");
  return {
    meshesJson: world?.getAttribute("data-meshes-json"),
    instancesJson: world?.getAttribute("data-instances-json"),
    statusJson: document.querySelector(".semio-node-graph-host")?.getAttribute("data-status-json"),
    // try to find eval json on graph extras if exposed
    worldHtml: world?.outerHTML.slice(0, 500),
  };
});
await writeFile(path.join(ticketDir, "eval-json-probe.json"), JSON.stringify({ dump, logs: logs.filter(l => /mesh|eval|preview|tessell|error|invoke/i.test(l)).slice(-50) }, null, 2));
console.log(JSON.stringify(dump, null, 2));
console.log("logs", logs.filter(l => /mesh|eval|preview|tessell|error|invoke|geometry/i.test(l)).slice(-30));
await browser.close();
