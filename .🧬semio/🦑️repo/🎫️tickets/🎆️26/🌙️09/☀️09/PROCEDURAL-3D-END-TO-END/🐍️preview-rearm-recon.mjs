/** 🖼️ Which gesture empties the EDIT preview and never re-arms it.
 *
 * The gap probe's `inspection-edit` step reads "the preview re-evaluated" as a change in the edit
 * preview's published `data-meshes-json`; by the time it runs, that payload is `[]` and stays `[]` for
 * 90 s, so the reading cannot express anything. This walks the same gestures one at a time and records
 * the edit preview's mesh count, byte length and status phase after each, plus the `previewEval` run
 * state the status names — so the gesture that empties it is named rather than guessed.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=generate-chord/rearm bun 🐍️preview-rearm-recon.mjs
 * @see 🐍️react-gap-probe.mjs step `inspection-edit`
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "generate-chord/rearm");
mkdirSync(outDir, { recursive: true });
const SURFACE = "window:procedural-main";

const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 }, acceptDownloads: true })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 600)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 600)}`));

const report = { url, steps: [] };
const read = () => page.evaluate(() => {
  const parse = (text) => { try { return JSON.parse(text ?? ""); } catch { return null; } };
  const previews = [...document.querySelectorAll("[data-meshes-json]")].map((el) => {
    const status = parse(el.getAttribute("data-status-json"));
    return {
      surfaceId: el.getAttribute("data-surface-id"),
      bytes: (el.getAttribute("data-meshes-json") ?? "").length,
      meshes: (() => { const value = parse(el.getAttribute("data-meshes-json")); return Array.isArray(value) ? value.length : 0; })(),
      phase: status?.phase ?? null,
      cancellable: status?.cancellable ?? null,
      fault: status?.fault?.code ?? null,
    };
  });
  const main = document.querySelector(`[data-surface-id="window:procedural-main"]`);
  const fixture = parse(main?.getAttribute("data-fixture-json"));
  return { previews, wires: (fixture?.synapses ?? []).length, widgets: (fixture?.widgets ?? []).length };
});
const note = async (step, extra = {}) => {
  const state = await read();
  report.steps.push({ step, ...state, ...extra, t: Date.now() - t0 });
  console.log(`[DEBUG] ${step} ${JSON.stringify({ ...state, ...extra })}`);
  writeFileSync(join(outDir, "recon.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
};
const until = async (pred, seconds) => { let v = await read(); for (let i = 0; i < seconds && !pred(v); i++) { await page.waitForTimeout(1000); v = await read(); } return v; };
const editMeshes = (state) => state.previews.filter((preview) => !/generate/.test(preview.surfaceId ?? "")).reduce((sum, preview) => sum + preview.meshes, 0);

await page.goto(url, { waitUntil: "domcontentloaded" });
await until((state) => editMeshes(state) > 0, 180);
await note("boot");

// ✏️ Does an INSPECTOR patch move the preview on a fresh, healthy boot? The one measurement that
// separates "the inspector cannot drive the preview" from "the gap probe's earlier steps left the
// preview stale".
if (process.env.SEMIO_PROBE_PATCH === "1") {
  const click = (id) => page.locator(`[id="${id}"]`).first().click({ timeout: 8000 }).catch((e) => lines.push(`${id} ${String(e).slice(0, 140)}`));
  await click("framework.panel.artifact");
  await page.waitForTimeout(2000);
  await page.locator('[data-slot="panel"] [id="panel:procedural-play-document/height"]').first().click({ timeout: 8000 }).catch((e) => lines.push(`height row ${String(e).slice(0, 140)}`));
  await page.waitForTimeout(2000);
  await click("framework.panel.inspection");
  await page.waitForTimeout(2500);
  const field = page.locator('[id="procedural-play-inspector.value.input"], [data-slot="panel"] [id$="procedural-play-inspector.value.input"]').first();
  const present = (await field.count()) > 0;
  const before = await read();
  let typed = null;
  if (present) {
    const old = Number(await field.inputValue().catch(() => "0"));
    typed = Number.isFinite(old) ? Number((old + 1).toFixed(3)) : 1;
    await field.fill(String(typed), { timeout: 8000 }).catch(() => {});
    await page.keyboard.press("Enter");
    await field.blur().catch(() => {});
  }
  const after = await until((state) => JSON.stringify(state.previews) !== JSON.stringify(before.previews), 60);
  await note("inspector-patch-on-fresh-boot", { present, typed, beforePreviews: before.previews, movedPreviews: JSON.stringify(after.previews) !== JSON.stringify(before.previews) });
  for (const tab of ["framework.panel.inspection", "framework.panel.artifact"]) { await click(tab); await page.waitForTimeout(800); await click(tab); await page.waitForTimeout(800); }
}

await page.keyboard.press("Meta+Alt+ArrowRight");
await page.waitForTimeout(6000);
await note("generate-entered");
const add = page.locator('[id="action.addGeneration"], :text-is("Add Generation")').first();
if (await add.count()) await add.click({ timeout: 8000 }).catch((e) => lines.push(`add ${String(e).slice(0, 120)}`));
await page.waitForTimeout(10000);
await note("generation-added");
await page.keyboard.press("Meta+Alt+ArrowLeft");
await page.waitForTimeout(10000);
await note("back-to-edit");
await note("back-to-edit-settled", { settled: editMeshes(await until((state) => editMeshes(state) > 0, 60)) });

// 🔪️ The wire cut, through the graph's own published port handle — the gap probe's own aim.
const target = await page.evaluate(([surface]) => {
  const parse = (text) => { try { return JSON.parse(text ?? ""); } catch { return null; } };
  const fixture = parse(document.querySelector(`[data-surface-id="${surface}"]`)?.getAttribute("data-fixture-json"));
  const edge = (fixture?.synapses ?? [])[0];
  return edge ? { to: edge.to, toPort: edge.toPort ?? edge.to_port } : null;
}, [SURFACE]);
if (target) {
  // ⏳️ The port handle's rect lands a few frames after the surface does — poll for it, exactly as
  // `🐍️react-gap-probe.mjs` does, or the drag starts on empty canvas and cuts nothing.
  const readPoint = () => page.evaluate(([surface, id]) => { const p = window.__semioFlowGraphProbe?.[surface]?.entity?.("handle", id) ?? null; return p?.visible ? (p.rect ? { x: p.rect.x + p.rect.width / 2, y: p.rect.y + p.rect.height / 2 } : p.point) : null; }, [SURFACE, `${target.to}@${target.toPort}`]);
  let point = await readPoint();
  for (let i = 0; i < 20 && !point; i += 1) { await page.waitForTimeout(1000); point = await readPoint(); }
  const host = await page.evaluate((id) => { const r = document.querySelector(`[data-surface-id="${id}"]`)?.getBoundingClientRect(); return r ? { x: r.x, y: r.y, width: r.width, height: r.height } : null; }, SURFACE);
  if (point && host) {
    const empty = { x: host.x + host.width * 0.5, y: host.y + host.height - 60 };
    await page.mouse.move(point.x, point.y); await page.mouse.down();
    await page.mouse.move((point.x + empty.x) / 2, (point.y + empty.y) / 2, { steps: 8 });
    await page.mouse.move(empty.x, empty.y, { steps: 8 }); await page.mouse.up();
    await page.waitForTimeout(8000);
  }
  await note("wire-cut", { target, point });
  await note("wire-cut-settled", { settled: editMeshes(await until((state) => editMeshes(state) > 0, 30)) });
  // 🖱️ Undo through the Actions pane ROW, not the chord: a mouse route exonerates (or convicts) the
  // keyboard loop, which is what this lane changed.
  await page.locator(`[data-surface-id="${SURFACE}"]`).first().click({ position: { x: 20, y: 20 } }).catch(() => {});
  await page.waitForTimeout(1200);
  const folded = await page.evaluate(() => document.getElementById("framework.window.proceduralMain.engagement")?.getAttribute("data-folded") === "true");
  if (folded) await page.locator('[id="framework.window.proceduralMain.engagement.toggle"]').first().click({ timeout: 8000 }).catch((e) => lines.push(`toggle ${String(e).slice(0, 120)}`));
  await page.waitForTimeout(2000);
  const route = process.env.SEMIO_PROBE_UNDO ?? "mouse";
  const undoRow = await page.locator('[id="action.undo"]').count();
  if (route === "chord") await page.keyboard.press("Meta+z");
  else if (undoRow) await page.locator('[id="action.undo"]').first().click({ timeout: 8000 }).catch((e) => lines.push(`undo row ${String(e).slice(0, 120)}`));
  await page.waitForTimeout(10000);
  await note(`wire-undone-by-${route}`, { undoRow, route });
  await note("wire-undone-settled", { settled: editMeshes(await until((state) => editMeshes(state) > 0, 90)), toolRunStarts: lines.filter((line) => line.includes('"actionId":"toolRunStart"')).length });
}

writeFileSync(join(outDir, "recon.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] REARM RECON DONE");
await browser.close();
