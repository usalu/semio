/** @emoji 🕸️ Trusted Chromium acceptance adapter for the canonical Flow NodeGraph surface. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv2020 from "ajv/dist/2020.js";
import { chromium, type BrowserContext, type Page } from "playwright";

type Renderer = "react" | "wgpu";
type Rect = { x: number; y: number; width: number; height: number };
type Delta = { x: number; y: number };
type ActionRow = { seq: number; controller: string; action: string; args?: string };
type GeometryRow = { id: string; point: { x: number; y: number }; rect: Rect; world?: { x: number; y: number } };
type GeometrySnapshot = { surface: Rect; rows: GeometryRow[] };
type ProbeCase = {
  id: "select-and-escape" | "escape-cancels-node-drag" | "commit-node-drag" | "wheel-zoom" | "middle-button-pan";
  gesture: string;
  delta?: Delta;
  deltaY?: number;
  cancelKey?: "Escape";
  expectedActions?: string[];
  expectedAction?: string;
  forbiddenAction?: string;
  operation?: "move";
  button?: "middle";
  consequence: string;
};
type Fixture = {
  version: 1;
  target: {
    plugin: "flow";
    controllerId: "s.flow.flow@1/*#editor";
    windowId: "flow-main";
    surfaceId: "flow.play.main";
    surfaceKind: "node-graph";
    reactSelector: string;
    urls: Record<Renderer, string>;
  };
  specimen: { minimumNodes: number; preferredNodeIds: ["slider", "add", "preview"] };
  cases: ProbeCase[];
  limits: {
    settleMilliseconds: number;
    geometryEpsilonPixels: number;
    cancelTolerancePixels: number;
    minimumCommittedMovementPixels: number;
    surfaceInsetPixels: number;
  };
};
type CaseResult = { id: string; status: "passed" | "failed"; [key: string]: unknown };

const command = process.argv[2] ?? "test";
const renderer = (process.env.SEMIO_SURFACE_RENDERER ?? "react") as Renderer;
const fixturePath = join(import.meta.dir, "🧫️fixtures", "🔣️.json");
const schemaPath = join(import.meta.dir, "🧬️schema", "🕸️flow-node-graph", "🔣️.json");
const ticketRoot = resolve(import.meta.dir, "..");
const repoRoot = resolve(ticketRoot, "../../../../../../../");
const outputRoot = join(ticketRoot, "🗑️generated", "astra-surface-interactions");
const outputDirectory = join(outputRoot, renderer);
const viewport = { width: 1600, height: 1000 };

/** 🧬️ Validates the language-neutral physical acceptance contract before any browser work. */
function loadFixture(): Fixture {
  const value = JSON.parse(readFileSync(fixturePath, "utf8")) as Fixture;
  const ajv = new Ajv2020({ strict: true });
  const validate = ajv.compile(JSON.parse(readFileSync(schemaPath, "utf8")));
  assert.ok(validate(value), ajv.errorsText(validate.errors));
  assert.equal(value.version, 1);
  assert.deepEqual(value.specimen.preferredNodeIds, ["slider", "add", "preview"]);
  assert.deepEqual(value.cases.map(({ id }) => id), ["select-and-escape", "escape-cancels-node-drag", "commit-node-drag", "wheel-zoom", "middle-button-pan"]);
  assert.equal(new Set(value.cases.map(({ id }) => id)).size, value.cases.length);
  for (const row of value.cases) assert.ok(row.consequence.length > 0);
  return value;
}

const contract = loadFixture();
assert.ok(renderer === "react" || renderer === "wgpu", `Unknown renderer: ${renderer}`);
const explicitUrl = process.env.SEMIO_SURFACE_URL ?? (renderer === "react" ? process.env.SEMIO_SURFACE_REACT_URL : process.env.SEMIO_SURFACE_WGPU_URL);
const url = explicitUrl ?? contract.target.urls[renderer];
mkdirSync(outputDirectory, { recursive: true });

/** ⏳️ Polls one read-only witness under the fixture's fixed settle deadline. */
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

/** 🚀️ Waits for the renderer-owned boot beacon and refuses its explicit error state. */
async function waitForBoot(page: Page): Promise<void> {
  await page.waitForFunction(() => document.documentElement.getAttribute("data-semio-os-ready") !== null || document.documentElement.getAttribute("data-semio-os-error") !== null, undefined, { timeout: 120_000 });
  const error = await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-error"));
  if (error) throw new Error(`${renderer} shell reported ${error}`);
  await page.waitForTimeout(1_000);
}

/** 🎓️ Dismisses the optional introduction only through its published physical control. */
async function dismissIntroduction(page: Page): Promise<void> {
  if (renderer === "react") {
    const row = page.locator("#ui\\.introduction\\.skip").first();
    const rect = await row.boundingBox().catch(() => null);
    if (rect) await page.mouse.click(rect.x + rect.width / 2, rect.y + rect.height / 2);
    return;
  }
  const dump = await wgpuDump(page, "dumpChrome");
  const hit = dump?.hits?.find((entry: any) => entry.controlId === "ui.introduction.skip");
  if (hit) await page.mouse.click(hit.rect[0] + hit.rect[2] / 2, hit.rect[1] + hit.rect[3] / 2);
}

/** 🔬️ Reads one public WGPU introspection lane. */
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

/** 📐️ Finds the exact retained ComponentScene rect for Flow's main body. */
async function wgpuSurfaceRect(page: Page): Promise<Rect | null> {
  const first = await wgpuDump(page, "dumpStructure", contract.target.windowId);
  const ids = first?.windowIds?.length ? first.windowIds : [contract.target.windowId];
  for (const id of ids) {
    const dump = id === contract.target.windowId ? first : await wgpuDump(page, "dumpStructure", id);
    const scenes = (dump?.nodes ?? []).filter((node: any) => node.kind === "componentScene" && node.visible && node.rect?.[2] > 0 && node.rect?.[3] > 0);
    const node = scenes.find((entry: any) => String(entry.path).includes(contract.target.surfaceId)) ?? (id === contract.target.windowId && scenes.length === 1 ? scenes[0] : null);
    if (node) return { x: node.rect[0], y: node.rect[1], width: node.rect[2], height: node.rect[3] };
  }
  return null;
}

/** 📐️ Reads the mounted surface box from the renderer's own published surface. */
async function surfaceRect(page: Page): Promise<Rect | null> {
  if (renderer === "wgpu") return wgpuSurfaceRect(page);
  const box = await page.locator(contract.target.reactSelector).first().boundingBox();
  return box ? { x: box.x, y: box.y, width: box.width, height: box.height } : null;
}

/** 🎬️ Reads the renderer's bounded action ledger without injecting an action. */
async function actionRows(page: Page): Promise<ActionRow[]> {
  if (renderer === "react") {
    return page.evaluate(() =>
      ((globalThis as any).__semioInputLedger?.recent ?? []).map((row: any) => ({ seq: row.inputSeq, controller: row.controllerId ?? "?", action: row.action, args: JSON.stringify(row.args ?? null) })),
    );
  }
  const dump = await wgpuDump(page, "dumpChrome");
  return (dump?.actions ?? []).map((row: any) => ({ seq: row.seq, controller: row.controllerId, action: row.action, args: row.args }));
}

/** 🎬️ Marks the last dispatched action sequence. */
async function actionCursor(page: Page): Promise<number> {
  return (await actionRows(page)).at(-1)?.seq ?? 0;
}

/** 🎬️ Returns only actions produced after a marked physical step. */
async function actionsAfter(page: Page, cursor: number): Promise<ActionRow[]> {
  return (await actionRows(page)).filter(({ seq }) => seq > cursor);
}

/** 🎬️ Waits for one exact Flow action from the canonical controller. */
async function waitForAction(page: Page, cursor: number, action: string): Promise<ActionRow[]> {
  return waitUntil(
    () => actionsAfter(page, cursor),
    (rows) => rows.some((row) => row.controller === contract.target.controllerId && row.action === action),
    `${action} action`,
  );
}

/** 🧾️ Decodes a bounded action's JSON arguments. */
function actionArgs(row: ActionRow): Record<string, any> {
  if (!row.args) return {};
  try {
    return JSON.parse(row.args) as Record<string, any>;
  } catch {
    return {};
  }
}

/** 🧾️ Decodes the framework interaction target string nested inside action args. */
function interactionTargets(row: ActionRow): Array<{ granularity?: string; id?: string }> {
  const targets = actionArgs(row).targets;
  if (Array.isArray(targets)) return targets;
  if (typeof targets !== "string") return [];
  try {
    const parsed = JSON.parse(targets);
    return Array.isArray(parsed) ? parsed : [];
  } catch {
    return [];
  }
}

/** 🗺️ Decodes the newest WGPU node-geometry census for Flow. */
function latestWgpuCensus(lines: readonly string[]): any[] | null {
  const marker = `[DEBUG] wgpu node-graph geometry surface=${contract.target.surfaceId} entities=`;
  for (let index = lines.length - 1; index >= 0; index -= 1) {
    const start = lines[index].indexOf(marker);
    if (start < 0) continue;
    try {
      const parsed = JSON.parse(lines[index].slice(start + marker.length));
      if (Array.isArray(parsed)) return parsed;
    } catch {
      continue;
    }
  }
  return null;
}

/** 🗺️ Reads every visible node from the mounted engine's own geometry projection. */
async function geometry(page: Page, consoleLines: readonly string[]): Promise<GeometrySnapshot | null> {
  if (renderer === "react") {
    return page.evaluate(
      ({ surfaceId }) => {
        const probe = (globalThis as any).__semioFlowGraphProbe?.[surfaceId];
        if (!probe) return null;
        const surface = probe.rect?.();
        const ids: string[] = probe.nodeIds?.() ?? [];
        const layout = probe.nodeLayout?.() ?? {};
        const rows = ids.flatMap((id) => {
          const entity = probe.entity?.("node", id);
          if (!entity?.visible || !entity.point || !entity.rect) return [];
          return [{ id, point: entity.point, rect: entity.rect, world: layout[id] }];
        });
        return surface ? { surface, rows } : null;
      },
      { surfaceId: contract.target.surfaceId },
    );
  }
  const surface = await wgpuSurfaceRect(page);
  const census = latestWgpuCensus(consoleLines);
  if (!surface || !census) return null;
  const rows = census.flatMap((entry: any) => {
    const value = entry.geometry;
    if (entry.kind !== "node" || !value?.visible || !Array.isArray(value.rect) || value.rect.length !== 4) return [];
    return [{ id: entry.id, point: { x: surface.x + value.x, y: surface.y + value.y }, rect: { x: surface.x + value.rect[0], y: surface.y + value.rect[1], width: value.rect[2], height: value.rect[3] } }];
  });
  return { surface, rows };
}

/** 🗺️ Waits until the real specimen has enough targetable nodes. */
async function mountedGeometry(page: Page, consoleLines: readonly string[]): Promise<GeometrySnapshot> {
  return waitUntil(
    () => geometry(page, consoleLines),
    (value) => Boolean(value && value.rows.length >= contract.specimen.minimumNodes),
    `mounted ${contract.target.surfaceId} geometry`,
  ) as Promise<GeometrySnapshot>;
}

/** 🎯️ Selects a stable preferred node that is physically inside the surface. */
function targetNode(snapshot: GeometrySnapshot, excluded: readonly string[] = []): GeometryRow {
  const inset = contract.limits.surfaceInsetPixels;
  const inside = (row: GeometryRow) => row.point.x >= snapshot.surface.x + inset && row.point.x <= snapshot.surface.x + snapshot.surface.width - inset && row.point.y >= snapshot.surface.y + inset && row.point.y <= snapshot.surface.y + snapshot.surface.height - inset;
  const row = contract.specimen.preferredNodeIds.map((id) => snapshot.rows.find((entry) => entry.id === id && !excluded.includes(id) && inside(entry))).find(Boolean) ?? snapshot.rows.find((entry) => !excluded.includes(entry.id) && inside(entry));
  assert.ok(row, "the Flow specimen has a preferred visible node inside its surface");
  return row;
}

/** 📏️ Measures the screen displacement of one node between snapshots. */
function nodeDisplacement(before: GeometrySnapshot, after: GeometrySnapshot, id: string): number {
  const left = before.rows.find((row) => row.id === id);
  const right = after.rows.find((row) => row.id === id);
  assert.ok(left && right, `node ${id} remains introspectable`);
  return Math.hypot(right.point.x - left.point.x, right.point.y - left.point.y);
}

/** 📏️ Measures world-space displacement where the React probe exposes it. */
function worldDisplacement(before: GeometrySnapshot, after: GeometrySnapshot, id: string): number | null {
  const left = before.rows.find((row) => row.id === id)?.world;
  const right = after.rows.find((row) => row.id === id)?.world;
  return left && right ? Math.hypot(right.x - left.x, right.y - left.y) : null;
}

/** 🖼️ Captures the mounted Flow surface and returns its content hash. */
async function capture(page: Page, name: string): Promise<{ file: string; hash: string; rect: Rect }> {
  const rect = await waitUntil(surfaceRect.bind(null, page), Boolean, `${name} surface rect`) as Rect;
  const file = `${name}.png`;
  const bytes = await page.screenshot({ clip: rect, path: join(outputDirectory, file) });
  return { file, hash: createHash("sha256").update(bytes).digest("hex"), rect };
}

/** 🖱️ Performs one trusted drag with Playwright's browser input. */
async function drag(page: Page, from: { x: number; y: number }, delta: Delta, button: "left" | "middle" = "left", release = true): Promise<void> {
  await page.mouse.move(from.x, from.y);
  await page.mouse.down({ button });
  await page.mouse.move(from.x + delta.x, from.y + delta.y, { steps: 12 });
  if (release) await page.mouse.up({ button });
}

/** 🧭️ Returns a camera argument from a viewport action. */
function viewportFrom(rows: readonly ActionRow[]): { x: number; y: number; zoom: number } | null {
  for (const row of [...rows].reverse()) {
    if (row.action !== "nodeGraphViewport") continue;
    const viewport = actionArgs(row).viewport;
    if (viewport && [viewport.x, viewport.y, viewport.zoom].every((value) => typeof value === "number" && Number.isFinite(value))) return viewport;
  }
  return null;
}

/** 🎯️ Drives selection and proves Escape clears the exact framework node target. */
async function runSelection(page: Page, consoleLines: readonly string[], row: ProbeCase): Promise<CaseResult> {
  const before = await mountedGeometry(page, consoleLines);
  const target = targetNode(before);
  const baseline = await capture(page, `${row.id}-before`);
  const selectCursor = await actionCursor(page);
  await page.mouse.click(target.point.x, target.point.y);
  const selectedRows = await waitForAction(page, selectCursor, "interactionSelect");
  const selected = selectedRows.find((action) => action.action === "interactionSelect" && interactionTargets(action).some((entry) => entry.granularity === "node" && entry.id === target.id));
  assert.ok(selected, `selection publishes exact node ${target.id}`);
  const selectedCapture = await capture(page, `${row.id}-selected`);
  assert.notEqual(selectedCapture.hash, baseline.hash, "selection has an app-visible painted consequence");
  const clearCursor = await actionCursor(page);
  await page.keyboard.press("Escape");
  const clearedRows = await waitForAction(page, clearCursor, "clearSelection");
  assert.ok(clearedRows.some((action) => action.controller === contract.target.controllerId && action.action === "clearSelection"));
  await page.mouse.move(before.surface.x + contract.limits.surfaceInsetPixels, before.surface.y + contract.limits.surfaceInsetPixels);
  const clearedCapture = await capture(page, `${row.id}-cleared`);
  assert.notEqual(clearedCapture.hash, selectedCapture.hash, "Escape visibly retires the selected paint");
  return { id: row.id, status: "passed", target: target.id, actions: [...selectedRows, ...clearedRows], baseline, selected: selectedCapture, cleared: clearedCapture };
}

/** ⎋️ Starts a real node drag and requires Escape to retire it before release. */
async function runEscapeCancellation(page: Page, consoleLines: readonly string[], row: ProbeCase): Promise<CaseResult> {
  assert.ok(row.delta && row.cancelKey && row.forbiddenAction);
  const before = await mountedGeometry(page, consoleLines);
  const target = targetNode(before);
  const cursor = await actionCursor(page);
  await drag(page, target.point, row.delta, "left", false);
  await page.keyboard.press(row.cancelKey);
  await page.mouse.up({ button: "left" });
  await page.waitForTimeout(500);
  const actions = await actionsAfter(page, cursor);
  assert.ok(actions.some(({ action }) => action === "clearSelection"), "Escape crosses the app-owned cancellation route");
  assert.ok(actions.every(({ action }) => action !== row.forbiddenAction), "cancelled drag publishes no graph edit");
  const after = await mountedGeometry(page, consoleLines);
  const displacement = worldDisplacement(before, after, target.id) ?? nodeDisplacement(before, after, target.id);
  assert.ok(displacement <= contract.limits.cancelTolerancePixels, `cancelled ${target.id} displacement ${displacement} stays within tolerance`);
  return { id: row.id, status: "passed", target: target.id, displacement, actions, after: await capture(page, `${row.id}-after`) };
}

/** 🫳️ Commits one node drag and proves both the move action and persisted geometry. */
async function runCommittedDrag(page: Page, consoleLines: readonly string[], row: ProbeCase): Promise<CaseResult> {
  assert.ok(row.delta && row.expectedAction && row.operation);
  const before = await mountedGeometry(page, consoleLines);
  const target = targetNode(before);
  const cursor = await actionCursor(page);
  await drag(page, target.point, row.delta);
  await waitForAction(page, cursor, row.expectedAction);
  const after = await waitUntil(
    () => mountedGeometry(page, consoleLines),
    (value) => (worldDisplacement(before, value, target.id) ?? nodeDisplacement(before, value, target.id)) >= contract.limits.minimumCommittedMovementPixels,
    `${target.id} committed movement`,
  );
  await page.waitForTimeout(350);
  const actions = await actionsAfter(page, cursor);
  const edits = actions.filter(({ action }) => action === row.expectedAction);
  assert.equal(edits.length, 1, "one released drag publishes exactly one graph edit");
  const operations = actionArgs(edits[0]).operations;
  assert.ok(Array.isArray(operations) && operations.length === 1 && operations[0].operation === row.operation && operations[0].nodeId === target.id, `move operation addresses ${target.id}`);
  const displacement = worldDisplacement(before, after, target.id) ?? nodeDisplacement(before, after, target.id);
  return { id: row.id, status: "passed", target: target.id, displacement, actions, after: await capture(page, `${row.id}-after`) };
}

/** 🔍️ Applies physical wheel zoom and requires live geometry plus persisted viewport change. */
async function runWheel(page: Page, consoleLines: readonly string[], row: ProbeCase): Promise<CaseResult> {
  assert.ok(row.deltaY && row.expectedAction);
  const before = await mountedGeometry(page, consoleLines);
  const target = targetNode(before);
  const cursor = await actionCursor(page);
  await page.mouse.move(before.surface.x + before.surface.width / 2, before.surface.y + before.surface.height / 2);
  await page.mouse.wheel(0, row.deltaY);
  const rows = await waitForAction(page, cursor, row.expectedAction);
  const viewportValue = viewportFrom(rows);
  assert.ok(viewportValue && viewportValue.zoom > 0, "wheel publishes a finite positive zoom");
  const after = await waitUntil(
    () => mountedGeometry(page, consoleLines),
    (value) => nodeDisplacement(before, value, target.id) > contract.limits.geometryEpsilonPixels || Math.abs((value.rows.find((entry) => entry.id === target.id)?.rect.width ?? 0) - target.rect.width) > contract.limits.geometryEpsilonPixels,
    "wheel live geometry change",
  );
  assert.ok((await actionsAfter(page, cursor)).every(({ action }) => action !== "nodeGraphEdit"), "wheel does not mutate graph content");
  return { id: row.id, status: "passed", target: target.id, viewport: viewportValue, movement: nodeDisplacement(before, after, target.id), actions: rows, after: await capture(page, `${row.id}-after`) };
}

/** 🖐️ Applies middle-button camera pan and requires translated geometry without a graph edit. */
async function runPan(page: Page, consoleLines: readonly string[], row: ProbeCase): Promise<CaseResult> {
  assert.ok(row.delta && row.button === "middle" && row.expectedAction && row.forbiddenAction);
  const before = await mountedGeometry(page, consoleLines);
  const start = { x: before.surface.x + before.surface.width - contract.limits.surfaceInsetPixels, y: before.surface.y + before.surface.height - contract.limits.surfaceInsetPixels };
  const cursor = await actionCursor(page);
  await drag(page, start, row.delta, "middle");
  const rows = await waitForAction(page, cursor, row.expectedAction);
  const after = await waitUntil(
    () => mountedGeometry(page, consoleLines),
    (value) => before.rows.some((entry) => value.rows.some((next) => next.id === entry.id && Math.hypot(next.point.x - entry.point.x, next.point.y - entry.point.y) >= contract.limits.minimumCommittedMovementPixels)),
    "middle pan live geometry translation",
  );
  const actions = await actionsAfter(page, cursor);
  assert.ok(actions.every(({ action }) => action !== row.forbiddenAction), "camera pan does not mutate node layout");
  assert.ok(viewportFrom(rows), "middle pan publishes a finite viewport");
  return { id: row.id, status: "passed", viewport: viewportFrom(rows), movedNodes: before.rows.filter((entry) => nodeDisplacement(before, after, entry.id) >= contract.limits.minimumCommittedMovementPixels).map(({ id }) => id), actions, after: await capture(page, `${row.id}-after`) };
}

/** 🧪️ Runs the language-neutral schema plus a real Chromium trusted-event oracle. */
async function testContract(): Promise<void> {
  const flowSource = readFileSync(join(repoRoot, "✏️s", "🔌️plugins", "🌊️flow", "🗿️artifacts", "🌊️flow", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "✏️editor", "🎭️modes", "✏️edit", "🪟️windows", "🌊️main", "🦀️.rs"), "utf8");
  const graphSource = readFileSync(join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "📺️renderer", "🧑‍🎨engine", "🧱️elements", "🕸️NodeGraph", "🟦️.tsx"), "utf8");
  for (const token of [`FLOW_PLAY_WINDOW_MAIN: &str = "${contract.target.windowId}"`, `FLOW_PLAY_SURFACE_MAIN: &str = "${contract.target.surfaceId}"`, "editable: Some(true)"]) assert.ok(flowSource.includes(token), `Flow source carries ${token}`);
  for (const token of ["__semioFlowGraphProbe", "onPointerCancel", "onWheel", "flowGestureIsCameraPan", "nodeGraphActions.clearSelection"]) assert.ok(graphSource.includes(token), `React NodeGraph source carries ${token}`);
  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext({ viewport: { width: 480, height: 320 } });
  try {
    const page = await context.newPage();
    await page.setContent(`<div id="surface" tabindex="0" style="width:320px;height:220px"></div><script>
      globalThis.receipts=[];
      const surface=document.querySelector('#surface');
      for(const type of ['pointerdown','pointermove','pointerup','wheel','keydown']) surface.addEventListener(type,event=>globalThis.receipts.push({type,isTrusted:event.isTrusted,button:event.button,buttons:event.buttons,key:event.key,deltaY:event.deltaY}));
    </script>`);
    const surface = page.locator("#surface");
    const box = await surface.boundingBox();
    assert.ok(box);
    await surface.focus();
    await page.mouse.move(box.x + 40, box.y + 40);
    await page.mouse.down();
    await page.mouse.move(box.x + 88, box.y + 72, { steps: 3 });
    await page.keyboard.press("Escape");
    await page.mouse.up();
    await page.mouse.move(box.x + 200, box.y + 140);
    await page.mouse.down({ button: "middle" });
    await page.mouse.move(box.x + 240, box.y + 170, { steps: 2 });
    await page.mouse.up({ button: "middle" });
    await page.mouse.wheel(0, contract.cases.find(({ id }) => id === "wheel-zoom")!.deltaY!);
    const receipts = await page.evaluate(() => (globalThis as any).receipts);
    assert.ok(receipts.length >= 10 && receipts.every((entry: any) => entry.isTrusted === true), "Chromium supplies trusted physical input");
    assert.ok(receipts.some((entry: any) => entry.type === "keydown" && entry.key === "Escape"));
    assert.ok(receipts.some((entry: any) => entry.type === "pointerdown" && entry.button === 1));
    assert.ok(receipts.some((entry: any) => entry.type === "wheel" && entry.deltaY < 0));
    mkdirSync(outputRoot, { recursive: true });
    writeFileSync(join(outputRoot, "contract.json"), `${JSON.stringify({ fixture: contract, chromiumReceipts: receipts }, null, 2)}\n`);
  } finally {
    await context.close();
    await browser.close();
  }
  process.stdout.write("[DEBUG] Flow NodeGraph fixture/schema/source/Chromium oracle passed\n");
}

/** 🧭️ Opens a freshly activated renderer and publishes read-only calibration evidence. */
async function preparePage(context: BrowserContext): Promise<{ page: Page; consoleLines: string[] }> {
  const page = await context.newPage();
  const consoleLines: string[] = [];
  page.on("console", (message) => consoleLines.push(`${message.type()} ${message.text()}`));
  page.on("pageerror", (error) => consoleLines.push(`pageerror ${String(error)}`));
  await page.addInitScript(() => localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"));
  await page.goto(url, { waitUntil: "domcontentloaded" });
  await waitForBoot(page);
  await dismissIntroduction(page);
  return { page, consoleLines };
}

/** 🔎️ Records whether the exact specimen is mounted; this command never reports acceptance. */
async function discover(): Promise<void> {
  const browser = await chromium.launch({ headless: process.env.SEMIO_SURFACE_HEADED !== "1" });
  const context = await browser.newContext({ viewport, deviceScaleFactor: 1 });
  try {
    const { page, consoleLines } = await preparePage(context);
    const mounted = await mountedGeometry(page, consoleLines);
    const evidence = { renderer, url: page.url(), target: contract.target, geometry: mounted, actions: await actionRows(page) };
    writeFileSync(join(outputDirectory, "discovery.json"), `${JSON.stringify(evidence, null, 2)}\n`);
    writeFileSync(join(outputDirectory, "console.txt"), `${consoleLines.join("\n")}\n`);
    await page.screenshot({ path: join(outputDirectory, "discovery.png") });
    process.stdout.write(`[DEBUG] Flow discovery mounted ${mounted.rows.length} nodes; no acceptance claimed\n`);
  } finally {
    await context.close();
    await browser.close();
  }
}

/** 🎯️ Executes all physical cases, preserves every failure artifact, and fails closed. */
async function run(): Promise<void> {
  const browser = await chromium.launch({ headless: process.env.SEMIO_SURFACE_HEADED !== "1" });
  const context = await browser.newContext({ viewport, deviceScaleFactor: 1 });
  const results: CaseResult[] = [];
  let page: Page | null = null;
  let consoleLines: string[] = [];
  try {
    ({ page, consoleLines } = await preparePage(context));
    await mountedGeometry(page, consoleLines);
    for (const row of contract.cases) {
      try {
        const result = row.id === "select-and-escape"
          ? await runSelection(page, consoleLines, row)
          : row.id === "escape-cancels-node-drag"
            ? await runEscapeCancellation(page, consoleLines, row)
            : row.id === "commit-node-drag"
              ? await runCommittedDrag(page, consoleLines, row)
              : row.id === "wheel-zoom"
                ? await runWheel(page, consoleLines, row)
                : await runPan(page, consoleLines, row);
        results.push(result);
      } catch (error) {
        const failure = error instanceof Error ? `${error.name}: ${error.message}` : String(error);
        const file = `${row.id}-failure.png`;
        await page.screenshot({ path: join(outputDirectory, file) }).catch(() => {});
        results.push({ id: row.id, status: "failed", error: failure, screenshot: file, actions: await actionRows(page).catch(() => []) });
      }
    }
    const evidence = { renderer, url: page.url(), target: contract.target, results, finalGeometry: await geometry(page, consoleLines), actions: await actionRows(page) };
    writeFileSync(join(outputDirectory, "results.json"), `${JSON.stringify(evidence, null, 2)}\n`);
    writeFileSync(join(outputDirectory, "console.txt"), `${consoleLines.join("\n")}\n`);
    const failures = results.filter(({ status }) => status === "failed");
    assert.equal(failures.length, 0, `${renderer} Flow physical failures: ${failures.map(({ id, error }) => `${id}: ${error}`).join("; ")}`);
    process.stdout.write(`[DEBUG] ${renderer} Flow physical acceptance passed ${results.length}/${results.length}\n`);
  } finally {
    await context.close();
    await browser.close();
  }
}

if (command === "test") await testContract();
else if (command === "discover") await discover();
else if (command === "run") await run();
else throw new Error(`Unknown command ${command}`);
