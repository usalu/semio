/** 🧊️ W7a — is the wgpu canvas still PRESENTING? Boots, waits, screenshots, then hovers a navbar chip
 * and a scene point and screenshots again. Three byte-identical frames mean the presentation lane has
 * stopped: the model keeps changing (`dumpChrome`'s generation climbs) while no new frame reaches the
 * swapchain. Read-only — it clicks nothing and rebuilds nothing.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6213/?plugin=puzzle3d bun 🐍️w7a-canvas-liveness-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6213/?plugin=puzzle3d";
const settle = Number(process.env.SEMIO_PROBE_BOOT ?? 22) * 1000;
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "w7a-live");
mkdirSync(outDir, { recursive: true });

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, deviceScaleFactor: 1 });
await page.addInitScript(() => { try { globalThis.localStorage?.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(settle);
await page.screenshot({ path: join(outDir, "a-idle.png") });
await page.mouse.move(1200, 14);
await page.waitForTimeout(2500);
await page.screenshot({ path: join(outDir, "b-hover-inspection.png") });
await page.mouse.move(700, 450);
await page.waitForTimeout(2500);
await page.screenshot({ path: join(outDir, "c-hover-scene.png") });
console.log(`[DEBUG] DONE — compare the three PNGs under ${outDir}; identical frames mean a frozen swapchain`);
await browser.close();
