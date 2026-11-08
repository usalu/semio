/** 🛤️ Exercises a real EN 1990 field, its dependent check and persisted document history in both locales. */
import { mkdirSync, readdirSync } from "node:fs";
import { join } from "node:path";
import type { TimeTravelScenarioV1 } from "@semio-tech/framework/time-travel/testing/scenario";
import { page, renderer, currentLocale, OUT, stamp, mod, expectedReloads, setExpectedReloads, setProbeReloadAt } from "@semio-tech/framework/time-travel/testing/state";
import { evalSafe, waitUntil, shot } from "@semio-tech/framework/time-travel/testing/page";
import { closePanels, waitForBoot } from "@semio-tech/framework/time-travel/testing/chrome";
import { allHistoryRows, documentEntries, openHistory } from "@semio-tech/framework/time-travel/testing/history";
import { attachFolder } from "@semio-tech/framework/time-travel/testing/history-session";
import { mirror, mirrorActivate, wgpuPress } from "@semio-tech/framework/time-travel/testing/wgpu-mirror";
import { reprojectionSettled } from "@semio-tech/framework/time-travel/testing/reprojection";
import { verdict } from "@semio-tech/framework/time-travel/testing/report";
import matrix from "./🧫️witness/🔣️.json" with { type: "json" };

type Surface = { key: string; window: string; label: string; value: string | null; role: string; inputType: string | null };
type Context = { folder?: string; baseline?: number; rows?: { key: string; label: string }[] };
type State = { value: number; utilization: number };

const selected = process.env.SEMIO_NORM_ARTIFACT ?? "en1990";
const row = matrix.artifacts.find(artifact => artifact.variant === selected);
if (row === undefined) throw new Error(`SEMIO_NORM_ARTIFACT names no matrix artifact: ${selected}`);
const origin = (port: number) => `http://127.0.0.1:${port}/?plugin=${row.variant}&role=editor&example=${row.example}`;
const pin = row.pin;
const witness = { variant: row.variant, windows: matrix.windows.map(role => `norm-${row.variant}-${role}`), serves: { react: origin(row.ports.react), wgpu: origin(row.ports.wgpu) }, field: pin?.field ?? { path: "", key: "", owner: "", ownerWindow: `norm-${row.variant}-outliner`, argument: "", label: { en: "", de: "" }, limit: 1, unit: "" }, check: pin?.check ?? "", states: pin?.states ?? { before: { value: 0, utilization: 0 }, after: { value: 0, utilization: 0 } } };

const number = (value: string | null): number => value === null ? NaN : Number(value.replace(/\s/gu, "").replace(",", "."));
const same = (actual: number, expected: number): boolean => Number.isFinite(actual) && Math.abs(actual - expected) < 1e-8;
const relevant = (key: string): boolean => key.endsWith(witness.field.owner) || key.includes(witness.field.key) || key.includes(witness.check);

/** 🪞️ Reads the actual DOM or accessibility mirror without accessing the guest's document store. */
async function surfaces(): Promise<Surface[]> {
  if (renderer === "wgpu") return (await mirror()).filter(node => relevant(node.key)).map(node => ({ key: node.key, window: node.window, label: `${node.label} ${node.description}`, value: node.value ?? node.valueNow, role: node.role, inputType: node.inputType }));
  return evalSafe((fixture) => {
    const nodes = Array.from(document.querySelectorAll<HTMLElement>("[id]")).filter(node => node.id.endsWith(fixture.owner) || node.id.includes(fixture.key) || node.id.includes(fixture.check));
    return nodes.map(node => {
      const input = node instanceof HTMLInputElement ? node : node.querySelector<HTMLInputElement>("input");
      return { key: node.id, window: node.closest('[data-slot="window"]')?.id ?? "", label: `${node.getAttribute("aria-label") ?? ""} ${node.innerText} ${node.getAttribute("title") ?? ""}`, value: input?.value ?? node.getAttribute("aria-valuenow"), role: node.getAttribute("role") ?? node.tagName.toLowerCase(), inputType: input?.type ?? null };
    });
  }, [] as Surface[], { owner: witness.field.owner, key: witness.field.key, check: witness.check });
}

/** 🧷️ Locates the exact rendered element without interpolating authored identities into CSS. */
async function target(node: Surface, input = false) {
  const found = await evalSafe((arg) => {
    document.querySelectorAll("[data-norm-witness-target]").forEach(element => element.removeAttribute("data-norm-witness-target"));
    const row = arg.renderer === "wgpu" ? Array.from(document.querySelectorAll<HTMLElement>("#semio-wgpu-accessibility [data-node-key]")).find(element => element.dataset.nodeKey === arg.key && element.dataset.window === arg.window) : document.getElementById(arg.key);
    const element = arg.input && !(row instanceof HTMLInputElement) ? row?.querySelector("input") : row;
    element?.setAttribute("data-norm-witness-target", "true");
    return Boolean(element);
  }, false, { key: node.key, window: node.window, input, renderer });
  return found ? page.locator('[data-norm-witness-target="true"]').first() : null;
}

/** 👆️ Selects the SLS entity owning the field and its check through its authored outliner identity. */
async function selectOwner(): Promise<boolean> {
  const shown = await waitUntil(async () => (await surfaces()).filter(node => node.key.endsWith(witness.field.owner) && (node.window === witness.field.ownerWindow || node.window.endsWith(`::${witness.field.ownerWindow}`))).sort((a, b) => a.key.length - b.key.length)[0] ?? null, node => node !== null, 30000, 500);
  if (!shown.value) return false;
  if (renderer === "wgpu") return mirrorActivate(shown.value.key, shown.value.window);
  const row = await target(shown.value);
  if (!row) return false;
  await row.click({ timeout: 5000 });
  return true;
}

/** 📏️ Requires both the edited quantity and the rendered engineering utilization to agree. */
async function stateIs(name: string, expected: State): Promise<boolean> {
  const observed = await waitUntil(async () => {
    const nodes = await surfaces();
    const fields = nodes.filter(node => node.key.includes(witness.field.key) && node.value !== null);
    const checks = nodes.filter(node => node.key.includes(witness.check));
    const value = fields.map(node => number(node.value)).find(value => same(value, expected.value));
    const percentage = expected.utilization * 100;
    const check = checks.find(node => Array.from(node.label.matchAll(/(-?\d+(?:[.,]\d+)?)\s*%/gu)).some(match => Math.abs(number(match[1]) - percentage) < 0.51));
    return { value: value ?? null, check: check?.label.slice(0, 400) ?? null, fields: fields.map(node => ({ key: node.key, value: node.value })), checks: checks.map(node => node.label.slice(0, 200)) };
  }, value => value.value !== null && value.check !== null, 45000, 500);
  verdict(name, observed.ok, { expected, observed: observed.value, waitedMs: observed.waitedMs, locale: currentLocale });
  await shot(name);
  return observed.ok;
}

/** 🪟️ Requires the five visual editor windows of the artifact to be mounted and records every one in the report. */
async function windowsMounted(): Promise<void> {
  const mounted = await waitUntil(async () => evalSafe((ids) => ids.filter(id => document.getElementById(id) !== null || document.querySelector(`[data-slot="window"][id$="${id}"]`) !== null), [] as string[], witness.windows), found => found.length === witness.windows.length, 45000, 500);
  verdict("visual-editor-windows-mounted", mounted.ok, { expected: witness.windows, mounted: mounted.value });
}

/** 📌️ Fails loudly while the artifact has no pinned numeric witness so an unpinned row can never read as a pass. */
function pinned(step: string): boolean {
  if (pin === null) verdict(`${step}-needs-pinned-witness`, false, { variant: row!.variant, example: row!.example });
  return pin !== null;
}

/** 📁️ Binds the document before editing so a later reload exercises its real folder archive. */
async function boot(ctx: Context): Promise<void> {
  await windowsMounted();
  if (!pinned("boot")) return;
  const selected = await selectOwner();
  verdict("engineering-owner-selectable", selected, { owner: witness.field.owner });
  await stateIs("engineering-before", witness.states.before);
  ctx.folder = join(OUT, `norm-${stamp}-${currentLocale}`);
  mkdirSync(ctx.folder, { recursive: true });
  const attached = await attachFolder(ctx.folder);
  verdict("engineering-folder-attached", attached.card && attached.typed === ctx.folder && attached.attachButton, { attached, folder: ctx.folder });
  await closePanels();
  await selectOwner();
  await stateIs("folder-attach-preserves-engineering-state", witness.states.before);
  await openHistory();
  ctx.baseline = documentEntries(await allHistoryRows()).length;
  await closePanels();
}

/** ✍️ Commits one localized numeric input and checks its one document-history entry. */
async function edit(ctx: Context): Promise<void> {
  if (!pinned("edit")) return;
  const ready = await waitUntil(async () => (await surfaces()).filter(node => node.key.includes(witness.field.key) && node.value !== null).sort((a, b) => a.key.length - b.key.length)[0] ?? null, node => node !== null, 15000, 300);
  if (!ready.value) throw new Error(`Missing numeric field ${witness.field.key}`);
  const field = await target(ready.value, true);
  if (!field) throw new Error(`Missing numeric input ${ready.value.key}`);
  const text = String(witness.states.after.value);
  await field.fill(currentLocale === "de" && ready.value.inputType !== "number" ? text.replace(".", ",") : text, { timeout: 5000 });
  await field.press("Enter");
  await page.keyboard.press("Tab");
  await stateIs("engineering-after-edit", witness.states.after);
  await openHistory();
  const history = await waitUntil(allHistoryRows, rows => ctx.baseline !== undefined && documentEntries(rows).length === ctx.baseline + 1, 15000, 500);
  ctx.rows = documentEntries(history.value).map(row => ({ key: row.key, label: row.label }));
  verdict("one-engineering-edit-one-document-entry", history.ok, { before: ctx.baseline, after: ctx.rows.length, rows: ctx.rows });
  await closePanels();
}

/** ⏪️ Uses the shell history chord after moving focus out of the numeric text editor. */
async function historyChord(redo: boolean): Promise<void> {
  if (!pinned(redo ? "redo" : "undo")) return;
  await selectOwner();
  await page.keyboard.press(redo ? `${mod}+Shift+z` : `${mod}+z`);
  await stateIs(redo ? "engineering-after-redo" : "engineering-after-undo", redo ? witness.states.after : witness.states.before);
}

/** 🔂️ Reloads and reopens the same persisted artifact, then compares its quantity, result and history identities. */
async function reload(ctx: Context): Promise<void> {
  if (!pinned("reload")) return;
  if (!ctx.folder || !ctx.rows?.length) throw new Error("Engineering reload requires the committed folder-bound edit");
  const folder = ctx.folder;
  const written = await waitUntil(async () => readdirSync(folder, { recursive: true, withFileTypes: true }).filter(entry => entry.isFile()).length, count => count > 0, 30000, 500);
  verdict("engineering-archive-written", written.ok, { folder, files: written.value });
  if (!written.ok) throw new Error("No persisted engineering archive");
  setExpectedReloads(expectedReloads + 1);
  setProbeReloadAt(Date.now());
  await page.reload({ waitUntil: "domcontentloaded", timeout: 60000 });
  verdict("engineering-reloaded-host", await waitForBoot("norm-reload", 60));
  const offered = await waitUntil(async () => renderer === "wgpu" ? (await mirror()).some(node => node.key === "s-folder-reconnect") : await page.locator("#s-folder-reconnect").count() > 0, Boolean, 15000, 500);
  if (offered.ok) {
    if (renderer === "wgpu") await wgpuPress("s-folder-reconnect");
    else await page.locator("#s-folder-reconnect").click({ timeout: 5000 });
  } else {
    const attached = await attachFolder(folder);
    verdict("engineering-folder-reopened", attached.card && attached.typed === folder && attached.attachButton, { attached });
  }
  const settled = await reprojectionSettled(60000);
  verdict("engineering-reload-reprojection-settled", settled.ok, { settled });
  await closePanels();
  await selectOwner();
  await stateIs("engineering-persisted-after-reload", witness.states.after);
  await openHistory();
  const before = ctx.rows;
  const hydrated = await waitUntil(allHistoryRows, rows => before.every(expected => documentEntries(rows).some(actual => actual.key === expected.key && actual.label === expected.label)), 30000, 500);
  verdict("engineering-history-persists-after-reload", hydrated.ok, { before, after: documentEntries(hydrated.value).map(row => ({ key: row.key, label: row.label })) });
  await closePanels();
}

/** 🧭️ The norm-owned scenario uses real controls, native inference and the host's persistence path. */
export const timeTravelScenarioV1: TimeTravelScenarioV1<Context> = {
  variant: witness.variant,
  defaultServes: witness.serves,
  bootWindowPrefix: "norm-",
  editorNumbers: witness.field.argument === "" ? [] : [witness.field.argument],
  paletteQuery: { en: witness.field.label.en, de: witness.field.label.de },
  stepNumbers: [1, 2, 3, 4, 5],
  order: [1, 2, 3, 4, 5],
  steps: { 1: boot, 2: edit, 3: () => historyChord(false), 4: () => historyChord(true), 5: reload },
  context: () => ({}),
  prepare: async () => null,
  beforeStep: async () => {},
  afterSteps: async () => {},
  diagnostics: async () => ({ engineering: await surfaces() }),
  exploreInventory: async () => ({ engineering: await surfaces(), witness, row }),
};
