import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { PNG } from "pngjs";
import { readFileSync } from "node:fs";

const backend = process.env.BACKEND ?? "metal";
const out = join(import.meta.dir, "🗑️generated", `wgpu-backend-${backend}`);
mkdirSync(out, { recursive: true });
const args = ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", ...(backend === "swiftshader" ? ["--use-angle=swiftshader", "--use-webgpu-adapter=swiftshader", "--enable-unsafe-swiftshader"] : ["--use-angle=metal"])];
const browser = await chromium.launch({ headless: true, args });
const page = await browser.newPage({ viewport: { width: 1280, height: 800 } });
if (process.env.TRACE_SCOPE === "1" || process.env.WASM_OVERRIDE) await page.route("**/*", async route => {
  const url = decodeURIComponent(route.request().url());
  if (process.env.WASM_OVERRIDE && url.includes("/renderer-modules/wgpu/semio-renderer-wgpu-component_bg.wasm")) return route.fulfill({ path: process.env.WASM_OVERRIDE, contentType: "application/wasm" });
  if (process.env.TRACE_SCOPE !== "1" || !url.includes("🎞️frame-worker.js/🟨️.js")) return route.continue();
  const response = await route.fetch();
  const body = (await response.text()).replace("const tick = await performInvocation(requireChannel(instanceId), instanceId, addressed.invocation, slimView);", "console.log('[DEBUG] scope-probe', JSON.stringify({dispatch, addressed, slimView})); const tick = await performInvocation(requireChannel(instanceId), instanceId, addressed.invocation, slimView);");
  return route.fulfill({ response, body });
});
const lines = [];
const start = Date.now();
page.on("console", m => lines.push(`${Date.now()-start} ${m.type()} ${m.text()}`));
page.on("pageerror", e => lines.push(`${Date.now()-start} pageerror ${e.message}`));
page.on("requestfailed", r => lines.push(`${Date.now()-start} requestfailed ${r.url()} ${r.failure()?.errorText}`));
await page.goto("http://127.0.0.1:6118/?plugin=generation3d&mode=generate&example=hexagonal-mushroom-column", { waitUntil: "domcontentloaded" });
const adapter = await page.evaluate(async () => { const a = await navigator.gpu?.requestAdapter(); return a ? { info: a.info, fallback: a.isFallbackAdapter } : null; });
let firstPaintMs = null;
for (let i = 0; i < Number(process.env.SMOKE_SECONDS ?? 90) * 2; i++) {
  if (i < Number(process.env.PUMP_SECONDS ?? 90) * 2 && i % Number(process.env.PUMP_INTERVAL_TICKS ?? 1) === 0) await page.mouse.move(3 + Math.floor(i / Number(process.env.PUMP_INTERVAL_TICKS ?? 1)) % 2, 3);
  await page.waitForTimeout(500);
  if (firstPaintMs === null && i % 10 === 9 && lines.some(line => line.includes("wgpu-worker boot_shell leave"))) {
    const pixels = PNG.sync.read(await page.screenshot()).data;
    for (let p = 4; p < pixels.length; p += 4) if (pixels[p] !== pixels[0] || pixels[p+1] !== pixels[1] || pixels[p+2] !== pixels[2]) { firstPaintMs = Date.now()-start; break; }
  }
}
await page.screenshot({ path: join(out, "shot.png") });
writeFileSync(join(out, "console.txt"), lines.join("\n"));
const png = PNG.sync.read(readFileSync(join(out, "shot.png")));
const colors = new Set();
for (let i = 0; i < png.data.length; i += 4) colors.add(`${png.data[i]},${png.data[i+1]},${png.data[i+2]}`);
const faults = lines.filter(line => /panicked|pageerror|surface fault|unmapped effect "publish-event"/i.test(line) || (line.includes("requestfailed") && !line.includes("net::ERR_ABORTED")));
const booted = lines.some(line => line.includes("wgpu-worker boot_shell leave"));
console.log(JSON.stringify({ backend, adapter, seconds: (Date.now()-start)/1000, firstPaintMs, lines: lines.length, booted, colors: colors.size, faults }));
await browser.close();
if (!booted || faults.length || colors.size < 2) process.exitCode = 1;
