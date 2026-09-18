/** 🩺️ Wgpu chrome hit-registry dump for the visual-parity audit (📓️audit-visual-parity-puzzle3d.md).
 * Arms SEMIO_RUNTIME_DIAGNOSTICS (required for the ledger to record), lets the shell settle, then reads
 * `window.semioWgpuIntrospection.dumpChrome()` — every published chrome hit target (navbar chips, footer
 * chips, window caps, pane-row chips, panel toggles) with its rect [x,y,w,h], control_id, kind and action.
 * Also clicks the Artifact/Inspection navbar chips by SCREEN COORDINATE (a GPU canvas has no DOM to
 * click through) and re-dumps so panel content hits are captured too.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6213/?plugin=puzzle3d bun 🐍️wgpu-chrome-hits-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6213/?plugin=puzzle3d";
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 20);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "audit-visual/wgpu-chrome");
mkdirSync(outDir, { recursive: true });

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, deviceScaleFactor: Number(process.env.SEMIO_PROBE_DPR ?? 1) });
await page.addInitScript(() => { try { globalThis.localStorage?.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
const lines = [];
page.on("console", (msg) => lines.push(`${msg.type()} ${msg.text()}`));
page.on("pageerror", (err) => lines.push(`pageerror ${String(err)}`));
await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(seconds * 1000);

const readChrome = () => page.evaluate(async () => {
  const introspection = globalThis.semioWgpuIntrospection;
  if (!introspection) return { available: false };
  try {
    return { available: true, dump: JSON.parse(await introspection.dumpChrome()) };
  } catch (error) {
    return { available: true, error: String(error) };
  }
});

const before = await readChrome();
await page.screenshot({ path: join(outDir, "before.png"), type: "png" });

// Click a screen coordinate (navbar chips are along the top-left/top-right, y~14 per the wgpu-boot
// screenshot geometry) then re-dump. Coordinates are refined from the audit's own screenshot pixel-checks.
const clicks = { artifact: [37, 14], inspection: [1200, 14] };
const clickResults = {};
for (const [label, [x, y]] of Object.entries(clicks)) {
  await page.mouse.click(x, y);
  await page.waitForTimeout(1000);
  clickResults[label] = await readChrome();
  await page.screenshot({ path: join(outDir, `after-${label}.png`), type: "png" });
}

writeFileSync(join(outDir, "chrome-dump.json"), JSON.stringify({ before, clickResults }, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] DONE before.hits=", before?.dump?.hits?.length, "armed=", before?.dump?.armed, "generation=", before?.dump?.generation);
await browser.close();
