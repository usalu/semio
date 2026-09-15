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
  const cancel = document.querySelector('[data-slot="world-compute-cancel"]');
  const rows = [...document.querySelectorAll('[id*="procedural3d-play-generate.generation."]')].filter((el) => !el.id.endsWith(".rename")).map((el) => el.id);
  return {
    hosts,
    generationRows: rows,
    history: history ? parse(history.getAttribute("data-history-json")) : null,
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
