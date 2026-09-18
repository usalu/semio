/** 🩺️ Wgpu accessibility-projection dump for the visual-parity audit (📓️audit-visual-parity-puzzle3d.md).
 * `window.semioWgpuIntrospection.dumpAccessibility(windowId?)` returns AccessibilityProjectionNode[]
 * (role/label/rect/depth/...) — the chrome-geometry equivalent of the React DOM dump for a canvas
 * target, since a GPU surface has no elements to getBoundingClientRect().
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6213/?plugin=puzzle3d bun 🐍️wgpu-accessibility-dump-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6213/?plugin=puzzle3d";
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 25);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "audit-visual/wgpu-a11y");
mkdirSync(outDir, { recursive: true });

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, deviceScaleFactor: Number(process.env.SEMIO_PROBE_DPR ?? 1) });
const lines = [];
page.on("console", (msg) => lines.push(`${msg.type()} ${msg.text()}`));
page.on("pageerror", (err) => lines.push(`pageerror ${String(err)}`));
await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(seconds * 1000);
await page.screenshot({ path: join(outDir, "final.png"), type: "png" });

const result = await page.evaluate(async () => {
  const introspection = globalThis.semioWgpuIntrospection;
  if (!introspection) return { available: false };
  const readWindow = async (windowId) => {
    try {
      const raw = await introspection.dumpAccessibility(windowId);
      return JSON.parse(raw);
    } catch (error) {
      return { error: String(error) };
    }
  };
  const structureRaw = await introspection.dumpStructure().catch((e) => `<failed: ${e}>`);
  let windowIds = [];
  try { windowIds = JSON.parse(structureRaw).windowIds ?? []; } catch {}
  const perWindow = {};
  for (const id of windowIds) perWindow[id] = await readWindow(id);
  const noArg = await readWindow(undefined);
  return { available: true, windowIds, noArg, perWindow };
});

writeFileSync(join(outDir, "accessibility.json"), JSON.stringify(result, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] DONE available", result.available, "windowIds", JSON.stringify(result.windowIds), "noArgLen", Array.isArray(result.noArg) ? result.noArg.length : typeof result.noArg);
await browser.close();
