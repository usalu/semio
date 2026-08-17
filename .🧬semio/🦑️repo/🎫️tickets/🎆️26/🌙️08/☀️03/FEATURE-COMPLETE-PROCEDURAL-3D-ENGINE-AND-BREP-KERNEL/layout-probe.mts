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
await page.waitForSelector(".semio-node-graph-host", { timeout: 240000 });
await page.waitForTimeout(8000);
const dump = await page.evaluate(() => {
  const pick = (sel: string) => {
    const el = document.querySelector(sel) as HTMLElement | null;
    if (!el) return { sel, missing: true };
    const r = el.getBoundingClientRect();
    const cs = getComputedStyle(el);
    return {
      sel,
      w: r.width,
      h: r.height,
      display: cs.display,
      visibility: cs.visibility,
      overflow: cs.overflow,
      fixtureLen: el.getAttribute("data-fixture-json")?.length ?? 0,
      statusLen: el.getAttribute("data-status-json")?.length ?? 0,
      meshesLen: el.getAttribute("data-meshes-json")?.length ?? 0,
      instancesLen: el.getAttribute("data-instances-json")?.length ?? 0,
      childCount: el.children.length,
      className: el.className,
    };
  };
  return {
    navbar: pick('[data-slot="navbar"]'),
    graph: pick(".semio-node-graph-host"),
    world: pick(".semio-world-3d-host"),
    shell: pick('[data-slot="shell"]'),
    main: pick("main"),
    body: { w: document.body.clientWidth, h: document.body.clientHeight },
    html: document.documentElement.outerHTML.slice(0, 4000),
  };
});
await writeFile(path.join(ticketDir, "layout-probe.json"), JSON.stringify({ dump, logs: logs.slice(-80) }, null, 2));
await page.screenshot({ path: path.join(ticketDir, "layout-probe.png"), fullPage: false });
await browser.close();
console.log(JSON.stringify(dump, null, 2));
