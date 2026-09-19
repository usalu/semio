/** 🩺️ Shared boot + interaction-bar probe for the six dormant plugins of slice B1a.
 *
 * The bar, asserted in order: the page reaches `data-semio-os-ready` with no `data-semio-os-error`;
 * the default example renders (the shell's example combobox resolves and the document panes carry
 * text); one Actions-panel verb dispatches AND lands an app-owned entry in the shell's history
 * ledger; `framework.history.undo` retires that entry and restores the pre-action document
 * signature.
 *
 * The state witness is the shell's own `data-history-json` (cursor/entries/actionIds), NOT a DOM
 * diff: expanding an action row to reach its arguments is itself a `[role="treeitem"]` change, so a
 * tree diff scores a refused verb as a state change. Document-signature text is kept as secondary
 * evidence and deliberately excludes every row inside a window's `…​.engagement` (Actions) pane.
 * Dispatch refusals reach the console as `warning`, not `error` — the fault filter reads every level.
 *
 * Driven by the per-plugin wrappers `🐍️b1a-<plugin>-boot-probe.mjs`; env overrides:
 * SEMIO_B1A_PLUGIN, SEMIO_B1A_PORT, SEMIO_B1A_URL, SEMIO_B1A_SECONDS, SEMIO_B1A_ACTIONS.
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const plugin = process.env.SEMIO_B1A_PLUGIN ?? "architect";
const port = process.env.SEMIO_B1A_PORT ?? "6090";
const variant = process.env.SEMIO_B1A_VARIANT ?? plugin;
const seconds = Number(process.env.SEMIO_B1A_SECONDS ?? 120);
const wanted = (process.env.SEMIO_B1A_ACTIONS ?? "").split(",").map((entry) => entry.trim()).filter(Boolean);
const prefix = process.env.SEMIO_B1A_PREFIX ?? "b1a";
const ticketDir = dirname(fileURLToPath(import.meta.url));
const outDir = join(ticketDir, "🗑️generated");
mkdirSync(outDir, { recursive: true });
const url = process.env.SEMIO_B1A_URL ?? `http://127.0.0.1:${port}/?plugin=${variant}`;

/** 🚨️ Every console shape that means a dispatch, a surface or the guest itself failed. */
const FAULT = /refused|dispatch-failed|dropped action|unsupported|unreachable|panicked|RuntimeError|wasm trap|not-ui-safe|catalog-incomplete|fault|error|Failed to/i;
/** 🔇️ Noise that is not a plugin defect: the dev server's favicon probe and vite's own chatter. */
const NOISE = /favicon|\[vite\]|Download the React DevTools|WebGL|Lit is in dev mode|status of 404/i;

const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (message) => lines.push(`${Date.now() - t0} ${message.type()} ${message.text().slice(0, 4000)}`));
page.on("pageerror", (error) => lines.push(`${Date.now() - t0} pageerror ${String(error).slice(0, 4000)}`));

const shell = () => page.evaluate(() => {
  const text = (element) => (element?.innerText ?? "").replace(/\s+/g, " ").trim();
  let history = null;
  try { history = JSON.parse(document.querySelector("[data-history-json]")?.getAttribute("data-history-json") ?? "null"); } catch { history = null; }
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    shellError: document.documentElement.getAttribute("data-semio-os-error"),
    title: document.title,
    history,
    panes: [...document.querySelectorAll("[data-surface-id]")].map((element) => ({
      id: element.getAttribute("data-surface-id"),
      canvases: element.querySelectorAll("canvas").length,
      svg: element.querySelectorAll("svg *").length,
      chars: text(element).length,
    })),
    windowKinds: [...new Set([...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")))],
    toggles: [...document.querySelectorAll('[id$=".engagement.toggle"]')].map((element) => element.id),
    actions: [...new Set([...document.querySelectorAll('[id^="action."]')].map((element) => element.id))].filter((id) => !id.startsWith("action.category.") && !id.includes(".arg.")),
    // 📄️ Document rows are the tree rows INSIDE a document surface only: the shell's own History
    // panel is a tree too, and every undo appends a row to it, so including it would make the
    // "document restored" witness unsatisfiable by construction.
    documentRows: [...document.querySelectorAll('[data-surface-id] [role="treeitem"]')].filter((element) => !element.closest('[id$=".engagement"]')).map((element) => text(element).slice(0, 80)),
    combobox: [...document.querySelectorAll('[role="combobox"]')].map((element) => text(element)),
    body: text(document.body).slice(0, 4000),
  };
});

/** 🔬️ The document witness an interaction must move — panes plus non-Actions tree rows. */
const signature = (view) => JSON.stringify({ panes: view.panes.map((pane) => `${pane.id}:${pane.chars}:${pane.svg}:${pane.canvases}`), rows: view.documentRows });
/** 📜️ The app-owned (non-`shell.*`) entries of the shell's undo ledger — the mutation witness. */
const appEntries = (view) => (view.history?.actionIds ?? []).filter((id) => !String(id).startsWith("shell."));
const faultsSince = (mark) => lines.slice(mark).filter((line) => FAULT.test(line) && !NOISE.test(line)).map((line) => line.slice(0, 700));
const click = async (id) => {
  const element = page.locator(`[id="${id}"]`).first();
  if (!(await element.count())) return "absent";
  return element.click({ timeout: 8000, force: true }).then(() => "ok").catch((error) => String(error).slice(0, 120));
};
/** ⏪️ Click the undo row's own control button, reporting `disabled` rather than pretending a click
 * on an inert row counted. */
const clickUndoButton = async () => {
  const button = page.locator('[id="framework.history.undo"] button').first();
  if (!(await button.count())) return "no-button";
  if (await button.isDisabled().catch(() => false)) return "disabled";
  return button.click({ timeout: 8000 }).then(() => "ok").catch((error) => String(error).split("\n")[0]);
};
/** 🪗️ Open every collapsed tree section between `id` and the document root, by clicking the header
 * that declares `aria-controls="<collapsed content id>"`. Returns one entry per click. */
const expandAncestors = async (id) => {
  const opened = [];
  for (let round = 0; round < 4; round++) {
    const header = await page.evaluate((target) => {
      const element = document.getElementById(target);
      if (!element) return null;
      for (let node = element; node && node !== document.documentElement; node = node.parentElement) {
        if (!node.id || !(node.hasAttribute("hidden") || getComputedStyle(node).display === "none")) continue;
        const trigger = document.querySelector(`[aria-controls="${CSS.escape(node.id)}"]`);
        if (trigger) return trigger.id || null;
      }
      return null;
    }, id);
    if (!header) break;
    opened.push(`${header}:${await click(header)}`);
    await page.waitForTimeout(900);
  }
  return opened.length ? opened : "not-needed";
};
const settle = async (predicate, ticks = 24) => {
  let view = await shell();
  for (let i = 0; i < ticks && !predicate(view); i++) { await page.waitForTimeout(500); view = await shell(); }
  return view;
};

const report = { plugin, variant, port, url, startedAt: new Date().toISOString(), steps: [] };
const step = (name, detail, mark) => { report.steps.push({ step: name, ms: Date.now() - t0, detail, faults: faultsSince(mark ?? 0) }); };

let boot = null;
let bootFailure = null;
try {
  await page.goto(url, { waitUntil: "domcontentloaded", timeout: 60000 });
  for (let i = 0; i < seconds; i++) {
    await page.waitForTimeout(1000);
    boot = await shell();
    if (boot.shellError) break;
    if (boot.ready && boot.panes.length && i > 6) break;
  }
  await page.waitForTimeout(4000);
  boot = await shell();
} catch (error) { bootFailure = String(error).slice(0, 2000); }
step("boot", boot ? { ready: boot.ready, shellError: boot.shellError, title: boot.title, panes: boot.panes, windowKinds: boot.windowKinds, combobox: boot.combobox, history: boot.history, bodyHead: boot.body.slice(0, 600) } : { bootFailure });

const exampleRendered = Boolean(boot?.combobox?.length) && boot.panes.some((pane) => pane.chars > 0 || pane.svg > 0 || pane.canvases > 0);
const interaction = { attempted: false, dispatched: false, action: null, undo: null };

if (boot?.ready && !boot.shellError) {
  // 📜️ `framework.history.undo` only exists while the History panel tab is mounted, so open it
  // BEFORE the baseline is sampled — opening it afterwards would move the document signature the
  // undo step is supposed to restore.
  const historyPanel = await click("framework.panel.history");
  await page.waitForTimeout(1500);
  step("history-panel", { opened: historyPanel, undoPresent: await page.locator('[id="framework.history.undo"]').count() });

  for (const toggle of boot.toggles) {
    const before = (await shell()).actions.length;
    await click(toggle);
    await page.waitForTimeout(1200);
    if ((await shell()).actions.length > before) break;
  }
  const opened = await shell();
  step("actions-panel", { toggles: boot.toggles, actionCount: opened.actions.length, actions: opened.actions.slice(0, 80) });

  // 🚫️ The Actions pane also lists the framework's own reserved verbs (history, clipboard,
  // selection, alternatives). They are NOT this app's document surface — scoring `undo` or
  // `selectAll` as "a mutating action dispatched" is exactly the false positive this probe exists to
  // avoid — so app-owned rows are tried first and reserved ones only as labelled evidence.
  const RESERVED = new Set([
    "undo", "redo", "revert", "copy", "cut", "paste", "selectAll", "clearSelection", "setSelectionMode",
    "setInteractionGranularity", "commitCheckpoint", "checkoutCheckpoint", "createAlternative",
    "switchAlternative", "noteShellCommand", "filter", "interactionSelect", "interactionHover",
  ]);
  const candidates = wanted.length ? wanted.map((name) => `action.${name}`).filter((id) => opened.actions.includes(id)) : opened.actions;
  const order = candidates.filter((id) => !RESERVED.has(id.replace(/^action\./, "")));
  const tried = [];
  for (const rowId of order) {
    const action = rowId.replace(/^action\./, "");
    const mark = lines.length;
    const baseView = await shell();
    const baseEntries = appEntries(baseView).length;
    const baseSignature = signature(baseView);
    interaction.attempted = true;
    const expanded = await click(rowId);
    await page.waitForTimeout(1200);
    const execute = page.locator(`[id$=".action.${action}.execute"]`).first();
    const submitted = (await execute.count()) ? await execute.click({ timeout: 8000, force: true }).then(() => "ok").catch((error) => String(error).slice(0, 120)) : "row-is-its-own-trigger";
    // ⏱️ A refusal is loud and immediate, so stop waiting for a ledger entry the moment one lands —
    // otherwise every dead verb costs the full settle budget.
    const after = await settle((view) => appEntries(view).length > baseEntries || faultsSince(mark).some((line) => /refused|dropped action/.test(line)), 16);
    const dispatched = appEntries(after).length > baseEntries;
    const attempt = { action, expanded, submitted, entriesBefore: baseEntries, entriesAfter: appEntries(after).length, actionIds: after.history?.actionIds ?? [], signatureMoved: signature(after) !== baseSignature, dispatched, faults: faultsSince(mark) };
    tried.push(attempt);
    if (dispatched) {
      interaction.dispatched = true;
      interaction.action = action;
      const undoMark = lines.length;
      // 🗂️ `framework.panel.history` is a tab button that TOGGLES the side panel shut when History is
      // already the active tab — clicking it unconditionally is how the first run lost the undo
      // control. Re-select it only when `framework.history.undo` is genuinely unmounted.
      let undoPresent = await page.locator('[id="framework.history.undo"]').count();
      let reopened = "not-needed";
      if (!undoPresent) {
        reopened = await click("framework.panel.history");
        await page.waitForTimeout(1200);
        undoPresent = await page.locator('[id="framework.history.undo"]').count();
      }
      // 🧰️ The undo row sits inside a COLLAPSED tree section of the History panel, so it is mounted
      // with a 0×0 box and `force` cannot click it. The section header is not `framework.history.commands`
      // (that one controls the entries list) — resolve the real expander through the collapsed
      // ancestor's `aria-controls` back-reference instead of guessing an id.
      const commandsExpanded = await expandAncestors("framework.history.undo");
      // ↩️ The ledger does NOT rewind its cursor on undo: it APPENDS an `undo` entry and flips
      // `canRedo`. So the witness is one undo click — the app mutation is the newest entry, and a
      // second click would start retiring the shell's own panel verbs (which the guest then
      // re-dispatches as undeclared app actions, a separate defect recorded in the report).
      const beforeUndo = await shell();
      // ⏺️ `framework.history.undo` is a tree ROW whose `control` slot holds the real `<button>`
      // (`🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:9087`).
      // Clicking the row is silently inert — the button is what carries `onAction({action:"undo"})`.
      const undoClicks = [await clickUndoButton()];
      await page.waitForTimeout(900);
      const undone = await settle((view) => Boolean(view.history?.canRedo) && signature(view) === baseSignature, 12);
      interaction.undo = {
        clicked: "framework.history.undo",
        reopenedHistoryPanel: reopened,
        undoPresent,
        commandsExpanded,
        undoClicks,
        appEntriesBefore: appEntries(beforeUndo).length,
        appEntriesAfter: appEntries(undone).length,
        cursorBefore: beforeUndo.history?.cursor ?? null,
        cursorAfter: undone.history?.cursor ?? null,
        canRedo: undone.history?.canRedo ?? null,
        signatureRestored: signature(undone) === baseSignature,
        faults: faultsSince(undoMark),
      };
      break;
    }
    await click(rowId);
    await page.waitForTimeout(400);
  }
  step("invoke-action", { tried, reservedRowsSkipped: candidates.filter((id) => RESERVED.has(id.replace(/^action\./, ""))), dispatched: interaction.dispatched, action: interaction.action, undo: interaction.undo });
}

const final = boot ? await shell().catch(() => null) : null;
step("final", final ? { ready: final.ready, shellError: final.shellError, history: final.history, panes: final.panes } : null);
await page.screenshot({ path: join(outDir, `${prefix}-${plugin}.png`), fullPage: false }).catch(() => {});

const faults = faultsSince(0);
/** ↩️ Undo counts only when the shell offers a redo of the retired command AND the document witness
 * is back at its pre-action value — `canRedo` alone would pass on an undo of a shell panel verb. */
const undoWorks = Boolean(interaction.undo && (interaction.undo.canRedo ?? false) && interaction.undo.signatureRestored);
report.summary = {
  ready: final?.ready ?? boot?.ready ?? null,
  shellError: final?.shellError ?? boot?.shellError ?? bootFailure,
  loadsClean: Boolean((final ?? boot)?.ready) && !(final ?? boot)?.shellError && faults.length === 0,
  exampleRendered,
  dispatched: interaction.dispatched,
  action: interaction.action,
  undoWorks,
  faultLines: faults.length,
  consoleLines: lines.length,
};
report.summary.bar = report.summary.loadsClean && report.summary.exampleRendered && report.summary.dispatched && report.summary.undoWorks;
writeFileSync(join(outDir, `${prefix}-${plugin}-console.txt`), [
  `# ${plugin} react playground ${url}`,
  `# summary ${JSON.stringify(report.summary)}`,
  "",
  "## shell view",
  JSON.stringify({ windowKinds: (final ?? boot)?.windowKinds, panes: (final ?? boot)?.panes, combobox: (final ?? boot)?.combobox, documentRows: (final ?? boot)?.documentRows, actions: (final ?? boot)?.actions }, null, 1),
  "",
  "## steps",
  JSON.stringify(report.steps, null, 1),
  "",
  `## fault lines (${faults.length})`,
  ...faults.slice(0, 200),
  "",
  `## full console (${lines.length})`,
  ...lines,
].join("\n"));
process.stdout.write(`${prefix.toUpperCase()} ${plugin} ${JSON.stringify(report.summary)}\n`);
await browser.close();
