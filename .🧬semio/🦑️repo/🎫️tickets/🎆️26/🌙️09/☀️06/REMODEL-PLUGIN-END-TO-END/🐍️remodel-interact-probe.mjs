/** 🎛️ Remodel interaction probe: boots the remodel react playground (6063), then (1) records the boot state
 * (shell beacon, Model window World3d host, panel toggles, Actions rows), (2) opens the Media panel,
 * (3) runs the `addStream` action from the Frames window Actions pane (document mutation) and checks the
 * history ledger + Media panel stream count, (4) presses mod+z until the stream is gone (undo proves the mutation
 * landed in the document store), (5) flips the Model window's `mesh` layer toggle measure
 * (`setLayerVisibility`, window-config lane) and checks the measure re-publishes, (6) picks the "Synthetic
 * Orbit" example in the navbar and checks the Media panel gains its stream + assets, (7) switches to the
 * Capture mode and checks the Frames window host renders.
 * Every step records the console delta, guest/host fault lines and a screenshot.
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=remodel-interact-1 bun 🐍️remodel-interact-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6063/?plugin=remodel";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "remodel-interact");
const settleSeconds = Number(process.env.SEMIO_PROBE_SETTLE ?? 12);
const actionId = process.env.SEMIO_PROBE_ACTION ?? "addStream";
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, m.type() === "error" ? 6000 : 600)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
const report = { url, steps: [] };
const faultLines = (from) => lines.slice(from).filter((l) => /trapped|panicked|action failed|shell fault|faults=|pageerror|unreachable|Fault \{|not a framework-reserved|do not decode|refused|DuplicateSiblingKey/.test(l)).map((l) => l.slice(0, 400));
const historyLines = (from) => lines.slice(from).filter((l) => /history patch applied|patchCursor/.test(l)).map((l) => l.slice(0, 200));
const note = async (step, detail, from) => {
  report.steps.push({ step, t: Date.now() - t0, detail, faults: faultLines(from), history: historyLines(from).slice(0, 6) });
  console.log(`[DEBUG] ${step} ${JSON.stringify(detail).slice(0, 1500)}`);
  writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
  await page.screenshot({ path: join(outDir, `${report.steps.length}-${step.replace(/[^a-z0-9]+/gi, "-")}.png`) }).catch(() => {});
};
const state = () => page.evaluate(() => {
  const body = document.body.innerText.replace(/\s+/g, " ");
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  const hosts = [...document.querySelectorAll("[data-surface-id]")].map((el) => { const st = parse(el.getAttribute("data-status-json")); let meshes = 0; try { const v = JSON.parse(el.getAttribute("data-meshes-json") ?? "[]"); meshes = Array.isArray(v) ? v.length : 0; } catch {} return { id: el.getAttribute("data-surface-id"), phase: st?.phase, fault: st?.fault?.code ?? null, canvases: el.querySelectorAll("canvas").length, textLength: (el.innerText ?? "").length, meshes }; });
  const treeRows = [...document.querySelectorAll('[role="treeitem"]')].map((el) => el.innerText.replace(/\s+/g, " ").trim().slice(0, 80));
  const gcpRows = treeRows.filter((t) => /GCP|Passpunkt|gcp/.test(t));
  const streamSummary = treeRows.find((t) => /^Streams: \d+/.test(t)) ?? null;
  const streamRows = treeRows.filter((t) => /\(Image Sequence|\(Video|Bildsequenz/.test(t));
  const toggles = [...document.querySelectorAll("button[aria-pressed]")].map((el) => ({ id: el.id, pressed: el.getAttribute("aria-pressed") })).filter((t) => t.id);
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    error: document.documentElement.getAttribute("data-semio-os-error"),
    hosts,
    treeItems: treeRows.length,
    gcpRows,
    streamSummary,
    streamRows,
    treeRows: treeRows.slice(0, 40),
    panelToggles: toggles.filter((t) => /panel/.test(t.id)).slice(0, 30),
    measureIds: [...document.querySelectorAll('[id*="measure"]')].map((el) => el.id).slice(0, 30),
    actionRows: [...document.querySelectorAll('[id^="action."]')].map((el) => el.id).slice(0, 60),
    engagements: [...document.querySelectorAll('[id$=".engagement"]')].map((el) => el.id),
    modeButtons: [...document.querySelectorAll('[id*="mode"]')].map((el) => el.id).slice(0, 20),
    bodyHead: body.slice(0, 700),
  };
});

await page.goto(url, { waitUntil: "domcontentloaded" });
let s = null;
for (let i = 0; i < 240; i++) { await page.waitForTimeout(1000); s = await state(); if (s.ready && s.hosts.length && i > 8) break; if (s.error) break; }
await note("boot", s, 0);

// ── open the Media panel (streams summary + rows) ──
{
  const from = lines.length;
  const opened = [];
  for (const id of ["remodeling.media"]) {
    const toggle = page.locator(`[id="${id}"]`).first();
    if (!(await toggle.count())) { opened.push(`absent:${id}`); continue; }
    const pressed = await toggle.getAttribute("aria-pressed");
    if (pressed === "true") { opened.push(`already:${id}`); continue; }
    opened.push(await toggle.click({ timeout: 8000, force: true }).then(() => `ok:${id}`).catch((e) => String(e).slice(0, 120)));
    await page.waitForTimeout(800);
  }
  await page.waitForTimeout(settleSeconds * 250);
  const after = await state();
  await note("open-panels", { opened, ...after }, from);
  s = after;
}
let before = s;

// ── Actions pane → addGcp (document mutation) ─────────────────────────────
{
  const from = lines.length;
  const engagement = before.engagements.find((id) => /remodelingFrames/i.test(id)) ?? before.engagements[0];
  let toggled = "absent";
  if (engagement) {
    const toggle = page.locator(`[id="${engagement}.toggle"]`).first();
    if (await toggle.count()) toggled = await toggle.click({ timeout: 8000, force: true }).then(() => `ok:${engagement}`).catch((e) => String(e).slice(0, 120));
    else toggled = `no-toggle:${engagement}`;
  }
  await page.waitForTimeout(1500);
  const opened = await state();
  const rowId = `action.${actionId}`;
  let clicked = "absent";
  if (opened.actionRows.includes(rowId)) clicked = await page.locator(`[id="${rowId}"]`).first().click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  await page.waitForTimeout(1500);
  let submitted = "none";
  const submit = page.locator(`[id$=".action.${actionId}.execute"]`).first();
  if (await submit.count()) submitted = await submit.click({ timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  else submitted = "no-execute-control";
  let after = null;
  const applied = () => lines.slice(from).filter((l) => /history patch applied.*create-stream/.test(l)).length;
  for (let i = 0; i < settleSeconds * 2; i++) { await page.waitForTimeout(500); after = await state(); if (applied() > 0 && after.streamSummary === "Streams: 1 - Assets: 0") break; }
  await page.waitForTimeout(1500);
  after = await state();
  await note("action-add-stream", { engagement, toggled, rowId, clicked, submitted, applied: applied(), actionRows: opened.actionRows, streamSummary: after.streamSummary, streamRows: after.streamRows, treeRows: after.treeRows, hosts: after.hosts }, from);
  before = after;
}

// ── undo through the keybinding ──────────────────────────────────────────
{
  const from = lines.length;
  // ↩️ The shell journals its own panel/engagement toggles too, so undo until the create-stream edit is gone.
  await page.locator('[id="remodeling.media"]').first().focus().catch(() => {});
  let after = null;
  let presses = 0;
  for (let attempt = 0; attempt < 4; attempt++) {
    await page.keyboard.press(process.platform === "darwin" ? "Meta+z" : "Control+z");
    presses += 1;
    for (let i = 0; i < settleSeconds; i++) { await page.waitForTimeout(500); after = await state(); if (after.streamSummary === "Streams: 0 - Assets: 0") break; }
    if (after.streamSummary === "Streams: 0 - Assets: 0") break;
  }
  await page.waitForTimeout(1000);
  after = await state();
  const undone = lines.slice(from).filter((l) => /history patch applied.*"Undo"/.test(l)).length;
  await note("undo", { presses, undoPatches: undone, undone: after.streamSummary === "Streams: 0 - Assets: 0", streamSummary: after.streamSummary, streamRows: after.streamRows, hosts: after.hosts }, from);
  before = after;
}

// ── Model window mesh layer toggle measure → setLayerVisibility ──────────
{
  const from = lines.length;
  const options = page.locator('[id="framework.window.remodelingMain.measures.unfold"]').first();
  let optionsOpened = "absent";
  if (await options.count()) optionsOpened = await options.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  await page.waitForTimeout(1500);
  const mid = await state();
  const toggle = page.locator('[id="remodeling-main/remodeling-measure-layer-mesh"]').first();
  let clicked = `absent (options ${optionsOpened})`;
  if (await toggle.count()) clicked = await toggle.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  await page.waitForTimeout(3000);
  const dispatched = lines.slice(from).filter((l) => /setLayerVisibility|layer-visibility/.test(l)).map((l) => l.slice(0, 220));
  let published = null;
  for (let i = 0; i < settleSeconds * 2; i++) { await page.waitForTimeout(500); published = await page.evaluate(() => document.getElementById("remodeling-main/remodeling-measure-layer-mesh")?.getAttribute("data-published-value") ?? document.getElementById("remodeling-main/remodeling-measure-layer-mesh")?.getAttribute("aria-pressed") ?? null); if (published === "false") break; }
  const after = await state();
  await note("measure-layer-mesh-toggle", { optionsOpened, measureIds: mid.measureIds, clicked, dispatched: dispatched.slice(0, 4), published, hosts: after.hosts }, from);
  await page.keyboard.press("Escape").catch(() => {});
}

// ── example switch → Synthetic Orbit (10 committed frames) ────────────────
{
  const from = lines.length;
  const example = process.env.SEMIO_PROBE_EXAMPLE ?? "Synthetic Orbit";
  const combo = page.locator('[role="combobox"]').first();
  let picked = "skipped";
  let options = [];
  if (await combo.count()) {
    await combo.click({ timeout: 5000 }).catch((e) => (picked = String(e).slice(0, 100)));
    await page.waitForTimeout(600);
    options = await page.locator('[role="option"]').allInnerTexts().catch(() => []);
    const option = page.locator('[role="option"]').filter({ hasText: example }).first();
    if (await option.count()) picked = await option.click({ timeout: 5000 }).then(() => `ok:${example}`).catch((e) => String(e).slice(0, 100));
    else { picked = "no-option"; await page.keyboard.press("Escape"); }
  }
  let after = null;
  for (let i = 0; i < settleSeconds * 3; i++) { await page.waitForTimeout(500); after = await state(); if (after.streamSummary && !/^Streams: 0/.test(after.streamSummary)) break; }
  await page.keyboard.press("Escape");
  await page.waitForTimeout(400);
  if (await page.locator('[role="option"]').count()) { await page.mouse.click(800, 700); await page.waitForTimeout(400); }
  const media = page.locator('[id="remodeling.media"]').first();
  if ((await media.getAttribute("aria-pressed")) !== "true") await media.click({ timeout: 4000, force: true }).catch(() => {});
  await page.waitForTimeout(2500);
  after = await state();
  const applied = lines.slice(from).filter((l) => /history patch applied.*(create-stream|create-asset|add-stream-frame)/.test(l)).length;
  await note("example-switch", { picked, options, applied, streamSummary: after.streamSummary, streamRows: after.streamRows, hosts: after.hosts }, from);
}

// ── Capture mode → Frames window ─────────────────────────────────────────
{
  const from = lines.length;
  const mode = page.locator('[id*="mode"][id*="capture"], button:has-text("Capture")').first();
  let clicked = "absent";
  if (await mode.count()) clicked = await mode.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  let after = null;
  for (let i = 0; i < settleSeconds * 2; i++) { await page.waitForTimeout(500); after = await state(); if (after.hosts.some((h) => /frames/i.test(h.id ?? ""))) break; }
  await note("capture-mode", { clicked, hosts: after.hosts, modeButtons: after.modeButtons, bodyHead: after.bodyHead }, from);
}

writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] INTERACT DONE", report.steps.length);
await browser.close();
