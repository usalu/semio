/** 🎛️ Current layout UI edits, exact history, independent media and new-owner archive round-trip. */
import assert from "node:assert/strict";
import { repoToolCacheEnv } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { validateMedia, compareEditedGeometry, validateCanvasCapture } from "./media-oracles.mjs";
import { assertSurfaceHealthy } from "./browser-health.mjs";

const ticket = dirname(fileURLToPath(import.meta.url));
const contract = JSON.parse(readFileSync(join(ticket, "browser-journey.json"), "utf8"));
const runName = (process.env.SEMIO_PROBE_OUT ?? "layout-current").replace(/[^a-zA-Z0-9_.-]/g, "-");
assert.ok(runName !== "." && runName !== "..", "generated output directory name");
const outDir = join(ticket, "🗑️generated", runName);
mkdirSync(outDir, { recursive: true });
const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6079/?plugin=layout";
const report = { url, contractVersion: contract.version, steps: [], complete: false };
const lines = [];
const start = Date.now();
let browser;
let page;
let cancellation = null;
let hardExit;
const cancel = reason => {
  if (cancellation) return;
  cancellation = { reason, t: Date.now() - start };
  report.failure = { cancellation };
  report.complete = false;
  write();
  void browser?.close().catch(() => {});
  hardExit = setTimeout(() => { write(); process.exit(reason === "deadline" ? 124 : reason === "SIGINT" ? 130 : 143); }, 5000);
};
const deadline = setTimeout(() => cancel("deadline"), 600000);
const onInterrupt = () => cancel("SIGINT");
const onTerminate = () => cancel("SIGTERM");
process.once("SIGINT", onInterrupt);
process.once("SIGTERM", onTerminate);
const attr = (name, value) => "[" + name + "=" + JSON.stringify(value) + "]";
const write = () => {
  writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(outDir, "console.txt"), lines.map(line => JSON.stringify(line)).join("\n"));
};
try {
  process.env.PLAYWRIGHT_BROWSERS_PATH = repoToolCacheEnv(join(ticket, "../../../../../../..")).PLAYWRIGHT_BROWSERS_PATH;
  const { chromium } = await import("playwright");
  report.browser = { executable: chromium.executablePath(), browsers: process.env.PLAYWRIGHT_BROWSERS_PATH };
  browser = await chromium.launch({ downloadsPath: join(outDir, "downloads"), headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
  if (cancellation) throw Error("Browser journey cancelled: " + cancellation.reason);
  const context = await browser.newContext({ viewport: { width: 1900, height: 1200 }, acceptDownloads: true });
  page = await context.newPage();
  page.on("console", message => lines.push({ t: Date.now() - start, type: message.type(), text: message.text() }));
  page.on("pageerror", error => lines.push({ t: Date.now() - start, type: "pageerror", text: String(error) }));
  page.setDefaultTimeout(15000);

  const artifact = () => page.locator('[data-slot="panel"][data-panel-visible="true"][data-active-tab-id="framework.panel.artifact"]');
  const tabs = () => page.locator('[data-slot="mode-dock-tab"]');
  const windowRoot = owner => page.locator('[data-slot="window"]' + attr("id", owner));
  const snapshot = () => page.evaluate(() => {
    const visible = element => !!(element.getClientRects().length && getComputedStyle(element).visibility !== "hidden");
    const panels = [...document.querySelectorAll('[data-slot="panel"][data-panel-visible="true"][data-active-tab-id="framework.panel.artifact"]')].filter(visible);
    const rows = panels.length === 1 ? [...panels[0].querySelectorAll('[role="treeitem"]')] : [];
    const windows = [...document.querySelectorAll('[data-slot="mode-dock-tab"]')];
    const active = windows.filter(node => node.getAttribute("data-active") === "true");
    const hosts = [...document.querySelectorAll("[data-surface-id]")].filter(visible).map(node => {
      return {
        id: node.getAttribute("data-surface-id"),
        owner: node.closest('[data-slot="window"]')?.id ?? null,
        statusJson: node.getAttribute("data-status-json"),
        boardStatusJson: node.getAttribute("data-board-status-json"),
        phase: node.getAttribute("data-phase"),
        fault: node.getAttribute("data-fault"),
        canvases: [...node.querySelectorAll("canvas")].filter(visible).map(canvas => ({ width: canvas.width, height: canvas.height })),
      };
    });
    return {
      ready: document.documentElement.getAttribute("data-semio-os-ready"),
      error: document.documentElement.getAttribute("data-semio-os-error"),
      artifactPanels: panels.length,
      frameIds: rows.filter(row => /^frame-/.test(row.id)).map(row => row.id).sort(),
      pageIds: rows.filter(row => row.id.startsWith("layout-document.page.")).map(row => row.id).sort(),
      selected: rows.filter(row => row.getAttribute("aria-selected") === "true").map(row => row.id).sort(),
      ownerWindowId: active.length === 1 ? active[0].getAttribute("data-window-id") : null,
      windowIds: [...new Set(windows.map(node => node.getAttribute("data-window-id")))].sort(),
      hosts,
      surfaceFaults: [...document.querySelectorAll("[data-semio-window-fault],[data-shell-fault-boundary],[data-semio-time-travel-fault],[data-semio-os-shell-error]")].filter(visible).map(node => ({ id: node.id, fault: node.getAttribute("data-semio-window-fault"), code: node.getAttribute("data-semio-window-fault-code"), text: node.textContent?.slice(0, 400) })),
    };
  });

  const healthy = state => {
    assert.equal(state.ready, "layout", "selected layout component is ready");
    assert.equal(state.error, null, "shell boot error");
    assert.deepEqual(state.surfaceFaults, [], "visible surface fault");
    for (const host of state.hosts) assertSurfaceHealthy(host);
  };
  const faults = from => lines.slice(from).filter(line => {
    if (line.text.includes("contributions push refused empty pack")) return false;
    return line.type === "error" || line.type === "pageerror" || /trapped|panicked|action failed|shell fault|unreachable|Fault \{|not-ui-safe|missing-owned|import.*(?:failed|unreadable|cancelled)/i.test(line.text);
  });
  const record = async (step, detail, from = 0, state = undefined) => {
    const observed = state ?? await snapshot();
    healthy(observed);
    const errors = faults(from);
    assert.deepEqual(errors, [], step + " console failures");
    report.steps.push({ step, t: Date.now() - start, detail, state: observed, faults: errors });
    write();
    await page.screenshot({ path: join(outDir, String(report.steps.length).padStart(2, "0") + "-" + step + ".png") });
    console.log("[DEBUG] " + step + " " + JSON.stringify(detail).slice(0, 1600));
  };
  const poll = async (label, predicate, timeout = 45000) => {
    const until = Date.now() + timeout;
    let last;
    while (Date.now() < until) {
      last = await snapshot();
      if (last.error || last.surfaceFaults.length) healthy(last);
      if (await predicate(last)) { healthy(last); return last; }
      await page.waitForTimeout(200);
    }
    assert.fail(label + " timed out: " + JSON.stringify(last));
  };
  const one = async (locator, label) => {
    await locator.waitFor({ state: "visible" });
    assert.equal(await locator.count(), 1, label + " has one control");
    assert.ok(await locator.isVisible(), label + " is visible");
    const until = Date.now() + 10000;
    while (!await locator.isEnabled() && Date.now() < until) await page.waitForTimeout(100);
    assert.ok(await locator.isEnabled(), label + " is enabled");
    return locator;
  };
  const togglePanel = async id => {
    const tab = await one(page.locator(attr("data-tab-id", id) + ":visible"), id);
    await tab.click();
  };
  const openArtifact = async () => {
    if (await artifact().count() === 0) await togglePanel("framework.panel.artifact");
    await one(artifact(), "focused Artifact panel");
  };
  const closeArtifact = async () => {
    if (await artifact().count()) {
      await togglePanel("framework.panel.artifact");
      await page.waitForFunction(() => document.querySelector('[data-slot="panel"][data-panel-visible="true"][data-active-tab-id="framework.panel.artifact"]') === null);
    }
  };
  const focusOwner = async owner => {
    await closeArtifact();
    const tab = await one(tabs().and(page.locator(attr("data-window-id", owner))), "owner dock tab");
    await tab.click();
    await poll("active owner", state => state.ownerWindowId === owner);
    await one(windowRoot(owner), "active owner window");
  };
  const pane = owner => windowRoot(owner).locator('[data-slot="window-action-pane"]');
  const openActions = async owner => {
    await one(windowRoot(owner), "action owner");
    if (await pane(owner).count() === 0 || !await pane(owner).isVisible()) {
      const toggle = windowRoot(owner).locator('[id$=".engagement.toggle"]');
      await (await one(toggle, "Actions toggle")).click();
    }
    await one(pane(owner), "owner Actions pane");
  };
  const stageAction = async (owner, action, args) => {
    await openActions(owner);
    const scope = pane(owner);
    let execute = scope.locator('[id$=".action.' + action + '.execute"]');
    if (await execute.count() === 0) {
      const row = await one(scope.locator(attr("id", "action." + action)), action + " row");
      await row.scrollIntoViewIfNeeded();
      await row.click();
      await page.waitForTimeout(150);
      execute = scope.locator('[id$=".action.' + action + '.execute"]');
      if (Object.keys(args).length === 0 && await execute.count() === 0) return { submitted: "row", action };
    }
    await one(execute, action + " Execute");
    for (const [name, value] of Object.entries(args)) {
      const row = await one(scope.locator(attr("id", "action." + action + ".arg." + name)), action + "." + name);
      if (name === "kind") {
        const trigger = await one(row.locator('[data-slot="select-trigger"][role="combobox"]'), action + ".kind trigger");
        const listboxId = await trigger.getAttribute("aria-controls");
        assert.ok(listboxId, "kind Select names its portal listbox");
        await trigger.click();
        const listbox = await one(page.locator('[role="listbox"]' + attr("id", listboxId) + ":visible"), "kind listbox");
        const option = await one(listbox.locator('[role="option"]' + attr("data-value", String(value))), "kind option " + value);
        await option.click();
        await listbox.waitFor({ state: "hidden" });
        await trigger.waitFor({ state: "visible" });
        assert.equal(await trigger.getAttribute("aria-expanded"), "false", "kind selection closes its popup");
        const verificationId = await trigger.getAttribute("aria-controls");
        assert.ok(verificationId, "kind verification names its listbox");
        await trigger.click();
        const verification = page.locator('[role="listbox"]' + attr("id", verificationId) + ":visible");
        const selected = await one(verification.locator('[role="option"]' + attr("data-value", String(value))), "selected kind option");
        assert.equal(await selected.getAttribute("aria-selected"), "true", "actual kind selection retained");
        await page.keyboard.press("Escape");
        await verification.waitFor({ state: "hidden" });
        assert.equal(await trigger.getAttribute("aria-expanded"), "false", "kind verification closes before history baseline");
      } else {
        const input = row.locator('input:not([type="hidden"]),textarea');
        await (await one(input, action + "." + name + " input")).fill(String(value));
        await input.press("Tab");
      }
    }
    return { submitted: null, action, execute };
  };
  const executeStaged = async staged => {
    if (staged.submitted === "row") return { action: staged.action, submitted: "row" };
    await one(staged.execute, staged.action + " Execute");
    await staged.execute.focus();
    assert.equal(await staged.execute.evaluate(element => document.activeElement === element), true, "Execute owns keyboard focus");
    await staged.execute.press("Enter");
    return { action: staged.action, submitted: "keyboard" };
  };
  const submitAction = async (owner, action, args = {}) => executeStaged(await stageAction(owner, action, args));
  const sameDocument = (left, right) => JSON.stringify(left.frameIds) === JSON.stringify(right.frameIds) && JSON.stringify(left.pageIds) === JSON.stringify(right.pageIds);
  const added = (before, after) => {
    const remaining = [...before];
    const introduced = [];
    for (const id of after) { const index = remaining.indexOf(id); if (index < 0) introduced.push(id); else remaining.splice(index, 1); }
    return { introduced, removed: remaining };
  };
  const selectContentPage = async () => {
    await openArtifact();
    await (await one(artifact().locator('[role="treeitem"]' + attr("id", "layout-document.page.page-1")), "content page row")).click();
  };
  const download = async (owner, action, name) => {
    await closeArtifact();
    await openActions(owner);
    const waiting = page.waitForEvent("download", { timeout: 120000 });
    const submitted = await submitAction(owner, action);
    const result = await waiting;
    assert.equal(await result.failure(), null, name + " download completed");
    assert.ok(result.suggestedFilename().endsWith(name.slice(name.lastIndexOf("."))), name + " download extension");
    const path = join(outDir, name);
    await result.saveAs(path);
    const bytes = readFileSync(path);
    assert.ok(bytes.length > 32, name + " payload");
    return { bytes, path, submitted, filename: result.suggestedFilename() };
  };
  const inspectorText = () => page.locator('[data-slot="panel"][data-panel-visible="true"][data-active-tab-id="framework.panel.inspection"]')
    .locator(attr("id", "layout-play-inspector.global.story.story-1.content"))
    .locator('textarea[id^="layout-play-inspector.global.story.story-1.content.span."][id$=".input"]');
  const openInspector = async () => {
    const visible = page.locator('[data-slot="panel"][data-panel-visible="true"][data-active-tab-id="framework.panel.inspection"]');
    if (await visible.count() === 0) await togglePanel("framework.panel.inspection");
    await one(visible, "focused Inspector panel");
    await one(inspectorText(), "story-1 authored textarea");
  };
  const storyMarker = async (marker, previous) => {
    await openInspector();
    if (previous !== undefined) assert.equal(await inspectorText().inputValue(), previous, "story before edit");
    await inspectorText().fill(marker);
    await inspectorText().press("Tab");
    await page.waitForFunction(expected => {
      const root = document.querySelector('[data-slot="panel"][data-panel-visible="true"][data-active-tab-id="framework.panel.inspection"]');
      const row = root?.querySelector('[id="layout-play-inspector.global.story.story-1.content"]');
      const input = row?.querySelector('textarea[id$=".input"]');
      return input?.value === expected && document.activeElement !== input;
    }, marker);
    assert.equal(await inspectorText().inputValue(), marker, "committed story marker");
  };

  await page.goto(url, { waitUntil: "domcontentloaded" });
  const boot = await poll("layout boot", state => state.ready === "layout" && state.ownerWindowId !== null && state.hosts.some(host => host.canvases.some(canvas => canvas.width > 100 && canvas.height > 100)), 180000);
  let owner = boot.ownerWindowId;
  assert.ok(/blueprint/i.test(owner), "boot focuses a Blueprint owner");
  const canvas = windowRoot(owner).locator('[data-surface-id] canvas:visible').first();
  await one(canvas, "rendered content canvas");
  const canvasBytes = await canvas.screenshot();
  writeFileSync(join(outDir, "boot-canvas.png"), canvasBytes);
  const rendered = await validateCanvasCapture(canvasBytes);
  await record("boot", { ready: boot.ready, error: boot.error, owner, ...rendered }, 0, boot);
  await openArtifact();
  let baseline = await poll("demo Artifact rows", state => state.artifactPanels === 1 && state.frameIds.length >= 3 && state.pageIds.length === contract.demo.pageIds.length);
  assert.deepEqual(baseline.pageIds, contract.demo.pageIds, "exact demo pages");
  for (const id of contract.demo.requiredFrames) assert.ok(baseline.frameIds.includes(id), "demo frame " + id);
  await record("artifact-panel-baseline", { observed: true, frameIds: baseline.frameIds, pageIds: baseline.pageIds });

  await selectContentPage();
  const beforeSvg = await download(owner, "exportSvg", "baseline.svg");
  const beforePng = await download(owner, "exportPng", "baseline.png");
  await validateMedia("svg", beforeSvg.bytes, baseline.pageIds.length, contract.geometry);
  await validateMedia("png", beforePng.bytes, baseline.pageIds.length, contract.geometry);
  await openArtifact();
  await record("baseline-media", { valid: true });

  await closeArtifact();
  const staged = await stageAction(owner, "addFrame", contract.frame);
  assert.equal(staged.submitted, null, "addFrame requires the populated form");
  await openArtifact();
  baseline = await snapshot();
  healthy(baseline);
  assert.equal(baseline.artifactPanels, 1, "Artifact stays mounted during exact history law");
  const fromCreate = lines.length;
  const submission = await executeStaged(staged);
  const created = await poll("one exact frame added", state => state.artifactPanels === 1 && state.frameIds.length === baseline.frameIds.length + 1 && JSON.stringify(state.pageIds) === JSON.stringify(baseline.pageIds));
  const delta = added(baseline.frameIds, created.frameIds);
  assert.deepEqual(delta.removed, [], "creation removes no frame");
  assert.equal(delta.introduced.length, 1, "exactly one new frame ID");
  const createdId = delta.introduced[0];
  await page.keyboard.press(process.platform === "darwin" ? "Meta+z" : "Control+z");
  const undone = await poll("exact undo snapshot", state => state.artifactPanels === 1 && sameDocument(state, baseline));
  assert.deepEqual(undone.frameIds, baseline.frameIds);
  assert.deepEqual(undone.pageIds, baseline.pageIds);
  await page.keyboard.press(process.platform === "darwin" ? "Meta+Shift+z" : "Control+Shift+z");
  const redone = await poll("exact redo snapshot", state => state.artifactPanels === 1 && sameDocument(state, created));
  assert.deepEqual(redone.frameIds, created.frameIds);
  assert.deepEqual(redone.pageIds, created.pageIds);
  await record("exact-create-undo-redo", { submitted: submission.submitted, createdId, exactCreate: true, exactUndo: true, exactRedo: true, undone: { frameIds: undone.frameIds, pageIds: undone.pageIds }, baseline: { frameIds: baseline.frameIds, pageIds: baseline.pageIds }, created: { frameIds: created.frameIds, pageIds: created.pageIds } }, fromCreate, redone);

  const fromMarker = lines.length;
  await storyMarker(contract.marker, contract.demo.stockText);
  await openArtifact();
  await poll("edited owner remains focused", state => state.ownerWindowId === owner && sameDocument(state, redone));
  await record("story-marker", { committed: true, marker: contract.marker }, fromMarker);
  await selectContentPage();
  const editedSvg = await download(owner, "exportSvg", "edited.svg");
  const svg = await validateMedia("svg", editedSvg.bytes, redone.pageIds.length, contract.geometry);
  await openArtifact();
  await record("export-svg", svg);
  const editedPng = await download(owner, "exportPng", "edited.png");
  const png = await validateMedia("png", editedPng.bytes, redone.pageIds.length, contract.geometry);
  await openArtifact();
  await record("export-png", png);
  const geometry = await compareEditedGeometry(beforeSvg.bytes, beforePng.bytes, editedSvg.bytes, editedPng.bytes, contract.geometry);
  await record("edited-geometry", geometry);
  const editedPdf = await download(owner, "exportPdf", "edited.pdf");
  const pdf = await validateMedia("pdf", editedPdf.bytes, redone.pageIds.length, { ...contract.geometry, text: contract.marker });
  assert.ok(!pdf.text.includes(contract.demo.stockText), "PDF uses the edited story rather than the stock demo text");
  await openArtifact();
  await record("export-pdf", { ...pdf, authoredMarker: true });

  const saved = await snapshot();
  assert.ok(saved.frameIds.includes(createdId));
  const archive = await download(owner, "exportArtifactDocument", "edited.semio-archive");
  await openArtifact();
  assert.ok(sameDocument(await snapshot(), saved), "archive creation preserves document");
  await record("export-archive", { exported: true, bytes: archive.bytes.length, owner, frameIds: saved.frameIds, pageIds: saved.pageIds, marker: contract.marker });

  await closeArtifact();
  await submitAction(owner, "addPage");
  await openArtifact();
  const changed = await poll("original-only page", state => state.pageIds.length === saved.pageIds.length + 1);
  const pageDelta = added(saved.pageIds, changed.pageIds);
  assert.equal(pageDelta.introduced.length, 1);
  assert.deepEqual(pageDelta.removed, []);
  const originalOnlyPage = pageDelta.introduced[0];
  await storyMarker(contract.originalOnlyMarker, contract.marker);
  await openArtifact();
  await record("diverge-original", { addedPage: true, originalOnlyPage, originalOnlyMarker: contract.originalOnlyMarker });

  const ownersBefore = new Set((await snapshot()).windowIds);
  await closeArtifact();
  await openActions(owner);
  const fromImport = lines.length;
  const waiting = page.waitForEvent("filechooser", { timeout: 30000 });
  await submitAction(owner, "importArtifactDocument");
  const chooser = await waiting;
  assert.equal(chooser.isMultiple(), false, "archive picker accepts one archive");
  await chooser.setFiles(archive.path);
  const imported = await poll("new Blueprint owner after archive import", state => state.windowIds.some(id => !ownersBefore.has(id) && /blueprint/i.test(id)), 120000);
  const newOwners = imported.windowIds.filter(id => !ownersBefore.has(id) && /blueprint/i.test(id));
  assert.equal(newOwners.length, 1, "one imported Blueprint owner");
  owner = newOwners[0];
  assert.notEqual(owner, saved.ownerWindowId, "archive creates a separate document owner");
  await focusOwner(owner);
  await openArtifact();
  const restored = await poll("saved archive document in new owner", state => state.ownerWindowId === owner && state.artifactPanels === 1 && sameDocument(state, saved), 120000);
  assert.deepEqual(restored.frameIds, saved.frameIds, "restored exact frame IDs");
  assert.deepEqual(restored.pageIds, saved.pageIds, "restored exact page IDs");
  assert.ok(!restored.pageIds.includes(originalOnlyPage), "import excludes original-only page");
  await openInspector();
  assert.equal(await inspectorText().inputValue(), contract.marker, "new owner's saved story marker");
  assert.notEqual(await inspectorText().inputValue(), contract.originalOnlyMarker, "new owner excludes original-only marker");
  await openArtifact();
  await record("reopen-archive", { restored: true, newOwner: true, owner, marker: contract.marker, excludesOriginalOnlyPage: true, excludesOriginalOnlyMarker: true, frameIds: restored.frameIds, pageIds: restored.pageIds }, fromImport);

  for (const expected of contract.steps) {
    const actual = report.steps.find(step => step.step === expected.step);
    assert.ok(actual, "required journey step " + expected.step);
    for (const [key, value] of Object.entries(expected.values)) assert.equal(actual.detail[key], value, expected.step + "." + key);
    assert.deepEqual(actual.faults, [], expected.step + " faults");
  }
  report.complete = true;
  write();
  console.log("[DEBUG] Layout browser journey passed " + report.steps.length + " steps");
} catch (error) {
  report.complete = false;
  report.failure = { ...report.failure, message: String(error), stack: error?.stack };
  write();
  if (page && !page.isClosed()) await page.screenshot({ path: join(outDir, "failure.png") }).catch(() => {});
  console.error("[DEBUG] Browser journey failed: " + String(error));
  process.exitCode = cancellation ? cancellation.reason === "deadline" ? 124 : cancellation.reason === "SIGINT" ? 130 : 143 : 1;
} finally {
  clearTimeout(deadline);
  process.removeListener("SIGINT", onInterrupt);
  process.removeListener("SIGTERM", onTerminate);
  let closed = browser === undefined;
  if (browser) {
    let closeDeadline;
    try { await Promise.race([browser.close().then(() => { closed = true; }).catch(() => {}), new Promise(resolve => { closeDeadline = setTimeout(resolve, 2000); })]); }
    finally { clearTimeout(closeDeadline); }
  }
  if (closed) clearTimeout(hardExit);
  else {
    report.failure = { ...report.failure, closeTimeout: true };
    report.complete = false;
    write();
    process.exitCode = 1;
    if (!hardExit) hardExit = setTimeout(() => process.exit(1), 1000);
  }
}
