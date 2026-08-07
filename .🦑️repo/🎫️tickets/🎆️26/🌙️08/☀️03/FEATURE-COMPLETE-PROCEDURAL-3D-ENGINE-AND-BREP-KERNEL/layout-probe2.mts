#!/usr/bin/env bun
import { chromium } from "playwright";
import { writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
const ticketDir = path.dirname(fileURLToPath(import.meta.url));
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.goto("http://127.0.0.1:6018/?plugin=procedural3d", { waitUntil: "domcontentloaded", timeout: 240000 });
await page.waitForSelector(".semio-node-graph-host[data-fixture-json]", { timeout: 240000 });
await page.waitForTimeout(10000);
const dump = await page.evaluate(() => {
  const graph = document.querySelector(".semio-node-graph-host");
  const world = document.querySelector(".semio-world-3d-host");
  const ancestry = (el: Element | null) => {
    const rows: Array<Record<string, unknown>> = [];
    let cur: Element | null = el;
    let depth = 0;
    while (cur && depth < 12) {
      const r = (cur as HTMLElement).getBoundingClientRect();
      const cs = getComputedStyle(cur as HTMLElement);
      rows.push({
        depth,
        tag: cur.tagName,
        id: (cur as HTMLElement).id,
        slot: cur.getAttribute("data-slot"),
        className: (cur as HTMLElement).className?.toString().slice(0, 120),
        w: r.width,
        h: r.height,
        display: cs.display,
        position: cs.position,
        height: cs.height,
        minHeight: cs.minHeight,
        flex: cs.flex,
        overflow: cs.overflow,
      });
      cur = cur.parentElement;
      depth++;
    }
    return rows;
  };
  return {
    fixtureJson: graph?.getAttribute("data-fixture-json"),
    statusJson: graph?.getAttribute("data-status-json"),
    meshesJson: world?.getAttribute("data-meshes-json"),
    instancesJson: world?.getAttribute("data-instances-json"),
    graphAncestry: ancestry(graph),
    worldAncestry: ancestry(world),
    canvasCount: document.querySelectorAll("canvas").length,
    canvasSizes: [...document.querySelectorAll("canvas")].slice(0, 5).map((c) => {
      const r = c.getBoundingClientRect();
      return { w: r.width, h: r.height, cw: (c as HTMLCanvasElement).width, ch: (c as HTMLCanvasElement).height };
    }),
  };
});
await writeFile(path.join(ticketDir, "layout-probe2.json"), JSON.stringify(dump, null, 2));
console.log(JSON.stringify({
  statusJson: dump.statusJson,
  meshesJson: dump.meshesJson,
  instancesJson: dump.instancesJson,
  fixtureWidgets: (() => { try { return JSON.parse(dump.fixtureJson || "{}").widgets?.length; } catch { return null; } })(),
  graphAncestry: dump.graphAncestry,
  worldAncestry: dump.worldAncestry,
  canvasSizes: dump.canvasSizes,
}, null, 2));
await browser.close();
