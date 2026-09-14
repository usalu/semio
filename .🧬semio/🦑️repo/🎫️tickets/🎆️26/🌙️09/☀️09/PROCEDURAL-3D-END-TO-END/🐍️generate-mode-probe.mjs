/** 🧬 Generate-mode probe: boot → converge → ⌘⌥→ → add two generations → select the second → rename it
 * inline → remove the first → type into the Form window → read the generate preview back.
 *
 * 🧾 Steps and what each proves (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, gaps #5/#7/#8):
 *   add          — `addGeneration` from the tree's Actions row (the step this probe has always had)
 *   select       — clicking an existing generation row dispatches `selectGeneration`, and the SELECTED
 *                  row is the only one carrying the inline rename editor
 *   rename       — typing into that editor and blurring dispatches `renameGeneration` with the typed
 *                  text (a scalar `commit` payload arrives as `value`), and the roster shows the name
 *   remove       — the row's trash button dispatches `removeGeneration` and the roster shrinks
 *   form         — typing into a Form control dispatches `updateGenerationValues` and the generate
 *                  preview's mesh payload changes
 *   gumball      — the generate preview publishes `gumballActive:true` under a transform utility, i.e.
 *                  a generated instance can be moved the same way as in the edit preview
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=<dir> bun 🐍️generate-mode-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "generate-mode");
mkdirSync(outDir, { recursive: true });
const lines = []; const t0 = Date.now(); const results = { steps: [] };
const note = (step, ok, detail) => { results.steps.push({ step, ok, detail, t: Date.now() - t0 }); console.log(`[DEBUG] ${step} ok=${ok} ${JSON.stringify(detail).slice(0, 800)}`); };
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 600)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 600)}`));

const hosts = () => page.evaluate(() => [...document.querySelectorAll("[data-status-json]")].map((el) => { let m = 0; try { m = JSON.parse(el.getAttribute("data-meshes-json") ?? "[]").length; } catch {} let st = null; try { st = JSON.parse(el.getAttribute("data-status-json")); } catch {} return { id: el.getAttribute("data-surface-id"), meshes: m, meshesLen: (el.getAttribute("data-meshes-json") ?? "").length, phase: st?.phase, ratio: st?.progress?.ratio, hint: st?.hint }; }));
const waitFor = async (label, pred, seconds) => { let h = null; for (let i = 0; i < seconds; i++) { await page.waitForTimeout(1000); h = await hosts(); if (pred(h)) break; } console.log(`[DEBUG] ${label} t=${Date.now() - t0} ${JSON.stringify(h)}`); await page.screenshot({ path: join(outDir, `${label}.png`) }); return h; };

/** 🗂️ The Generations window as the DOM sees it: one row per generation (authored key
 * `procedural3d-play-generate.generation.<id>`), which of them carries the inline rename editor
 * (authored key `…<id>.rename`), and the row-action buttons painted on each row. */
const roster = () => page.evaluate(() => {
  const rows = [...document.querySelectorAll('[id*="procedural3d-play-generate.generation."]')].filter((el) => !el.id.endsWith(".rename")).map((el) => ({ id: el.id, generation: el.id.split(".").pop(), text: (el.textContent ?? "").trim().slice(0, 60) }));
  const editors = [...document.querySelectorAll('input[id$=".rename"]')].map((el) => ({ id: el.id, generation: el.id.slice(0, -".rename".length).split(".").pop(), value: el.value }));
  return { rows, rowIds: rows.map((r) => r.id), generations: rows.map((r) => r.generation), rowTexts: rows.map((r) => r.text), editors };
});

/** 📝️ The Form window's rendered controls, by authored node id (`generate.form.<questionId>.*`). A
 * `slider` question paints as a `role="slider"` widget driven by arrow keys, never as an `<input>`. */
const formControls = () => page.evaluate(() => [...document.querySelectorAll('[id*="generate.form."]')].map((el) => ({
  id: el.id,
  tag: el.tagName,
  role: el.getAttribute("role"),
  type: el.getAttribute("type"),
  value: el.value ?? el.getAttribute("aria-valuenow") ?? null,
})));

/** 🕹️ The generate preview's own guest selection lane — `gumballActive`/`transformMode` are exactly
 * what `World3dHost` gates the gumball on. */
const generateGuestSelection = () => page.evaluate(() => {
  const el = document.querySelector('[data-surface-id="window:generation3d-generate-preview"]');
  if (!el) return null;
  try { return JSON.parse(el.getAttribute("data-guest-selection-json") ?? "null"); } catch { return null; }
});

const generatePreview = async () => (await hosts()).find((h) => h.id === "window:generation3d-generate-preview") ?? null;

await page.goto(url, { waitUntil: "domcontentloaded" });
await waitFor("1-boot", (h) => h.some((x) => x.id === "window:procedural-preview" && x.meshes > 0), 120);
await page.keyboard.press("Meta+Alt+ArrowRight");
await waitFor("2-generate-mode", (h) => h.some((x) => x.id === "window:generation3d-generate-preview"), 20);

//#region ➕️Add two generations
const addRow = () => page.locator(':text-is("Add Generation")').first();
/** ⏱️ How long one `addGeneration` may take to paint its row. A FIXED 2.5 s wait graded the SECOND add
 * red on a run whose own later `add` step then found both rows — the generation had arrived, just after
 * the wait (`🗑️generated/react-oracle/generate-mode/results.json`: `add-2` false, `add` true with two
 * rows). A dropped add and a slow add are different findings, so the row polls for the roster it
 * requires and REPORTS the seconds it took (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
 * `📓️generate-add-flow-wire-quiet-tick-2026-09-14.md`). */
const addRowSeconds = Number(process.env.SEMIO_PROBE_ADD_WAIT ?? 60);
for (const round of [1, 2]) {
  if (await addRow().count()) { await addRow().click({ timeout: 4000 }); } else { note(`add-${round}`, false, "Add Generation row not found"); break; }
  const clickedAt = Date.now();
  let current = await roster();
  for (let i = 0; i < addRowSeconds * 2 && current.rowIds.length !== round; i++) { await page.waitForTimeout(500); current = await roster(); }
  note(`add-${round}`, current.rowIds.length === round, { seconds: Math.round((Date.now() - clickedAt) / 100) / 10, ...current });
}
await waitFor("3-added", (h) => h.some((x) => x.id === "window:generation3d-generate-preview" && x.meshes > 0), 120);
const afterAdd = await roster();
note("add", afterAdd.rowIds.length === 2, afterAdd);
//#endregion

//#region 🎯️Select the second generation
const second = afterAdd.rowIds[1];
if (second) {
  await page.locator(`[id="${second}"]`).first().click({ timeout: 4000 }).catch((error) => note("select-click", false, String(error).slice(0, 200)));
  await page.waitForTimeout(3000);
}
const afterSelect = await roster();
note("select", afterSelect.editors.length === 1 && afterSelect.editors[0]?.generation === afterAdd.generations[1], { second, ...afterSelect });
await page.screenshot({ path: join(outDir, "4-selected.png") });
//#endregion

//#region 🖊️Rename it inline
const editorId = afterSelect.editors[0]?.id;
if (editorId) {
  const editor = page.locator(`[id="${editorId}"]`).first();
  await editor.click({ timeout: 4000 }).catch(() => {});
  await page.keyboard.press("Meta+A").catch(() => {});
  await page.keyboard.type("Balcony Study", { delay: 40 }).catch((error) => note("rename-type", false, String(error).slice(0, 200)));
  const typed = await editor.inputValue().catch(() => null);
  note("rename-typed", typed === "Balcony Study", { typed });
  await page.keyboard.press("Enter").catch(() => {});
  await page.waitForTimeout(3500);
}
const afterRename = await roster();
note("rename", afterRename.rowTexts.some((text) => text.includes("Balcony Study")), afterRename);
await page.screenshot({ path: join(outDir, "5-renamed.png") });
//#endregion

//#region 🗑️Remove the first generation
const first = afterAdd.rowIds[0];
if (first) {
  await page.locator(`[id="${first}"]`).first().hover().catch(() => {});
  await page.waitForTimeout(300);
  // 🖱️ The row-action button lives in a hover-revealed region, so a scripted click is dispatched on the
  // element itself rather than at a hit-tested point.
  const clicked = await page.evaluate((rowId) => {
    const row = document.getElementById(rowId);
    if (!row) return { ok: false, reason: "row missing" };
    const buttons = [...row.querySelectorAll("button")].map((el) => ({ el, label: (el.getAttribute("title") ?? el.getAttribute("aria-label") ?? el.textContent ?? "").trim() }));
    const remove = buttons.find((entry) => /remove|entfernen/i.test(entry.label));
    if (!remove) return { ok: false, reason: "no remove button", buttons: buttons.map((entry) => entry.label) };
    remove.el.click();
    return { ok: true, label: remove.label };
  }, first);
  note("remove-click", clicked.ok, clicked);
  await page.waitForTimeout(4000);
}
const afterRemove = await roster();
note("remove", afterRemove.rowIds.length === 1 && !afterRemove.rowIds.includes(first), afterRemove);
await page.screenshot({ path: join(outDir, "6-removed.png") });
//#endregion

//#region 📝️Type into the Form window
const before = await generatePreview();
const controls = await formControls();
const sliderId = controls.find((control) => control.id.endsWith(".slider"))?.id ?? null;
if (sliderId) {
  const handle = page.locator(`[id="${sliderId}"] [role="slider"], [id="${sliderId}"] input[type="range"], [id="${sliderId}"]`).first();
  await handle.scrollIntoViewIfNeeded().catch(() => {});
  await handle.click({ timeout: 4000, force: true }).catch(() => {});
  await handle.focus().catch(() => {});
  for (let step = 0; step < 8; step++) { await page.keyboard.press("ArrowRight"); await page.waitForTimeout(150); }
  await page.waitForTimeout(1500);
  const changed = await waitFor("7-form-edited", (h) => { const now = h.find((x) => x.id === "window:generation3d-generate-preview"); return now && before && now.meshesLen !== before.meshesLen; }, 90);
  const now = changed.find((x) => x.id === "window:generation3d-generate-preview");
  note("form", Boolean(now && before && now.meshesLen !== before.meshesLen), { slider: sliderId, beforeMeshes: before, afterMeshes: now, sliderHtml: await page.evaluate((id) => document.getElementById(id)?.outerHTML.slice(0, 700) ?? null, sliderId) });
} else note("form", false, { controls });
//#endregion

//#region 🕹️Gumball on the generate preview
// 🧰️ The transform rail is the window's own Utilities chrome menu — the generate preview never had one
// until this lane gave it `window_kind_utilities`, so a generated instance could not be transformed.
const previewWindow = page.locator('[data-window-instance-id="generation3d-generate-preview"], [id*="generation3d-generate-preview"]').first();
await previewWindow.hover().catch(() => {});
const utilities = page.locator('button[title="Utilities"], button:has-text("Utilities")');
let rail = [];
for (let attempt = 0; attempt < await utilities.count(); attempt++) {
  await utilities.nth(attempt).click({ timeout: 3000, force: true }).catch(() => {});
  await page.waitForTimeout(700);
  rail = await page.evaluate(() => [...document.querySelectorAll('[role="menuitem"],[role="menuitemradio"],button,[data-slot="tree-label"]')].map((el) => (el.getAttribute("title") ?? el.textContent ?? "").trim()).filter((text) => ["Move", "Rotate", "Scale", "Verschieben", "Drehen", "Skalieren"].includes(text)));
  if (rail.length >= 3) break;
}
note("gumball-rail", rail.length >= 3, { rail, utilityButtons: await utilities.count() });
await page.screenshot({ path: join(outDir, "8-utility-rail.png") });

// 🖱️ Pick a generated instance, then read the lane `World3dHost` gates the gumball on.
const canvas = page.locator('[data-surface-id="window:generation3d-generate-preview"] canvas').first();
if (await canvas.count()) {
  const box = await canvas.boundingBox();
  if (box) {
    await page.mouse.click(box.x + box.width / 2, box.y + box.height / 2);
    await page.waitForTimeout(2500);
  }
}
const guest = await generateGuestSelection();
const armed = await page.evaluate(() => {
  const el = document.querySelector('[data-surface-id="window:generation3d-generate-preview"]');
  try { return JSON.parse(el?.getAttribute("data-selection-json") ?? "null"); } catch { return null; }
});
note("gumball-lane", guest !== null && "gumballActive" in (guest ?? {}), { guest, painted: armed });
await page.screenshot({ path: join(outDir, "9-gumball.png") });
//#endregion

writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("DONE", JSON.stringify(results.steps.map((s) => `${s.step}=${s.ok}`)));
await browser.close();
