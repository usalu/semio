/** 🌐️ Exercises the served Draw guest through real pointer input and independently parses its export. */
import assert from "node:assert/strict";
import { existsSync, mkdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { setTimeout as delay } from "node:timers/promises";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { createRequire } from "node:module";
const require = createRequire('/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🟦️.mjs');
const { chromium } = require("playwright");
const ajv = require("ajv");
const Ajv = ajv.default ?? ajv;
import { assertPdfDifference, assertPdfPainting } from '/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🔮️oracle.mjs';

const root = '/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser';
const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(root, "📐️.json"), "utf8")));
assert(validate(fixture), JSON.stringify(validate.errors));
const args = process.argv.slice(2);
const flag = name => args[args.indexOf(name) + 1];
for (const name of ["--serve", "--locale", "--output"]) assert(args.includes(name) && flag(name) && !flag(name).startsWith("--"), `${name} required`);
assert(fixture.locales.includes(flag("--locale")), "Explicit supported locale required");
const output = resolve(flag("--output"));
mkdirSync(output, { recursive: true });
const folder = join(output, "document");
assert(join(folder, ".semio", "canonical-bootstrap.db").length <= 256, "Generated persistence path exceeds 256 characters");
assert(!existsSync(folder), "Acceptance requires a fresh document folder; choose a new output directory");
mkdirSync(folder, { recursive: true });
const url = new URL(flag("--serve"));
url.searchParams.set("plugin", "draw");
const report = { url: url.href, locale: flag("--locale"), steps: [], errors: [], debug: [] };
const browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
const cancellation = new AbortController();
const cancel = signal => {
  cancellation.abort();
  report.failure = `Cancelled by ${signal}`;
  process.exitCode = 1;
  void browser.close().catch(() => undefined);
};
const interrupt = () => cancel("SIGINT");
const terminate = () => cancel("SIGTERM");
process.once("SIGINT", interrupt);
process.once("SIGTERM", terminate);
const context = await browser.newContext({ locale: report.locale, viewport: { width: 1600, height: 1000 }, acceptDownloads: true });
const page = await context.newPage();
const errors = report.errors;
if (process.env.VARIANT === "off") await page.route(url => decodeURIComponent(url.href).includes("handle-turn-drive"), async route => {
  const response = await route.fetch();
  const before = await response.text();
  const body = before.replace("PLUGIN_JOB_PROGRESS_SHARE = 4", "PLUGIN_JOB_PROGRESS_SHARE = 0").replace("if (published || isolatedJobUiPollEverySteps(step))", "if (progressed.value || isolatedJobUiPollEverySteps(step))");
  if (before.includes("createSpawnedJobDriver") && body === before) report.unpatched = true;
  await route.fulfill({ response, body });
});
const workerObservers = [];
page.on("worker", worker => {
  workerObservers.push(worker.evaluate(() => {
    self.addEventListener("error", event => console.error("Draw worker error", event.message));
    self.addEventListener("unhandledrejection", event => console.error("Draw worker rejection", String(event.reason)));
  }).catch(error => errors.push(`Worker observation failed: ${error}`)));
});
page.on("pageerror", error => errors.push(String(error)));
page.on("console", message => {
  const value = message.text();
  if (value.startsWith("[DEBUG] ")) return void report.debug.push(value);
  if (message.type() === "error" || message.type() === "warning" && value.includes("refused: dispatch-failed")) errors.push(value);
});
const engagement = page.locator('[id^="framework.window."][id$=".engagement.toggle"]').first();
const statusSelector = '[id="drawing-composite"] [data-slot="engagement-status"]';
const status = page.locator(statusSelector);
const setEngagement = async open => {
  if (await status.isVisible() !== open) await engagement.click({ timeout: 15000 });
  await status.waitFor({ state: open ? "visible" : "hidden", timeout: 15000 });
};
const state = () => page.evaluate(selector => {
  const status = document.querySelector(selector)?.textContent?.match(/(\d+) (?:layers?|Ebenen?) · (\d+) (?:selected|ausgewählt)/);
  return { ready: document.documentElement.dataset.semioOsReady, error: document.documentElement.dataset.semioOsError, layers: status ? Number(status[1]) : null, locale: status ? (status[0].includes("ausgewählt") ? "de" : "en") : null };
}, statusSelector);
const waitLayers = async (expected, minimum = false) => {
  await setEngagement(true);
  await page.waitForFunction(({ selector, expected, minimum }) => {
    const status = document.querySelector(selector)?.textContent?.match(/(\d+) (?:layers?|Ebenen?) · (\d+) (?:selected|ausgewählt)/);
    if (!status) return false;
    const layers = Number(status[1]);
    return minimum ? layers >= expected : layers === expected;
  }, { selector: statusSelector, expected, minimum }, { timeout: 30000 });
};
const initialEventLogBytes = async path => {
  const deadline = Date.now() + 30000;
  while (Date.now() < deadline) {
    const bytes = statSync(path, { throwIfNoEntry: false })?.size ?? 0;
    if (bytes > 0) return bytes;
    await delay(100, undefined, { signal: cancellation.signal });
  }
  throw Error("Attaching the folder did not write its initial event log");
};
const click = id => page.locator(`[id="${id}"]`).first().click({ timeout: 15000 });
const syncPanel = page.locator('[data-slot="panel"][id="framework.panelTab.s-sync-status"]').first();
const closeSyncPanel = async () => {
  await page.keyboard.press("Escape");
  if (await syncPanel.waitFor({ state: "hidden", timeout: 2000 }).then(() => false, () => true)) await click("s-sync-status");
  await syncPanel.waitFor({ state: "hidden", timeout: 15000 });
};
const openFolderCard = async () => {
  if (!(await syncPanel.isVisible())) await click("s-sync-status");
  const utility = page.locator('[id="framework.sync.folder"]').first();
  if (!(await utility.isVisible())) await click("ui.utilities.group.sync");
  await utility.click({ timeout: 15000 });
  await page.locator('[id="framework.sync.folder.path"]').waitFor({ state: "visible", timeout: 15000 });
};
const waitSaved = async () => {
  await page.waitForFunction(() => [...document.querySelectorAll('[data-semio-sync-status]')].some(element => /(?:^|·)\s*(?:saved|gespeichert)(?:\s*·|$)/i.test(element.textContent ?? "")), undefined, { timeout: 60000 });
};
const note = async step => {
  await Promise.all(workerObservers);
  assert.equal(errors.length, 0, errors.join("\n"));
  await setEngagement(true);
  const current = await state();
  assert.equal(current.ready, fixture.ready, JSON.stringify(current));
  assert.equal(current.error, undefined, JSON.stringify(current));
  assert.equal(current.locale, report.locale, "Draw must render the explicitly requested language");
  report.steps.push({ step, ...current });
  writeFileSync(join(output, "report.json"), JSON.stringify(report, null, 2));
  await setEngagement(false);
  if (step === "rectangle" || step === "reload") await page.screenshot({ path: join(output, `${step}.png`) });
  console.log(`[draw-browser] ${step}`);
};
const exportPdf = async name => {
  await setEngagement(true);
  const format = page.locator('[id$=".action.exportDocument.arg.format.input"][role="combobox"]').first();
  if (!(await format.isVisible())) await click("action.exportDocument");
  await format.click({ timeout: 15000 });
  await page.getByRole("option", { name: "PDF", exact: true }).click({ timeout: 15000 });
  console.log("[draw-browser] waiting for PDF export");
  const [download] = await Promise.all([
    page.waitForEvent("download", { timeout: 60000 }),
    page.locator('[id^="framework.window."][id$=".action.exportDocument.execute"]').first().click({ timeout: 15000 }),
  ]);
  const pdfPath = join(output, name);
  await download.saveAs(pdfPath);
  assert.equal(await download.failure(), null);
  const pixels = await assertPdfPainting(readFileSync(pdfPath), fixture.export.pages);
  await setEngagement(false);
  return pixels;
};
await page.addInitScript(faults => {
  const NativeWorker = globalThis.Worker;
  globalThis.Worker = class extends NativeWorker {
    constructor(...arguments_) {
      super(...arguments_);
      this.addEventListener("message", event => {
        if (faults.includes(event.data?.kind)) console.error("Draw guest fault", JSON.stringify(event.data));
      });
      this.addEventListener("error", event => console.error("Draw worker error", event.message));
      this.addEventListener("messageerror", () => console.error("Draw worker message error"));
    }
  };
}, fixture.guestFaults);
try {
  console.log("[draw-browser] opening default Draw session");
  await page.goto(url.href, { waitUntil: "domcontentloaded", timeout: 60000 });
  await page.waitForFunction(() => document.documentElement.dataset.semioOsReady || document.documentElement.dataset.semioOsError, undefined, { timeout: 90000 });
  assert.equal((await state()).ready, fixture.ready, JSON.stringify(await state()));
  const canvas = page.locator('[data-surface-role="editor"] [data-ui-node-key="drawing.play.composite"] canvas').first();
  await canvas.waitFor({ state: "visible", timeout: 30000 });
  const picker = page.locator('[id="playground.navbar.fixture"]').first();
  await picker.getByText(fixture.example.label, { exact: true }).waitFor({ state: "visible", timeout: 60000 });
  await waitLayers(fixture.minimumBootLayers, true);
  const before = (await state()).layers;
  await setEngagement(false);
  const labels = report.locale === "de"
    ? { artifact: "Dokument", catalogue: "Katalog", inspection: "Inspektion", addPath: "Pfad hinzufügen", path: "Pfad" }
    : { artifact: "Artifact", catalogue: "Catalogue", inspection: "Inspection", addPath: "Add Path", path: "Path" };
  const openPanel = async (id, name) => {
    const button = page.locator(`button[id="${id}"]`).first();
    assert((await button.innerText()).includes(name), `Localized panel label required: ${name}`);
    const panel = page.locator(`[data-slot="panel"][id="framework.panelTab.${id}"]`).first();
    if (!(await panel.isVisible())) await button.click({ timeout: 15000 });
    await panel.waitFor({ state: "visible", timeout: 15000 });
    return panel;
  };
  const layersPanel = await openPanel("framework.panel.artifact", labels.artifact);
  await layersPanel.getByText(labels.addPath, { exact: true }).first().waitFor({ state: "visible", timeout: 15000 });
  const layerRow = layersPanel.locator(`[role="treeitem"][id="panel:framework.panel.artifact/drawing-play-layers.path.${fixture.exampleLayer}"]`);
  await layerRow.click({ timeout: 15000 });
  const inspectionPanel = await openPanel("framework.panel.inspection", labels.inspection);
  await inspectionPanel.locator(`[id*="/drawing-inspector."]`).first().waitFor({ state: "visible", timeout: 15000 });
  const cataloguePanel = await openPanel("framework.panel.catalogue", labels.catalogue);
  await cataloguePanel.getByText(labels.path, { exact: true }).first().waitFor({ state: "visible", timeout: 15000 });
  await openPanel("framework.panel.artifact", labels.artifact);
  await note("boot");
  await picker.click({ timeout: 15000 });
  await page.getByRole("option", { name: fixture.example.label, exact: true }).waitFor({ state: "visible", timeout: 15000 });
  assert.deepEqual((await page.getByRole("option").allInnerTexts()).map(text => text.trim()), [fixture.emptyExample.labels[report.locale], fixture.example.label, fixture.blankExample.labels[report.locale]], "Draw advertises the empty document, Demo and Blank Drawing");
  await page.getByRole("option", { name: fixture.blankExample.labels[report.locale], exact: true }).click({ timeout: 15000 });
  await waitLayers(0);
  await note("blank");
  await picker.click({ timeout: 15000 });
  await page.getByRole("option", { name: fixture.example.label, exact: true }).click({ timeout: 15000 });
  await waitLayers(before);
  assert((await picker.innerText()).includes(fixture.example.label));
  await note("example");
  await openFolderCard();
  await page.locator('[id="framework.sync.folder.path"]').fill(folder);
  await page.getByRole("button", { name: /^(attach|verbinden)$/i }).click({ timeout: 15000 });
  await page.locator('[id="framework.sync.folder.path"]').waitFor({ state: "hidden", timeout: 15000 });
  await openFolderCard();
  await page.getByRole("button", { name: /^(detach|trennen)$/i }).waitFor({ state: "visible", timeout: 15000 });
  await closeSyncPanel();
  await note("attach");
  const eventPath = join(folder, ".semio", "events.semio");
  const beforeEventBytes = await initialEventLogBytes(eventPath);
  const beforePixels = await exportPdf("before.pdf");
  await page.locator('[id^="framework.window."][id$=".utilityBar.unfold"]').first().click({ timeout: 15000 });
  await page.locator('[id^="ui.utilities."][id$=".group.group:Drawing"]').first().click({ timeout: 15000 });
  for (const pressed of fixture.utilityPressStates) {
    await click(fixture.utility);
    await page.waitForFunction(({ id, pressed }) => {
      return document.getElementById(id)?.getAttribute("aria-pressed") === String(pressed);
    }, { id: fixture.utility, pressed }, { timeout: 15000 });
  }
  const rect = await canvas.boundingBox();
  assert(rect && rect.width > 0 && rect.height > 0, "Nonempty Draw canvas required");
  const [x0, y0] = fixture.gesture.from, [x1, y1] = fixture.gesture.to;
  await page.mouse.move(rect.x + rect.width * x0, rect.y + rect.height * y0);
  await page.mouse.down();
  await page.mouse.move(rect.x + rect.width * x1, rect.y + rect.height * y1, { steps: fixture.gesture.steps });
  await page.mouse.up();
  await waitLayers(before + fixture.layerDelta);
  await note("rectangle");
  const canvasSurface = page.locator('[data-surface-role="editor"] [data-ui-node-key="drawing.play.composite"]').first();
  const mod = process.platform === "darwin" ? "Meta" : "Control";
  await canvasSurface.focus();
  await page.keyboard.press(`${mod}+z`);
  await waitLayers(before);
  await note("undo");
  await canvasSurface.focus();
  await page.keyboard.press(`${mod}+Shift+z`);
  await waitLayers(before + fixture.layerDelta);
  await note("redo");
  await openFolderCard();
  await waitSaved();
  report.persistence = { folder, beforeEventBytes, eventLogBytes: statSync(eventPath).size };
  assert(report.persistence.eventLogBytes - beforeEventBytes >= fixture.persistence.minimumEventGrowthBytes, "Current rectangle/history events must grow the fresh folder log");
  await closeSyncPanel();
  await note("persisted");
  report.export = assertPdfDifference(beforePixels, await exportPdf("drawing.pdf"));
  await note("export");
  console.log("[draw-browser] reloading persisted document");
  await page.reload({ waitUntil: "domcontentloaded", timeout: 60000 });
  await page.waitForFunction(() => document.documentElement.dataset.semioOsReady || document.documentElement.dataset.semioOsError, undefined, { timeout: 90000 });
  assert.equal((await state()).ready, fixture.ready, JSON.stringify(await state()));
  assert.equal((await state()).error, undefined, JSON.stringify(await state()));
  await click(fixture.persistence.reconnect);
  await waitLayers(before + fixture.layerDelta);
  await note("reload");
  assert.deepEqual(report.steps.map(row => row.step), fixture.steps);
} catch (error) {
  report.failure ??= String(error);
  await page.screenshot({ path: join(output, "failure.png") }).catch(() => undefined);
  throw error;
} finally {
  writeFileSync(join(output, "report.json"), JSON.stringify(report, null, 2));
  await browser.close();
  process.off("SIGINT", interrupt);
  process.off("SIGTERM", terminate);
}
