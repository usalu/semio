/** 🪟️ Trusted physical docking acceptance for React and WGPU hosts. */
import assert from "node:assert/strict";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { chromium, type Browser, type Page } from "playwright";

type Renderer = "react" | "wgpu";
type Rect = [number, number, number, number];
type Side = "left" | "right" | "top" | "bottom";
type DockCase =
  | { id: string; kind: "split"; side: Side; expectedRelation: string; stackCountDelta: number; consequence: string }
  | { id: string; kind: "merge"; targetPosition: "after"; stackCountDelta: number; consequence: string }
  | { id: string; kind: "reorder"; setup: "merge-after"; targetPosition: "before"; stackCountDelta: number; consequence: string }
  | { id: string; kind: "cancel"; mechanism: "Escape"; consequence: string }
  | { id: string; kind: "template"; targetPosition: "after"; windowCountDelta: number; consequence: string };
type Control = { id: string; kind: string; rect: Rect; windowId?: string };
type DockTab = { windowId: string; stackKey: string; rect: Rect; handleRect: Rect; active: boolean };
type DockStack = { key: string; rect: Rect; tabs: DockTab[] };
type Camera = { windowId: string; value: any };
type Snapshot = { controls: Control[]; windows: string[]; stacks: DockStack[]; indicator: Rect | null; dragging: boolean; cameras: Camera[]; generation: number };

const fixturePath = join(import.meta.dir, "🧫️fixtures", "🔣️.json");
const schemaPath = join(import.meta.dir, "🧬️schema", "🔣️.json");
const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as {
  version: number;
  target: { plugin: string; urls: Record<Renderer, string>; viewport: { width: number; height: number }; minimumWindows: number; minimumStacks: number; tabsPerSelectedStack: number; dragThresholdPx: number };
  selectors: Record<string, string>;
  template: { mime: string; displayCategoryId: string; rowSuffix: string; selectionId: string; semantic: { projection: string; direction: number[]; up: number[]; vectorTolerance: number }; neutralPayload: { windowKindId: string; templateId: string } };
  cases: DockCase[];
};

const centre = (rect: Rect): [number, number] => [rect[0] + rect[2] / 2, rect[1] + rect[3] / 2];
const contains = (rect: Rect, point: readonly number[]) => point[0]! >= rect[0] && point[0]! <= rect[0] + rect[2] && point[1]! >= rect[1] && point[1]! <= rect[1] + rect[3];
const area = (rect: Rect) => rect[2] * rect[3];
const rectOf = (value: any): Rect => [Number(value[0]), Number(value[1]), Number(value[2]), Number(value[3])];
const sameSet = (left: readonly string[], right: readonly string[]) => left.length === right.length && [...left].sort().every((value, index) => value === [...right].sort()[index]);
const union = (rects: readonly Rect[]): Rect => {
  const x = Math.min(...rects.map((rect) => rect[0]));
  const y = Math.min(...rects.map((rect) => rect[1]));
  const right = Math.max(...rects.map((rect) => rect[0] + rect[2]));
  const bottom = Math.max(...rects.map((rect) => rect[1] + rect[3]));
  return [x, y, right - x, bottom - y];
};

async function dumpWgpu(page: Page, name: "dumpChrome" | "dumpMeshStats" | "dumpStructure"): Promise<any> {
  return page.evaluate(async (probe) => {
    const beacon = (globalThis as any).semioWgpuIntrospection;
    if (typeof beacon?.[probe] !== "function") return null;
    const raw = await beacon[probe]();
    return raw ? JSON.parse(raw) : null;
  }, name);
}

async function reactSnapshot(page: Page): Promise<Snapshot> {
  return page.evaluate((selectors) => {
    const rect = (element: Element): Rect => {
      const value = element.getBoundingClientRect();
      return [value.x, value.y, value.width, value.height];
    };
    const visible = (element: Element) => {
      const value = element.getBoundingClientRect();
      return value.width > 0 && value.height > 0;
    };
    const controls = [...document.querySelectorAll<HTMLElement>("[id]")]
      .filter(visible)
      .map((element) => ({ id: element.id, kind: element.getAttribute("data-slot") ?? element.getAttribute("role") ?? element.tagName.toLowerCase(), rect: rect(element), windowId: element.closest<HTMLElement>("[data-window-id]")?.dataset.windowId }));
    const stacks = [...document.querySelectorAll<HTMLElement>(selectors.reactStack)].filter(visible).map((stack) => {
      const key = stack.dataset.stackPath ?? "";
      const body = stack.querySelector<HTMLElement>(selectors.reactBody);
      const tabs = [...stack.querySelectorAll<HTMLElement>(selectors.reactTab)].filter(visible).map((tab) => {
        const handle = tab.querySelector<HTMLElement>(selectors.reactHandle);
        if (!handle || !visible(handle)) throw new Error(`React tab ${tab.dataset.windowId} has no physical drag handle`);
        return { windowId: tab.dataset.windowId ?? "", stackKey: key, rect: rect(tab), handleRect: rect(handle), active: tab.dataset.active === "true" };
      });
      if (!body || !visible(body)) throw new Error(`React stack ${key || "root"} has no physical body`);
      return { key, rect: rect(body), tabs };
    });
    const indicator = document.querySelector<HTMLElement>(selectors.reactIndicator);
    const cameras = [...document.querySelectorAll<HTMLElement>("[data-viewport-camera-json][data-window-instance-id]")].map((element) => {
      let value = null;
      try { value = JSON.parse(element.dataset.viewportCameraJson ?? "null"); } catch {}
      return { windowId: element.dataset.windowInstanceId ?? "", value };
    });
    const windows = [...new Set(stacks.flatMap((stack) => stack.tabs.map((tab) => tab.windowId)))].sort();
    return { controls, windows, stacks, indicator: indicator && visible(indicator) ? rect(indicator) : null, dragging: document.querySelector(selectors.reactMode)?.getAttribute("data-dragging") === "true", cameras, generation: 0 };
  }, fixture.selectors);
}

function wgpuTabIdentity(controlId: string, windowIds: readonly string[]): { stackKey: string; windowId: string } | null {
  if (!controlId.startsWith(fixture.selectors.wgpuTabPrefix)) return null;
  const rest = controlId.slice(fixture.selectors.wgpuTabPrefix.length);
  const windowId = [...windowIds].sort((a, b) => b.length - a.length).find((candidate) => rest.endsWith(`.${candidate}`));
  if (!windowId) return null;
  return { stackKey: rest.slice(0, -(windowId.length + 1)), windowId };
}

async function wgpuSnapshot(page: Page): Promise<Snapshot> {
  const [chrome, meshes, structure] = await Promise.all([dumpWgpu(page, "dumpChrome"), dumpWgpu(page, "dumpMeshStats"), dumpWgpu(page, "dumpStructure")]);
  if (!chrome?.hits || !chrome?.surfaces) throw new Error("WGPU host exposes no armed dumpChrome diagnostics");
  const controls: Control[] = chrome.hits.map((hit: any) => ({ id: hit.controlId ?? "", kind: hit.kind ?? "", rect: rectOf(hit.rect), windowId: hit.windowId ?? undefined }));
  const surfaceIds = [...new Set<string>([...(structure?.windowIds ?? []), ...chrome.surfaces.filter((surface: any) => surface.level === "window").map((surface: any) => String(surface.id))])];
  const tabControls = controls.filter((control) => control.kind === "Window").flatMap((control) => {
    const identity = wgpuTabIdentity(control.id, surfaceIds);
    return identity ? [{ control, ...identity }] : [];
  });
  const byStack = new Map<string, DockTab[]>();
  for (const { control, stackKey, windowId } of tabControls) {
    const handle = controls.find((candidate) => candidate.id === `${control.id}${fixture.selectors.wgpuHandleSuffix}`);
    if (!handle) throw new Error(`WGPU tab ${windowId} has no exact .drag hit`);
    const related = controls.filter((candidate) => candidate.id === control.id || candidate.id.startsWith(`${control.id}.`)).map((candidate) => candidate.rect);
    const body = controls.filter((candidate) => candidate.id === windowId && area(candidate.rect) > 0).sort((a, b) => area(b.rect) - area(a.rect))[0];
    const tab = { windowId, stackKey, rect: union(related), handleRect: handle.rect, active: Boolean(body) };
    byStack.set(stackKey, [...(byStack.get(stackKey) ?? []), tab]);
  }
  const stacks: DockStack[] = [...byStack].map(([key, tabs]) => {
    tabs.sort((a, b) => a.rect[0] - b.rect[0] || a.rect[1] - b.rect[1]);
    const body = tabs.flatMap((tab) => controls.filter((control) => control.id === tab.windowId)).sort((a, b) => area(b.rect) - area(a.rect))[0];
    if (!body) throw new Error(`WGPU stack ${key || "root"} has no published physical window body`);
    return { key, rect: body.rect, tabs };
  });
  const cameras = (meshes?.surfaces ?? []).map((surface: any) => ({ windowId: String(surface.surfaceId), value: surface.liveCamera ?? surface.camera ?? null }));
  return { controls, windows: [...new Set(tabControls.map((row) => row.windowId))].sort(), stacks, indicator: null, dragging: false, cameras, generation: Number(chrome.generation ?? 0) };
}

const snapshot = (page: Page, renderer: Renderer) => renderer === "react" ? reactSnapshot(page) : wgpuSnapshot(page);

async function waitFor<T>(read: () => Promise<T>, predicate: (value: T) => boolean, message: string, timeoutMs = 20_000): Promise<T> {
  const deadline = Date.now() + timeoutMs;
  let last: T | undefined;
  do {
    last = await read();
    if (predicate(last)) return last;
    await new Promise((resolve) => setTimeout(resolve, 150));
  } while (Date.now() < deadline);
  throw new Error(`${message}; last=${JSON.stringify(last)}`);
}

async function clickControl(page: Page, renderer: Renderer, candidates: readonly string[]): Promise<string> {
  if (renderer === "react") {
    const hit = await page.evaluate((ids) => {
      for (const id of ids) {
        const element = document.getElementById(id);
        if (!element) continue;
        const rect = element.getBoundingClientRect();
        if (rect.width > 0 && rect.height > 0) return { id, rect: [rect.x, rect.y, rect.width, rect.height] as Rect };
      }
      return null;
    }, candidates);
    if (!hit) throw new Error(`React exposes none of the physical controls: ${candidates.join(", ")}`);
    const point = centre(hit.rect);
    const owner = await page.evaluate(([x, y, id]) => document.elementFromPoint(x, y)?.closest("[id]")?.id === id, [point[0], point[1], hit.id] as [number, number, string]);
    if (!owner) throw new Error(`React control ${hit.id} is physically obstructed`);
    await page.mouse.click(...point);
    return hit.id;
  }
  const state = await wgpuSnapshot(page);
  const control = candidates.flatMap((id) => state.controls.filter((entry) => entry.id === id)).at(0);
  if (!control) throw new Error(`WGPU exposes none of the physical controls: ${candidates.join(", ")}`);
  const point = centre(control.rect);
  const top = [...state.controls].reverse().find((entry) => contains(entry.rect, point));
  if (top?.id !== control.id) throw new Error(`WGPU control ${control.id} is physically covered by ${top?.id ?? "canvas"}`);
  await page.mouse.click(...point);
  return control.id;
}

async function physicalDismissIntroduction(page: Page, renderer: Renderer): Promise<void> {
  if (renderer === "react") {
    const id = await page.evaluate(() => ["ui.introduction.skip", "ui.introduction.close"].find((candidate) => {
      const element = document.getElementById(candidate);
      if (!element) return false;
      const rect = element.getBoundingClientRect();
      return rect.width > 0 && rect.height > 0;
    }) ?? null);
    if (id) await clickControl(page, renderer, [id]);
    return;
  }
  const chrome = await dumpWgpu(page, "dumpChrome");
  const id = (chrome?.hits ?? []).map((hit: any) => hit.controlId).find((candidate: string) => candidate === "ui.introduction.skip" || candidate === "ui.introduction.close");
  if (id) await clickControl(page, renderer, [id]);
}

function selectedPair(state: Snapshot): { source: DockStack; target: DockStack; sourceTab: DockTab; targetTab: DockTab } {
  const stacks = state.stacks
    .filter((stack) => stack.tabs.length === fixture.target.tabsPerSelectedStack && stack.rect[2] > 80 && stack.rect[3] > 80)
    .sort((a, b) => a.rect[0] - b.rect[0] || a.rect[1] - b.rect[1]);
  if (stacks.length < fixture.target.minimumStacks) throw new Error(`Host requires two single-tab physical stacks; observed ${JSON.stringify(state.stacks)}`);
  const source = stacks[0]!;
  const target = stacks[1]!;
  return { source, target, sourceTab: source.tabs[0]!, targetTab: target.tabs[0]! };
}

async function baseline(page: Page, renderer: Renderer): Promise<Snapshot> {
  if (renderer === "wgpu") await waitFor(
    () => dumpWgpu(page, "dumpChrome"),
    value => value?.armed === true && Array.isArray(value.hits) && Array.isArray(value.surfaces),
    "WGPU diagnostics did not finish booting",
    Number(process.env.SEMIO_DOCK_BOOT_MS ?? 180_000),
  );
  const state = await waitFor(
    () => snapshot(page, renderer),
    (value) => value.windows.length >= fixture.target.minimumWindows && value.stacks.length >= fixture.target.minimumStacks,
    `${renderer} did not expose the required desktop dock host`,
    Number(process.env.SEMIO_DOCK_BOOT_MS ?? 180_000),
  );
  const started = Date.now();
  let previous = "";
  let quiet = 0;
  await waitFor(async () => {
    const current = await snapshot(page, renderer);
    const signature = JSON.stringify({ windows: current.windows, topology: topology(current), geometry: geometry(current), cameras: current.cameras });
    quiet = signature === previous ? quiet + 1 : 0;
    previous = signature;
    await page.waitForTimeout(1000);
    return current;
  }, () => Date.now() - started >= 10000 && quiet >= 3, `${renderer} dock never settled before introduction dismissal`, Number(process.env.SEMIO_DOCK_BOOT_MS ?? 180_000));
  await physicalDismissIntroduction(page, renderer);
  const ready = await waitFor(
    () => snapshot(page, renderer),
    (value) => value.windows.length >= fixture.target.minimumWindows && value.stacks.length >= fixture.target.minimumStacks && !value.controls.some(control => control.id.startsWith("ui.introduction.")),
    `${renderer} dock did not settle after introduction dismissal`,
  );
  selectedPair(ready);
  return ready;
}

async function assertHandleOwned(page: Page, renderer: Renderer, tab: DockTab): Promise<void> {
  const point = centre(tab.handleRect);
  if (renderer === "react") {
    const owned = await page.evaluate(([x, y, windowId]) => document.elementFromPoint(x, y)?.closest<HTMLElement>('[data-slot="mode-dock-tab"]')?.dataset.windowId === windowId, [point[0], point[1], tab.windowId] as [number, number, string]);
    assert.equal(owned, true, `React drag handle for ${tab.windowId} is obstructed`);
    return;
  }
  const state = await wgpuSnapshot(page);
  const expected = `${fixture.selectors.wgpuTabPrefix}${tab.stackKey}.${tab.windowId}${fixture.selectors.wgpuHandleSuffix}`;
  const top = [...state.controls].reverse().find((control) => contains(control.rect, point));
  assert.equal(top?.id, expected, `WGPU drag handle for ${tab.windowId} is obstructed by ${top?.id ?? "canvas"}`);
}

async function beginDockDrag(page: Page, renderer: Renderer, tab: DockTab, to: readonly [number, number]): Promise<Snapshot> {
  await assertHandleOwned(page, renderer, tab);
  const from = centre(tab.handleRect);
  await page.mouse.move(...from);
  await page.mouse.down();
  await page.waitForTimeout(100);
  await page.mouse.move(from[0] + fixture.target.dragThresholdPx + 3, from[1] + 2, { steps: 3 });
  await page.mouse.move(to[0], to[1], { steps: 12 });
  return waitFor(() => snapshot(page, renderer), (value) => !value.windows.includes(tab.windowId), `${renderer} did not promote the tab drag for ${tab.windowId}`, 5_000);
}

async function commitDockDrag(page: Page, renderer: Renderer, tab: DockTab, to: readonly [number, number], destination: (state: Snapshot) => readonly [number, number]): Promise<Snapshot> {
  let during = await beginDockDrag(page, renderer, tab, to);
  const target = destination(during);
  await page.mouse.move(target[0], target[1], { steps: 8 });
  await page.waitForTimeout(250);
  during = await snapshot(page, renderer);
  await page.mouse.up();
  await waitFor(() => snapshot(page, renderer), (value) => value.windows.includes(tab.windowId), `${renderer} did not publish the committed tab`, 20_000);
  return during;
}

function splitPoint(rect: Rect, side: Side): [number, number] {
  if (side === "left") return [rect[0] + rect[2] * 0.12, rect[1] + rect[3] * 0.5];
  if (side === "right") return [rect[0] + rect[2] * 0.88, rect[1] + rect[3] * 0.5];
  if (side === "top") return [rect[0] + rect[2] * 0.5, rect[1] + rect[3] * 0.12];
  return [rect[0] + rect[2] * 0.5, rect[1] + rect[3] * 0.88];
}

function tabDropPoint(tab: DockTab, position: "before" | "after"): [number, number] {
  return [position === "before" ? tab.rect[0] + 2 : tab.rect[0] + tab.rect[2] - 2, tab.rect[1] + tab.rect[3] / 2];
}

function stackWith(state: Snapshot, windowId: string): DockStack {
  const stack = state.stacks.find((candidate) => candidate.tabs.some((tab) => tab.windowId === windowId));
  if (!stack) throw new Error(`No stack contains ${windowId}`);
  return stack;
}

function assertWindowsPreserved(before: Snapshot, after: Snapshot): void {
  assert.equal(sameSet(before.windows, after.windows), true, `Window identities changed: before=${before.windows} after=${after.windows}`);
}

function assertSplitRelation(side: Side, source: Rect, target: Rect): void {
  const sourceCentre = centre(source);
  const targetCentre = centre(target);
  if (side === "left") assert.ok(sourceCentre[0] < targetCentre[0], "source did not land left of target");
  if (side === "right") assert.ok(sourceCentre[0] > targetCentre[0], "source did not land right of target");
  if (side === "top") assert.ok(sourceCentre[1] < targetCentre[1], "source did not land above target");
  if (side === "bottom") assert.ok(sourceCentre[1] > targetCentre[1], "source did not land below target");
}

function topology(state: Snapshot): unknown {
  return state.stacks.map((stack) => ({ key: stack.key, tabs: stack.tabs.map((tab) => tab.windowId), active: stack.tabs.find((tab) => tab.active)?.windowId ?? null })).sort((a, b) => a.key.localeCompare(b.key));
}

function geometry(state: Snapshot): Record<string, Rect> {
  return Object.fromEntries(state.stacks.flatMap((stack) => stack.tabs.map((tab) => [tab.windowId, stack.rect])));
}

function assertGeometryNear(before: Record<string, Rect>, after: Record<string, Rect>, tolerance = 2): void {
  assert.deepEqual(Object.keys(after).sort(), Object.keys(before).sort());
  for (const [id, expected] of Object.entries(before)) {
    const actual = after[id]!;
    expected.forEach((value, index) => assert.ok(Math.abs(value - actual[index]!) <= tolerance, `${id} geometry[${index}] changed from ${value} to ${actual[index]}`));
  }
}

async function runSplit(page: Page, renderer: Renderer, authored: Extract<DockCase, { kind: "split" }>, before: Snapshot): Promise<unknown> {
  const { source, target, sourceTab, targetTab } = selectedPair(before);
  const point = splitPoint(target.rect, authored.side);
  const during = await commitDockDrag(page, renderer, sourceTab, point, state => splitPoint(stackWith(state, targetTab.windowId).rect, authored.side));
  if (renderer === "react") assert.ok(during.indicator, `${authored.id} exposed no React split preview`);
  const after = await waitFor(() => snapshot(page, renderer), (value) => value.stacks.length === before.stacks.length + authored.stackCountDelta, `${authored.id} did not settle to the expected stack count`);
  assertWindowsPreserved(before, after);
  const sourceAfter = stackWith(after, sourceTab.windowId);
  const targetAfter = stackWith(after, targetTab.windowId);
  assert.notEqual(sourceAfter.key, targetAfter.key, `${authored.id} merged instead of splitting`);
  assert.deepEqual(sourceAfter.tabs.map((tab) => tab.windowId), [sourceTab.windowId]);
  assert.equal(sourceAfter.tabs.some((tab) => tab.windowId === sourceTab.windowId && tab.active), true);
  assertSplitRelation(authored.side, sourceAfter.rect, targetAfter.rect);
  return { id: authored.id, status: "passed", source: sourceTab.windowId, target: targetTab.windowId, point, duringIndicator: during.indicator, before: topology(before), after: topology(after), geometry: geometry(after) };
}

async function runMerge(page: Page, renderer: Renderer, authored: Extract<DockCase, { kind: "merge" }>, before: Snapshot): Promise<unknown> {
  const { sourceTab, targetTab } = selectedPair(before);
  const during = await commitDockDrag(page, renderer, sourceTab, tabDropPoint(targetTab, authored.targetPosition), state => tabDropPoint(stackWith(state, targetTab.windowId).tabs.find(tab => tab.windowId === targetTab.windowId)!, authored.targetPosition));
  const after = await waitFor(() => snapshot(page, renderer), (value) => value.stacks.length === before.stacks.length + authored.stackCountDelta, "tab merge did not collapse one stack");
  assertWindowsPreserved(before, after);
  const merged = stackWith(after, sourceTab.windowId);
  assert.equal(merged.key, stackWith(after, targetTab.windowId).key);
  const order = merged.tabs.map((tab) => tab.windowId);
  assert.ok(order.indexOf(targetTab.windowId) < order.indexOf(sourceTab.windowId), `merge order is ${order}`);
  assert.equal(merged.tabs.find((tab) => tab.windowId === sourceTab.windowId)?.active, true);
  return { id: authored.id, status: "passed", source: sourceTab.windowId, target: targetTab.windowId, duringDockOut: !during.windows.includes(sourceTab.windowId), order, before: topology(before), after: topology(after) };
}

async function runReorder(page: Page, renderer: Renderer, authored: Extract<DockCase, { kind: "reorder" }>, before: Snapshot): Promise<unknown> {
  const { sourceTab, targetTab } = selectedPair(before);
  await commitDockDrag(page, renderer, sourceTab, tabDropPoint(targetTab, "after"), state => tabDropPoint(stackWith(state, targetTab.windowId).tabs.find(tab => tab.windowId === targetTab.windowId)!, "after"));
  const merged = await waitFor(() => snapshot(page, renderer), (value) => value.stacks.length === before.stacks.length - 1, "reorder setup did not merge tabs");
  const sourceMerged = stackWith(merged, sourceTab.windowId).tabs.find((tab) => tab.windowId === sourceTab.windowId)!;
  const targetMerged = stackWith(merged, targetTab.windowId).tabs.find((tab) => tab.windowId === targetTab.windowId)!;
  await commitDockDrag(page, renderer, sourceMerged, tabDropPoint(targetMerged, authored.targetPosition), state => tabDropPoint(stackWith(state, targetTab.windowId).tabs.find(tab => tab.windowId === targetTab.windowId)!, authored.targetPosition));
  const after = await waitFor(() => snapshot(page, renderer), (value) => {
    const stack = value.stacks.find((candidate) => candidate.tabs.some((tab) => tab.windowId === sourceTab.windowId));
    if (!stack) return false;
    const order = stack.tabs.map((tab) => tab.windowId);
    return order.indexOf(sourceTab.windowId) < order.indexOf(targetTab.windowId);
  }, "tab reorder did not place the source before the target");
  assertWindowsPreserved(before, after);
  assert.equal(after.stacks.length, before.stacks.length + authored.stackCountDelta);
  const order = stackWith(after, sourceTab.windowId).tabs.map((tab) => tab.windowId);
  return { id: authored.id, status: "passed", source: sourceTab.windowId, target: targetTab.windowId, setupOrder: stackWith(merged, sourceTab.windowId).tabs.map((tab) => tab.windowId), order, before: topology(before), after: topology(after) };
}

async function runCancel(page: Page, renderer: Renderer, authored: Extract<DockCase, { kind: "cancel" }>, before: Snapshot): Promise<unknown> {
  const { sourceTab, target } = selectedPair(before);
  const during = await beginDockDrag(page, renderer, sourceTab, splitPoint(target.rect, "left"));
  assert.equal(during.windows.includes(sourceTab.windowId), false, "cancel case never entered promoted drag state");
  await page.keyboard.press(authored.mechanism);
  const restored = await waitFor(() => snapshot(page, renderer), (value) => value.windows.includes(sourceTab.windowId) && !value.indicator && !value.dragging, "Escape did not retire the dock drag");
  await page.mouse.up();
  const after = await waitFor(() => snapshot(page, renderer), (value) => JSON.stringify(topology(value)) === JSON.stringify(topology(restored)), "post-cancel pointer release changed topology");
  assert.deepEqual(topology(after), topology(before));
  assertGeometryNear(geometry(before), geometry(after));
  return { id: authored.id, status: "passed", source: sourceTab.windowId, mechanism: authored.mechanism, promoted: true, restored: topology(after), geometry: geometry(after) };
}

async function visibleControls(page: Page, renderer: Renderer): Promise<Control[]> {
  if (renderer === "wgpu") return (await wgpuSnapshot(page)).controls;
  return page.evaluate(() => [...document.querySelectorAll<HTMLElement>("[id]")].flatMap((element) => {
    const rect = element.getBoundingClientRect();
    return rect.width > 0 && rect.height > 0 ? [{ id: element.id, kind: element.getAttribute("data-slot") ?? element.getAttribute("role") ?? element.tagName.toLowerCase(), rect: [rect.x, rect.y, rect.width, rect.height] as Rect }] : [];
  }));
}

async function openTemplateSource(page: Page, renderer: Renderer): Promise<{ id: string; handleRect: Rect; payload: any | null }> {
  await clickControl(page, renderer, [fixture.template.displayCategoryId]);
  let controls = await waitFor(() => visibleControls(page, renderer), (rows) => rows.some((row) => row.id.includes("framework.display.windows.")), "Display category exposed no window-template tree");
  const exactSuffix = (id: string, suffix: string) => id === suffix || id.endsWith(`.${suffix}`) || id.endsWith(suffix);
  let kind = controls.find((row) => row.id.includes("framework.display.windows.") && row.id.endsWith(".kind") && !row.id.startsWith("tree.drag.transfer."));
  if (!kind) {
    const section = controls.find((row) => /framework\.display\.windows\.[^.]+$/.test(row.id) && !row.id.endsWith(".panel"));
    if (!section) throw new Error("Display category exposes no expandable window-kind section");
    await clickControl(page, renderer, [section.id]);
    controls = await waitFor(() => visibleControls(page, renderer), (rows) => rows.some((row) => row.id.endsWith(".kind")), "Window-kind section did not expose its template rows");
    kind = controls.find((row) => row.id.includes("framework.display.windows.") && row.id.endsWith(".kind") && !row.id.startsWith("tree.drag.transfer."));
  }
  if (!kind) throw new Error("Display tree has no physical window-kind row");
  const marker = kind.id.indexOf("framework.display.windows.");
  const base = kind.id.slice(marker).slice(0, -".kind".length);
  const parallelId = `${base}.projection.parallel`;
  const rowId = `${base}${fixture.template.rowSuffix}`;
  if (!controls.some((row) => exactSuffix(row.id, rowId))) {
    const parallel = controls.find((row) => exactSuffix(row.id, parallelId));
    if (!parallel) throw new Error(`Display tree exposes no ${parallelId} branch`);
    if (renderer === "wgpu") {
      controls = await waitFor(() => visibleControls(page, renderer), rows => rows.some(row => row.id.startsWith("tree.chevron.") && row.id.endsWith(parallelId)), `Display branch ${parallelId} exposes no physical disclosure`);
      const chevron = controls.find(row => row.id.startsWith("tree.chevron.") && row.id.endsWith(parallelId))!;
      await clickControl(page, renderer, [chevron.id]);
    } else {
      const disclosure = await page.evaluate(id => {
        const row = document.getElementById(id);
        const button = row?.querySelector<HTMLElement>('[data-slot="tree-gutter"] button');
        if (!button) return null;
        const rect = button.getBoundingClientRect();
        const point = [rect.x + rect.width / 2, rect.y + rect.height / 2];
        return button.contains(document.elementFromPoint(point[0]!, point[1]!)) ? point : null;
      }, parallel.id);
      if (!disclosure) throw new Error(`Display branch ${parallel.id} exposes no unobstructed physical disclosure`);
      await page.mouse.click(disclosure[0]!, disclosure[1]!);
    }
    controls = await waitFor(() => visibleControls(page, renderer), (rows) => rows.some((row) => exactSuffix(row.id, rowId)), `Display branch did not expose ${rowId}`);
  }
  if (renderer === "wgpu") {
    const handle = controls.find((row) => row.id === `tree.drag.transfer.${rowId}`);
    if (!handle) throw new Error(`WGPU template ${rowId} exposes no exact transfer hit`);
    return { id: rowId, handleRect: handle.rect, payload: null };
  }
  const found = await page.evaluate(({ rowId, mime }) => {
    const row = [document.getElementById(rowId), document.getElementById(`tree.label.${rowId}`)].find(Boolean) as HTMLElement | undefined;
    const handle = row?.querySelector<HTMLElement>('[data-slot="drag-handle"][data-drag-role="transfer"], [data-slot="drag-handle"]');
    const payloadOwner = row?.closest<HTMLElement>("[data-drag-payload]") ?? row;
    if (!row || !handle) return null;
    const rect = handle.getBoundingClientRect();
    let payload = null;
    if (payloadOwner?.dataset.dragMime === mime && payloadOwner.dataset.dragPayload) {
      try { payload = JSON.parse(payloadOwner.dataset.dragPayload); } catch {}
    }
    return { rect: [rect.x, rect.y, rect.width, rect.height] as Rect, payload };
  }, { rowId, mime: fixture.template.mime });
  if (!found) throw new Error(`React template ${rowId} exposes no transfer handle`);
  return { id: rowId, handleRect: found.rect, payload: found.payload };
}

function projectionOf(camera: any): string | null {
  const value = camera?.projection;
  if (typeof value === "string") return value.toLowerCase();
  const kind = value?.mode?.kind ?? value?.kind;
  return typeof kind === "string" ? kind.toLowerCase() : null;
}

function normalized(vector: readonly number[]): number[] {
  const length = Math.hypot(...vector);
  return length > 0 ? vector.map((value) => value / length) : vector.map(() => 0);
}

function directionOf(camera: any): number[] | null {
  if (Array.isArray(camera?.direction)) return normalized(camera.direction);
  if (!Array.isArray(camera?.position) || !Array.isArray(camera?.target)) return null;
  return normalized(camera.position.map((value: number, index: number) => value - camera.target[index]));
}

function assertVector(actual: number[] | null, expected: number[], tolerance: number, name: string): void {
  assert.ok(actual && actual.length === expected.length, `${name} is unavailable`);
  expected.forEach((value, index) => assert.ok(Math.abs(value - actual![index]!) <= tolerance, `${name}[${index}] expected ${value}, got ${actual![index]}`));
}

async function runTemplate(page: Page, renderer: Renderer, authored: Extract<DockCase, { kind: "template" }>, before: Snapshot): Promise<unknown> {
  const source = await openTemplateSource(page, renderer);
  if (renderer === "react") {
    assert.equal(typeof source.payload?.windowKindId, "string", "React template payload omits windowKindId");
    const encoded = source.payload?.templateId;
    assert.equal(typeof encoded, "string", "React template payload omits templateId");
    const spec = encoded.startsWith("world-projection:") ? JSON.parse(encoded.slice("world-projection:".length)) : null;
    assert.equal(spec?.mode?.kind ?? encoded, fixture.template.selectionId);
    assert.equal(spec?.orientation?.view, "plan");
  }
  const current = await snapshot(page, renderer);
  const oldWindows = new Set(current.windows);
  const targetId = selectedPair(before).target.tabs[0]!.windowId;
  const target = stackWith(current, targetId).tabs.find((tab) => tab.windowId === targetId)!;
  const from = centre(source.handleRect);
  const to = tabDropPoint(target, authored.targetPosition);
  await page.mouse.move(...from);
  await page.mouse.down();
  await page.waitForTimeout(100);
  await page.mouse.move(from[0] + fixture.target.dragThresholdPx + 3, from[1] + 2, { steps: 3 });
  await page.mouse.move(...to, { steps: 14 });
  await page.mouse.up();
  const after = await waitFor(() => snapshot(page, renderer), (value) => value.windows.filter((id) => !oldWindows.has(id)).length === authored.windowCountDelta, "Template transfer did not create exactly one window", 30_000);
  const created = after.windows.find((id) => !oldWindows.has(id));
  if (!created) throw new Error("Template transfer published no new window identity");
  assert.equal(stackWith(after, created).key, stackWith(after, targetId).key, "Template did not land in the target tab stack");
  const configured = await waitFor(() => snapshot(page, renderer), (value) => value.cameras.some((camera) => camera.windowId === created && projectionOf(camera.value) === fixture.template.semantic.projection), "Created template window never exposed its orthographic camera", 30_000);
  const camera = configured.cameras.find((entry) => entry.windowId === created)!.value;
  assert.equal(projectionOf(camera), fixture.template.semantic.projection);
  assertVector(directionOf(camera), fixture.template.semantic.direction, fixture.template.semantic.vectorTolerance, "camera direction");
  assertVector(Array.isArray(camera?.up) ? normalized(camera.up) : null, fixture.template.semantic.up, fixture.template.semantic.vectorTolerance, "camera up");
  return { id: authored.id, status: "passed", source: source.id, payload: source.payload, target: targetId, created, projection: projectionOf(camera), direction: directionOf(camera), up: camera.up, after: topology(configured) };
}

async function runCase(browser: Browser, renderer: Renderer, url: string, authored: DockCase, output: string): Promise<unknown> {
  const context = await browser.newContext({ viewport: fixture.target.viewport, deviceScaleFactor: 1 });
  const page = await context.newPage();
  const consoleLines: string[] = [];
  const faults: string[] = [];
  page.on("console", message => {
    const text = message.text();
    consoleLines.push(`[${message.type()}] ${text}`);
    if (/wgpu renderer fault:|RuntimeError:.*(?:unreachable|out of bounds)/i.test(text)) faults.push(text);
  });
  page.on("pageerror", error => { consoleLines.push(`[pageerror] ${error.message}`); faults.push(error.message); });
  await page.addInitScript(() => localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"));
  try {
    await page.goto(url, { waitUntil: "domcontentloaded" });
    const before = await baseline(page, renderer);
    let result: unknown;
    if (authored.kind === "split") result = await runSplit(page, renderer, authored, before);
    else if (authored.kind === "merge") result = await runMerge(page, renderer, authored, before);
    else if (authored.kind === "reorder") result = await runReorder(page, renderer, authored, before);
    else if (authored.kind === "cancel") result = await runCancel(page, renderer, authored, before);
    else result = await runTemplate(page, renderer, authored, before);
    if (faults.length) throw new Error(faults.join("; "));
    await page.screenshot({ path: join(output, `${authored.id}.png`) });
    return result;
  } catch (error) {
    await page.screenshot({ path: join(output, `${authored.id}-failed.png`), timeout: 10000 }).catch(() => {});
    await snapshot(page, renderer).then(state => writeFileSync(join(output, `${authored.id}-failed-state.json`), JSON.stringify(state, null, 2))).catch(() => {});
    return { id: authored.id, status: "failed", error: String(error), faults };
  } finally {
    writeFileSync(join(output, `${authored.id}-console.txt`), consoleLines.join("\n"));
    await context.close();
  }
}

async function run(renderer: Renderer, url: string): Promise<void> {
  const output = join(import.meta.dir, "..", "🗑️generated", process.env.SEMIO_DOCK_OUT ?? "dock-interactions", renderer);
  mkdirSync(output, { recursive: true });
  const browser = await chromium.launch({ headless: process.env.SEMIO_DOCK_HEADED !== "1", args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", ...(process.platform === "darwin" ? ["--use-angle=metal"] : [])] });
  const receipt = { version: fixture.version, renderer, url, startedAt: new Date().toISOString(), cases: [] as unknown[] };
  try {
    const only = (process.env.SEMIO_DOCK_CASES ?? "").split(",").filter(Boolean);
    for (const id of only) assert.ok(fixture.cases.some(entry => entry.id === id), `Unknown Dock case ${id}`);
    for (const authored of fixture.cases.filter(entry => only.length === 0 || only.includes(entry.id))) {
      const result = await runCase(browser, renderer, url, authored, output);
      receipt.cases.push(result);
      writeFileSync(join(output, "receipt.json"), JSON.stringify(receipt, null, 2));
      console.log(`[DEBUG] ${renderer} ${authored.id} ${(result as { status: string }).status}`);
    }
  } finally {
    await browser.close();
  }
  process.stdout.write(`${JSON.stringify(receipt)}\n`);
  if (receipt.cases.some(result => (result as { status: string }).status === "failed")) process.exitCode = 1;
}

function neutralSplit(side: Side): { source: Rect; target: Rect; relation: string } {
  const target: Rect = [100, 100, 400, 300];
  if (side === "left") return { source: [100, 100, 200, 300], target: [300, 100, 200, 300], relation: "source-left-of-target" };
  if (side === "right") return { source: [300, 100, 200, 300], target: [100, 100, 200, 300], relation: "source-right-of-target" };
  if (side === "top") return { source: [100, 100, 400, 150], target: [100, 250, 400, 150], relation: "source-above-target" };
  return { source: [100, 250, 400, 150], target: [100, 100, 400, 150], relation: "source-below-target" };
}

async function testContract(): Promise<void> {
  const schema = JSON.parse(readFileSync(schemaPath, "utf8"));
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const validate = new Ajv2020({ allErrors: true, strict: true }).compile(schema);
  assert.equal(validate(fixture), true, JSON.stringify(validate.errors));
  assert.deepEqual(fixture.cases.map((entry) => entry.id), ["split-left", "split-right", "split-top", "split-bottom", "tab-merge", "tab-reorder", "escape-cancel", "template-configuration"]);
  for (const authored of fixture.cases.filter((entry): entry is Extract<DockCase, { kind: "split" }> => entry.kind === "split")) {
    const neutral = neutralSplit(authored.side);
    assert.equal(neutral.relation, authored.expectedRelation);
    assertSplitRelation(authored.side, neutral.source, neutral.target);
    assert.equal(contains([100, 100, 400, 300], splitPoint([100, 100, 400, 300], authored.side)), true);
  }

  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage({ viewport: { width: 520, height: 240 } });
  try {
    await page.setContent('<div id="source" draggable="true" style="position:absolute;left:20px;top:20px;width:50px;height:50px"></div><div id="drop" style="position:absolute;left:220px;top:30px;width:140px;height:120px"></div><div id="pointer" style="position:absolute;left:20px;top:170px;width:50px;height:50px"></div>');
    await page.evaluate(({ mime, payload }) => {
      const events: any[] = [];
      const source = document.getElementById("source")!;
      const drop = document.getElementById("drop")!;
      source.addEventListener("dragstart", (event) => {
        const drag = event as DragEvent;
        drag.dataTransfer!.setData(mime, JSON.stringify(payload));
        events.push({ type: event.type, trusted: event.isTrusted });
      });
      for (const type of ["dragover", "drop"]) drop.addEventListener(type, (event) => {
        const drag = event as DragEvent;
        event.preventDefault();
        events.push({ type, trusted: event.isTrusted, types: [...drag.dataTransfer!.types], payload: type === "drop" ? drag.dataTransfer!.getData(mime) : "" });
      });
      let active = false;
      let commits = 0;
      const pointer = document.getElementById("pointer")!;
      pointer.addEventListener("pointerdown", (event) => { active = true; events.push({ type: event.type, trusted: event.isTrusted }); });
      document.addEventListener("pointermove", (event) => { if (active) events.push({ type: event.type, trusted: event.isTrusted }); });
      document.addEventListener("keydown", (event) => { if (event.key === "Escape" && active) { active = false; events.push({ type: event.type, key: event.key, trusted: event.isTrusted }); } });
      document.addEventListener("pointerup", (event) => { if (active) commits += 1; active = false; events.push({ type: event.type, trusted: event.isTrusted }); });
      (globalThis as any).__dockOracle = { events, read: () => ({ events, commits, active }) };
    }, { mime: fixture.template.mime, payload: fixture.template.neutralPayload });
    const source = (await page.locator("#source").boundingBox())!;
    const drop = (await page.locator("#drop").boundingBox())!;
    await page.mouse.move(...centre([source.x, source.y, source.width, source.height]));
    await page.mouse.down();
    await page.mouse.move(...centre([drop.x, drop.y, drop.width, drop.height]), { steps: 12 });
    await page.mouse.up();
    const pointer = (await page.locator("#pointer").boundingBox())!;
    await page.mouse.move(...centre([pointer.x, pointer.y, pointer.width, pointer.height]));
    await page.mouse.down();
    await page.mouse.move(pointer.x + pointer.width + 40, pointer.y + pointer.height / 2, { steps: 4 });
    await page.keyboard.press("Escape");
    await page.mouse.up();
    const oracle = await page.evaluate(() => (globalThis as any).__dockOracle.read());
    assert.ok(oracle.events.length > 0 && oracle.events.every((entry: any) => entry.trusted));
    assert.ok(oracle.events.some((entry: any) => entry.type === "dragstart"));
    const dropped = oracle.events.find((entry: any) => entry.type === "drop");
    assert.ok(dropped.types.includes(fixture.template.mime));
    assert.equal(dropped.payload, JSON.stringify(fixture.template.neutralPayload));
    assert.ok(oracle.events.some((entry: any) => entry.type === "keydown" && entry.key === "Escape"));
    assert.equal(oracle.commits, 0);
    assert.equal(oracle.active, false);
    process.stdout.write(`${JSON.stringify({ fixture: "PASS", version: fixture.version, cases: fixture.cases.map((entry) => entry.id), trustedEvents: oracle.events.map((entry: any) => entry.type), templateMime: fixture.template.mime, cancelledCommits: oracle.commits })}\n`);
  } finally {
    await browser.close();
  }
}

const command = process.argv[2] ?? "test";
if (command === "test") await testContract();
else if (command === "run") {
  const renderer = process.argv[3] as Renderer;
  if (renderer !== "react" && renderer !== "wgpu") throw new Error("Usage: 📜️script.ts run <react|wgpu> [url]");
  await run(renderer, process.argv[4] ?? process.env.SEMIO_DOCK_URL ?? fixture.target.urls[renderer]);
} else throw new Error("Usage: 📜️script.ts <test|run> [react|wgpu] [url]");
