/** 🕳️ The user-visible gaps `📓️audit-user-journey-gaps-2026-09-13.md` lists that NO other probe in
 * this ticket covers. One boot, eight steps, each reading the app's OWN published evidence.
 *
 *   window-focus       — clicking a window body moves `[data-slot="window"][data-active="true"]` and
 *                        the dock tab that mirrors it.
 *   dock-resize        — dragging `[data-slot="resizable-handle"]` changes the flanking pane widths.
 *   export-after-generate — add a generation, return to edit, export through the window's own Actions
 *                        pane, and read the shell's download off disk.
 *   wire-undo          — cut a wire through the graph's published port handle, then `mod+z`, and
 *                        assert the wire comes BACK in the fixture (punchlist gap #4).
 *   doc-panel-select   — the Document (artifact) panel's tree row is clicked and the framework
 *                        selection it is supposed to publish is read back.
 *   inspection-edit    — with that selection standing, `procedural-play-inspector.value.input`
 *                        dispatches `patchFlowWidgets` and the preview RE-EVALUATES.
 *   catalogue-add      — a Catalogue OPERATOR row (`…catalogue.neuron.…`) dispatches `addWidget`.
 *   actions-pane-de    — the locale is switched to German and the Actions pane's own row labels read.
 *
 * 🪪️ Order matters: every Actions-pane step runs BEFORE the side panels are opened. With the
 * top-left/top-right panels expanded the window chrome's pane toggles stop accepting a click
 * (recorded by `🐍️react-gap-recon.mjs`), so a probe that opens the panels first can never reach them.
 * 🪪️ A step that finds nothing to act on records `ok:false` with what it saw — that IS the finding.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=react-verify/gaps bun 🐍️react-gap-probe.mjs
 * @see 🐍️react-battery.mjs, 🐍️react-gap-recon.mjs, 📓️audit-user-journey-gaps-2026-09-13.md §9
 */
import { chromium } from "playwright";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "react-gaps");
const bootWait = Number(process.env.SEMIO_PROBE_BOOT_WAIT ?? 180);
mkdirSync(outDir, { recursive: true });

const SURFACE = "window:procedural-main";
const PREVIEW = "window:procedural-preview";
/** 🇩🇪️ The English/German label pairs `✏️editor/🦀️.rs` declares for the palette-visible verbs. */
const ACTION_DE = { addWidget: ["Add Widget", "Element hinzufügen"], exportDocument: ["Export Document", "Dokument exportieren"], importDocumentRequest: ["Import Document…", "Dokument importieren…"], reorganize: ["Reorganize", "Neu anordnen"], deleteSelection: ["Delete Selection", "Auswahl löschen"] };

const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const context = await browser.newContext({ viewport: { width: 1600, height: 1000 }, acceptDownloads: true });
const page = await context.newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 1200)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 1200)}`));

const results = { url, steps: [] };
let shot = 0;
const note = async (step, ok, detail) => {
  shot += 1;
  await page.screenshot({ path: join(outDir, `${shot}-${step}.png`) }).catch(() => {});
  results.steps.push({ step, ok, detail, t: Date.now() - t0 });
  console.log(`[DEBUG] ${step} ok=${ok} ${JSON.stringify(detail).slice(0, 1100)}`);
  // ── 9. The addGeneration chord, in the mode whose window declares it ────────
{
  await page.keyboard.press("Meta+Alt+ArrowRight");
  await page.waitForTimeout(6000);
  const before = await snap();
  const mark = lines.length;
  await page.keyboard.press("Meta+Shift+g");
  const after = await until(snap, (s) => s.previews.some((p) => /generate/.test(p.surfaceId ?? "") && p.meshes > 0), 90);
  const chordInvoked = invoked(mark);
  await page.keyboard.press("Meta+Alt+ArrowLeft");
  await page.waitForTimeout(4000);
  await note("generate-chord", chordInvoked.includes("addGeneration"), { chord: "mod+shift+g", chordInvoked, previewsBefore: before.previews, previewsAfter: after.previews });
}

writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2));
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
};
/** 📣️ The action ids the shell invoked since `mark` — the only honest answer to "did the click reach
 * anything", since a row that reaches nothing logs nothing. */
const invoked = (mark) => [...new Set(lines.slice(mark).flatMap((l) => [...l.matchAll(/"actionId":"([^"]+)"/g)].map((m) => m[1])))];

/** 🗺️ Everything this probe reads off the page in one hop. */
const snap = () => page.evaluate(([surface]) => {
  const parse = (s) => { try { return JSON.parse(s ?? ""); } catch { return null; } };
  const main = document.querySelector(`[data-surface-id="${surface}"]`);
  const raw = main?.getAttribute("data-fixture-json") ?? window.__semioFlowGraphProbe?.[surface]?.fixtureJson?.() ?? null;
  const fx = parse(raw);
  const widgets = Array.isArray(fx?.widgets) ? fx.widgets.map((w) => { const key = w && typeof w === "object" ? Object.keys(w)[0] : null; const inner = key && typeof w[key] === "object" ? w[key] : null; return { kind: w?.kind ?? key ?? null, id: w?.id ?? inner?.id ?? null, value: w?.value ?? inner?.value ?? null }; }) : [];
  const previews = [...document.querySelectorAll("[data-meshes-json]")].map((el) => ({ surfaceId: el.getAttribute("data-surface-id"), bytes: (el.getAttribute("data-meshes-json") ?? "").length, meshes: (() => { const v = parse(el.getAttribute("data-meshes-json")); return Array.isArray(v) ? v.length : 0; })(), phase: parse(el.getAttribute("data-status-json"))?.phase ?? null }));
  return {
    widgets, widgetIds: widgets.map((w) => w.id).filter(Boolean), synapses: fx?.synapses ?? [],
    previews,
    selection: [...document.querySelectorAll("[data-selection-json]")].map((el) => ({ surfaceId: el.getAttribute("data-surface-id"), selectedIds: parse(el.getAttribute("data-selection-json"))?.selectedIds ?? null })),
    treeSelected: [...document.querySelectorAll('[role="treeitem"][aria-selected="true"]')].map((el) => el.id),
    activeWindow: document.querySelector('[data-slot="window"][data-active="true"]')?.id ?? null,
    dockTabs: [...document.querySelectorAll('[data-slot="mode-dock-tab"]')].map((el) => ({ id: el.getAttribute("data-window-id"), active: el.getAttribute("data-active") === "true" })),
    panes: [...document.querySelectorAll('[data-slot="resizable-panel"]')].map((el) => { const r = el.getBoundingClientRect(); return { id: el.id, w: Math.round(r.width), h: Math.round(r.height) }; }),
    handles: [...document.querySelectorAll('[data-slot="resizable-handle"]')].map((el) => { const r = el.getBoundingClientRect(); return { orientation: el.getAttribute("data-resize-orientation"), x: Math.round(r.x + r.width / 2), y: Math.round(r.y + r.height / 2) }; }),
  };
}, [SURFACE]);

const wires = (s) => (s.synapses ?? []).map((x) => `${x.from}@${x.fromPort ?? x.from_port}->${x.to}@${x.toPort ?? x.to_port}`).sort();
const previewBytes = (s) => Object.fromEntries(s.previews.map((p) => [p.surfaceId, p.bytes]));
const clickId = (id, timeout = 8000) => page.locator(`[id="${id}"]`).first().click({ timeout });
const hasId = async (id) => (await page.locator(`[id="${id}"]`).count()) > 0;
/** ⏳️ Poll `read` until `pred`, then hand back the last reading — never a single blind sample. */
const until = async (read, pred, seconds) => { let v = await read(); for (let i = 0; i < seconds && !pred(v); i++) { await page.waitForTimeout(1000); v = await read(); } return v; };

// ── boot ─────────────────────────────────────────────────────────────────────
await page.goto(url, { waitUntil: "domcontentloaded" });
const afterBoot = await until(snap, (s) => s.widgetIds.length > 0 && s.previews.some((p) => p.meshes > 0), bootWait);
await note("boot", afterBoot.widgetIds.length > 0 && afterBoot.previews.some((p) => p.meshes > 0), { widgets: afterBoot.widgetIds, previews: afterBoot.previews, activeWindow: afterBoot.activeWindow });
const slider = afterBoot.widgets.find((w) => /slider/i.test(String(w.kind ?? "")));

// ── 1. Dock resizing ─────────────────────────────────────────────────────────
{
  const before = await snap();
  const handle = before.handles[0] ?? null;
  let after = before;
  if (handle) {
    const dx = handle.orientation === "vertical" ? 0 : 110;
    const dy = handle.orientation === "vertical" ? 90 : 0;
    await page.mouse.move(handle.x, handle.y);
    await page.mouse.down();
    await page.mouse.move(handle.x + dx / 2, handle.y + dy / 2, { steps: 8 });
    await page.mouse.move(handle.x + dx, handle.y + dy, { steps: 8 });
    await page.mouse.up();
    await page.waitForTimeout(2500);
    after = await snap();
  }
  const moved = Boolean(handle) && before.panes.some((p, i) => after.panes[i] && (after.panes[i].w !== p.w || after.panes[i].h !== p.h));
  await note("dock-resize", moved, { handle, panesBefore: before.panes, panesAfter: after.panes });
}

// ── 2. Export after a generate ───────────────────────────────────────────────
{
  await page.keyboard.press("Meta+Alt+ArrowRight");
  await page.waitForTimeout(6000);
  const mark = lines.length;
  const add = page.locator('[id="action.addGeneration"], :text-is("Add Generation"), :text-is("Generation hinzufügen")').first();
  const addFound = (await add.count()) > 0;
  if (addFound) await add.click({ timeout: 8000 }).catch((e) => lines.push(`addGeneration click ${String(e).slice(0, 160)}`));
  const gen = await until(snap, (s) => s.previews.some((p) => /generate/.test(p.surfaceId ?? "") && p.meshes > 0), 120);
  const generated = gen.previews.filter((p) => /generate/.test(p.surfaceId ?? ""));
  const genInvoked = invoked(mark);
  await page.keyboard.press("Meta+Alt+ArrowLeft");
  await page.waitForTimeout(8000);
  let row = {};
  try {
    await page.locator(`[data-surface-id="${SURFACE}"]`).first().click({ position: { x: 20, y: 20 } });
    await page.waitForTimeout(1500);
    await clickId("framework.window.proceduralMain.engagement.toggle");
    await page.waitForTimeout(2000);
    for (let attempt = 0; attempt < 3 && (await page.locator("#format").count()) === 0; attempt += 1) { await clickId("action.exportDocument").catch(() => {}); await page.waitForTimeout(1800); }
    await page.locator("#format").click({ timeout: 8000 });
    await page.waitForTimeout(900);
    const options = await page.evaluate(() => [...document.querySelectorAll('[role="option"]')].map((n, i) => ({ i, text: (n.textContent ?? "").trim() })));
    const wanted = options.find((o) => /stl/i.test(o.text)) ?? options[0];
    await page.locator('[role="option"]').nth(wanted.i).click({ timeout: 8000 });
    await page.waitForTimeout(900);
    const [download] = await Promise.all([
      page.waitForEvent("download", { timeout: 120000 }),
      clickId("framework.window.proceduralMain.action.exportDocument.execute", 10000),
    ]);
    const saved = join(outDir, download.suggestedFilename() || "export-after-generate.bin");
    await download.saveAs(saved);
    const bytes = readFileSync(saved);
    row = { picked: wanted.text, filename: download.suggestedFilename(), bytes: bytes.length, head: bytes.subarray(0, 24).toString("latin1") };
  } catch (e) { row = { error: String(e).slice(0, 300) }; }
  await note("export-after-generate", addFound && generated.some((p) => p.meshes > 0) && (row.bytes ?? 0) > 0, { addFound, genInvoked, generated, export: row });
}

// ── 3. Cut a wire, then mod+z ────────────────────────────────────────────────
{
  const before = await snap();
  const target = before.synapses[0] ?? null;
  /** 🎯️ The graph paints its own ports, so the only honest aim point is the rect the host publishes
   * for the port handle — polled, because it lands a few frames after the surface does. */
  const portPoint = (port) => until(
    () => page.evaluate(([surface, id]) => { const p = window.__semioFlowGraphProbe?.[surface]?.entity?.("handle", id) ?? null; return p?.visible ? (p.rect ? { x: p.rect.x + p.rect.width / 2, y: p.rect.y + p.rect.height / 2 } : p.point) : null; }, [SURFACE, port]),
    (v) => Boolean(v), 20,
  );
  let cut = before; let undone = before; let sink = null; let cutLabelled = false; const mark = lines.length;
  if (target) {
    sink = await portPoint(`${target.to}@${target.toPort ?? target.to_port}`);
    const host = await page.evaluate((id) => { const el = document.querySelector(`[data-surface-id="${id}"]`); const r = el?.getBoundingClientRect(); return r ? { x: r.x, y: r.y, width: r.width, height: r.height } : null; }, SURFACE);
    if (sink && host) {
      const empty = { x: host.x + host.width * 0.5, y: host.y + host.height - 60 };
      await page.mouse.move(sink.x, sink.y); await page.mouse.down();
      await page.mouse.move((sink.x + empty.x) / 2, (sink.y + empty.y) / 2, { steps: 8 });
      await page.mouse.move(empty.x, empty.y, { steps: 8 }); await page.mouse.up();
      cut = await until(snap, (s) => wires(s).length < wires(before).length, 90);
      cutLabelled = lines.slice(mark).some((l) => /disconnect-synapse/.test(l));
      await page.keyboard.press("Meta+z");
      undone = await until(snap, (s) => wires(s).length === wires(before).length && wires(s).join() === wires(before).join(), 90);
    }
  }
  const historyLines = lines.slice(mark).filter((l) => /history (patch applied|route)|undo route/.test(l)).slice(-6).map((l) => l.slice(0, 220));
  const wasCut = wires(cut).length === wires(before).length - 1;
  await note("wire-undo", Boolean(target) && wasCut && cutLabelled && wires(undone).join() === wires(before).join(), { target, sink, before: wires(before), afterCut: wires(cut), afterUndo: wires(undone), wasCut, cutLabelled, undoInvoked: invoked(mark), historyLines });
}

// ── 4. Document panel: a tree row must publish a selection ───────────────────
{
  const tabOk = (await hasId("framework.panel.artifact")) ? await clickId("framework.panel.artifact").then(() => true).catch(() => false) : false;
  await page.waitForTimeout(2500);
  const rows = await page.evaluate(() => [...document.querySelectorAll('[data-slot="panel"] [role="treeitem"]')].map((el) => ({ id: el.id, selected: el.getAttribute("aria-selected"), text: (el.textContent ?? "").trim().slice(0, 40) })));
  const wanted = slider ? rows.find((r) => r.id.endsWith(`/${slider.id}`)) : rows.find((r) => /procedural-play-document\//.test(r.id));
  const target = (wanted?.id ?? "").split("/").pop() ?? "";
  const mark = lines.length;
  let after = await snap();
  if (wanted) {
    await page.locator(`[data-slot="panel"] [id="${wanted.id}"]`).first().click({ timeout: 8000 }).catch((e) => lines.push(`doc row click ${String(e).slice(0, 160)}`));
    after = await until(snap, (s) => s.selection.some((x) => (x.selectedIds ?? []).includes(target)) || s.treeSelected.some((id) => id.endsWith(`/${target}`)), 25);
  }
  const selectedHere = after.treeSelected.some((id) => id === wanted?.id || id.endsWith(`/${target}`));
  const published = after.selection.some((x) => (x.selectedIds ?? []).includes(target));
  await note("doc-panel-select", Boolean(wanted) && published, { tabOk, rows: rows.map((r) => r.id), clicked: wanted?.id ?? null, target, invoked: invoked(mark), treeSelected: after.treeSelected, selection: after.selection, selectedHere, published });
}

// ── 5. Inspection panel: the slider field → patchFlowWidgets → re-evaluate ───
{
  const readInspector = () => page.evaluate(() => [...document.querySelectorAll('[data-slot="panel"] [id*="procedural-play-inspector"]')].map((el) => ({ id: el.id, tag: el.tagName, value: el.value ?? null, text: (el.textContent ?? "").trim().slice(0, 60) })));
  let tabOk = false;
  let inspectorRows = await readInspector();
  for (let attempt = 0; attempt < 3 && inspectorRows.length === 0; attempt += 1) {
    tabOk = (await hasId("framework.panel.inspection")) ? await clickId("framework.panel.inspection").then(() => true).catch(() => false) : false;
    await page.waitForTimeout(2500);
    inspectorRows = await readInspector();
  }
  const field = page.locator('[data-slot="panel"] [id$="procedural-play-inspector.value.input"], [id="procedural-play-inspector.value.input"]').first();
  const present = (await field.count()) > 0;
  const before = await snap();
  let after = before; let typed = null; const mark = lines.length;
  if (present) {
    const old = Number(await field.inputValue().catch(() => "0"));
    typed = Number.isFinite(old) ? Number((old + 1).toFixed(3)) : 1;
    await field.fill(String(typed), { timeout: 8000 }).catch(() => {});
    await page.keyboard.press("Enter");
    await field.blur().catch(() => {});
    after = await until(snap, (s) => { const w = s.widgets.find((x) => x.id === (slider?.id ?? "")); return Boolean(w) && Number(w.value) === typed && JSON.stringify(previewBytes(s)) !== JSON.stringify(previewBytes(before)); }, 60);
  }
  const widgetAfter = after.widgets.find((x) => x.id === (slider?.id ?? "")) ?? null;
  const valueMoved = widgetAfter != null && typed != null && Number(widgetAfter.value) === typed;
  const previewMoved = JSON.stringify(previewBytes(after)) !== JSON.stringify(previewBytes(before));
  await note("inspection-edit", present && valueMoved && previewMoved, { tabOk, present, inspectorRows, sliderId: slider?.id ?? null, typed, widgetAfter, invoked: invoked(mark), bytesBefore: previewBytes(before), bytesAfter: previewBytes(after), valueMoved, previewMoved });
}

// ── 6. Catalogue panel: an operator row dispatches addWidget ─────────────────
{
  const tabOk = (await hasId("framework.panel.catalogue")) ? await clickId("framework.panel.catalogue").then(() => true).catch(() => false) : false;
  await page.waitForTimeout(2500);
  const rows = await page.evaluate(() => [...document.querySelectorAll('[data-slot="panel"] [role="treeitem"]')].map((el) => ({ id: el.id, text: (el.textContent ?? "").trim().slice(0, 40) })));
  // 🪪️ Several kinds, because `addWidget` swallows a refused kind (`🎮️commands/🧩️add-widget`
  // answers `Emit::default()` when `host.add_widget` fails), so one dead row cannot be told from a
  // dead action without trying more than one.
  const candidates = [rows.find((r) => /catalogue\.(inputSlider|slider)/i.test(r.id)), rows.find((r) => /catalogue\.neuron\.math\./.test(r.id)), rows.find((r) => /catalogue\.neuron\./.test(r.id))].filter(Boolean);
  const tried = [];
  let before = await snap();
  let after = before;
  for (const candidate of candidates) {
    const mark = lines.length;
    await page.locator(`[data-slot="panel"] [id="${candidate.id}"]`).first().click({ timeout: 8000 }).catch((e) => lines.push(`catalogue click ${String(e).slice(0, 160)}`));
    after = await until(snap, (s) => s.widgetIds.length > before.widgetIds.length, 30);
    tried.push({ id: candidate.id, text: candidate.text, invoked: invoked(mark), widgetsBefore: before.widgetIds.length, widgetsAfter: after.widgetIds.length, added: after.widgetIds.filter((id) => !before.widgetIds.includes(id)) });
    if (after.widgetIds.length > before.widgetIds.length) break;
    before = after;
  }
  await note("catalogue-add", tried.some((t) => t.added.length > 0), { tabOk, rows: rows.length, tried, rowSample: rows.slice(0, 8).map((r) => r.id) });
}

// ── 7. German labels in the Actions pane ────────────────────────────────────
{
  // 🪟️ Fold the side panels away first: an expanded panel makes the window chrome's pane toggles
  // unclickable, which would hide the i18n answer behind a layout problem.
  for (const tab of ["framework.panel.catalogue", "framework.panel.inspection", "framework.panel.artifact"]) await clickId(tab).catch(() => {});
  await page.waitForTimeout(2500);
  const readPane = () => page.evaluate(() => {
    const pane = document.querySelector('[id="framework.window.proceduralMain.engagement"]');
    return { paneFound: Boolean(pane), folded: pane?.getAttribute("data-folded") ?? null, rows: [...(pane ?? document).querySelectorAll('[id^="action."]')].map((el) => ({ id: el.id, label: (el.querySelector('[data-slot="tree-label"]')?.textContent ?? el.textContent ?? "").trim().slice(0, 60) })) };
  });
  await page.locator(`[data-surface-id="${SURFACE}"]`).first().click({ position: { x: 20, y: 20 } }).catch(() => {});
  await page.waitForTimeout(1500);
  if ((await readPane()).rows.length === 0) { await clickId("framework.window.proceduralMain.engagement.toggle").catch((e) => lines.push(`en toggle ${String(e).slice(0, 160)}`)); await page.waitForTimeout(2500); }
  const english = await readPane();
  let settings = false;
  try {
    await clickId("framework.settings");
    await page.waitForTimeout(1800);
    await clickId("framework.settings.language");
    await page.waitForTimeout(1200);
    const opt = page.locator('[role="option"]').filter({ hasText: /Deutsch/ }).first();
    if (await opt.count()) { await opt.click({ timeout: 8000 }); settings = true; }
    await page.keyboard.press("Escape");
    await page.waitForTimeout(5000);
  } catch (e) { lines.push(`german settings ${String(e).slice(0, 200)}`); }
  await page.locator(`[data-surface-id="${SURFACE}"]`).first().click({ position: { x: 20, y: 20 } }).catch(() => {});
  await page.waitForTimeout(1500);
  if ((await readPane()).rows.length === 0) { await clickId("framework.window.proceduralMain.engagement.toggle").catch((e) => lines.push(`de toggle ${String(e).slice(0, 160)}`)); await page.waitForTimeout(2500); }
  const german = await readPane();
  const trim = (s) => s.replace(/…$/, "").trim();
  const checks = Object.entries(ACTION_DE).map(([id, [en, de]]) => { const row = german.rows.find((r) => r.id === `action.${id}`); return { id, en, de, seen: row?.label ?? null, ok: row ? trim(row.label) === trim(de) : null }; });
  const decided = checks.filter((c) => c.ok !== null);
  await note("actions-pane-de", settings && decided.length > 0 && decided.every((c) => c.ok), { settings, english, german, checks, decided: decided.length, stillEnglish: checks.filter((c) => c.ok === false).map((c) => `${c.id}=${c.seen}`) });
}

// ── 8. Window focus ──────────────────────────────────────────────────────────
{
  const before = await snap();
  await page.locator(`[data-surface-id="${SURFACE}"]`).first().click({ position: { x: 20, y: 20 } }).catch(() => {});
  await page.waitForTimeout(2000);
  const onMain = await snap();
  // 🪪️ NOT the preview surface itself: a World3d canvas captures its own pointer for camera and
  // picking, so a click there is a gesture on the scene, never a focus change. The dock tab is the
  // affordance a user actually moves focus with.
  await page.locator('[data-slot="mode-dock-tab"][data-window-id="procedural-preview"]').first().click({ timeout: 8000 }).catch((e) => lines.push(`preview tab ${String(e).slice(0, 160)}`));
  await page.waitForTimeout(2500);
  const onPreview = await snap();
  const ok = onMain.activeWindow === "procedural-main" && onPreview.activeWindow === "procedural-preview"
    && onPreview.dockTabs.some((t) => t.id === "procedural-preview" && t.active);
  await note("window-focus", ok, { atBoot: before.activeWindow, afterMainClick: onMain.activeWindow, afterPreviewClick: onPreview.activeWindow, dockTabs: onPreview.dockTabs });
}

writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log(`[DEBUG] GAPS DONE ${results.steps.filter((s) => s.ok).length}/${results.steps.length}`);
await browser.close();
