#!/usr/bin/env bun
import { chromium } from "playwright";
import { writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const ticketDir = path.dirname(fileURLToPath(import.meta.url));
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const logs: string[] = [];
page.on("console", (msg) => logs.push(`[console.${msg.type()}] ${msg.text()}`));
page.on("pageerror", (err) => logs.push(`[pageerror] ${err.message}`));

// First load a solid fixture, then wire via URL to reproduce switch
await page.goto("http://127.0.0.1:6018/?plugin=procedural3d&fixture=face-sweep-extrude&bust=" + Date.now(), { waitUntil: "domcontentloaded", timeout: 240000 });
await page.waitForSelector(".semio-node-graph-host[data-status-json]", { timeout: 240000 });
await page.waitForTimeout(8000);
const solid = await page.evaluate(() => ({
  trigger: (document.getElementById("playground.navbar.fixture.trigger")?.textContent || "").trim(),
  keys: Object.keys(JSON.parse(document.querySelector(".semio-node-graph-host")?.getAttribute("data-status-json") || "{}")),
  meshCount: JSON.parse(document.querySelector(".semio-world-3d-host")?.getAttribute("data-meshes-json") || "[]").length,
}));
console.log("[DEBUG] solid", JSON.stringify(solid));

await page.goto("http://127.0.0.1:6018/?plugin=procedural3d&fixture=rectangle-wire-preview&bust=" + Date.now(), { waitUntil: "domcontentloaded", timeout: 240000 });
await page.waitForSelector(".semio-node-graph-host[data-status-json]", { timeout: 240000 });
for (let i = 0; i < 30; i++) {
  const snap = await page.evaluate(() => {
    const status = JSON.parse(document.querySelector(".semio-node-graph-host")?.getAttribute("data-status-json") || "{}");
    const meshes = JSON.parse(document.querySelector(".semio-world-3d-host")?.getAttribute("data-meshes-json") || "[]");
    return {
      trigger: (document.getElementById("playground.navbar.fixture.trigger")?.textContent || "").trim(),
      keys: Object.keys(status).sort(),
      allOk: Object.values(status).every((e: any) => e.status === "ok"),
      meshCount: meshes.length,
      meshEdges: meshes.map((m: any) => m.data?.edgePositions?.length ?? 0),
      meshPositions: meshes.map((m: any) => m.data?.positions?.length ?? 0),
      meshSample: meshes[0]?.data ? Object.keys(meshes[0].data) : [],
    };
  });
  console.log("[DEBUG] wire tick", i, JSON.stringify(snap));
  if (snap.trigger.includes("Wire") && snap.allOk && snap.meshCount > 0) break;
  await page.waitForTimeout(1000);
}
await writeFile(path.join(ticketDir, "wire-debug-report.json"), JSON.stringify({ logs: logs.filter((l) => /tessellat|preview|mesh|error|Error|wire|edge/i.test(l)).slice(0, 80) }, null, 2));
await browser.close();
console.log("[DEBUG] done");
