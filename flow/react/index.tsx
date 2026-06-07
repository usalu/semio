// #region 🧲Header
/** @emoji 🌊 `@flow/react` — WASM flow renderer + React canvas. */
// #endregion 🧲Header

import React, { useCallback, useEffect, useRef, useState } from "react";
import { ContextMenuController, SelectionMarquee, type ContextMenuItem, type SelectionMarqueeCoverage } from "@ui/react";
import { serializeGraphVelloThemePaletteJson } from "@ui/styling";
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
  | { readonly kind: "inputSlider"; readonly id: string; readonly value: number }
  | { readonly kind: "inputNote"; readonly id: string; readonly text: string }
  | { readonly kind: "outputPreview"; readonly id: string }
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

// #region 🔖Spotlight
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
  const suggestions = flowRankCatalogueSuggestions(sections, query);
  const visible = expanded ? suggestions : suggestions.slice(0, 1);
  const hasMore = suggestions.length > 1;

  const syncGhost = useCallback(
    (item: CatalogueItem | undefined) => {
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

  const commitItem = useCallback(
    (item: CatalogueItem) => {
      const descriptor = flowCatalogueItemDescriptor(item);
      session.clearGhostWidget();
      onCommit({ descriptor, screen: anchor.screen, world: anchor.world });
      onClose();
    },
    [anchor.screen, anchor.world, onClose, onCommit, session],
  );

  useEffect(() => {
    inputRef.current?.focus();
  }, []);

  useEffect(() => {
    setActiveIndex(0);
  }, [query]);

  useEffect(() => {
    syncGhost(suggestions[activeIndex]);
  }, [activeIndex, suggestions, syncGhost]);

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
          placeholder="Add function…"
          className="min-w-0 flex-1 bg-transparent text-sm outline-none placeholder:text-muted"
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
                  className={`flex w-full flex-col gap-0.5 px-3 py-1.5 text-left text-sm ${active ? "bg-accent/15 text-accent" : "hover:bg-accent/10"}`}
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
  readonly contextMenu?: readonly ContextMenuItem[];
  readonly onContextMenu?: (detail: { readonly clientX: number; readonly clientY: number; readonly hoveredNodeId: string | null }) => void;
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
  automaticLod = true,
  lod,
  onLodChange,
}: FlowCanvasProps): React.JSX.Element {
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
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
  const onContextMenuRef = useRef(onContextMenu);
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
      session.setVelloThemeJson(serializeGraphVelloThemePaletteJson());
    } catch {
      /* document theme tokens not ready */
    }
  }, []);

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
    onSelectionChangeRef.current?.(selected);
    onPreselectChangeRef.current?.(preselect);
    onHoverChangeRef.current?.(hovered);
    syncMarqueeOverlay(session);
    console.log(`[DEBUG] flow interaction selected=[${selected.join(", ")}] preselect=[${preselect.ids.join(", ")}] hover=${hovered ?? "—"}`);
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
      sessionRef.current?.renderFrame();
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
      if (!session || !pointerRef.current.active || pointerRef.current.id !== e.pointerId) return;
      const { x, y } = clientToCanvas(e.clientX, e.clientY);
      session.pointerMoveScreen(x, y, e.shiftKey, e.metaKey || e.ctrlKey, e.altKey);
      emitInteractionState(session);
      renderFrame();
    },
    [clientToCanvas, emitInteractionState, renderFrame],
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

  const onCanvasDoubleClick = useCallback(
    (e: React.MouseEvent<HTMLCanvasElement>) => {
      const session = sessionRef.current;
      if (!session) return;
      e.preventDefault();
      const screen = clientToCanvas(e.clientX, e.clientY);
      const world = JSON.parse(session.worldFromScreen(screen.x, screen.y)) as { x: number; y: number };
      setSpotlight({ screen, world });
    },
    [clientToCanvas],
  );

  const syncWheelZoomActive = useCallback((active: boolean) => {
    try {
      sessionRef.current?.setWheelZoomActive(active);
    } catch {
      /* gpu not ready */
    }
  }, []);

  const onWheel = useCallback(
    (e: React.WheelEvent<HTMLCanvasElement>) => {
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

  const onCanvasContextMenu = useCallback(
    (e: React.MouseEvent<HTMLCanvasElement>) => {
      if (e.altKey) {
        e.preventDefault();
        return;
      }
      e.preventDefault();
      const session = sessionRef.current;
      const hoveredNodeId = session?.hoveredWidgetId() ?? null;
      onContextMenuRef.current?.({ clientX: e.clientX, clientY: e.clientY, hoveredNodeId });
      const items = contextMenu ?? [];
      if (items.length > 0) {
        setSurfaceContextMenu({ clientX: e.clientX, clientY: e.clientY, items });
      }
    },
    [contextMenu],
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
        onPointerDown={onPointerDown}
        onPointerMove={onPointerMove}
        onPointerUp={onPointerUp}
        onPointerCancel={onPointerUp}
        onDoubleClick={onCanvasDoubleClick}
        onWheel={onWheel}
        onContextMenu={onCanvasContextMenu}
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
