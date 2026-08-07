#!/usr/bin/env bun
import { chromium } from "playwright";
import { writeFile, mkdir } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const ticketDir = path.dirname(fileURLToPath(import.meta.url));
const outDir = path.join(ticketDir, "example-shots");
await mkdir(outDir, { recursive: true });

const examples = [
  "hexagonal-mushroom-column",
  "rectangle-extrude-volume",
  "sphere-cut-with-torus",
  "box-fillet-preview",
  "sphere-box-fuse",
  "face-sweep-extrude",
  "rectangle-wire-preview",
  "box-shell-preview",
];

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.goto("http://127.0.0.1:6018/?plugin=procedural3d&bust=" + Date.now(), { waitUntil: "domcontentloaded", timeout: 240000 });
await page.waitForSelector(".semio-node-graph-host[data-status-json]", { timeout: 240000 });
await page.waitForTimeout(6000);

async function settle(timeoutMs = 60000) {
  const deadline = Date.now() + timeoutMs;
  let last: any = null;
  while (Date.now() < deadline) {
    last = await page.evaluate(() => {
      const status = JSON.parse(document.querySelector(".semio-node-graph-host")?.getAttribute("data-status-json") || "{}");
      const meshes = JSON.parse(document.querySelector(".semio-world-3d-host")?.getAttribute("data-meshes-json") || "[]");
      const entries = Object.values(status) as any[];
      return {
        allOk: entries.length > 0 && entries.every((e) => e.status === "ok"),
        meshCount: meshes.length,
        keys: Object.keys(status).sort(),
        blocked: Object.entries(status).filter(([, v]: any) => v.status === "blocked").map(([k]) => k),
        errors: Object.entries(status).filter(([, v]: any) => v.status === "error").map(([k]) => k),
        trigger: (document.getElementById("playground.navbar.fixture.trigger")?.textContent || "").trim(),
        graphH: Math.round((document.querySelector(".semio-node-graph-host") as HTMLElement | null)?.getBoundingClientRect().height || 0),
      };
    });
    if (last.allOk && last.meshCount > 0) return last;
    await page.waitForTimeout(1000);
  }
  return last;
}

const labels: Record<string, string> = {"hexagonal-mushroom-column": "Hexagonal Mushroom Column", "rectangle-extrude-volume": "Rectangle Extrude Volume", "sphere-cut-with-torus": "Sphere Cut With Torus", "box-fillet-preview": "Box Fillet Preview", "sphere-box-fuse": "Sphere Box Fuse", "face-sweep-extrude": "Face Sweep Extrude", "rectangle-wire-preview": "Rectangle Wire Preview", "box-shell-preview": "Box Shell Preview"};

async function selectExample(exampleId: string) {
  const label = labels[exampleId] ?? exampleId;
  return page.evaluate(async ({ id, label }) => {
    const trigger = document.getElementById("playground.navbar.fixture.trigger") as HTMLButtonElement | null;
    if (!trigger) return "no-trigger";
    const current = (trigger.textContent || "").trim();
    if (current === label) return "already:" + label;
    trigger.click();
    await new Promise((r) => setTimeout(r, 500));
    const items = Array.from(document.querySelectorAll("[data-slot=select-item], [role=option]")) as HTMLElement[];
    const item = items.find((el) => ((el.textContent || "").trim() === label));
    if (!item) {
      trigger.click();
      return "missing:" + items.map((el) => (el.textContent || "").trim()).slice(0, 12).join("|");
    }
    item.click();
    await new Promise((r) => setTimeout(r, 200));
    return "clicked:" + label + " -> " + ((document.getElementById("playground.navbar.fixture.trigger")?.textContent || "").trim());
  }, { id: exampleId, label });
}

const results: any[] = [];
for (const exampleId of examples) {
  console.log("[DEBUG] example start", exampleId);
  const how = await selectExample(exampleId);
  await page.waitForTimeout(2500);
  const snap = await settle(60000);
  const shot = path.join(outDir, exampleId + ".png");
  try { await page.screenshot({ path: shot, fullPage: false, timeout: 8000 }); } catch {}
  results.push({ exampleId, how, snap, shot });
  console.log("[DEBUG] example done", JSON.stringify({ exampleId, how, allOk: snap?.allOk, meshCount: snap?.meshCount, keys: snap?.keys, blocked: snap?.blocked, errors: snap?.errors, graphH: snap?.graphH }));
}
await writeFile(path.join(ticketDir, "example-shots-report.json"), JSON.stringify({ results }, null, 2));
await browser.close();
console.log("[DEBUG] report written");
