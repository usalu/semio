/** 🌐️ Exercises the served Draw guest through real pointer input and independently parses its export. */
import assert from "node:assert/strict";
import { existsSync, mkdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { setTimeout as delay } from "node:timers/promises";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { createRequire } from "node:module";
const require = createRequire(resolve("✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🟦️.mjs"));
const { chromium } = require("playwright");
const Ajv = require("ajv");

const root = resolve("✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser");
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
await page.routeWebSocket(address => address.host === url.host && address.searchParams.has("token"), () => undefined);
report.documentLoads = 0;
page.on("domcontentloaded", () => report.documentLoads++);
const errors = report.errors;
const workerObservers = [];
page.on("worker", worker => {
  worker.on("console", m => { if (process.env.PROBE_DIAG) console.log("[DEBUG] W", Date.now() % 1000000, (m.text().includes("archive/replacement wait") ? m.text().replace(/grant=Retained[^}]*}/, "").slice(0, 20000) : m.text().slice(0, 600))); });
  workerObservers.push(worker.evaluate(() => {
    self.addEventListener("error", event => console.error("Draw worker error", event.message));
    self.addEventListener("unhandledrejection", event => console.error("Draw worker rejection", String(event.reason)));
  }).catch(error => errors.push(`Worker observation failed: ${error}`)));
});
page.on("pageerror", error => errors.push(String(error)));
page.on("console", message => {
  const value = message.text();
  if (value.startsWith("[DEBUG] ")) { console.log("[DEBUG] P", Date.now() % 1000000, value.slice(0, 300)); return void report.debug.push(value); }
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
  return { ready: document.documentElement.dataset.semioOsReady, error: document.documentElement.dataset.semioOsError, layers: status ? Number(status[1]) : null, selected: status ? Number(status[2]) : null, locale: status ? (status[0].includes("ausgewählt") ? "de" : "en") : null };
}, statusSelector);
const waitStatus = async (field, expected, minimum = false, restoring = false) => {
  const started = Date.now();
  let deadline = started + 30000;
  let lastProgress = null;
  let observed = null;
  do {
    assert.equal(errors.length, 0, errors.join("\n"));
    await setEngagement(true);
    observed = (await state())[field];
    const progress = restoring ? await page.locator('[data-semio-history-reprojection="load"] [data-semio-history-reprojection-announcement]').allTextContents() : [];
    if (progress.length && progress[0] !== lastProgress) {
      lastProgress = progress[0];
      deadline = started + 120000;
      (report.replayProgress ??= []).push({ elapsedMs: Date.now() - started, status: lastProgress });
    }
    if (!progress.length && observed !== null && (minimum ? observed >= expected : observed === expected)) return;
    await delay(100, undefined, { signal: cancellation.signal });
  } while (Date.now() < deadline);
  throw Error(`Expected ${minimum ? "at least " : ""}${expected} ${field}, last read ${observed}`);
};
const waitLayers = (expected, minimum) => waitStatus("layers", expected, minimum);
const waitSelected = expected => waitStatus("selected", expected);
const paintedPixels = async (surface, { color, tolerance }) => page.evaluate(async ({ png, color, tolerance }) => {
  const image = await createImageBitmap(await (await fetch(`data:image/png;base64,${png}`)).blob());
  const context = new OffscreenCanvas(image.width, image.height).getContext("2d");
  context.drawImage(image, 0, 0);
  const { data } = context.getImageData(0, 0, image.width, image.height);
  let pixels = 0;
  for (let index = 0; index < data.length; index += 4) if (color.every((value, channel) => Math.abs(data[index + channel] - value) <= tolerance)) pixels += 1;
  return pixels;
}, { png: (await surface.screenshot()).toString("base64"), color, tolerance });
const waitPainted = async (surface, expected) => {
  const deadline = Date.now() + 30000;
  for (;;) {
    const pixels = await paintedPixels(surface, expected);
    if (pixels >= expected.minimumPixels) return pixels;
    if (Date.now() >= deadline) throw Error(`Expected ${expected.minimumPixels} pixels of ${expected.color}, last counted ${pixels}`);
    await delay(250, undefined, { signal: cancellation.signal });
  }
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
  if (step === "rectangle" || step === "reload") await page.screenshot({ path: join(output, `${step}.png`) });
  console.log(`[draw-browser] ${step}`);
};
const exportPdf = async name => {
  const started = performance.now();
  await setEngagement(true);
  const format = page.locator('[id$=".action.exportDocument.arg.format.input"][role="combobox"]').first();
  if (!(await format.isVisible())) await click("action.exportDocument");
  await format.click({ timeout: 15000 });
  await page.getByRole("option", { name: "PDF", exact: true }).click({ timeout: 15000 });
  console.log("[draw-browser] waiting for PDF export");
  const [download] = await Promise.all([
    page.waitForEvent("download", { timeout: 120000 }),
    page.locator('[id^="framework.window."][id$=".action.exportDocument.execute"]').first().click({ timeout: 15000 }),
  ]);
  const pdfPath = join(output, name);
  await download.saveAs(pdfPath);
  assert.equal(await download.failure(), null);
  const pixels = await assertPdfPainting(readFileSync(pdfPath), fixture.export.pages);
  (report.exportDurationsMs ??= {})[name] = Math.round(performance.now() - started);
  await setEngagement(false);
  return pixels;
};
await page.addInitScript(() => {
  const W = globalThis.Worker;
  const t0 = performance.now();
  globalThis.__msgs = [];
  const describe = d => { try { if (d && typeof d === "object") { const keys = Object.keys(d); const k = d.kind ?? d.type ?? keys[0]; const parts = keys.slice(0, 8).map(key => { const v = d[key]; return `${key}=${v instanceof ArrayBuffer || ArrayBuffer.isView(v) ? `bytes${v.byteLength}` : typeof v === "bigint" ? `${v}n` : typeof v === "object" && v ? `{${Object.keys(v).slice(0, 5).join(",")}}` : String(v).slice(0, 50)}`; }); const st = d.value && typeof d.value === "object" && d.value.status !== undefined ? ` status=${JSON.stringify(d.value.status, (kk, v) => typeof v === "bigint" ? `${v}n` : ArrayBuffer.isView(v) ? `bytes${v.byteLength}` : v).slice(0, 80)}` : ""; return `${k}|${parts.join(" ")}${st}`; } return typeof d + ":" + String(d).slice(0, 80); } catch (e) { return "?" + e; } };
  globalThis.Worker = class extends W {
    constructor(...a) { super(...a); this.addEventListener("message", e => globalThis.__msgs.push([Math.round(performance.now() - t0), "in", describe(e.data)])); const post = this.postMessage.bind(this); this.postMessage = (m, ...r) => { globalThis.__msgs.push([Math.round(performance.now() - t0), "out", describe(m)]); return post(m, ...r); }; }
  };
});
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
  const canvasSurface = page.locator('[data-surface-role="editor"] [data-ui-node-key="drawing.play.composite"]').first();
  report.bootPixels = await waitPainted(canvasSurface, fixture.bootPaint);
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
  const only = process.env.PANELS ?? "artifact,select,inspection,catalogue,artifact2";
  const want = only.split(",");
  const layersPanel = want.includes("artifact") ? await openPanel("framework.panel.artifact", labels.artifact) : null;
  if (layersPanel) {
    await layersPanel.getByText(labels.addPath, { exact: true }).first().waitFor({ state: "visible", timeout: 15000 });
    if (want.includes("select")) { await layersPanel.locator(`[role="treeitem"][id="panel:framework.panel.artifact/${fixture.exampleLayer}"]`).click({ timeout: 15000 }); await waitSelected(1); }
  }
  if (want.includes("inspection")) { const p = await openPanel("framework.panel.inspection", labels.inspection); await p.locator(`[id*="/drawing-inspector."]`).first().waitFor({ state: "visible", timeout: 15000 }); }
  if (want.includes("catalogue")) { const p = await openPanel("framework.panel.catalogue", labels.catalogue); await p.getByText(labels.path, { exact: true }).first().waitFor({ state: "visible", timeout: 15000 }); }
  if (want.includes("artifact2")) await openPanel("framework.panel.artifact", labels.artifact);
  if (!process.env.NONOTE) await note("boot");
  await picker.click({ timeout: 15000 });
  await page.getByRole("option", { name: fixture.example.label, exact: true }).waitFor({ state: "visible", timeout: 15000 });
  await page.getByRole("option", { name: fixture.blankExample.labels[report.locale], exact: true }).click({ timeout: 15000 });
  console.log("[DEBUG] CLICKED", Date.now() % 1000000);
  await page.evaluate(() => globalThis.__msgs.push([Math.round(performance.now()), "mark", "switch"]));
  const t = Date.now(); report.switchAt = await page.evaluate(() => Math.round(performance.now() - 0)); 
  try { await waitLayers(0); console.log("[DEBUG] OK blank", Date.now() - t);  } catch (e) { console.log("[DEBUG] STALL", String(e));
    if (false) {
      const cdp = await context.newCDPSession(page);
      const targets = []; const messages = [];
      cdp.on("Target.attachedToTarget", ev => targets.push(ev.sessionId + " " + ev.targetInfo.type + " " + ev.targetInfo.url.slice(-60)));
      cdp.on("Target.receivedMessageFromTarget", ev => messages.push(ev));
      await cdp.send("Target.setAutoAttach", { autoAttach: true, waitForDebuggerOnStart: false, flatten: false });
      await delay(1500);
      console.log("[DEBUG] targets", JSON.stringify(targets));
      const workerTargets = [];
      for (const line of targets) { const [sessionId, type] = line.split(" "); if (type === "worker") workerTargets.push(sessionId); }
      const send = (sessionId, id, method, params = {}) => cdp.send("Target.sendMessageToTarget", { sessionId, message: JSON.stringify({ id, method, params }) });
      for (const sessionId of workerTargets) { await send(sessionId, 1, "Profiler.enable"); await send(sessionId, 2, "Profiler.setSamplingInterval", { interval: 200 }); await send(sessionId, 3, "Profiler.start"); }
      await delay(6000);
      for (const sessionId of workerTargets) await send(sessionId, 4, "Profiler.stop");
      await delay(3000);
      for (const ev of messages) { try { const msg = JSON.parse(ev.message); if (msg.id === 4 && msg.result?.profile) writeFileSync(join(output, `profile-${ev.sessionId}.json`), JSON.stringify(msg.result.profile)); } catch {} }
      console.log("[DEBUG] profiled", workerTargets.length);
    } writeFileSync(join(output, "msgs.json"), JSON.stringify(await page.evaluate(() => globalThis.__msgs))); await page.screenshot({ path: join(output, "stall.png") });
    const settled = async (label, ms) => { const end = Date.now() + ms; while (Date.now() < end) { if ((await state()).layers === 0) { console.log("[DEBUG] UNSTUCK by", label, Date.now() - t); writeFileSync(join(output, "msgs-after.json"), JSON.stringify((await page.evaluate(() => globalThis.__msgs)).filter(m => m[1] !== "in" || !String(m[2]).startsWith("original-turn-receipt")))); await page.evaluate(() => globalThis.__msgs.push([Math.round(performance.now()), "mark", "unstuck"])); return true; } await delay(500); } return false; };
    const steps = [
      ["eval-only-wait", async () => {}, 20000],
      ["screenshot", async () => { await page.screenshot({ path: join(output, "nudge.png") }); }, 6000],
      ["mouse-canvas", async () => { const b = await canvasSurface.boundingBox(); await page.mouse.move(b.x + 200, b.y + 200); await page.mouse.move(b.x + 260, b.y + 240); }, 8000],
      ["wait60", async () => {}, 60000],
    ];
    for (const [label, act, ms] of steps) { await act().catch(error => console.log("[DEBUG] nudge failed", label, String(error).slice(0, 120))); console.log("[DEBUG] nudge", label); if (await settled(label, ms)) break; }
  }
} finally { await browser.close(); }
