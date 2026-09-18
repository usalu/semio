/** 🥽️ Watches the wgpu boot for the World3d GLB lane: every `/mesh/…` request the page or its frame
 * Worker issues, with status and byte count, plus the worker console and the introspection dumps.
 *
 * The diagnostics probe answers what the PRODUCER published; this one answers whether the renderer
 * ever asked the network for a url-declared mesh, which is the question that separates "the bridge
 * dropped it" from "the fetch was reserved but nothing drained it".
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6213/?plugin=puzzle3d SEMIO_PROBE_OUT=w3d-fetch bun 🐍️w3d-wgpu-mesh-fetch-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6213/?plugin=puzzle3d";
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 60);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "w3d-fetch");
mkdirSync(outDir, { recursive: true });
const lines = [];
const requests = [];
const browser = await chromium.launch({ headless: process.env.SEMIO_PROBE_HEADED !== "1", args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, deviceScaleFactor: Number(process.env.SEMIO_PROBE_DPR ?? 1) });
const t0 = Date.now();
const record = (source, type, text) => lines.push(`${Date.now() - t0} ${source} ${type} ${text.slice(0, 4000)}`);
page.on("console", (msg) => record("page", msg.type(), msg.text()));
page.on("pageerror", (error) => record("page", "pageerror", String(error)));
page.on("worker", (worker) => worker.on("console", (msg) => record("worker", msg.type(), msg.text())));
page.on("request", (request) => {
  if (request.url().includes("/mesh/")) requests.push(`${Date.now() - t0} REQUEST ${request.url()}`);
});
// 🚫️ Never reads the body: `response.body()` on a stream the page is still consuming can cancel it,
// which shows up as a spurious `net::ERR_ABORTED` on the very fetch this probe exists to watch.
page.on("response", (response) => {
  if (!response.url().includes("/mesh/")) return;
  requests.push(`${Date.now() - t0} RESPONSE ${response.status()} ${response.headers()["content-length"] ?? "?"} ${response.url()}`);
});
page.on("requestfailed", (request) => requests.push(`${Date.now() - t0} FAILED ${request.failure()?.errorText} ${request.url()}`));
await page.addInitScript(() => {
  try {
    globalThis.localStorage?.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1");
  } catch {}
});
await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(seconds * 1000);
// 🖱️ SEMIO_PROBE_JIGGLE=1 keeps the event-driven shell ticking after it settles, which separates
// "the lane never asked" from "the lane asked and nothing drove the frames its decode needs".
if (process.env.SEMIO_PROBE_JIGGLE === "1") {
  for (let step = 0; step < Number(process.env.SEMIO_PROBE_JIGGLE_STEPS ?? 400); step++) {
    await page.mouse.move(700 + (step % 60), 400 + ((step * 7) % 60));
    await page.waitForTimeout(25);
  }
}
await page.screenshot({ path: join(outDir, "final.png"), type: "png" });
const dumps = await page.evaluate(async () => {
  const introspection = globalThis.semioWgpuIntrospection;
  if (!introspection) return { available: false };
  const read = async (name) => {
    try {
      return await introspection[name]();
    } catch (error) {
      return `<${name} failed: ${error}>`;
    }
  };
  return { available: true, frameStats: await read("dumpFrameStats"), meshStats: await read("dumpMeshStats") };
});
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "mesh-requests.txt"), requests.join("\n"));
writeFileSync(join(outDir, "dumps.json"), JSON.stringify(dumps, null, 2));
console.log(`[DEBUG] DONE lines ${lines.length} mesh-requests ${requests.length}`);
console.log(requests.join("\n"));
await browser.close();
