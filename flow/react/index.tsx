// #region 🧲Header
/** @emoji 🌊 `@flow/react` — WASM flow renderer + React canvas. */
// #endregion 🧲Header

import React, { useCallback, useEffect, useRef, useState } from "react";
import { ContextMenuController, SelectionMarquee, type ContextMenuItem, type SelectionMarqueeCoverage } from "@ui/react";
import { clearColorResolveCache, resolveColorHex, resolveSemanticColorHex, serializeGraphVelloThemePaletteJson, tokenVar } from "@ui/styling";
import { isDagDrawLodKind, type DagDrawLodKind } from "@dag/react";
import initFlowWasm, { FlowSession, initSync } from "../core/pkg/flow_core.js";

// #region 🔖GpuWasmBridge
if (import.meta.env.VITEST) {
  const { readFileSync } = await import("node:fs");
  const { dirname, join } = await import("node:path");
  const { fileURLToPath } = await import("node:url");
  const reactDir = dirname(fileURLToPath(import.meta.url));
  initSync({ module: readFileSync(join(reactDir, "../core/pkg/flow_core_bg.wasm")) });
  const [
    { initSync: initMathSync },
    { initSync: initTextSync },
    { initSync: initLogicSync },
    { initSync: initDictionarySync },
    { initSync: initListSync },
  ] = await Promise.all([
    import("../modules/math/pkg/flow_module_math.js"),
    import("../modules/text/pkg/flow_module_text.js"),
    import("../modules/logic/pkg/flow_module_logic.js"),
    import("../modules/dictionary/pkg/flow_module_dictionary.js"),
    import("../modules/list/pkg/flow_module_list.js"),
  ]);
  initMathSync({ module: readFileSync(join(reactDir, "../modules/math/pkg/flow_module_math_bg.wasm")) });
  initTextSync({ module: readFileSync(join(reactDir, "../modules/text/pkg/flow_module_text_bg.wasm")) });
  initLogicSync({ module: readFileSync(join(reactDir, "../modules/logic/pkg/flow_module_logic_bg.wasm")) });
  initDictionarySync({ module: readFileSync(join(reactDir, "../modules/dictionary/pkg/flow_module_dictionary_bg.wasm")) });
  initListSync({ module: readFileSync(join(reactDir, "../modules/list/pkg/flow_module_list_bg.wasm")) });
} else {
  await initFlowWasm();
}

export async function ensureFlowWasmLoaded(): Promise<void> {
  await initFlowWasm();
}

export { FlowSession };

export {
  DAG_LOD_MODE_AUTOMATIC,
  dagPlayLodTiers,
  dagLodAutomaticSelectLabel,
  dagLodCanvasProps,
  dagPlayLodTierMenuLabel,
  getDagLodScale,
  isDagDrawLodKind,
  type DagDrawLodKind,
  type DagLodModeKind,
} from "@dag/react";
// #endregion 🔖GpuWasmBridge

// #region 🔖ExtensionHost
export interface FlowModuleVariadicSpecV1 {
  readonly slotKey: string;
  readonly min: number;
  readonly max?: number;
}

export interface FlowModuleNeuronKindV1 {
  readonly id: string;
  readonly module: string;
  readonly name: string;
  readonly abbreviation: string;
  readonly icon: string;
  readonly summary: string;
  readonly inputs: readonly string[];
  readonly outputs: readonly string[];
  readonly variadicInput?: FlowModuleVariadicSpecV1;
  readonly variadicOutput?: FlowModuleVariadicSpecV1;
}

export interface FlowModuleCommandV1 {
  readonly id: string;
  readonly title: string;
}

export interface FlowModuleSettingV1 {
  readonly id: string;
  readonly type: string;
  readonly default: unknown;
  readonly description: string;
}

export interface FlowModuleWidgetV1 {
  readonly kind: string;
  readonly name: string;
  readonly summary: string;
}

export interface FlowModuleManifestV1 {
  readonly schema: "flow.module/v1";
  readonly id: string;
  readonly name: string;
  readonly version: string;
  readonly activationEvents: readonly string[];
  readonly contributes: {
    readonly neuronKinds: readonly FlowModuleNeuronKindV1[];
    readonly widgets: readonly FlowModuleWidgetV1[];
    readonly commands: readonly FlowModuleCommandV1[];
    readonly settings: readonly FlowModuleSettingV1[];
  };
}

export interface FlowExtensionEntry {
  readonly id: string;
  readonly manifest: FlowModuleManifestV1;
  readonly active: boolean;
}

interface FlowModuleGlue {
  readonly manifest: () => string;
  readonly evaluate: (kindId: string, inputJson: string) => string;
  readonly command: (commandId: string, argsJson: string) => string;
  readonly activate: () => void;
  readonly deactivate: () => void;
}

type FlowModulePackage = {
  readonly default?: () => Promise<unknown>;
  readonly manifest: () => string;
  readonly evaluate: (kindId: string, inputJson: string) => string;
  readonly command: (commandId: string, argsJson: string) => string;
  readonly activate: () => void;
  readonly deactivate: () => void;
};

type FlowModuleLoader = () => Promise<FlowModulePackage>;

const FLOW_MODULE_LOADERS: Record<string, FlowModuleLoader> = {
  math: () => import("@flow/module-math"),
  text: () => import("@flow/module-text"),
  logic: () => import("@flow/module-logic"),
  dictionary: () => import("@flow/module-dictionary"),
  list: () => import("@flow/module-list"),
};

export const FLOW_DEFAULT_MODULE_IDS = ["math", "text", "logic", "dictionary", "list"] as const;
export type FlowModuleId = (typeof FLOW_DEFAULT_MODULE_IDS)[number];

export const FLOW_INSTALLED_MODULE_IDS = Object.keys(FLOW_MODULE_LOADERS);

interface ActiveFlowModule {
  readonly glue: FlowModuleGlue;
  readonly manifest: FlowModuleManifestV1;
}

export class FlowExtensionHost {
  private readonly kindToModule = new Map<string, string>();
  private readonly active = new Map<string, ActiveFlowModule>();
  private revision = 0;
  private readonly listeners = new Set<() => void>();

  getRevision(): number {
    return this.revision;
  }

  subscribe(listener: () => void): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  private notify(): void {
    this.revision += 1;
    for (const listener of this.listeners) {
      listener();
    }
  }

  listInstalled(): readonly string[] {
    return FLOW_INSTALLED_MODULE_IDS;
  }

  listEntries(): readonly FlowExtensionEntry[] {
    return FLOW_INSTALLED_MODULE_IDS.map((id) => ({
      id,
      manifest: this.active.get(id)?.manifest ?? this.placeholderManifest(id),
      active: this.active.has(id),
    }));
  }

  isActive(id: string): boolean {
    return this.active.has(id);
  }

  activeCommands(): readonly FlowModuleCommandV1[] {
    const commands: FlowModuleCommandV1[] = [];
    for (const entry of this.active.values()) {
      commands.push(...entry.manifest.contributes.commands);
    }
    return commands;
  }

  async activateDefaults(): Promise<void> {
    for (const id of FLOW_DEFAULT_MODULE_IDS) {
      if (!this.active.has(id)) {
        await this.activate(id);
      }
    }
  }

  async activate(id: string): Promise<void> {
    if (this.active.has(id)) return;
    const loader = FLOW_MODULE_LOADERS[id];
    if (!loader) {
      throw new Error(`unknown flow module: ${id}`);
    }
    const mod = await loader();
    if (typeof mod.default === "function") {
      await mod.default();
    }
    const glue: FlowModuleGlue = {
      manifest: () => mod.manifest(),
      evaluate: (kindId, inputJson) => mod.evaluate(kindId, inputJson),
      command: (commandId, argsJson) => mod.command(commandId, argsJson),
      activate: () => mod.activate(),
      deactivate: () => mod.deactivate(),
    };
    glue.activate();
    const manifest = parseFlowModuleManifest(glue.manifest());
    this.active.set(id, { glue, manifest });
    for (const kind of manifest.contributes.neuronKinds) {
      this.kindToModule.set(kind.id, id);
    }
    console.log(`[DEBUG] flow extension activated: ${id}`);
    this.notify();
  }

  async deactivate(id: string): Promise<void> {
    const entry = this.active.get(id);
    if (!entry) return;
    entry.glue.deactivate();
    for (const kind of entry.manifest.contributes.neuronKinds) {
      this.kindToModule.delete(kind.id);
    }
    this.active.delete(id);
    console.log(`[DEBUG] flow extension deactivated: ${id}`);
    this.notify();
  }

  async setActive(id: string, enabled: boolean): Promise<void> {
    if (enabled) {
      await this.activate(id);
    } else {
      await this.deactivate(id);
    }
  }

  evaluate(kindId: string, inputJson: string): string {
    const moduleId = this.kindToModule.get(kindId);
    if (!moduleId) {
      return JSON.stringify({ error: `no module for kind: ${kindId}` });
    }
    const entry = this.active.get(moduleId);
    if (!entry) {
      return JSON.stringify({ error: `module not active: ${moduleId}` });
    }
    return entry.glue.evaluate(kindId, inputJson);
  }

  executeCommand(commandId: string, argsJson = "{}"): string {
    for (const entry of this.active.values()) {
      if (!entry.manifest.contributes.commands.some((command) => command.id === commandId)) {
        continue;
      }
      return entry.glue.command(commandId, argsJson);
    }
    return JSON.stringify({ error: `unknown command: ${commandId}` });
  }

  catalogueSections(): CatalogueSection[] {
    const sections: CatalogueSection[] = [];
    for (const [id, entry] of this.active) {
      const items = entry.manifest.contributes.neuronKinds.map((kind) => ({
        kind: "neuron",
        neuronKind: kind.id,
        name: kind.name,
        abbreviation: kind.abbreviation,
        icon: kind.icon,
        summary: kind.summary,
      }));
      if (items.length === 0) continue;
      sections.push({ id, title: entry.manifest.name, items });
    }
    return sections;
  }

  catalogueJson(): string {
    return JSON.stringify(this.catalogueSections());
  }

  kindInfosJson(): string {
    const kinds: FlowModuleNeuronKindV1[] = [];
    for (const entry of this.active.values()) {
      kinds.push(...entry.manifest.contributes.neuronKinds);
    }
    return JSON.stringify(kinds);
  }

  private placeholderManifest(id: string): FlowModuleManifestV1 {
    return {
      schema: "flow.module/v1",
      id,
      name: titleizeModuleId(id),
      version: "0.0.0",
      activationEvents: [],
      contributes: { neuronKinds: [], widgets: [], commands: [], settings: [] },
    };
  }
}

export const flowExtensionHost = new FlowExtensionHost();

declare global {
  interface Window {
    __flowExtensionHost?: FlowExtensionHost;
  }
}

if (typeof window !== "undefined") {
  window.__flowExtensionHost = flowExtensionHost;
}

export function parseFlowModuleManifest(json: string): FlowModuleManifestV1 {
  const parsed = JSON.parse(json) as FlowModuleManifestV1;
  if (parsed.schema !== "flow.module/v1") {
    throw new Error(`unsupported module manifest schema: ${parsed.schema}`);
  }
  return parsed;
}

function titleizeModuleId(moduleId: string): string {
  return moduleId.length > 0 ? moduleId[0]!.toUpperCase() + moduleId.slice(1) : moduleId;
}

export function createFlowEvalBridge(host: FlowExtensionHost = flowExtensionHost): (kindId: string, inputJson: string) => string {
  return (kindId, inputJson) => host.evaluate(kindId, inputJson);
}
// #endregion 🔖ExtensionHost

// #region 🔖Fixture
export interface FlowFixtureV1 {
  readonly schema: "flow.fixture/v1";
  readonly camera: { readonly x: number; readonly y: number; readonly zoom: number };
  readonly widgets: readonly FlowWidgetV1[];
  readonly synapses: readonly {
    readonly id: string;
    readonly from: string;
    readonly to: string;
    readonly fromPort?: string;
    readonly toPort?: string;
  }[];
  readonly layout?: Readonly<Record<string, { readonly x: number; readonly y: number }>>;
}

export type FlowWidgetV1 =
  | { readonly kind: "neuron"; readonly id: string; readonly neuronKind: string; readonly inputPorts?: readonly string[] }
  | { readonly kind: "inputSlider"; readonly id: string; readonly value: number; readonly min?: number; readonly max?: number; readonly step?: number }
  | { readonly kind: "inputNote"; readonly id: string; readonly text: string }
  | { readonly kind: "inputImage"; readonly id: string; readonly src?: string }
  | { readonly kind: "outputPreview"; readonly id: string; readonly expanded?: readonly string[] }
  | { readonly kind: "outputAction"; readonly id: string; readonly action: string };

export const FLOW_DEFAULT_FIXTURE: FlowFixtureV1 = {
  schema: "flow.fixture/v1",
  camera: { x: 0, y: 0, zoom: 1 },
  widgets: [
    { kind: "inputSlider", id: "slider", value: 3 },
    { kind: "neuron", id: "add", neuronKind: "math.add" },
    { kind: "outputPreview", id: "preview" },
  ],
  synapses: [
    { id: "s1", from: "slider", to: "add", fromPort: "out", toPort: "a" },
    { id: "s2", from: "add", to: "preview", fromPort: "out", toPort: "in" },
  ],
};

export function flowFixtureToJson(fixture: FlowFixtureV1): string {
  return JSON.stringify(fixture);
}
// #endregion 🔖Fixture

// #region 🔖FlowStore
export interface FlowStore {
  load(): string | null;
  save(fixtureJson: string): void;
  clear(): void;
}

const FLOW_STORE_KEY = "flow.fixture/v1";

export function createLocalFlowStore(storage: Pick<Storage, "getItem" | "setItem" | "removeItem"> = globalThis.localStorage): FlowStore {
  return {
    load(): string | null {
      return storage.getItem(FLOW_STORE_KEY);
    },
    save(fixtureJson: string): void {
      storage.setItem(FLOW_STORE_KEY, fixtureJson);
    },
    clear(): void {
      storage.removeItem(FLOW_STORE_KEY);
    },
  };
}
// #endregion 🔖FlowStore

// #region 🔖Catalogue
export interface CatalogueItem {
  readonly kind: string;
  readonly neuronKind?: string;
  readonly action?: string;
  readonly name: string;
  readonly abbreviation: string;
  readonly icon: string;
  readonly summary: string;
}

export interface CatalogueSection {
  readonly id: string;
  readonly title: string;
  readonly items: readonly CatalogueItem[];
}

export const FLOW_WIDGET_DRAG_V1_MIME = "application/x-flow-widget-v1";
export const FLOW_WIDGET_DRAG_PLAIN_MIME = "text/plain";

/** @emoji 📍 Flow widget palette drop: descriptor plus pointer in CSS space and world on the canvas. */
export interface FlowWidgetDropDetail {
  readonly descriptor: string;
  readonly screen: { readonly x: number; readonly y: number };
  readonly world: { readonly x: number; readonly y: number };
}

export function flowCatalogueItemDescriptor(item: CatalogueItem): string {
  if (item.kind === "neuron" && item.neuronKind) {
    return JSON.stringify({ kind: "neuron", neuronKind: item.neuronKind });
  }
  if (item.kind === "outputAction") {
    return JSON.stringify({ kind: "outputAction", action: item.action ?? "log" });
  }
  return JSON.stringify({ kind: item.kind });
}

export function encodeFlowWidgetDescriptorForDragV1(descriptorJson: string): string {
  return descriptorJson;
}

export function decodeFlowWidgetDescriptorFromDragV1(encoded: string): string | null {
  const trimmed = encoded.trim();
  if (!trimmed) return null;
  try {
    const parsed = JSON.parse(trimmed) as { kind?: string };
    return typeof parsed.kind === "string" ? trimmed : null;
  } catch {
    return null;
  }
}

export function readFlowWidgetDragDataTransfer(dataTransfer: DataTransfer): string | null {
  const custom = dataTransfer.getData(FLOW_WIDGET_DRAG_V1_MIME);
  if (custom?.trim()) {
    return decodeFlowWidgetDescriptorFromDragV1(custom);
  }
  const plain = dataTransfer.getData(FLOW_WIDGET_DRAG_PLAIN_MIME);
  if (plain?.trim()) {
    return decodeFlowWidgetDescriptorFromDragV1(plain);
  }
  return null;
}

export function flowPlayCatalogueItemDragData(item: CatalogueItem): Record<string, string> {
  const encoded = encodeFlowWidgetDescriptorForDragV1(flowCatalogueItemDescriptor(item));
  return { [FLOW_WIDGET_DRAG_V1_MIME]: encoded, [FLOW_WIDGET_DRAG_PLAIN_MIME]: encoded };
}

export const flowWidgetPaletteDragRef = { active: false };
export const flowWidgetPalettePointerDragRef = { active: false, encoded: null as string | null };
export const flowWidgetDropPointerToWorldRef = {
  current: null as ((clientX: number, clientY: number) => { screen: { x: number; y: number }; world: { x: number; y: number } } | null) | null,
};

/** @emoji 🖱️ Begins pointer palette drag with an encoded widget descriptor. */
export function beginFlowWidgetPalettePointerDrag(encoded: string): void {
  flowWidgetPalettePointerDragRef.active = true;
  flowWidgetPalettePointerDragRef.encoded = encoded;
  flowWidgetPaletteDragRef.active = true;
  window.dispatchEvent(new CustomEvent("flow-widget-drag-session", { detail: { encoded } }));
}

/** @emoji 🖱️ Ends pointer palette drag without committing a drop. */
export function cancelFlowWidgetPalettePointerDrag(): void {
  if (!flowWidgetPalettePointerDragRef.active && !flowWidgetPaletteDragRef.active) {
    return;
  }
  flowWidgetPalettePointerDragRef.active = false;
  flowWidgetPalettePointerDragRef.encoded = null;
  flowWidgetPaletteDragRef.active = false;
  window.dispatchEvent(new CustomEvent("flow-widget-drag-session", { detail: null }));
}

/** @emoji 🎯 True when client coordinates are over the flow widget drop host. */
export function isClientPointOverFlowWidgetDropHost(clientX: number, clientY: number, host: HTMLElement | null | undefined): boolean {
  if (!host) return false;
  const rect = host.getBoundingClientRect();
  return clientX >= rect.left && clientX <= rect.right && clientY >= rect.top && clientY <= rect.bottom;
}

/** @emoji 📥 Commits a palette widget drop at client coordinates. */
export function commitFlowWidgetDropAtClient(
  clientX: number,
  clientY: number,
  descriptor: string,
  host: HTMLElement | null | undefined,
  onWidgetDrop: ((detail: FlowWidgetDropDetail) => void) | undefined,
): boolean {
  const toWorld = flowWidgetDropPointerToWorldRef.current;
  if (!toWorld || !onWidgetDrop) return false;
  const mapped = toWorld(clientX, clientY);
  if (!mapped) return false;
  if (!isClientPointOverFlowWidgetDropHost(clientX, clientY, host)) return false;
  onWidgetDrop({ descriptor, screen: mapped.screen, world: mapped.world });
  return true;
}

/** @emoji 🖱️ Ends pointer palette drag and drops on the viewport when over the host. */
export function endFlowWidgetPalettePointerDrag(
  clientX: number,
  clientY: number,
  host: HTMLElement | null | undefined,
  onWidgetDrop: ((detail: FlowWidgetDropDetail) => void) | undefined,
): void {
  if (!flowWidgetPalettePointerDragRef.active) return;
  const encoded = flowWidgetPalettePointerDragRef.encoded;
  cancelFlowWidgetPalettePointerDrag();
  if (!encoded) return;
  const descriptor = decodeFlowWidgetDescriptorFromDragV1(encoded);
  if (!descriptor) return;
  commitFlowWidgetDropAtClient(clientX, clientY, descriptor, host, onWidgetDrop);
}

/** @emoji 🔍 True when `dataTransfer.types` carries a flow widget palette drag. */
export function flowWidgetDragMimeInTypes(types: readonly string[]): boolean {
  return types.includes(FLOW_WIDGET_DRAG_V1_MIME) || types.includes(FLOW_WIDGET_DRAG_PLAIN_MIME);
}

/** @emoji 🔍 Whether the viewport should accept a palette widget drop for this drag gesture. */
export function flowWidgetDragAcceptsTransfer(types: readonly string[]): boolean {
  if (flowWidgetPalettePointerDragRef.active || flowWidgetPaletteDragRef.active) {
    return true;
  }
  return flowWidgetDragMimeInTypes(types);
}

/** @emoji 🖱️ {@link TreeDragAndDropController} for workbench rows that carry flow widget palette `dragData`. */
export function flowWidgetPaletteTreeDragController(
  dragDataByItemId: ReadonlyMap<string, Record<string, string>>,
): import("@framework/platform/core").TreeDragAndDropController {
  const readEncoded = (dragData: Record<string, string> | undefined): string | undefined => {
    const payload = dragData?.[FLOW_WIDGET_DRAG_V1_MIME];
    return payload?.trim() ? payload : undefined;
  };
  return {
    getDragData: ({ sourceItem }) => dragDataByItemId.get(sourceItem.id),
    pointerPaletteDrag: {
      readEncodedDragPayload: readEncoded,
      begin: beginFlowWidgetPalettePointerDrag,
      cancel: cancelFlowWidgetPalettePointerDrag,
    },
    onDragStart: ({ sourceItem }) => {
      if (flowWidgetPalettePointerDragRef.active) return;
      flowWidgetPaletteDragRef.active = true;
      const payload = readEncoded(dragDataByItemId.get(sourceItem.id));
      if (payload) {
        window.dispatchEvent(new CustomEvent("flow-widget-drag-session", { detail: { encoded: payload } }));
      }
    },
    onDragEnd: () => {
      if (flowWidgetPalettePointerDragRef.active) return;
      flowWidgetPaletteDragRef.active = false;
      window.dispatchEvent(new CustomEvent("flow-widget-drag-session", { detail: null }));
    },
  };
}

export function parseFlowCatalogueSections(json: string): CatalogueSection[] {
  try {
    const parsed = JSON.parse(json) as CatalogueSection[];
    return Array.isArray(parsed) ? parsed : [];
  } catch {
    return [];
  }
}

function flowCatalogueItemSearchText(item: CatalogueItem): string {
  return [item.name, item.summary, item.neuronKind ?? "", item.kind, item.action ?? ""].join(" ").toLowerCase();
}

function flowCatalogueItemRankScore(item: CatalogueItem, query: string): number {
  if (!query) return item.kind === "neuron" ? 0 : 1;
  const q = query.toLowerCase();
  const name = item.name.toLowerCase();
  const kind = (item.neuronKind ?? item.kind).toLowerCase();
  if (name === q || kind === q) return 0;
  if (name.startsWith(q) || kind.startsWith(q)) return 1;
  if (name.includes(q) || kind.includes(q)) return 2;
  if (flowCatalogueItemSearchText(item).includes(q)) return 3;
  return -1;
}

/** @emoji 🔍 Ranks catalogue items for flow canvas spotlight search. */
export function flowRankCatalogueSuggestions(sections: readonly CatalogueSection[], query: string): CatalogueItem[] {
  const items = sections.flatMap((section) => section.items);
  const trimmed = query.trim();
  if (!trimmed) {
    return [...items].sort((a, b) => {
      const rankDelta = flowCatalogueItemRankScore(a, "") - flowCatalogueItemRankScore(b, "");
      return rankDelta !== 0 ? rankDelta : a.name.localeCompare(b.name);
    });
  }
  return items
    .map((item) => ({ item, score: flowCatalogueItemRankScore(item, trimmed) }))
    .filter((entry) => entry.score >= 0)
    .sort((a, b) => (a.score !== b.score ? a.score - b.score : a.item.name.localeCompare(b.item.name)))
    .map((entry) => entry.item);
}
// #endregion 🔖Catalogue

export interface FlowReorganizeRequest {
  readonly epoch: number;
  readonly optionsJson: string;
}

export interface FlowCanvasCommandRequest {
  readonly epoch: number;
  readonly command: string;
  readonly argsJson?: string;
}

export interface FlowCanvasContextMenuContext {
  readonly hoveredNodeId: string | null;
  readonly selectedNodeIds: readonly string[];
  readonly isImageWidget: boolean;
  readonly isBackground: boolean;
  readonly previewOffNodeIds: readonly string[];
  readonly screen: { readonly x: number; readonly y: number };
  readonly world: { readonly x: number; readonly y: number };
  readonly clientX: number;
  readonly clientY: number;
}

export type FlowContextMenuDispatch = (command: string, args?: Record<string, unknown>) => void;

/** @emoji 🖱️ Context-aware flow canvas right-click menu entries. */
export function buildFlowContextMenuItems(ctx: FlowCanvasContextMenuContext, dispatch: FlowContextMenuDispatch): ContextMenuItem[] {
  const targetIds = ctx.selectedNodeIds.length > 0 ? [...ctx.selectedNodeIds] : ctx.hoveredNodeId ? [ctx.hoveredNodeId] : [];
  const previewOff = new Set(ctx.previewOffNodeIds);
  const allPreviewOff = targetIds.length > 0 && targetIds.every((id) => previewOff.has(id));
  const items: ContextMenuItem[] = [];
  if (ctx.isBackground) {
    items.push({
      id: "flow.ctx.add",
      label: "Add node…",
      icon: "plus",
      onSelect: () => dispatch("canvasCommand", { command: "openSpotlight", argsJson: JSON.stringify({ screen: ctx.screen, world: ctx.world }) }),
    });
    items.push({ id: "flow.ctx.sep.bg-1", separator: true });
  }
  if (targetIds.length > 0) {
    items.push({
      id: "flow.ctx.delete",
      label: targetIds.length === 1 ? "Delete" : `Delete (${targetIds.length})`,
      icon: "trash-2",
      destructive: true,
      shortcut: "⌫",
      onSelect: () => dispatch("canvasCommand", { command: "deleteSelection" }),
    });
    items.push({
      id: "flow.ctx.preview",
      label: allPreviewOff ? "Show in preview" : "Hide from preview",
      icon: allPreviewOff ? "eye" : "eye-off",
      checked: allPreviewOff,
      onSelect: () => dispatch("canvasCommand", { command: "togglePreview", argsJson: JSON.stringify({ ids: targetIds }) }),
    });
    if (ctx.isImageWidget && ctx.hoveredNodeId) {
      items.push({
        id: "flow.ctx.replaceImage",
        label: "Replace image…",
        icon: "image",
        onSelect: () => dispatch("canvasCommand", { command: "replaceImage", argsJson: JSON.stringify({ widgetId: ctx.hoveredNodeId }) }),
      });
    }
    if (ctx.selectedNodeIds.length > 0) {
      items.push({
        id: "flow.ctx.clearSelection",
        label: "Clear selection",
        icon: "x",
        onSelect: () => dispatch("canvasCommand", { command: "clearSelection" }),
      });
    }
    items.push({ id: "flow.ctx.sep.node-1", separator: true });
  }
  items.push({
    id: "flow.ctx.selectAll",
    label: "Select all",
    icon: "check-square",
    shortcut: "⌘A",
    onSelect: () => dispatch("canvasCommand", { command: "selectAll" }),
  });
  items.push({
    id: "flow.ctx.reorganize",
    label: "Reorganize",
    icon: "layout-grid",
    onSelect: () => dispatch("reorganize"),
  });
  return items;
}

// #region 🔖Spotlight
export interface FlowSpotlightSliderSpec {
  readonly value: number;
  readonly min: number;
  readonly max: number;
  readonly step: number;
  readonly label: string;
}

function flowDecimalPlacesFromNumberToken(token: string): number {
  const dot = token.trim().indexOf(".");
  if (dot < 0) return 0;
  return token.trim().length - dot - 1;
}

function flowSliderStepFromDecimalPlaces(places: number): number {
  if (places <= 0) return 1;
  return 10 ** -places;
}

function flowSensibleSliderMax(value: number): number {
  const v = Math.abs(value);
  if (v <= 1) return 1;
  if (v <= 10) return 10;
  const magnitude = 10 ** Math.floor(Math.log10(v));
  const normalized = v / magnitude;
  const nice = normalized <= 1 ? 1 : normalized <= 2 ? 2 : normalized <= 5 ? 5 : 10;
  return Math.max(nice * magnitude, v);
}

/** @emoji 🎚️ Parses flow spotlight input as a slider value or min..max range. */
export function flowParseSpotlightSliderQuery(query: string): FlowSpotlightSliderSpec | null {
  const trimmed = query.trim();
  if (!trimmed) return null;
  const rangeMatch = /^(-?\d+(?:\.\d+)?)\s*\.\.\s*(-?\d+(?:\.\d+)?)$/.exec(trimmed);
  if (rangeMatch) {
    const minToken = rangeMatch[1]!;
    const maxToken = rangeMatch[2]!;
    let min = Number(minToken);
    let max = Number(maxToken);
    if (!Number.isFinite(min) || !Number.isFinite(max)) return null;
    if (min > max) [min, max] = [max, min];
    const places = Math.max(flowDecimalPlacesFromNumberToken(minToken), flowDecimalPlacesFromNumberToken(maxToken));
    const step = flowSliderStepFromDecimalPlaces(places);
    return { value: min, min, max, step, label: `${min}–${max}` };
  }
  if (!/^-?\d+(?:\.\d+)?$/.test(trimmed)) return null;
  const value = Number(trimmed);
  if (!Number.isFinite(value)) return null;
  const step = flowSliderStepFromDecimalPlaces(flowDecimalPlacesFromNumberToken(trimmed));
  if (value < 0) {
    const bound = flowSensibleSliderMax(value);
    return { value, min: -bound, max: bound, step, label: String(value) };
  }
  return { value, min: 0, max: flowSensibleSliderMax(value), step, label: String(value) };
}

/** @emoji 🎚️ Builds a flow widget descriptor for a spotlight slider spec. */
export function flowSpotlightSliderDescriptor(spec: FlowSpotlightSliderSpec): string {
  return JSON.stringify({ kind: "inputSlider", value: spec.value, min: spec.min, max: spec.max, step: spec.step });
}

export interface FlowSpotlightNoteSpec {
  readonly text: string;
  readonly label: string;
}

function flowSpotlightNoteSummary(text: string): string {
  const compact = text.replace(/\s+/g, " ").trim();
  if (compact.length <= 48) return compact;
  return `${compact.slice(0, 45)}…`;
}

/** @emoji 📝 Parses flow spotlight input as note text when it is not numeric and has no catalogue match. */
export function flowParseSpotlightNoteQuery(query: string, sections: readonly CatalogueSection[]): FlowSpotlightNoteSpec | null {
  const trimmed = query.trim();
  if (!trimmed || flowParseSpotlightSliderQuery(trimmed)) return null;
  const matches = flowRankCatalogueSuggestions(sections, trimmed);
  if (matches.length > 0 && flowCatalogueItemRankScore(matches[0]!, trimmed) >= 0) return null;
  return { text: trimmed, label: flowSpotlightNoteSummary(trimmed) };
}

/** @emoji 📝 Builds a flow widget descriptor for a spotlight note spec. */
export function flowSpotlightNoteDescriptor(spec: FlowSpotlightNoteSpec): string {
  return JSON.stringify({ kind: "inputNote", text: spec.text });
}

const FLOW_SPOTLIGHT_SLIDER_ITEM: CatalogueItem = {
  kind: "inputSlider",
  name: "Slider",
  abbreviation: "Slider",
  icon: "emoji:🎚️",
  summary: "Number input",
};

const FLOW_SPOTLIGHT_NOTE_ITEM: CatalogueItem = {
  kind: "inputNote",
  name: "Note",
  abbreviation: "Note",
  icon: "emoji:📝",
  summary: "Text input",
};

interface FlowSpotlightAnchor {
  readonly screen: { readonly x: number; readonly y: number };
  readonly world: { readonly x: number; readonly y: number };
}

interface FlowSpotlightProps {
  readonly anchor: FlowSpotlightAnchor;
  readonly sections: readonly CatalogueSection[];
  readonly session: FlowSession;
  readonly onCommit: (detail: FlowWidgetDropDetail) => void;
  readonly onClose: () => void;
  readonly renderFrame: () => void;
}

function FlowSpotlight({ anchor, sections, session, onCommit, onClose, renderFrame }: FlowSpotlightProps): React.JSX.Element {
  const rootRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);
  const [query, setQuery] = useState("");
  const [expanded, setExpanded] = useState(false);
  const [activeIndex, setActiveIndex] = useState(0);
  const sliderSpec = flowParseSpotlightSliderQuery(query);
  const noteSpec = sliderSpec ? null : flowParseSpotlightNoteQuery(query, sections);
  const spotlightWidget = sliderSpec ? ("slider" as const) : noteSpec ? ("note" as const) : null;
  const catalogueSuggestions = flowRankCatalogueSuggestions(sections, spotlightWidget ? "" : query);
  const suggestions = sliderSpec
    ? [{ ...FLOW_SPOTLIGHT_SLIDER_ITEM, summary: `${sliderSpec.label} · ${sliderSpec.min}–${sliderSpec.max}` }]
    : noteSpec
      ? [{ ...FLOW_SPOTLIGHT_NOTE_ITEM, summary: noteSpec.label }]
      : catalogueSuggestions;
  const visible = expanded ? suggestions : suggestions.slice(0, 1);
  const hasMore = !spotlightWidget && suggestions.length > 1;

  const syncGhost = useCallback(
    (item: CatalogueItem | undefined, slider: FlowSpotlightSliderSpec | null, note: FlowSpotlightNoteSpec | null) => {
      if (slider) {
        try {
          session.setGhostWidget(flowSpotlightSliderDescriptor(slider), anchor.world.x, anchor.world.y);
          renderFrame();
        } catch {
          session.clearGhostWidget();
        }
        return;
      }
      if (note) {
        try {
          session.setGhostWidget(flowSpotlightNoteDescriptor(note), anchor.world.x, anchor.world.y);
          renderFrame();
        } catch {
          session.clearGhostWidget();
        }
        return;
      }
      if (!item) {
        session.clearGhostWidget();
        renderFrame();
        return;
      }
      try {
        session.setGhostWidget(flowCatalogueItemDescriptor(item), anchor.world.x, anchor.world.y);
        renderFrame();
      } catch {
        session.clearGhostWidget();
      }
    },
    [anchor.world.x, anchor.world.y, renderFrame, session],
  );

  const commitDescriptor = useCallback(
    (descriptor: string) => {
      session.clearGhostWidget();
      onCommit({ descriptor, screen: anchor.screen, world: anchor.world });
      onClose();
    },
    [anchor.screen, anchor.world, onClose, onCommit, session],
  );

  const commitItem = useCallback(
    (item: CatalogueItem) => {
      commitDescriptor(flowCatalogueItemDescriptor(item));
    },
    [commitDescriptor],
  );

  useEffect(() => {
    inputRef.current?.focus();
  }, []);

  useEffect(() => {
    setActiveIndex(0);
  }, [query]);

  useEffect(() => {
    syncGhost(suggestions[activeIndex], sliderSpec, noteSpec);
  }, [activeIndex, noteSpec, sliderSpec, suggestions, syncGhost]);

  useEffect(() => {
    return () => {
      session.clearGhostWidget();
    };
  }, [session]);

  useEffect(() => {
    const onPointerDown = (event: PointerEvent) => {
      const root = rootRef.current;
      if (!root || root.contains(event.target as Node)) return;
      session.clearGhostWidget();
      renderFrame();
      onClose();
    };
    window.addEventListener("pointerdown", onPointerDown);
    return () => window.removeEventListener("pointerdown", onPointerDown);
  }, [onClose, renderFrame, session]);

  const onInputKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
    if (event.key === "Escape") {
      event.preventDefault();
      session.clearGhostWidget();
      renderFrame();
      onClose();
      return;
    }
    if (event.key === "ArrowDown") {
      event.preventDefault();
      if (!suggestions.length) return;
      setActiveIndex((index) => Math.min(index + 1, suggestions.length - 1));
      if (!expanded && hasMore) setExpanded(true);
      return;
    }
    if (event.key === "ArrowUp") {
      event.preventDefault();
      setActiveIndex((index) => Math.max(index - 1, 0));
      return;
    }
    if (event.key === "Enter") {
      event.preventDefault();
      if (sliderSpec) {
        commitDescriptor(flowSpotlightSliderDescriptor(sliderSpec));
        return;
      }
      if (noteSpec) {
        commitDescriptor(flowSpotlightNoteDescriptor(noteSpec));
        return;
      }
      const item = suggestions[activeIndex];
      if (item) commitItem(item);
    }
  };

  return (
    <div
      ref={rootRef}
      className="absolute z-20 min-w-[14rem] max-w-[20rem] rounded-md border border-accent/50 bg-canvas shadow-lg ring-1 ring-accent/40"
      style={{ left: anchor.screen.x, top: anchor.screen.y }}
      onPointerDown={(event) => event.stopPropagation()}
      onDoubleClick={(event) => event.stopPropagation()}
    >
      <div className="flex items-center gap-1 border-b border-accent/30 px-2 py-1.5">
        <input
          ref={inputRef}
          type="text"
          value={query}
          placeholder="Add function, number, or text…"
          className="min-w-0 flex-1 bg-transparent text-sm text-foreground outline-none placeholder:text-muted"
          onChange={(event) => setQuery(event.target.value)}
          onKeyDown={onInputKeyDown}
        />
        {hasMore ? (
          <button
            type="button"
            aria-label={expanded ? "Collapse suggestions" : "Show all suggestions"}
            className="shrink-0 rounded px-1 text-muted hover:bg-accent/10 hover:text-accent"
            onClick={() => setExpanded((value) => !value)}
          >
            {expanded ? "▴" : "▾"}
          </button>
        ) : null}
      </div>
      <ul className="max-h-56 overflow-y-auto py-1">
        {visible.length === 0 ? (
          <li className="px-3 py-2 text-sm text-muted">No matches</li>
        ) : (
          visible.map((item, index) => {
            const globalIndex = expanded ? index : 0;
            const active = globalIndex === activeIndex;
            return (
              <li key={`${item.kind}-${item.neuronKind ?? item.action ?? item.name}`}>
                <button
                  type="button"
                  className={`flex w-full flex-col gap-0.5 px-3 py-1.5 text-left text-sm ${active ? "bg-accent/15 text-accent" : "text-foreground hover:bg-accent/10"}`}
                  onMouseEnter={() => setActiveIndex(globalIndex)}
                  onClick={() => commitItem(item)}
                >
                  <span className="font-medium">{item.name}</span>
                  <span className="text-xs text-muted">{item.summary}</span>
                </button>
              </li>
            );
          })
        )}
      </ul>
    </div>
  );
}
// #endregion 🔖Spotlight

// #region 🔖TextOverlay
const FLOW_LABEL_SCREEN_PX = 11;
const FLOW_LABEL_FONT_FAMILY = "ui-sans-serif, system-ui, sans-serif";

interface FlowLabelOverlayRow {
  readonly id: string;
  readonly text: string;
  readonly layout: "horizontal" | "vertical";
  readonly align?: "left" | "center" | "right";
  readonly x: number;
  readonly y: number;
  readonly nodeW: number;
  readonly nodeH: number;
  readonly fontScreenPx?: number;
}

interface FlowLabelOverlayPaintState {
  readonly camera: { readonly x: number; readonly y: number; readonly zoom: number };
  readonly width: number;
  readonly height: number;
  readonly labels: readonly FlowLabelOverlayRow[];
}

function flowWorldToScreen(
  point: { readonly x: number; readonly y: number },
  camera: { readonly x: number; readonly y: number; readonly zoom: number },
  width: number,
  height: number,
): { readonly x: number; readonly y: number } {
  return {
    x: (point.x - camera.x) * camera.zoom + width / 2,
    y: (point.y - camera.y) * camera.zoom + height / 2,
  };
}

function flowClampLabelFontPx(ctx: CanvasRenderingContext2D, text: string, targetPx: number, maxW: number, maxH: number): number {
  let px = Math.max(4, Math.round(targetPx));
  ctx.font = `${px}px ${FLOW_LABEL_FONT_FAMILY}`;
  if (ctx.measureText(text).width <= maxW && px * 1.2 <= maxH) {
    return px;
  }
  let low = 4;
  let high = px;
  let best = 4;
  while (low <= high) {
    const mid = Math.floor((low + high) / 2);
    ctx.font = `${mid}px ${FLOW_LABEL_FONT_FAMILY}`;
    const w = ctx.measureText(text).width;
    const h = mid * 1.2;
    if (w <= maxW && h <= maxH) {
      best = mid;
      low = mid + 1;
    } else {
      high = mid - 1;
    }
  }
  return best;
}

/** @emoji 🎯 Committed selection vs area-select preview chrome for flow label overlays. */
export function flowElementInteractionChrome(
  selectionIds: Iterable<string>,
  preselection: FlowPreselectSnapshot,
): { readonly selectedIds: Set<string>; readonly highlightedIds: Set<string> } {
  if (!preselection.ids.length && !preselection.removedIds.length) {
    return { selectedIds: new Set(selectionIds), highlightedIds: new Set() };
  }
  return { selectedIds: new Set(preselection.ids), highlightedIds: new Set(preselection.removedIds) };
}

function flowOverlayLabelFill(
  nodeId: string,
  hoveredId: string | null,
  chrome: { readonly selectedIds: Set<string>; readonly highlightedIds: Set<string> },
  previewOffIds: readonly string[],
): string {
  if (previewOffIds.includes(nodeId)) {
    return resolveSemanticColorHex("border-element-color", "gray");
  }
  if (chrome.selectedIds.has(nodeId)) {
    return resolveSemanticColorHex("border-emphasized-color", "dark");
  }
  if (chrome.highlightedIds.has(nodeId)) {
    return resolveSemanticColorHex("border-emphasized-color", "dark");
  }
  if (hoveredId === nodeId) {
    return resolveSemanticColorHex("border-emphasized-color", "dark");
  }
  return resolveSemanticColorHex("border-element-color", "gray");
}

function paintFlowLabelOverlays(session: FlowSession, canvas: HTMLCanvasElement, width: number, height: number, dpr: number): void {
  let state: FlowLabelOverlayPaintState;
  try {
    state = JSON.parse(session.labelOverlayPaintStateJson()) as FlowLabelOverlayPaintState;
  } catch {
    return;
  }
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  const pixelW = Math.max(1, Math.round(width * dpr));
  const pixelH = Math.max(1, Math.round(height * dpr));
  if (canvas.width !== pixelW || canvas.height !== pixelH) {
    canvas.width = pixelW;
    canvas.height = pixelH;
  }
  ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
  ctx.clearRect(0, 0, width, height);
  const zoom = Math.max(0.05, Number(state.camera?.zoom) || 1);
  const camera = {
    x: Number(state.camera?.x) || 0,
    y: Number(state.camera?.y) || 0,
    zoom,
  };
  const viewportW = Number(state.width) || width;
  const viewportH = Number(state.height) || height;
  const hoveredId = session.hoveredWidgetId() ?? null;
  const selectedIds = parseFlowWidgetIdArray(session.selectedWidgetIds());
  const preselect = parseFlowPreselectJson(session.preselectWidgetIdsJson());
  const chrome = flowElementInteractionChrome(selectedIds, preselect);
  const previewOffIds = parseFlowWidgetIdArray(session.previewOffWidgetIds());
  const inset = 0.88;
  for (const row of state.labels ?? []) {
    const text = typeof row.text === "string" ? row.text.trim() : "";
    if (!text) continue;
    const anchor = flowWorldToScreen({ x: Number(row.x), y: Number(row.y) }, camera, viewportW, viewportH);
    const maxW = Math.max(4, Number(row.nodeW) * zoom * inset);
    const maxH = Math.max(4, Number(row.nodeH) * zoom * inset);
    const fontScreenPx = Number(row.fontScreenPx);
    const targetPx = Number.isFinite(fontScreenPx) && fontScreenPx > 0 ? fontScreenPx : FLOW_LABEL_SCREEN_PX;
    const fontPx = flowClampLabelFontPx(ctx, text, targetPx, maxW, maxH);
    ctx.font = `${fontPx}px ${FLOW_LABEL_FONT_FAMILY}`;
    ctx.fillStyle = flowOverlayLabelFill(row.id, hoveredId, chrome, previewOffIds);
    ctx.globalAlpha = previewOffIds.includes(row.id) ? 0.5 : 1;
    if (row.layout === "vertical") {
      ctx.save();
      ctx.translate(anchor.x, anchor.y);
      ctx.rotate(-Math.PI / 2);
      ctx.textAlign = "center";
      ctx.textBaseline = "middle";
      ctx.fillText(text, 0, 0);
      ctx.restore();
    } else {
      const align = row.align === "left" || row.align === "right" ? row.align : "center";
      ctx.textAlign = align;
      ctx.textBaseline = "middle";
      ctx.fillText(text, anchor.x, anchor.y);
    }
    ctx.globalAlpha = 1;
  }
}
// #endregion 🔖TextOverlay

// #region 🔖FlowCanvas
export type FlowSelectionMode = "default" | "additive" | "subtractive" | "invertive";
export type FlowSelectionMethod = "rectangle" | "lasso";

export interface FlowPreselectSnapshot {
  readonly ids: readonly string[];
  readonly removedIds: readonly string[];
}

export interface FlowCanvasProps {
  readonly fixtureJson?: string;
  readonly className?: string;
  readonly store?: FlowStore;
  readonly fixtureDragDrop?: boolean;
  readonly reorganize?: FlowReorganizeRequest;
  readonly extensionRevision?: number;
  readonly extensionHost?: FlowExtensionHost;
  readonly onPreviewText?: (text: string) => void;
  readonly onEvalOutputs?: (outputsJson: string) => void;
  readonly onFixtureChange?: (fixtureJson: string) => void;
  readonly onCatalogueReady?: (sections: readonly CatalogueSection[]) => void;
  readonly onWidgetDrop?: (detail: FlowWidgetDropDetail) => void;
  readonly onSelectionChange?: (ids: readonly string[]) => void;
  readonly onPreselectChange?: (snapshot: FlowPreselectSnapshot) => void;
  readonly onHoverChange?: (id: string | null) => void;
  readonly selectedNodeIds?: readonly string[];
  readonly hoveredNodeId?: string | null;
  readonly previewOffNodeIds?: readonly string[];
  readonly selectionMode?: FlowSelectionMode;
  readonly selectionMethod?: FlowSelectionMethod;
  readonly contextMenu?: (ctx: FlowCanvasContextMenuContext) => readonly ContextMenuItem[];
  readonly onContextMenu?: (detail: { readonly clientX: number; readonly clientY: number; readonly hoveredNodeId: string | null }) => void;
  readonly commandRequest?: FlowCanvasCommandRequest;
  readonly onPreviewOffChange?: (ids: readonly string[]) => void;
  readonly automaticLod?: boolean;
  readonly lod?: DagDrawLodKind;
  readonly onLodChange?: (lod: DagDrawLodKind) => void;
}

export function parseFlowPreselectJson(json: string): FlowPreselectSnapshot {
  try {
    const parsed = JSON.parse(json) as { ids?: unknown; removedIds?: unknown };
    const ids = Array.isArray(parsed.ids) ? parsed.ids.filter((value): value is string => typeof value === "string") : [];
    const removedIds = Array.isArray(parsed.removedIds) ? parsed.removedIds.filter((value): value is string => typeof value === "string") : [];
    return { ids, removedIds };
  } catch {
    return { ids: [], removedIds: [] };
  }
}

export function parseFlowSelectionPreviewPoints(json: string): readonly { readonly x: number; readonly y: number }[] {
  try {
    const parsed = JSON.parse(json) as unknown;
    if (!Array.isArray(parsed)) return [];
    return parsed
      .map((entry) => {
        if (!Array.isArray(entry) || entry.length < 2) return null;
        const x = Number(entry[0]);
        const y = Number(entry[1]);
        return Number.isFinite(x) && Number.isFinite(y) ? { x, y } : null;
      })
      .filter((entry): entry is { x: number; y: number } => entry != null);
  } catch {
    return [];
  }
}

export function parseFlowWidgetIdArray(json: string): string[] {
  try {
    const parsed = JSON.parse(json) as unknown;
    return Array.isArray(parsed) ? parsed.filter((value): value is string => typeof value === "string") : [];
  } catch {
    return [];
  }
}

export function flowWidgetIdArraysEqual(left: readonly string[], right: readonly string[]): boolean {
  if (left.length !== right.length) return false;
  return left.every((value, index) => value === right[index]);
}

function flowWheelDeltaScale(deltaMode: number): number {
  if (deltaMode === WheelEvent.DOM_DELTA_LINE) return 16;
  if (deltaMode === WheelEvent.DOM_DELTA_PAGE) return 400;
  return 1;
}

/** @emoji 🖱️ Scroll wheel zooms the flow canvas. */
export function flowWheelGestureIsZoom(_event?: Pick<WheelEvent, "ctrlKey" | "metaKey">): boolean {
  return true;
}

export function FlowCanvas({
  fixtureJson,
  className,
  store,
  fixtureDragDrop = false,
  reorganize,
  extensionRevision = 0,
  extensionHost = flowExtensionHost,
  onPreviewText,
  onEvalOutputs,
  onFixtureChange,
  onCatalogueReady,
  onWidgetDrop,
  onSelectionChange,
  onPreselectChange,
  onHoverChange,
  selectedNodeIds,
  hoveredNodeId,
  previewOffNodeIds,
  selectionMode = "default",
  selectionMethod = "rectangle",
  contextMenu,
  onContextMenu,
  commandRequest,
  onPreviewOffChange,
  automaticLod = true,
  lod,
  onLodChange,
}: FlowCanvasProps): React.JSX.Element {
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const textOverlayRef = useRef<HTMLCanvasElement>(null);
  const viewportRef = useRef({ width: 1, height: 1, dpr: 1 });
  const sessionRef = useRef<FlowSession | null>(null);
  const rafRef = useRef<number | null>(null);
  const wheelZoomEndRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const onPreviewTextRef = useRef(onPreviewText);
  const onEvalOutputsRef = useRef(onEvalOutputs);
  const onFixtureChangeRef = useRef(onFixtureChange);
  const onCatalogueReadyRef = useRef(onCatalogueReady);
  const onWidgetDropRef = useRef(onWidgetDrop);
  const onSelectionChangeRef = useRef(onSelectionChange);
  const onPreselectChangeRef = useRef(onPreselectChange);
  const onHoverChangeRef = useRef(onHoverChange);
  const storeRef = useRef(store ?? createLocalFlowStore());
  const pointerRef = useRef({ active: false, pan: false, id: -1, shift: false, ctrl: false, alt: false });
  const imageFileInputRef = useRef<HTMLInputElement>(null);
  const pendingImageWidgetIdRef = useRef<string | null>(null);
  const onContextMenuRef = useRef(onContextMenu);
  const contextMenuRef = useRef(contextMenu);
  const onPreviewOffChangeRef = useRef(onPreviewOffChange);
  const onLodChangeRef = useRef(onLodChange);
  const lastAutomaticLodRef = useRef<boolean | null>(null);
  const lastForcedLodRef = useRef<string | null>(null);
  const lastReportedLodRef = useRef<DagDrawLodKind | null>(null);
  const [surfaceContextMenu, setSurfaceContextMenu] = useState<{
    readonly clientX: number;
    readonly clientY: number;
    readonly items: readonly ContextMenuItem[];
  } | null>(null);
  const fixtureDragDepthRef = useRef(0);
  const bootstrapFixtureJsonRef = useRef(fixtureJson);
  const catalogueSectionsRef = useRef<CatalogueSection[]>([]);
  const [fixtureDragActive, setFixtureDragActive] = useState(false);
  const [spotlight, setSpotlight] = useState<FlowSpotlightAnchor | null>(null);
  const [marqueeOverlay, setMarqueeOverlay] = useState<{
    readonly coverage: SelectionMarqueeCoverage;
    readonly shape: "rect" | "polygon";
    readonly rect?: { readonly x: number; readonly y: number; readonly width: number; readonly height: number };
    readonly points?: readonly { readonly x: number; readonly y: number }[];
  } | null>(null);

  const syncVelloTheme = useCallback(() => {
    if (typeof document === "undefined") return;
    const session = sessionRef.current;
    if (!session) return;
    try {
      clearColorResolveCache();
      session.setVelloThemeJson(serializeGraphVelloThemePaletteJson());
    } catch {
      /* document theme tokens not ready */
    }
  }, []);

  useEffect(() => {
    if (typeof document === "undefined") return;
    const root = document.documentElement;
    const obs = new MutationObserver(() => syncVelloTheme());
    obs.observe(root, { attributes: true, attributeFilter: ["class", "style", "data-theme"] });
    return () => obs.disconnect();
  }, [syncVelloTheme]);

  useEffect(() => {
    onPreviewTextRef.current = onPreviewText;
  }, [onPreviewText]);

  useEffect(() => {
    onEvalOutputsRef.current = onEvalOutputs;
  }, [onEvalOutputs]);

  useEffect(() => {
    onFixtureChangeRef.current = onFixtureChange;
  }, [onFixtureChange]);

  useEffect(() => {
    onCatalogueReadyRef.current = onCatalogueReady;
  }, [onCatalogueReady]);

  useEffect(() => {
    onWidgetDropRef.current = onWidgetDrop;
  }, [onWidgetDrop]);

  useEffect(() => {
    onSelectionChangeRef.current = onSelectionChange;
  }, [onSelectionChange]);

  useEffect(() => {
    onPreselectChangeRef.current = onPreselectChange;
  }, [onPreselectChange]);

  useEffect(() => {
    onHoverChangeRef.current = onHoverChange;
  }, [onHoverChange]);

  useEffect(() => {
    onContextMenuRef.current = onContextMenu;
  }, [onContextMenu]);

  useEffect(() => {
    contextMenuRef.current = contextMenu;
  }, [contextMenu]);

  useEffect(() => {
    onPreviewOffChangeRef.current = onPreviewOffChange;
  }, [onPreviewOffChange]);

  useEffect(() => {
    onLodChangeRef.current = onLodChange;
  }, [onLodChange]);

  useEffect(() => {
    if (store) storeRef.current = store;
  }, [store]);

  const syncLodMode = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    const nextAutomatic = automaticLod ?? true;
    if (lastAutomaticLodRef.current !== nextAutomatic) {
      session.setAutomaticLod(nextAutomatic);
      lastAutomaticLodRef.current = nextAutomatic;
    }
    const forced = nextAutomatic ? "" : lod && isDagDrawLodKind(lod) ? lod : "";
    if (lastForcedLodRef.current !== forced) {
      session.setForcedDrawLodLabel(forced);
      lastForcedLodRef.current = forced;
    }
  }, [automaticLod, lod]);

  const reportDrawLod = useCallback(() => {
    const session = sessionRef.current;
    if (!session || !onLodChangeRef.current) return;
    try {
      const label = session.drawLodLabel();
      if (!isDagDrawLodKind(label)) return;
      if (lastReportedLodRef.current === label) return;
      lastReportedLodRef.current = label;
      onLodChangeRef.current(label);
    } catch {
      /* session not ready */
    }
  }, []);

  const syncMarqueeOverlay = useCallback((session: FlowSession) => {
    const points = parseFlowSelectionPreviewPoints(session.selectionPreviewPointsJson());
    if (points.length < 2) {
      setMarqueeOverlay(null);
      return;
    }
    const coverage: SelectionMarqueeCoverage = session.selectionPreviewCrossing() ? "partial" : "full";
    if (selectionMethod === "lasso" && points.length >= 3) {
      setMarqueeOverlay({ coverage, shape: "polygon", points });
      return;
    }
    const xs = points.map((point) => point.x);
    const ys = points.map((point) => point.y);
    const minX = Math.min(...xs);
    const minY = Math.min(...ys);
    const maxX = Math.max(...xs);
    const maxY = Math.max(...ys);
    setMarqueeOverlay({ coverage, shape: "rect", rect: { x: minX, y: minY, width: maxX - minX, height: maxY - minY } });
  }, [selectionMethod]);

  const emitInteractionState = useCallback((session: FlowSession) => {
    const selected = parseFlowWidgetIdArray(session.selectedWidgetIds());
    const hovered = session.hoveredWidgetId() ?? null;
    const preselect = parseFlowPreselectJson(session.preselectWidgetIdsJson());
    const previewOff = parseFlowWidgetIdArray(session.previewOffWidgetIds());
    onSelectionChangeRef.current?.(selected);
    onPreselectChangeRef.current?.(preselect);
    onHoverChangeRef.current?.(hovered);
    onPreviewOffChangeRef.current?.(previewOff);
    syncMarqueeOverlay(session);
    console.log(`[DEBUG] flow interaction selected=[${selected.join(", ")}] preselect=[${preselect.ids.join(", ")}] hover=${hovered ?? "—"} previewOff=[${previewOff.join(", ")}]`);
  }, [syncMarqueeOverlay]);

  const persistFixture = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    try {
      const json = session.fixtureJson();
      storeRef.current.save(json);
      onFixtureChangeRef.current?.(json);
    } catch {
      /* fixture not ready */
    }
  }, []);

  const renderFrame = useCallback(() => {
    try {
      syncLodMode();
      syncVelloTheme();
      const session = sessionRef.current;
      session?.renderFrame();
      const overlay = textOverlayRef.current;
      if (session && overlay) {
        const { width, height, dpr } = viewportRef.current;
        paintFlowLabelOverlays(session, overlay, width, height, dpr);
      }
      reportDrawLod();
    } catch {
      /* gpu not ready */
    }
  }, [reportDrawLod, syncLodMode, syncVelloTheme]);

  useEffect(() => {
    lastAutomaticLodRef.current = null;
    lastForcedLodRef.current = null;
    renderFrame();
  }, [automaticLod, lod, renderFrame]);

  const evaluate = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    const outputsJson = session.evaluate();
    const text = session.previewText();
    onPreviewTextRef.current?.(text);
    onEvalOutputsRef.current?.(outputsJson);
    console.log(`[DEBUG] flow evaluate preview: ${text}`);
  }, []);

  useEffect(() => {
    const session = sessionRef.current;
    if (!session || !reorganize || reorganize.epoch <= 0) return;
    try {
      session.reorganize(reorganize.optionsJson);
      console.log("[DEBUG] flow canvas reorganized:", session.fixtureJson());
      evaluate();
      persistFixture();
      renderFrame();
    } catch (err) {
      console.log(`[DEBUG] flow canvas reorganize failed: ${String(err)}`);
    }
  }, [reorganize?.epoch, reorganize?.optionsJson, evaluate, persistFixture, renderFrame]);

  const syncExtensionSurface = useCallback(
    (session: FlowSession) => {
      session.setEvalBridge(createFlowEvalBridge(extensionHost));
      session.setCatalogueJson(extensionHost.catalogueJson());
      session.setNeuronKindInfosJson(extensionHost.kindInfosJson());
      const sections = parseFlowCatalogueSections(session.catalogueJson());
      catalogueSectionsRef.current = sections;
      onCatalogueReadyRef.current?.(sections);
    },
    [extensionHost],
  );

  const loadCatalogue = useCallback(
    (session: FlowSession) => {
      syncExtensionSurface(session);
    },
    [syncExtensionSurface],
  );

  const commitWidgetDrop = useCallback(
    (detail: FlowWidgetDropDetail) => {
      const session = sessionRef.current;
      if (!session) return false;
      session.clearGhostWidget();
      const handler = onWidgetDropRef.current;
      if (handler) {
        handler(detail);
      } else {
        session.addWidget(detail.descriptor, detail.world.x, detail.world.y);
        evaluate();
        persistFixture();
      }
      renderFrame();
      console.log(`[DEBUG] flow add widget at ${detail.world.x.toFixed(1)}, ${detail.world.y.toFixed(1)}`);
      return true;
    },
    [evaluate, persistFixture, renderFrame],
  );

  useEffect(() => {
    const session = sessionRef.current;
    if (!session || extensionRevision < 0) return;
    syncExtensionSurface(session);
    evaluate();
    renderFrame();
  }, [evaluate, extensionRevision, renderFrame, syncExtensionSurface]);

  useEffect(() => {
    return extensionHost.subscribe(() => {
      const session = sessionRef.current;
      if (!session) return;
      syncExtensionSurface(session);
      evaluate();
      renderFrame();
    });
  }, [evaluate, extensionHost, renderFrame, syncExtensionSurface]);

  useEffect(() => {
    const session = sessionRef.current;
    if (!session) return;
    if (selectedNodeIds !== undefined) {
      const current = parseFlowWidgetIdArray(session.selectedWidgetIds());
      if (!flowWidgetIdArraysEqual(current, selectedNodeIds)) {
        session.setSelection(JSON.stringify([...selectedNodeIds]));
      }
    }
    if (hoveredNodeId !== undefined) {
      const current = session.hoveredWidgetId() ?? null;
      if (current !== hoveredNodeId) {
        session.setHover(hoveredNodeId);
      }
    }
    if (previewOffNodeIds !== undefined) {
      const current = parseFlowWidgetIdArray(session.previewOffWidgetIds());
      if (!flowWidgetIdArraysEqual(current, previewOffNodeIds)) {
        session.setPreviewOff(JSON.stringify([...previewOffNodeIds]));
      }
    }
    renderFrame();
  }, [hoveredNodeId, previewOffNodeIds, renderFrame, selectedNodeIds]);

  useEffect(() => {
    const session = sessionRef.current;
    if (!session) return;
    session.setSelectionOptions(selectionMethod, selectionMode);
    renderFrame();
  }, [renderFrame, selectionMethod, selectionMode]);

  useEffect(() => {
    const container = containerRef.current;
    if (!container) return;
    const onKeyDown = (event: KeyboardEvent) => {
      const session = sessionRef.current;
      if (!session) return;
      const mod = event.metaKey || event.ctrlKey;
      if (mod && event.key.toLowerCase() === "a") {
        event.preventDefault();
        session.selectAll();
        emitInteractionState(session);
        evaluate();
        persistFixture();
        renderFrame();
        return;
      }
      if (event.key === "Escape") {
        if (session.cancelAreaSelect()) {
          event.preventDefault();
          emitInteractionState(session);
          renderFrame();
        }
        return;
      }
      if (event.key === "Delete" || event.key === "Backspace") {
        const selected = parseFlowWidgetIdArray(session.selectedWidgetIds());
        if (!selected.length) return;
        event.preventDefault();
        try {
          session.deleteSelection();
          emitInteractionState(session);
          evaluate();
          persistFixture();
          renderFrame();
        } catch (err) {
          console.log(`[DEBUG] flow deleteSelection failed: ${String(err)}`);
        }
      }
    };
    container.addEventListener("keydown", onKeyDown);
    return () => container.removeEventListener("keydown", onKeyDown);
  }, [emitInteractionState, evaluate, persistFixture, renderFrame]);

  useEffect(() => {
    const canvas = canvasRef.current;
    const container = containerRef.current;
    if (!canvas || !container) return;
    let cancelled = false;
    let cleanupResize: (() => void) | undefined;
    void (async () => {
      await extensionHost.activateDefaults();
      if (cancelled) return;
      const session = new FlowSession();
      sessionRef.current = session;
      const saved = storeRef.current.load();
      const json = saved ?? bootstrapFixtureJsonRef.current ?? flowFixtureToJson(FLOW_DEFAULT_FIXTURE);
      session.loadFixtureJson(json);
      syncExtensionSurface(session);
      syncVelloTheme();
      evaluate();
      try {
        const layout = JSON.parse(session.fixtureJson()).layout ?? {};
        console.log(`[DEBUG] flow fixture layout: ${JSON.stringify(layout)}`);
      } catch {
        /* fixture not serializable yet */
      }
      const rect = container.getBoundingClientRect();
      const dpr = globalThis.devicePixelRatio || 1;
      const initW = Math.max(1, Math.round(rect.width));
      const initH = Math.max(1, Math.round(rect.height));
      viewportRef.current = { width: initW, height: initH, dpr };
      session.setSize(initW, initH, dpr);
      canvas.width = Math.round(initW * dpr);
      canvas.height = Math.round(initH * dpr);
      canvas.style.width = `${initW}px`;
      canvas.style.height = `${initH}px`;
      await session.attachCanvas(canvas, initW, initH, dpr);
      if (cancelled) return;
      const resize = () => {
        const rect = container.getBoundingClientRect();
        const dpr = globalThis.devicePixelRatio || 1;
        const w = Math.max(1, Math.round(rect.width));
        const h = Math.max(1, Math.round(rect.height));
        viewportRef.current = { width: w, height: h, dpr };
        canvas.width = Math.round(w * dpr);
        canvas.height = Math.round(h * dpr);
        canvas.style.width = `${w}px`;
        canvas.style.height = `${h}px`;
        session.setSize(w, h, dpr);
        renderFrame();
      };
      resize();
      const ro = new ResizeObserver(resize);
      ro.observe(container);
      const visualViewport = globalThis.visualViewport;
      visualViewport?.addEventListener("resize", resize);
      const tick = () => {
        renderFrame();
        rafRef.current = requestAnimationFrame(tick);
      };
      rafRef.current = requestAnimationFrame(tick);
      cleanupResize = () => {
        ro.disconnect();
        visualViewport?.removeEventListener("resize", resize);
        if (rafRef.current != null) cancelAnimationFrame(rafRef.current);
      };
    })();
    return () => {
      cancelled = true;
      cleanupResize?.();
      if (wheelZoomEndRef.current != null) {
        clearTimeout(wheelZoomEndRef.current);
        wheelZoomEndRef.current = null;
      }
      if (rafRef.current != null) cancelAnimationFrame(rafRef.current);
      sessionRef.current = null;
    };
  }, [evaluate, extensionHost, renderFrame, syncExtensionSurface]);

  const clientToCanvas = useCallback((clientX: number, clientY: number): { x: number; y: number } => {
    const canvas = canvasRef.current;
    if (!canvas) return { x: clientX, y: clientY };
    const rect = canvas.getBoundingClientRect();
    return { x: clientX - rect.left, y: clientY - rect.top };
  }, []);

  const onPointerDown = useCallback(
    (e: React.PointerEvent<HTMLCanvasElement>) => {
      const session = sessionRef.current;
      if (!session || e.button > 2) return;
      if (e.button === 2) return;
      e.currentTarget.setPointerCapture(e.pointerId);
      pointerRef.current = { active: true, pan: false, id: e.pointerId, shift: e.shiftKey, ctrl: e.metaKey || e.ctrlKey, alt: e.altKey };
      const { x, y } = clientToCanvas(e.clientX, e.clientY);
      session.pointerDownScreen(x, y, e.button, e.shiftKey, e.metaKey || e.ctrlKey, e.altKey, false);
      emitInteractionState(session);
      renderFrame();
    },
    [clientToCanvas, emitInteractionState, renderFrame],
  );

  const onPointerMove = useCallback(
    (e: React.PointerEvent<HTMLCanvasElement>) => {
      const session = sessionRef.current;
      if (!session) return;
      if (pointerRef.current.active && pointerRef.current.id !== e.pointerId) return;
      const { x, y } = clientToCanvas(e.clientX, e.clientY);
      session.pointerMoveScreen(x, y, e.shiftKey, e.metaKey || e.ctrlKey, e.altKey);
      emitInteractionState(session);
      if (pointerRef.current.active) {
        if (session.widgetDragActive()) {
          evaluate();
        }
        renderFrame();
      }
    },
    [clientToCanvas, emitInteractionState, evaluate, renderFrame],
  );

  const onPointerLeave = useCallback(
    (e: React.PointerEvent<HTMLCanvasElement>) => {
      const session = sessionRef.current;
      if (!session || pointerRef.current.active) return;
      session.setHover(null);
      emitInteractionState(session);
    },
    [emitInteractionState],
  );

  const onPointerUp = useCallback(
    (e: React.PointerEvent<HTMLCanvasElement>) => {
      const session = sessionRef.current;
      if (!session || pointerRef.current.id !== e.pointerId) return;
      const { x, y } = clientToCanvas(e.clientX, e.clientY);
      session.pointerUpScreen(x, y, e.shiftKey, e.metaKey || e.ctrlKey, e.altKey);
      pointerRef.current = { active: false, pan: false, id: -1, shift: false, ctrl: false, alt: false };
      emitInteractionState(session);
      evaluate();
      persistFixture();
      renderFrame();
    },
    [clientToCanvas, emitInteractionState, evaluate, persistFixture, renderFrame],
  );

  const closeSpotlight = useCallback(() => {
    setSpotlight(null);
  }, []);

  const openImagePicker = useCallback((widgetId: string) => {
    pendingImageWidgetIdRef.current = widgetId;
    imageFileInputRef.current?.click();
  }, []);

  useEffect(() => {
    const session = sessionRef.current;
    if (!session || !commandRequest || commandRequest.epoch <= 0) return;
    const args = commandRequest.argsJson ? (JSON.parse(commandRequest.argsJson) as Record<string, unknown>) : {};
    try {
      switch (commandRequest.command) {
        case "openSpotlight": {
          const screen = args.screen as { x: number; y: number } | undefined;
          const world = args.world as { x: number; y: number } | undefined;
          if (screen && world) setSpotlight({ screen, world });
          break;
        }
        case "selectAll":
          session.selectAll();
          emitInteractionState(session);
          evaluate();
          persistFixture();
          renderFrame();
          break;
        case "clearSelection":
          session.setSelection(JSON.stringify([]));
          emitInteractionState(session);
          renderFrame();
          break;
        case "deleteSelection":
          session.deleteSelection();
          emitInteractionState(session);
          evaluate();
          persistFixture();
          renderFrame();
          break;
        case "togglePreview": {
          const ids = Array.isArray(args.ids) ? args.ids.filter((value): value is string => typeof value === "string") : [];
          const current = parseFlowWidgetIdArray(session.previewOffWidgetIds());
          const off = new Set(current);
          for (const id of ids) {
            if (off.has(id)) off.delete(id);
            else off.add(id);
          }
          session.setPreviewOff(JSON.stringify([...off]));
          emitInteractionState(session);
          evaluate();
          renderFrame();
          break;
        }
        case "replaceImage": {
          const widgetId = typeof args.widgetId === "string" ? args.widgetId : null;
          if (widgetId) openImagePicker(widgetId);
          break;
        }
        default:
          console.log(`[DEBUG] flow canvas unknown command: ${commandRequest.command}`);
      }
    } catch (err) {
      console.log(`[DEBUG] flow canvas command failed: ${String(err)}`);
    }
  }, [commandRequest?.epoch, commandRequest?.command, commandRequest?.argsJson, emitInteractionState, evaluate, openImagePicker, persistFixture, renderFrame]);

  const onImageFileChange = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      const session = sessionRef.current;
      const file = e.target.files?.[0];
      const widgetId = pendingImageWidgetIdRef.current;
      e.target.value = "";
      pendingImageWidgetIdRef.current = null;
      if (!session || !file || !widgetId) return;
      const reader = new FileReader();
      reader.onload = () => {
        const src = reader.result;
        if (typeof src !== "string") return;
        session.setImageSrc(widgetId, src);
        evaluate();
        persistFixture();
        renderFrame();
      };
      reader.readAsDataURL(file);
    },
    [evaluate, persistFixture, renderFrame],
  );

  const onCanvasDoubleClick = useCallback(
    (e: React.MouseEvent<HTMLCanvasElement>) => {
      const session = sessionRef.current;
      if (!session) return;
      e.preventDefault();
      const hoveredId = session.hoveredWidgetId();
      if (hoveredId) {
        try {
          const fixture = JSON.parse(session.fixtureJson()) as FlowFixtureV1;
          const widget = fixture.widgets.find((entry) => entry.id === hoveredId);
          if (widget?.kind === "inputImage") {
            openImagePicker(hoveredId);
            return;
          }
        } catch {
          /* fixture not ready */
        }
      }
      const screen = clientToCanvas(e.clientX, e.clientY);
      const world = JSON.parse(session.worldFromScreen(screen.x, screen.y)) as { x: number; y: number };
      setSpotlight({ screen, world });
    },
    [clientToCanvas, openImagePicker],
  );

  const syncWheelZoomActive = useCallback((active: boolean) => {
    try {
      sessionRef.current?.setWheelZoomActive(active);
    } catch {
      /* gpu not ready */
    }
  }, []);

  const handleWheel = useCallback(
    (e: WheelEvent) => {
      const session = sessionRef.current;
      if (!session) return;
      e.preventDefault();
      const scale = flowWheelDeltaScale(e.deltaMode);
      const deltaX = e.deltaX * scale;
      const deltaY = e.deltaY * scale;
      syncWheelZoomActive(true);
      if (wheelZoomEndRef.current != null) {
        clearTimeout(wheelZoomEndRef.current);
      }
      wheelZoomEndRef.current = setTimeout(() => {
        wheelZoomEndRef.current = null;
        syncWheelZoomActive(false);
      }, 120);
      const { x, y } = clientToCanvas(e.clientX, e.clientY);
      session.wheelScreen(x, y, deltaX, deltaY, flowWheelGestureIsZoom(e));
      persistFixture();
      renderFrame();
    },
    [clientToCanvas, persistFixture, renderFrame, syncWheelZoomActive],
  );

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    canvas.addEventListener("wheel", handleWheel, { passive: false });
    return () => canvas.removeEventListener("wheel", handleWheel);
  }, [handleWheel]);

  const onCanvasContextMenu = useCallback(
    (e: React.MouseEvent<HTMLCanvasElement>) => {
      if (e.altKey) {
        e.preventDefault();
        return;
      }
      e.preventDefault();
      const session = sessionRef.current;
      if (!session) return;
      const hoveredNodeId = session.hoveredWidgetId() ?? null;
      const selectedNodeIds = parseFlowWidgetIdArray(session.selectedWidgetIds());
      const previewOffNodeIds = parseFlowWidgetIdArray(session.previewOffWidgetIds());
      if (hoveredNodeId && !selectedNodeIds.includes(hoveredNodeId)) {
        session.setSelection(JSON.stringify([hoveredNodeId]));
        emitInteractionState(session);
        renderFrame();
      }
      const screen = clientToCanvas(e.clientX, e.clientY);
      const world = JSON.parse(session.worldFromScreen(screen.x, screen.y)) as { x: number; y: number };
      let isImageWidget = false;
      if (hoveredNodeId) {
        try {
          const fixture = JSON.parse(session.fixtureJson()) as FlowFixtureV1;
          const widget = fixture.widgets.find((entry) => entry.id === hoveredNodeId);
          isImageWidget = widget?.kind === "inputImage";
        } catch {
          /* fixture not ready */
        }
      }
      const menuCtx: FlowCanvasContextMenuContext = {
        hoveredNodeId,
        selectedNodeIds: hoveredNodeId && !selectedNodeIds.includes(hoveredNodeId) ? [hoveredNodeId] : selectedNodeIds,
        isImageWidget,
        isBackground: !hoveredNodeId,
        previewOffNodeIds,
        screen,
        world,
        clientX: e.clientX,
        clientY: e.clientY,
      };
      onContextMenuRef.current?.({ clientX: e.clientX, clientY: e.clientY, hoveredNodeId });
      const buildMenu = contextMenuRef.current;
      const items = buildMenu ? buildMenu(menuCtx) : [];
      if (items.length > 0) {
        setSurfaceContextMenu({ clientX: e.clientX, clientY: e.clientY, items });
      }
    },
    [clientToCanvas, emitInteractionState, renderFrame],
  );

  const resetFixtureDragDepth = useCallback(() => {
    fixtureDragDepthRef.current = 0;
    setFixtureDragActive(false);
  }, []);

  const onDragEnter = useCallback(
    (e: React.DragEvent<HTMLDivElement>) => {
      if (!fixtureDragDrop) return;
      if (!flowWidgetDragAcceptsTransfer([...e.dataTransfer.types])) return;
      fixtureDragDepthRef.current += 1;
      setFixtureDragActive(true);
    },
    [fixtureDragDrop],
  );

  const onDragLeave = useCallback(
    (e: React.DragEvent<HTMLDivElement>) => {
      if (!fixtureDragDrop) return;
      const target = e.currentTarget as HTMLElement;
      const related = e.relatedTarget as Node | null;
      if (related && target.contains(related)) return;
      fixtureDragDepthRef.current = Math.max(0, fixtureDragDepthRef.current - 1);
      if (fixtureDragDepthRef.current === 0) {
        setFixtureDragActive(false);
      }
    },
    [fixtureDragDrop],
  );

  const onDragOver = useCallback(
    (e: React.DragEvent<HTMLDivElement>) => {
      if (!fixtureDragDrop) return;
      if (!flowWidgetDragAcceptsTransfer([...e.dataTransfer.types])) return;
      e.preventDefault();
      e.dataTransfer.dropEffect = "copy";
    },
    [fixtureDragDrop],
  );

  const commitWidgetDropAtClient = useCallback(
    (clientX: number, clientY: number, descriptor: string) => {
      const session = sessionRef.current;
      const host = containerRef.current ?? canvasRef.current;
      if (!session || !host) return false;
      const rect = host.getBoundingClientRect();
      const screen = { x: clientX - rect.left, y: clientY - rect.top };
      const world = JSON.parse(session.worldFromScreen(screen.x, screen.y)) as { x: number; y: number };
      return commitWidgetDrop({ descriptor, screen, world });
    },
    [commitWidgetDrop],
  );

  const onDrop = useCallback(
    (e: React.DragEvent<HTMLDivElement>) => {
      if (!fixtureDragDrop) return;
      const descriptor = readFlowWidgetDragDataTransfer(e.dataTransfer);
      const accepts = descriptor !== null || flowWidgetDragAcceptsTransfer([...e.dataTransfer.types]);
      if (!accepts) return;
      e.preventDefault();
      resetFixtureDragDepth();
      if (!descriptor) return;
      commitWidgetDropAtClient(e.clientX, e.clientY, descriptor);
    },
    [commitWidgetDropAtClient, fixtureDragDrop, resetFixtureDragDepth],
  );

  useEffect(() => {
    if (!fixtureDragDrop) {
      flowWidgetDropPointerToWorldRef.current = null;
      return;
    }
    flowWidgetDropPointerToWorldRef.current = (clientX, clientY) => {
      const session = sessionRef.current;
      const host = containerRef.current ?? canvasRef.current;
      if (!session || !host) return null;
      const rect = host.getBoundingClientRect();
      const screen = { x: clientX - rect.left, y: clientY - rect.top };
      const world = JSON.parse(session.worldFromScreen(screen.x, screen.y)) as { x: number; y: number };
      return { screen, world };
    };
    return () => {
      flowWidgetDropPointerToWorldRef.current = null;
    };
  }, [fixtureDragDrop]);

  useEffect(() => {
    if (!fixtureDragDrop) return;
    const dropHost = (): HTMLElement | null => containerRef.current ?? canvasRef.current;
    const onWindowPointerUp = (event: PointerEvent) => {
      if (!flowWidgetPalettePointerDragRef.active) return;
      resetFixtureDragDepth();
      endFlowWidgetPalettePointerDrag(event.clientX, event.clientY, dropHost(), (detail) => {
        commitWidgetDrop(detail);
      });
    };
    const onWindowPointerCancel = () => {
      if (!flowWidgetPalettePointerDragRef.active) return;
      resetFixtureDragDepth();
      cancelFlowWidgetPalettePointerDrag();
    };
    window.addEventListener("pointerup", onWindowPointerUp);
    window.addEventListener("pointercancel", onWindowPointerCancel);
    return () => {
      window.removeEventListener("pointerup", onWindowPointerUp);
      window.removeEventListener("pointercancel", onWindowPointerCancel);
    };
  }, [commitWidgetDrop, fixtureDragDrop, resetFixtureDragDepth]);

  return (
    <div
      ref={containerRef}
      tabIndex={0}
      className={className ?? `relative h-full min-h-0 w-full min-w-0 bg-canvas outline-none${fixtureDragActive ? " ring-2 ring-inset ring-accent" : ""}`}
      onDragEnter={onDragEnter}
      onDragLeave={onDragLeave}
      onDragOver={onDragOver}
      onDrop={onDrop}
    >
      <canvas
        ref={canvasRef}
        className="absolute inset-0 block h-full w-full touch-none"
        aria-hidden
        onPointerDown={onPointerDown}
        onPointerMove={onPointerMove}
        onPointerLeave={onPointerLeave}
        onPointerUp={onPointerUp}
        onPointerCancel={onPointerUp}
        onDoubleClick={onCanvasDoubleClick}
        onContextMenu={onCanvasContextMenu}
      />
      <canvas
        ref={textOverlayRef}
        aria-hidden
        className="pointer-events-none absolute inset-0 block h-full w-full"
        data-testid="flow-text-overlay"
      />
      <input
        ref={imageFileInputRef}
        type="file"
        accept="image/*"
        className="hidden"
        tabIndex={-1}
        aria-hidden
        onChange={onImageFileChange}
      />
      <ContextMenuController
        items={surfaceContextMenu?.items ?? []}
        onOpenChange={(open) => {
          if (!open) setSurfaceContextMenu(null);
        }}
        open={surfaceContextMenu !== null}
        position={surfaceContextMenu ? { x: surfaceContextMenu.clientX, y: surfaceContextMenu.clientY } : null}
      />
      {marqueeOverlay?.shape === "rect" && marqueeOverlay.rect ? (
        <SelectionMarquee coverage={marqueeOverlay.coverage} shape="rect" rect={marqueeOverlay.rect} />
      ) : null}
      {marqueeOverlay?.shape === "polygon" && marqueeOverlay.points ? (
        <SelectionMarquee coverage={marqueeOverlay.coverage} shape="polygon" points={marqueeOverlay.points} />
      ) : null}
      {spotlight && sessionRef.current ? (
        <FlowSpotlight
          anchor={spotlight}
          sections={catalogueSectionsRef.current}
          session={sessionRef.current}
          onCommit={commitWidgetDrop}
          onClose={closeSpotlight}
          renderFrame={renderFrame}
        />
      ) : null}
    </div>
  );
}
// #endregion 🔖FlowCanvas

// #region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("flow label overlay", () => {
    it("flowWorldToScreen maps world center to viewport center at zoom 1", () => {
      const screen = flowWorldToScreen({ x: 0, y: 0 }, { x: 0, y: 0, zoom: 1 }, 800, 600);
      expect(screen).toEqual({ x: 400, y: 300 });
    });

    it("flowOverlayLabelFill uses element default, emphasized when selected or hovered", () => {
      const element = resolveSemanticColorHex("border-element-color", "gray");
      const emphasized = resolveSemanticColorHex("border-emphasized-color", "dark");
      const idle = flowElementInteractionChrome([], { ids: [], removedIds: [] });
      const selected = flowElementInteractionChrome(["node-a"], { ids: [], removedIds: [] });
      const preview = flowElementInteractionChrome([], { ids: ["node-a"], removedIds: [] });
      expect(flowOverlayLabelFill("node-a", null, idle, [])).toBe(element);
      expect(flowOverlayLabelFill("node-a", "node-a", idle, [])).toBe(emphasized);
      expect(flowOverlayLabelFill("node-a", null, selected, [])).toBe(emphasized);
      expect(flowOverlayLabelFill("node-a", null, preview, [])).toBe(emphasized);
      expect(flowOverlayLabelFill("node-a", null, idle, ["node-a"])).toBe(element);
    });
  });

  describe("flow react fixture", () => {
    it("default fixture serializes", () => {
      const json = flowFixtureToJson(FLOW_DEFAULT_FIXTURE);
      expect(json).toContain("flow.fixture/v1");
      expect(json).toContain("math.add");
    });
  });

  describe("flow store", () => {
    it("round-trips fixture json", () => {
      const backing = new Map<string, string>();
      const store = createLocalFlowStore({
        getItem: (k) => backing.get(k) ?? null,
        setItem: (k, v) => {
          backing.set(k, v);
        },
        removeItem: (k) => {
          backing.delete(k);
        },
      });
      const json = flowFixtureToJson(FLOW_DEFAULT_FIXTURE);
      store.save(json);
      expect(store.load()).toBe(json);
      store.clear();
      expect(store.load()).toBeNull();
    });
  });

  describe("flow extension host", () => {
    it("parses module manifest", () => {
      const manifest = parseFlowModuleManifest(
        JSON.stringify({
          schema: "flow.module/v1",
          id: "math",
          name: "Math",
          version: "0.1.0",
          activationEvents: ["onStartup"],
          contributes: {
            neuronKinds: [{ id: "math.add", module: "math", name: "Add", abbreviation: "Add", icon: "emoji:➕", summary: "Sum", inputs: ["a"], outputs: ["number"] }],
            widgets: [],
            commands: [],
            settings: [],
          },
        }),
      );
      expect(manifest.id).toBe("math");
      expect(manifest.contributes.neuronKinds[0]?.id).toBe("math.add");
    });

    it("aggregates active catalogue sections", async () => {
      const host = new FlowExtensionHost();
      await host.activate("math");
      const sections = host.catalogueSections();
      expect(sections.some((section) => section.id === "math")).toBe(true);
      expect(JSON.stringify(sections)).toContain("math.add");
      await host.deactivate("math");
      expect(host.catalogueSections().some((section) => section.id === "math")).toBe(false);
    });
  });

  describe("flow spotlight slider query", () => {
    it("parses a single number into a sensible slider range", () => {
      const spec = flowParseSpotlightSliderQuery("5");
      expect(spec).toEqual({ value: 5, min: 0, max: 10, step: 1, label: "5" });
    });

    it("parses decimal single numbers with matching step", () => {
      const spec = flowParseSpotlightSliderQuery("10.2");
      expect(spec?.value).toBe(10.2);
      expect(spec?.min).toBe(0);
      expect(spec?.max).toBe(20);
      expect(spec?.step).toBe(0.1);
    });

    it("parses min..max range with decimal precision", () => {
      const spec = flowParseSpotlightSliderQuery("10.2..15");
      expect(spec).toEqual({ value: 10.2, min: 10.2, max: 15, step: 0.1, label: "10.2–15" });
    });

    it("builds slider descriptor json", () => {
      const spec = flowParseSpotlightSliderQuery("10.2..15");
      expect(spec).not.toBeNull();
      expect(flowSpotlightSliderDescriptor(spec!)).toBe('{"kind":"inputSlider","value":10.2,"min":10.2,"max":15,"step":0.1}');
    });

    it("ignores non-numeric spotlight queries", () => {
      expect(flowParseSpotlightSliderQuery("mul")).toBeNull();
    });
  });

  describe("flow spotlight note query", () => {
    const sections: CatalogueSection[] = [
      {
        id: "math",
        title: "Math",
        items: [{ kind: "neuron", neuronKind: "math.multiply", name: "Multiply", abbreviation: "Mul", icon: "emoji:✖️", summary: "Product" }],
      },
    ];

    it("parses free text into a note spec", () => {
      expect(flowParseSpotlightNoteQuery("some text", sections)).toEqual({ text: "some text", label: "some text" });
    });

    it("defers to catalogue matches for function search", () => {
      expect(flowParseSpotlightNoteQuery("mul", sections)).toBeNull();
    });

    it("builds note descriptor json", () => {
      const spec = flowParseSpotlightNoteQuery("some text", sections);
      expect(spec).not.toBeNull();
      expect(flowSpotlightNoteDescriptor(spec!)).toBe('{"kind":"inputNote","text":"some text"}');
    });
  });

  describe("flow catalogue suggestions", () => {
    const sections: CatalogueSection[] = [
      {
        id: "math",
        title: "Math",
        items: [
          { kind: "neuron", neuronKind: "math.add", name: "Add", abbreviation: "Add", icon: "emoji:➕", summary: "Sum" },
          { kind: "neuron", neuronKind: "math.multiply", name: "Multiply", summary: "Product" },
        ],
      },
      {
        id: "inputs",
        title: "Inputs",
        items: [{ kind: "inputSlider", name: "Slider", summary: "Number input" }],
      },
    ];

    it("ranks prefix query to top match", () => {
      const ranked = flowRankCatalogueSuggestions(sections, "mul");
      expect(ranked[0]?.name).toBe("Multiply");
    });

    it("prefers neurons in empty query", () => {
      const ranked = flowRankCatalogueSuggestions(sections, "");
      expect(ranked[0]?.kind).toBe("neuron");
    });

    it("returns default ordering for empty query", () => {
      const ranked = flowRankCatalogueSuggestions(sections, "");
      expect(ranked.length).toBe(3);
    });
  });

  describe("flow catalogue descriptor", () => {
    it("builds neuron descriptor", () => {
      const item: CatalogueItem = { kind: "neuron", neuronKind: "math.add", name: "Add", abbreviation: "Add", icon: "emoji:➕", summary: "Sum" };
      expect(flowCatalogueItemDescriptor(item)).toContain("math.add");
    });

    it("round-trips drag payload", () => {
      const item: CatalogueItem = { kind: "neuron", neuronKind: "math.add", name: "Add", abbreviation: "Add", icon: "emoji:➕", summary: "Sum" };
      const encoded = encodeFlowWidgetDescriptorForDragV1(flowCatalogueItemDescriptor(item));
      expect(decodeFlowWidgetDescriptorFromDragV1(encoded)).toContain("math.add");
    });
  });

  describe("flow context menu", () => {
    const baseCtx: FlowCanvasContextMenuContext = {
      hoveredNodeId: null,
      selectedNodeIds: [],
      isImageWidget: false,
      isBackground: true,
      previewOffNodeIds: [],
      screen: { x: 100, y: 200 },
      world: { x: 1, y: 2 },
      clientX: 300,
      clientY: 400,
    };

    it("background menu includes add node and reorganize", () => {
      const commands: string[] = [];
      const items = buildFlowContextMenuItems(baseCtx, (command) => commands.push(command));
      expect(items.some((item) => item.id === "flow.ctx.add")).toBe(true);
      expect(items.some((item) => item.id === "flow.ctx.reorganize")).toBe(true);
      expect(items.some((item) => item.id === "flow.ctx.delete")).toBe(false);
      items.find((item) => item.id === "flow.ctx.add")?.onSelect?.(new Event("click"));
      expect(commands).toEqual(["canvasCommand"]);
    });

    it("node menu includes delete and preview toggle", () => {
      const items = buildFlowContextMenuItems(
        { ...baseCtx, hoveredNodeId: "node-a", isBackground: false, selectedNodeIds: ["node-a"] },
        () => {},
      );
      expect(items.some((item) => item.id === "flow.ctx.delete")).toBe(true);
      expect(items.some((item) => item.id === "flow.ctx.preview")).toBe(true);
      expect(items.some((item) => item.id === "flow.ctx.add")).toBe(false);
    });

    it("image widget menu includes replace image", () => {
      const items = buildFlowContextMenuItems(
        { ...baseCtx, hoveredNodeId: "img", isBackground: false, isImageWidget: true, selectedNodeIds: ["img"] },
        () => {},
      );
      expect(items.some((item) => item.id === "flow.ctx.replaceImage")).toBe(true);
    });
  });

  describe("flow interaction helpers", () => {
    it("parses widget id arrays from session json", () => {
      expect(parseFlowWidgetIdArray('["a","b"]')).toEqual(["a", "b"]);
      expect(parseFlowWidgetIdArray("invalid")).toEqual([]);
    });

    it("compares widget id arrays in order", () => {
      expect(flowWidgetIdArraysEqual(["a"], ["a"])).toBe(true);
      expect(flowWidgetIdArraysEqual(["a"], ["b"])).toBe(false);
    });

    it("parses preselect json", () => {
      const snap = parseFlowPreselectJson(JSON.stringify({ ids: ["a"], removedIds: ["b"] }));
      expect(snap.ids).toEqual(["a"]);
      expect(snap.removedIds).toEqual(["b"]);
    });

    it("maps preselect preview chrome for label overlays", () => {
      const idle = flowElementInteractionChrome(["committed"], { ids: [], removedIds: [] });
      expect([...idle.selectedIds]).toEqual(["committed"]);
      expect([...idle.highlightedIds]).toEqual([]);
      const live = flowElementInteractionChrome(["committed"], { ids: ["preview"], removedIds: ["exit"] });
      expect([...live.selectedIds]).toEqual(["preview"]);
      expect([...live.highlightedIds]).toEqual(["exit"]);
    });

    it("parses selection preview points", () => {
      const points = parseFlowSelectionPreviewPoints(JSON.stringify([[0, 1], [3, 4]]));
      expect(points).toEqual([
        { x: 0, y: 1 },
        { x: 3, y: 4 },
      ]);
    });

    it("treats scroll wheel as zoom", () => {
      expect(flowWheelGestureIsZoom({ ctrlKey: false, metaKey: false })).toBe(true);
      expect(flowWheelGestureIsZoom({ ctrlKey: true, metaKey: false })).toBe(true);
    });
  });
}
// #endregion 🧪Tests
