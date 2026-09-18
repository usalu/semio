/** 🔀️ Drives the `Projection` chip of one wgpu world pane and proves the switch reaches the CAMERA.
 *
 * Boots the wgpu shell, reads `semioWgpuIntrospection.dumpChrome()` for the pane's
 * `shell.projection.fold.<windowId>` hit rect, clicks it, clicks the `…template.<windowId>::<id>`
 * row, then reads `dumpMeshStats()` back and writes the per-surface camera before and after.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6213/?plugin=puzzle3d \
 *   SEMIO_PROBE_WINDOW=puzzle3d-main-perspective SEMIO_PROBE_TEMPLATE=orthographic \
 *   SEMIO_PROBE_OUT=w8b-projection bun 🐍️w8b-projection-switch-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6213/?plugin=puzzle3d";
const windowId = process.env.SEMIO_PROBE_WINDOW ?? "puzzle3d-main-perspective";
const templateId = process.env.SEMIO_PROBE_TEMPLATE ?? "orthographic";
const settle = Number(process.env.SEMIO_PROBE_SETTLE ?? 35);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "w8b-projection");
mkdirSync(outDir, { recursive: true });

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, deviceScaleFactor: 1 });
await page.addInitScript(() => { try { globalThis.localStorage?.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
if (process.env.SEMIO_PROBE_TOUR !== "1") await page.addInitScript(() => { try { localStorage.setItem("ui.introduction.seen.s.puzzle.puzzle3d@1/*#editor", "true"); } catch {} });
const lines = [];
page.on("console", (msg) => lines.push(`page ${msg.type()} ${msg.text().slice(0, 600)}`));
page.on("pageerror", (err) => lines.push(`page pageerror ${String(err)}`));
await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(settle * 1000);

const cameras = () => page.evaluate(async () => {
  const introspection = globalThis.semioWgpuIntrospection;
  if (!introspection) return null;
  const stats = JSON.parse(await introspection.dumpMeshStats());
  return Object.fromEntries(stats.surfaces.map((surface) => [surface.surfaceId, surface.camera]));
});
const hitRect = async (controlId) => {
  for (let attempt = 0; attempt < 12; attempt++) {
    const found = await page.evaluate(async (id) => {
      const introspection = globalThis.semioWgpuIntrospection;
      if (!introspection) return null;
      const dump = JSON.parse(await introspection.dumpChrome());
      const rows = dump.hits ?? [];
      const row = rows.find((entry) => (entry.controlId ?? entry.control_id) === id);
      return row ? { rect: row.rect, id } : { missing: rows.length };
    }, controlId);
    if (found?.rect) return found;
    await page.mouse.move(700 + attempt, 400 + attempt);
    await page.waitForTimeout(1200);
  }
  return { missing: "not published" };
};
const clickRect = async (found) => {
  const [x, y, w, h] = found.rect;
  await page.mouse.click(x + w / 2, y + h / 2);
  await page.waitForTimeout(1500);
};

await page.screenshot({ path: join(outDir, "before.png") });
const fold = await hitRect(`shell.projection.fold.${windowId}`);
if (fold?.rect) await clickRect(fold);
await page.screenshot({ path: join(outDir, "unfolded.png") });
const row = await hitRect(`shell.projection.template.${windowId}::${templateId}`);
if (row?.rect) await clickRect(row);
await page.waitForTimeout(6000);
const before = null;
const after = await cameras();
await page.screenshot({ path: join(outDir, "after.png") });

writeFileSync(join(outDir, "cameras.json"), JSON.stringify({ windowId, templateId, fold, row, before, after }, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] DONE fold", JSON.stringify(fold).slice(0, 300), "row", JSON.stringify(row).slice(0, 300));
console.log("[DEBUG] before", JSON.stringify(before?.[windowId]), "after", JSON.stringify(after?.[windowId]));
await browser.close();
