/** 🪵️ Slice-B3f bar probe for 🪵️sourcing's ONE real document mutation (from B3e's, control fixed).
 *
 * B3e located the control with `[id="…curated"] ~ button`, which can never match: the readout input
 * sits inside its own `input-root` wrapper, so the increment button is the input's UNCLE, not its
 * sibling. This one presses `[data-slot="table-stepper-increment"]` (the marker the React table
 * stepper now carries) and additionally drives the cell's keyboard route.
 *
 * sourcing publishes sixteen Actions-rail verbs and none of them is an undoable document edit:
 * `stockFromCatalogue` is a `HostOnly` tool that answers with `Effect::LoadDocument`, and the
 * curation vocabulary (`curationAdd` / `curationRemove` / `curationSetCount` / `dropOn*`) is bound to
 * the Pool TABLE's cells, not to the rail. The shared rail probe therefore scores sourcing as an app
 * with no mutation; this one drives the real control instead: the `curated` column is a
 * `TableCell::Stepper` per stock kind, id `window:sourcing-pool.<kind-id>.curated`, bound to
 * `curationSetCount` on `Trigger::Change`.
 *
 * Bar clauses measured, same five as the shared probe: boots, example loads, one real mutation
 * (ledger row + `#s-checkin` edit count), undo → redo, zero console fault lines.
 */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { writeFileSync, mkdirSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { join, dirname } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6081/?plugin=sourcing";
const settleMs = Number(process.env.SEMIO_PROBE_SETTLE_MS ?? 60_000);
const bootMs = Number(process.env.SEMIO_PROBE_BOOT_MS ?? 30_000);
const target = process.env.SEMIO_PROBE_KIND ?? "beam-glulam-gl24h";
const here = dirname(fileURLToPath(new URL(import.meta.url)));
const outDir = join(here, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "b3e-sourcing-curation");
mkdirSync(outDir, { recursive: true });

const lines = [];
const report = {};
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
const t0 = Date.now();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 900)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 900)}`));
page.setDefaultNavigationTimeout(180_000);
const note = (key, value) => {
  report[key] = value;
  lines.push(`${Date.now() - t0} probe ${key} ${JSON.stringify(value).slice(0, 900)}`);
  console.log(key, JSON.stringify(value).slice(0, 900));
};

const witness = () =>
  page.evaluate(() => {
    const ledger = [...document.querySelectorAll('[id^="framework.history.entry."]')].map((el) => `${el.id}:${(el.textContent ?? "").trim().slice(0, 40)}`);
    const checkin = (document.querySelector("#s-checkin")?.textContent ?? "").trim().slice(0, 60);
    const edits = Number(/(\d+)/.exec(checkin)?.[1] ?? 0);
    const stepper = document.querySelector('[id$=".curated"]');
    return { ledger: ledger.slice(-6), ledgerCount: ledger.length, checkin, edits, stepper: stepper ? { id: stepper.id, value: stepper.value } : null };
  });

const press = async (id) => {
  const active = `[data-slot="window"][data-active="true"] [id="${id}"]`;
  for (const selector of [active, `[id="${id}"] >> nth=0`, `[id="${id}"] >> nth=1`]) {
    try {
      await page.locator(selector).first().click({ timeout: 4_000 });
      return `${selector}=ok`;
    } catch (error) {
      lines.push(`${Date.now() - t0} probe press-miss ${selector} ${String(error).slice(0, 120)}`);
    }
  }
  return "absent";
};

const until = async (predicate) => {
  const deadline = Date.now() + settleMs;
  while (Date.now() < deadline) {
    const state = await witness();
    if (predicate(state)) return state;
    await page.waitForTimeout(1_000);
  }
  return witness();
};

await page.goto(url, { waitUntil: "domcontentloaded", timeout: 180_000 });
await page.waitForFunction(() => document.querySelector("[data-semio-os-ready]"), null, { timeout: 180_000 });
await page.waitForTimeout(bootMs);

note("boot", await page.evaluate(() => ({
  ready: document.querySelector("[data-semio-os-ready]")?.getAttribute("data-semio-os-ready") ?? null,
  error: document.querySelector("[data-semio-os-error]")?.getAttribute("data-semio-os-error") ?? null,
  windowIds: [...document.querySelectorAll("[data-surface-id]")].map((el) => el.getAttribute("data-surface-id")),
  poolChars: (document.querySelector('[data-surface-id="window:sourcing-pool"]')?.textContent ?? "").trim().length,
})));

const opened = await page
  .locator('[data-slot="panel-tab-button"][id="framework.panel.history"], [id="framework.panel.history"]')
  .first()
  .click({ timeout: 8_000 })
  .then(() => "ok")
  .catch((error) => String(error).slice(0, 120));
await page.waitForTimeout(2_000);
note("open-history", { opened, ...(await witness()) });

const stepperId = `window:sourcing-pool.${target}.curated`;
const before = await witness();
// 🔼️ The stepper cell renders as a non-editable input flanked by its own increment/decrement
// buttons, so the mutation is driven by pressing a sibling button, not by typing into the input.
const filled = [];
for (const selector of [`[data-slot="table-stepper-increment"][data-stepper-for="${stepperId}"]`, `td:has([id="${stepperId}"]) button >> nth=-1`, `[id="${stepperId}"] ~ button`]) {
  const result = await page
    .locator(selector)
    .first()
    .click({ timeout: 6_000 })
    .then(() => "ok")
    .catch((error) => String(error).slice(0, 90));
  filled.push(`${selector}=${result}`);
  await page.waitForTimeout(2_000);
  const state = await witness();
  if (state.edits > before.edits || state.ledgerCount > before.ledgerCount) break;
}
const afterInvoke = await until((state) => state.edits > before.edits || state.ledgerCount > before.ledgerCount);
note("invoke-curation-set-count", { stepperId, filled, before, afterInvoke, mutated: afterInvoke.edits > before.edits || afterInvoke.ledgerCount > before.ledgerCount });

const undoClick = await press("action.undo");
const afterUndo = await until((state) => state.edits < afterInvoke.edits || state.ledgerCount < afterInvoke.ledgerCount);
note("undo", { undoClick, afterUndo, undone: afterUndo.edits < afterInvoke.edits || afterUndo.ledgerCount < afterInvoke.ledgerCount });

const redoClick = await press("action.redo");
const afterRedo = await until((state) => state.edits >= afterInvoke.edits || state.ledgerCount >= afterInvoke.ledgerCount);
note("redo", { redoClick, afterRedo, redone: afterRedo.edits >= afterInvoke.edits || afterRedo.ledgerCount >= afterInvoke.ledgerCount });

// ⌨️ The keyboard route of the same cell: focus the spinbutton group and press ArrowUp.
const keyboardBefore = await witness();
const keyboardFocus = await page
  .locator(`[data-slot="table-stepper"][data-stepper-for="${stepperId}"]`)
  .first()
  .focus({ timeout: 6_000 })
  .then(() => "ok")
  .catch((error) => String(error).slice(0, 90));
if (keyboardFocus === "ok") await page.keyboard.press("ArrowUp");
const afterKeyboard = await until((state) => state.edits > keyboardBefore.edits || state.ledgerCount > keyboardBefore.ledgerCount);
note("keyboard-arrow-up", { keyboardFocus, keyboardBefore, afterKeyboard, stepped: afterKeyboard.edits > keyboardBefore.edits || afterKeyboard.ledgerCount > keyboardBefore.ledgerCount });

const faultLines = lines.filter((line) => / error | pageerror /.test(line) && !/404|favicon/.test(line));
note("SUMMARY", {
  ready: report.boot.ready,
  error: report.boot.error,
  mutated: report["invoke-curation-set-count"].mutated,
  undone: report.undo.undone,
  redone: report.redo.redone,
  faultLines: faultLines.length,
  faults: faultLines.slice(0, 4),
});

await page.screenshot({ path: join(outDir, "sourcing-curation.png"), fullPage: false }).catch(() => {});
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 1));
await browser.close();
