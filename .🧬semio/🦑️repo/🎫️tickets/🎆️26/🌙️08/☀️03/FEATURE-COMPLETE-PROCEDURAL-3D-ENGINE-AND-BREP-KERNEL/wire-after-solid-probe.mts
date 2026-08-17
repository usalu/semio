#!/usr/bin/env bun
import { chromium } from "playwright";
import { writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const ticketDir = path.dirname(fileURLToPath(import.meta.url));
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const logs: string[] = [];
page.on("console", (msg) => {
  const t = msg.text();
  if (/\[DEBUG\]|tessellat|preview|HostEffect|flowTessellate|mesh/i.test(t)) logs.push(t);
});

await page.goto("http://127.0.0.1:6018/?plugin=procedural3d&bust=" + Date.now(), { waitUntil: "domcontentloaded", timeout: 240000 });
await page.waitForSelector(".semio-node-graph-host[data-status-json]", { timeout: 240000 });
await page.waitForTimeout(5000);

async function select(label: string) {
  for (let i = 0; i < 8; i++) {
    const how = await page.evaluate(async (label) => {
      const trigger = document.getElementById("playground.navbar.fixture.trigger") as HTMLButtonElement | null;
      if (!trigger) return "no-trigger";
      if ((trigger.textContent || "").trim() === label) return "already";
      trigger.click();
      await new Promise((r) => setTimeout(r, 800));
      const items = Array.from(document.querySelectorAll("[data-slot=select-item], [role=option]")) as HTMLElement[];
      const item = items.find((el) => (el.textContent || "").trim() === label);
      if (!item) return "missing";
      item.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true }));
      item.click();
      return "clicked";
    }, label);
    await page.waitForTimeout(1500);
    const trigger = await page.evaluate(() => (document.getElementById("playground.navbar.fixture.trigger")?.textContent || "").trim());
    console.log("[DEBUG] select", label, how, trigger);
    if (trigger === label) return true;
  }
  return false;
}

await select("Face Sweep Extrude");
await page.waitForTimeout(5000);
const solid = await page.evaluate(() => ({
  trigger: (document.getElementById("playground.navbar.fixture.trigger")?.textContent || "").trim(),
  meshCount: JSON.parse(document.querySelector(".semio-world-3d-host")?.getAttribute("data-meshes-json") || "[]").length,
  keys: Object.keys(JSON.parse(document.querySelector(".semio-node-graph-host")?.getAttribute("data-status-json") || "{}")),
}));
console.log("[DEBUG] solid snap", JSON.stringify(solid));

await select("Rectangle Wire Preview");
const ticks = [];
for (let i = 0; i < 40; i++) {
  const snap = await page.evaluate(() => {
    const status = JSON.parse(document.querySelector(".semio-node-graph-host")?.getAttribute("data-status-json") || "{}");
    const meshes = JSON.parse(document.querySelector(".semio-world-3d-host")?.getAttribute("data-meshes-json") || "[]");
    const host = document.querySelector(".semio-node-graph-host");
    return {
      trigger: (document.getElementById("playground.navbar.fixture.trigger")?.textContent || "").trim(),
      keys: Object.keys(status).sort(),
      allOk: Object.values(status).every((e: any) => e.status === "ok"),
      meshCount: meshes.length,
      meshEdges: meshes.map((m: any) => m.data?.edgePositions?.length ?? 0),
      effects: host?.getAttribute("data-effects-json") || host?.getAttribute("data-pending-effects-json") || null,
      statusJsonLen: (host?.getAttribute("data-status-json") || "").length,
    };
  });
  ticks.push(snap);
  console.log("[DEBUG] wire tick", i, JSON.stringify(snap));
  if (snap.trigger === "Rectangle Wire Preview" && snap.allOk && snap.meshCount > 0) break;
  await page.waitForTimeout(1000);
}
await writeFile(path.join(ticketDir, "wire-after-solid-report.json"), JSON.stringify({ solid, ticks, logs: logs.slice(-100) }, null, 2));
await browser.close();
console.log("[DEBUG] done");
