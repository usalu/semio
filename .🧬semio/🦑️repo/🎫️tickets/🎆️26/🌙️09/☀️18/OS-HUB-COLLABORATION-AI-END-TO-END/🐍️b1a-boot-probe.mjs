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
    documentRows: [...document.querySelectorAll('[role="treeitem"]')].filter((element) => !element.closest('[id$=".engagement"]')).map((element) => text(element).slice(0, 80)),
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
  for (const toggle of boot.toggles) {
    const before = (await shell()).actions.length;
    await click(toggle);
    await page.waitForTimeout(1200);
    if ((await shell()).actions.length > before) break;
  }
  const opened = await shell();
  step("actions-panel", { toggles: boot.toggles, actionCount: opened.actions.length, actions: opened.actions.slice(0, 80) });

  const order = wanted.length ? wanted.map((name) => `action.${name}`).filter((id) => opened.actions.includes(id)) : opened.actions;
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
    const after = await settle((view) => appEntries(view).length > baseEntries, 16);
    const dispatched = appEntries(after).length > baseEntries;
    const attempt = { action, expanded, submitted, entriesBefore: baseEntries, entriesAfter: appEntries(after).length, actionIds: after.history?.actionIds ?? [], signatureMoved: signature(after) !== baseSignature, dispatched, faults: faultsSince(mark) };
    tried.push(attempt);
    if (dispatched) {
      interaction.dispatched = true;
      interaction.action = action;
      const undoMark = lines.length;
      const undone = await (async () => { await click("framework.history.undo"); await page.waitForTimeout(800); return settle((view) => appEntries(view).length <= baseEntries || (view.history?.cursor ?? 0) < (after.history?.cursor ?? 0), 16); })();
      interaction.undo = {
        clicked: "framework.history.undo",
        cursorBefore: after.history?.cursor ?? null,
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
  step("invoke-action", { tried, dispatched: interaction.dispatched, action: interaction.action, undo: interaction.undo });
}

const final = boot ? await shell().catch(() => null) : null;
step("final", final ? { ready: final.ready, shellError: final.shellError, history: final.history, panes: final.panes } : null);
await page.screenshot({ path: join(outDir, `b1a-${plugin}.png`), fullPage: false }).catch(() => {});

const faults = faultsSince(0);
const undoWorks = Boolean(interaction.undo && interaction.undo.cursorAfter !== null && interaction.undo.cursorAfter < interaction.undo.cursorBefore && (interaction.undo.canRedo ?? false));
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
writeFileSync(join(outDir, `b1a-${plugin}-console.txt`), [
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
process.stdout.write(`B1A ${plugin} ${JSON.stringify(report.summary)}\n`);
await browser.close();
