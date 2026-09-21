/** @emoji 🖱️ Physical Chromium probe for the canonical Draw Canvas2d and Note InkCanvas surfaces. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv2020 from "ajv/dist/2020.js";
import { chromium, type BrowserContext, type Page } from "playwright";

type Renderer = "react" | "wgpu";
type TargetName = "draw" | "layout" | "note";
type Rect = { x: number; y: number; width: number; height: number };
type ProbeTarget = {
  plugin: string;
  surfaceKind: string;
  controllerId: string;
  windowId: string;
  surfaceId: string;
  reactSelector: string;
  initialExampleId?: string;
  initialTextBlockId?: string;
  urls: Record<Renderer, string>;
};
type ProbeCase = {
  id: string;
  clipboardKind?: "text" | "image" | null;
  payload?: string | null;
  mime?: string;
  base64?: string;
  width?: number;
  height?: number;
  expectedAction?: string | null;
  expectedOperations?: string[];
  input?: string;
  expectedActions?: string[];
  consequence: string;
};
type LayoutCatalogueCase = {
  id: string;
  kind: "page" | "rect" | "text" | "image";
  sourceId: string;
  dragPayload: string;
  previewPixels: "changed" | "unchanged";
  createdCollection: "pages" | "frames";
  createdDelta: 1;
  consequence: string;
};
type DrawSelectionCase = {
  id: "single-selection-rename" | "multi-selection-rename" | "subtracted-selection-rename";
  mergeMode: "replace" | "add" | "subtract";
  namePrefix: string;
  expectedConsequence: "selected-row-renamed" | "no-row-renamed" | "original-row-renamed";
};
type Fixture = {
  version: 3;
  targets: Record<TargetName, ProbeTarget>;
  layoutCatalogue: {
    transport: "pointer-catalogue";
    mime: string;
    kindMimePrefix: string;
    catalogueTabId: "framework.panel.catalogue";
    artifactTabId: "framework.panel.artifact";
    transferHandlePrefix: "tree.drag.transfer.";
    pageSectionId: "layout-document.pages";
    frameSectionId: "layout-document.frames";
    seed: { exampleId: string; reset: "browser-reload"; pages: number; frames: number };
    previewAction: "canvasDragOver";
    leaveAction: "canvasDragLeave";
    dropAction: "canvasDrop";
    cases: LayoutCatalogueCase[];
  };
  drawSelection: {
    version: 1;
    engagementFieldId: "drawing-canvas-engagement";
    searchToggleId: "framework.window.drawingComposite.search.toggle";
    layerRowIdPrefix: "drawing-play-layers.shape.";
    cases: DrawSelectionCase[];
  };
  drawCases: ProbeCase[];
  inkCases: ProbeCase[];
  cancellation: {
    producer: string;
    address: string[];
    cancelDeltaActions: string[];
    nextGestureAction: string;
    nextGesturePhase: string;
  };
  limits: { clipboardItems: number; clipboardTextBytes: number; actionStringBytes: number; settleMilliseconds: number };
};
type ActionRow = { seq: number; controller: string; action: string; args?: string; windowId?: string | null; outcome?: unknown };

const command = process.argv[2] ?? "discover";
const targetName = (process.argv[3] ?? process.env.SEMIO_CANVAS_TARGET ?? "draw") as TargetName;
const renderer = (process.env.SEMIO_CANVAS_RENDERER ?? "react") as Renderer;
const fixturePath = join(import.meta.dir, "🧫️fixtures", "🔣️.json");
const layoutProbeSchemaPath = join(import.meta.dir, "🧬️schema", "📐️layout-catalogue", "🔣️.json");
const drawSelectionSchemaPath = join(import.meta.dir, "🧬️schema", "🎯️draw-selection", "🔣️.json");
const repoRoot = resolve(import.meta.dir, "../../../../../../../../");
const layoutNeutralFixturePath = join(repoRoot, "✏️s", "🔌️plugins", "📏️layout", "🧫️fixtures", "🛍️canvas-catalogue", "🔣️.json");
const layoutNeutralSchemaPath = join(repoRoot, "✏️s", "🔌️plugins", "📏️layout", "🧬️schema", "🛍️canvas-catalogue", "🔣️.json");
const outputRoot = join(import.meta.dir, "..", "🗑️generated", process.env.SEMIO_CANVAS_OUT ?? "canvas-interactions");
const viewport = { width: 1600, height: 1000 };
const browserArgs = ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", ...(process.platform === "darwin" ? ["--use-angle=metal"] : [])];

function fixture(): Fixture {
  const parsed = JSON.parse(readFileSync(fixturePath, "utf8")) as Fixture;
  assert.equal(parsed.version, 3);
  assert.deepEqual(Object.keys(parsed.targets), ["draw", "layout", "note"]);
  assert.equal(parsed.targets.draw.surfaceKind, "canvas-2d");
  assert.equal(parsed.targets.draw.surfaceId, "drawing.play.composite");
  assert.equal(parsed.targets.layout.controllerId, "s.layout.layout@1/*#editor");
  assert.equal(parsed.targets.layout.windowId, "layout-blueprint");
  assert.equal(parsed.targets.layout.surfaceId, "layout.play.blueprint");
  assert.equal(parsed.targets.note.surfaceKind, "ink-canvas");
  assert.equal(parsed.targets.note.surfaceId, "note.play.composite");
  assert.equal(parsed.targets.note.controllerId, "s.note.note@1/*#editor");
  assert.deepEqual(parsed.drawCases.map(({ id }) => id), ["primary-draw", "primary-select", "selection-modifiers", "repeated-geometry-creation", "escape-cancel", "double-click-commit"]);
  assert.deepEqual(parsed.inkCases.map(({ id }) => id), ["selected-block-copy", "structured-text-paste", "plain-text-paste", "tiny-rgba-paste", "editor-priority", "exact-pointer-cancel"]);
  assert.equal(new Set([...parsed.drawCases, ...parsed.inkCases].map(({ id }) => id)).size, parsed.drawCases.length + parsed.inkCases.length);
  assert.deepEqual(parsed.cancellation.address, ["windowId", "surfaceGeneration", "pointerId"]);
  assert.equal(parsed.cancellation.producer, "chromium-cdp-touchCancel");
  assert.deepEqual(parsed.layoutCatalogue.cases.map(({ kind }) => kind), ["page", "rect", "text", "image"]);
  assert.deepEqual(parsed.layoutCatalogue.seed, { exampleId: "demo", reset: "browser-reload", pages: 2, frames: 3 });
  assert.equal(parsed.layoutCatalogue.catalogueTabId, "framework.panel.catalogue");
  assert.equal(parsed.layoutCatalogue.artifactTabId, "framework.panel.artifact");
  assert.equal(parsed.layoutCatalogue.transferHandlePrefix, "tree.drag.transfer.");
  for (const row of parsed.layoutCatalogue.cases) {
    assert.equal(row.sourceId, `layout-catalogue.${row.kind}`);
    assert.deepEqual(JSON.parse(row.dragPayload), { kind: row.kind });
    assert.equal(row.createdDelta, 1);
    assert.ok(row.consequence.length > 0);
  }
  const ajv = new Ajv2020({ strict: true });
  const layoutProbe = { target: parsed.targets.layout, ...parsed.layoutCatalogue };
  const validateProbe = ajv.compile(JSON.parse(readFileSync(layoutProbeSchemaPath, "utf8")));
  assert.ok(validateProbe(layoutProbe), `Layout physical probe schema: ${ajv.errorsText(validateProbe.errors)}`);
  const validateDrawSelection = ajv.compile(JSON.parse(readFileSync(drawSelectionSchemaPath, "utf8")));
  assert.ok(validateDrawSelection(parsed.drawSelection), `Draw selection probe schema: ${ajv.errorsText(validateDrawSelection.errors)}`);
  assert.deepEqual(parsed.drawSelection.cases.map(({ mergeMode }) => mergeMode), ["replace", "add", "subtract"]);
  const neutralFixture: any = JSON.parse(readFileSync(layoutNeutralFixturePath, "utf8"));
  const validateNeutral = ajv.compile(JSON.parse(readFileSync(layoutNeutralSchemaPath, "utf8")));
  assert.ok(validateNeutral(neutralFixture), `Layout neutral catalogue schema: ${ajv.errorsText(validateNeutral.errors)}`);
  const neutralCases = (neutralFixture as { cases: any[] }).cases;
  for (const row of parsed.layoutCatalogue.cases) {
    assert.ok(neutralCases.some((entry: any) => entry.action === parsed.layoutCatalogue.previewAction && entry.kind === row.kind));
    assert.ok(neutralCases.some((entry: any) => entry.action === parsed.layoutCatalogue.dropAction && entry.kind === row.kind && entry.args.dragData === row.dragPayload));
  }
  assert.equal(parsed.limits.clipboardItems, 16);
  for (const entry of [...parsed.drawCases, ...parsed.inkCases]) assert.ok(entry.consequence.length > 0);
  for (const entry of parsed.drawCases) assert.ok(entry.input && entry.expectedActions?.length);
  const image = parsed.inkCases.find(({ id }) => id === "tiny-rgba-paste");
  assert.equal(image?.mime, "image/png");
  assert.equal(image?.width, 1);
  assert.equal(image?.height, 1);
  assert.ok(image?.base64 && Buffer.from(image.base64, "base64").length > 0);
  return parsed;
}

const contract = fixture();
assert.ok(targetName === "draw" || targetName === "layout" || targetName === "note", `Unknown target: ${targetName}`);
assert.ok(renderer === "react" || renderer === "wgpu", `Unknown renderer: ${renderer}`);
const target = contract.targets[targetName];
const explicitUrl = process.env.SEMIO_CANVAS_URL ?? (renderer === "react" ? process.env.SEMIO_CANVAS_REACT_URL : process.env.SEMIO_CANVAS_WGPU_URL);
const url = explicitUrl ?? target.urls[renderer];
const outputDirectory = join(outputRoot, targetName, renderer);
mkdirSync(outputDirectory, { recursive: true });

async function waitForBoot(page: Page): Promise<void> {
  await page.waitForFunction(
    () => document.documentElement.getAttribute("data-semio-os-ready") !== null || document.documentElement.getAttribute("data-semio-os-error") !== null,
    undefined,
    { timeout: 120_000 },
  );
  const error = await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-error"));
  if (error) throw new Error(`${renderer} shell reported ${error}`);
  await page.waitForTimeout(2_000);
}

async function waitUntil<T>(read: () => Promise<T>, accept: (value: T) => boolean, label: string): Promise<T> {
  const started = Date.now();
  let value = await read();
  while (!accept(value) && Date.now() - started < contract.limits.settleMilliseconds) {
    await new Promise((resolve) => setTimeout(resolve, 100));
    value = await read();
  }
  assert.ok(accept(value), `${label} did not settle: ${JSON.stringify(value)}`);
  return value;
}

async function dismissIntroduction(page: Page): Promise<boolean> {
  const selectors = ["#ui\\.introduction\\.skip", '[data-node-key="ui.introduction.skip"]'];
  for (const selector of selectors) {
    const row = page.locator(selector).first();
    if (await row.count()) {
      const box = await row.boundingBox();
      if (box) {
        await page.mouse.click(box.x + box.width / 2, box.y + box.height / 2);
        await page.waitForTimeout(500);
        return true;
      }
    }
  }
  if (renderer === "wgpu") {
    const hit = await wgpuHit(page, "ui.introduction.skip");
    if (hit) {
      await page.mouse.click(hit.x + hit.width / 2, hit.y + hit.height / 2);
      await page.waitForTimeout(500);
      return true;
    }
  }
  return false;
}

async function wgpuDump(page: Page, probe: "dumpChrome" | "dumpStructure", windowId?: string): Promise<any> {
  return page.evaluate(
    async ({ probe, windowId }) => {
      const beacon = (globalThis as any).semioWgpuIntrospection;
      if (typeof beacon?.[probe] !== "function") return { unavailable: probe };
      const raw = await beacon[probe](windowId);
      return raw ? JSON.parse(raw) : null;
    },
    { probe, windowId },
  );
}

async function wgpuHit(page: Page, controlId: string): Promise<Rect | null> {
  const dump = await wgpuDump(page, "dumpChrome");
  const hit = dump?.hits?.find((entry: any) => entry.controlId === controlId);
  return hit ? { x: hit.rect[0], y: hit.rect[1], width: hit.rect[2], height: hit.rect[3] } : null;
}

async function wgpuSceneRect(page: Page, wanted: ProbeTarget): Promise<Rect | null> {
  const first = await wgpuDump(page, "dumpStructure", wanted.windowId);
  const ids = first?.windowIds?.length ? first.windowIds : [wanted.windowId];
  for (const id of ids) {
    const dump = id === wanted.windowId ? first : await wgpuDump(page, "dumpStructure", id);
    const scenes = (dump?.nodes ?? []).filter((node: any) => node.kind === "componentScene" && node.visible && node.rect?.[2] > 0 && node.rect?.[3] > 0);
    const node = scenes.find((entry: any) => String(entry.path).includes(wanted.surfaceId)) ?? (id === wanted.windowId && scenes.length === 1 ? scenes[0] : null);
    if (node) return { x: node.rect[0], y: node.rect[1], width: node.rect[2], height: node.rect[3] };
  }
  return null;
}

async function surfaceRect(page: Page, wanted = target): Promise<Rect | null> {
  if (renderer === "wgpu") return wgpuSceneRect(page, wanted);
  const surface = page.locator(wanted.reactSelector).first();
  if (!(await surface.count())) return null;
  const box = await surface.boundingBox();
  return box ? { x: box.x, y: box.y, width: box.width, height: box.height } : null;
}

async function actionRows(page: Page): Promise<ActionRow[]> {
  if (renderer === "react") {
    return page.evaluate(() =>
      ((globalThis as any).__semioInputLedger?.recent ?? []).map((row: any) => ({
        seq: row.inputSeq,
        controller: row.controllerId ?? "?",
        action: row.action,
        windowId: row.windowId,
        outcome: row.outcome,
      })),
    );
  }
  const dump = await wgpuDump(page, "dumpChrome");
  return (dump?.actions ?? []).map((row: any) => ({ seq: row.seq, controller: row.controllerId, action: row.action, args: row.args }));
}

async function actionCursor(page: Page): Promise<number> {
  return (await actionRows(page)).at(-1)?.seq ?? 0;
}

async function actionsAfter(page: Page, cursor: number): Promise<ActionRow[]> {
  return (await actionRows(page)).filter(({ seq }) => seq > cursor);
}

async function waitForAction(page: Page, cursor: number, action: string): Promise<ActionRow[]> {
  return waitUntil(() => actionsAfter(page, cursor), (rows) => rows.some((row) => row.action === action), `action ${action}`);
}

async function clickControl(page: Page, ids: readonly string[]): Promise<string> {
  if (renderer === "wgpu") {
    for (const id of ids) {
      const rect = await wgpuHit(page, id);
      if (rect) {
        await page.mouse.click(rect.x + rect.width / 2, rect.y + rect.height / 2);
        return id;
      }
    }
  } else {
    const found = await page.evaluate((ids) => {
      for (const id of ids) {
        const exact = document.getElementById(id);
        const suffix = [...document.querySelectorAll<HTMLElement>("[id]")].find((element) => element.id.endsWith(`.${id}`) || element.id.endsWith(`-${id}`));
        const element = exact instanceof HTMLElement ? exact : suffix;
        if (!element) continue;
        const rect = element.getBoundingClientRect();
        if (rect.width > 0 && rect.height > 0) return { id: element.id, x: rect.x, y: rect.y, width: rect.width, height: rect.height };
      }
      return null;
    }, ids);
    if (found) {
      await page.mouse.click(found.x + found.width / 2, found.y + found.height / 2);
      return found.id;
    }
  }
  throw new Error(`No physical control matches ${ids.join(", ")}`);
}

function noteElements(page: Page, selector: string) {
  return page.locator(target.reactSelector).locator(selector);
}

async function ensureNoteDemo(page: Page): Promise<void> {
  if (renderer === "react" && (await noteElements(page, `[data-ink-block-id="${target.initialTextBlockId}"]`).count())) return;
  if (renderer === "wgpu" && (await actionRows(page)).some(({ action, args }) => action === "setActiveExample" && args?.includes(target.initialExampleId!))) return;
  const cursor = await actionCursor(page);
  await clickControl(page, ["playground.navbar.fixture"]);
  await page.waitForTimeout(250);
  if (renderer === "wgpu") await clickControl(page, [`shell.example.${target.initialExampleId}`]);
  else {
    const option = page.getByRole("option", { name: "Demo", exact: true }).first();
    await option.click();
  }
  if (renderer === "react") await waitUntil(() => noteElements(page, `[data-ink-block-id="${target.initialTextBlockId}"]`).count(), (count) => count === 1, "Note demo text block");
  else {
    const rows = await waitForAction(page, cursor, "setActiveExample");
    assert.ok(rows.some(({ action, args }) => action === "setActiveExample" && args?.includes(target.initialExampleId!)), "WGPU physically selected the Note demo");
    await waitUntil(() => wgpuSceneRect(page, target), Boolean, "Note InkCanvas scene");
  }
}

async function observe(page: Page): Promise<Record<string, unknown>> {
  const rect = await surfaceRect(page);
  const actions = await actionRows(page);
  if (renderer === "wgpu") {
    const structure = await wgpuDump(page, "dumpStructure", target.windowId);
    return { href: page.url(), target: targetName, renderer, rect, actions, structure };
  }
  return page.evaluate(
    ({ selector, targetName, renderer, rect, actions }) => ({
      href: location.href,
      target: targetName,
      renderer,
      rect,
      actions,
      surfaces: [...document.querySelectorAll("[data-surface-id]")].map((element) => ({
        surfaceId: element.getAttribute("data-surface-id"),
        controllerId: element.getAttribute("data-controller-id"),
        tag: element.tagName,
      })),
      blocks: [...document.querySelectorAll("[data-ink-block-id]")].map((element) => ({
        id: element.getAttribute("data-ink-block-id"),
        text: element.textContent?.replace(/\s+/g, " ").trim(),
        image: element.querySelector("img")?.getAttribute("src") ?? null,
      })),
      canvases: [...document.querySelectorAll("canvas")].map(element => {
        const bounds = element.getBoundingClientRect();
        const owners = [];
        for (let owner: HTMLElement | null = element; owner && owners.length < 6; owner = owner.parentElement) owners.push({ tag: owner.tagName, id: owner.id, className: owner.className, data: { ...owner.dataset } });
        return { rect: { x: bounds.x, y: bounds.y, width: bounds.width, height: bounds.height }, owners };
      }),
      text: document.body.innerText,
      controls: [...document.querySelectorAll<HTMLElement>("[id]")].flatMap(element => {
        const rect = element.getBoundingClientRect();
        return rect.width > 0 && rect.height > 0 ? [{ id: element.id, role: element.getAttribute("role"), slot: element.dataset.slot, text: element.textContent?.trim(), rect: [rect.x, rect.y, rect.width, rect.height] }] : [];
      }),
      hostCount: document.querySelectorAll(selector).length,
      physicalDragEvents: (globalThis as any).__semioCanvasPhysicalDragEvents ?? [],
      physicalPointerTransfers: (globalThis as any).__semioCanvasPhysicalPointerTransfers ?? [],
    }),
    { selector: target.reactSelector, targetName, renderer, rect, actions },
  );
}

async function preparePage(context: BrowserContext): Promise<{ page: Page; consoleLines: string[] }> {
  const page = await context.newPage();
  const consoleLines: string[] = [];
  page.on("console", (message) => consoleLines.push(`${message.type()} ${message.text()}`));
  page.on("pageerror", (error) => consoleLines.push(`pageerror ${String(error)}`));
  await page.addInitScript(() => localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"));
  await page.addInitScript(({ key, mime }) => {
    const events: unknown[] = [];
    (globalThis as any).__semioCanvasPhysicalDragEvents = events;
    const pointers: unknown[] = [];
    (globalThis as any).__semioCanvasPhysicalPointerTransfers = pointers;
    let source: { id: string; mime: string | undefined; payload: string | undefined; pointerId: number } | null = null;
    for (const type of ["pointerdown", "pointermove", "pointerup", "pointercancel"]) document.addEventListener(type, event => {
      const pointer = event as PointerEvent;
      const target: Element | null = event.target instanceof Element ? event.target : null;
      if (type === "pointerdown") {
        const owner = target?.closest("[data-drag-mime][data-drag-payload]") as HTMLElement | null;
        source = owner ? { id: owner.id, mime: owner.dataset.dragMime, payload: owner.dataset.dragPayload, pointerId: pointer.pointerId } : null;
      }
      if (!source || source.pointerId !== pointer.pointerId) return;
      const surface = document.elementFromPoint(pointer.clientX, pointer.clientY)?.closest("[data-ui-surface-shell]") as HTMLElement | null;
      const host = surface?.querySelector("[data-surface-id]") as HTMLElement | null;
      if (pointers.length < 1024) pointers.push({ type, trusted: event.isTrusted, source, x: pointer.clientX, y: pointer.clientY, key: surface?.dataset.uiNodeKey, surfaceId: host?.dataset.surfaceId, controllerId: host?.dataset.controllerId });
      if (type === "pointerup" || type === "pointercancel") source = null;
    }, true);
    for (const type of ["dragstart", "dragend", "dragover", "dragleave", "drop"]) document.addEventListener(type, event => {
      const target: Element | null = event.target instanceof Element ? event.target : null;
      if ((type === "dragstart" || type === "dragend") && events.length < 1024) {
        events.push({ type, trusted: event.isTrusted, sourceId: target?.closest("[id]")?.id });
        return;
      }
      const surface = target?.closest("[data-ui-surface-shell]") as HTMLElement | null;
      if (surface?.dataset.uiNodeKey !== key || events.length >= 1024) return;
      const host = surface.querySelector("[data-surface-id]") as HTMLElement | null;
      const drag = event as DragEvent;
      events.push({ type, trusted: event.isTrusted, key, surfaceId: host?.dataset.surfaceId, controllerId: host?.dataset.controllerId, types: [...(drag.dataTransfer?.types ?? [])], payload: type === "drop" ? drag.dataTransfer?.getData(mime) : undefined });
    }, true);
  }, { key: target.surfaceId, mime: contract.layoutCatalogue.mime });
  await page.goto(url, { waitUntil: "domcontentloaded" });
  await waitForBoot(page);
  await dismissIntroduction(page);
  return { page, consoleLines };
}

async function discover(): Promise<void> {
  const browser = await chromium.launch({ headless: process.env.SEMIO_CANVAS_HEADED !== "1", args: browserArgs });
  const context = await browser.newContext({ viewport, deviceScaleFactor: 1 });
  try {
    const { page, consoleLines } = await preparePage(context);
    await waitUntil(() => surfaceRect(page), Boolean, `${targetName} discovery surface`).catch(() => undefined);
    const observation = await observe(page);
    await page.screenshot({ path: join(outputDirectory, "discovery.png") });
    writeFileSync(join(outputDirectory, "discovery.json"), JSON.stringify({ url, target, observation, consoleLines }, null, 2));
    process.stdout.write(JSON.stringify({ target: targetName, renderer, url, available: Boolean(await surfaceRect(page)), actions: (observation.actions as unknown[]).length }, null, 2) + "\n");
  } finally {
    await browser.close();
  }
}

function pasteChord(): string {
  return process.platform === "darwin" ? "Meta+V" : "Control+V";
}

function copyChord(): string {
  return process.platform === "darwin" ? "Meta+C" : "Control+C";
}

async function writeClipboardText(page: Page, text: string): Promise<void> {
  await page.evaluate((value) => navigator.clipboard.writeText(value), text);
}

async function writeClipboardImage(page: Page, row: ProbeCase): Promise<void> {
  assert.ok(row.base64 && row.mime);
  await page.evaluate(
    async ({ base64, mime }) => {
      const bytes = Uint8Array.from(atob(base64), (char) => char.charCodeAt(0));
      const blob = new Blob([bytes], { type: mime });
      await navigator.clipboard.write([new ClipboardItem({ [mime]: blob })]);
    },
    { base64: row.base64, mime: row.mime },
  );
}

function assertWgpuOperations(row: ActionRow, expected: readonly string[]): void {
  if (renderer !== "wgpu") return;
  assert.ok(row.args, "WGPU diagnostic action carries bounded args");
  for (const operation of expected) assert.ok(row.args.includes(operation), `WGPU action args include ${operation}: ${row.args}`);
}

type NoteSurfaceCapture = { file: string; hash: string; rect: Rect };

async function captureNoteSurface(page: Page, name: string): Promise<NoteSurfaceCapture> {
  await page.mouse.move(1, 1);
  await page.waitForTimeout(500);
  const rect = await surfaceRect(page);
  assert.ok(rect, `${name} has a mounted Ink surface`);
  const x = Math.max(0, rect.x);
  const y = Math.max(0, rect.y);
  const width = Math.min(rect.x + rect.width, viewport.width) - x;
  const height = Math.min(rect.y + rect.height, viewport.height) - y;
  assert.ok(width > 0 && height > 0, `${name} Ink surface intersects the viewport`);
  const file = `${name}.png`;
  const bytes = await page.screenshot({ clip: { x, y, width, height }, path: join(outputDirectory, file) });
  return { file, hash: createHash("sha256").update(bytes).digest("hex"), rect: { x, y, width, height } };
}

async function waitForNoteSurfaceChange(page: Page, name: string, before: NoteSurfaceCapture): Promise<NoteSurfaceCapture> {
  return waitUntil(() => captureNoteSurface(page, name), (capture) => capture.hash !== before.hash, `${name} visible Ink publication`);
}

async function pasteCase(page: Page, row: ProbeCase): Promise<Record<string, unknown>> {
  const rect = await surfaceRect(page);
  assert.ok(rect, "Ink surface has a physical rect");
  await page.mouse.click(rect.x + rect.width * 0.8, rect.y + rect.height * 0.8);
  const before = await captureNoteSurface(page, `${row.id}-before`);
  const beforeBlocks = renderer === "react" ? await noteElements(page, "[data-ink-block-id]").count() : null;
  const cursor = await actionCursor(page);
  if (row.clipboardKind === "image") await writeClipboardImage(page, row);
  else await writeClipboardText(page, row.payload ?? "");
  await page.keyboard.press(pasteChord());
  const delta = await waitForAction(page, cursor, row.expectedAction!);
  const action = delta.find((entry) => entry.action === row.expectedAction)!;
  assertWgpuOperations(action, row.expectedOperations ?? []);
  const after = await waitForNoteSurfaceChange(page, `${row.id}-after`, before);
  if (renderer === "react") {
    const count = await waitUntil(() => noteElements(page, "[data-ink-block-id]").count(), (value) => value === beforeBlocks! + 1, `${row.id} block publication`);
    if (row.id === "structured-text-paste") {
      assert.equal(await noteElements(page, '[data-ink-block-id="probe-text"]').count(), 0, "structured paste re-identifies the source id");
      assert.equal(await page.getByText("Clipboard block", { exact: true }).count(), 1);
    }
    if (row.id === "plain-text-paste") {
      assert.equal(await page.getByText(/Hello.*Welt/s).count(), 1);
    }
    if (row.id === "tiny-rgba-paste") {
      const image = noteElements(page, "[data-ink-block-id] img").last();
      await waitUntil(async () => image.evaluate((entry: HTMLImageElement) => ({ complete: entry.complete, width: entry.naturalWidth, height: entry.naturalHeight })), (value) => value.complete && value.width === row.width && value.height === row.height, "natural PNG dimensions");
    }
    return { id: row.id, status: "passed", action: action.action, blocks: count, before, after };
  }
  return { id: row.id, status: "passed", action: action.action, args: action.args, before, after };
}

async function selectedBlockCopy(page: Page): Promise<Record<string, unknown>> {
  const rect = await surfaceRect(page);
  assert.ok(rect);
  await page.mouse.click(rect.x + rect.width * 0.85, rect.y + rect.height * 0.85);
  const before = await captureNoteSurface(page, "selected-block-copy-before");
  const cursor = await actionCursor(page);
  if (renderer === "react") await noteElements(page, `[data-ink-block-id="${target.initialTextBlockId}"]`).click({ position: { x: 40, y: 30 } });
  else await page.mouse.click(rect.x + 88, rect.y + 78);
  await waitForAction(page, cursor, "setSelection");
  const after = await waitForNoteSurfaceChange(page, "selected-block-copy-after", before);
  await writeClipboardText(page, "");
  await page.keyboard.press(copyChord());
  const text = await waitUntil(() => page.evaluate(() => navigator.clipboard.readText()), (value) => value.includes('"schema":"ink.clipboard"'), "selected Ink copy");
  const payload = JSON.parse(text);
  assert.equal(payload.schema, "ink.clipboard");
  assert.equal(payload.blocks.length, 1);
  assert.equal(payload.blocks[0].id, target.initialTextBlockId);
  assert.equal("assets" in payload, false);
  return { id: "selected-block-copy", status: "passed", blockId: payload.blocks[0].id, assetsEmbedded: false, before, after };
}

async function editorPriority(page: Page, row: ProbeCase): Promise<Record<string, unknown>> {
  const rect = await surfaceRect(page);
  assert.ok(rect);
  if (renderer === "react") await noteElements(page, `[data-ink-block-id="${target.initialTextBlockId}"]`).dblclick({ position: { x: 60, y: 40 } });
  else await page.mouse.dblclick(rect.x + 108, rect.y + 88);
  if (renderer === "react") await waitUntil(() => noteElements(page, '[contenteditable="true"]').count(), (count) => count === 1, "React Ink text editor");
  else {
    await waitUntil(
      async () => (await wgpuDump(page, "dumpStructure", target.windowId))?.nodes ?? [],
      (nodes) => nodes.some((node: any) => node.kind === "input" && node.visible),
      "WGPU Ink text editor",
    );
  }
  const before = await captureNoteSurface(page, `${row.id}-before`);
  const cursor = await actionCursor(page);
  await writeClipboardText(page, row.payload ?? "");
  await page.keyboard.press(pasteChord());
  await page.waitForTimeout(700);
  const delta = await actionsAfter(page, cursor);
  assert.equal(delta.some(({ action }) => action === "inkApplyEvents"), false, "active editor owns paste without a block action");
  if (renderer === "react") assert.match(await noteElements(page, '[contenteditable="true"]').innerText(), /editor-owned/);
  await page.keyboard.press("Escape");
  const after = await waitForNoteSurfaceChange(page, `${row.id}-after`, before);
  return { id: row.id, status: "passed", blockActions: delta.filter(({ action }) => action === "inkApplyEvents").length, before, after };
}

type TouchStep = { type: "touchStart" | "touchMove" | "touchEnd" | "touchCancel"; x?: number; y?: number };

async function touchCommand(session: Awaited<ReturnType<BrowserContext["newCDPSession"]>>, { type, x, y }: TouchStep): Promise<void> {
  const touchPoints = type === "touchEnd" || type === "touchCancel" ? [] : [{ x: x!, y: y!, radiusX: 1, radiusY: 1, force: 1, id: 1 }];
  await session.send("Input.dispatchTouchEvent", { type, touchPoints });
}

async function touchSequence(page: Page, steps: readonly TouchStep[]): Promise<void> {
  const session = await page.context().newCDPSession(page);
  try {
    for (const step of steps) await touchCommand(session, step);
  } finally {
    await session.detach();
  }
}

async function exactPointerCancel(page: Page): Promise<Record<string, unknown>> {
  await clickControl(page, ["framework.utility.toggle.pencil", "pencil"]);
  const rect = await surfaceRect(page);
  assert.ok(rect);
  const before = await captureNoteSurface(page, "exact-pointer-cancel-before");
  const x = rect.x + rect.width * 0.72;
  const y = rect.y + rect.height * 0.72;
  const session = await page.context().newCDPSession(page);
  await touchCommand(session, { type: "touchStart", x, y });
  await touchCommand(session, { type: "touchMove", x: x + 24, y: y + 18 });
  await page.waitForTimeout(250);
  const live = await waitForNoteSurfaceChange(page, "exact-pointer-cancel-live", before);
  const cancelCursor = await actionCursor(page);
  await touchCommand(session, { type: "touchCancel" });
  await session.detach();
  await page.waitForTimeout(700);
  const cancelled = await actionsAfter(page, cancelCursor);
  assert.deepEqual(cancelled.filter(({ action }) => action === "inkApplyEvents" || action === "setSelection"), [], "cancel emits no semantic terminal action");
  const retired = await captureNoteSurface(page, "exact-pointer-cancel-retired");
  assert.equal(retired.hash, before.hash, "pointer cancel retires the visible Ink draft without publishing it");
  const nextCursor = await actionCursor(page);
  await touchSequence(page, [
    { type: "touchStart", x: x + 50, y },
    { type: "touchEnd" },
  ]);
  const next = await waitForAction(page, nextCursor, contract.cancellation.nextGestureAction);
  const ink = next.filter(({ action }) => action === contract.cancellation.nextGestureAction);
  assert.ok(ink.length >= 1);
  if (renderer === "wgpu") assert.ok(ink.some(({ args }) => args?.includes(contract.cancellation.nextGesturePhase)), "next WGPU gesture reaches commit");
  const committed = await waitForNoteSurfaceChange(page, "exact-pointer-cancel-committed", before);
  return { id: "exact-pointer-cancel", status: "passed", producer: contract.cancellation.producer, before, live, retired, committed, cancelActions: cancelled, nextActions: ink.map(({ action, args }) => ({ action, args })) };
}

async function runNote(page: Page): Promise<Record<string, unknown>[]> {
  await ensureNoteDemo(page);
  assert.ok(await surfaceRect(page), "canonical Note InkCanvas surface is mounted");
  const results: Record<string, unknown>[] = [];
  results.push(await selectedBlockCopy(page));
  for (const id of ["structured-text-paste", "plain-text-paste", "tiny-rgba-paste"]) {
    results.push(await pasteCase(page, contract.inkCases.find((row) => row.id === id)!));
  }
  results.push(await editorPriority(page, contract.inkCases.find(({ id }) => id === "editor-priority")!));
  results.push(await exactPointerCancel(page));
  return results;
}

async function runDraw(page: Page): Promise<Record<string, unknown>[]> {
  const rect = await surfaceRect(page);
  assert.ok(rect, "canonical Draw Canvas2d host is mounted");
  const point = (x: number, y: number) => ({ x: rect.x + rect.width * x, y: rect.y + rect.height * y });
  const results: Record<string, unknown>[] = [];
  const layerCount = async (): Promise<number | null> => {
    let text: string;
    if (renderer === "react") text = await page.locator("body").innerText();
    else {
      const first = await wgpuDump(page, "dumpStructure");
      const documents = await Promise.all((first?.windowIds ?? []).map((id: string) => wgpuDump(page, "dumpStructure", id)));
      text = [first, ...documents].flatMap((document) => document?.nodes ?? []).map((node: any) => node.text ?? "").join("\n");
    }
    const match = text.match(/\b(\d+) layers?\s*·/);
    return match ? Number(match[1]) : null;
  };
  type DrawLayerRow = { id: string; text: string };
  const layerRows = async (): Promise<DrawLayerRow[]> => {
    const prefix = contract.drawSelection.layerRowIdPrefix;
    if (renderer === "react") {
      return page.evaluate((prefix) => {
        const rows = new Map<string, string>();
        for (const element of document.querySelectorAll<HTMLElement>("[id]")) {
          const start = element.id.indexOf(prefix);
          const rect = element.getBoundingClientRect();
          if (start < 0 || rect.width <= 0 || rect.height <= 0) continue;
          const id = element.id.slice(start).split("/")[0]!;
          rows.set(id, element.textContent?.replace(/\s+/g, " ").trim() ?? "");
        }
        return [...rows].map(([id, text]) => ({ id, text }));
      }, prefix);
    }
    const first = await wgpuDump(page, "dumpStructure", target.windowId);
    const windowIds: string[] = first?.windowIds?.length ? first.windowIds : [target.windowId];
    const documents = await Promise.all(windowIds.map((id) => (first?.windowId === id ? first : wgpuDump(page, "dumpStructure", id))));
    const rows = new Map<string, string[]>();
    for (const node of documents.flatMap((document) => document?.nodes ?? [])) {
      const path = String(node.path ?? "");
      const start = path.indexOf(prefix);
      if (start < 0 || !node.visible) continue;
      const id = path.slice(start).split("/")[0]!;
      const text = String(node.text ?? "").trim();
      if (text) rows.set(id, [...(rows.get(id) ?? []), text]);
    }
    return [...rows].map(([id, text]) => ({ id, text: [...new Set(text)].join(" ") }));
  };
  const waitForLayerRows = (minimum: number, label: string): Promise<DrawLayerRow[]> => waitUntil(layerRows, (rows) => rows.length >= minimum, label);
  const controlRect = async (id: string): Promise<Rect | null> => {
    if (renderer === "wgpu") return wgpuHit(page, id);
    return page.evaluate((id) => {
      const element = document.getElementById(id) ?? [...document.querySelectorAll<HTMLElement>("[id]")].find((candidate) => candidate.id.endsWith(`.${id}`));
      if (!(element instanceof HTMLElement)) return null;
      const rect = element.getBoundingClientRect();
      return rect.width > 0 && rect.height > 0 ? { x: rect.x, y: rect.y, width: rect.width, height: rect.height } : null;
    }, id);
  };
  const engagementRect = async (): Promise<Rect> => {
    let field = await controlRect(contract.drawSelection.engagementFieldId);
    if (!field) {
      await clickControl(page, [contract.drawSelection.searchToggleId, "framework.window.drawing-composite.search.toggle"]);
      field = await waitUntil<Rect | null>(() => controlRect(contract.drawSelection.engagementFieldId), (value) => value !== null, "Draw engagement input");
    }
    assert.ok(field);
    return field;
  };
  const submitEngagement = async (value: string): Promise<ActionRow[]> => {
    const field = await engagementRect();
    await page.mouse.click(field.x + field.width / 2, field.y + field.height / 2);
    await page.keyboard.press(process.platform === "darwin" ? "Meta+A" : "Control+A");
    await page.keyboard.type(value);
    const cursor = await actionCursor(page);
    await page.keyboard.press("Enter");
    return waitForAction(page, cursor, "engagementSubmit");
  };
  const capture = async (name: string): Promise<string> => {
    await page.mouse.move(1, 1);
    await page.waitForTimeout(600);
    const bytes = await page.screenshot({ clip: rect, path: join(outputDirectory, `${name}.png`) });
    return createHash("sha256").update(bytes).digest("hex");
  };
  const checkActions = async (cursor: number, id: string): Promise<ActionRow[]> => {
    const expected = contract.drawCases.find((row) => row.id === id)!.expectedActions!;
    const rows = await waitUntil(() => actionsAfter(page, cursor), (rows) => expected.every((action) => rows.some((row) => row.action === action)) && (renderer === "wgpu" || rows.filter(row => expected.includes(row.action)).every(row => Boolean(row.outcome))), `${id} settled action sequence`);
    if (renderer === "react") for (const row of rows.filter(row => expected.includes(row.action))) assert.equal((row.outcome as any)?.kind, "applied", `${id} ${row.action}: ${JSON.stringify(row.outcome)}`);
    return rows;
  };
  const utility = async (id: string): Promise<string> => {
    const controlId = renderer === "react" ? id : `framework.utility.toggle.${id}`;
    if (renderer === "react" && await page.locator(`[id="${id}"]`).getAttribute("aria-pressed").catch(() => null) === "true") return id;
    if (!(await controlRect(controlId))) {
      const unfoldId = "framework.window.drawingComposite.utilityBar.unfold";
      if (await controlRect(unfoldId)) await clickControl(page, [unfoldId]);
      const groupId = renderer === "react" ? "ui.utilities.drawing-composite.group.group:Drawing" : "framework.utility.collection.group:Drawing";
      await waitUntil(async () => (await controlRect(controlId)) ?? (await controlRect(groupId)), Boolean, "Draw utility rail publication");
      if (!(await controlRect(controlId))) await clickControl(page, [groupId]);
      await waitUntil(() => controlRect(controlId), Boolean, `Draw utility ${id}`);
    }
    const clicked = await clickControl(page, [controlId]);
    if (renderer === "react") await waitUntil(() => page.locator(`[id="${id}"]`).getAttribute("aria-pressed"), value => value === "true", `Draw utility ${id} selected`);
    return clicked;
  };
  const click = async (x: number, y: number): Promise<void> => {
    const p = point(x, y);
    await page.mouse.click(p.x, p.y);
  };
  const drawRectangle = async (x: number, name: string, caseId = "primary-draw"): Promise<Record<string, unknown>> => {
    await utility("shapeRect");
    const count = await waitUntil(layerCount, (value) => value !== null, "live Draw layer count");
    const rows = await layerRows();
    const rowIds = new Set(rows.map(({ id }) => id));
    const before = await capture(`${name}-before`);
    const cursor = await actionCursor(page);
    const start = point(x, 0.35);
    const end = point(x + 0.13, 0.5);
    await page.mouse.move(start.x, start.y);
    await page.mouse.down();
    await page.mouse.move(end.x, end.y, { steps: 3 });
    await page.mouse.up();
    const actions = await checkActions(cursor, caseId);
    const afterCount = await waitUntil(layerCount, (value) => value === count! + 1, "one committed rectangle");
    const nextRows = await waitUntil(layerRows, (value) => value.some(({ id }) => !rowIds.has(id)), "committed rectangle layer row");
    const layerId = nextRows.find(({ id }) => !rowIds.has(id))!.id;
    const after = await capture(`${name}-after`);
    if (caseId === "primary-draw") assert.notEqual(after, before, "committed rectangle changes the Canvas2d pixels");
    if (renderer === "react") await waitUntil(() => page.locator('[id="selectDirect"]').getAttribute("aria-pressed"), value => value === "true", "Draw one-shot rectangle returns to Direct Select");
    return { beforeCount: count, afterCount, before, after, actions, layerId, start, end };
  };
  await clickControl(page, ["framework.window.drawingComposite.engagement.toggle"]);
  await waitUntil(layerCount, value => value !== null, "Draw Actions layer count");
  await clickControl(page, ["framework.panel.artifact"]);
  await waitUntil(() => layoutPanelActive(page, "framework.panel.artifact"), Boolean, "Draw Artifact panel");
  const firstRectangle = await drawRectangle(0.25, "rectangle-one");
  results.push({ id: "primary-draw", status: "passed", ...firstRectangle });
  const secondRectangle = await drawRectangle(0.52, "rectangle-two");
  const firstLayerId = String(firstRectangle.layerId);
  const secondLayerId = String(secondRectangle.layerId);
  await waitForLayerRows(2, "two physical Draw layer rows");
  const nameSuffix = Date.now().toString(36);
  const selectionNames = Object.fromEntries(contract.drawSelection.cases.map((row) => [row.id, `${row.namePrefix} ${nameSuffix}`])) as Record<DrawSelectionCase["id"], string>;
  await utility("selectDirect");
  await click(0.82, 0.8);
  const unselected = await capture("selection-empty");
  const selectCursor = await actionCursor(page);
  await click(0.315, 0.425);
  const selectActions = await checkActions(selectCursor, "primary-select");
  const selected = await capture("selection-one");
  const singleName = selectionNames["single-selection-rename"];
  const singleSubmit = await submitEngagement(singleName);
  const singleRows = await waitUntil(layerRows, (rows) => rows.some(({ id, text }) => id === firstLayerId && text.includes(singleName)), "single selection engagement rename");
  assert.ok(singleRows.some(({ id, text }) => id === secondLayerId && !text.includes(singleName)), "single selection renames only the first physical layer");
  results.push({ id: "primary-select", status: "passed", before: unselected, after: selected, selectedLayerId: firstLayerId, renamedTo: singleName, actions: [...selectActions, ...singleSubmit] });
  const modifierCursor = await actionCursor(page);
  await page.keyboard.down("Shift");
  try { await click(0.585, 0.425); } finally { await page.keyboard.up("Shift"); }
  await checkActions(modifierCursor, "selection-modifiers");
  const added = await capture("selection-two");
  const multiName = selectionNames["multi-selection-rename"];
  const multiSubmit = await submitEngagement(multiName);
  await page.waitForTimeout(500);
  const multiRows = await layerRows();
  assert.ok(multiRows.some(({ id, text }) => id === firstLayerId && text.includes(singleName)), "multi-selection engagement leaves the first layer name unchanged");
  assert.equal(multiRows.some(({ text }) => text.includes(multiName)), false, "multi-selection engagement refuses rename mutation");
  const modifier = process.platform === "darwin" ? "Meta" : "Control";
  await page.keyboard.down(modifier);
  try { await click(0.585, 0.425); } finally { await page.keyboard.up(modifier); }
  const subtracted = await capture("selection-subtracted");
  const subtractName = selectionNames["subtracted-selection-rename"];
  const subtractSubmit = await submitEngagement(subtractName);
  const subtractRows = await waitUntil(layerRows, (rows) => rows.some(({ id, text }) => id === firstLayerId && text.includes(subtractName)), "subtracted selection engagement rename");
  assert.ok(subtractRows.some(({ id, text }) => id === secondLayerId && !text.includes(subtractName)), "modifier subtraction keeps the second physical layer unchanged");
  results.push({
    id: "selection-modifiers",
    status: "passed",
    selected,
    added,
    subtracted,
    firstLayerId,
    secondLayerId,
    rejectedMultiName: multiName,
    renamedAfterSubtract: subtractName,
    actions: [...multiSubmit, ...subtractSubmit],
  });
  const repeatedRectangle = await drawRectangle(0.52, "rectangle-three", "repeated-geometry-creation");
  assert.deepEqual(repeatedRectangle.start, secondRectangle.start, "repeated creation uses the same physical start");
  assert.deepEqual(repeatedRectangle.end, secondRectangle.end, "repeated creation uses the same physical end");
  assert.notEqual(repeatedRectangle.layerId, secondLayerId, "distinct creation operations retain distinct layer identities");
  results.push({ id: "repeated-geometry-creation", status: "passed", originalLayerId: secondLayerId, ...repeatedRectangle });
  await utility("pen");
  const countBeforeCancel = await layerCount();
  const beforeDraft = await capture("draft-before");
  const cancelCursor = await actionCursor(page);
  await click(0.27, 0.65);
  await click(0.34, 0.72);
  const draft = await capture("draft-live");
  assert.notEqual(draft, beforeDraft, "the pen draft is visibly live before cancellation");
  await page.keyboard.press("Escape");
  const cancelActions = await checkActions(cancelCursor, "escape-cancel");
  assert.equal(await layerCount(), countBeforeCancel, "Escape does not commit a layer");
  const cancelled = await capture("draft-cancelled");
  assert.equal(cancelled, beforeDraft, "Escape retires the visible draft preview");
  results.push({ id: "escape-cancel", status: "passed", layers: countBeforeCancel, beforeDraft, draft, cancelled, actions: cancelActions });
  await utility("pen");
  const beforeCommit = await layerCount();
  const commitCursor = await actionCursor(page);
  await click(0.54, 0.64);
  await click(0.61, 0.72);
  const end = point(0.68, 0.64);
  await page.mouse.dblclick(end.x, end.y);
  const commitActions = await checkActions(commitCursor, "double-click-commit");
  const committed = await waitUntil(layerCount, (value) => value === beforeCommit! + 1, "exactly one double-click draft commit");
  results.push({ id: "double-click-commit", status: "passed", beforeCount: beforeCommit, afterCount: committed, after: await capture("draft-committed"), actions: commitActions });
  return results;
}

async function beginMouseDrag(page: Page, source: Rect, destination: Rect): Promise<void> {
  const from = { x: source.x + source.width / 2, y: source.y + source.height / 2 };
  const to = { x: destination.x + destination.width / 2, y: destination.y + destination.height / 2 };
  await page.mouse.move(from.x, from.y);
  await page.mouse.down();
  if (renderer === "react") await waitUntil(() => page.evaluate(({ x, y }) => Boolean(document.elementFromPoint(x, y)?.closest('[draggable="true"]')), from), Boolean, "physical transfer handle arms its native drag owner");
  else await page.waitForTimeout(100);
  await page.mouse.move(from.x + Math.min(12, source.width / 3), from.y, { steps: 4 });
  await page.mouse.move(to.x, to.y, { steps: 16 });
}

async function layoutPanelActive(page: Page, tabId: string): Promise<boolean> {
  if (renderer === "react") return (await page.locator(`[data-active-tab-id="${tabId}"][data-panel-visible="true"]`).count()) > 0;
  const dump = await wgpuDump(page, "dumpStructure", "shell.chrome");
  return (dump?.nodes ?? []).some((node: any) => node.key === tabId && (node.checked === true || node.selected === true));
}

async function openLayoutPanel(page: Page, tabId: string): Promise<void> {
  if (!(await layoutPanelActive(page, tabId))) await clickControl(page, [tabId, `framework.panelTab.${tabId}`]);
  await waitUntil(() => layoutPanelActive(page, tabId), Boolean, `Layout panel ${tabId}`);
}

async function reloadLayoutDemo(page: Page): Promise<void> {
  await page.reload({ waitUntil: "domcontentloaded" });
  await waitForBoot(page);
  await dismissIntroduction(page);
  await waitUntil(() => surfaceRect(page), Boolean, "Layout demo Canvas2d surface");
}

async function layoutCollectionCount(page: Page, collection: LayoutCatalogueCase["createdCollection"]): Promise<number> {
  const sectionId = collection === "pages" ? contract.layoutCatalogue.pageSectionId : contract.layoutCatalogue.frameSectionId;
  await openLayoutPanel(page, contract.layoutCatalogue.artifactTabId);
  if (renderer === "react") {
    const section = page.locator(`[data-active-tab-id="${contract.layoutCatalogue.artifactTabId}"][data-panel-visible="true"] [id="panel:layout-document/${sectionId}"]`);
    await waitUntil(() => section.count(), (count) => count === 1, `Layout ${collection} section`);
    if ((await section.getAttribute("data-state")) === "closed") await section.click();
    const owner = section.locator("xpath=..");
    return owner.locator('[role="treeitem"][data-tree-row-kind]').count();
  }
  const dump = await waitUntil(
    () => wgpuDump(page, "dumpStructure", contract.layoutCatalogue.artifactTabId),
    (value) => (value?.nodes ?? []).some((node: any) => node.key === sectionId || String(node.key).endsWith(`/${sectionId}`)),
    `Layout ${collection} retained section`,
  );
  const nodes = dump.nodes as any[];
  const index = nodes.findIndex((node) => node.key === sectionId || String(node.key).endsWith(`/${sectionId}`));
  const depth = nodes[index].depth;
  let count = 0;
  for (const node of nodes.slice(index + 1)) {
    if (node.depth <= depth) break;
    if (node.depth === depth + 1 && node.role === "treeitem") count += 1;
  }
  return count;
}

async function captureLayoutSurface(page: Page, name: string): Promise<{ file: string; hash: string; rect: Rect }> {
  await page.waitForTimeout(250);
  const rect = await surfaceRect(page);
  assert.ok(rect, `${name} has the canonical Layout Canvas2d surface`);
  const file = `${name}.png`;
  let bytes: Buffer;
  if (renderer === "react") {
    const data = await page.locator(target.reactSelector).evaluate(canvas => (canvas as HTMLCanvasElement).toDataURL("image/png"));
    assert.ok(data.startsWith("data:image/png;base64,"), `${name} exports the actual Canvas2d bitmap`);
    bytes = Buffer.from(data.slice("data:image/png;base64,".length), "base64");
    writeFileSync(join(outputDirectory, file), bytes);
    await page.screenshot({ clip: rect, path: join(outputDirectory, `${name}.visible.png`) });
  } else bytes = await page.screenshot({ clip: rect, path: join(outputDirectory, file) });
  return { file, hash: createHash("sha256").update(bytes).digest("hex"), rect };
}

async function layoutSourceRect(page: Page, row: LayoutCatalogueCase): Promise<Rect> {
  await openLayoutPanel(page, contract.layoutCatalogue.catalogueTabId);
  if (renderer === "wgpu") {
    const hit = await waitUntil<Rect | null>(
      () => wgpuHit(page, `${contract.layoutCatalogue.transferHandlePrefix}${row.sourceId}`),
      (value) => value !== null,
      `${row.id} retained transfer handle`,
    );
    return hit!;
  }
  const handle = page.locator(`[data-active-tab-id="${contract.layoutCatalogue.catalogueTabId}"][data-panel-visible="true"] [id="panel:layout-catalogue/${row.sourceId}"] [data-slot="drag-handle"][data-drag-role="transfer"]`);
  const witness = await waitUntil(
    () =>
      handle.evaluate((element) => {
        const source = element.closest<HTMLElement>("[data-drag-mime][data-drag-payload]");
        if (!source) return null;
        const rect = element.getBoundingClientRect();
        return { mime: source.dataset.dragMime, payload: source.dataset.dragPayload, rect: { x: rect.x, y: rect.y, width: rect.width, height: rect.height } };
      }).catch(() => null),
    Boolean,
    `${row.id} React transfer handle`,
  );
  assert.equal(witness!.mime, contract.layoutCatalogue.mime);
  assert.equal(witness!.payload, row.dragPayload);
  return witness!.rect;
}

async function beginLayoutCatalogueDrag(page: Page, row: LayoutCatalogueCase): Promise<{ cursor: number; target: Rect }> {
  const source = await layoutSourceRect(page, row);
  const targetRect = await surfaceRect(page);
  assert.ok(targetRect, `${row.id} has a Layout drop target`);
  const cursor = await actionCursor(page);
  await beginMouseDrag(page, source, { x: targetRect.x + targetRect.width * 0.55, y: targetRect.y + targetRect.height * 0.5, width: 1, height: 1 });
  const preview = await waitForAction(page, cursor, contract.layoutCatalogue.previewAction);
  const over = preview.find(({ action }) => action === contract.layoutCatalogue.previewAction)!;
  await assertLayoutDragWitness(page, row, over, "dragover");
  return { cursor, target: targetRect };
}

async function assertLayoutDragWitness(page: Page, row: LayoutCatalogueCase, action: ActionRow, type: "dragover" | "drop"): Promise<void> {
  if (renderer === "wgpu") {
    assert.equal(action.controller, target.controllerId, `${row.id} stays with the Layout editor`);
    const args = JSON.parse(action.args ?? "null");
    if (type === "drop") assert.equal(args?.dragData, row.dragPayload, `${row.id} drop carries the canonical payload`);
    else {
      assert.ok(args?.types?.includes(contract.layoutCatalogue.mime), `${row.id} preview carries the base MIME`);
      assert.ok(args?.types?.includes(`${contract.layoutCatalogue.kindMimePrefix}${row.kind}`), `${row.id} preview carries the kind MIME`);
    }
    return;
  }
  const event = await page.evaluate(({ type, key }) => ((globalThis as any).__semioCanvasPhysicalPointerTransfers ?? []).filter((event: any) => event.type === type && event.key === key).at(-1), { type: type === "drop" ? "pointerup" : "pointermove", key: target.surfaceId });
  assert.ok(event?.trusted, `${row.id} has a trusted catalogue pointer ${type}`);
  assert.equal(event.key, target.surfaceId);
  assert.equal(event.surfaceId, `window:${target.windowId}`);
  assert.equal(action.controller, event.controllerId, `${row.id} ledger action matches the addressed host record`);
  assert.equal(event.source.id, `panel:layout-catalogue/${row.sourceId}`);
  assert.equal(event.source.mime, contract.layoutCatalogue.mime);
  assert.equal(event.source.payload, row.dragPayload);
  assert.equal(action.windowId, target.windowId);
  const settled = await waitUntil(() => actionRows(page).then(rows => rows.find(candidate => candidate.seq === action.seq)), value => Boolean(value?.outcome), `${row.id} ${action.action} terminal outcome`);
  assert.equal((settled?.outcome as any)?.kind, "applied", `${row.id} ${action.action}: ${JSON.stringify(settled?.outcome)}`);
}

async function runLayout(page: Page): Promise<Record<string, unknown>[]> {
  const results: Record<string, unknown>[] = [];
  for (const row of contract.layoutCatalogue.cases) {
    await reloadLayoutDemo(page);
    const beforeCount = await waitUntil(
      () => layoutCollectionCount(page, row.createdCollection),
      (count) => count === contract.layoutCatalogue.seed[row.createdCollection],
      `${row.id} seed ${row.createdCollection}`,
    );
    await openLayoutPanel(page, contract.layoutCatalogue.catalogueTabId);
    const before = await captureLayoutSurface(page, `${row.id}-before`);
    const leaveDrag = await beginLayoutCatalogueDrag(page, row);
    const preview =
      row.previewPixels === "changed"
        ? await waitUntil(() => captureLayoutSurface(page, `${row.id}-preview`), (capture) => capture.hash !== before.hash, `${row.id} preview pixels`)
        : await captureLayoutSurface(page, `${row.id}-preview`);
    assert.equal(preview.hash === before.hash, row.previewPixels === "unchanged", `${row.id} preview pixel policy`);
    await page.mouse.move(2, 2, { steps: 12 });
    const leaveRows = await waitForAction(page, leaveDrag.cursor, contract.layoutCatalogue.leaveAction);
    await page.mouse.up();
    const retired = await waitUntil(() => captureLayoutSurface(page, `${row.id}-retired`), (capture) => capture.hash === before.hash, `${row.id} leave retirement`);
    assert.equal(leaveRows.some(({ action }) => action === contract.layoutCatalogue.dropAction), false, `${row.id} leave does not synthesize a drop`);
    assert.equal(await layoutCollectionCount(page, row.createdCollection), beforeCount, `${row.id} leave preserves the document`);
    const leaveActions = await settledLayoutActions(page, leaveDrag.cursor);

    await openLayoutPanel(page, contract.layoutCatalogue.catalogueTabId);
    const dropDrag = await beginLayoutCatalogueDrag(page, row);
    await page.mouse.up();
    const dropped = await waitForAction(page, dropDrag.cursor, contract.layoutCatalogue.dropAction);
    const terminal = dropped.filter(({ action }) => action === contract.layoutCatalogue.leaveAction || action === contract.layoutCatalogue.dropAction);
    assert.deepEqual(terminal.slice(-2).map(({ action }) => action), [contract.layoutCatalogue.leaveAction, contract.layoutCatalogue.dropAction], `${row.id} drop retires preview before commit`);
    const drop = terminal.at(-1)!;
    await assertLayoutDragWitness(page, row, drop, "drop");
    const afterCount = await waitUntil(
      () => layoutCollectionCount(page, row.createdCollection),
      (count) => count === beforeCount + row.createdDelta,
      `${row.id} exact created ${row.createdCollection}`,
    );
    const committed = await captureLayoutSurface(page, `${row.id}-committed`);
    const dropActions = await settledLayoutActions(page, dropDrag.cursor);
    results.push({ id: row.id, status: "passed", kind: row.kind, sourceId: row.sourceId, beforeCount, afterCount, before, preview, retired, committed, leaveActions, dropActions });
  }
  return results;
}

async function settledLayoutActions(page: Page, cursor: number): Promise<ActionRow[]> {
  const names: readonly string[] = [contract.layoutCatalogue.previewAction, contract.layoutCatalogue.leaveAction, contract.layoutCatalogue.dropAction];
  const rows = await waitUntil(() => actionsAfter(page, cursor), rows => renderer === "wgpu" || rows.filter(row => names.includes(row.action)).every(row => Boolean(row.outcome)), "Layout transfer terminal outcomes");
  if (renderer === "react") for (const row of rows.filter(row => names.includes(row.action))) assert.equal((row.outcome as any)?.kind, "applied", `${row.action}: ${JSON.stringify(row.outcome)}`);
  return rows;
}

async function run(): Promise<void> {
  const browser = await chromium.launch({ headless: process.env.SEMIO_CANVAS_HEADED !== "1", args: browserArgs });
  const context = await browser.newContext({ viewport, deviceScaleFactor: 1 });
  await context.grantPermissions(["clipboard-read", "clipboard-write"], { origin: new URL(url).origin });
  let page: Page | undefined;
  let consoleLines: string[] = [];
  try {
    ({ page, consoleLines } = await preparePage(context));
    const mountedPage = page;
    const mounted = await waitUntil(() => surfaceRect(mountedPage), Boolean, `${targetName} ${target.surfaceKind} surface`);
    const cases = targetName === "note" ? await runNote(page) : targetName === "layout" ? await runLayout(page) : await runDraw(page);
    await page.screenshot({ path: join(outputDirectory, "final.png") });
    const receipt = { target: targetName, renderer, url, mounted, cases, consoleLines, observation: await observe(page) };
    writeFileSync(join(outputDirectory, "receipt.json"), JSON.stringify(receipt, null, 2));
    process.stdout.write(JSON.stringify({ target: targetName, renderer, url, cases: cases.map(({ id, status }) => ({ id, status })) }, null, 2) + "\n");
  } catch (error) {
    if (page) await page.screenshot({ path: join(outputDirectory, "failure.png") }).catch(() => undefined);
    const observation = page ? await observe(page).catch((failure) => ({ error: String(failure) })) : null;
    writeFileSync(join(outputDirectory, "failure.json"), JSON.stringify({ target: targetName, renderer, url, error: String(error), consoleLines, observation }, null, 2));
    throw error;
  } finally {
    await browser.close();
  }
}

async function testContract(): Promise<void> {
  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext({ viewport: { width: 320, height: 240 }, hasTouch: true });
  try {
    const page = await context.newPage();
    await page.setContent('<div id="surface" style="width:320px;height:240px;touch-action:none"></div>');
    await page.evaluate(() => {
      (globalThis as any).__pointerEvents = [];
      const surface = document.getElementById("surface")!;
      for (const type of ["pointerdown", "pointermove", "pointerup", "pointercancel"]) {
        surface.addEventListener(type, (event) => (globalThis as any).__pointerEvents.push({ type, trusted: event.isTrusted, pointerId: (event as PointerEvent).pointerId }));
      }
    });
    await touchSequence(page, [
      { type: "touchStart", x: 40, y: 40 },
      { type: "touchMove", x: 64, y: 60 },
      { type: "touchCancel" },
    ]);
    const events = await page.evaluate(() => (globalThis as any).__pointerEvents);
    assert.ok(events.some((event: any) => event.type === "pointerdown"));
    assert.ok(events.some((event: any) => event.type === "pointermove"));
    assert.ok(events.some((event: any) => event.type === "pointercancel"));
    assert.equal(events.some((event: any) => event.type === "pointerup"), false);
    assert.ok(events.every((event: any) => event.trusted), `CDP touch events remain browser-trusted: ${JSON.stringify(events)}`);
    await page.setContent('<div id="source" draggable="true" style="position:absolute;left:20px;top:20px;width:40px;height:40px"></div><div id="drop" style="position:absolute;left:180px;top:40px;width:100px;height:100px"></div>');
    await page.evaluate(
      ({ mime, kindMime, payload }) => {
        (globalThis as any).__dragEvents = [];
        const source = document.getElementById("source")!;
        const drop = document.getElementById("drop")!;
        source.addEventListener("dragstart", (event) => {
          event.dataTransfer!.setData(mime, payload);
          event.dataTransfer!.setData(kindMime, "");
          (globalThis as any).__dragEvents.push({ type: event.type, trusted: event.isTrusted });
        });
        for (const type of ["dragover", "dragleave", "drop"]) {
          drop.addEventListener(type, (event) => {
            event.preventDefault();
            (globalThis as any).__dragEvents.push({ type, trusted: event.isTrusted, types: [...event.dataTransfer!.types], payload: event.dataTransfer!.getData(mime) });
          });
        }
      },
      { mime: contract.layoutCatalogue.mime, kindMime: `${contract.layoutCatalogue.kindMimePrefix}rect`, payload: '{"kind":"rect"}' },
    );
    const source = (await page.locator("#source").boundingBox())!;
    const drop = (await page.locator("#drop").boundingBox())!;
    await beginMouseDrag(page, source, drop);
    await page.waitForFunction(() => (globalThis as any).__dragEvents.some((event: any) => event.type === "dragover"));
    await page.mouse.move(310, 220, { steps: 8 });
    await page.waitForFunction(() => (globalThis as any).__dragEvents.some((event: any) => event.type === "dragleave"));
    await page.mouse.up();
    await beginMouseDrag(page, source, drop);
    await page.mouse.up();
    await page.waitForFunction(() => (globalThis as any).__dragEvents.some((event: any) => event.type === "drop"));
    const dragEvents = await page.evaluate(() => (globalThis as any).__dragEvents);
    assert.ok(dragEvents.every((event: any) => event.trusted), `mouse drag events remain browser-trusted: ${JSON.stringify(dragEvents)}`);
    const dropEvent = dragEvents.findLast((event: any) => event.type === "drop");
    assert.ok(dropEvent.types.includes(contract.layoutCatalogue.mime));
    assert.ok(dropEvent.types.includes(`${contract.layoutCatalogue.kindMimePrefix}rect`));
    assert.equal(dropEvent.payload, '{"kind":"rect"}');
    process.stdout.write(JSON.stringify({ fixture: "PASS", version: contract.version, targets: Object.keys(contract.targets), drawCases: contract.drawCases.length, layoutCases: contract.layoutCatalogue.cases.length, inkCases: contract.inkCases.length, cancellation: contract.cancellation.producer, chromium: events, physicalDrag: dragEvents }) + "\n");
  } finally {
    await browser.close();
  }
}

if (command === "test") {
  await testContract();
} else if (command === "run") await run();
else if (command === "discover") await discover();
else throw new Error(`Unknown command: ${command}`);
