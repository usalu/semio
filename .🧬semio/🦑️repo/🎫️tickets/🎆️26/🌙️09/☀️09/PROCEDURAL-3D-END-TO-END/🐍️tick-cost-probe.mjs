import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

// 🩺️ Same harness as 🐍️console-dump-probe.mjs, plus: (a) arms the shell's own
// SEMIO_RUNTIME_DIAGNOSTICS localStorage switch before any page script runs (see
// 📓️example-switch-runtime-2026-09-12.md §1.1 — no source change needed), and (b) timestamps every
// [DEBUG] line with a monotonic performance.now()-class clock so gaps between lines are measurable
// to sub-ms precision instead of the 1 ms Date.now() bucket the older probe used.
const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 60);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "tick-cost");
mkdirSync(outDir, { recursive: true });

const lines = [];
const debugLines = [];
const browser = await chromium.launch({ headless: true });
const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
// 🩺️ As of 26/09/12 this switch reaches the GUEST too (the shard worker seeds
// `wasi:cli/environment` from it — `📓️wasm-hot-path-opt-level-2026-09-12.md` §1), so an armed run
// now carries the guest's own per-turn Rust traces and pays their console cost. Set
// SEMIO_PROBE_DIAGNOSTICS=0 for an uninstrumented convergence measurement.
const armDiagnostics = (process.env.SEMIO_PROBE_DIAGNOSTICS ?? "1") !== "0";
if (armDiagnostics) {
  await context.addInitScript(() => {
    try { window.localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {}
  });
}
const page = await context.newPage();
const t0 = performance.now();
page.on("console", (msg) => {
  const t = performance.now() - t0;
  const text = msg.text();
  lines.push(`${t.toFixed(2)} ${msg.type()} ${text.slice(0, 4000)}`);
  if (text.includes("[DEBUG]")) debugLines.push({ tMs: Number(t.toFixed(2)), text: text.slice(0, 4000) });
});
page.on("pageerror", (err) => lines.push(`${(performance.now() - t0).toFixed(2)} pageerror ${String(err).slice(0, 2000)}`));

await page.goto(url, { waitUntil: "domcontentloaded" });

// ⏱️ Instrumentation-free convergence clock: polls the rendered surfaces' own attributes rather than
// the console, so the SAME number is comparable across runs with and without diagnostics armed.
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
      const parsed = JSON.parse(main.status);
      const nodes = Object.values(parsed);
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

// 🧮️ Derived per-[DEBUG]-line deltas (ms since the previous [DEBUG] line), the raw material for a
// per-turn breakdown table without hand-diffing the console dump.
const deltas = debugLines.map((row, i) => ({ ...row, deltaMs: i === 0 ? null : Number((row.tMs - debugLines[i - 1].tMs).toFixed(2)) }));

await page.screenshot({ path: join(outDir, "final.png"), type: "png" });
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "debug-deltas.jsonl"), deltas.map((d) => JSON.stringify(d)).join("\n"));
writeFileSync(join(outDir, "hosts.json"), JSON.stringify(hosts, null, 2));
writeFileSync(join(outDir, "convergence.json"), JSON.stringify({ url, seconds, armDiagnostics, allOkMs, meshesMs, meshesSeen, consoleLines: lines.length, debugLines: debugLines.length }, null, 2));
console.log("DONE lines", lines.length, "debugLines", debugLines.length, "allOkMs", allOkMs, "meshesMs", meshesMs, "meshesSeen", meshesSeen, "hosts", JSON.stringify(hosts).slice(0, 600));
await browser.close();
