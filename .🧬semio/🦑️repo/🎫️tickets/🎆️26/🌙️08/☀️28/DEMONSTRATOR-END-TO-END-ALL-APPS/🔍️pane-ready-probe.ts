/** 🔍️ Deep-links one demonstrator pane headlessly and prints its `data-shell-*` beacon timeline plus non-debug console errors. Read-only. */
import { chromium } from "playwright";
const BASE = process.env.PROBE_BASE_URL ?? "http://127.0.0.1:6029/";
const PANE = process.env.PROBE_PANE ?? "generator";
const BUDGET_MS = Number(process.env.PROBE_BUDGET_MS ?? 120_000);
const browser = await chromium.launch({ args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
const errors: string[] = [];
page.on("pageerror", (e) => errors.push("PAGEERROR " + String(e).slice(0, 200)));
page.on("console", (m) => { const t = m.text(); if (m.type() === "error" && !t.startsWith("[DEBUG]")) errors.push(t.slice(0, 200)); });
const t0 = Date.now();
await page.goto(`${BASE}#${PANE}`, { waitUntil: "domcontentloaded", timeout: 60_000 });
let state = "";
while (Date.now() - t0 < BUDGET_MS) {
  const s = await page.$eval(`[data-shell-id="${PANE}"]`, (e) => e.hasAttribute("data-shell-ready") ? "ready" : e.hasAttribute("data-shell-error") ? "error:" + (e.getAttribute("data-shell-error") ?? "").slice(0, 160) : "booting").catch(() => "absent");
  if (s !== state) { state = s; console.log(`${PANE} ${((Date.now() - t0) / 1000).toFixed(0)}s: ${s}`); }
  if (s === "ready" || s.startsWith("error")) break;
  await page.waitForTimeout(3_000);
}
await page.waitForTimeout(5_000);
const windows = await page.$$eval('[id^="framework.window."]', (els) => [...new Set(els.map((e) => e.id.split(".").slice(0, 3).join(".")))]);
console.log(`${PANE} windows:`, windows.join(", "), "| canvases:", await page.$$eval("canvas", (c) => c.length));
console.log(`${PANE} non-debug errors (${errors.length}):`); for (const e of [...new Set(errors)].slice(0, 5)) console.log("  -", e);
await browser.close();
