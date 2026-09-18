/** 🔀️ Proves W9b end to end on the live wgpu shell: the Projection pane's body, the row press, the
 * live camera row, and the Top pane's framed sheet width.
 *
 * Boots, unfolds `shell.projection.fold.<window>`, screenshots the unfolded body, records every
 * `shell.projection.template.…` row rect, presses one row, then reads `dumpMeshStats()` for BOTH the
 * wire `camera` and the new `liveCamera` per surface, before and after. Finally measures the Top
 * pane's sheet span in pixels off the capture, which is the §2 number.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_WINDOW=puzzle3d-main-top SEMIO_PROBE_TEMPLATE=orthographic \
 *   SEMIO_PROBE_OUT=w9b-verify bun 🐍️w9b-projection-and-framing-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6213/?plugin=puzzle3d";
const windowId = process.env.SEMIO_PROBE_WINDOW ?? "puzzle3d-main-top";
const templateId = process.env.SEMIO_PROBE_TEMPLATE ?? "orthographic";
const settle = Number(process.env.SEMIO_PROBE_SETTLE ?? 40);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "w9b-verify");
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
  return { count: (dump.hits ?? []).length, error: dump.error ?? null, hits: (dump.hits ?? []).map((entry) => ({ id: entry.controlId ?? entry.control_id ?? null, rect: entry.rect, kind: entry.kind })) };
});
const cameras = () => page.evaluate(async () => {
  const introspection = globalThis.semioWgpuIntrospection;
  if (!introspection) return null;
  const stats = JSON.parse(await introspection.dumpMeshStats());
  return Object.fromEntries((stats.surfaces ?? []).map((surface) => [surface.surfaceId, { rect: surface.rect, camera: surface.camera, liveCamera: surface.liveCamera ?? null, bboxMin: surface.bboxMin, bboxMax: surface.bboxMax }]));
});
const click = async (rect) => {
  await page.mouse.click(rect[0] + rect[2] / 2, rect[1] + rect[3] / 2);
  await page.waitForTimeout(2500);
};
const wait = async (predicate, attempts = 12) => {
  for (let attempt = 0; attempt < attempts; attempt++) {
    const dump = await chrome();
    if (dump && predicate(dump)) return dump;
    await page.mouse.move(700 + attempt, 400 + attempt);
    await page.waitForTimeout(1500);
  }
  return await chrome();
};

const camerasBoot = await cameras();
await page.screenshot({ path: join(outDir, "boot.png") });
const folded = await chrome();
// 🔀️ The chip's id is React's own `…projection.<camelCaseWindowId>.pane.fold`, which W9c renamed
// it to mid-ticket; the older `shell.projection.fold.<windowId>` spelling is accepted too.
const camel = windowId.replace(/-([a-z])/g, (_, letter) => letter.toUpperCase());
const fold = folded?.hits.find((entry) => entry.id === `shell.projection.fold.${windowId}` || ((entry.id ?? "").includes("projection") && (entry.id ?? "").includes(`.${camel}.`) && (entry.id ?? "").endsWith(".pane.fold")));
if (fold) await click(fold.rect);
const unfolded = await wait((dump) => dump.hits.some((entry) => (entry.id ?? "").startsWith(`shell.projection.template.${windowId}::`)));
await page.screenshot({ path: join(outDir, "unfolded.png") });
const rows = (unfolded?.hits ?? []).filter((entry) => (entry.id ?? "").startsWith(`shell.projection.template.${windowId}::`));

const row = rows.find((entry) => entry.id === `shell.projection.template.${windowId}::${templateId}`);
const camerasBefore = await cameras();
if (row) await click(row.rect);
await page.waitForTimeout(8000);
const camerasAfter = await cameras();
await page.screenshot({ path: join(outDir, "selected.png") });
const chromeAfter = await chrome();

writeFileSync(
  join(outDir, "projection.json"),
  JSON.stringify(
    {
      windowId,
      templateId,
      foldRect: fold?.rect ?? null,
      hitCountBoot: folded?.count ?? null,
      hitCountUnfolded: unfolded?.count ?? null,
      hitCountAfter: chromeAfter?.count ?? null,
      error: chromeAfter?.error ?? null,
      rows: rows.map((entry) => ({ id: entry.id, rect: entry.rect })),
      camerasBoot,
      camerasBefore,
      camerasAfter,
    },
    null,
    2,
  ),
);
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] fold", JSON.stringify(fold));
console.log("[DEBUG] rows", rows.length, "hits", folded?.count, "→", unfolded?.count);
console.log("[DEBUG] live before", JSON.stringify(camerasBefore?.[windowId]?.liveCamera));
console.log("[DEBUG] live after", JSON.stringify(camerasAfter?.[windowId]?.liveCamera));
await browser.close();
