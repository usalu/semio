/** 🎛️ Current layout UI edits, exact history, independent media and new-owner archive round-trip. */
import assert from "node:assert/strict";
import { repoToolCacheEnv } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { validateMedia, compareEditedGeometry, validateCanvasCapture } from "./media-oracles.mjs";
import { assertSurfaceHealthy } from "./browser-health.mjs";
import { assertRenderedTextContrast } from "./browser-text-contrast.mjs";
import { planBrowserOutputPaths } from "./browser-output-paths.mjs";
import { waitForProjection } from "./browser-projection.mjs";

const ticket = dirname(fileURLToPath(import.meta.url));
const contract = JSON.parse(readFileSync(join(ticket, "browser-journey.json"), "utf8"));
const outputChildren = ["report.json", "console.txt", "failure.png", "boot-canvas.png", "baseline.svg", "baseline.png", "edited.svg", "edited.png", "edited.pdf", "edited.semio-archive", "downloads", join("downloads", "a".repeat(36)), ...contract.steps.map((step, index) => String(index + 1).padStart(2, "0") + "-" + step.step + ".png")];
const { outDir } = planBrowserOutputPaths(ticket, process.env.SEMIO_PROBE_OUT ?? "layout-current", outputChildren);
mkdirSync(outDir, { recursive: true });
const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6179/?plugin=layout&example=%F0%9F%8E%AC%EF%B8%8Fdemo";
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
  const context = await browser.newContext({ viewport: { width: 1900, height: 1200 }, acceptDownloads: true, locale: "en-US" });
  page = await context.newPage();
  page.on("console", message => lines.push({ t: Date.now() - start, type: message.type(), text: message.text() }));
  page.on("pageerror", error => lines.push({ t: Date.now() - start, type: "pageerror", text: String(error) }));
  page.setDefaultTimeout(15000);

  const nodes = () => page.locator('[data-semio-wgpu-accessibility] [data-node-key]');
  const key = value => nodes().and(page.locator(attr("data-node-key", value)));
  const artifact = () => nodes().and(page.locator('[data-window="framework.panel.artifact"]')).filter({ hasNot: page.locator('[aria-hidden="true"]') });
  const tabs = () => nodes().and(page.locator('[role="tab"][data-node-key^="dock.tab."]'));
  const snapshot = () => page.evaluate(() => {
    const nodes = [...document.querySelectorAll('[data-semio-wgpu-accessibility] [data-node-key]')].filter(node => node.getAttribute("aria-hidden") !== "true");
    const rows = nodes.filter(node => node.dataset.window === "framework.panel.artifact" && node.getAttribute("role") === "treeitem");
    const tabs = nodes.filter(node => node.getAttribute("role") === "tab" && /^dock\.tab\.[^.]*\.[^.]+$/.test(node.dataset.nodeKey));
    const active = tabs.filter(node => node.getAttribute("aria-selected") === "true" && /blueprint/i.test(node.dataset.nodeKey));
    const canvas = document.getElementById("semio-wgpu-canvas");
    const statuses = nodes.filter(node => node.getAttribute("role") === "status" || node.getAttribute("role") === "alert");
    return {
      ready: document.documentElement.getAttribute("data-semio-os-ready"),
      error: document.documentElement.getAttribute("data-semio-os-error"),
      artifactPanels: rows.length ? 1 : 0,
      frameIds: rows.map(row => row.dataset.nodeKey).filter(id => /^frame-/.test(id)).sort(),
      pageIds: rows.map(row => row.dataset.nodeKey).filter(id => id.startsWith("layout-document.page.")).sort(),
      selected: rows.filter(row => row.getAttribute("aria-selected") === "true").map(row => row.dataset.nodeKey).sort(),
      ownerWindowId: active.length === 1 ? active[0].dataset.nodeKey.split(".").slice(3).join(".") : null,
      windowIds: [...new Set(tabs.map(node => node.dataset.nodeKey.split(".").slice(3).join(".")))].sort(),
      hosts: canvas ? [{ id: canvas.id, phase: null, fault: null, statusJson: null, boardStatusJson: null, canvases: [{ width: canvas.width, height: canvas.height }], uiTurn: canvas.dataset.uiTurn }] : [],
      accessibility: { nodes: nodes.length, unnamed: nodes.filter(node => node.dataset.actionable === "true" && !node.getAttribute("aria-label") && !node.textContent?.trim()).map(node => node.dataset.nodeKey) },
      surfaceFaults: statuses.filter(node => node.getAttribute("role") === "alert" || /fault|failed|refused|unreachable/i.test(node.getAttribute("aria-label") ?? "")).map(node => ({ id: node.dataset.nodeKey, text: node.getAttribute("aria-label") })),
    };
  });

  const healthy = state => {
    assert.equal(state.ready, "layout", "selected layout component is ready");
    assert.equal(state.error, null, "shell boot error");
    assert.deepEqual(state.surfaceFaults, [], "visible surface fault");
    assert.ok(state.accessibility.nodes > 0, "current renderer publishes accessible controls");
    assert.deepEqual(state.accessibility.unnamed, [], "actionable controls carry accessible names");
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
    await locator.first().waitFor({ state: "attached" });
    assert.equal(await locator.count(), 1, label + " has one current accessibility control");
    assert.notEqual(await locator.getAttribute("aria-hidden"), "true", label + " is exposed");
    const until = Date.now() + 10000;
    while ((!await locator.isEnabled() || await locator.getAttribute("aria-disabled") === "true") && Date.now() < until) await page.waitForTimeout(100);
    assert.ok(await locator.isEnabled() && await locator.getAttribute("aria-disabled") !== "true", label + " is enabled");
    return locator;
  };
  const activate = async (locator, label) => {
    const control = await one(locator, label);
    await control.focus();
    assert.ok(await control.evaluate(element => document.activeElement === element), label + " receives keyboard focus");
    await control.press("Enter");
  };
  const togglePanel = async id => activate(key(id), id);
  const openArtifact = async () => {
    if (!(await snapshot()).artifactPanels) await togglePanel("framework.panel.artifact");
    await poll("Artifact accessibility document", state => state.artifactPanels === 1);
  };
  const closeArtifact = async () => {
    if ((await snapshot()).artifactPanels) {
      await togglePanel("framework.panel.artifact");
      await poll("Artifact document retired", state => state.artifactPanels === 0);
    }
  };
  const focusOwner = async owner => {
    await closeArtifact();
    await activate(tabs().and(page.locator('[data-node-key$=".' + owner + '"]')), "owner dock tab");
    await poll("active owner", state => state.ownerWindowId === owner);
  };
  const pane = owner => nodes().and(page.locator(attr("data-window", owner + contract.renderer.actionsSurfaceSuffix)));
  const openActions = async owner => {
    if (await pane(owner).count() === 0) await activate(key("framework.window." + owner + ".engagement.toggle"), "Actions toggle");
    await one(pane(owner).and(page.locator(attr("data-node-key", "window-action-pane"))), "owner Actions pane");
  };
  const stageAction = async (owner, action, args) => {
    await openActions(owner);
    const scope = pane(owner);
    let execute = scope.and(page.locator('[data-node-key$=".action.' + action + '.execute"]'));
    if (await execute.count() === 0) {
      const row = await one(scope.and(page.locator(attr("data-node-key", "action." + action))), action + " row");
      await activate(row, action + " row");
      execute = scope.and(page.locator('[data-node-key$=".action.' + action + '.execute"]'));
      if (Object.keys(args).length === 0 && await execute.count() === 0) return { submitted: "row", action };
    }
    await one(execute, action + " Execute");
    for (const [name, value] of Object.entries(args)) {
      const row = await one(scope.and(page.locator(attr("data-node-key", "action." + action + ".arg." + name))), action + "." + name);
      if (name === "kind") {
        const trigger = () => pane(owner).and(page.locator(attr("data-node-key", "action." + action + ".arg." + name))).locator('[role="combobox"]');
        const option = () => pane(owner).and(page.locator('[role="option"][data-node-key$="::option::' + value + '"]'));
        const projection = () => page.evaluate(({ windowId, rowKey, optionSuffix }) => {
          const controls = [...document.querySelectorAll('[data-semio-wgpu-accessibility] [data-node-key]')].filter(node => node.dataset.window === windowId);
          const row = controls.find(node => node.dataset.nodeKey === rowKey);
          const trigger = row?.querySelector('[role="combobox"]');
          const option = controls.find(node => node.getAttribute("role") === "option" && node.dataset.nodeKey.endsWith(optionSuffix));
          return { expanded: trigger?.getAttribute("aria-expanded") ?? null, selected: option?.getAttribute("aria-selected") ?? null };
        }, { windowId: owner + contract.renderer.actionsSurfaceSuffix, rowKey: "action." + action + ".arg." + name, optionSuffix: "::option::" + value });
        const wait = (label, accept) => waitForProjection(label, projection, accept, { pause: milliseconds => page.waitForTimeout(milliseconds) });
        await activate(trigger(), action + ".kind trigger");
        await wait("kind options open", state => state.expanded === "true");
        await activate(option(), "kind option " + value);
        await wait("kind option closes", state => state.expanded === "false");
        await activate(trigger(), "kind verification");
        await wait("kind selection retained on reopen", state => state.expanded === "true" && state.selected === "true");
        await (await one(trigger(), "kind verification trigger")).press("Escape");
        await wait("kind verification closes before history baseline", state => state.expanded === "false");
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
    await activate(artifact().and(page.locator('[role="treeitem"]' + attr("data-node-key", "layout-document.page.page-1"))), "content page row");
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
  const inspectorText = () => nodes().and(page.locator('[data-window="framework.panel.inspection"] textarea[data-node-key*="global.story.story-1.content"]'));
  const openInspector = async () => {
    if (await inspectorText().count() === 0) await togglePanel("framework.panel.inspection");
    await one(inspectorText(), "story-1 authored textarea");
  };
  const storyMarker = async (marker, previous) => {
    await openInspector();
    if (previous !== undefined) assert.equal(await inspectorText().inputValue(), previous, "story before edit");
    const previousProjection = await inspectorText().elementHandle();
    assert.ok(previousProjection, "story has a current renderer projection");
    try {
      await inspectorText().fill(marker);
      await inspectorText().press("Tab");
      await waitForProjection("fresh committed story projection", async () => ({
        replaced: await previousProjection.evaluate(element => !element.isConnected),
        value: await inspectorText().count() === 1 ? await inspectorText().inputValue() : null,
        blurred: await inspectorText().count() === 1 && await inspectorText().evaluate(element => document.activeElement !== element),
      }), state => state.replaced && state.value === marker && state.blurred, { pause: milliseconds => page.waitForTimeout(milliseconds) });
    } finally { await previousProjection.dispose(); }
  };

  await page.goto(url, { waitUntil: "domcontentloaded" });
  const boot = await poll("layout boot", state => state.ready === "layout" && state.ownerWindowId !== null && state.hosts.some(host => host.canvases.some(canvas => canvas.width > 100 && canvas.height > 100)), 180000);
  let owner = boot.ownerWindowId;
  assert.ok(/blueprint/i.test(owner), "boot focuses a Blueprint owner");
  const canvas = page.locator('#semio-wgpu-canvas');
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
  const contrastInput = await one(pane(owner).and(page.locator('[data-node-key="action.addFrame.arg.x"]')).locator("input"), "authored X input for rendered contrast");
  const contrastKey = await contrastInput.getAttribute("data-node-key");
  const contrastWindow = await contrastInput.getAttribute("data-window");
  const contrastLabel = await contrastInput.inputValue();
  assert.equal(contrastLabel, String(contract.frame.x), "contrast witness is the actual authored X value");
  const contrastProjection = await page.evaluate(async windowId => {
    const reader = window.semioWgpuIntrospection;
    if (!reader) throw Error("Current renderer introspection reader is required for contrast measurement");
    return { structure: JSON.parse(await reader.dumpStructure(windowId)), frame: JSON.parse(await reader.dumpFrameStats(windowId)) };
  }, contrastWindow);
  const textContrast = assertRenderedTextContrast(contrastProjection.structure, contrastProjection.frame, { key: contrastKey, window: contrastWindow, label: contrastLabel, kind: "input" }, contract.textContrast.minimumRatio);
  await record("rendered-text-contrast", textContrast);

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
