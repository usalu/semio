/** 🛍️ Boots the wgpu shell and reports whether the app-static catalogue document parses.
 *
 * Pass/fail signals, all from the shell's own `debug_log` traces:
 *   ❌ `wgpu shell app catalogue fetch failed: renderDocument result parse failed` — the defect
 *   ❌ `wgpu shell app catalogue reassembly failed`
 *   ✅ `wgpu shell app catalogue ready bytes=<n>`
 * Also counts `renderSurface` turns for `framework.section.catalogue` and screenshots the shell. */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6118/?plugin=generation3d";
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 75);
const outDir = join(import.meta.dir, "🗑️generated", "wgpu-catalogue", process.env.SEMIO_PROBE_OUT ?? "run");
mkdirSync(outDir, { recursive: true });
const lines = [];
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
const t0 = Date.now();
page.on("console", (msg) => lines.push(`${Date.now() - t0} ${msg.type()} ${msg.text().slice(0, 3000)}`));
page.on("pageerror", (err) => lines.push(`${Date.now() - t0} pageerror ${String(err).slice(0, 3000)}`));
await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(seconds * 1000);
await page.screenshot({ path: join(outDir, "shell.png"), type: "png" });
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));

const has = (needle) => lines.filter((line) => line.includes(needle));
const report = {
  url,
  seconds,
  lines: lines.length,
  parseFailed: has("app catalogue fetch failed"),
  reassemblyFailed: has("app catalogue reassembly failed"),
  ready: has("app catalogue ready bytes="),
  fixedMapRefusals: has("UiFixedMap requires"),
  catalogueRenders: has("framework.section.catalogue").length,
};
writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
console.log(JSON.stringify({ ...report, parseFailed: report.parseFailed.slice(0, 2), reassemblyFailed: report.reassemblyFailed.slice(0, 2), ready: report.ready.slice(0, 2), fixedMapRefusals: report.fixedMapRefusals.slice(0, 2) }, null, 2));
await browser.close();
