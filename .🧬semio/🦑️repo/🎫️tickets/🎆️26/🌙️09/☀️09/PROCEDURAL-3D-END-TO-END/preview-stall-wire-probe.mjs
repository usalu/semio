import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

/** 🧭️ Capped probe: scans every worker message (both directions) for the wire names a stalled preview should carry, to prove which hop never crosses. One browser, hard cap. */
const out = join(import.meta.dir, "🗑️generated/preview-stall"); mkdirSync(out, { recursive: true });
const url = process.env.SEMIO_BATTERY_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const tag = process.env.PROBE_TAG ?? "w";
const browser = await chromium.launch({ headless: true, args: ["--js-flags=--max-old-space-size=1024", "--renderer-process-limit=1", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const cap = setTimeout(() => { browser.close().finally(() => process.exit(3)); }, 280000);
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
const lines = []; const t0 = Date.now(); const stamp = () => `+${Date.now() - t0}ms`;
await page.addInitScript(() => {
  const NAMES = ["os.on-demand.request", "toolRunStart", "flowEvalTick", "invokeExtension", "flowEvalResolve", "setContributions", "toolRunFinalize", "requestedEffects", "refreshUi"];
  window.__wire = []; window.__wireCount = {};
  const scan = (dir, data) => {
    try {
      let text = null;
      if (typeof data === "string") text = data;
      else if (data instanceof ArrayBuffer) text = new TextDecoder("latin1").decode(new Uint8Array(data));
      else if (ArrayBuffer.isView(data)) text = new TextDecoder("latin1").decode(new Uint8Array(data.buffer, data.byteOffset, data.byteLength));
      else if (data && typeof data === "object") { const parts = []; for (const v of Object.values(data)) { if (typeof v === "string") parts.push(v); else if (v instanceof ArrayBuffer) parts.push(new TextDecoder("latin1").decode(new Uint8Array(v))); else if (ArrayBuffer.isView(v)) parts.push(new TextDecoder("latin1").decode(new Uint8Array(v.buffer, v.byteOffset, v.byteLength))); } text = parts.join("\u0000"); }
      if (!text) return;
      for (const n of NAMES) if (text.includes(n)) { const k = `${dir} ${n}`; window.__wireCount[k] = (window.__wireCount[k] ?? 0) + 1; if (window.__wire.length < 400) window.__wire.push(`${Math.round(performance.now())} ${k}`); }
    } catch {}
  };
  const W = window.Worker;
  window.Worker = class extends W {
    constructor(...a) { super(...a); this.addEventListener("message", (e) => scan("w2m", e.data)); const p = this.postMessage.bind(this); this.postMessage = (m, t) => { scan("m2w", m); return p(m, t); }; }
  };
});
const state = () => page.evaluate(() => { const g = (id) => document.querySelector(`[data-surface-id="${id}"]`); const p = JSON.parse(g("window:procedural-preview")?.getAttribute("data-status-json") ?? "{}"); return `${p.phase} ${p.fault?.code ?? ""} units=${p.progress?.unitsDone}/${p.progress?.unitsTotal}`; });
await page.goto(url, { waitUntil: "load" });
await page.waitForTimeout(25000);
lines.push(`${stamp()} PRE-ASK ${JSON.stringify(await page.evaluate(() => window.__wireCount))}`);
for (const k of ["brep.solid.extrude", "math.vector"]) lines.push(`${stamp()} ASK ${k} ${await page.evaluate((n) => window.semioReactActivateOnDemand?.(JSON.stringify({ kind: "node-kind", nodeKind: n })), k)}`);
for (let t = 0; t < 130; t += 10) { lines.push(`${stamp()} STATE ${await state()} ${JSON.stringify(await page.evaluate(() => window.__wireCount))}`); await page.waitForTimeout(10000); }
writeFileSync(join(out, `${tag}-wire.txt`), lines.join("\n") + "\n" + (await page.evaluate(() => window.__wire.join("\n"))));
console.log(lines.join("\n"));
clearTimeout(cap); await browser.close();
