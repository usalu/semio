/** 🩺️ Wgpu boot probe with RUNTIME DIAGNOSTICS ARMED plus the structural oracle.
 *
 * The plain boot probe (`🐍️wgpu-console-dump-probe.mjs`) reads the console a USER would see, which is
 * exactly what packet W3a gated the per-frame dumps out of. This one arms
 * `SEMIO_RUNTIME_DIAGNOSTICS` in `localStorage` before the page loads — the trunk door stamps the frame
 * Worker's url from it, and the Worker hands it to the renderer wasm — so the per-frame census comes
 * back when it is asked for, and then dumps `semioWgpuIntrospection.dumpStructure/dumpFrameStats/dumpMeshStats`.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6213/?plugin=puzzle3d SEMIO_PROBE_OUT=w3a-diag bun 🐍️w3a-wgpu-diagnostics-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6213/?plugin=puzzle3d";
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 40);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "w3a-diag");
mkdirSync(outDir, { recursive: true });
const lines = [];
const browser = await chromium.launch({ headless: process.env.SEMIO_PROBE_HEADED !== "1", args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, deviceScaleFactor: Number(process.env.SEMIO_PROBE_DPR ?? 1) });
const t0 = Date.now();
const record = (source, type, text) => lines.push(`${Date.now() - t0} ${source} ${type} ${text.slice(0, 6000)}`);
page.on("console", (msg) => record("page", msg.type(), msg.text()));
page.on("pageerror", (error) => record("page", "pageerror", String(error)));
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
await page.waitForTimeout(seconds * 1000);
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
  return { available: true, structure: await read("dumpStructure"), frameStats: await read("dumpFrameStats"), meshStats: await read("dumpMeshStats") };
});
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "dumps.json"), JSON.stringify(dumps, null, 2));
console.log("[DEBUG] DONE lines", lines.length, "introspection", dumps.available, String(dumps.frameStats ?? "").slice(0, 400));
await browser.close();
