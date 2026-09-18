/** 🧭️ Lowpoly end-to-end journey (react playground 6078): boot → catalogue addPrimitive → object pick →
 * Actions `translateSelection` → face granularity + pick → extrude → ⌘Z → extrude again → `toggleSmooth` →
 * Paint mode → `paintFill` → `exportMesh` (a real OBJ download, parsed) → `loadMeshRequest` (the shell's
 * file picker answered with a hand-written multi-object OBJ) → the document is the imported one.
 * Each step records the console delta, fault lines and a screenshot; the run ends with a verdict table.
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=lowpoly-journey-1 bun 🐍️lowpoly-journey-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6078/?plugin=lowpoly";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "lowpoly-journey");
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const context = await browser.newContext({ viewport: { width: 1600, height: 1000 }, acceptDownloads: true });
if (process.env.SEMIO_PROBE_DIAGNOSTICS === "1") await context.addInitScript(() => { try { localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
const page = await context.newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, m.type() === "error" ? 6000 : 600)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
const report = { url, steps: [], verdicts: [] };
const faultLines = (from) => lines.slice(from).filter((l) => /trapped|panicked|action failed|shell fault|faults=|pageerror|unreachable|Fault \{|not-ui-safe|missing-owned|refused|dropped action|invalid-args|unsupported/.test(l)).filter((l) => !/setActiveExample/.test(l)).map((l) => l.slice(0, 500));
const note = async (step, detail, from) => {
  report.steps.push({ step, t: Date.now() - t0, detail, faults: faultLines(from) });
  console.log(`[DEBUG] ${step} ${JSON.stringify(detail).slice(0, 900)}`);
  writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
  await page.screenshot({ path: join(outDir, `${report.steps.length}-${step.replace(/[^a-z0-9]+/gi, "-")}.png`) }).catch(() => {});
};
const verdict = (name, ok, detail) => { report.verdicts.push({ name, ok, detail }); console.log(`[DEBUG] VERDICT ${ok ? "PASS" : "FAIL"} ${name} ${detail ?? ""}`); };
const state = () => page.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  const world = document.querySelector('[data-surface-id="window:lowpoly-main"]');
  const meshes = parse(world?.getAttribute("data-meshes-json") ?? "[]") ?? [];
  const selection = parse(world?.getAttribute("data-selection-json") ?? "null");
  const rect = world?.getBoundingClientRect();
  const history = parse(document.querySelector("[data-history-json]")?.getAttribute("data-history-json") ?? "null");
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    error: document.documentElement.getAttribute("data-semio-os-error"),
    world: rect ? { x: Math.round(rect.x), y: Math.round(rect.y), w: Math.round(rect.width), h: Math.round(rect.height) } : null,
    meshes: meshes.map((m) => ({ id: m.id, faces: (m.data?.faceIds ?? []).length, triangles: (m.data?.indices ?? []).length / 3, center: m.transform?.position ?? m.position ?? null })),
    selection: selection ? { mode: selection.selectionMode, ids: selection.selectedIds ?? selection.ids, componentIds: selection.componentIds, active: selection.activeObjectId, gumballActive: selection.gumballActive } : null,
    engagements: [...document.querySelectorAll('[id$=".engagement"]')].map((el) => el.id),
    actionRows: [...document.querySelectorAll('[id^="action."]')].map((el) => el.id),
    cursor: history?.cursor ?? null,
    labels: history?.labels ?? [],
    bodyHead: document.body.innerText.replace(/\s+/g, " ").slice(0, 300),
  };
});
const faces = (s) => s.meshes.reduce((sum, m) => sum + m.faces, 0);
const settle = async (predicate, seconds = 20) => { let after = null; for (let i = 0; i < seconds * 2; i++) { await page.waitForTimeout(500); after = await state(); if (predicate(after)) break; } return after; };
const clickId = async (id) => { const el = page.locator(`[id="${id}"]`).first(); return (await el.count()) ? el.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "absent"; };
const openActions = async () => { const s = await state(); if (!s.actionRows.length && s.engagements.length) await clickId(`${s.engagements[0]}.toggle`); await page.waitForTimeout(800); };
/** 🎛️ Expands an Actions-pane row, stages args by control id, and clicks its execute control. */
const submitAction = async (actionId, args = {}) => {
  await openActions();
  const opened = await state();
  let clicked = "already-expanded";
  if (!(await page.locator(`[id$=".action.${actionId}.execute"]`).count())) clicked = opened.actionRows.includes(`action.${actionId}`) ? await clickId(`action.${actionId}`) : `absent in ${opened.actionRows.length} rows`;
  await page.waitForTimeout(900);
  const staged = {};
  for (const [key, value] of Object.entries(args)) {
    // 🎛️ `renderStagedArgControl` gives the control the bare arg id (`dx`, `format`), scoped by the form row.
    const control = page.locator(`[id$=".action.${actionId}.arg.${key}"] [id="${key}"], [id$=".action.${actionId}.arg.${key}"] input, [id="${key}"]`).first();
    if (!(await control.count())) { staged[key] = "no-control"; continue; }
    const tag = await control.evaluate((el) => el.tagName.toLowerCase() + (el.getAttribute("role") ? `[${el.getAttribute("role")}]` : ""));
    if (tag === "select") { await control.selectOption(String(value)).catch(() => {}); staged[key] = "select"; }
    else if (tag.includes("combobox")) { await control.click({ force: true }); await page.waitForTimeout(300); const opt = page.locator(`[role="option"]:has-text("${String(value).toUpperCase()}"), [role="option"]:has-text("${value}")`).first(); staged[key] = (await opt.count()) ? await opt.click().then(() => "option") : "no-option"; }
    else { await control.fill(String(value)).catch(() => {}); staged[key] = "filled"; }
  }
  const submit = page.locator(`[id$=".action.${actionId}.execute"]`).first();
  let submitted = "no-execute-control";
  if (await submit.count()) submitted = await submit.click({ timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  else if (clicked === "ok") submitted = "row-click-executes";
  return { clicked, staged, submitted };
};
const openOptions = async () => { if (await page.locator('[id="lowpoly-main/lowpoly-select-face"]').count()) return "open"; const b = page.locator('button:has-text("Window Options")').first(); if (await b.count()) await b.click({ force: true }); await page.waitForTimeout(800); return "clicked"; };
const clickWorld = async (fx = 0.5, fy = 0.5) => { const s = await state(); if (!s.world) return "no-world"; await page.mouse.click(s.world.x + s.world.w * fx, s.world.y + s.world.h * fy); return "ok"; };

await page.goto(url, { waitUntil: "domcontentloaded" });
let s = null;
for (let i = 0; i < 240; i++) { await page.waitForTimeout(1000); s = await state(); if ((s.ready && s.world && s.meshes.length && i > 8) || s.error) break; }
await note("boot", s, 0);
verdict("boot", s.ready === "lowpoly" && !s.error && s.meshes.length === 1, `meshes=${s.meshes.length} faces=${faces(s)}`);
const bootFaces = faces(s);

// ➕️ Catalogue: addPrimitive box → a second mesh.
{
  const from = lines.length;
  const a = await submitAction("addPrimitive", { kind: "box" });
  const after = await settle((x) => x.meshes.length > 1, 30);
  await note("add-primitive", { ...a, meshes: after.meshes, cursor: after.cursor }, from);
  verdict("addPrimitive", after.meshes.length === 2 && faultLines(from).length === 0, `meshes=${after.meshes.length} faults=${faultLines(from).length}`);
}
// 🧲️ Object move: the new box is active; translateSelection dx=1 on the whole object.
{
  const from = lines.length;
  const before = await state();
  const a = await submitAction("translateSelection", { dx: 1 });
  const after = await settle((x) => x.cursor !== before.cursor, 20);
  await note("translate-selection", { ...a, cursorBefore: before.cursor, cursor: after.cursor, labels: after.labels.slice(-2).map((l) => l.slice(0, 40)) }, from);
  verdict("translateSelection", after.cursor !== before.cursor && faultLines(from).length === 0, `cursor ${before.cursor}→${after.cursor} faults=${faultLines(from).length}`);
}
// ✂️ Face pick + extrude on the ACTIVE object (the box is at the centre now? pick whatever is at centre).
let extrudeBase = 0;
{
  const from = lines.length;
  await openOptions();
  await clickId("lowpoly-main/lowpoly-select-face");
  await settle((x) => x.selection?.mode === "face", 10);
  await clickWorld();
  const picked = await settle((x) => (x.selection?.componentIds ?? []).length > 0, 15);
  extrudeBase = faces(picked);
  const a = await submitAction("extrude", {});
  const after = await settle((x) => faces(x) > extrudeBase, 30);
  await note("pick-face-extrude", { ...a, picked: picked.selection, before: extrudeBase, faces: faces(after) }, from);
  verdict("face pick + extrude", (picked.selection?.componentIds ?? []).length > 0 && faces(after) > extrudeBase, `comp=${JSON.stringify(picked.selection?.componentIds)} faces ${extrudeBase}→${faces(after)}`);
}
// ↩️ Undo then the identical extrude again.
{
  const from = lines.length;
  await page.mouse.move(10, 10);
  await page.keyboard.press(process.platform === "darwin" ? "Meta+z" : "Control+z");
  const undone = await settle((x) => faces(x) === extrudeBase, 30);
  const a = await submitAction("extrude", {});
  const again = await settle((x) => faces(x) > extrudeBase, 30);
  await note("undo-extrude-again", { undone: faces(undone), ...a, again: faces(again) }, from);
  verdict("undo + identical extrude", faces(undone) === extrudeBase && faces(again) > extrudeBase, `undo→${faces(undone)} again→${faces(again)}`);
}
// 🔘️ toggleSmooth lands as a document edit.
{
  const from = lines.length;
  const before = await state();
  const a = await submitAction("toggleSmooth", {});
  const after = await settle((x) => x.cursor !== before.cursor, 15);
  await note("toggle-smooth", { ...a, cursorBefore: before.cursor, cursor: after.cursor }, from);
  verdict("toggleSmooth", after.cursor !== before.cursor && faultLines(from).length === 0, `cursor ${before.cursor}→${after.cursor}`);
}
// 🎨️ Paint: paintFill on the active layer commits an EditPaintLayer.
{
  const from = lines.length;
  const before = await state();
  const a = await submitAction("paintFill", {});
  const after = await settle((x) => x.cursor !== before.cursor, 20);
  await note("paint-fill", { ...a, cursorBefore: before.cursor, cursor: after.cursor, labels: after.labels.slice(-2).map((l) => l.slice(0, 40)) }, from);
  verdict("paintFill", after.cursor !== before.cursor && faultLines(from).length === 0, `cursor ${before.cursor}→${after.cursor} faults=${faultLines(from).length}`);
}
// 📤️ Export: a real OBJ download.
{
  const from = lines.length;
  const download = page.waitForEvent("download", { timeout: 30000 }).catch(() => null);
  const a = await submitAction("exportMesh", { format: "obj" });
  const file = await download;
  let parsed = null;
  if (file) { const path = join(outDir, file.suggestedFilename()); await file.saveAs(path); const text = await Bun.file(path).text(); parsed = { filename: file.suggestedFilename(), bytes: text.length, vertices: (text.match(/^v /gm) ?? []).length, faces: (text.match(/^f /gm) ?? []).length, objects: (text.match(/^o /gm) ?? []).length }; }
  await note("export-obj", { ...a, download: parsed }, from);
  verdict("exportMesh obj download", !!parsed && parsed.faces > 0 && parsed.filename.endsWith(".obj"), JSON.stringify(parsed));
}
// 📥️ Import: loadMeshRequest opens the shell's file picker; answer it with a hand-written OBJ.
{
  const from = lines.length;
  const objText = "o Quad\nv 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\nf 1 2 3 4\no Pentagon\nv 0 0 2\nv 1 0 2\nv 1.5 1 2\nv 0.5 1.8 2\nv -0.5 1 2\nf 5 6 7 8 9\n";
  const objPath = join(outDir, "journey-import.obj");
  writeFileSync(objPath, objText);
  const chooser = page.waitForEvent("filechooser", { timeout: 30000 }).catch(() => null);
  const a = await submitAction("loadMeshRequest", {});
  const fc = await chooser;
  let chosen = "no-filechooser";
  if (fc) { await fc.setFiles(objPath); chosen = "set"; }
  const after = await settle((x) => x.meshes.length === 2 && x.meshes.every((m) => m.faces <= 8) , 30);
  await note("import-obj", { ...a, chosen, meshes: after.meshes }, from);
  const ok = chosen === "set" && after.meshes.length === 2 && after.meshes.some((m) => m.faces === 2) && after.meshes.some((m) => m.faces === 3);
  verdict("loadMeshRequest + importMeshFile", ok, `chosen=${chosen} meshes=${JSON.stringify(after.meshes.map((m) => [m.id, m.faces]))} faults=${faultLines(from).length}`);
}

writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] JOURNEY DONE", report.verdicts.filter((v) => v.ok).length, "/", report.verdicts.length);
await browser.close();
