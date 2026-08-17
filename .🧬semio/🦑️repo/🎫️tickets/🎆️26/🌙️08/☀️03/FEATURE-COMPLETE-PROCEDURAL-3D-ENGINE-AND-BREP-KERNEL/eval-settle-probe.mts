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
await page.goto("http://127.0.0.1:6018/?plugin=procedural3d&bust=1786118674136623000", { waitUntil: "domcontentloaded", timeout: 240000 });
await page.waitForSelector(".semio-node-graph-host[data-status-json]", { timeout: 240000 });
const samples: unknown[] = [];
const deadline = Date.now() + 60_000;
while (Date.now() < deadline) {
  const snap = await page.evaluate(() => {
    const graph = document.querySelector(".semio-node-graph-host");
    const world = document.querySelector(".semio-world-3d-host");
    const statusJson = graph?.getAttribute("data-status-json") ?? "{}";
    const meshesJson = world?.getAttribute("data-meshes-json") ?? "[]";
    const instancesJson = world?.getAttribute("data-instances-json") ?? "[]";
    const worldStatusJson = world?.getAttribute("data-status-json") ?? "";
    let status: Record<string, { status?: string }> = {};
    try { status = JSON.parse(statusJson); } catch {}
    let meshCount = 0, instanceCount = 0;
    try { meshCount = JSON.parse(meshesJson).length; } catch {}
    try { instanceCount = JSON.parse(instancesJson).length; } catch {}
    const entries = Object.values(status);
    const allOk = entries.length > 0 && entries.every((e) => e.status === "ok");
    let evalKeys = [] as string[];
    let handleSample = [] as string[];
    try {
      const ws = JSON.parse(worldStatusJson || "{}");
      const raw = typeof ws.evalHead === "string" ? ws.evalHead : "";
      const ev = raw ? JSON.parse(raw) : (ws.eval ?? null);
      if (ev && typeof ev === "object") {
        evalKeys = Object.keys(ev);
        const walk = (v: unknown) => {
          if (!v || typeof v !== "object") return;
          if (Array.isArray(v)) { for (const x of v) walk(x); return; }
          const o = v as Record<string, unknown>;
          if (typeof o.handle === "string") handleSample.push(o.handle.slice(0, 20));
          for (const x of Object.values(o)) walk(x);
        };
        walk(ev);
      }
    } catch {}
    return { t: Date.now(), status, meshCount, instanceCount, allOk, evalKeys, handleSample: handleSample.slice(0, 8), meshesJson: meshesJson.slice(0, 200), instancesJson: instancesJson.slice(0, 200), worldStatusJson: worldStatusJson.slice(0, 900) };
  });
  samples.push(snap);
  console.log("[DEBUG] settle", JSON.stringify(snap));
  if (snap.allOk && snap.meshCount > 0) break;
  await page.waitForTimeout(1000);
}
await writeFile(path.join(ticketDir, "eval-settle-probe.json"), JSON.stringify({ samples, logs: logs.filter(l => /eval|mesh|status|error|fail|tessell|queued|computing/i.test(l)).slice(-100) }, null, 2));
await browser.close();
const last = samples.at(-1);
console.log("[DEBUG] final", JSON.stringify(last));
