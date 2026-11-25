import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

/** 🧭️ Capped probe: samples the host's crossing census (worker round trips by class, with effects returned) around the on-demand activation to prove whether any guest turn after the contributions install returns an effect and whether turns keep arriving. One browser, hard cap. */
const out = join(import.meta.dir, "🗑️generated/preview-stall"); mkdirSync(out, { recursive: true });
const url = process.env.SEMIO_BATTERY_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const tag = process.env.PROBE_TAG ?? "c";
const browser = await chromium.launch({ headless: true, args: ["--js-flags=--max-old-space-size=1024", "--renderer-process-limit=1", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const cap = setTimeout(() => { browser.close().finally(() => process.exit(3)); }, 280000);
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
const lines = []; const t0 = Date.now(); const stamp = () => `+${Date.now() - t0}ms`;
const state = () => page.evaluate(() => { const g = (id) => document.querySelector(`[data-surface-id="${id}"]`); const p = JSON.parse(g("window:procedural-preview")?.getAttribute("data-status-json") ?? "{}"); return `${p.phase} units=${p.progress?.unitsDone}/${p.progress?.unitsTotal}`; });
const census = () => page.evaluate(() => JSON.stringify(Object.fromEntries(Object.entries(globalThis.__semioCrossingCensus ?? {}).map(([k, v]) => [k, `${v.crossings}c/${v.events}e/${v.patches}p/${v.effects}f`]))));
await page.goto(url, { waitUntil: "load" });
await page.waitForTimeout(25000);
lines.push(`${stamp()} PRE-ASK ${await state()} ${await census()}`);
for (const k of ["brep.solid.extrude", "math.vector"]) lines.push(`${stamp()} ASK ${k} ${await page.evaluate((n) => window.semioReactActivateOnDemand?.(JSON.stringify({ kind: "node-kind", nodeKind: n })), k)}`);
for (let t = 0; t < 140; t += 10) { lines.push(`${stamp()} ${await state()} ${await census()}`); await page.waitForTimeout(10000); }
writeFileSync(join(out, `${tag}-census.txt`), lines.join("\n"));
console.log(lines.join("\n"));
clearTimeout(cap); await browser.close();
