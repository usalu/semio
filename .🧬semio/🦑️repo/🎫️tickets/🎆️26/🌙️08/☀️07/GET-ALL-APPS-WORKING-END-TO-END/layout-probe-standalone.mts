#!/usr/bin/env bun
import { chromium } from "playwright";
import { writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const ticketDir = path.dirname(fileURLToPath(import.meta.url));
const url = process.env.PROBE_URL ?? "http://127.0.0.1:6018/";
const label = process.env.PROBE_LABEL ?? "procedural3d";
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const consoleErrors: string[] = [];
page.on("pageerror", (err) => consoleErrors.push(String(err)));
page.on("console", (msg) => {
  if (msg.type() === "error") consoleErrors.push(msg.text());
});
await page.goto(url, { waitUntil: "domcontentloaded", timeout: 240_000 });
await page.waitForTimeout(12_000);
const dump = await page.evaluate(() => {
  const layout = document.querySelector('[data-slot="layout"]') as HTMLElement | null;
  const scope = document.querySelector(".semio-scope") as HTMLElement | null;
  const host =
    (document.querySelector(".semio-node-graph-host") as HTMLElement | null) ??
    (document.querySelector(".semio-world-3d-host") as HTMLElement | null) ??
    (document.querySelector("canvas")?.parentElement as HTMLElement | null);
  const rect = (el: HTMLElement | null) => {
    if (!el) return null;
    const r = el.getBoundingClientRect();
    const cs = getComputedStyle(el);
    return {
      w: Math.round(r.width),
      h: Math.round(r.height),
      overflow: cs.overflow,
      flex: cs.flex,
      height: cs.height,
      className: (typeof el.className === "string" ? el.className : "").slice(0, 160),
    };
  };
  return {
    title: document.title,
    body: { scrollH: document.body.scrollHeight, clientH: document.body.clientHeight, scrollW: document.body.scrollWidth, clientW: document.body.clientWidth },
    hasRenderError: /Render error|is not defined/i.test(document.body.innerText),
    bodySample: document.body.innerText.split(/\n/).map((s) => s.trim()).filter(Boolean).slice(0, 30),
    layout: rect(layout),
    scope: rect(scope),
    host: rect(host),
  };
});
await page.screenshot({ path: path.join(ticketDir, `🧪layout-${label}.png`), fullPage: false });
const result = { label, url, dump, consoleErrors: consoleErrors.slice(0, 20) };
await writeFile(path.join(ticketDir, `🧪layout-${label}.json`), JSON.stringify(result, null, 2));
console.log(JSON.stringify(result, null, 2));
await browser.close();
const h = dump.host?.h ?? dump.layout?.h ?? 0;
const ok =
  !dump.hasRenderError &&
  h > 0 &&
  h < 2000 &&
  dump.body.scrollH <= dump.body.clientH + 50 &&
  dump.body.scrollW <= dump.body.clientW + 50;
if (!ok) {
  console.error("[FAIL] layout still broken", { h, scrollH: dump.body.scrollH, clientH: dump.body.clientH, hasRenderError: dump.hasRenderError });
  process.exit(1);
}
console.log("[OK]", label);
