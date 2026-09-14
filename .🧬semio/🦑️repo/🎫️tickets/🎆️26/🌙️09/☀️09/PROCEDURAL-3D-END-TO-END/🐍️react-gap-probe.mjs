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
 *   inspection-edit    — with that selection standing, `procedural-play-inspector.value.input` exists
 *                        and its edit dispatches `patchFlowWidgets` into the DOCUMENT.
 *   inspection-preview-rearm — and the edit preview then re-evaluates. Split from the step above
 *                        because it is a different owner (`🧵️preview-eval`'s run lifecycle) and a
 *                        different failure: the payload's own DIGEST, not merely its length, decides.
 *   catalogue-add      — a Catalogue OPERATOR row (`…catalogue.neuron.…`) dispatches `addWidget`.
 *   actions-pane-de    — the locale is switched to German and the Actions pane's own row labels read.
 *
 * 🪪️ Order matters, twice. (a) Every Actions-pane step needs the window chrome's own pane toggles
 * uncovered, so any step that opens a side panel folds it again afterwards — an expanded panel sits on
 * top of those toggles (recorded by `🐍️react-gap-recon.mjs`, re-measured by `🐍️actions-pane-recon.mjs`),
 * and a panel TAB folds its panel only on a re-press of the tab that is already active, never on a
 * fresh pick. (b) `inspection-edit` reads "the preview re-evaluated" as a change in the preview's
 * published mesh payload, which an EMPTY preview cannot express — so it first RESTORES that
 * precondition by re-picking the same example from the navbar (the one gesture this app guarantees
 * re-arms every attached preview, `setActiveExample` → `rearm_attached_previews`), and records
 * `previewArmed:false` rather than blaming the inspector if the preview stays empty anyway.
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
/** 🇩🇪️ Spot-checks against the exact German `✏️editor/🦀️.rs` declares — a guard that the comparison
 * below is reading real labels and not, say, two identical fallback strings. */
const ACTION_DE = { addWidget: ["Add Widget", "Element hinzufügen"], exportDocument: ["Export Document", "Dokument exportieren"], importDocumentRequest: ["Import Document…", "Dokument importieren…"], reorganize: ["Reorganize", "Neu anordnen"], deleteSelection: ["Delete Selection", "Auswahl löschen"] };
/** 🟰️ Action ids whose German is legitimately the same text as the English — proper nouns and
 * international terms. Everything else in the pane must differ between the two passes. */
const ACTION_DE_IDENTICAL_BY_DESIGN = [];

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
  /** 🔢️ A payload's own CONTENT, not merely its length: two different meshes of the same topology
   * serialize to the same number of characters (a column whose height changes keeps every digit
   * count), so a byte-length comparison reports "nothing happened" for a real re-evaluation. */
  const digest = (text) => { let hash = 2166136261; for (let index = 0; index < text.length; index += 1) { hash ^= text.charCodeAt(index); hash = Math.imul(hash, 16777619); } return (hash >>> 0).toString(16); };
  const previews = [...document.querySelectorAll("[data-meshes-json]")].map((el) => ({ surfaceId: el.getAttribute("data-surface-id"), bytes: (el.getAttribute("data-meshes-json") ?? "").length, digest: digest(el.getAttribute("data-meshes-json") ?? ""), meshes: (() => { const v = parse(el.getAttribute("data-meshes-json")); return Array.isArray(v) ? v.length : 0; })(), phase: parse(el.getAttribute("data-status-json"))?.phase ?? null }));
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
const previewBytes = (s) => Object.fromEntries(s.previews.map((p) => [p.surfaceId, `${p.bytes}:${p.digest}`]));
const clickId = (id, timeout = 8000) => page.locator(`[id="${id}"]`).first().click({ timeout });
/** 🪟️ Whether the flow window's Actions pane is folded right now — the only honest precondition for
 * pressing its toggle, which toggles rather than opens. */
const paneFolded = () => page.evaluate(() => document.getElementById("framework.window.proceduralMain.engagement")?.getAttribute("data-folded") === "true");
/** 🧹️ Drops every standing selection so a step that needs ONE selected widget really has one. The
 * interaction domain binds `escape` to `clearSelection`; the surface must hold focus for it. */
const clearSelection = async () => {
  // 🎯️ Empty canvas, low and centre: the graph paints its nodes around the middle band and the top-left
  // corner is where a side panel overlaps the surface, so a click there is a click on the panel.
  const host = await page.evaluate((id) => { const r = document.querySelector(`[data-surface-id="${id}"]`)?.getBoundingClientRect(); return r ? { x: r.x, y: r.y, width: r.width, height: r.height } : null; }, SURFACE);
  if (host) await page.mouse.click(host.x + host.width * 0.5, host.y + host.height - 40);
  await page.keyboard.press("Escape");
  await page.waitForTimeout(1200);
};
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
    // 🪟️ The toggle TOGGLES: a pane left open by an earlier step is folded by an unconditional click,
    // and every row this step needs then disappears. Only unfold a pane that is actually folded.
    if (await paneFolded()) await clickId("framework.window.proceduralMain.engagement.toggle");
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
  // 🗂️ BOTH panels: the slider is selected from the Document panel's own row and the answer is read
  // from the Inspection panel, so a step that opens only one of them can never establish its own
  // precondition (measured: `tabOk:true, present:false` with the inspector on its empty branch).
  const docTabOk = (await hasId("framework.panel.artifact")) ? await clickId("framework.panel.artifact").then(() => true).catch(() => false) : false;
  await page.waitForTimeout(1800);
  let tabOk = false;
  let inspectorRows = await readInspector();
  for (let attempt = 0; attempt < 3 && inspectorRows.length === 0; attempt += 1) {
    tabOk = (await hasId("framework.panel.inspection")) ? await clickId("framework.panel.inspection").then(() => true).catch(() => false) : false;
    await page.waitForTimeout(2500);
    inspectorRows = await readInspector();
  }
  // 🎯️ The inspector reads the FIRST selected node, so this step must really leave the slider — and
  // only the slider — selected. The wire-cut two steps up leaves a handle standing in the graph
  // domain, and a `replace` merge in the `node` granularity does not clear the `handle` one, so the
  // inspector was answering about a neuron and truthfully painting no value control for it.
  let selectedSlider = null;
  for (let attempt = 0; attempt < 3 && slider && selectedSlider === null; attempt += 1) {
    await clearSelection();
    await page.locator(`[data-slot="panel"] [id="panel:procedural-play-document/${slider.id}"]`).first().click({ timeout: 8000 }).catch((e) => lines.push(`inspector row click ${String(e).slice(0, 160)}`));
    await page.waitForTimeout(2500);
    inspectorRows = await readInspector();
    selectedSlider = inspectorRows.some((row) => row.id.endsWith("procedural-play-inspector.id") && row.text.includes(slider.id)) ? slider.id : null;
  }
  const field = page.locator('[data-slot="panel"] [id$="procedural-play-inspector.value.input"], [id="procedural-play-inspector.value.input"]').first();
  const present = (await field.count()) > 0;
  // 🖼️ "The preview RE-EVALUATES" is read as a change in its published mesh payload, which an EMPTY
  // preview cannot express: the earlier mode round trip and wire cut leave the edit preview's
  // `data-meshes-json` at `[]` for a while, and two empty payloads compare equal however well the
  // edit landed. Wait for the preview to carry geometry again before typing, and say so if it never
  // does rather than blaming the inspector for it.
  const editMeshes = (s) => s.previews.filter((p) => !/generate/.test(p.surfaceId ?? "")).reduce((sum, p) => sum + p.meshes, 0);
  let armed = await until(snap, (s) => editMeshes(s) > 0, 20);
  // 🔁️ Re-pick the SAME example from the navbar when the preview is empty. That is the one gesture
  // this app guarantees re-arms every attached preview (`setActiveExample` →
  // `rearm_attached_previews`), and the journey probe proves it for all eight examples — so an empty
  // preview here is a precondition this step can restore by itself rather than a reason to give up.
  let rearmed = false;
  if (editMeshes(armed) === 0) {
    // 🔁️ TWO hops, not one: re-picking the option the picker already shows dispatches nothing at all
    // (measured — no second `setActiveExample` reached the guest), so the example has to actually
    // change and change back for `rearm_attached_previews` to run.
    const pick = async (match) => {
      const opened = await page.locator('[id="playground.navbar.fixture"]').first().click({ timeout: 8000 }).then(() => true).catch((e) => { lines.push(`example picker ${String(e).slice(0, 160)}`); return false; });
      if (!opened) return false;
      await page.waitForTimeout(1200);
      const option = page.locator('[role="option"]').filter({ hasText: match }).first();
      if (!(await option.count())) { await page.keyboard.press("Escape"); lines.push(`example option missing ${match}`); return false; }
      await option.click({ timeout: 8000 }).catch((e) => lines.push(`example option ${String(e).slice(0, 160)}`));
      await page.waitForTimeout(6000);
      return true;
    };
    rearmed = (await pick(/Rectangle Extrude|Rechteck/)) && (await pick(/Hexagonal|Sechseckige/));
    if (rearmed) armed = await until(snap, (s) => editMeshes(s) > 0, 120);
  }
  const previewArmed = editMeshes(armed) > 0;
  const before = armed;
  let after = before; let typed = null; const mark = lines.length;
  if (present) {
    const old = Number(await field.inputValue().catch(() => "0"));
    typed = Number.isFinite(old) ? Number((old + 1).toFixed(3)) : 1;
    await field.fill(String(typed), { timeout: 8000 }).catch(() => {});
    await page.keyboard.press("Enter");
    await field.blur().catch(() => {});
    // 🖼️ "The preview re-evaluated" is a NEW payload that still carries geometry: the first thing a
    // re-armed chain publishes is the emptied preview it is about to refill, and stopping there would
    // call a preview that merely went blank a success.
    after = await until(snap, (s) => { const w = s.widgets.find((x) => x.id === (slider?.id ?? "")); return Boolean(w) && Number(w.value) === typed && JSON.stringify(previewBytes(s)) !== JSON.stringify(previewBytes(before)) && editMeshes(s) > 0; }, 120);
  }
  const widgetAfter = after.widgets.find((x) => x.id === (slider?.id ?? "")) ?? null;
  const valueMoved = widgetAfter != null && typed != null && Number(widgetAfter.value) === typed;
  const previewMoved = JSON.stringify(previewBytes(after)) !== JSON.stringify(previewBytes(before)) && editMeshes(after) > 0;
  // ✏️ TWO subjects, two verdicts. The inspector's own subject is that its number control exists and
  // EDITS THE DOCUMENT; whether the preview then re-evaluates belongs to the preview-evaluation
  // pipeline, which is a different owner (`🧵️preview-eval`) and a different failure. Reporting them as
  // one line made a green inspector read as a red one and hid which half was broken.
  await note("inspection-edit", present && valueMoved, { tabOk, docTabOk, present, selectedSlider, inspectorRows, sliderId: slider?.id ?? null, typed, widgetAfter, invoked: invoked(mark), valueMoved });
  await note("inspection-preview-rearm", previewArmed && previewMoved, { previewArmed, rearmed, valueMoved, typed, bytesBefore: previewBytes(before), bytesAfter: previewBytes(after), previewMoved, toolRunStarts: lines.filter((line) => line.includes('"actionId":"toolRunStart"')).length });
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
  // 🪟️ A panel tab re-press folds its panel, but only when that tab is already the ACTIVE one — a
  // single pass over three tabs is three fresh PICKS and folds nothing, which is how this step used to
  // run with all three panels still covering the window chrome it then tried to click.
  for (const tab of ["framework.panel.catalogue", "framework.panel.inspection", "framework.panel.artifact"]) {
    await clickId(tab).catch(() => {});
    await page.waitForTimeout(900);
    await clickId(tab).catch(() => {});
    await page.waitForTimeout(900);
  }
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
  // 🇩🇪️ EVERY row the pane paints, not the five this probe used to name. Hardcoding pairs meant a
  // verb added later was never checked in German at all — 5 of ~38 were (`📓️window-coverage-audit-
  // 2026-09-14.md` §5 item 6). The English pass is the oracle: a row whose German text is byte-equal
  // to its English text was never translated, and a row that vanished between the passes is a
  // different defect again.
  const englishById = Object.fromEntries(english.rows.map((r) => [r.id, trim(r.label)]));
  const germanById = Object.fromEntries(german.rows.map((r) => [r.id, trim(r.label)]));
  const ids = Object.keys(englishById);
  const stillEnglish = ids.filter((id) => englishById[id] && germanById[id] === englishById[id] && !ACTION_DE_IDENTICAL_BY_DESIGN.includes(id.replace(/^action\./, "")));
  const vanished = ids.filter((id) => !(id in germanById));
  const unlabelled = ids.filter((id) => !englishById[id] || englishById[id] === id.replace(/^action\./, ""));
  await note("actions-pane-de", settings && ids.length > 0 && decided.length > 0 && decided.every((c) => c.ok) && stillEnglish.length === 0 && vanished.length === 0 && unlabelled.length === 0, {
    settings,
    rows: ids.length,
    spotChecks: checks,
    spotDecided: decided.length,
    stillEnglish: stillEnglish.map((id) => `${id}=${germanById[id]}`),
    vanished,
    unlabelled,
    english,
    german,
  });
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

// ── 9. The addGeneration chord, in the mode whose window declares it ────────
{
  await page.keyboard.press("Meta+Alt+ArrowRight");
  await page.waitForTimeout(6000);
  // 🎯️ Focus the window that DECLARES the verb first: `addGeneration` sits in
  // `window_kind_action_refs(GENERATIONS, …)`, so a chord pressed while another window holds focus
  // has no owner to resolve against. Clicking the Generations window's own dock tab is what a user
  // does before reaching for its shortcut.
  // 🎯️ The Generations window is a TREE, not a canvas: a user focuses it by clicking a row inside it.
  // Its window root carries no `data-surface-id`, only the DOM id `generation3d-generations`.
  const focusTargets = ['[id="generation3d-generations"] [role="treeitem"]', '[id="generation3d-generations"] [data-slot="tree-item-row"]', '[id="generation3d-generations"]'];
  let focused = false;
  for (const selector of focusTargets) {
    if (!(await page.locator(selector).count())) continue;
    focused = await page.locator(selector).first().click({ timeout: 8000, force: true }).then(() => true).catch((e) => { lines.push(`focus ${selector} ${String(e).slice(0, 120)}`); return false; });
    if (focused) break;
  }
  await page.waitForTimeout(2500);
  const activeWindow = (await snap()).activeWindow;
  const mountedWindows = await page.evaluate(() => ({ surfaces: [...document.querySelectorAll("[data-surface-id]")].map((el) => el.getAttribute("data-surface-id")), windows: [...document.querySelectorAll('[data-slot="window"]')].map((el) => el.id), tabs: [...document.querySelectorAll('[data-slot="mode-dock-tab"]')].map((el) => el.getAttribute("data-window-id")) }));
  const before = await snap();
  const mark = lines.length;
  await page.keyboard.press("Meta+Shift+g");
  const after = await until(snap, (s) => s.previews.some((p) => /generate/.test(p.surfaceId ?? "") && p.meshes > 0), 90);
  const chordInvoked = invoked(mark);
  await page.keyboard.press("Meta+Alt+ArrowLeft");
  await page.waitForTimeout(4000);
  await note("generate-chord", chordInvoked.includes("addGeneration"), { chord: "mod+shift+g", focusedGenerations: focused, activeWindow, mountedWindows, chordInvoked, previewsBefore: before.previews, previewsAfter: after.previews });
}

writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log(`[DEBUG] GAPS DONE ${results.steps.filter((s) => s.ok).length}/${results.steps.length}`);
await browser.close();
