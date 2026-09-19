/** 🩺️ W14a — gis2d on wgpu: is the frame EMPTY or is the worker STUCK?
 *
 * Arms SEMIO_RUNTIME_DIAGNOSTICS, boots the wgpu trunk, then polls
 * `semioWgpuIntrospection.dumpFrameStats/dumpStructure/dumpChrome` on a timer. A worker that
 * answers every poll is alive; a frame whose `drawCalls`/`quadCount` stay 0 is painting nothing.
 * Every poll is timed, so a poll that blocks is visible as a gap rather than as silence.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6140/?plugin=gis2d SEMIO_PROBE_OUT=w14a-x bun 🐍️w14a-gis2d-frame-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6140/?plugin=gis2d";
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 90);
const every = Number(process.env.SEMIO_PROBE_POLL_SECONDS ?? 10);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "w14a-gis2d");
mkdirSync(outDir, { recursive: true });
const lines = [];
const browser = await chromium.launch({ headless: process.env.SEMIO_PROBE_HEADED !== "1", args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, deviceScaleFactor: Number(process.env.SEMIO_PROBE_DPR ?? 1) });
const t0 = Date.now();
const record = (source, type, text) => lines.push(`${Date.now() - t0} ${source} ${type} ${text.slice(0, 6000)}`);
page.on("console", (msg) => record("page", msg.type(), msg.text()));
page.on("pageerror", (error) => record("page", "pageerror", String(error)));
page.on("requestfailed", (request) => record("page", "requestfailed", `${request.url()} ${request.failure()?.errorText ?? ""}`));
page.on("worker", (worker) => {
  record("page", "info", `worker created ${worker.url()}`);
  worker.on("console", (msg) => record("worker", msg.type(), msg.text()));
});
await page.addInitScript(() => {
  try {
    globalThis.localStorage?.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1");
  } catch {}
});
await page.goto(url, { waitUntil: "domcontentloaded" });

const polls = [];
const poll = async (label) => {
  const started = Date.now();
  const dump = await page.evaluate(async () => {
    const introspection = globalThis.semioWgpuIntrospection;
    if (!introspection) return { available: false };
    const read = async (name) => {
      const t = performance.now();
      try {
        const value = await introspection[name]();
        return { ms: Math.round(performance.now() - t), value };
      } catch (error) {
        return { ms: Math.round(performance.now() - t), error: String(error) };
      }
    };
    return {
      available: true,
      ready: document.body.dataset.semioOsReady ?? null,
      error: document.body.dataset.semioOsError ?? null,
      frameStats: await read("dumpFrameStats"),
      structure: await read("dumpStructure"),
      chrome: await read("dumpChrome"),
    };
  }).catch((error) => ({ available: false, evaluateError: String(error) }));
  polls.push({ label, atMs: started - t0, roundTripMs: Date.now() - started, dump });
  record("probe", "poll", `${label} roundTrip=${Date.now() - started}ms frame=${JSON.stringify(dump?.frameStats?.value ?? dump)?.slice(0, 400)}`);
  await page.screenshot({ path: join(outDir, `shot-${label}.png`), type: "png" });
};

for (let elapsed = every; elapsed <= seconds; elapsed += every) {
  await page.waitForTimeout(every * 1000);
  await poll(`${elapsed}s`);
}
await page.screenshot({ path: join(outDir, "final.png"), type: "png" });
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "polls.json"), JSON.stringify(polls, null, 2));
console.log("[DEBUG] DONE lines", lines.length, "polls", polls.length);
for (const entry of polls) console.log(entry.label, "roundTrip", entry.roundTripMs, "frame", JSON.stringify(entry.dump?.frameStats ?? entry.dump).slice(0, 300));
await browser.close();
