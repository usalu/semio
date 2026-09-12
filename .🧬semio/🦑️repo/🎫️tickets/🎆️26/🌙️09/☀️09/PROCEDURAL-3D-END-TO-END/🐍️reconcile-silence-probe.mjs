import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

// 🩺️ Localizes the ~10.4 s of total console silence that follows every `reactor more-work
// sources=["reconcile"]` answer (`📓️wasm-hot-path-opt-level-2026-09-12.md` §4.3) WITHOUT touching
// host source: an init script installs (a) a `longtask` PerformanceObserver, so a blocked main thread
// names itself, and (b) a MessageChannel-driven heartbeat, so an IDLE main thread names itself too —
// a beat that keeps ticking through the silence proves the host is waiting, not computing.
const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 150);
const outDir = join(import.meta.dir, "🗑️generated", "reconcile-silence", process.env.SEMIO_PROBE_OUT ?? "run");
mkdirSync(outDir, { recursive: true });

const lines = [];
const browser = await chromium.launch({ headless: true });
const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
if ((process.env.SEMIO_PROBE_DIAGNOSTICS ?? "1") !== "0") {
  await context.addInitScript(() => { try { window.localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
}
await context.addInitScript(() => {
  const t0 = performance.now();
  try {
    new PerformanceObserver((list) => {
      for (const entry of list.getEntries()) {
        if (entry.duration < 120) continue;
        const attribution = (entry.attribution ?? []).map((a) => `${a.name}:${a.containerType ?? ""}:${a.containerName ?? ""}:${a.containerSrc ?? ""}`).join("|");
        console.log(`[DEBUG] longtask start=${entry.startTime.toFixed(1)} dur=${entry.duration.toFixed(1)} name=${entry.name} attr=${attribution}`);
      }
    }).observe({ entryTypes: ["longtask"] });
  } catch (error) { console.log(`[DEBUG] longtask observer unavailable ${String(error)}`); }
  const channel = new MessageChannel();
  let last = performance.now();
  let beats = 0;
  channel.port1.onmessage = () => {
    const now = performance.now();
    const late = now - last;
    beats += 1;
    if (late > 150 || beats % 200 === 0) console.log(`[DEBUG] mainbeat at=${(now - t0).toFixed(1)} sinceLastBeat=${late.toFixed(1)} beats=${beats}`);
    last = now;
    channel.port2.postMessage(0);
  };
  channel.port2.postMessage(0);
  // ⏱️ A second heartbeat on a real timer: if the MessageChannel beat stalls but this one does not,
  // the macrotask queue itself is saturated rather than the thread being blocked.
  let timerLast = performance.now();
  const timerBeat = () => {
    const now = performance.now();
    const late = now - timerLast - 100;
    if (late > 150) console.log(`[DEBUG] timerbeat at=${(now - t0).toFixed(1)} lateBy=${late.toFixed(1)}`);
    timerLast = now;
    setTimeout(timerBeat, 100);
  };
  setTimeout(timerBeat, 100);
});

const page = await context.newPage();
const t0 = performance.now();
page.on("console", (msg) => lines.push(`${(performance.now() - t0).toFixed(2)} ${msg.type()} ${msg.text().slice(0, 4000)}`));
page.on("pageerror", (err) => lines.push(`${(performance.now() - t0).toFixed(2)} pageerror ${String(err).slice(0, 2000)}`));
await page.goto(url, { waitUntil: "domcontentloaded" });

const readSurfaces = () => page.evaluate(() => [...document.querySelectorAll("[data-status-json]")].map((el) => ({
  surfaceId: el.getAttribute("data-surface-id"),
  status: el.getAttribute("data-status-json") ?? "",
  meshes: (() => { try { const v = JSON.parse(el.getAttribute("data-meshes-json") ?? "[]"); return Array.isArray(v) ? v.length : 0; } catch { return 0; } })(),
})));
let allOkMs = null, meshesMs = null, meshesSeen = 0;
const deadline = performance.now() + seconds * 1000;
while (performance.now() < deadline) {
  await page.waitForTimeout(250);
  let surfaces = [];
  try { surfaces = await readSurfaces(); } catch { continue; }
  const main = surfaces.find((s) => s.surfaceId === "window:procedural-main");
  if (allOkMs === null && main && main.status.length > 0) {
    try {
      const nodes = Object.values(JSON.parse(main.status));
      if (nodes.length >= 7 && nodes.every((n) => n && n.status === "ok")) allOkMs = Number((performance.now() - t0).toFixed(2));
    } catch {}
  }
  const best = Math.max(0, ...surfaces.map((s) => s.meshes));
  if (best > meshesSeen) { meshesSeen = best; if (meshesMs === null && best >= 3) meshesMs = Number((performance.now() - t0).toFixed(2)); }
  if (allOkMs !== null && meshesMs !== null) break;
}
const hosts = await page.evaluate(() => [...document.querySelectorAll("[data-status-json]")].map((el) => ({
  surfaceId: el.getAttribute("data-surface-id"),
  status: el.getAttribute("data-status-json")?.slice(0, 1500),
  meshes: (() => { try { const v = JSON.parse(el.getAttribute("data-meshes-json") ?? "[]"); return Array.isArray(v) ? v.length : 0; } catch { return 0; } })(),
})));
await page.screenshot({ path: join(outDir, "final.png"), type: "png" });
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "hosts.json"), JSON.stringify(hosts, null, 2));
writeFileSync(join(outDir, "convergence.json"), JSON.stringify({ url, seconds, allOkMs, meshesMs, meshesSeen, consoleLines: lines.length }, null, 2));
console.log("DONE lines", lines.length, "allOkMs", allOkMs, "meshesMs", meshesMs, "meshesSeen", meshesSeen);
await browser.close();
