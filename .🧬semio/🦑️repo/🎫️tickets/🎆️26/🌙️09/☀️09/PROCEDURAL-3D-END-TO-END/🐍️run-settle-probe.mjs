/** ⏯️ Focused probe for the ONE journey row that stalls with `computing: true` while its own chain
 * publishes `phase: idle, ratio: 1`: boot, re-pick the BOOT example (whose nodes are all cache hits,
 * so the chain settles in one hop), then sample the preview's published status every second for
 * `SEMIO_PROBE_SECONDS` while recording the whole console with guest `[DEBUG]` diagnostics armed.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6024/?plugin=generation3d \
 *   SEMIO_PROBE_PICK="Hexagonal Mushroom Column" SEMIO_PROBE_OUT=flow-inline/run-settle bun 🐍️run-settle-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6024/?plugin=generation3d";
const pickText = process.env.SEMIO_PROBE_PICK ?? "Hexagonal Mushroom Column";
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 45);
const bootWait = Number(process.env.SEMIO_PROBE_BOOT ?? 30);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "run-settle");
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 1500)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 1500)}`));
if (process.env.SEMIO_PROBE_GUEST_DIAGNOSTICS === "1") await page.addInitScript(() => { try { localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
await page.goto(url, { waitUntil: "domcontentloaded" });

const snap = () => page.evaluate(() => [...document.querySelectorAll("[data-status-json]")]
  .filter((el) => (el.getAttribute("data-surface-id") ?? "").endsWith("-preview"))
  .map((el) => { try { const s = JSON.parse(el.getAttribute("data-status-json") ?? "{}"); return { surfaceId: el.getAttribute("data-surface-id"), computing: s.computing ?? null, phase: s.phase ?? null, ratio: s.progress?.ratio ?? null, inFlight: s.progress?.inFlight ?? null, nodesDone: s.progress?.nodesDone ?? null, nodesTotal: s.progress?.nodesTotal ?? null, cancellable: s.cancellable ?? null }; } catch { return { surfaceId: el.getAttribute("data-surface-id"), parseError: true }; } }));

await page.waitForTimeout(bootWait * 1000);
lines.push(`${Date.now() - t0} probe BOOT-SETTLED ${JSON.stringify(await snap())}`);

try {
  const combo = page.locator('[role="combobox"]').first();
  await combo.click({ timeout: 20000 });
  await page.waitForTimeout(400);
  await page.locator('[role="option"]').filter({ hasText: pickText }).first().click({ timeout: 20000 });
  lines.push(`${Date.now() - t0} probe PICKED ${pickText}`);
} catch (error) {
  lines.push(`${Date.now() - t0} probe PICK-FAILED ${String(error).slice(0, 400)}`);
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
  await page.screenshot({ path: join(outDir, "pick-failed.png") });
  await browser.close();
  console.log("PICK FAILED — console written");
  process.exit(1);
}

const samples = [];
for (let i = 0; i < seconds; i += 1) {
  await page.waitForTimeout(1000);
  const row = { t: Date.now() - t0, hosts: await snap() };
  samples.push(row);
  console.log(`[DEBUG] +${(row.t / 1000).toFixed(0)}s ${JSON.stringify(row.hosts)}`);
}
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "samples.json"), JSON.stringify(samples, null, 2));
await page.screenshot({ path: join(outDir, "final.png") });
await browser.close();
console.log("DONE lines", lines.length, "samples", samples.length);
