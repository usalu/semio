/** 🧪️ Wave B36's isolating repro for the "passes in a fresh `--only=` lane, fails in the full battery"
 * family: one cold boot, one scripted case, an UNBOUNDED console log (wave B33 §7.1 — a ring buffer makes
 * every length mark invalid), and a `VERDICT` line per reading. It never reuses `🔍️browser-probe.ts`
 * (B35 owns that file); it reimplements only its boot recipe so a case boots exactly like a battery does.
 *
 * Run: `bun 🔍️b36-pollution.ts --case=<name> [--port=6013]`
 * Cases: `inspection-toggle`, `settings-language`, `grid-spacing`, `outliner-toggle`, `history-toggle`,
 * `volume-arm`, `dup-delete`.
 * Ticket 26/09/02/PUZZLE-3D-END-TO-END. */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const OUT = join(import.meta.dir, "🗑️generated");
mkdirSync(OUT, { recursive: true });
const stamp = new Date().toISOString().replace(/[:.]/g, "-").slice(0, 19);
const port = process.argv.find((a) => a.startsWith("--port="))?.slice(7) ?? "6013";
const kase = process.argv.find((a) => a.startsWith("--case="))?.slice(7) ?? "inspection-toggle";
const t0 = Date.now();
const lines: string[] = [];
const console_: string[] = [];
const log = (m: string) => {
  const row = `[${((Date.now() - t0) / 1000).toFixed(1)}s] ${m}`;
  lines.push(row);
  console.log(row);
};
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, acceptDownloads: true });
page.on("console", (msg) => console_.push(`${msg.type()}: ${msg.text().slice(0, 400)}`));
page.on("pageerror", (error) => console_.push(`pageerror: ${String(error).slice(0, 400)}`));

const snapshot = () =>
  page.evaluate(() => ({
    windows: Array.from(document.querySelectorAll('[data-slot="window"]')).length,
    canvases: document.querySelectorAll("canvas").length,
    dialogs: Array.from(document.querySelectorAll('[role="dialog"]')).map((d) => (d as HTMLElement).innerText.slice(0, 60)),
  }));
await page.goto(`http://127.0.0.1:${port}/?plugin=puzzle3d`, { waitUntil: "domcontentloaded", timeout: 60000 });
let booted = false;
for (let i = 0; i < 40; i++) {
  await page.waitForTimeout(3000);
  const s = await snapshot();
  if (s.dialogs.length && i % 3 === 0) {
    const skip = page.locator('[role="dialog"] button', { hasText: /skip/i }).first();
    if (await skip.count()) await skip.click({ timeout: 2000 }).catch(() => {});
  }
  if (s.windows >= 2 && s.canvases >= 2) {
    booted = true;
    log(`booted windows=${s.windows} canvases=${s.canvases}`);
    break;
  }
}
const skipTour = page.locator("button", { hasText: /^\s*(x\s*)?skip\s*$/i }).first();
if (await skipTour.count()) {
  await skipTour.click({ timeout: 3000 }).catch(() => {});
  await page.waitForTimeout(1500);
}
log(`boot=${booted} case=${kase}`);

/** 🔎️ One DOM census per family this wave measures, all read in a single evaluate so a reading is a
 * consistent snapshot rather than five racing ones. */
const census = () =>
  page.evaluate(() => {
    const text = (el: Element | null) => (el as HTMLElement | null)?.innerText.replace(/\n/g, " | ").trim().slice(0, 200) ?? null;
    const value = (id: string) => {
      const el = document.getElementById(id) as HTMLInputElement | null;
      return el ? (el.value ?? el.getAttribute("aria-valuenow") ?? el.getAttribute("data-value")) : null;
    };
    const idsWith = (prefix: string) => Array.from(document.querySelectorAll(`[id^="${prefix}"]`)).map((el) => el.id);
    const tabs = Array.from(document.querySelectorAll('[data-slot="panel-tab-button"]')).map((b) => ({ id: b.id, active: b.getAttribute("aria-selected") ?? b.getAttribute("data-state") ?? null }));
    const inspectorRows = idsWith("panel:puzzle3d-play-inspector/");
    return {
      tabs,
      inspectorRows,
      inspectorPopulated: inspectorRows.length > 0,
      inspectorText: text(document.getElementById("puzzle3d-play-inspector")),
      documentRows: idsWith("panel:puzzle3d-play-document/"),
      historyEntries: idsWith("framework.history.entry."),
      languageControl: document.querySelectorAll('[id="framework.settings.language"]').length,
      railSpacing: value("puzzle3d-play-grid-spacing"),
      settingsSpacing: value("panel:puzzle3d-play-settings/puzzle3d-play-settings.grid-spacing.control"),
      selected: Array.from(document.querySelectorAll("[data-interaction-json]")).map((el) => {
        try {
          const utility = JSON.parse(el.getAttribute("data-interaction-json") || "{}") as { activeUtility?: string };
          const painted = JSON.parse(el.getAttribute("data-selection-json") || "{}") as { selectedIds?: string[] };
          return { selectedIds: painted.selectedIds ?? [], activeUtility: utility.activeUtility ?? null };
        } catch {
          return { selectedIds: [], activeUtility: "parse-failed" };
        }
      }),
      instances: (() => {
        const el = document.querySelector("[data-instances-json]");
        try {
          return (JSON.parse(el?.getAttribute("data-instances-json") || "[]") as { id?: string }[]).map((row) => row.id ?? "?");
        } catch {
          return ["parse-failed"];
        }
      })(),
    };
  });
const verdict = (name: string, note: string) => log(`VERDICT ${name} ${note}`);
const clickId = async (id: string) => {
  const found = await page.locator(`[id="${id}"]`).count();
  if (found) await page.locator(`[id="${id}"]`).last().click({ force: true, timeout: 4000 }).catch(() => {});
  return found;
};
const pickCanvas = async (x = 470, y = 420) => {
  await page.locator("canvas").last().click({ position: { x, y }, timeout: 5000 }).catch(() => {});
  await page.waitForTimeout(3500);
};

if (booted && kase === "inspection-toggle") {
  const zero = await census();
  verdict("boot-inspector", `rows=${zero.inspectorRows.length} tabs=${JSON.stringify(zero.tabs.map((t) => t.id))}`);
  await pickCanvas();
  const afterPick = await census();
  verdict("after-first-pick", `selected=${JSON.stringify(afterPick.selected)} inspectorRows=${afterPick.inspectorRows.length} text=${afterPick.inspectorText}`);
  const clicked = await clickId("framework.panel.inspection");
  await page.waitForTimeout(2500);
  const afterTab = await census();
  verdict("after-inspection-tab-click", `tabButtons=${clicked} inspectorRows=${afterTab.inspectorRows.length} selected=${JSON.stringify(afterTab.selected)} text=${afterTab.inspectorText}`);
  await clickId("framework.panel.inspection");
  await page.waitForTimeout(2500);
  const again = await census();
  verdict("after-second-tab-click", `inspectorRows=${again.inspectorRows.length} text=${again.inspectorText}`);
}

if (booted && kase === "settings-language") {
  const zero = await census();
  verdict("boot-language", `control=${zero.languageControl} tabs=${JSON.stringify(zero.tabs.map((t) => t.id))}`);
  const opened = await clickId("puzzle3d.panel.settings");
  await page.waitForTimeout(2500);
  const afterPuzzle = await census();
  verdict("after-puzzle3d-settings", `clicked=${opened} language=${afterPuzzle.languageControl} settingsSpacing=${afterPuzzle.settingsSpacing} tabs=${JSON.stringify(afterPuzzle.tabs.map((t) => t.id))}`);
  await clickId("framework.settings");
  await page.waitForTimeout(2500);
  const afterFramework = await census();
  verdict("after-framework-settings", `language=${afterFramework.languageControl}`);
  await clickId("puzzle3d.panel.settings");
  await page.waitForTimeout(2500);
  const back = await census();
  verdict("after-puzzle3d-again", `language=${back.languageControl}`);
}

if (booted && kase === "grid-spacing") {
  const unfold = await page.locator('[id="framework.window.puzzle3dMainPerspective.measures.unfold"]').count();
  if (unfold) await page.locator('[id="framework.window.puzzle3dMainPerspective.measures.unfold"]').first().click({ timeout: 4000 }).catch(() => {});
  await page.waitForTimeout(2000);
  const zero = await census();
  verdict("boot-spacing", `rail=${zero.railSpacing} settings=${zero.settingsSpacing}`);
  const slider = page.locator('[id="puzzle3d-play-grid-spacing"]').first();
  if (await slider.count()) {
    await slider.focus().catch(() => {});
    await page.keyboard.press("ArrowRight").catch(() => {});
    await page.keyboard.press("ArrowRight").catch(() => {});
  }
  await page.waitForTimeout(3000);
  const afterRail = await census();
  verdict("after-rail-nudge", `rail=${afterRail.railSpacing} settings=${afterRail.settingsSpacing}`);
  await clickId("puzzle3d.panel.settings");
  await page.waitForTimeout(2500);
  const opened = await census();
  verdict("settings-panel-open", `rail=${opened.railSpacing} settings=${opened.settingsSpacing}`);
  const stepper = page.locator('[id="panel:puzzle3d-play-settings/puzzle3d-play-settings.grid-spacing.control"]').first();
  if (await stepper.count()) {
    await stepper.focus().catch(() => {});
    await page.keyboard.press("ArrowUp").catch(() => {});
  }
  await page.waitForTimeout(3500);
  const afterSettings = await census();
  verdict("after-settings-bump", `rail=${afterSettings.railSpacing} settings=${afterSettings.settingsSpacing}`);
}

if (booted && kase === "outliner-toggle") {
  const zero = await census();
  verdict("boot-document", `rows=${zero.documentRows.length} tabs=${JSON.stringify(zero.tabs.map((t) => t.id))}`);
  const first = await clickId("framework.panel.artifact");
  await page.waitForTimeout(2500);
  const afterFirst = await census();
  verdict("after-first-artifact-click", `clicked=${first} rows=${afterFirst.documentRows.length}`);
  await clickId("framework.panel.artifact");
  await page.waitForTimeout(2500);
  const afterSecond = await census();
  verdict("after-second-artifact-click", `rows=${afterSecond.documentRows.length}`);
}

if (booted && kase === "history-toggle") {
  const first = await clickId("framework.panel.history");
  await page.waitForTimeout(2500);
  const afterFirst = await census();
  verdict("after-first-history-click", `clicked=${first} entries=${afterFirst.historyEntries.length}`);
  await clickId("framework.panel.history");
  await page.waitForTimeout(2500);
  const afterSecond = await census();
  verdict("after-second-history-click", `entries=${afterSecond.historyEntries.length}`);
  await clickId("framework.panel.history");
  await page.waitForTimeout(2500);
  const afterThird = await census();
  verdict("after-third-history-click", `entries=${afterThird.historyEntries.length}`);
}

if (booted && kase === "volume-arm") {
  const unfold = page.locator('[id$="utilityBar.unfold"]').last();
  if (await unfold.count()) await unfold.click({ timeout: 4000 }).catch(() => {});
  await page.waitForTimeout(1500);
  const utilities = await page.evaluate(() => Array.from(document.querySelectorAll('[id="brush"], [id="volumeBrush"], [id="transform"], [id="worldRelocate"]')).map((el) => `${el.id}=${el.getAttribute("aria-pressed")}`));
  verdict("utilities", JSON.stringify(utilities));
  await clickId("brush");
  await page.waitForTimeout(4000);
  const armedBrush = await census();
  verdict("after-arm-brush", `utility=${JSON.stringify(armedBrush.selected.map((s) => s.activeUtility))}`);
  await page.keyboard.press("Escape");
  await page.waitForTimeout(3000);
  const afterEscape = await census();
  verdict("after-escape", `utility=${JSON.stringify(afterEscape.selected.map((s) => s.activeUtility))}`);
  await clickId("volumeBrush");
  await page.waitForTimeout(5000);
  const armedVolume = await census();
  verdict("after-arm-volume", `utility=${JSON.stringify(armedVolume.selected.map((s) => s.activeUtility))}`);
}

if (booted && kase === "dup-delete") {
  await pickCanvas();
  const selected = await census();
  verdict("selected", `selected=${JSON.stringify(selected.selected)} instances=${selected.instances.length}`);
  const poll = async (predicate: (ids: string[]) => boolean, budgetMs: number) => {
    const start = Date.now();
    let ids = (await census()).instances;
    while (!predicate(ids) && Date.now() - start < budgetMs) {
      await page.waitForTimeout(500);
      ids = (await census()).instances;
    }
    return { ids, waitedMs: Date.now() - start, ok: predicate(ids) };
  };
  const beforeDup = (await census()).instances;
  await page.keyboard.press("Meta+d");
  const dup = await poll((ids) => ids.length > beforeDup.length, 20000);
  verdict("duplicate", `before=${beforeDup.length} after=${dup.ids.length} waitedMs=${dup.waitedMs} ok=${dup.ok}`);
  const settle = await poll((ids) => ids.length === dup.ids.length, 3000);
  const beforeDelete = settle.ids;
  await page.keyboard.press("Delete");
  const removal = await poll((ids) => ids.length < beforeDelete.length, 20000);
  verdict("delete", `before=${beforeDelete.length} after=${removal.ids.length} waitedMs=${removal.waitedMs} ok=${removal.ok}`);
  const late = await poll((ids) => ids.length !== removal.ids.length, 8000);
  verdict("delete-late-arrival", `settled=${removal.ids.length} later=${late.ids.length} changed=${late.ok}`);
}

writeFileSync(join(OUT, `b36-${kase}-${stamp}.txt`), lines.join("\n"));
writeFileSync(join(OUT, `b36-${kase}-${stamp}.console.txt`), console_.join("\n"));
log(`done → 🗑️generated/b36-${kase}-${stamp}.txt (+ .console.txt, ${console_.length} lines)`);
await browser.close();
process.exit(0);
