/** 🧪️ W14b — generation3d wgpu probe: waits for the introspection beacon (boot is ~70 s under peer
 * load), then dumps structure / frame-stats / mesh-stats / chrome and screenshots.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6118/?plugin=generation3d SEMIO_PROBE_OUT=w14b-x bun 🐍️w14b-gen3d-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6118/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "w14b-probe");
const bootTimeout = Number(process.env.SEMIO_PROBE_BOOT_MS ?? 240000);
const settle = Number(process.env.SEMIO_PROBE_SETTLE_MS ?? 15000);
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
let ready = false;
try {
  await page.waitForFunction(() => typeof globalThis.semioWgpuIntrospection?.dumpStructure === "function", { timeout: bootTimeout });
  ready = true;
} catch (error) {
  record("probe", "warn", `introspection never appeared: ${error}`);
}
record("probe", "info", `introspection ready=${ready} at ${Date.now() - t0} ms`);
await page.waitForTimeout(settle);
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
  return { available: true, structure: await read("dumpStructure"), frameStats: await read("dumpFrameStats"), meshStats: await read("dumpMeshStats"), chrome: await read("dumpChrome"), accessibility: await read("dumpAccessibility") };
});
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "dumps.json"), JSON.stringify(dumps, null, 2));
const errors = lines.filter((line) => line.includes("pageerror") || /worker-[a-z]+-failed|terminal fault|paint fault/.test(line));
writeFileSync(join(outDir, "errors.txt"), errors.join("\n"));
console.log("[DEBUG] DONE lines", lines.length, "introspection", dumps.available, "errors", errors.length);
console.log("[DEBUG] frameStats", String(dumps.frameStats ?? "").slice(0, 600));
await browser.close();
