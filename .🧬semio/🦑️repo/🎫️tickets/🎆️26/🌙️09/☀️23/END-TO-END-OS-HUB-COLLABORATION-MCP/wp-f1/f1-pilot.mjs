#!/usr/bin/env bun
/** 🧪️ F1 pilot — boots `s`, leaves Home idle, prints every traced thread (names, busy, counts) to learn the trace shape.
 * usage: bun f1-pilot.mjs <baseUrl> [seconds] */
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { launch, metrics, profileWindow, sleep, summarize, traceWindow } from "./f1-lib.mjs";

const [baseUrl = "http://127.0.0.1:6620/", seconds = "10"] = process.argv.slice(2);
const sweep = await import("/Users/ueli/Documents/semio/.tmp-ticket-0918/🐍️s6-all-kinds-sweep.mjs");
const { browser, page, cdp, workers } = await launch();
const t0 = Date.now();
await page.goto(baseUrl, { waitUntil: "commit" });
const beacon = await sweep.awaitBeacon(page, Date.now() + 300_000);
await sweep.dismissIntroduction(page);
const bootMs = Date.now() - t0;
await sleep(8_000);
const gl = await page.evaluate(() => {
  const gl = document.createElement("canvas").getContext("webgl");
  const info = gl?.getExtension("WEBGL_debug_renderer_info");
  return { webgl: info ? gl.getParameter(info.UNMASKED_RENDERER_WEBGL) : null, webgpu: typeof navigator.gpu, hidden: document.hidden, coi: self.crossOriginIsolated };
});
await page.evaluate(() => window.__f1.sites(true));
const before = await page.evaluate(() => window.__f1.read());
const m0 = await metrics(cdp);
const trace = await traceWindow(cdp, Number(seconds) * 1000);
const m1 = await metrics(cdp);
const after = await page.evaluate(() => window.__f1.read());
const sites = await page.evaluate(() => window.__f1.sites(false));
const heap = await workers.heap();
const profile = await profileWindow(cdp, 3_000);
const result = {
  beacon,
  bootMs,
  gl,
  summary: summarize(trace, Number(seconds)),
  metricsDelta: { TaskDuration: +(m1.TaskDuration - m0.TaskDuration).toFixed(3), ScriptDuration: +(m1.ScriptDuration - m0.ScriptDuration).toFixed(3), RecalcStyleDuration: +(m1.RecalcStyleDuration - m0.RecalcStyleDuration).toFixed(3), LayoutDuration: +(m1.LayoutDuration - m0.LayoutDuration).toFixed(3), RecalcStyleCount: m1.RecalcStyleCount - m0.RecalcStyleCount, LayoutCount: m1.LayoutCount - m0.LayoutCount },
  heap: { jsUsedMB: +(m1.JSHeapUsedSize / 2 ** 20).toFixed(1), jsTotalMB: +(m1.JSHeapTotalSize / 2 ** 20).toFixed(1), mainWasm: after, rafDuringWindow: after.rafCallbacks - before.rafCallbacks, workers: heap },
  sites,
  profile,
  threads: trace.threads,
};
writeFileSync(fileURLToPath(new URL(`./generated/f1-pilot-${Date.now()}.json`, import.meta.url)), JSON.stringify(result, null, 1));
console.log(JSON.stringify({ ...result, threads: trace.threads.slice(0, 25) }, null, 1));
await browser.close();
