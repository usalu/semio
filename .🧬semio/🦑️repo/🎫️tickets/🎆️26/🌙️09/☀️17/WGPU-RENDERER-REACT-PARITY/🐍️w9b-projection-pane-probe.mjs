/** 🔀️ Unfolds one wgpu world pane's `Projection` chip and reports EVERY chrome row it publishes.
 *
 * Where `🐍️w8b-projection-switch-probe.mjs` asks for one row id and gives up, this dumps the whole
 * hit ledger before and after the fold press, the shell's own error, and the ids that changed — the
 * evidence P1 needs to tell "the rows never painted" from "the ledger never republished".
 *
 * Usage: cd <ticket> && SEMIO_PROBE_WINDOW=puzzle3d-main-perspective SEMIO_PROBE_OUT=w9b-projection \
 *   bun 🐍️w9b-projection-pane-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6213/?plugin=puzzle3d";
const windowId = process.env.SEMIO_PROBE_WINDOW ?? "puzzle3d-main-perspective";
const settle = Number(process.env.SEMIO_PROBE_SETTLE ?? 35);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "w9b-projection");
mkdirSync(outDir, { recursive: true });

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, deviceScaleFactor: 1 });
await page.addInitScript(() => { try { globalThis.localStorage?.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
await page.addInitScript(() => { try { localStorage.setItem("ui.introduction.seen.s.puzzle.puzzle3d@1/*#editor", "true"); } catch {} });
const lines = [];
page.on("console", (msg) => lines.push(`page ${msg.type()} ${msg.text().slice(0, 900)}`));
page.on("pageerror", (err) => lines.push(`page pageerror ${String(err)}`));
await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(settle * 1000);

const chrome = () => page.evaluate(async () => {
  const introspection = globalThis.semioWgpuIntrospection;
  if (!introspection) return null;
  const dump = JSON.parse(await introspection.dumpChrome());
  return { keys: Object.keys(dump), count: (dump.hits ?? []).length, error: dump.error ?? null, hits: (dump.hits ?? []).map((entry) => ({ id: entry.controlId ?? entry.control_id ?? null, rect: entry.rect, kind: entry.kind })) };
});

const before = await chrome();
// 🆔️ React's own id for the pane's fold chip — `world3dProjectionPaneElementId(segment)` plus `Pane`'s
// default `chromeToggleId`. It was `shell.projection.fold.<windowId>` until W9c converged the five pane
// chips on React's spelling (📓️w9c-behaviour-parity-run-2.md §4); the segment is `elementIdSegment`.
const segment = windowId.replace(/[-_. ]+(.)/g, (_, character) => character.toUpperCase());
const fold = before?.hits.find((entry) => entry.id === `framework.worldOrbit.projection.${segment}.pane.fold`);
await page.screenshot({ path: join(outDir, "folded.png") });
if (fold) {
  const [x, y, w, h] = fold.rect;
  await page.mouse.click(x + w / 2, y + h / 2);
}
const samples = [];
for (let attempt = 0; attempt < 10; attempt++) {
  await page.waitForTimeout(1500);
  const dump = await chrome();
  samples.push({ attempt, count: dump?.count, error: dump?.error, projection: dump?.hits.filter((entry) => (entry.id ?? "").includes("projection")).map((entry) => entry.id) });
  await page.mouse.move(700 + attempt, 400 + attempt);
}
await page.screenshot({ path: join(outDir, "unfolded.png") });
const after = await chrome();
writeFileSync(join(outDir, "chrome.json"), JSON.stringify({ windowId, fold, beforeCount: before?.count, beforeKeys: before?.keys, samples, afterCount: after?.count, afterProjection: after?.hits.filter((entry) => (entry.id ?? "").includes("projection")) }, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] fold", JSON.stringify(fold));
console.log("[DEBUG] samples", JSON.stringify(samples));
await browser.close();
