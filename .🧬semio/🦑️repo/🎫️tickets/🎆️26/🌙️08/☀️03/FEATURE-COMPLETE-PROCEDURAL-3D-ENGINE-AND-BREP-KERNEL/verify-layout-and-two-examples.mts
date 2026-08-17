#!/usr/bin/env bun
import { chromium } from "playwright";
import { writeFile, mkdir } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
const ticketDir = path.dirname(fileURLToPath(import.meta.url));
await mkdir(path.join(ticketDir, "example-shots"), { recursive: true });
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.goto("http://127.0.0.1:6018/?plugin=procedural3d&bust=" + Date.now(), { waitUntil: "domcontentloaded", timeout: 240000 });
await page.waitForSelector(".semio-node-graph-host[data-status-json]", { timeout: 240000 });
await page.waitForTimeout(8000);
const layout = async () => page.evaluate(() => {
  const box = (sel: string) => {
    const el = document.querySelector(sel) as HTMLElement | null;
    if (!el) return null;
    const r = el.getBoundingClientRect();
    return { w: Math.round(r.width), h: Math.round(r.height) };
  };
  return { graph: box(".semio-node-graph-host"), world: box(".semio-world-3d-host"), navbar: box("[data-slot=navbar]"), bodyScroll: document.body.scrollHeight };
});
const settle = async (ms = 60000) => {
  const deadline = Date.now() + ms;
  let last: any = null;
  while (Date.now() < deadline) {
    last = await page.evaluate(() => {
      const status = JSON.parse(document.querySelector(".semio-node-graph-host")?.getAttribute("data-status-json") || "{}");
      const meshes = JSON.parse(document.querySelector(".semio-world-3d-host")?.getAttribute("data-meshes-json") || "[]");
      const entries = Object.values(status) as any[];
      return {
        allOk: entries.length > 0 && entries.every((e) => e.status === "ok"),
        meshCount: meshes.length,
        blocked: Object.entries(status).filter(([, v]: any) => v.status === "blocked").map(([k]) => k),
        trigger: (document.getElementById("playground.navbar.fixture.trigger")?.textContent || "").trim(),
      };
    });
    if (last.allOk && last.meshCount > 0) return last;
    await page.waitForTimeout(1000);
  }
  return last;
};
const select = async (label: string) => page.evaluate((label) => {
  const trigger = document.getElementById("playground.navbar.fixture.trigger") as HTMLButtonElement | null;
  if (!trigger) return "no-trigger";
  if ((trigger.textContent || "").toLowerCase().includes(label.toLowerCase().slice(0, 10))) return "already";
  trigger.click();
  const opt = Array.from(document.querySelectorAll("[role=option], [data-slot=select-item], [data-radix-collection-item]")).find((n) => (n.textContent || "").toLowerCase().includes(label.toLowerCase()));
  if (!opt) { trigger.click(); return "missing"; }
  (opt as HTMLElement).click();
  return "clicked";
}, label);

const results: any[] = [];
const layout0 = await layout();
console.log("[DEBUG] layout0", JSON.stringify(layout0));
for (const ex of [
  { id: "hexagonal-mushroom-column", label: "Hexagonal Mushroom Column" },
  { id: "face-sweep-extrude", label: "Face Sweep" },
  { id: "rectangle-wire-preview", label: "Rectangle Wire" },
  { id: "rectangle-extrude-volume", label: "Rectangle Extrude" },
]) {
  const how = await select(ex.label);
  await page.waitForTimeout(3000);
  const snap = await settle(45000);
  const lay = await layout();
  const shot = path.join(ticketDir, "example-shots", ex.id + ".png");
  try { await page.screenshot({ path: shot, fullPage: false, timeout: 8000 }); } catch {}
  results.push({ ex, how, snap, lay });
  console.log("[DEBUG] done", JSON.stringify({ id: ex.id, how, snap, lay }));
}
await writeFile(path.join(ticketDir, "verify-layout-two.json"), JSON.stringify({ layout0, results }, null, 2));
await browser.close();
console.log("[DEBUG] report written");
