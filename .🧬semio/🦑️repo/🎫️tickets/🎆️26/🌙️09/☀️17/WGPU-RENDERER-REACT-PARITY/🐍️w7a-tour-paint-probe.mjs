/** 🎓️ W7a — does the wgpu boot tour ever PAINT? Samples `dumpChrome` + a screenshot at a ladder of
 * instants from first paint onward, so a tour that paints-then-vanishes is told apart from one that
 * registers hits it never draws. Read-only: clicks nothing, rebuilds nothing.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6213/?plugin=puzzle3d bun 🐍️w7a-tour-paint-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6213/?plugin=puzzle3d";
const marks = (process.env.SEMIO_PROBE_MARKS ?? "4,8,12,18,25,35,50").split(",").map(Number);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "w7a-tour-paint");
mkdirSync(outDir, { recursive: true });

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, deviceScaleFactor: 1 });
await page.addInitScript(() => { try { globalThis.localStorage?.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
const lines = [];
page.on("console", (msg) => lines.push(`${msg.type()} ${msg.text()}`));
page.on("pageerror", (err) => lines.push(`pageerror ${String(err)}`));
await page.goto(url, { waitUntil: "domcontentloaded" });

const readChrome = () => page.evaluate(async () => {
  const introspection = globalThis.semioWgpuIntrospection;
  if (!introspection) return { available: false };
  try {
    const dump = JSON.parse(await introspection.dumpChrome());
    return { available: true, armed: dump.armed, generation: dump.generation, hits: dump.hits.length, tour: dump.hits.filter((hit) => /tour|introduction/.test(hit.controlId ?? "")).map((hit) => [hit.controlId, hit.rect]), actions: (dump.actions ?? []).slice(-12).map((entry) => entry.action) };
  } catch (error) {
    return { available: true, error: String(error) };
  }
});

const samples = [];
let previous = 0;
for (const mark of marks) {
  await page.waitForTimeout(Math.max(0, mark - previous) * 1000);
  previous = mark;
  const chrome = await readChrome();
  await page.screenshot({ path: join(outDir, `t${String(mark).padStart(3, "0")}.png`), type: "png" });
  samples.push({ mark, chrome });
  console.log(`[DEBUG] t=${mark}s armed=${chrome.armed} gen=${chrome.generation} hits=${chrome.hits} tour=${JSON.stringify(chrome.tour)}`);
}
writeFileSync(join(outDir, "samples.json"), JSON.stringify(samples, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
await browser.close();
