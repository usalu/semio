/** 🀄️ WFC acceptance inputs for the repository-owned browser journey. */
import { readFileSync, mkdirSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { createHash } from "node:crypto";
import type { TimeTravelScenarioV1 } from "@semio-tech/framework-time-travel-ts/testing/scenario";
import { page, renderer, OUT, stamp, currentLocale, cancelled, expectedReloads, setExpectedReloads, setProbeReloadAt } from "@semio-tech/framework-time-travel-ts/testing/state";
import { verdict, note } from "@semio-tech/framework-time-travel-ts/testing/report";
import { waitUntil, sleep } from "@semio-tech/framework-time-travel-ts/testing/page";
import { openHistory } from "@semio-tech/framework-time-travel-ts/testing/history";
import { closePanels, openTab, waitForBoot } from "@semio-tech/framework-time-travel-ts/testing/chrome";
import { attachFolder } from "@semio-tech/framework-time-travel-ts/testing/history-session";
import { click, clickUncovered, fillStagedArgument, submitStagedVerb, unfoldActionsRail, readShell, witness } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts";

type Form = { action: string; args: Record<string, string>; window?: string };
type Workload = { forms: Form[]; slots?: { count: number; idPrefix: string; columns: number; spacing: number } };
type ReloadSurface = { window: string; surface: string; meaning: string };
type SurfaceWitness = { pixels: string; accessibility: string };
type Variant = { reloadSurfaces: ReloadSurface[]; variant: string; port: number; authorWindow: string; previewWindow: string; projection: "canvas" | "grid3d" | "wfc3d"; preview: string; forms: Form[]; workload: Workload; keep?: string; keepWindow?: string };
type Cases = { schema: string; timeoutMs: number; setupTimeoutMs: number; pauseHoldMs: number; variants: Variant[]; controlCoverage: { pause: string; abort: string; terminalRace: string } };
type Run = { scope: string; status: string; progress: string };
type Context = { folder: string; ledger: string[]; controlsPrepared: boolean };
const cases = JSON.parse(readFileSync(new URL("./wfc-browser-cases.json", import.meta.url), "utf8")) as Cases;
if (cases.schema !== "semio.wfc.browser-cases/v1" || cases.timeoutMs > 120000 || cases.setupTimeoutMs > 900000 || cases.controlCoverage.pause !== "observed-held-resumed" || cases.controlCoverage.abort !== "observed-active-aborted" || cases.controlCoverage.terminalRace !== "coverage-gap") throw new Error("WFC browser case contract");
const terminal = /^(Finalized|Abgeschlossen|Aborted|Abgebrochen|Faulted|Failed|Fehlgeschlagen)/u;
const finalized = /^(Finalized|Abgeschlossen)/u;
const ready = /^(Complete, ready to finalize|Fertig, bereit zum Abschließen)/u;

/** 🔭️ Chromium observes the public tool-run ARIA projection, independently of WFC's job implementation. */
async function readRun(): Promise<Run | null> {
  return page.evaluate(() => {
    const nodes = [...document.querySelectorAll('[data-ui-node-key^="framework.toolRun."][data-ui-node-key$=".status"]')].filter(node => !(node.getAttribute("data-ui-node-key") ?? "").startsWith("framework.toolRun.ready"));
    const node = nodes.at(-1);
    if (!node) return null;
    const scope = node.getAttribute("data-ui-node-key")!.slice(0, -7);
    const progress = document.querySelector('[data-ui-node-key="' + scope + '.progress"]');
    return { scope, status: (node.textContent ?? "").trim(), progress: progress?.getAttribute("aria-valuetext") ?? progress?.getAttribute("aria-valuenow") ?? "" };
  });
}

async function control(action: string, scope?: string): Promise<boolean> {
  const key = scope ? '[data-ui-node-key="' + scope + "." + action + '"]' : '[data-ui-node-key$=".' + action + '"]';
  return await click(page, key) === "ok";
}

async function runPanel(): Promise<boolean> {
  return openTab("framework.panel.toolRun");
}

async function previewDigest(variant: Variant): Promise<string | null> {
  await closePanels();
  if (!(await activateWindow(variant.previewWindow))) return null;
  await page.mouse.move(0, 0);
  const surface = page.locator('[data-surface-id="' + variant.preview + '"]').first();
  if (!(await surface.count())) return null;
  return surfaceDigest(variant.preview);
}

async function surfaceDigest(id: string): Promise<string | null> {
  const surface = page.locator('[data-surface-id="' + id + '"]').first();
  if (!(await surface.count())) return null;
  let previous: string | null = null;
  const stable = await waitUntil(async () => {
    const current = createHash("sha256").update(await surface.screenshot({ timeout: 8000 })).digest("hex");
    const matches = current === previous;
    previous = current;
    return matches ? current : null;
  }, digest => digest !== null, 8000, 250);
  return stable.ok ? stable.value : null;
}

async function worldProjection(variant: Variant): Promise<{ solved: number; total: number } | null> {
  await closePanels();
  if (!(await activateWindow(variant.previewWindow))) return null;
  const surface = page.locator('[data-surface-id="' + variant.preview + '"]').first();
  if (!(await surface.count())) return null;
  const status = JSON.parse(await surface.getAttribute("data-status-json") ?? "null");
  if (variant.projection === "grid3d") return status && Number.isInteger(status.solved) && Number.isInteger(status.cells) ? { solved: status.solved, total: status.cells } : null;
  const counts = typeof status?.message === "string" ? /^Solved (\d+) of (\d+) slots\.$/u.exec(status.message) : null;
  if (counts) return { solved: Number(counts[1]), total: Number(counts[2]) };
  return status?.message === "Not solved yet." ? { solved: 0, total: 0 } : null;
}

async function projection(variant: Variant, empty: string | null, solved: boolean): Promise<boolean> {
  if (variant.projection === "canvas") {
    const pixels = await previewDigest(variant);
    return pixels !== null && empty !== null && (solved ? pixels !== empty : pixels === empty);
  }
  const state = await worldProjection(variant);
  return state !== null && (solved ? state.solved > 0 && state.solved === state.total : state.solved === 0);
}

async function refreshAfterEdit(variant: Variant, empty: string | null): Promise<void> {
  const seed = variant.forms.find(form => Object.keys(form.args).includes("seed"));
  if (!verdict(variant.variant + ".revision.seed-form-available", seed !== undefined) || seed === undefined) return;
  if (!(await forms(variant, [{ ...seed, args: { seed: "4002" } }], "same-extent-revision"))) return;
  const staleHidden = await waitUntil(() => projection(variant, empty, false), value => value, 10000, 250);
  verdict(variant.variant + ".revision.hides-prior-solution", staleHidden.ok);
  const scope = await start(variant);
  if (!verdict(variant.variant + ".revision.refill-starts", scope !== null) || scope === null) return;
  const deadline = Date.now() + cases.timeoutMs;
  let run: Run | null = null;
  while (Date.now() < deadline && !cancelled.aborted) {
    await runPanel();
    run = await readRun();
    if (!run || run.scope !== scope || terminal.test(run.status)) break;
    if (ready.test(run.status)) await control("toolRunFinalize", scope);
    await sleep(25);
  }
  verdict(variant.variant + ".revision.refill-completes", run?.scope === scope && finalized.test(run.status), { run });
  const restored = await waitUntil(() => projection(variant, empty, true), value => value, 10000, 250);
  verdict(variant.variant + ".revision.refill-renders-solution", restored.ok);
}


async function stage(key: string, value: string): Promise<string> {
  const slider = page.locator('[id$=".arg.' + key + '"][role="slider"], [id$=".arg.' + key + '"] [role="slider"]').first();
  if (!(await slider.count())) return fillStagedArgument(page, key, value, "__unused_live_id__");
  await slider.focus();
  let current = Number(await slider.getAttribute("aria-valuenow"));
  for (let turn = 0; turn < 512 && current !== Number(value) && !cancelled.aborted; turn += 1) {
    await slider.press(current < Number(value) ? "ArrowRight" : "ArrowLeft");
    const next = Number(await slider.getAttribute("aria-valuenow"));
    if (current === next) break;
    current = next;
  }
  return key + "=" + current;
}

async function activateWindow(prefix: string): Promise<boolean> {
  const selector = '[data-slot="mode-dock-tab"][data-window-id^="' + prefix + '"]';
  if (await clickUncovered(page, selector + ' [role="tab"]') !== "ok") return false;
  const active = await waitUntil(() => page.locator(selector + '[data-active="true"]').count(), count => count > 0, 10000, 100);
  return active.ok;
}

async function forms(variant: Variant, entries: Form[] = variant.forms, purpose = "author"): Promise<boolean> {
  const deadline = Date.now() + cases.setupTimeoutMs;
  for (const [index, form] of entries.entries()) {
    if (cancelled.aborted || Date.now() >= deadline) {
      verdict(variant.variant + "." + purpose + ".within-setup-bound", false, { prepared: index, requested: entries.length, boundMs: cases.setupTimeoutMs, cancelled: cancelled.aborted });
      return false;
    }
    await openHistory();
    const before = witness(await readShell(page));
    await closePanels();
    const focused = await activateWindow(form.window ?? variant.authorWindow);
    await unfoldActionsRail(page);
    const opened = await clickUncovered(page, '[data-slot="window-action-pane"] [id="action.' + form.action + '"]');
    const filled: string[] = [];
    if (focused && opened === "ok") for (const [key, value] of Object.entries(form.args)) filled.push(await stage(key, value));
    const seated = filled.length === Object.keys(form.args).length && filled.every((read, index) => read === Object.entries(form.args)[index]!.join("="));
    const name = variant.variant + "." + purpose + ".form." + index + "." + form.action;
    if (!focused || opened !== "ok" || !seated) {
      verdict(name, false, { focused, opened, filled, before, submitted: "not attempted: public form was not seated" });
      return false;
    }
    const submitted = await submitStagedVerb(page, form.action);
    await openHistory();
    const changed = await waitUntil(async () => witness(await readShell(page)), read => read.edits === before.edits + 1 || read.applied === before.applied + 1, 30000);
    if (!verdict(name, submitted === "ok" && changed.ok, { filled, before, after: changed.value, submitted })) return false;
  }
  return verdict(variant.variant + "." + purpose + ".within-setup-bound", Date.now() <= deadline, { prepared: entries.length, boundMs: cases.setupTimeoutMs });
}

/** 🧩️ Workload construction expands only public create-slot forms, never private guest state. */
export function controlWorkloadForms(variant: Variant, locale: string): Form[] {
  const entries = [...variant.workload.forms];
  const slots = variant.workload.slots;
  if (slots) for (let index = 0; index < slots.count; index += 1) entries.push({
    action: "create-slot",
    args: { id: slots.idPrefix + locale + "-" + index.toString().padStart(3, "0"), x: String((index % slots.columns) * slots.spacing), y: String(Math.floor(index / slots.columns) * slots.spacing), width: "1", height: "1" },
  });
  return entries;
}

async function start(variant: Variant): Promise<string | null> {
  await closePanels();
  if (!(await activateWindow(variant.authorWindow))) return null;
  await click(page, '[id="framework.category.tool"]');
  const tool = page.locator('[id="tool.fill"]').first();
  if (!(await tool.count())) return null;
  if (await tool.getAttribute("aria-pressed") !== "true") await tool.click({ timeout: 8000 });
  if (!(await runPanel())) return null;
  const previous = (await readRun())?.scope;
  if (!(await control("toolRunStart"))) return null;
  const began = await waitUntil(readRun, run => run !== null && run.scope !== previous, 10000, 16);
  return began.ok ? began.value!.scope : null;
}

async function solve(variant: Variant): Promise<void> {
  const empty = await previewDigest(variant);
  verdict(variant.variant + ".preview.visible-before-solve", empty !== null);
  await openHistory();
  const before = witness(await readShell(page));
  const startedAt = Date.now();
  const started = await start(variant);
  if (!verdict(variant.variant + ".fill.start", started !== null) || started === null) return;
  const scope = started;
  const observations = new Set<string>();
  let pauseRequested = false;
  let pauseObserved = false;
  let pauseRaceRecorded = false;
  let resumed = false;
  const deadline = startedAt + cases.timeoutMs;
  let run: Run | null = null;
  while (Date.now() < deadline && !cancelled.aborted) {
    run = await readRun();
    if (!run || run.scope !== scope) break;
    if (run.progress) observations.add(run.progress);
    if (!pauseRequested && !pauseRaceRecorded && (terminal.test(run.status) || ready.test(run.status))) {
      pauseRaceRecorded = true;
      note(variant.variant + ".fill.pause-raced-with-completion", { run, requested: false, coverage: "gap: the workload completed before a pause was observed" });
    }
    if (!pauseRequested && !terminal.test(run.status) && !ready.test(run.status)) {
      pauseRequested = await control("toolRunPause", scope);
      if (pauseRequested) {
        const held = await waitUntil(readRun, value => value?.scope === scope && (/^(Paused|Pausiert)/u.test(value.status) || terminal.test(value.status) || ready.test(value.status)), 5000, 25);
        await sleep(cases.pauseHoldMs);
        const later = await readRun();
        if (held.value?.progress) observations.add(held.value.progress);
        if (held.value && later && (terminal.test(held.value.status) || ready.test(held.value.status) || terminal.test(later.status) || ready.test(later.status))) {
          pauseRaceRecorded = true;
          note(variant.variant + ".fill.pause-raced-with-completion", { held: held.value, later, coverage: "gap: active pause/resume was not observed" });
        } else {
          pauseObserved = held.ok && held.value?.scope === scope && later?.scope === scope && /^(Paused|Pausiert)/u.test(held.value.status) && /^(Paused|Pausiert)/u.test(later.status) && held.value.progress === later.progress;
          verdict(variant.variant + ".fill.pause-holds", pauseObserved, { held: held.value, later });
          resumed = await control("toolRunResume", scope);
          verdict(variant.variant + ".fill.resume", resumed);
        }
      }
      run = await readRun();
      if (!run || run.scope !== scope) break;
    }
    if (ready.test(run.status)) await control("toolRunFinalize", scope);
    if (terminal.test(run.status)) break;
    await sleep(25);
  }
  if (!run || !finalized.test(run.status)) await control("toolRunAbort", scope);
  verdict(variant.variant + ".fill.completes-within-bound", Boolean(run && finalized.test(run.status) && Date.now() <= deadline), { run, boundMs: cases.timeoutMs });
  verdict(variant.variant + ".fill.progress-visible", observations.size > 0, { observations: [...observations].slice(0, 12) });
  verdict(variant.variant + ".fill.progress-advances", observations.size > 1, { observations: [...observations].slice(0, 12) });
  verdict(variant.variant + ".fill.pause-resume-observed", pauseObserved && resumed, { pauseRequested, pauseObserved, resumed });
  if (!pauseObserved || !resumed) note(variant.variant + ".fill.pause-coverage-gap", { reason: "positive paused hold and resume were not both observed on the configured workload; completion is separate coverage" });
  const rendered = await waitUntil(() => projection(variant, empty, true), value => value, 10000, 250);
  verdict(variant.variant + ".fill.renders-solution", rendered.ok);
  const digest = await previewDigest(variant);
  await runPanel();
  const dismissed = await control("toolRunDismiss", scope);
  const retained = await previewDigest(variant);
  verdict(variant.variant + ".fill.preview-retained-after-dismiss", digest !== null && dismissed && retained === digest, { digest, retained, dismissed, meaning: "rendered preview pixels only; assignment semantics are covered by native oracle tests" });
  await openHistory();
  const after = witness(await readShell(page));
  verdict(variant.variant + ".fill.read-only", after.edits === before.edits && after.applied === before.applied, { before, after });
  const abortScope = await start(variant);
  const abortState = await readRun();
  const active = abortScope !== null && abortState?.scope === abortScope && !terminal.test(abortState.status) && !ready.test(abortState.status);
  let abortObserved = false;
  if (abortScope && abortState?.scope === abortScope && (terminal.test(abortState.status) || ready.test(abortState.status))) {
    note(variant.variant + ".fill.abort-raced-with-completion", { run: abortState, coverage: "gap: active-work cancellation was not observed" });
    if (ready.test(abortState.status)) {
      const requested = await control("toolRunAbort", abortScope);
      const aborted = await waitUntil(readRun, value => value?.scope === abortScope && /^(Aborted|Abgebrochen)/u.test(value.status), 10000, 25);
      verdict(variant.variant + ".fill.abort-before-finalize", requested && aborted.ok, { before: abortState, run: aborted.value, coverage: "completed-result abort only; active-work cancellation is not claimed" });
    }
  } else if (active) {
    const requested = await control("toolRunAbort", abortScope!);
    const aborted = await waitUntil(readRun, value => value?.scope === abortScope && /^(Aborted|Abgebrochen)/u.test(value.status), 10000, 25);
    abortObserved = requested && aborted.ok;
    verdict(variant.variant + ".fill.abort", abortObserved, { before: abortState, run: aborted.value });
    await openHistory();
    const abortedHistory = witness(await readShell(page));
    verdict(variant.variant + ".fill.abort-writes-nothing", abortObserved && abortedHistory.edits === after.edits && abortedHistory.applied === after.applied, { before: after, after: abortedHistory });
    const afterAbort = await previewDigest(variant);
    verdict(variant.variant + ".fill.abort-preserves-prior-preview", afterAbort !== null && afterAbort === retained, { retained, afterAbort });
  }
  verdict(variant.variant + ".fill.active-abort-observed", active && abortObserved, { scope: abortScope, before: abortState });
  if (!active || !abortObserved) note(variant.variant + ".fill.abort-coverage-gap", { reason: "the configured workload was not observed active and then aborted; aborting a completed result is separate coverage" });
  await openHistory();
  const afterControls = witness(await readShell(page));
  verdict(variant.variant + ".fill.control-runs-read-only", afterControls.edits === after.edits && afterControls.applied === after.applied, { before: after, after: afterControls });
  await refreshAfterEdit(variant, empty);
  await openHistory();
  const afterRevision = witness(await readShell(page));
  if (variant.keep) {
    await closePanels();
    const focused = await activateWindow(variant.keepWindow!);
    await unfoldActionsRail(page);
    const kept = await clickUncovered(page, '[data-slot="window-action-pane"] [id="action.' + variant.keep + '"]');
    await openHistory();
    const pinned = await waitUntil(async () => witness(await readShell(page)), read => read.edits === afterRevision.edits + 1 || read.applied === afterRevision.applied + 1, 30000);
    verdict(variant.variant + ".keep-preview-authors-pins", focused && kept === "ok" && pinned.ok, { before: afterRevision, after: pinned.value });
  }
  note(variant.variant + ".solve-persistence", { authored: "model, seed and pins", preview: "ephemeral/derived", bitmapPinSolution: variant.keep ?? "not applicable" });
}

/** 🔎️ Public pixels and accessibility labels witness authored rendering without reading guest state. */
async function surfaceWitness(target: ReloadSurface): Promise<SurfaceWitness | null> {
  await closePanels();
  if (!(await activateWindow(target.window))) return null;
  await page.mouse.move(0, 0);
  const surface = page.locator('[data-surface-id="' + target.surface + '"]').first();
  if (!(await surface.count())) return null;
  const accessibility = await surface.evaluate((root: Element) => JSON.stringify([root, ...root.querySelectorAll('[role], [aria-label], [aria-valuetext], [data-ui-node-key]')].map(node => ({
    role: node.getAttribute("role"),
    label: node.getAttribute("aria-label"),
    value: node.getAttribute("aria-valuetext"),
    text: node.children.length === 0 ? (node.textContent ?? "").trim() : "",
  }))));
  const pixels = await surfaceDigest(target.surface);
  if (pixels === null) return null;
  return { pixels, accessibility };
}

async function reload(variant: Variant, context: Context): Promise<void> {
  context.folder = join(OUT, "wfc-folder-" + variant.variant + "-" + stamp + "-" + currentLocale);
  mkdirSync(context.folder, { recursive: true });
  const attached = await attachFolder(context.folder);
  const files = await waitUntil(async () => readdirSync(context.folder), names => names.length > 0, 30000);
  verdict(variant.variant + ".folder-written", attached.card && attached.attachButton && files.ok, { attached, entries: files.value });
  if (!files.ok) return;
  const surfaces: Array<{ target: ReloadSurface; before: SurfaceWitness | null }> = [];
  for (const target of variant.reloadSurfaces) surfaces.push({ target, before: await surfaceWitness(target) });
  await runPanel();
  const priorRun = await readRun();
  verdict(variant.variant + ".reload.ephemeral-run-witnessed", priorRun !== null, { priorRun });
  await openHistory();
  context.ledger = witness(await readShell(page)).ledger;
  setExpectedReloads(expectedReloads + 1);
  setProbeReloadAt(Date.now());
  await page.reload({ waitUntil: "domcontentloaded", timeout: 90000 });
  const booted = await waitForBoot("WFC persisted reload", 30, 1);
  await openHistory();
  const restored = await waitUntil(async () => witness(await readShell(page)), read => context.ledger.length > 0 && context.ledger.every(row => read.ledger.includes(row)), 30000);
  verdict(variant.variant + ".authored-history-survives-reload", booted && restored.ok, { expected: context.ledger, actual: restored.value.ledger });
  for (const { target, before } of surfaces) {
    const after = await waitUntil(() => surfaceWitness(target), value => before !== null && value !== null && value.pixels === before.pixels && value.accessibility === before.accessibility, 30000, 1000);
    verdict(variant.variant + ".reload.surface." + target.surface, booted && before !== null && after.ok, { target, before, after: after.value, meaning: target.meaning + "; exact hidden seed and unrendered fields are outside this observer" });
  }
  await runPanel();
  const reset = await waitUntil(readRun, value => value === null, 10000, 100);
  const priorScopeAbsent = priorRun !== null && await page.locator('[data-ui-node-key="' + priorRun.scope + '.status"]').count() === 0;
  verdict(variant.variant + ".reload.tool-run-reset", booted && priorRun !== null && reset.ok && priorScopeAbsent, { before: priorRun, after: reset.value, priorScopeAbsent, meaning: "ephemeral framework tool-run UI is absent after boot; cache contents are checked by native revision/render laws" });
  note(variant.variant + ".reload.observer-limits", { exactSeed: "not publicly rendered in every editor; canonical snapshot/SPR tests remain required", authored: "matching rendered geometry, labels and bitmap kept pins, plus replayed history", ephemeral: "prior tool-run projection removed; no private guest or base-pack-only decoding" });
}

/** 🧩️ One wrapper selects a catalog variant; the harness owns launch, locales, cancellation and reports. */
export function createWfcScenario(variantId: string): TimeTravelScenarioV1<Context> {
  const variant = cases.variants.find(row => row.variant === variantId);
  if (!variant) throw new Error("unknown WFC variant " + variantId);
  return {
    variant: variantId, defaultServes: { react: "http://127.0.0.1:" + variant.port + "/", wgpu: "http://127.0.0.1:" + variant.port + "/" },
    bootWindowPrefix: variant.authorWindow, editorNumbers: [], paletteQuery: { en: "WFC", de: "WFC" },
    stepNumbers: [101, 102, 103], order: [101, 102, 103],
    context: () => ({ folder: "", ledger: [], controlsPrepared: false }),
    prepare: async () => renderer === "react" ? null : { name: "wfc.scenario.react-required", prerequisite: "these public DOM observers target React; WGPU needs a mirror observer", exempt: [] },
    beforeStep: async () => {}, afterSteps: async () => {},
    steps: { 101: async context => { const authored = await forms(variant); context.controlsPrepared = authored && await forms(variant, controlWorkloadForms(variant, currentLocale), "control-workload"); }, 102: async context => { if (!verdict(variant.variant + ".controls.workload-ready", context.controlsPrepared)) return; try { await solve(variant); } finally { const run = await readRun().catch(() => null); if (run && !terminal.test(run.status)) await control("toolRunAbort", run.scope).catch(() => false); } }, 103: async context => reload(variant, context) },
    diagnostics: async () => ({ variant: variantId, run: await readRun(), shell: await readShell(page) }),
    exploreInventory: async () => ({ variant: variantId, shell: await readShell(page) }),
  };
}
