// #region 🧲Header
/** @emoji 🌊 `@flow/react` — WASM flow renderer + React canvas. */
// #endregion 🧲Header

import React, { useCallback, useEffect, useRef, useState } from "react";
import { serializeGraphVelloThemePaletteJson } from "@ui/styling";
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
  ] = await Promise.all([
    import("../modules/math/pkg/flow_module_math.js"),
    import("../modules/text/pkg/flow_module_text.js"),
    import("../modules/logic/pkg/flow_module_logic.js"),
    import("../modules/dictionary/pkg/flow_module_dictionary.js"),
  ]);
  initMathSync({ module: readFileSync(join(reactDir, "../modules/math/pkg/flow_module_math_bg.wasm")) });
  initTextSync({ module: readFileSync(join(reactDir, "../modules/text/pkg/flow_module_text_bg.wasm")) });
  initLogicSync({ module: readFileSync(join(reactDir, "../modules/logic/pkg/flow_module_logic_bg.wasm")) });
  initDictionarySync({ module: readFileSync(join(reactDir, "../modules/dictionary/pkg/flow_module_dictionary_bg.wasm")) });
} else {
  await initFlowWasm();
}

export async function ensureFlowWasmLoaded(): Promise<void> {
  await initFlowWasm();
}

export { FlowSession };
// #endregion 🔖GpuWasmBridge

// #region 🔖ExtensionHost
export interface FlowModuleNeuronKindV1 {
  readonly id: string;
  readonly module: string;
  readonly name: string;
  readonly summary: string;
  readonly inputs: readonly string[];
  readonly outputs: readonly string[];
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
};

export const FLOW_DEFAULT_MODULE_IDS = ["math", "text", "logic", "dictionary"] as const;
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
  readonly synapses: readonly { readonly id: string; readonly from: string; readonly to: string }[];
  readonly layout?: Readonly<Record<string, { readonly x: number; readonly y: number }>>;
}

export type FlowWidgetV1 =
  | { readonly kind: "neuron"; readonly id: string; readonly neuronKind: string }
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
    { id: "s1", from: "slider", to: "add" },
    { id: "s2", from: "add", to: "preview" },
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
// #endregion 🔖Catalogue

export interface FlowReorganizeRequest {
  readonly epoch: number;
  readonly optionsJson: string;
}

// #region 🔖FlowCanvas
export interface FlowCanvasProps {
  readonly fixtureJson?: string;
  readonly className?: string;
  readonly store?: FlowStore;
  readonly fixtureDragDrop?: boolean;
  readonly reorganize?: FlowReorganizeRequest;
  readonly extensionRevision?: number;
  readonly extensionHost?: FlowExtensionHost;
  readonly onPreviewText?: (text: string) => void;
  readonly onFixtureChange?: (fixtureJson: string) => void;
  readonly onCatalogueReady?: (sections: readonly CatalogueSection[]) => void;
  readonly onWidgetDrop?: (detail: FlowWidgetDropDetail) => void;
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
  onFixtureChange,
  onCatalogueReady,
  onWidgetDrop,
}: FlowCanvasProps): React.JSX.Element {
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const sessionRef = useRef<FlowSession | null>(null);
  const rafRef = useRef<number | null>(null);
  const onPreviewTextRef = useRef(onPreviewText);
  const onFixtureChangeRef = useRef(onFixtureChange);
  const onCatalogueReadyRef = useRef(onCatalogueReady);
  const onWidgetDropRef = useRef(onWidgetDrop);
  const storeRef = useRef(store ?? createLocalFlowStore());
  const pointerRef = useRef({ active: false, pan: false, id: -1 });
  const fixtureDragDepthRef = useRef(0);
  const lastVelloThemeJsonRef = useRef("");
  const [fixtureDragActive, setFixtureDragActive] = useState(false);

  const syncVelloTheme = useCallback(() => {
    if (typeof document === "undefined") return;
    try {
      const json = serializeGraphVelloThemePaletteJson();
      if (json !== lastVelloThemeJsonRef.current) {
        lastVelloThemeJsonRef.current = json;
        sessionRef.current?.setVelloThemeJson(json);
      }
    } catch {
      lastVelloThemeJsonRef.current = "";
    }
  }, []);

  useEffect(() => {
    onPreviewTextRef.current = onPreviewText;
  }, [onPreviewText]);

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
    if (store) storeRef.current = store;
  }, [store]);

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
      syncVelloTheme();
      sessionRef.current?.renderFrame();
    } catch {
      /* gpu not ready */
    }
  }, [syncVelloTheme]);

  const evaluate = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    session.evaluate();
    const text = session.previewText();
    onPreviewTextRef.current?.(text);
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
      const sections = parseFlowCatalogueSections(session.catalogueJson());
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
      const json = saved ?? fixtureJson ?? flowFixtureToJson(FLOW_DEFAULT_FIXTURE);
      session.loadFixtureJson(json);
      syncExtensionSurface(session);
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
      const tick = () => {
        renderFrame();
        rafRef.current = requestAnimationFrame(tick);
      };
      rafRef.current = requestAnimationFrame(tick);
      cleanupResize = () => {
        ro.disconnect();
        if (rafRef.current != null) cancelAnimationFrame(rafRef.current);
      };
    })();
    return () => {
      cancelled = true;
      cleanupResize?.();
      if (rafRef.current != null) cancelAnimationFrame(rafRef.current);
      sessionRef.current = null;
    };
  }, [evaluate, extensionHost, fixtureJson, renderFrame, syncExtensionSurface]);

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
      e.currentTarget.setPointerCapture(e.pointerId);
      pointerRef.current = { active: true, pan: e.button === 1 || e.shiftKey, id: e.pointerId };
      const { x, y } = clientToCanvas(e.clientX, e.clientY);
      session.pointerDownScreen(x, y, e.metaKey || e.ctrlKey, pointerRef.current.pan);
      renderFrame();
    },
    [clientToCanvas, renderFrame],
  );

  const onPointerMove = useCallback(
    (e: React.PointerEvent<HTMLCanvasElement>) => {
      const session = sessionRef.current;
      if (!session || !pointerRef.current.active || pointerRef.current.id !== e.pointerId) return;
      const { x, y } = clientToCanvas(e.clientX, e.clientY);
      session.pointerMoveScreen(x, y);
      renderFrame();
    },
    [clientToCanvas, renderFrame],
  );

  const onPointerUp = useCallback(
    (e: React.PointerEvent<HTMLCanvasElement>) => {
      const session = sessionRef.current;
      if (!session || pointerRef.current.id !== e.pointerId) return;
      const { x, y } = clientToCanvas(e.clientX, e.clientY);
      session.pointerUpScreen(x, y);
      pointerRef.current = { active: false, pan: false, id: -1 };
      evaluate();
      persistFixture();
      renderFrame();
    },
    [clientToCanvas, evaluate, persistFixture, renderFrame],
  );

  const onWheel = useCallback(
    (e: React.WheelEvent<HTMLCanvasElement>) => {
      const session = sessionRef.current;
      if (!session) return;
      e.preventDefault();
      const { x, y } = clientToCanvas(e.clientX, e.clientY);
      session.wheel(x, y, e.deltaY);
      persistFixture();
      renderFrame();
    },
    [clientToCanvas, persistFixture, renderFrame],
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
      className={className ?? `relative h-full min-h-0 w-full min-w-0 bg-canvas${fixtureDragActive ? " ring-2 ring-inset ring-accent" : ""}`}
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
        onWheel={onWheel}
      />
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
            neuronKinds: [{ id: "math.add", module: "math", name: "Add", summary: "Sum", inputs: ["a"], outputs: ["number"] }],
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

  describe("flow catalogue descriptor", () => {
    it("builds neuron descriptor", () => {
      const item: CatalogueItem = { kind: "neuron", neuronKind: "math.add", name: "Add", summary: "Sum" };
      expect(flowCatalogueItemDescriptor(item)).toContain("math.add");
    });

    it("round-trips drag payload", () => {
      const item: CatalogueItem = { kind: "neuron", neuronKind: "math.add", name: "Add", summary: "Sum" };
      const encoded = encodeFlowWidgetDescriptorForDragV1(flowCatalogueItemDescriptor(item));
      expect(decodeFlowWidgetDescriptorFromDragV1(encoded)).toContain("math.add");
    });
  });
}
// #endregion 🧪Tests
