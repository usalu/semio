/** 🧊️ wgpu browser-boot probe for the procedural 3d playground.
 *
 * The wgpu shell paints into ONE `<canvas id="semio-wgpu-canvas">` owned by the frame Worker, so the
 * React probes' `[data-status-json]` DOM oracle does not exist here. The three readable surfaces are
 * `🚀️browser-boot/🟦️.ts`'s `[role="status"]` progress line, its `[role="alert"]` fault banner, and
 * `window.semioWgpuIntrospection.dumpStructure()` / `.dumpFrameStats()` (attached only once the Worker
 * reports `booted` — waiting for the function is therefore a truthful boot gate).
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6118/?plugin=generation3d SEMIO_PROBE_OUT=wgpu-boot/run-1 bun 🐍️wgpu-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6118/?plugin=generation3d";
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 300);
const shotEvery = Number(process.env.SEMIO_PROBE_SHOT_SECONDS ?? 30);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-boot/run");
mkdirSync(outDir, { recursive: true });

const lines = [];
const timeline = [];
const t0 = Date.now();
const at = () => Date.now() - t0;
const browser = await chromium.launch({ headless: true, args: process.env.SEMIO_PROBE_WEBGPU === "0" ? [] : ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 2000)}`));
page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 2000)}`));
page.on("requestfailed", (r) => lines.push(`${at()} requestfailed ${r.url().slice(0, 300)} ${r.failure()?.errorText ?? ""}`));

/** 👁️ One sample of everything the UI isolate can answer without the retained tree. */
const snap = () => page.evaluate(async () => {
  const text = (selector) => document.querySelector(selector)?.textContent?.replace(/\s+/g, " ").trim() ?? null;
  const beacon = window.semioWgpuIntrospection;
  let dump = null;
  let stats = null;
  if (typeof beacon?.dumpStructure === "function") {
    try { const raw = await beacon.dumpStructure(); dump = raw ? JSON.parse(raw) : null; } catch (error) { dump = { error: String(error).slice(0, 300) }; }
  }
  if (typeof beacon?.dumpFrameStats === "function") {
    try { const raw = await beacon.dumpFrameStats(); stats = raw ? JSON.parse(raw) : null; } catch (error) { stats = { error: String(error).slice(0, 300) }; }
  }
  const nodes = Array.isArray(dump?.nodes) ? dump.nodes : [];
  const canvas = document.getElementById("semio-wgpu-canvas");
  return {
    status: text('[role="status"]'),
    alert: document.querySelector('[role="alert"]')?.textContent?.slice(0, 3000) ?? null,
    canvas: canvas ? { w: canvas.width, h: canvas.height } : null,
    beacon: typeof beacon?.dumpStructure === "function",
    nodeCount: nodes.length,
    kinds: [...new Set(nodes.map((n) => n.kind))].slice(0, 40),
    paths: nodes.map((n) => n.path).filter((p) => typeof p === "string" && /window|surface|panel|navbar/i.test(p)).slice(0, 60),
    texts: nodes.map((n) => n.text).filter((t) => typeof t === "string" && t.trim().length > 0).slice(0, 120),
    scenes: nodes.filter((n) => n.kind === "componentScene").map((n) => ({ path: n.path, rect: n.rect })).slice(0, 20),
    stats,
  };
});

const write = () => {
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
  writeFileSync(join(outDir, "timeline.json"), JSON.stringify(timeline, null, 2));
};

await page.goto(url, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 500)}`));

let shots = 0;
let settledFor = 0;
let last = null;
for (let second = 0; second < seconds; second += 1) {
  await page.waitForTimeout(1000);
  const sample = await snap().catch((error) => ({ error: String(error).slice(0, 300) }));
  sample.t = at();
  timeline.push(sample);
  last = sample;
  if (sample.t >= (shots + 1) * shotEvery * 1000) {
    shots += 1;
    await page.screenshot({ path: join(outDir, `shot-${String(shots).padStart(2, "0")}-${Math.round(sample.t / 1000)}s.png`), type: "png" }).catch(() => {});
    write();
  }
  if (sample.alert) { lines.push(`${at()} PROBE fault banner`); break; }
  if (sample.beacon && sample.nodeCount > 0) { settledFor += 1; if (settledFor >= 20) break; } else settledFor = 0;
}

await page.screenshot({ path: join(outDir, "final.png"), type: "png" }).catch(() => {});
write();
writeFileSync(join(outDir, "final.json"), JSON.stringify(last, null, 2));
console.log("DONE seconds", Math.round(at() / 1000), "console", lines.length, "beacon", last?.beacon, "nodes", last?.nodeCount, "status", last?.status, "alert", last?.alert ? last.alert.slice(0, 200) : null);
await browser.close();
