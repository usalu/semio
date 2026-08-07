#!/usr/bin/env bun
import { chromium } from "playwright";
import { writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
const ticketDir = path.dirname(fileURLToPath(import.meta.url));
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.goto("http://127.0.0.1:6018/?plugin=procedural3d&bust=" + Date.now(), { waitUntil: "domcontentloaded", timeout: 240000 });
await page.waitForSelector(".semio-node-graph-host", { timeout: 240000 });
await page.waitForTimeout(8000);
const dump = await page.evaluate(() => {
  const host = document.querySelector(".semio-node-graph-host") as HTMLElement | null;
  if (!host) return { missing: true };
  const chain: any[] = [];
  let el: HTMLElement | null = host;
  while (el && chain.length < 20) {
    const cs = getComputedStyle(el);
    const r = el.getBoundingClientRect();
    chain.push({
      tag: el.tagName,
      id: el.id,
      slot: el.getAttribute("data-slot"),
      className: (typeof el.className === "string" ? el.className : "").slice(0, 160),
      w: Math.round(r.width),
      h: Math.round(r.height),
      display: cs.display,
      position: cs.position,
      overflow: cs.overflow,
      height: cs.height,
      minHeight: cs.minHeight,
      flex: cs.flex,
      flexGrow: cs.flexGrow,
      flexBasis: cs.flexBasis,
    });
    el = el.parentElement;
  }
  return { chain, body: { scrollH: document.body.scrollHeight, clientH: document.body.clientHeight } };
});
await writeFile(path.join(ticketDir, "layout-ancestors.json"), JSON.stringify(dump, null, 2));
console.log(JSON.stringify(dump, null, 2));
await browser.close();
