/** 🧹️ Does a selection made in ONE bundled example survive into the next one?
 *
 * The `interact` battery lane answers this too, but only through a pick on the 3d preview — which
 * needs the preview to have delivered geometry. While extension evaluation is down every example
 * publishes zero meshes, so that lane cannot pick at all and its `selection-reset` rows are green
 * for the wrong reason (nothing was ever selected). This probe picks in the NODE GRAPH instead,
 * which draws straight from the document and needs no evaluation, and reads the same two lanes:
 *
 *   `data-guest-selection-json` — what the GUEST published for the pane.
 *   `data-selection-json`       — what the pane PAINTS, after the host's leftover overlay.
 *
 * Splitting those two is the whole point: on 2026-09-14 the guest was already correct (`[]` after
 * every switch) while the host overlay kept the previous example's id.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6025/?plugin=generation3d SEMIO_PROBE_OUT=react-interact/prune bun 🐍️selection-prune-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6025/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "react-interact/prune");
const examples = (process.env.SEMIO_PROBE_EXAMPLES ?? "Box Shell Preview,Sphere Cut With Torus,Rectangle Extrude Volume,Box Fillet Preview").split(",").map((s) => s.trim()).filter(Boolean);
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const results = [];
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 900)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 900)}`));

/** 🔦️ Both selection lanes of the world pane, plus the Inspection panel's own id row. */
const snap = () =>
  page.evaluate(() => {
    const parse = (raw) => { try { return JSON.parse(raw ?? "null"); } catch { return null; } };
    // 🎯️ Keyed on the lane only the WORLD pane publishes: since 2026-09-15 the node-graph host publishes
    // its own `data-selection-json` too (the graph twin of this attribute), and it comes FIRST in document
    // order — so a bare `[data-selection-json]` now answers the Flow window, not the 3D pane this probe reads.
    const pane = document.querySelector("[data-guest-selection-json]");
    const combo = document.querySelector('[role="combobox"]');
    const idRow = [...document.querySelectorAll('[data-slot="panel"] [id*="procedural-play-inspector"]')].find((el) => el.id.endsWith("procedural-play-inspector.id")) ?? null;
    return {
      example: combo?.innerText?.replace(/\s+/g, " ").trim() ?? null,
      host: parse(pane?.getAttribute("data-selection-json")),
      guest: parse(pane?.getAttribute("data-guest-selection-json")),
      inspector: idRow ? (idRow.textContent ?? "").trim().slice(0, 80) : null,
    };
  });

const pick = async (label) => {
  await page.locator('[role="combobox"]').first().click({ timeout: 8000 });
  await page.waitForTimeout(500);
  await page.locator('[role="option"]').filter({ hasText: label }).first().click({ timeout: 8000 });
  await page.waitForTimeout(6000);
};

/** 🖱️ Sweep the node-graph canvas until a click lands on a node — the graph draws from the document
 * alone, so this pick works on an example whose preview never evaluated. */
const pickNode = async () => {
  const canvas = page.locator('[data-slot="node-graph"] canvas, .semio-node-graph canvas, canvas').first();
  const box = await canvas.boundingBox();
  if (!box) return { point: null, after: await snap() };
  const points = [];
  for (let row = 1; row <= 6; row += 1) for (let col = 1; col <= 8; col += 1) points.push([box.x + (box.width * col) / 9, box.y + (box.height * row) / 7]);
  for (const [x, y] of points) {
    await page.mouse.click(x, y);
    await page.waitForTimeout(700);
    const after = await snap();
    if ((after.guest?.selectedIds ?? []).length > 0 || (after.host?.selectedIds ?? []).length > 0) return { point: [Math.round(x), Math.round(y)], after };
  }
  return { point: null, after: await snap() };
};

await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(25_000);
await page.locator('[id="framework.panel.inspection"]').first().click({ timeout: 8000 }).catch(() => {});
await page.waitForTimeout(1500);

for (let index = 0; index < examples.length; index += 1) {
  const label = examples[index];
  await pick(label).catch((error) => lines.push(`pick ${label} failed ${String(error).slice(0, 200)}`));
  const afterSwitch = await snap();
  const carried = (afterSwitch.host?.selectedIds ?? []).length > 0 || (afterSwitch.guest?.selectedIds ?? []).length > 0;
  results.push({ example: label, step: "selection-after-switch", ok: index === 0 || !carried, detail: afterSwitch });
  console.log(`[DEBUG] ${label} after-switch host=${JSON.stringify(afterSwitch.host?.selectedIds ?? null)} guest=${JSON.stringify(afterSwitch.guest?.selectedIds ?? null)} inspector=${JSON.stringify(afterSwitch.inspector)}`);

  const picked = await pickNode();
  const host = picked.after.host?.selectedIds ?? [];
  const guest = picked.after.guest?.selectedIds ?? [];
  const replaced = host.length === 1;
  results.push({ example: label, step: "pick-replaces", ok: replaced, detail: { point: picked.point, host, guest, activeObjectId: picked.after.host?.activeObjectId ?? null, inspector: picked.after.inspector } });
  console.log(`[DEBUG] ${label} pick host=${JSON.stringify(host)} guest=${JSON.stringify(guest)} active=${JSON.stringify(picked.after.host?.activeObjectId ?? null)} inspector=${JSON.stringify(picked.after.inspector)}`);
  await page.screenshot({ path: join(outDir, `${index}-${label.replace(/\s+/g, "-").toLowerCase()}.png`) });
}

writeFileSync(join(outDir, "results.json"), JSON.stringify({ url, examples, results }, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log(`[DEBUG] DONE rows=${results.length} green=${results.filter((r) => r.ok).length}`);
await browser.close();
