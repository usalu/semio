/** ⌨️ Runtime proof for the editor's keyboard surface: every declared chord pressed in the state that
 * OWNS it, and every step decided by the effect the chord must have — never by its exit code.
 *
 * 🪪️ Each row carries its own `ok` and the `oracle` sentence it was decided by, so the battery reads a
 * verdict instead of inventing one. Three rows used to ride as `undecided` and are now real:
 *
 * - `baseline` — no chord is pressed, so the answer is the PRE-state: the preview window published a
 *   status and nothing was invoked. A shell that dispatches without a gesture is the defect this forbids.
 * - `add-generation` (`mod+shift+g`) — `addGeneration` is declared on the Generations window, which only
 *   GENERATE mode mounts. The probe switches modes first and decides the chord on the roster growing by
 *   one, so an app-wide chord that never resolves to its owning window is a red row, not an exemption.
 * - `cancel-preview-eval` (`mod+.`, `TOOL_RUN_ABORT_CHORD`) — pressed while an evaluation is genuinely in
 *   flight (`cancellable: true` on the preview's published status, the slowest example loaded), and
 *   decided by `🧫️fixtures/🛑️preview-cancel.json`'s own law: a cancel stamps `phase: "cancelled"` and
 *   leaves `cancellable` false.
 *
 * The mode-independent chords keep their original oracle — the `performInvocation` line the host emits —
 * because reaching NOTHING is exactly the finding for an unbound chord.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=keys-1 bun 🐍️editor-verbs-keyboard-probe.mjs
 * @see 🐍️react-battery.mjs (`keyboard-verbs`), 🐍️generate-mode-probe.mjs, 🐍️cancel-preview-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", "editor-verbs", process.env.SEMIO_PROBE_OUT ?? "keys");
const bootWait = Number(process.env.SEMIO_PROBE_BOOT_WAIT ?? 180);
const slowExample = process.env.SEMIO_PROBE_PICK ?? "Sphere Cut With Torus";
const armWait = Number(process.env.SEMIO_PROBE_ARM_WAIT ?? 120);
const cancelWait = Number(process.env.SEMIO_PROBE_CANCEL_WAIT ?? 30);
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.addInitScript(() => { try { localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 2000)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 1500)}`));

/** 📈️ Every surface the keyboard lane is written over, straight off the published DOM contracts. */
const snap = () => page.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  const hosts = [...document.querySelectorAll("[data-status-json]")].map((el) => ({
    surfaceId: el.getAttribute("data-surface-id"),
    showMode: el.getAttribute("data-show-mode"),
    lodMode: el.getAttribute("data-lod-mode"),
    status: parse(el.getAttribute("data-status-json")),
  }));
  const history = document.querySelector("[data-history-json]");
  const field = document.querySelector('[data-slot="panel"] input[type="number"]');
  const previewHost = document.querySelector('[data-surface-id="window:procedural-preview"]');
  const cancel = document.querySelector('[data-slot="world-compute-cancel"]');
  const rows = [...document.querySelectorAll('[id*="procedural3d-play-generate.generation."]')].filter((el) => !el.id.endsWith(".rename")).map((el) => el.id);
  return {
    hosts,
    generationRows: rows,
    history: history ? parse(history.getAttribute("data-history-json")) : null,
    inspectorValue: field instanceof HTMLInputElement ? field.value : null,
    previewHostCarriesMeshes: previewHost?.hasAttribute("data-meshes-json") ?? false,
    /** 📐️ The delivered geometry's own bounding box, rounded — the comparable the armed-history rows
     * are stated in.
     *
     * NOT a count: raising `Column Height` from 6 to 8 re-extrudes the same topology, so the vertex and
     * triangle totals are byte-identical before and after the edit and an edit that really reached the
     * kernel would read as no change at all. The BOX moves, and it moves back on undo and forward again
     * on redo, which is what makes these rows a product assertion. Read off EVERY element carrying
     * `data-meshes-json` rather than off the preview surface's own node: the mesh payload and
     * `data-status-json` are not always on the same element (measured — the surface node answered an
     * empty payload while the meshes were delivered on a sibling). */
    previewBox: (() => {
      const min = [Infinity, Infinity, Infinity];
      const max = [-Infinity, -Infinity, -Infinity];
      for (const el of document.querySelectorAll("[data-meshes-json]")) {
        let meshes = [];
        try { const parsedMeshes = JSON.parse(el.getAttribute("data-meshes-json") ?? "[]"); meshes = Array.isArray(parsedMeshes) ? parsedMeshes : []; } catch { meshes = []; }
        for (const mesh of meshes) {
          // 📐️ `positions` FIRST, `edgePositions` only when it is EMPTY — a wire mesh publishes
          // `positions: []` beside a full `edgePositions`, and `positions ?? edgePositions` keeps the
          // empty array (it is not nullish), which is how the box read `null` on a painted preview.
          const solid = mesh?.data?.positions ?? [];
          const positions = solid.length > 0 ? solid : (mesh?.data?.edgePositions ?? []);
          for (let i = 0; i + 2 < positions.length; i += 3) for (let axis = 0; axis < 3; axis += 1) {
            const v = positions[i + axis];
            if (v < min[axis]) min[axis] = v;
            if (v > max[axis]) max[axis] = v;
          }
        }
      }
      return Number.isFinite(min[0]) ? [...min, ...max].map((v) => Math.round(v * 1000) / 1000).join(",") : null;
    })(),
    /** 🔢️ FNV-1a over every delivered mesh payload — the discriminator the armed-history rows assert.
     * A slider edit re-extrudes the SAME topology, so counts do not move and even the bounding box can
     * stay put for an edit that only changes interior coordinates; the payload itself always moves. */
    previewDigest: (() => {
      let hash = 0x811c9dc5;
      let seen = 0;
      for (const el of document.querySelectorAll("[data-meshes-json]")) {
        const raw = el.getAttribute("data-meshes-json") ?? "";
        if (raw.length === 0) continue;
        seen += raw.length;
        for (let i = 0; i < raw.length; i += 1) {
          hash ^= raw.charCodeAt(i);
          hash = Math.imul(hash, 0x01000193) >>> 0;
        }
      }
      return seen === 0 ? null : `${hash.toString(16)}:${seen}`;
    })(),
    /** 🎚️ The edited widget's own value straight off the document the graph publishes — panel-independent,
     * unlike `inspectorValue`, which is `null` whenever the Inspection panel is not the open one. */
    fixtureValues: (() => {
      try {
        const host = document.querySelector('[data-surface-id="window:procedural-main"]');
        const fixture = JSON.parse(host?.getAttribute("data-fixture-json") ?? "null");
        return Object.fromEntries((fixture?.widgets ?? []).filter((w) => w && w.kind === "inputSlider").map((w) => [w.id, w.value]));
      } catch { return null; }
    })(),
    cancelButton: cancel ? { action: cancel.getAttribute("data-cancel-action") } : null,
    modes: [...document.querySelectorAll("[data-show-mode],[data-lod-mode]")].map((el) => ({
      slot: el.getAttribute("data-slot"), show: el.getAttribute("data-show-mode"), lod: el.getAttribute("data-lod-mode"),
    })),
  };
});

const results = [];
const invocationsSince = (mark) => lines.slice(mark).filter((l) => l.includes("performInvocation") && !l.includes("settled")).map((l) => (l.match(/"actionId":"([^"]+)"/) ?? [])[1]).filter(Boolean);
const surface = (s, id) => s.hosts.find((h) => h.surfaceId === id) ?? null;
const editPreview = (s) => surface(s, "window:procedural-preview");
const generatePreview = (s) => surface(s, "window:generation3d-generate-preview");

/** ⏱️ Polls the published DOM contract until `done` answers, then returns the last snapshot. */
const until = async (label, seconds, done) => {
  let s = await snap();
  for (let i = 0; i < seconds * 4 && !done(s); i += 1) {
    await page.waitForTimeout(250);
    s = await snap();
  }
  console.log(`[DEBUG] until ${label} settled=${done(s)} t=${Date.now() - t0}`);
  return s;
};

/** 🧾️ Records one row: its chord, what it invoked, the oracle sentence and the verdict that sentence gives. */
const record = async (label, chord, snapshot, ok, oracle, evidence = {}) => {
  const entry = { label, chord: chord ?? null, at: Date.now() - t0, invoked: evidence.invoked ?? [], ok, oracle, evidence, history: snapshot.history, modes: snapshot.modes, cancelButton: snapshot.cancelButton };
  results.push(entry);
  console.log(`[DEBUG] ${label} ok=${ok} ${JSON.stringify(entry).slice(0, 900)}`);
  await page.screenshot({ path: join(outDir, `${results.length}-${label.replace(/[^a-z0-9]+/gi, "-")}.png`) });
  return entry;
};

/** ⏪️ `mod+z` / `mod+shift+z`, decided by the FRAMEWORK HISTORY they are supposed to move, not by the
 * invocation they mint.
 *
 * The shell publishes its history projection on `data-history-json` (`shellHistoryCursorDomV1`,
 * `🛠️ShellHelpers/🟦️.tsx`) — cursor, `canUndo`/`canRedo` and the labels the two presses would act on.
 * Until that attribute existed these two rows could only watch an action id cross the console, which a
 * chord bound to nothing at all would also produce (`📓️react-current-tree-battery-2026-09-14.md` §6,
 * `historyJsonPublished: false`). An `armed` row is recorded alongside so a red says whether the CHORD
 * missed or the history was empty when it was pressed. */
const historyChordStep = async (label, chord, direction) => {
  const before = (await snap()).history;
  const armed = Boolean(before && (direction === "undo" ? before.canUndo : before.canRedo));
  await record(`${label}-armed`, null, await snap(), Boolean(before), "the shell publishes its history cursor on data-history-json", { history: before, armed });
  const mark = lines.length;
  await page.keyboard.press(chord);
  const settled = await until(`${label} history`, 20, (s) => !armed || (s.history != null && before != null && s.history.cursor !== before.cursor));
  const invoked = invocationsSince(mark);
  const after = settled.history;
  const moved = Boolean(before && after && (direction === "undo" ? after.cursor < before.cursor : after.cursor > before.cursor));
  const ok = invoked.includes(direction) && (armed ? moved : Boolean(after) && after.cursor === before.cursor);
  await record(label, chord, settled, ok, armed
    ? `the chord invokes ${direction} AND the published history cursor moves ${direction === "undo" ? "back" : "forward"}`
    : `the published history had nothing to ${direction}, so the chord must invoke ${direction} and leave the cursor where it is`,
    { invoked, armed, cursorBefore: before?.cursor ?? null, cursorAfter: after?.cursor ?? null, moved, undoLabel: before?.undoLabel ?? null, redoLabel: before?.redoLabel ?? null });
  return settled;
};

/** 🧘️ Polls until the edit preview is QUIET and its delivered box has stopped moving, then applies
 * `accept`.
 *
 * 🐛️ A plain "the payload differs" wait returns the moment the FIRST partial delivery of a new
 * evaluation lands: measured here as `box "-0.5,-0.433,0,0.5,0.433,0"` with a 442-char payload — the
 * profile wire alone, the extruded solid still computing. The undo pressed on that reading then raced
 * the edit's own convergence, and the height-8 geometry arrived AFTER the undo (`box …,0.433,8`), so
 * both armed rows read the wrong document. Three consecutive identical boxes on a preview that is not
 * cancellable is the settle this lane asserts against. */
const settledBox = async (label, seconds, accept) => {
  let last = null;
  let stable = 0;
  let snapshot = await snap();
  for (let i = 0; i < seconds * 2; i += 1) {
    snapshot = await snap();
    const quiet = editPreview(snapshot)?.status?.cancellable !== true && snapshot.previewBox !== null;
    stable = quiet && snapshot.previewBox === last ? stable + 1 : 0;
    last = quiet ? snapshot.previewBox : null;
    if (stable >= 3 && accept(snapshot)) break;
    await page.waitForTimeout(500);
  }
  console.log(`[DEBUG] settledBox ${label} stable=${stable} box=${snapshot.previewBox} t=${Date.now() - t0}`);
  return snapshot;
};

/** ⌨️ A mode-independent chord: pressed where it is already bound, decided by the invocation it mints. */
const chordStep = async (label, chord) => {
  const mark = lines.length;
  await page.keyboard.press(chord);
  await page.waitForTimeout(2500);
  const s = await snap();
  const invoked = invocationsSince(mark);
  await record(label, chord, s, invoked.length > 0, "the chord reaches an action: the host emits a performInvocation for it", { invoked });
  return s;
};

await page.goto(url, { waitUntil: "domcontentloaded" });

/** ⏳️ The playground boots ~20 wasm plugins; wait for the preview to publish a status at all. */
for (let i = 0; i < bootWait; i++) {
  const s = await snap();
  if (s.hosts.some((h) => h.status)) break;
  await page.waitForTimeout(1000);
}
await page.waitForTimeout(4000);
/** 🖱️ Focus the shell so chords reach the app, without clicking any verb under test.
 * `SEMIO_PROBE_FOCUS` picks the window to focus first: a keybinding for a WINDOW-OWNED action
 * (`window_kind_action_refs`) only resolves while that window has focus, so the focused window is
 * part of the result, not an incidental detail. */
const focusSelector = process.env.SEMIO_PROBE_FOCUS;
if (focusSelector) {
  const target = await page.$(focusSelector);
  const box = target ? await target.boundingBox() : null;
  if (!box) console.log(`[DEBUG] focus selector matched nothing clickable: ${focusSelector}`);
  else {
    await page.mouse.click(box.x + box.width / 2, box.y + box.height / 2);
    console.log(`[DEBUG] focused ${focusSelector} at ${Math.round(box.x + box.width / 2)},${Math.round(box.y + box.height / 2)}`);
  }
} else await page.mouse.click(720, 500);
await page.waitForTimeout(1500);

//#region 🧭️Baseline
/** ⌨️ Every verb a chord under test can mint. The baseline forbids exactly these without a gesture;
 * the app's own self-driven evaluation (`toolRunStart`, `flowEvalTick`, `toolRunPace`, the refreshes)
 * is not a gesture and runs at boot by design, so counting ALL invocations scored a converging
 * preview as a phantom keypress. */
const GESTURE_VERBS = ["cycleShowMode", "cycleLodMode", "addGeneration", "undo", "redo", "toolRunAbort"];
const baselineMark = lines.length;
await page.waitForTimeout(2500);
const baseline = await snap();
const baselineInvoked = invocationsSince(baselineMark);
const baselineGestures = baselineInvoked.filter((verb) => GESTURE_VERBS.includes(verb));
await record("baseline", null, baseline, editPreview(baseline)?.status != null && baselineGestures.length === 0, "with no gesture the preview window publishes a status and no chord-bound verb is invoked", { invoked: baselineInvoked, gestures: baselineGestures, previewStatus: editPreview(baseline)?.status ?? null });
//#endregion

//#region ⌨️Mode-independent chords
for (const [label, chord] of [["cycle-show-mode", "Control+Alt+d"], ["cycle-show-mode-again", "Control+Alt+d"], ["cycle-lod-mode", "Control+Alt+k"]]) await chordStep(label, chord);
await historyChordStep("undo", "Control+z", "undo");
await historyChordStep("redo", "Control+Shift+z", "redo");
//#endregion

//#region ⏪️ undo and redo against an ARMED history
/** 🗂️ Opens a panel and confirms its own body is on screen. A tab TOGGLES, so one click can leave the
 * panel shut; the body marker is what tells a shut panel from an empty one. */
const openPanelTab = async (id, marker) => {
  const tab = page.locator(`button#${id.replace(/\./gu, "\\.")}`);
  if ((await tab.count()) === 0) return false;
  for (let attempt = 0; attempt < 3; attempt += 1) {
    const box = await tab.first().boundingBox().catch(() => null);
    await tab.first().click({ position: { x: 8, y: Math.round((box?.height ?? 22) / 2) }, timeout: 8000 }).catch((e) => lines.push(`tab ${id} ${String(e).slice(0, 120)}`));
    await page.waitForTimeout(2200);
    if ((await page.evaluate((m) => document.querySelectorAll(`[id*="${m}"]`).length, marker)) > 0) return true;
  }
  return false;
};

/** ⏪️ The two chords, pressed on a history that HAS something to act on.
 *
 * The rows above press `mod+z`/`mod+shift+z` on this app's quiet boot, where the only entries are
 * config snapshots and the cursor legitimately stays put — so the ARMED branch of those rows had never
 * been exercised at all (`📓️react-remaining-reds-2026-09-15.md` §12). This drives a real document edit
 * first (a value typed into the Inspection panel's own field, which is what a user does), then requires
 * of undo and redo that BOTH the published cursor and the delivered geometry move the right way.
 */
{
  const artifactOpen = await openPanelTab("framework.panel.artifact", "procedural-play-graph");
  const row = page.locator('[data-slot="panel"] [role="treeitem"]').filter({ hasText: /Column Height|Profile Radius|Side Count/u }).first();
  const rowFound = (await row.count()) > 0;
  if (rowFound) await row.click({ timeout: 8000 }).catch((e) => lines.push(`armed row ${String(e).slice(0, 140)}`));
  await page.waitForTimeout(2500);
  const inspectionOpen = await openPanelTab("framework.panel.inspection", "procedural-play-inspector");
  const settledBefore = await settledBox("before the edit", 90, (s) => s.previewBox !== null);
  const originalValue = settledBefore.inspectorValue;
  const originalBox = settledBefore.previewBox;
  const originalDigest = settledBefore.previewDigest;
  await record("edit-target", null, settledBefore, artifactOpen && rowFound && inspectionOpen && originalValue !== null && originalBox !== null, "a widget is selected and its own number field is on screen over a painted preview", { invoked: [], artifactOpen, rowFound, inspectionOpen, originalValue, originalBox, originalDigest, fixtureValues: settledBefore.fixtureValues, previewHostCarriesMeshes: settledBefore.previewHostCarriesMeshes });

  const editMark = lines.length;
  const field = page.locator('[data-slot="panel"] input[type="number"]').first();
  const edited = originalValue === null ? null : String(Number(originalValue) + 2);
  if (edited !== null) {
    await field.fill(edited, { timeout: 8000 }).catch((e) => lines.push(`armed fill ${String(e).slice(0, 140)}`));
    await page.keyboard.press("Enter");
    await field.blur().catch(() => {});
  }
  const afterEdit = await settledBox("after the edit", 150, (s) => s.history?.canUndo === true && s.previewBox !== originalBox);
  await record("edit-arms-history", null, afterEdit, afterEdit.history?.canUndo === true && afterEdit.previewBox !== null && afterEdit.previewBox !== originalBox, "a typed value is a user edit: the shell's history arms canUndo and the preview SETTLES on a different delivered extent", { invoked: invocationsSince(editMark), edited, originalDigest, digest: afterEdit.previewDigest, originalBox, box: afterEdit.previewBox, fixtureValues: afterEdit.fixtureValues, history: afterEdit.history });
  const editedDigest = afterEdit.previewDigest;
  const editedBox = afterEdit.previewBox;
  const editedCursor = afterEdit.history?.cursor ?? null;

  /** ⏪️ `canRedo` is the direction discriminator, NOT the cursor's sign.
   *
   * `shellHistoryCursorDomV1` projects an APPEND-ONLY reduction of every `HistoryPatch` the guest
   * sent, so an undo APPENDS its own entry and the cursor goes UP — measured here 17 → 18 on a real
   * undo, and it also climbs on its own while the probe hovers and selects. The cursor is therefore
   * asserted as "it moved", and the direction is read where the shell actually states it: only an undo
   * can leave something to redo. The geometry is the substantive half. */
  const undoMark = lines.length;
  await page.keyboard.press("Control+z");
  const undone = await settledBox("after undo", 120, (s) => s.history?.canRedo === true && s.previewBox === originalBox);
  await record("undo-armed-history", "Control+z", undone, invocationsSince(undoMark).includes("undo") && undone.history?.canRedo === true && (undone.history?.cursor ?? null) !== editedCursor && undone.previewBox === originalBox, "on an armed history the chord invokes undo, the shell's cursor moves, it leaves something to REDO and the preview settles back on the extent it delivered before the edit", { invoked: invocationsSince(undoMark), cursorBefore: editedCursor, cursorAfter: undone.history?.cursor ?? null, canRedo: undone.history?.canRedo ?? null, digestBefore: editedDigest, digest: undone.previewDigest, originalDigest, boxBefore: editedBox, box: undone.previewBox, originalBox, fixtureValues: undone.fixtureValues });
  const undoneCursor = undone.history?.cursor ?? null;

  const redoMark = lines.length;
  await page.keyboard.press("Control+Shift+z");
  const redone = await settledBox("after redo", 120, (s) => (s.history?.cursor ?? null) !== undoneCursor && s.previewBox === editedBox);
  await record("redo-armed-history", "Control+Shift+z", redone, invocationsSince(redoMark).includes("redo") && (redone.history?.cursor ?? null) !== undoneCursor && redone.previewBox === editedBox, "on a history that has just been undone the chord invokes redo, the shell's cursor moves again and the preview settles back on the EDITED extent", { invoked: invocationsSince(redoMark), cursorBefore: undoneCursor, cursorAfter: redone.history?.cursor ?? null, digest: redone.previewDigest, editedDigest, originalDigest, box: redone.previewBox, editedBox, originalBox, fixtureValues: redone.fixtureValues });

  // 🛟️ Leave the document as it was found, so the later rows drive the example they expect.
  await page.keyboard.press("Control+z");
  await page.waitForTimeout(3000);
}
//#endregion

//#region ➕️add-generation, in the mode that owns the Generations window
await page.keyboard.press("Meta+Alt+ArrowRight");
const generate = await until("generate mode mounts", 40, (s) => generatePreview(s) !== null);
await record("generate-mode", "Meta+Alt+ArrowRight", generate, generatePreview(generate) !== null, "the mode chord mounts the generate preview window, so the Generations window exists to own addGeneration", { invoked: [], surfaces: generate.hosts.map((h) => h.surfaceId) });

const rowsBefore = generate.generationRows;
const addMark = lines.length;
await page.keyboard.press("Control+Shift+g");
const added = await until("a generation is added", 40, (s) => s.generationRows.length > rowsBefore.length);
const addInvoked = invocationsSince(addMark);
await record("add-generation", "Control+Shift+g", added, addInvoked.includes("addGeneration") && added.generationRows.length === rowsBefore.length + 1, "pressed in generate mode the chord resolves to the Generations window: addGeneration is invoked and the roster grows by exactly one row", { invoked: addInvoked, rowsBefore, rowsAfter: added.generationRows });
//#endregion

//#region 🛑️cancel-preview-eval, pressed on an evaluation that is genuinely in flight
await page.keyboard.press("Meta+Alt+ArrowLeft");
await until("edit mode returns", 40, (s) => editPreview(s) !== null);
try {
  await page.locator('[role="combobox"]').first().click({ timeout: 10000 });
  await page.waitForTimeout(400);
  await page.locator('[role="option"]').filter({ hasText: slowExample }).first().click({ timeout: 10000 });
} catch (error) {
  console.log(`[DEBUG] example switch unavailable: ${String(error).slice(0, 200)}`);
}
const armed = await until("an evaluation is cancellable", armWait, (s) => editPreview(s)?.status?.cancellable === true);
const armedStatus = editPreview(armed)?.status ?? null;
await record("cancellable-evaluation-armed", null, armed, armedStatus?.cancellable === true, "loading the slowest example puts work in flight: the preview publishes cancellable: true", { invoked: [], status: armedStatus });

const cancelMark = lines.length;
await page.keyboard.press("Control+.");
const cancelled = await until("the evaluation reports cancelled", cancelWait, (s) => editPreview(s)?.status?.phase === "cancelled");
const cancelInvoked = invocationsSince(cancelMark);
const cancelledStatus = editPreview(cancelled)?.status ?? null;
await record("cancel-preview-eval", "Control+.", cancelled, cancelInvoked.includes("toolRunAbort") && cancelledStatus?.phase === "cancelled" && cancelledStatus?.cancellable === false, "pressed on work in flight the chord aborts the run: toolRunAbort is invoked and the preview stamps phase cancelled with cancellable false", { invoked: cancelInvoked, armedStatus, status: cancelledStatus });

/** 🖱️ The SAME verb through the affordance the status itself declares (`cancelAction` + `cancelArgs`),
 * on a freshly armed evaluation. It is the discriminator the chord row needs: the chord dispatches
 * `toolRunAbort` with no run address, the button dispatches it with `runId`/`generation`, so a row
 * where the button cancels and the chord does not names the chord's addressing as the defect rather
 * than the cancellation itself. */
if (cancelledStatus?.phase !== "cancelled") {
  try {
    await page.locator('[role="combobox"]').first().click({ timeout: 10000 });
    await page.waitForTimeout(400);
    await page.locator('[role="option"]').filter({ hasText: slowExample }).first().click({ timeout: 10000 });
  } catch (error) {
    console.log(`[DEBUG] example re-switch unavailable: ${String(error).slice(0, 200)}`);
  }
  const rearmed = await until("a second evaluation is cancellable", armWait, (s) => editPreview(s)?.status?.cancellable === true);
  const buttonMark = lines.length;
  const clicked = await page.locator('[data-slot="world-compute-cancel"]').first().click({ timeout: 5000 }).then(() => true).catch(() => false);
  const buttonCancelled = await until("the button cancel reports cancelled", cancelWait, (s) => editPreview(s)?.status?.phase === "cancelled");
  const buttonStatus = editPreview(buttonCancelled)?.status ?? null;
  await record("cancel-affordance-discriminator", null, buttonCancelled, clicked && buttonStatus?.phase === "cancelled" && buttonStatus?.cancellable === false, "the status's OWN cancel affordance, carrying runId and generation, stamps phase cancelled with cancellable false", { invoked: invocationsSince(buttonMark), clicked, armedStatus: editPreview(rearmed)?.status ?? null, status: buttonStatus });
}
//#endregion

writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log(`[DEBUG] DONE ${results.length} steps, ${results.filter((r) => r.ok).length} green; console ${lines.length} lines -> ${outDir}`);
await browser.close();
