// #region 🧲Header
/** @emoji 🌊 `@semio-tech/flow-react` — WASM flow renderer + React canvas. */
// #endregion 🧲Header

import React, { useCallback, useEffect, useRef, useState } from "react";
import { borderNormalBottomClass, canvasViewportClass, cn, CanvasPickMenu, ContextMenuController, floatingMenuItemClass, floatingMenuSurfaceClass, floatingToolbarSurfaceClass, Icon, menuListItemClassName, SelectionMarquee, useCanvasPickInteraction, useVelloThemeSync, type CanvasPickTarget, type ContextMenuItem, type ScreenRect, type SelectionMarqueeCoverage } from "@semio-tech/ui-react";
import { parseCanvasPickTargetKey } from "@semio-tech/framework-core";
import { resolveColorHex, resolveSemanticColorHex, syncSessionVelloTheme, tokenVar } from "@semio-tech/ui-styling";
import { isDagDrawLodKind, type DagDrawLodKind } from "@semio-tech/dag-react";
import initFlowWasm, { FlowSession, initSync } from "../core/pkg/flow_core.js";
import flowCoreWasmUrl from "../core/pkg/flow_core_bg.wasm?url";
import { FlowOrchestratorClient } from "../worker-client.ts";

// #region 🔖GpuWasmBridge
if (import.meta.env.VITEST) {
  const { readFileSync } = await import("node:fs");
  const { dirname, join } = await import("node:path");
  const { fileURLToPath } = await import("node:url");
  const reactDir = dirname(fileURLToPath(import.meta.url));
  initSync({ module: readFileSync(join(reactDir, "../core/pkg/flow_core_bg.wasm")) });
  const [
    { initSync: initCoreSync },
    { initSync: initMathSync },
    { initSync: initTextSync },
    { initSync: initLogicSync },
    { initSync: initDictionarySync },
    { initSync: initListSync },
    { initSync: initBrepSync },
    { initSync: initBimSync },
    { initSync: initDrawSync },
  ] = await Promise.all([
    import("../module/core/pkg/flow_module_core.js"),
    import("../module/math/pkg/flow_module_math.js"),
    import("../module/text/pkg/flow_module_text.js"),
    import("../module/logic/pkg/flow_module_logic.js"),
    import("../module/dictionary/pkg/flow_module_dictionary.js"),
    import("../module/list/pkg/flow_module_list.js"),
    import("../module/brep/pkg/flow_module_brep.js"),
    import("../module/bim/pkg/flow_module_bim.js"),
    import("../module/draw/pkg/flow_module_draw.js"),
  ]);
  initCoreSync({ module: readFileSync(join(reactDir, "../module/core/pkg/flow_module_core_bg.wasm")) });
  initMathSync({ module: readFileSync(join(reactDir, "../module/math/pkg/flow_module_math_bg.wasm")) });
  initTextSync({ module: readFileSync(join(reactDir, "../module/text/pkg/flow_module_text_bg.wasm")) });
  initLogicSync({ module: readFileSync(join(reactDir, "../module/logic/pkg/flow_module_logic_bg.wasm")) });
  initDictionarySync({ module: readFileSync(join(reactDir, "../module/dictionary/pkg/flow_module_dictionary_bg.wasm")) });
  initListSync({ module: readFileSync(join(reactDir, "../module/list/pkg/flow_module_list_bg.wasm")) });
  initBrepSync({ module: readFileSync(join(reactDir, "../module/brep/pkg/flow_module_brep_bg.wasm")) });
  initBimSync({ module: readFileSync(join(reactDir, "../module/bim/pkg/flow_module_bim_bg.wasm")) });
  initDrawSync({ module: readFileSync(join(reactDir, "../module/draw/pkg/flow_module_draw_bg.wasm")) });
} else {
  await initFlowWasm({ module_or_path: flowCoreWasmUrl });
}

export async function ensureFlowWasmLoaded(): Promise<void> {
  await initFlowWasm({ module_or_path: flowCoreWasmUrl });
}

export { FlowSession };
export { FlowOrchestratorClient, createFlowOrchestratorWorker, type FlowEvalWorkerResult } from "../worker-client.ts";
export {
  defaultComputeWorkerCount,
  effectiveComputeWorkerCount,
  initFlowThreadPool,
  isCrossOriginIsolatedRuntime,
  readStoredComputeWorkerCount,
} from "../compute.ts";

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
} from "@semio-tech/dag-react";
// #endregion 🔖GpuWasmBridge

// #region 🔖ExtensionHost
export interface FlowModuleVariadicSpecV1 {
  readonly slotKey: string;
  readonly min: number;
  readonly max?: number;
}

export interface FlowValueTypeV1 {
  readonly kind: string;
  readonly of?: FlowValueTypeV1 | string;
}

export interface FlowChannelSpecV1 {
  readonly name: string;
  readonly code: string;
  readonly abbreviation: string;
  readonly fullName: string;
  readonly operators: readonly string[];
  readonly default?: unknown;
  readonly label?: string;
  readonly cardinality?: string;
}

function flowCardinalityRange(cardinality: string | undefined): readonly [number, number | null] {
  const symbol = cardinality?.trim() || "!";
  if (symbol === "!") return [1, 1];
  if (symbol === "?") return [0, 1];
  if (symbol === "*") return [0, null];
  if (symbol === "+") return [1, null];
  const exact = Number(symbol);
  if (Number.isInteger(exact) && exact >= 0) return [exact, exact];
  return [1, 1];
}

function flowCardinalityRangeContains(input: string | undefined, output: string | undefined): boolean {
  const [inMin, inMax] = flowCardinalityRange(input);
  const [outMin, outMax] = flowCardinalityRange(output);
  if (outMin < inMin) return false;
  if (inMax === null) return true;
  if (outMax === null) return false;
  return outMax <= inMax;
}

export function flowChannelCompatible(output: FlowChannelSpecV1, input: FlowChannelSpecV1): boolean {
  if (!flowCardinalityRangeContains(input.cardinality, output.cardinality)) return false;
  if (!input.operators.length) return true;
  const provided = new Set(output.operators);
  return input.operators.every((required) => provided.has(required));
}

/** @emoji 🧾 Formats a flow eval channel value for display, including null and error lists. */
export function formatFlowEvalValue(value: unknown): string {
  if (value === null || value === undefined) return "null";
  if (typeof value === "string") return value;
  if (typeof value === "number" || typeof value === "boolean") return String(value);
  if (!Array.isArray(value) && typeof value === "object") {
    const record = value as Record<string, unknown>;
    if (record.$schema === "list") {
      const messages = Object.keys(record)
        .filter((key) => /^\d+$/.test(key))
        .sort((left, right) => Number(left) - Number(right))
        .map((key) => formatFlowEvalValue(record[key]))
        .filter((entry) => entry.length > 0 && entry !== "null");
      return messages.length ? messages.join("; ") : "—";
    }
    if (record.$schema === "text" && typeof record.value === "string") return record.value;
    if (record.value !== undefined && Object.keys(record).length <= 2) return formatFlowEvalValue(record.value);
  }
  try {
    return JSON.stringify(value);
  } catch {
    return "—";
  }
}

/** @emoji ⚠️ Extracts schema-component error messages from an eval output port map. */
export function readFlowEvalErrors(outPorts: Readonly<Record<string, unknown>> | null | undefined): readonly string[] {
  if (!outPorts) return [];
  const errors = outPorts.errors;
  if (!errors || typeof errors !== "object" || Array.isArray(errors)) return [];
  const record = errors as Record<string, unknown>;
  if (record.$schema !== "list") return [];
  return Object.keys(record)
    .filter((key) => /^\d+$/.test(key))
    .sort((left, right) => Number(left) - Number(right))
    .map((key) => formatFlowEvalValue(record[key]))
    .filter((entry) => entry.length > 0 && entry !== "null");
}

export interface FlowModuleSchemaFieldV1 {
  readonly key: string;
  readonly value: FlowValueTypeV1;
  readonly default?: unknown;
  readonly label?: string;
}

export interface FlowModuleSchemaV1 {
  readonly id: string;
  readonly module: string;
  readonly name: string;
  readonly icon: string;
  readonly summary: string;
  readonly fields: readonly FlowModuleSchemaFieldV1[];
}

export interface FlowModuleOperatorInfoV1 {
  readonly id: string;
  readonly module: string;
  readonly name: string;
  readonly abbreviation: string;
  readonly icon: string;
  readonly summary: string;
  readonly inputs: readonly FlowChannelSpecV1[];
  readonly outputs: readonly FlowChannelSpecV1[];
  readonly group?: readonly string[];
  readonly variadicInput?: FlowModuleVariadicSpecV1;
  readonly variadicOutput?: FlowModuleVariadicSpecV1;
}

export type FlowModuleNeuronKindV1 = FlowModuleOperatorInfoV1;

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
    readonly schemas: readonly FlowModuleSchemaV1[];
    readonly operators: readonly FlowModuleOperatorInfoV1[];
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

type FlowWasmInitInput = string | URL | Request | { readonly module_or_path?: string | URL | Request };
type FlowWasmInit = (input?: FlowWasmInitInput) => Promise<unknown>;

async function loadFlowWasmModule(
  jsModule: Promise<FlowModulePackage & { default?: FlowWasmInit }>,
  wasmUrlModule: Promise<{ default: string }>,
): Promise<FlowModulePackage> {
  const [mod, { default: wasmUrl }] = await Promise.all([jsModule, wasmUrlModule]);
  if (mod.default) await mod.default({ module_or_path: wasmUrl });
  return mod;
}

const FLOW_MODULE_LOADERS: Record<string, FlowModuleLoader> = {
  core: () => loadFlowWasmModule(import("../module/core/pkg/flow_module_core.js"), import("../module/core/pkg/flow_module_core_bg.wasm?url")),
  math: () => loadFlowWasmModule(import("../module/math/pkg/flow_module_math.js"), import("../module/math/pkg/flow_module_math_bg.wasm?url")),
  text: () => loadFlowWasmModule(import("../module/text/pkg/flow_module_text.js"), import("../module/text/pkg/flow_module_text_bg.wasm?url")),
  logic: () => loadFlowWasmModule(import("../module/logic/pkg/flow_module_logic.js"), import("../module/logic/pkg/flow_module_logic_bg.wasm?url")),
  dictionary: () =>
    loadFlowWasmModule(import("../module/dictionary/pkg/flow_module_dictionary.js"), import("../module/dictionary/pkg/flow_module_dictionary_bg.wasm?url")),
  list: () => loadFlowWasmModule(import("../module/list/pkg/flow_module_list.js"), import("../module/list/pkg/flow_module_list_bg.wasm?url")),
  brep: () => loadFlowWasmModule(import("../module/brep/pkg/flow_module_brep.js"), import("../module/brep/pkg/flow_module_brep_bg.wasm?url")),
  draw: () => loadFlowWasmModule(import("../module/draw/pkg/flow_module_draw.js"), import("../module/draw/pkg/flow_module_draw_bg.wasm?url")),
  bim: () => loadFlowWasmModule(import("../module/bim/pkg/flow_module_bim.js"), import("../module/bim/pkg/flow_module_bim_bg.wasm?url")),
};

export const FLOW_DEFAULT_MODULE_IDS = ["core", "math", "text", "logic", "dictionary", "list", "brep", "draw", "bim"] as const;
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
    for (const kind of manifest.contributes.operators) {
      this.kindToModule.set(kind.id, id);
    }
    console.log(`[DEBUG] flow extension activated: ${id}`);
    this.notify();
  }

  async deactivate(id: string): Promise<void> {
    const entry = this.active.get(id);
    if (!entry) return;
    entry.glue.deactivate();
    for (const kind of entry.manifest.contributes.operators) {
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
      const section = nestNeuronKindsIntoCatalogueSection(id, entry.manifest.name, entry.manifest.contributes.operators);
      if ((section.items ?? []).length === 0 && !(section.groups?.length ?? 0)) continue;
      sections.push(section);
    }
    return sections;
  }

  catalogueJson(): string {
    return JSON.stringify(this.catalogueSections());
  }

  kindInfosJson(): string {
    const kinds: FlowModuleNeuronKindV1[] = [];
    for (const entry of this.active.values()) {
      kinds.push(...entry.manifest.contributes.operators);
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
      contributes: { schemas: [], operators: [], widgets: [], commands: [], settings: [] },
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

export interface FlowDocumentV1 {
  readonly schema: "flow.document/v1";
  readonly flow: FlowGuiV1;
  readonly tree: FlowTreeV1;
}

export interface FlowGuiV1 {
  readonly camera: { readonly x: number; readonly y: number; readonly zoom: number };
  readonly nodes: Readonly<Record<string, FlowNodeGuiV1>>;
  readonly previews?: readonly FlowPreviewGuiV1[];
}

export interface FlowNodeGuiV1 {
  readonly layout: { readonly x: number; readonly y: number };
  readonly chrome: FlowNodeChromeV1;
}

export type FlowNodeChromeV1 =
  | { readonly kind: "plain" }
  | { readonly kind: "slider"; readonly min: number; readonly max: number; readonly step: number }
  | { readonly kind: "note" }
  | { readonly kind: "image" }
  | { readonly kind: "variable" };

export interface FlowSchemaRefV1 {
  readonly id: string;
  readonly name: string;
  readonly icon: string;
}

export interface FlowPreviewGuiV1 {
  readonly id: string;
  readonly source?: { readonly neuron: string; readonly channel: string };
  readonly mode: string;
}

export interface FlowTreeV1 {
  readonly neurons: readonly { readonly id: string; readonly kind: string; readonly params?: unknown }[];
  readonly synapses: readonly { readonly id: string; readonly from: string; readonly to: string; readonly fromPort?: string; readonly toPort?: string }[];
}

export type FlowWidgetV1 =
  | { readonly kind: "neuron"; readonly id: string; readonly neuronKind: string; readonly inputPorts?: readonly string[]; readonly outputPorts?: readonly string[] }
  | { readonly kind: "inputSlider"; readonly id: string; readonly value: number; readonly min?: number; readonly max?: number; readonly step?: number }
  | { readonly kind: "inputStepper"; readonly id: string; readonly schema: string; readonly fields?: readonly { readonly key: string; readonly value: number }[]; readonly step?: number }
  | { readonly kind: "inputNote"; readonly id: string; readonly text: string }
  | { readonly kind: "inputImage"; readonly id: string; readonly src?: string }
  | { readonly kind: "variable"; readonly id: string; readonly name: string; readonly schema: string }
  | { readonly kind: "outputPreview"; readonly id: string; readonly expanded?: readonly string[] }
  | { readonly kind: "outputAction"; readonly id: string; readonly action: string }
  | { readonly kind: "cluster"; readonly id: string; readonly name?: string; readonly tree: FlowTreeV1; readonly flow?: FlowGuiV1 };

export const FLOW_DEFAULT_FIXTURE: FlowFixtureV1 = {
  schema: "flow.fixture/v1",
  camera: { x: 0, y: 0, zoom: 1 },
  widgets: [
    { kind: "inputSlider", id: "slider", value: 3 },
    { kind: "neuron", id: "add", neuronKind: "math.add" },
    { kind: "outputPreview", id: "preview" },
  ],
  synapses: [
    { id: "s1", from: "slider", to: "add", fromPort: "number", toPort: "a" },
    { id: "s2", from: "add", to: "preview", fromPort: "sum", toPort: "" },
  ],
};

export function flowFixtureToJson(fixture: FlowFixtureV1): string {
  return JSON.stringify(fixture);
}

export type FlowFixtureEditOp =
  | { readonly op: "setDocument"; readonly document: FlowFixtureV1 }
  | { readonly op: "renameWidget"; readonly oldId: string; readonly newId: string }
  | { readonly op: "patchWidget"; readonly widgetId: string; readonly field: string; readonly value: unknown }
  | { readonly op: "patchWidgets"; readonly widgetIds: readonly string[]; readonly field: string; readonly value: unknown };

/** @emoji 🚪 Applies one semantic flow fixture edit (CQRS projection applier). */
export function applyFlowFixtureEditOp(fixture: FlowFixtureV1, op: FlowFixtureEditOp): FlowFixtureV1 {
  switch (op.op) {
    case "setDocument":
      return op.document;
    case "renameWidget": {
      const trimmed = op.newId.trim();
      if (!trimmed || trimmed === op.oldId || fixture.widgets.some((widget) => widget.id === trimmed)) {
        return fixture;
      }
      const widgets = fixture.widgets.map((widget) =>
        widget.id === op.oldId ? ({ ...widget, id: trimmed } as FlowWidgetV1) : widget,
      );
      const synapses = fixture.synapses.map((synapse) => ({
        ...synapse,
        from: synapse.from === op.oldId ? trimmed : synapse.from,
        to: synapse.to === op.oldId ? trimmed : synapse.to,
      }));
      if (!fixture.layout) {
        return { ...fixture, widgets, synapses };
      }
      const layout: Record<string, { readonly x: number; readonly y: number }> = {};
      for (const [key, value] of Object.entries(fixture.layout)) {
        layout[key === op.oldId ? trimmed : key] = value;
      }
      return { ...fixture, widgets, synapses, layout };
    }
    case "patchWidget":
      return applyFlowFixtureEditOp(fixture, {
        op: "patchWidgets",
        widgetIds: [op.widgetId],
        field: op.field,
        value: op.value,
      });
    case "patchWidgets": {
      const targets = new Set(op.widgetIds);
      const widgets = fixture.widgets.map((widget) => {
        if (!targets.has(widget.id)) return widget;
        if (op.field === "value" || op.field === "min" || op.field === "max" || op.field === "step") {
          const numeric = typeof op.value === "number" ? op.value : Number(op.value);
          if (!Number.isFinite(numeric)) return widget;
          return { ...widget, [op.field]: numeric } as FlowWidgetV1;
        }
        if (typeof op.value !== "string") return widget;
        return { ...widget, [op.field]: op.value } as FlowWidgetV1;
      });
      return { ...fixture, widgets };
    }
  }
}

/** @emoji ↩️ Inverts a flow fixture edit from the pre-apply projection. */
export function backwardsFlowFixtureEditOp(fixture: FlowFixtureV1, op: FlowFixtureEditOp): readonly FlowFixtureEditOp[] {
  switch (op.op) {
    case "setDocument":
      return [{ op: "setDocument", document: fixture }];
    case "renameWidget":
      return [{ op: "renameWidget", oldId: op.newId, newId: op.oldId }];
    case "patchWidget": {
      const widget = fixture.widgets.find((row) => row.id === op.widgetId);
      if (!widget) return [{ op: "setDocument", document: fixture }];
      return [{ op: "patchWidget", widgetId: op.widgetId, field: op.field, value: (widget as Record<string, unknown>)[op.field] }];
    }
    case "patchWidgets":
      return op.widgetIds.flatMap((widgetId) => {
        const widget = fixture.widgets.find((row) => row.id === widgetId);
        if (!widget) return [];
        return [{ op: "patchWidget", widgetId, field: op.field, value: (widget as Record<string, unknown>)[op.field] }];
      });
  }
}

/** @emoji 📊 Returns the flow fixture edit payload for persistence diffs. */
export function diffFlowFixtureEditOp(_fixture: FlowFixtureV1, operation: FlowFixtureEditOp): unknown {
  return operation;
}

/** @emoji 🧠 Neuron widget ids from a flow fixture JSON blob. */
export function neuronWidgetIdsFromFixtureJson(fixtureJson: string): string[] {
  try {
    const fixture = JSON.parse(fixtureJson) as { readonly widgets?: readonly { readonly kind?: string; readonly id?: string }[] };
    return (fixture.widgets ?? [])
      .filter((widget) => widget.kind === "neuron" && typeof widget.id === "string")
      .map((widget) => widget.id as string);
  } catch {
    return [];
  }
}

export interface FlowTreeDirtyResult {
  readonly ids: readonly string[];
  readonly path: readonly string[];
  readonly structural: boolean;
}

function canonicalizeFlowValue(value: unknown): unknown {
  if (value === null || value === undefined) return value;
  if (Array.isArray(value)) return value.map(canonicalizeFlowValue);
  if (typeof value === "object") {
    const sorted: Record<string, unknown> = {};
    for (const key of Object.keys(value as Record<string, unknown>).sort()) {
      sorted[key] = canonicalizeFlowValue((value as Record<string, unknown>)[key]);
    }
    return sorted;
  }
  return value;
}

function flowWidgetTreeSignature(widget: FlowWidgetV1): string {
  switch (widget.kind) {
    case "neuron":
      return JSON.stringify(
        canonicalizeFlowValue({
          neuronKind: widget.neuronKind,
          params: (widget as { readonly params?: unknown }).params ?? {},
          inputPorts: widget.inputPorts ?? [],
          outputPorts: widget.outputPorts ?? [],
        }),
      );
    case "inputSlider":
      return JSON.stringify(canonicalizeFlowValue({ value: widget.value }));
    case "inputStepper":
      return JSON.stringify(canonicalizeFlowValue({ schema: widget.schema, fields: widget.fields ?? [] }));
    case "inputNote":
      return JSON.stringify(canonicalizeFlowValue({ text: widget.text }));
    case "inputImage":
      return JSON.stringify(canonicalizeFlowValue({ src: widget.src ?? "" }));
    case "cluster":
      return JSON.stringify(canonicalizeFlowValue({ tree: widget.tree }));
    default:
      return JSON.stringify(canonicalizeFlowValue({ kind: widget.kind }));
  }
}

function parseFlowFixtureForDirtyDiff(json: string): { readonly widgets: readonly FlowWidgetV1[]; readonly synapses: FlowFixtureV1["synapses"] } | null {
  try {
    const fixture = JSON.parse(json) as FlowFixtureV1;
    if (!Array.isArray(fixture.widgets)) return null;
    return { widgets: fixture.widgets, synapses: fixture.synapses ?? [] };
  } catch {
    return null;
  }
}

function incomingSynapseKeys(widgetId: string, synapses: FlowFixtureV1["synapses"]): string[] {
  return synapses
    .filter((synapse) => synapse.to === widgetId)
    .map((synapse) => `${synapse.from}|${synapse.fromPort ?? ""}|${synapse.toPort ?? ""}`)
    .sort();
}

function isComputeFlowWidget(widget: FlowWidgetV1): boolean {
  return widget.kind === "neuron" || widget.kind === "cluster";
}

function downstreamAdjacency(synapses: FlowFixtureV1["synapses"]): Map<string, string[]> {
  const adjacency = new Map<string, string[]>();
  for (const synapse of synapses) {
    const next = adjacency.get(synapse.from) ?? [];
    next.push(synapse.to);
    adjacency.set(synapse.from, next);
  }
  return adjacency;
}

function downstreamSubgraphWidgetIds(
  roots: Iterable<string>,
  fixture: { readonly widgets: readonly FlowWidgetV1[]; readonly synapses: FlowFixtureV1["synapses"] },
): string[] {
  const adjacency = downstreamAdjacency(fixture.synapses);
  const visited = new Set<string>();
  const queue = [...roots];
  while (queue.length > 0) {
    const id = queue.shift()!;
    if (visited.has(id)) continue;
    visited.add(id);
    for (const next of adjacency.get(id) ?? []) {
      if (!visited.has(next)) queue.push(next);
    }
  }
  return [...visited];
}

function downstreamComputeWidgetIds(
  roots: Iterable<string>,
  fixture: { readonly widgets: readonly FlowWidgetV1[]; readonly synapses: FlowFixtureV1["synapses"] },
): string[] {
  const widgetById = new Map(fixture.widgets.map((widget) => [widget.id, widget]));
  return downstreamSubgraphWidgetIds(roots, fixture).filter((id) => {
    const widget = widgetById.get(id);
    return widget != null && isComputeFlowWidget(widget);
  });
}

function topoSortWidgetIds(widgetIds: readonly string[], synapses: FlowFixtureV1["synapses"]): string[] {
  const ids = new Set(widgetIds);
  const inDegree = new Map<string, number>();
  const adjacency = new Map<string, string[]>();
  for (const id of ids) {
    inDegree.set(id, 0);
    adjacency.set(id, []);
  }
  for (const synapse of synapses) {
    if (!ids.has(synapse.from) || !ids.has(synapse.to)) continue;
    adjacency.get(synapse.from)!.push(synapse.to);
    inDegree.set(synapse.to, (inDegree.get(synapse.to) ?? 0) + 1);
  }
  const queue = [...ids].filter((id) => inDegree.get(id) === 0).sort();
  const order: string[] = [];
  while (queue.length > 0) {
    const id = queue.shift()!;
    order.push(id);
    for (const next of adjacency.get(id) ?? []) {
      const degree = (inDegree.get(next) ?? 1) - 1;
      inDegree.set(next, degree);
      if (degree === 0) queue.push(next);
    }
    queue.sort();
  }
  return order.length === ids.size ? order : [...ids];
}

function flowFixtureComputePath(fixture: { readonly widgets: readonly FlowWidgetV1[]; readonly synapses: FlowFixtureV1["synapses"] }): string[] {
  return topoSortWidgetIds(
    fixture.widgets.map((widget) => widget.id),
    fixture.synapses,
  );
}

function flowDirtyComputePath(
  roots: Iterable<string>,
  fixture: { readonly widgets: readonly FlowWidgetV1[]; readonly synapses: FlowFixtureV1["synapses"] },
): string[] {
  return topoSortWidgetIds(downstreamSubgraphWidgetIds(roots, fixture), fixture.synapses);
}

/** @emoji 🌳 Predictive dirty neuron ids from a fixture diff (changed node + downstream). */
export function flowTreeDirtyNeuronIds(prevFixtureJson: string | null, currFixtureJson: string): FlowTreeDirtyResult {
  const curr = parseFlowFixtureForDirtyDiff(currFixtureJson);
  if (!curr) return { ids: [], path: [], structural: true };
  if (!prevFixtureJson) return { ids: [], path: flowFixtureComputePath(curr), structural: true };
  const prev = parseFlowFixtureForDirtyDiff(prevFixtureJson);
  if (!prev) return { ids: [], path: flowFixtureComputePath(curr), structural: true };

  const prevWidgets = new Map(prev.widgets.map((widget) => [widget.id, widget]));
  const currWidgetIds = new Set(curr.widgets.map((widget) => widget.id));
  const dirtyRoots = new Set<string>();
  for (const widget of curr.widgets) {
    const previous = prevWidgets.get(widget.id);
    if (!previous) {
      dirtyRoots.add(widget.id);
      continue;
    }
    if (flowWidgetTreeSignature(widget) !== flowWidgetTreeSignature(previous)) dirtyRoots.add(widget.id);
    if (JSON.stringify(incomingSynapseKeys(widget.id, prev.synapses)) !== JSON.stringify(incomingSynapseKeys(widget.id, curr.synapses))) {
      dirtyRoots.add(widget.id);
    }
  }
  for (const widget of prev.widgets) {
    if (currWidgetIds.has(widget.id)) continue;
    for (const synapse of prev.synapses) {
      if (synapse.from === widget.id && currWidgetIds.has(synapse.to)) dirtyRoots.add(synapse.to);
    }
  }
  if (dirtyRoots.size === 0) return { ids: [], path: [], structural: false };
  return {
    ids: downstreamComputeWidgetIds(dirtyRoots, curr),
    path: flowDirtyComputePath(dirtyRoots, curr),
    structural: false,
  };
}

export function flowComputeProgressPayload(path: readonly string[], activeIndex: number): { readonly active: string | null; readonly stale: string[] } {
  const active = path[activeIndex] ?? null;
  return { active, stale: path.slice(activeIndex + 1) };
}

function flowEvalAnimationPath(
  path: readonly string[],
  fixture: { readonly widgets: readonly FlowWidgetV1[] },
): string[] {
  const widgetById = new Map(fixture.widgets.map((widget) => [widget.id, widget]));
  return path.filter((id) => {
    const widget = widgetById.get(id);
    return widget != null && widget.kind !== "outputPreview" && widget.kind !== "outputAction";
  });
}

function flowIncomingSynapseCount(widgetId: string, synapses: FlowFixtureV1["synapses"]): number {
  return synapses.filter((synapse) => synapse.to === widgetId).length;
}

/** @emoji ✅ True when every dirty compute node in the path can evaluate without missing upstream inputs. */
export function flowDirtyComputePathReady(fixtureJson: string, dirtyPath: readonly string[]): boolean {
  const fixture = parseFlowFixtureForDirtyDiff(fixtureJson);
  if (!fixture || dirtyPath.length === 0) return true;
  const widgetById = new Map(fixture.widgets.map((widget) => [widget.id, widget]));
  for (const widgetId of dirtyPath) {
    const widget = widgetById.get(widgetId);
    if (!widget) continue;
    if (widget.kind === "inputSlider" || widget.kind === "inputNote" || widget.kind === "inputImage" || widget.kind === "inputStepper") continue;
    if (widget.kind === "outputPreview" || widget.kind === "outputAction") continue;
    if (widget.kind === "neuron") {
      const incoming = flowIncomingSynapseCount(widgetId, fixture.synapses);
      if (incoming > 0) continue;
      const params = (widget as { readonly params?: Record<string, unknown> }).params ?? {};
      if (Object.keys(params).length > 0) continue;
      if (widget.neuronKind.startsWith("brep.prim") || widget.neuronKind.startsWith("core.")) continue;
      return false;
    }
    if (widget.kind === "cluster" && flowIncomingSynapseCount(widgetId, fixture.synapses) === 0) {
      return false;
    }
  }
  return true;
}

export function isGlobalFlowEvalErrorJson(json: string): boolean {
  try {
    const parsed = JSON.parse(json) as Record<string, unknown>;
    return Object.keys(parsed).length === 1 && typeof parsed.error === "string";
  } catch {
    return true;
  }
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

/** @emoji 🫥 In-memory flow store: no load, no save — host owns persistence. */
export function createEphemeralFlowStore(): FlowStore {
  return {
    load(): string | null {
      return null;
    },
    save(): void {},
    clear(): void {},
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

export interface CatalogueGroup {
  readonly id: string;
  readonly title: string;
  readonly items: readonly CatalogueItem[];
  readonly groups?: readonly CatalogueGroup[];
}

export interface CatalogueSection {
  readonly id: string;
  readonly title: string;
  readonly items: readonly CatalogueItem[];
  readonly groups?: readonly CatalogueGroup[];
}

export interface CatalogueKindsTreeItem {
  readonly id: string;
  readonly label: string;
  readonly description?: string;
  readonly defaultOpen?: boolean;
  readonly draggable?: boolean;
  readonly dragData?: Readonly<Record<string, string>>;
  readonly items?: readonly CatalogueKindsTreeItem[];
}

export interface CatalogueKindsTreeSection {
  readonly id: string;
  readonly label: string;
  readonly defaultOpen?: boolean;
  readonly items: readonly CatalogueKindsTreeItem[];
}

function slugifyCatalogueGroupSegment(title: string): string {
  const slug = title
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "");
  return slug.length > 0 ? slug : "group";
}

function catalogueGroupIdFromPath(sectionId: string, titlePath: readonly string[]): string {
  return `${sectionId}.${titlePath.map(slugifyCatalogueGroupSegment).join(".")}`;
}

/** @emoji 🧷 Maps a neuron kind manifest row to a draggable catalogue item. */
export function neuronKindToCatalogueItem(kind: FlowModuleNeuronKindV1): CatalogueItem {
  return {
    kind: "neuron",
    neuronKind: kind.id,
    name: kind.name,
    abbreviation: kind.abbreviation,
    icon: kind.icon,
    summary: kind.summary,
  };
}

interface CatalogueGroupBuilder {
  readonly title: string;
  items: CatalogueItem[];
  children: Map<string, CatalogueGroupBuilder>;
}

function getOrCreateCatalogueGroupBuilder(map: Map<string, CatalogueGroupBuilder>, title: string): CatalogueGroupBuilder {
  let builder = map.get(title);
  if (!builder) {
    builder = { title, items: [], children: new Map() };
    map.set(title, builder);
  }
  return builder;
}

function insertCatalogueItemIntoGroupTree(root: Map<string, CatalogueGroupBuilder>, groupPath: readonly string[], item: CatalogueItem): void {
  let map = root;
  let builder: CatalogueGroupBuilder | null = null;
  for (const title of groupPath) {
    builder = getOrCreateCatalogueGroupBuilder(builder === null ? map : builder.children, title);
  }
  builder?.items.push(item);
}

function buildCatalogueGroupsFromBuilders(
  builders: Map<string, CatalogueGroupBuilder>,
  sectionId: string,
  titlePath: readonly string[],
): CatalogueGroup[] {
  const groups: CatalogueGroup[] = [];
  for (const builder of builders.values()) {
    const path = [...titlePath, builder.title];
    const childGroups = buildCatalogueGroupsFromBuilders(builder.children, sectionId, path);
    groups.push({
      id: catalogueGroupIdFromPath(sectionId, path),
      title: builder.title,
      items: builder.items,
      ...(childGroups.length ? { groups: childGroups } : {}),
    });
  }
  return groups;
}

/** @emoji 🌳 Nests neuron kinds into a catalogue section using each kind's authored group path. */
export function nestNeuronKindsIntoCatalogueSection(id: string, title: string, kinds: readonly FlowModuleNeuronKindV1[]): CatalogueSection {
  const items: CatalogueItem[] = [];
  const rootGroups = new Map<string, CatalogueGroupBuilder>();
  for (const kind of kinds) {
    const item = neuronKindToCatalogueItem(kind);
    const groupPath = kind.group ?? [];
    if (groupPath.length === 0) {
      items.push(item);
      continue;
    }
    insertCatalogueItemIntoGroupTree(rootGroups, groupPath, item);
  }
  const groups = buildCatalogueGroupsFromBuilders(rootGroups, id, []);
  return {
    id,
    title,
    items,
    ...(groups.length ? { groups } : {}),
  };
}

function normalizeCatalogueGroup(group: CatalogueGroup): CatalogueGroup {
  return {
    ...group,
    items: group.items ?? [],
    groups: (group.groups ?? []).map(normalizeCatalogueGroup),
  };
}

/** @emoji 🧹 Ensures catalogue sections always expose array `items` and `groups` after JSON round-trips. */
export function normalizeCatalogueSection(section: CatalogueSection): CatalogueSection {
  return {
    ...section,
    items: section.items ?? [],
    groups: (section.groups ?? []).map(normalizeCatalogueGroup),
  };
}

function flattenCatalogueGroupItems(group: CatalogueGroup): CatalogueItem[] {
  const nested = (group.groups ?? []).flatMap(flattenCatalogueGroupItems);
  return [...(group.items ?? []), ...nested];
}

/** @emoji 📋 Flattens every draggable catalogue item from nested sections and groups. */
export function flattenCatalogueItems(sections: readonly CatalogueSection[]): CatalogueItem[] {
  return sections.flatMap((section) => [...(section.items ?? []), ...(section.groups ?? []).flatMap(flattenCatalogueGroupItems)]);
}

function catalogueItemsToKindsTreeItems(
  items: readonly CatalogueItem[] | undefined,
  idPrefix: string,
  sectionId: string,
  dragDataFn: (item: CatalogueItem) => Record<string, string>,
  itemIndex: { value: number },
): CatalogueKindsTreeItem[] {
  return (items ?? []).map((item) => {
    const index = itemIndex.value++;
    return {
      id: `${idPrefix}.${sectionId}.${index}.${item.neuronKind ?? item.kind}`,
      label: item.name,
      description: item.summary,
      draggable: true,
      dragData: dragDataFn(item),
    };
  });
}

function catalogueGroupsToKindsTreeItems(
  groups: readonly CatalogueGroup[] | undefined,
  idPrefix: string,
  sectionId: string,
  dragDataFn: (item: CatalogueItem) => Record<string, string>,
  itemIndex: { value: number },
): CatalogueKindsTreeItem[] {
  return (groups ?? []).map((group) => ({
    id: `${idPrefix}.${sectionId}.group.${group.id}`,
    label: group.title,
    defaultOpen: false,
    items: [
      ...catalogueItemsToKindsTreeItems(group.items, idPrefix, sectionId, dragDataFn, itemIndex),
      ...catalogueGroupsToKindsTreeItems(group.groups, idPrefix, sectionId, dragDataFn, itemIndex),
    ],
  }));
}

/** @emoji 🌲 Builds recursive workbench tree sections from nested catalogue sections. */
export function buildCatalogueKindsTreeSections(
  sections: readonly CatalogueSection[],
  idPrefix: string,
  dragDataFn: (item: CatalogueItem) => Record<string, string> = flowPlayCatalogueItemDragData,
): CatalogueKindsTreeSection[] {
  return sections.map((section) => {
    const normalized = normalizeCatalogueSection(section);
    const itemIndex = { value: 0 };
    return {
      id: `${idPrefix}.${normalized.id}`,
      label: normalized.title,
      defaultOpen: false,
      items: [
        ...catalogueGroupsToKindsTreeItems(normalized.groups, idPrefix, normalized.id, dragDataFn, itemIndex),
        ...catalogueItemsToKindsTreeItems(normalized.items, idPrefix, normalized.id, dragDataFn, itemIndex),
      ],
    };
  });
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
export const flowWidgetPaletteDragEncodedRef = { current: null as string | null };
export const flowWidgetPaletteDragClientRef = { clientX: 0, clientY: 0 };
export const flowWidgetPaletteDropCommittedRef = { current: false };
export const flowWidgetDropPointerToWorldRef = {
  current: null as ((clientX: number, clientY: number) => { screen: { x: number; y: number }; world: { x: number; y: number } } | null) | null,
};
export const flowWidgetPaletteDragGhostRef = {
  current: null as ((clientX: number, clientY: number, descriptor: string | null) => void) | null,
};

let flowPaletteDragPreviewRafId: number | null = null;

function flowStopPaletteDragPreviewLoop(): void {
  if (flowPaletteDragPreviewRafId !== null) {
    globalThis.cancelAnimationFrame?.(flowPaletteDragPreviewRafId);
    flowPaletteDragPreviewRafId = null;
  }
}

function flowSyncPaletteDragGhostAtClient(clientX: number, clientY: number): void {
  const sync = flowWidgetPaletteDragGhostRef.current;
  if (!sync) {
    return;
  }
  const encoded = flowReadActivePaletteDragEncoded();
  if (!encoded) {
    sync(clientX, clientY, null);
    return;
  }
  sync(clientX, clientY, decodeFlowWidgetDescriptorFromDragV1(encoded));
}

function flowTickPaletteDragPreview(): void {
  const encoded = flowReadActivePaletteDragEncoded();
  if (!encoded) {
    flowStopPaletteDragPreviewLoop();
    return;
  }
  const { clientX, clientY } = flowWidgetPaletteDragClientRef;
  flowSyncPaletteDragGhostAtClient(clientX, clientY);
  const requestFrame = globalThis.requestAnimationFrame?.bind(globalThis);
  if (!requestFrame) {
    flowPaletteDragPreviewRafId = null;
    return;
  }
  flowPaletteDragPreviewRafId = requestFrame(flowTickPaletteDragPreview);
}

function flowStartPaletteDragPreviewLoop(): void {
  if (flowPaletteDragPreviewRafId !== null) {
    return;
  }
  flowTickPaletteDragPreview();
}

/** @emoji 📦 Reads the encoded palette drag payload when a workbench widget drag is active. */
export function flowReadActivePaletteDragEncoded(): string | null {
  const pointer = flowWidgetPalettePointerDragRef.encoded?.trim();
  if (pointer) {
    return pointer;
  }
  const shared = flowWidgetPaletteDragEncodedRef.current?.trim();
  return shared ? shared : null;
}

/** @emoji 👻 Notes palette-drag client coordinates and mirrors the WASM ghost on the flow canvas. */
export function flowNotePaletteWidgetDragClient(clientX: number, clientY: number): void {
  flowWidgetPaletteDragClientRef.clientX = clientX;
  flowWidgetPaletteDragClientRef.clientY = clientY;
  if (!flowReadActivePaletteDragEncoded()) {
    return;
  }
  flowSyncPaletteDragGhostAtClient(clientX, clientY);
  flowStartPaletteDragPreviewLoop();
}

/** @emoji ⎋ Aborts an in-flight workbench palette widget drag and clears the canvas ghost. */
export function abortFlowWidgetPaletteDrag(): void {
  const wasActive = flowWidgetPalettePointerDragRef.active || flowWidgetPaletteDragRef.active;
  flowWidgetPalettePointerDragRef.active = false;
  flowWidgetPalettePointerDragRef.encoded = null;
  flowWidgetPaletteDragEncodedRef.current = null;
  flowWidgetPaletteDragRef.active = false;
  if (wasActive) {
    window.dispatchEvent(new CustomEvent("flow-widget-drag-session", { detail: null }));
  }
  flowStopPaletteDragPreviewLoop();
  flowWidgetPaletteDragGhostRef.current?.(flowWidgetPaletteDragClientRef.clientX, flowWidgetPaletteDragClientRef.clientY, null);
}

/** @emoji 🖱️ Begins pointer palette drag with an encoded widget descriptor. */
export function beginFlowWidgetPalettePointerDrag(encoded: string): void {
  flowWidgetPaletteDropCommittedRef.current = false;
  flowWidgetPalettePointerDragRef.active = true;
  flowWidgetPalettePointerDragRef.encoded = encoded;
  flowWidgetPaletteDragEncodedRef.current = encoded;
  flowWidgetPaletteDragRef.active = true;
  window.dispatchEvent(new CustomEvent("flow-widget-drag-session", { detail: { encoded } }));
  flowStartPaletteDragPreviewLoop();
}

/** @emoji 🖱️ Ends pointer palette drag without committing a drop. */
export function cancelFlowWidgetPalettePointerDrag(): void {
  if (!flowWidgetPalettePointerDragRef.active && !flowWidgetPaletteDragRef.active) {
    return;
  }
  flowWidgetPalettePointerDragRef.active = false;
  flowWidgetPalettePointerDragRef.encoded = null;
  flowWidgetPaletteDragEncodedRef.current = null;
  flowWidgetPaletteDragRef.active = false;
  flowStopPaletteDragPreviewLoop();
  flowWidgetPaletteDragGhostRef.current?.(flowWidgetPaletteDragClientRef.clientX, flowWidgetPaletteDragClientRef.clientY, null);
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
): import("@semio-tech/framework-platform-core").TreeDragAndDropController {
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
      flowWidgetPaletteDropCommittedRef.current = false;
      flowWidgetPaletteDragRef.active = true;
      const payload = readEncoded(dragDataByItemId.get(sourceItem.id));
      if (payload) {
        flowWidgetPaletteDragEncodedRef.current = payload;
        window.dispatchEvent(new CustomEvent("flow-widget-drag-session", { detail: { encoded: payload } }));
        flowStartPaletteDragPreviewLoop();
      }
    },
    onDragEnd: () => {
      if (flowWidgetPalettePointerDragRef.active) return;
      flowWidgetPaletteDragEncodedRef.current = null;
      flowWidgetPaletteDragRef.active = false;
      if (!flowWidgetPaletteDropCommittedRef.current) {
        flowStopPaletteDragPreviewLoop();
        flowWidgetPaletteDragGhostRef.current?.(flowWidgetPaletteDragClientRef.clientX, flowWidgetPaletteDragClientRef.clientY, null);
      }
      flowWidgetPaletteDropCommittedRef.current = false;
      window.dispatchEvent(new CustomEvent("flow-widget-drag-session", { detail: null }));
    },
  };
}

export function parseFlowCatalogueSections(json: string): CatalogueSection[] {
  try {
    const parsed = JSON.parse(json) as CatalogueSection[];
    return Array.isArray(parsed) ? parsed.map(normalizeCatalogueSection) : [];
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
  if (kind === `${q}.${q}`) return 0;
  if (name.startsWith(q) || kind.startsWith(q)) return 1;
  if (name.includes(q) || kind.includes(q)) return 2;
  if (flowCatalogueItemSearchText(item).includes(q)) return 3;
  return -1;
}

/** @emoji 📶 Spotlight row label/detail matching DAG node label tiers. */
export function flowCatalogueItemLodDisplay(
  item: CatalogueItem,
  lod: DagDrawLodKind,
): { readonly primary: string; readonly detail: string | null; readonly showIcon: boolean } {
  const abbrev = item.abbreviation || item.name;
  const summary = item.summary?.trim() || null;
  switch (lod) {
    case "minimap":
    case "overview":
      return { primary: abbrev, detail: null, showIcon: true };
    case "compact":
    case "detail":
      return { primary: abbrev, detail: lod === "detail" ? summary : null, showIcon: false };
    case "normal":
      return { primary: item.name, detail: null, showIcon: false };
    case "micro":
      return { primary: item.name, detail: summary, showIcon: false };
  }
}

function flowSpotlightLodChrome(lod: DagDrawLodKind): {
  readonly textClass: string;
  readonly itemPy: string;
  readonly maxListH: string;
  readonly panelClass: string;
} {
  switch (lod) {
    case "minimap":
    case "overview":
      return {
        textClass: "text-xs",
        itemPy: "py-0.5",
        maxListH: "max-h-[min(12rem,50vh)]",
        panelClass: "min-w-[8rem] max-w-[11rem]",
      };
    case "compact":
    case "detail":
      return {
        textClass: "text-xs",
        itemPy: "py-0.5",
        maxListH: "max-h-[min(16rem,60vh)]",
        panelClass: "min-w-[9rem] max-w-[13rem]",
      };
    default:
      return {
        textClass: "text-sm",
        itemPy: "py-1",
        maxListH: "max-h-[min(24rem,70vh)]",
        panelClass: "min-w-[11rem] max-w-[16rem]",
      };
  }
}

/** @emoji 🔍 Scroll container classes for expanded flow spotlight suggestions. */
export function flowSpotlightSuggestionListScrollClass(expanded: boolean, lod: DagDrawLodKind): string {
  const chrome = flowSpotlightLodChrome(lod);
  return cn("min-h-0 overscroll-contain", expanded ? cn("overflow-y-auto", chrome.maxListH) : "overflow-hidden");
}

/** @emoji 🔍 Ranks catalogue items for flow canvas spotlight search. */
export function flowRankCatalogueSuggestions(sections: readonly CatalogueSection[], query: string): CatalogueItem[] {
  const items = flattenCatalogueItems(sections);
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

export type FlowGraphEditOp =
  | { readonly op: "addWidget"; readonly descriptor: string; readonly x: number; readonly y: number }
  | { readonly op: "connectPorts"; readonly from: string; readonly fromPort: string; readonly to: string; readonly toPort: string }
  | { readonly op: "disconnect"; readonly synapseId: string }
  | {
      readonly op: "insertBetween";
      readonly anchor: string;
      readonly anchorOutPort: string;
      readonly mid: string;
      readonly midInPort: string;
      readonly midOutPort: string;
    }
  | { readonly op: "moveWidget"; readonly id: string; readonly x: number; readonly y: number }
  | { readonly op: "makeSpace"; readonly anchor: string; readonly dx: number; readonly dy?: number }
  | { readonly op: "setPreviewOff"; readonly ids: readonly string[] }
  | { readonly op: "setSliderValue"; readonly id: string; readonly value: number }
  | { readonly op: "setStepperFieldValue"; readonly id: string; readonly key: string; readonly value: number }
  | { readonly op: "setVariableName"; readonly id: string; readonly name: string }
  | { readonly op: "setVariableSchema"; readonly id: string; readonly schema: string }
  | { readonly op: "setNeuronParams"; readonly id: string; readonly paramsJson: string }
  | { readonly op: "collapse"; readonly ids: readonly string[] }
  | { readonly op: "explode"; readonly id: string };

/** @emoji 🔧 Applies a batched list of generic flow graph edit primitives. */
export function runFlowGraphEdit(session: FlowSession, ops: readonly FlowGraphEditOp[]): void {
  for (const entry of ops) {
    switch (entry.op) {
      case "addWidget":
        session.addWidget(entry.descriptor, entry.x, entry.y);
        break;
      case "connectPorts":
        session.connectPorts(entry.from, entry.fromPort, entry.to, entry.toPort);
        break;
      case "disconnect":
        session.disconnect(entry.synapseId);
        break;
      case "insertBetween":
        session.insertBetween(entry.anchor, entry.anchorOutPort, entry.mid, entry.midInPort, entry.midOutPort);
        break;
      case "moveWidget":
        session.moveWidget(entry.id, entry.x, entry.y);
        break;
      case "makeSpace":
        session.makeSpace(entry.anchor, entry.dx, entry.dy ?? 0);
        break;
      case "setPreviewOff": {
        const current = parseFlowWidgetIdArray(session.previewOffWidgetIds());
        const off = new Set([...current, ...entry.ids]);
        session.setPreviewOff(JSON.stringify([...off]));
        break;
      }
      case "setSliderValue":
        session.setSliderValue(entry.id, entry.value);
        break;
      case "setStepperFieldValue":
        (session as FlowSessionStepperApi).setStepperFieldValue(entry.id, entry.key, entry.value);
        break;
      case "setVariableName":
        (session as FlowSessionVariableApi).setVariableName(entry.id, entry.name);
        break;
      case "setVariableSchema":
        (session as FlowSessionVariableApi).setVariableSchema(entry.id, entry.schema);
        break;
      case "setNeuronParams":
        session.setNeuronParams(entry.id, entry.paramsJson);
        break;
      case "collapse":
        (session as FlowSessionClusterApi).collapseSelection(JSON.stringify(entry.ids));
        break;
      case "explode":
        (session as FlowSessionClusterApi).explodeCluster(entry.id);
        break;
      default:
        break;
    }
  }
}

export interface FlowCanvasContextMenuContext {
  readonly hoveredNodeId: string | null;
  readonly selectedNodeIds: readonly string[];
  readonly clusterNodeIds: readonly string[];
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
    if (targetIds.length >= 2) {
      items.push({
        id: "flow.ctx.collapse",
        label: "Collapse to cluster",
        icon: "layers",
        onSelect: () => dispatch("canvasCommand", { command: "collapse", argsJson: JSON.stringify({ ids: targetIds }) }),
      });
    }
    if (targetIds.length === 1) {
      const clusterId = targetIds[0];
      if (ctx.clusterNodeIds.includes(clusterId)) {
        items.push({
          id: "flow.ctx.explode",
          label: "Explode cluster",
          icon: "expand",
          onSelect: () => dispatch("canvasCommand", { command: "explode", argsJson: JSON.stringify({ id: clusterId }) }),
        });
      }
    }
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

/** @emoji 🎚️ Derives a tight min/max/step for a slider from a typed number token. */
export function flowSensibleSliderRangeFromToken(
  token: string,
  value: number,
): { readonly min: number; readonly max: number; readonly step: number } {
  const step = flowSliderStepFromDecimalPlaces(flowDecimalPlacesFromNumberToken(token));
  if (value < 0) {
    const bound = flowSensibleSliderMax(value);
    return { min: -bound, max: bound, step };
  }
  return { min: 0, max: flowSensibleSliderMax(value), step };
}

/** @emoji 🎚️ Derives a tight min/max/step for a slider from its current value. */
export function flowSensibleSliderRange(value: number): { readonly min: number; readonly max: number; readonly step: number } {
  return flowSensibleSliderRangeFromToken(String(value), value);
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
  const { min, max, step } = flowSensibleSliderRangeFromToken(trimmed, value);
  return { value, min, max, step, label: trimmed };
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

const FLOW_SPOTLIGHT_VARIABLE_ITEM: CatalogueItem = {
  kind: "variable",
  name: "Variable",
  abbreviation: "Variable",
  icon: "emoji:🔣",
  summary: "Named typed dictionary",
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
  const listRef = useRef<HTMLDivElement>(null);
  const activeItemRef = useRef<HTMLButtonElement | null>(null);
  const [query, setQuery] = useState("");
  const [expanded, setExpanded] = useState(false);
  const [activeIndex, setActiveIndex] = useState(0);
  const drawLodLabel = session.drawLodLabel();
  const drawLod: DagDrawLodKind = isDagDrawLodKind(drawLodLabel) ? drawLodLabel : "normal";
  const lodChrome = flowSpotlightLodChrome(drawLod);
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
    if (!expanded) return;
    activeItemRef.current?.scrollIntoView({ block: "nearest" });
  }, [activeIndex, expanded]);

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
      className={cn("absolute z-20 flex min-h-0 flex-col", lodChrome.panelClass, floatingMenuSurfaceClass)}
      style={{ left: anchor.screen.x, top: anchor.screen.y }}
      onPointerDown={(event) => event.stopPropagation()}
      onDoubleClick={(event) => event.stopPropagation()}
      onWheel={(event) => event.stopPropagation()}
    >
      <div className={cn("flex shrink-0 items-center gap-1 px-2 py-1", borderNormalBottomClass)}>
        <input
          ref={inputRef}
          type="text"
          value={query}
          placeholder="Add function, number, or text…"
          className={cn("min-w-0 flex-1 bg-transparent text-foreground outline-none placeholder:text-muted", lodChrome.textClass)}
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
      <div
        ref={listRef}
        className={flowSpotlightSuggestionListScrollClass(expanded && hasMore, drawLod)}
        onWheel={(event) => event.stopPropagation()}
      >
        <ul className="py-0.5" role="listbox">
          {visible.length === 0 ? (
            <li className={cn("px-2 py-1 text-muted", lodChrome.textClass)}>No matches</li>
          ) : (
            visible.map((item, index) => {
              const globalIndex = expanded ? index : 0;
              const active = globalIndex === activeIndex;
              const display = flowCatalogueItemLodDisplay(item, drawLod);
              return (
                <li key={`${item.kind}-${item.neuronKind ?? item.action ?? item.name}`} role="option" aria-selected={active}>
                  <button
                    ref={active ? activeItemRef : undefined}
                    type="button"
                    className={cn(
                      floatingMenuItemClass,
                      "flex w-full min-w-0 items-center gap-1.5 px-2",
                      lodChrome.itemPy,
                      lodChrome.textClass,
                      active && "bg-active-base text-emphasized",
                    )}
                    onMouseEnter={() => setActiveIndex(globalIndex)}
                    onClick={() => commitItem(item)}
                  >
                    {display.showIcon ? <Icon icon={item.icon} size="tiny" className="shrink-0" /> : null}
                    <span className="min-w-0 truncate font-medium">{display.primary}</span>
                    {display.detail ? <span className="min-w-0 truncate text-muted">· {display.detail}</span> : null}
                  </button>
                </li>
              );
            })
          )}
        </ul>
      </div>
    </div>
  );
}
// #endregion 🔖Spotlight

// #region 🔖TextOverlay
const FLOW_LABEL_SCREEN_PX = 11;
const FLOW_LABEL_FONT_FAMILY = "ui-sans-serif, system-ui, sans-serif";

interface FlowLabelOverlayRow {
  readonly id: string;
  readonly kind?: "port" | "node";
  readonly text: string;
  readonly layout: "horizontal" | "vertical";
  readonly align?: "left" | "center" | "right";
  readonly x: number;
  readonly y: number;
  readonly nodeW: number;
  readonly nodeH: number;
  readonly fontScreenPx?: number;
  readonly maxScreenH?: number;
  readonly ghost?: boolean;
}

interface FlowLabelOverlayPaintState {
  readonly camera: { readonly x: number; readonly y: number; readonly zoom: number };
  readonly width: number;
  readonly height: number;
  readonly labels: readonly FlowLabelOverlayRow[];
}

interface FlowParamOverlayEditor {
  readonly nodeId: string;
  readonly portId: string;
  readonly label?: string;
  readonly type?: string;
  readonly value?: unknown;
  readonly default?: unknown;
  readonly x: number;
  readonly y: number;
  readonly w: number;
  readonly h: number;
}

interface FlowParamOverlayPaintState {
  readonly camera: { readonly x: number; readonly y: number; readonly zoom: number };
  readonly width: number;
  readonly height: number;
  readonly editors: readonly FlowParamOverlayEditor[];
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

function flowClampPortLabelFontPx(ctx: CanvasRenderingContext2D, text: string, targetPx: number, maxW: number, maxH: number): number {
  let px = Math.max(8, Math.round(targetPx));
  ctx.font = `${px}px ${FLOW_LABEL_FONT_FAMILY}`;
  if (ctx.measureText(text).width <= maxW && px * 1.25 <= maxH) {
    return px;
  }
  let low = 8;
  let high = px;
  let best = 8;
  while (low <= high) {
    const mid = Math.floor((low + high) / 2);
    ctx.font = `${mid}px ${FLOW_LABEL_FONT_FAMILY}`;
    if (ctx.measureText(text).width <= maxW) {
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
  ghost: boolean,
  hoveredId: string | null,
  chrome: { readonly selectedIds: Set<string>; readonly highlightedIds: Set<string> },
  previewOffIds: readonly string[],
): string {
  if (ghost) {
    return resolveColorHex(tokenVar("secondary"), "secondary");
  }
  if (previewOffIds.includes(nodeId)) {
    return resolveSemanticColorHex("border-element-color", "gray");
  }
  if (chrome.selectedIds.has(nodeId)) {
    return resolveSemanticColorHex("border-emphasized-color", "dark");
  }
  if (chrome.highlightedIds.has(nodeId)) {
    return resolveColorHex(tokenVar("secondary"), "secondary");
  }
  if (hoveredId === nodeId) {
    return resolveSemanticColorHex("border-emphasized-color", "dark");
  }
  return resolveSemanticColorHex("border-element-color", "gray");
}

function paramEditorScalar(editor: FlowParamOverlayEditor): string | number | boolean {
  const raw = editor.value ?? editor.default;
  if (editor.type === "boolean") return Boolean(raw);
  if (editor.type === "text") return typeof raw === "string" ? raw : String(raw ?? "");
  const n = Number(raw);
  return Number.isFinite(n) ? n : 0;
}

function FlowParamOverlay({
  state,
  width,
  height,
  onParamChange,
}: {
  readonly state: FlowParamOverlayPaintState | null;
  readonly width: number;
  readonly height: number;
  readonly onParamChange: (nodeId: string, portId: string, value: unknown) => void;
}): React.ReactElement | null {
  if (!state) return null;
  const camera = {
    x: Number(state.camera?.x) || 0,
    y: Number(state.camera?.y) || 0,
    zoom: Math.max(0.05, Number(state.camera?.zoom) || 1),
  };
  const viewportW = Number(state.width) || width;
  const viewportH = Number(state.height) || height;
  return (
    <div className="pointer-events-none absolute inset-0 z-30 overflow-visible" data-testid="flow-param-overlay">
      {state.editors.map((editor) => {
        const screen = flowWorldToScreen({ x: editor.x, y: editor.y }, camera, viewportW, viewportH);
        const w = Math.max(28, editor.w * camera.zoom);
        const h = Math.max(14, editor.h * camera.zoom);
        const left = screen.x - w * 0.5;
        const top = screen.y - h * 0.5;
        const key = `${editor.nodeId}:${editor.portId}`;
        if (editor.type === "boolean") {
          const checked = Boolean(paramEditorScalar(editor));
          return (
            <label
              key={key}
              className="pointer-events-auto absolute flex items-center gap-1 rounded border border-border bg-background/90 px-1 text-2xs text-foreground shadow-sm"
              style={{ left, top, width: w, height: h }}
            >
              <input
                type="checkbox"
                checked={checked}
                onChange={(event) => onParamChange(editor.nodeId, editor.portId, event.target.checked)}
              />
              <span>{editor.label?.trim() || editor.portId}</span>
            </label>
          );
        }
        const value = paramEditorScalar(editor);
        return (
          <input
            key={key}
            className="pointer-events-auto absolute rounded border border-border bg-background/90 px-1 text-2xs text-foreground shadow-sm"
            style={{ left, top, width: w, height: h }}
            type={editor.type === "text" ? "text" : "number"}
            value={typeof value === "string" ? value : String(value)}
            step={editor.type === "integer" ? 1 : "any"}
            onChange={(event) => {
              const next =
                editor.type === "text"
                  ? event.target.value
                  : editor.type === "integer"
                    ? Number.parseInt(event.target.value, 10) || 0
                    : Number.parseFloat(event.target.value) || 0;
              onParamChange(editor.nodeId, editor.portId, next);
            }}
          />
        );
      })}
    </div>
  );
}

interface FlowVariableOverlayEditor {
  readonly id: string;
  readonly name: string;
  readonly schema: string;
  readonly x: number;
  readonly y: number;
  readonly w: number;
  readonly h: number;
}

function flowVariableOverlayEditors(fixtureJson: string, width: number, height: number): FlowVariableOverlayEditor[] {
  try {
    const fixture = JSON.parse(fixtureJson) as FlowFixtureV1;
    const camera = fixture.camera ?? { x: 0, y: 0, zoom: 1 };
    const editors: FlowVariableOverlayEditor[] = [];
    for (const widget of fixture.widgets ?? []) {
      if (widget.kind !== "variable") continue;
      const layout = fixture.layout?.[widget.id] ?? { x: 0, y: 0 };
      const screen = flowWorldToScreen({ x: layout.x, y: layout.y }, camera, width, height);
      editors.push({
        id: widget.id,
        name: widget.name,
        schema: widget.schema,
        x: screen.x - 72,
        y: screen.y - 18,
        w: 144,
        h: 44,
      });
    }
    return editors;
  } catch {
    return [];
  }
}

function FlowVariableOverlay({
  fixtureJson,
  schemas,
  width,
  height,
  onNameChange,
  onSchemaChange,
}: {
  readonly fixtureJson: string;
  readonly schemas: readonly FlowSchemaRefV1[];
  readonly width: number;
  readonly height: number;
  readonly onNameChange: (id: string, name: string) => void;
  readonly onSchemaChange: (id: string, schema: string) => void;
}): React.JSX.Element | null {
  const editors = flowVariableOverlayEditors(fixtureJson, width, height);
  if (!editors.length) return null;
  return (
    <div className="pointer-events-none absolute inset-0 z-50" data-testid="flow-variable-overlay">
      {editors.map((editor) => (
        <div
          key={editor.id}
          className="pointer-events-auto absolute flex flex-col gap-0.5 rounded border border-border bg-background/95 p-1 shadow-sm"
          style={{ left: editor.x, top: editor.y, width: editor.w, minHeight: editor.h }}
        >
          <input
            className="w-full rounded border border-border bg-background px-1 text-2xs text-foreground"
            value={editor.name}
            aria-label="Variable name"
            onChange={(event) => onNameChange(editor.id, event.target.value)}
          />
          <select
            className="w-full rounded border border-border bg-background px-1 text-2xs text-foreground"
            value={editor.schema}
            aria-label="Variable schema"
            onChange={(event) => onSchemaChange(editor.id, event.target.value)}
          >
            {schemas.map((schema) => (
              <option key={schema.id} value={schema.id}>
                {schema.name}
              </option>
            ))}
          </select>
        </div>
      ))}
    </div>
  );
}

interface FlowStepperField {
  readonly key: string;
  readonly label: string;
  readonly value: number;
  readonly step: number;
  readonly x: number;
  readonly y: number;
  readonly w: number;
  readonly h: number;
}

interface FlowStepperWidgetState {
  readonly widgetId: string;
  readonly fields: readonly FlowStepperField[];
}

interface FlowStepperOverlayState {
  readonly camera: { readonly x: number; readonly y: number; readonly zoom: number };
  readonly width: number;
  readonly height: number;
  readonly steppers: readonly FlowStepperWidgetState[];
}

function FlowStepperOverlay({
  state,
  width,
  height,
  onFieldChange,
}: {
  readonly state: FlowStepperOverlayState | null;
  readonly width: number;
  readonly height: number;
  readonly onFieldChange: (widgetId: string, key: string, value: number) => void;
}): React.JSX.Element | null {
  if (!state?.steppers?.length) return null;
  const zoom = Math.max(0.05, Number(state.camera?.zoom) || 1);
  const camera = { x: Number(state.camera?.x) || 0, y: Number(state.camera?.y) || 0, zoom };
  const viewportW = Number(state.width) || width;
  const viewportH = Number(state.height) || height;
  return (
    <div className="pointer-events-none absolute inset-0 z-30 overflow-visible" data-testid="flow-stepper-overlay">
      {state.steppers.flatMap(({ widgetId, fields }) =>
        fields.map((field) => {
          const screen = flowWorldToScreen({ x: field.x, y: field.y }, camera, viewportW, viewportH);
          const w = Math.max(80, field.w * zoom);
          const h = Math.max(18, field.h * zoom);
          const left = screen.x - w * 0.5;
          const top = screen.y - h * 0.5;
          return (
            <div
              key={`${widgetId}:${field.key}`}
              className="pointer-events-auto absolute flex items-center gap-0.5 overflow-hidden rounded border border-border bg-background/90 px-1 shadow-sm"
              style={{ left, top, width: w, height: h }}
            >
              <span className="shrink-0 select-none text-2xs text-muted-foreground">{field.label}</span>
              <button
                type="button"
                className="shrink-0 select-none px-0.5 text-2xs leading-none text-muted-foreground hover:text-foreground"
                onClick={() => onFieldChange(widgetId, field.key, field.value - field.step)}
              >
                −
              </button>
              <input
                type="number"
                className="min-w-0 flex-1 bg-transparent text-center text-2xs text-foreground outline-none [appearance:textfield] [&::-webkit-inner-spin-button]:appearance-none [&::-webkit-outer-spin-button]:appearance-none"
                value={field.value}
                step={field.step}
                onChange={(event) => {
                  const n = Number.parseFloat(event.target.value);
                  if (Number.isFinite(n)) onFieldChange(widgetId, field.key, n);
                }}
              />
              <button
                type="button"
                className="shrink-0 select-none px-0.5 text-2xs leading-none text-muted-foreground hover:text-foreground"
                onClick={() => onFieldChange(widgetId, field.key, field.value + field.step)}
              >
                +
              </button>
            </div>
          );
        }),
      )}
    </div>
  );
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
    const isPort = row.kind === "port" || row.align === "left" || row.align === "right";
    const maxW = Math.max(4, Number(row.nodeW) * zoom * inset);
    const maxH = Math.max(
      4,
      isPort && Number.isFinite(Number(row.maxScreenH)) && Number(row.maxScreenH) > 0
        ? Number(row.maxScreenH)
        : Number(row.nodeH) * zoom * inset,
    );
    const fontScreenPx = Number(row.fontScreenPx);
    const targetPx = Number.isFinite(fontScreenPx) && fontScreenPx > 0 ? fontScreenPx : FLOW_LABEL_SCREEN_PX;
    const fontPx = isPort
      ? flowClampPortLabelFontPx(ctx, text, targetPx, maxW, maxH)
      : flowClampLabelFontPx(ctx, text, targetPx, maxW, maxH);
    ctx.font = `${fontPx}px ${FLOW_LABEL_FONT_FAMILY}`;
    ctx.fillStyle = flowOverlayLabelFill(row.id, row.ghost === true, hoveredId, chrome, previewOffIds);
    ctx.globalAlpha = row.ghost ? 0.85 : previewOffIds.includes(row.id) ? 0.5 : 1;
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

// #region 🔖SelectionBounds
export type FlowSelectionAlignMode =
  | "alignLeft"
  | "alignRight"
  | "alignTop"
  | "alignBottom"
  | "alignHorizontal"
  | "alignVertical"
  | "distributeHorizontal"
  | "distributeVertical";

export interface FlowSelectionUnionBoundsScreen {
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
}

/** @emoji 📦 Parses screen-space selection union bounds from the flow session. */
export function parseFlowSelectionUnionBoundsScreen(json: string): FlowSelectionUnionBoundsScreen | null {
  if (json.trim() === "null") return null;
  try {
    const parsed = JSON.parse(json) as { x?: unknown; y?: unknown; width?: unknown; height?: unknown };
    const x = Number(parsed.x);
    const y = Number(parsed.y);
    const width = Number(parsed.width);
    const height = Number(parsed.height);
    if (![x, y, width, height].every(Number.isFinite)) return null;
    if (width <= 0 || height <= 0) return null;
    return { x, y, width, height };
  } catch {
    return null;
  }
}

/** @emoji 📏 Compares selection union bounds for overlay state updates. */
export function flowSelectionUnionBoundsEqual(
  left: FlowSelectionUnionBoundsScreen | null,
  right: FlowSelectionUnionBoundsScreen | null,
): boolean {
  if (left === right) return true;
  if (!left || !right) return false;
  return left.x === right.x && left.y === right.y && left.width === right.width && left.height === right.height;
}

const FLOW_SELECTION_ALIGN_BUTTON_PX = 24;
const FLOW_SELECTION_ALIGN_BUTTON_GAP_PX = 2;
const FLOW_SELECTION_ALIGN_EDGE_OUTSET_PX = 4;
const FLOW_SELECTION_ALIGN_EDGE_STACK_GAP_PX = 0;

interface FlowSelectionAlignChromeLayout {
  readonly topRow: readonly ScreenRect[];
  readonly rightCol: readonly ScreenRect[];
  readonly leftDistribute: ScreenRect | null;
  readonly bottomDistribute: ScreenRect | null;
  readonly topAnchor: { readonly x: number; readonly y: number };
  readonly rightAnchor: { readonly x: number; readonly y: number };
  readonly leftAnchor: { readonly x: number; readonly y: number } | null;
  readonly bottomAnchor: { readonly x: number; readonly y: number } | null;
  readonly gapPx: number;
}

/** @emoji 📐 Shared anchor layout for selection align chrome and pointer hit testing. */
export function flowSelectionAlignChromeLayout(
  rect: FlowSelectionUnionBoundsScreen,
  selectionCount: number,
): FlowSelectionAlignChromeLayout | null {
  if (selectionCount < 2) return null;
  const btn = FLOW_SELECTION_ALIGN_BUTTON_PX;
  const gap = FLOW_SELECTION_ALIGN_BUTTON_GAP_PX;
  const outset = FLOW_SELECTION_ALIGN_EDGE_OUTSET_PX;
  const stackGap = FLOW_SELECTION_ALIGN_EDGE_STACK_GAP_PX;
  const centerX = rect.x + rect.width / 2;
  const centerY = rect.y + rect.height / 2;
  const topRowW = btn * 3 + gap * 2;
  const rightColH = btn * 3 + gap * 2;
  const topY = rect.y - stackGap - btn;
  const topStartX = centerX - topRowW / 2;
  const rightX = rect.x + rect.width + outset;
  const rightStartY = centerY - rightColH / 2;
  const topRow: ScreenRect[] = [
    { x: topStartX, y: topY, width: btn, height: btn },
    { x: topStartX + btn + gap, y: topY, width: btn, height: btn },
    { x: topStartX + (btn + gap) * 2, y: topY, width: btn, height: btn },
  ];
  const rightCol: ScreenRect[] = [
    { x: rightX, y: rightStartY, width: btn, height: btn },
    { x: rightX, y: rightStartY + btn + gap, width: btn, height: btn },
    { x: rightX, y: rightStartY + (btn + gap) * 2, width: btn, height: btn },
  ];
  return {
    topRow,
    rightCol,
    leftDistribute:
      selectionCount >= 3 ? { x: rect.x - outset - btn, y: centerY - btn / 2, width: btn, height: btn } : null,
    bottomDistribute:
      selectionCount >= 3 ? { x: centerX - btn / 2, y: rect.y + rect.height + stackGap, width: btn, height: btn } : null,
    topAnchor: { x: centerX, y: topY },
    rightAnchor: { x: rightX, y: centerY },
    leftAnchor: selectionCount >= 3 ? { x: rect.x - outset, y: centerY } : null,
    bottomAnchor: selectionCount >= 3 ? { x: centerX, y: rect.y + rect.height + stackGap } : null,
    gapPx: gap,
  };
}

/** @emoji 🎯 Screen-space hit regions for selection align controls (container-relative coords). */
export function flowSelectionAlignHitRegions(
  rect: FlowSelectionUnionBoundsScreen,
  selectionCount: number,
): readonly { readonly mode: FlowSelectionAlignMode; readonly rect: ScreenRect }[] {
  const layout = flowSelectionAlignChromeLayout(rect, selectionCount);
  if (!layout) return [];
  const regions: { mode: FlowSelectionAlignMode; rect: ScreenRect }[] = [
    { mode: "alignLeft", rect: layout.topRow[0]! },
    { mode: "alignHorizontal", rect: layout.topRow[1]! },
    { mode: "alignRight", rect: layout.topRow[2]! },
    { mode: "alignTop", rect: layout.rightCol[0]! },
    { mode: "alignVertical", rect: layout.rightCol[1]! },
    { mode: "alignBottom", rect: layout.rightCol[2]! },
  ];
  if (layout.leftDistribute) regions.push({ mode: "distributeVertical", rect: layout.leftDistribute });
  if (layout.bottomDistribute) regions.push({ mode: "distributeHorizontal", rect: layout.bottomDistribute });
  return regions;
}

/** @emoji 🖱️ True when a container-local point hits a selection align control. */
export function flowPointerHitsSelectionAlign(
  localX: number,
  localY: number,
  rect: FlowSelectionUnionBoundsScreen,
  selectionCount: number,
): FlowSelectionAlignMode | null {
  for (const region of flowSelectionAlignHitRegions(rect, selectionCount)) {
    const { x, y, width, height } = region.rect;
    if (localX >= x && localX <= x + width && localY >= y && localY <= y + height) {
      if ((region.mode === "distributeVertical" || region.mode === "distributeHorizontal") && selectionCount < 3) {
        continue;
      }
      return region.mode;
    }
  }
  return null;
}

interface FlowSelectionBoundsOverlayProps {
  readonly rect: FlowSelectionUnionBoundsScreen;
  readonly selectionCount: number;
  readonly onAlign: (mode: FlowSelectionAlignMode) => void;
}

function FlowSelectionBoundsButton({
  icon,
  label,
  disabled,
  onPress,
}: {
  readonly icon: string;
  readonly label: string;
  readonly disabled?: boolean;
  readonly onPress: () => void;
}): React.JSX.Element {
  return (
    <button
      type="button"
      aria-label={label}
      title={label}
      disabled={disabled}
      className={cn("pointer-events-auto flex h-6 w-6 items-center justify-center p-0 disabled:pointer-events-none disabled:opacity-40", floatingToolbarSurfaceClass, menuListItemClassName)}
      onPointerDown={(event) => {
        event.preventDefault();
        event.stopPropagation();
        if (disabled) return;
        onPress();
      }}
    >
      <Icon icon={icon} size="tiny" />
    </button>
  );
}

/** @emoji 📐 Selection union bounding rectangle with edge alignment controls. */
function FlowSelectionBoundsOverlay({ rect, selectionCount, onAlign }: FlowSelectionBoundsOverlayProps): React.JSX.Element {
  const layout = flowSelectionAlignChromeLayout(rect, selectionCount);
  return (
    <div className="pointer-events-none absolute inset-0 z-20 overflow-visible" aria-hidden data-testid="flow-selection-bounds">
      <div
        className="pointer-events-none absolute border border-emphasized"
        style={{ left: rect.x, top: rect.y, width: rect.width, height: rect.height }}
      />
      {layout ? (
        <>
          <div
            className="pointer-events-none absolute flex -translate-x-1/2"
            style={{ left: layout.topAnchor.x, top: layout.topAnchor.y, gap: layout.gapPx }}
          >
            <FlowSelectionBoundsButton icon="arrow-left" label="Align left" onPress={() => onAlign("alignLeft")} />
            <FlowSelectionBoundsButton icon="arrow-right-left" label="Align horizontal" onPress={() => onAlign("alignHorizontal")} />
            <FlowSelectionBoundsButton icon="arrow-right" label="Align right" onPress={() => onAlign("alignRight")} />
          </div>
          <div
            className="pointer-events-none absolute flex flex-col -translate-y-1/2"
            style={{ left: layout.rightAnchor.x, top: layout.rightAnchor.y, gap: layout.gapPx }}
          >
            <FlowSelectionBoundsButton icon="arrow-up" label="Align top" onPress={() => onAlign("alignTop")} />
            <FlowSelectionBoundsButton icon="chevrons-up-down" label="Align vertical" onPress={() => onAlign("alignVertical")} />
            <FlowSelectionBoundsButton icon="arrow-down" label="Align bottom" onPress={() => onAlign("alignBottom")} />
          </div>
          {layout.leftAnchor ? (
            <div
              className="pointer-events-none absolute -translate-x-full -translate-y-1/2"
              style={{ left: layout.leftAnchor.x, top: layout.leftAnchor.y }}
            >
              <FlowSelectionBoundsButton icon="grip-vertical" label="Distribute vertically" onPress={() => onAlign("distributeVertical")} />
            </div>
          ) : null}
          {layout.bottomAnchor ? (
            <div
              className="pointer-events-none absolute -translate-x-1/2"
              style={{ left: layout.bottomAnchor.x, top: layout.bottomAnchor.y }}
            >
              <FlowSelectionBoundsButton icon="more-horizontal" label="Distribute horizontally" onPress={() => onAlign("distributeHorizontal")} />
            </div>
          ) : null}
        </>
      ) : null}
    </div>
  );
}
// #endregion 🔖SelectionBounds

// #region 🔖WidgetPaletteDragPreview
/** @emoji 📍 Window pointer moves and global dragover ticks for palette widget drop ghosts. */
function FlowWidgetPaletteDragPreviewBridge(props: {
  readonly canvasRef: React.RefObject<HTMLCanvasElement | null>;
  readonly containerRef: React.RefObject<HTMLDivElement | null>;
  readonly enabled: boolean;
  readonly setFixtureDragActive: (active: boolean) => void;
}): null {
  useEffect(() => {
    if (!props.enabled) {
      return;
    }
    const onDragOver = (event: DragEvent): void => {
      if (!flowWidgetDragAcceptsTransfer([...event.dataTransfer!.types]) && !flowReadActivePaletteDragEncoded()) {
        return;
      }
      flowNotePaletteWidgetDragClient(event.clientX, event.clientY);
    };
    window.addEventListener("dragover", onDragOver);
    return () => window.removeEventListener("dragover", onDragOver);
  }, [props.enabled]);

  useEffect(() => {
    if (!props.enabled) {
      return;
    }
    const dropHost = (): HTMLElement | null => props.containerRef.current ?? props.canvasRef.current;

    const onWindowPointerMove = (event: PointerEvent): void => {
      if (!flowWidgetPalettePointerDragRef.active) {
        return;
      }
      props.setFixtureDragActive(isClientPointOverFlowWidgetDropHost(event.clientX, event.clientY, dropHost()));
      flowNotePaletteWidgetDragClient(event.clientX, event.clientY);
    };

    window.addEventListener("pointermove", onWindowPointerMove);
    return () => window.removeEventListener("pointermove", onWindowPointerMove);
  }, [props.canvasRef, props.containerRef, props.enabled, props.setFixtureDragActive]);

  return null;
}

/** @emoji ⎋ Global Escape handler while a workbench palette widget drag is active. */
function FlowWidgetPaletteDragEscapeBridge(props: { readonly enabled: boolean }): null {
  useEffect(() => {
    if (!props.enabled) {
      return;
    }
    const onKeyDown = (event: KeyboardEvent): void => {
      if (event.key !== "Escape") {
        return;
      }
      if (!flowReadActivePaletteDragEncoded()) {
        return;
      }
      event.preventDefault();
      abortFlowWidgetPaletteDrag();
    };
    window.addEventListener("keydown", onKeyDown, true);
    return () => window.removeEventListener("keydown", onKeyDown, true);
  }, [props.enabled]);
  return null;
}
// #endregion 🔖WidgetPaletteDragPreview

// #region 🔖FlowCanvas
export type FlowSelectionMode = "default" | "additive" | "subtractive" | "invertive";
export type FlowSelectionMethod = "rectangle" | "lasso";

export interface FlowPreselectSnapshot {
  readonly ids: readonly string[];
  readonly removedIds: readonly string[];
}

export interface FlowChannelRef {
  readonly widgetId: string;
  readonly port: string;
  readonly direction: "in" | "out";
}

type FlowSessionClusterApi = FlowSession & {
  collapseSelection(idsJson: string): string;
  explodeCluster(clusterId: string): void;
  takePendingClusterExplode(): string | undefined;
};

type FlowSessionVariableApi = FlowSession & {
  schemasJson(): string;
  setVariableName(widgetId: string, name: string): void;
  setVariableSchema(widgetId: string, schema: string): void;
};

type FlowSessionStepperApi = FlowSession & {
  setStepperFieldValue(widgetId: string, key: string, value: number): void;
  stepperOverlayStateJson(): string;
};

type FlowSessionChannelApi = FlowSession & {
  hoveredChannelJson(): string;
  selectedChannelsJson(): string;
  setHoverChannel(widgetId: string | null, port: string | null): void;
  setSelectedChannels(json: string): void;
};

export function parseFlowChannelRef(json: string | null | undefined): FlowChannelRef | null {
  if (!json || json === "null") return null;
  try {
    const parsed = JSON.parse(json) as FlowChannelRef;
    if (parsed.direction !== "in" && parsed.direction !== "out") return null;
    if (typeof parsed.widgetId !== "string" || typeof parsed.port !== "string") return null;
    return parsed;
  } catch {
    return null;
  }
}

export function parseFlowChannelRefArray(json: string): FlowChannelRef[] {
  try {
    const parsed = JSON.parse(json) as unknown;
    if (!Array.isArray(parsed)) return [];
    return parsed
      .map((entry) => parseFlowChannelRef(JSON.stringify(entry)))
      .filter((entry): entry is FlowChannelRef => entry != null);
  } catch {
    return [];
  }
}

function readFlowSessionChannels(session: FlowSession): { hovered: FlowChannelRef | null; selected: FlowChannelRef[] } {
  const api = session as FlowSessionChannelApi;
  return {
    hovered: parseFlowChannelRef(api.hoveredChannelJson?.()),
    selected: parseFlowChannelRefArray(api.selectedChannelsJson?.() ?? "[]"),
  };
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
  readonly onEvalOutputs?: (outputsJson: string, previewMeshes?: Readonly<Record<string, unknown>>) => void;
  readonly onFixtureChange?: (fixtureJson: string) => void;
  readonly onCatalogueReady?: (sections: readonly CatalogueSection[]) => void;
  readonly onWidgetDrop?: (detail: FlowWidgetDropDetail) => void;
  readonly onSelectionChange?: (ids: readonly string[]) => void;
  readonly onPreselectChange?: (snapshot: FlowPreselectSnapshot) => void;
  readonly onHoverChange?: (id: string | null) => void;
  readonly onChannelHoverChange?: (channel: FlowChannelRef | null) => void;
  readonly onSelectedChannelsChange?: (channels: readonly FlowChannelRef[]) => void;
  readonly selectedNodeIds?: readonly string[];
  readonly hoveredNodeId?: string | null;
  readonly hoveredChannel?: FlowChannelRef | null;
  readonly selectedChannels?: readonly FlowChannelRef[];
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
  readonly proximityDistance?: number;
}

export const FLOW_DEFAULT_PROXIMITY_DISTANCE = 48;

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

export function flowChannelRefEqual(left: FlowChannelRef | null | undefined, right: FlowChannelRef | null | undefined): boolean {
  if (left === right) return true;
  if (!left || !right) return false;
  return left.widgetId === right.widgetId && left.port === right.port && left.direction === right.direction;
}

export function flowChannelRefArraysEqual(left: readonly FlowChannelRef[], right: readonly FlowChannelRef[]): boolean {
  if (left.length !== right.length) return false;
  return left.every((value, index) => flowChannelRefEqual(value, right[index]));
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

type FlowSessionPickApi = FlowSession & { pickTargetsAtScreenJson(sx: number, sy: number): string };

type FlowPickTargetRow = { readonly domain: string; readonly id: string; readonly generality: number; readonly label?: string };

function parseFlowPickTargetsJson(json: string): FlowPickTargetRow[] {
  try {
    const parsed = JSON.parse(json) as FlowPickTargetRow[];
    return Array.isArray(parsed) ? parsed : [];
  } catch {
    return [];
  }
}

function flowPickRowToCanvas(target: FlowPickTargetRow): CanvasPickTarget {
  return { domain: target.domain, id: target.id, generality: target.generality, label: target.label };
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
  onChannelHoverChange,
  onSelectedChannelsChange,
  selectedNodeIds,
  hoveredNodeId,
  hoveredChannel,
  selectedChannels,
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
  proximityDistance = FLOW_DEFAULT_PROXIMITY_DISTANCE,
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
  const onChannelHoverChangeRef = useRef(onChannelHoverChange);
  const onSelectedChannelsChangeRef = useRef(onSelectedChannelsChange);
  const storeRef = useRef(store ?? createLocalFlowStore());
  const pointerRef = useRef({ active: false, pan: false, id: -1, shift: false, ctrl: false, alt: false, x: 0, y: 0 });
  const pointerGestureFixtureRef = useRef<string | null>(null);
  const imageFileInputRef = useRef<HTMLInputElement>(null);
  const pendingImageWidgetIdRef = useRef<string | null>(null);
  const onContextMenuRef = useRef(onContextMenu);
  const contextMenuRef = useRef(contextMenu);
  const onPreviewOffChangeRef = useRef(onPreviewOffChange);
  const onLodChangeRef = useRef(onLodChange);
  const lastAutomaticLodRef = useRef<boolean | null>(null);
  const lastProximityDistanceRef = useRef<number | null>(null);
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
  const [selectionBounds, setSelectionBounds] = useState<FlowSelectionUnionBoundsScreen | null>(null);
  const [selectionBoundsCount, setSelectionBoundsCount] = useState(0);
  const [paramOverlayState, setParamOverlayState] = useState<FlowParamOverlayPaintState | null>(null);
  const [stepperOverlayState, setStepperOverlayState] = useState<FlowStepperOverlayState | null>(null);
  const [variableFixtureJson, setVariableFixtureJson] = useState("");
  const [schemaCatalogue, setSchemaCatalogue] = useState<readonly FlowSchemaRefV1[]>([]);
  const selectionBoundsRef = useRef<FlowSelectionUnionBoundsScreen | null>(null);
  const selectionBoundsCountRef = useRef(0);
  const alignSelectionRef = useRef<(mode: FlowSelectionAlignMode) => void>(() => {});

  const syncVelloTheme = useCallback(() => {
    syncSessionVelloTheme(sessionRef.current);
  }, []);

  useVelloThemeSync(syncVelloTheme);

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
    onChannelHoverChangeRef.current = onChannelHoverChange;
  }, [onChannelHoverChange]);

  useEffect(() => {
    onSelectedChannelsChangeRef.current = onSelectedChannelsChange;
  }, [onSelectedChannelsChange]);

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

  useEffect(() => {
    bootstrapFixtureJsonRef.current = fixtureJson;
  }, [fixtureJson]);

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

  const syncProximityDistance = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    const next = Math.max(0, proximityDistance ?? FLOW_DEFAULT_PROXIMITY_DISTANCE);
    if (lastProximityDistanceRef.current !== next) {
      session.setProximityDistance(next);
      lastProximityDistanceRef.current = next;
    }
  }, [proximityDistance]);

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

  const syncSelectionBoundsOverlay = useCallback((session: FlowSession) => {
    const selected = parseFlowWidgetIdArray(session.selectedWidgetIds());
    if (!selected.length) {
      selectionBoundsRef.current = null;
      selectionBoundsCountRef.current = 0;
      setSelectionBounds((prev) => (prev === null ? prev : null));
      setSelectionBoundsCount((prev) => (prev === 0 ? prev : 0));
      return;
    }
    const count = selected.length;
    selectionBoundsCountRef.current = count;
    setSelectionBoundsCount((prev) => (prev === count ? prev : count));
    try {
      const next = parseFlowSelectionUnionBoundsScreen(session.selectionUnionBoundsScreenJson());
      selectionBoundsRef.current = next;
      setSelectionBounds((prev) => (flowSelectionUnionBoundsEqual(prev, next) ? prev : next));
    } catch {
      selectionBoundsRef.current = null;
      setSelectionBounds((prev) => (prev === null ? prev : null));
    }
  }, []);

  const emitInteractionState = useCallback((session: FlowSession) => {
    const selected = parseFlowWidgetIdArray(session.selectedWidgetIds());
    const hovered = session.hoveredWidgetId() ?? null;
    const channels = readFlowSessionChannels(session);
    const preselect = parseFlowPreselectJson(session.preselectWidgetIdsJson());
    const previewOff = parseFlowWidgetIdArray(session.previewOffWidgetIds());
    onSelectionChangeRef.current?.(selected);
    onPreselectChangeRef.current?.(preselect);
    onHoverChangeRef.current?.(hovered);
    onChannelHoverChangeRef.current?.(channels.hovered);
    onSelectedChannelsChangeRef.current?.(channels.selected);
    onPreviewOffChangeRef.current?.(previewOff);
    syncMarqueeOverlay(session);
    syncSelectionBoundsOverlay(session);
    console.log(
      `[DEBUG] flow interaction selected=[${selected.join(", ")}] preselect=[${preselect.ids.join(", ")}] hover=${hovered ?? "—"} channel=${channels.hovered ? `${channels.hovered.widgetId}.${channels.hovered.direction}.${channels.hovered.port}` : "—"} previewOff=[${previewOff.join(", ")}]`,
    );
  }, [syncMarqueeOverlay, syncSelectionBoundsOverlay]);

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
      syncProximityDistance();
      syncVelloTheme();
      const session = sessionRef.current;
      session?.renderFrame();
      const overlay = textOverlayRef.current;
      if (session && overlay) {
        const { width, height, dpr } = viewportRef.current;
        paintFlowLabelOverlays(session, overlay, width, height, dpr);
      }
      if (session) {
        try {
          const next = JSON.parse(session.paramOverlayPaintStateJson()) as FlowParamOverlayPaintState;
          setParamOverlayState((prev) => (JSON.stringify(prev) === JSON.stringify(next) ? prev : next));
        } catch {
          setParamOverlayState((prev) => (prev === null ? prev : null));
        }
        try {
          const nextStepper = JSON.parse((session as FlowSessionStepperApi).stepperOverlayStateJson()) as FlowStepperOverlayState;
          setStepperOverlayState((prev) => (JSON.stringify(prev) === JSON.stringify(nextStepper) ? prev : nextStepper));
        } catch {
          setStepperOverlayState((prev) => (prev === null ? prev : null));
        }
        try {
          setVariableFixtureJson(session.fixtureJson());
        } catch {
          setVariableFixtureJson("");
        }
        syncSelectionBoundsOverlay(session);
      }
      reportDrawLod();
    } catch {
      /* gpu not ready */
    }
  }, [reportDrawLod, syncLodMode, syncProximityDistance, syncSelectionBoundsOverlay, syncVelloTheme]);

  useEffect(() => {
    lastAutomaticLodRef.current = null;
    lastProximityDistanceRef.current = null;
    lastForcedLodRef.current = null;
    renderFrame();
  }, [automaticLod, lod, proximityDistance, renderFrame]);

  const evaluateTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const orchestratorRef = useRef<FlowOrchestratorClient | null>(null);
  const evalGenerationRef = useRef(0);
  const lastEvalFixtureRef = useRef<string | null>(null);

  useEffect(() => {
    if (import.meta.env.VITEST || typeof Worker === "undefined") return;
    const client = new FlowOrchestratorClient();
    orchestratorRef.current = client;
    return () => {
      client.terminate();
      orchestratorRef.current = null;
    };
  }, []);

  const runEvalAnimation = useCallback(
    (tick: () => boolean) => {
      if (!tick()) return;
      renderFrame();
      requestAnimationFrame(() => runEvalAnimation(tick));
    },
    [renderFrame],
  );

  const evaluate = useCallback(() => {
    if (evaluateTimerRef.current != null) clearTimeout(evaluateTimerRef.current);
    evaluateTimerRef.current = setTimeout(() => {
      evaluateTimerRef.current = null;
      void (async () => {
        const session = sessionRef.current;
        if (!session) return;
        const generation = ++evalGenerationRef.current;
        const fixture = session.fixtureJson();
        const dirty = flowTreeDirtyNeuronIds(lastEvalFixtureRef.current, fixture);
        if (!dirty.structural && dirty.ids.length === 0) {
          lastEvalFixtureRef.current = fixture;
          session.clearComputingWidgetIds();
          renderFrame();
          return;
        }
        const path = dirty.path;
        if (!dirty.structural && !flowDirtyComputePathReady(fixture, path)) {
          lastEvalFixtureRef.current = fixture;
          session.clearComputingWidgetIds();
          renderFrame();
          return;
        }
        const parsedFixture = parseFlowFixtureForDirtyDiff(fixture);
        const animationPath = parsedFixture ? flowEvalAnimationPath(path, parsedFixture) : [...path];
        const orchestrator = orchestratorRef.current;
        const willEvaluate = Boolean(orchestrator && !import.meta.env.VITEST) || import.meta.env.VITEST;
        if (!willEvaluate) {
          console.log("[DEBUG] flow orchestrator unavailable; skipped eval");
          lastEvalFixtureRef.current = fixture;
          session.clearComputingWidgetIds();
          renderFrame();
          return;
        }
        let pathIndex = 0;
        let lastPathAdvanceMs = performance.now();
        const applyPathProgress = () => {
          if (animationPath.length === 0) {
            session.clearComputingWidgetIds();
            return;
          }
          session.setComputingProgress(JSON.stringify(flowComputeProgressPayload(animationPath, pathIndex)));
        };
        applyPathProgress();
        renderFrame();
        let animating = true;
        runEvalAnimation(() => {
          if (!animating || generation !== evalGenerationRef.current) return false;
          const now = performance.now();
          if (animationPath.length > 0 && pathIndex < animationPath.length - 1 && now - lastPathAdvanceMs >= 160) {
            pathIndex += 1;
            lastPathAdvanceMs = now;
            applyPathProgress();
          }
          return true;
        });
        try {
          let outputsJson: string;
          let previewMeshes: Readonly<Record<string, unknown>> | undefined;
          if (orchestrator && !import.meta.env.VITEST) {
            await orchestrator.loadFixtureJson(fixture);
            const result = await orchestrator.evaluate();
            if (generation !== evalGenerationRef.current) return;
            outputsJson = result.outputsJson;
            session.applyEvalOutputsJson(outputsJson);
            animating = false;
            lastEvalFixtureRef.current = fixture;
            renderFrame();
            const text = session.previewText();
            onPreviewTextRef.current?.(text);
            previewMeshes = await orchestrator.tessellatePreviews(outputsJson);
            if (generation !== evalGenerationRef.current) return;
            onEvalOutputsRef.current?.(outputsJson, previewMeshes);
            console.log(`[DEBUG] flow evaluate preview: ${text}`);
          } else if (import.meta.env.VITEST) {
            outputsJson = await session.evaluate();
            if (generation !== evalGenerationRef.current) return;
            animating = false;
            lastEvalFixtureRef.current = fixture;
            const text = session.previewText();
            onPreviewTextRef.current?.(text);
            onEvalOutputsRef.current?.(outputsJson);
            console.log(`[DEBUG] flow evaluate preview: ${text}`);
          }
        } catch (err) {
          console.log(`[DEBUG] flow evaluate failed: ${String(err)}`);
        } finally {
          animating = false;
          if (generation === evalGenerationRef.current) {
            session.clearComputingWidgetIds();
          }
        }
        renderFrame();
      })();
    }, 32);
  }, [renderFrame, runEvalAnimation]);

  useEffect(() => {
    const session = sessionRef.current;
    if (!session || fixtureJson == null) return;
    try {
      const current = session.fixtureJson();
      if (current === fixtureJson) return;
      const treeDelta = flowTreeDirtyNeuronIds(lastEvalFixtureRef.current, fixtureJson);
      if (!treeDelta.structural && treeDelta.ids.length === 0) {
        session.loadFixtureJson(fixtureJson);
        lastEvalFixtureRef.current = fixtureJson;
        emitInteractionState(session);
        renderFrame();
        return;
      }
      session.loadFixtureJson(fixtureJson);
      emitInteractionState(session);
      evaluate();
      renderFrame();
    } catch {
      /* fixture not ready */
    }
  }, [fixtureJson, emitInteractionState, evaluate, renderFrame]);

  const alignSelection = useCallback(
    (mode: FlowSelectionAlignMode) => {
      const session = sessionRef.current;
      if (!session) return;
      try {
        session.alignSelection(mode);
        emitInteractionState(session);
        evaluate();
        persistFixture();
        renderFrame();
        console.log(`[DEBUG] flow align selection mode=${mode}`);
      } catch (err) {
        console.log(`[DEBUG] flow align selection failed: ${String(err)}`);
      }
    },
    [emitInteractionState, evaluate, persistFixture, renderFrame],
  );

  useEffect(() => {
    alignSelectionRef.current = alignSelection;
  }, [alignSelection]);

  const onNeuronParamChange = useCallback(
    (nodeId: string, portId: string, value: unknown) => {
      const session = sessionRef.current;
      if (!session) return;
      const atom =
        typeof value === "boolean"
          ? value
          : typeof value === "string"
            ? value
            : typeof value === "number"
              ? value
              : value;
      runFlowGraphEdit(session, [{ op: "setNeuronParams", id: nodeId, paramsJson: JSON.stringify({ [portId]: atom }) }]);
      emitInteractionState(session);
      evaluate();
      persistFixture();
      renderFrame();
      console.log(`[DEBUG] flow neuron param ${nodeId}.${portId}=${String(value)}`);
    },
    [emitInteractionState, evaluate, persistFixture, renderFrame],
  );

  const onStepperFieldChange = useCallback(
    (widgetId: string, key: string, value: number) => {
      const session = sessionRef.current;
      if (!session) return;
      runFlowGraphEdit(session, [{ op: "setStepperFieldValue", id: widgetId, key, value }]);
      emitInteractionState(session);
      evaluate();
      persistFixture();
      renderFrame();
    },
    [emitInteractionState, evaluate, persistFixture, renderFrame],
  );

  const onVariableNameChange = useCallback(
    (id: string, name: string) => {
      const session = sessionRef.current;
      if (!session) return;
      runFlowGraphEdit(session, [{ op: "setVariableName", id, name }]);
      emitInteractionState(session);
      evaluate();
      persistFixture();
      renderFrame();
    },
    [emitInteractionState, evaluate, persistFixture, renderFrame],
  );

  const onVariableSchemaChange = useCallback(
    (id: string, schema: string) => {
      const session = sessionRef.current;
      if (!session) return;
      runFlowGraphEdit(session, [{ op: "setVariableSchema", id, schema }]);
      emitInteractionState(session);
      evaluate();
      persistFixture();
      renderFrame();
    },
    [emitInteractionState, evaluate, persistFixture, renderFrame],
  );

  const onContainerPointerDownCapture = useCallback((event: React.PointerEvent<HTMLDivElement>) => {
    if (event.button !== 0) return;
    const rect = selectionBoundsRef.current;
    const count = selectionBoundsCountRef.current;
    if (!rect || count < 2) return;
    const host = containerRef.current;
    if (!host) return;
    const bounds = host.getBoundingClientRect();
    const mode = flowPointerHitsSelectionAlign(event.clientX - bounds.left, event.clientY - bounds.top, rect, count);
    if (!mode) return;
    event.preventDefault();
    event.stopPropagation();
    alignSelectionRef.current(mode);
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
      // In-WASM module registry handles evaluation; extension host loads manifests only.
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
      flowWidgetPaletteDropCommittedRef.current = true;
      flowStopPaletteDragPreviewLoop();
      flowWidgetPaletteDragEncodedRef.current = null;
      flowWidgetPalettePointerDragRef.active = false;
      flowWidgetPalettePointerDragRef.encoded = null;
      flowWidgetPaletteDragRef.active = false;
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
    if (hoveredNodeId !== undefined || hoveredChannel !== undefined) {
      const api = session as FlowSessionChannelApi;
      const currentChannels = readFlowSessionChannels(session);
      const nextChannel = hoveredChannel ?? null;
      if (!flowChannelRefEqual(currentChannels.hovered, nextChannel)) {
        api.setHoverChannel?.(nextChannel?.widgetId ?? null, nextChannel?.port ?? null);
      } else if (hoveredNodeId !== undefined) {
        const current = session.hoveredWidgetId() ?? null;
        if (current !== hoveredNodeId) {
          session.setHover(hoveredNodeId);
        }
      }
    }
    if (selectedChannels !== undefined) {
      const api = session as FlowSessionChannelApi;
      const current = readFlowSessionChannels(session).selected;
      if (!flowChannelRefArraysEqual(current, selectedChannels)) {
        api.setSelectedChannels?.(JSON.stringify([...selectedChannels]));
      }
    }
    if (previewOffNodeIds !== undefined) {
      const current = parseFlowWidgetIdArray(session.previewOffWidgetIds());
      if (!flowWidgetIdArraysEqual(current, previewOffNodeIds)) {
        session.setPreviewOff(JSON.stringify([...previewOffNodeIds]));
      }
    }
    renderFrame();
  }, [hoveredChannel, hoveredNodeId, previewOffNodeIds, renderFrame, selectedChannels, selectedNodeIds]);

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
      const key = event.key.toLowerCase();
      if (mod && key === "z" && !event.shiftKey) {
        if (session.undo()) {
          event.preventDefault();
          emitInteractionState(session);
          evaluate();
          persistFixture();
          renderFrame();
        }
        return;
      }
      if ((mod && key === "y") || (mod && event.shiftKey && key === "z")) {
        if (session.redo()) {
          event.preventDefault();
          emitInteractionState(session);
          evaluate();
          persistFixture();
          renderFrame();
        }
        return;
      }
      if (mod && key === "a") {
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
        if (!session.hasSelection()) return;
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
      try {
        setSchemaCatalogue(JSON.parse((session as FlowSessionVariableApi).schemasJson()) as FlowSchemaRefV1[]);
      } catch {
        setSchemaCatalogue([]);
      }
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

  const resolveFlowPickTargetsAtClient = useCallback(
    (client: { readonly x: number; readonly y: number }) => {
      const session = sessionRef.current as FlowSessionPickApi | null;
      if (!session) return [];
      const { x, y } = clientToCanvas(client.x, client.y);
      return parseFlowPickTargetsJson(session.pickTargetsAtScreenJson(x, y)).map(flowPickRowToCanvas);
    },
    [clientToCanvas],
  );

  const applyFlowHoverPickKey = useCallback(
    (key: string | null) => {
      const session = sessionRef.current;
      if (!session) return;
      if (!key) {
        session.setHover(null);
        emitInteractionState(session);
        renderFrame();
        return;
      }
      const parsed = parseCanvasPickTargetKey(key);
      if (parsed?.domain === "handle") {
        const [widgetId, port] = key.split(":");
        (session as FlowSessionChannelApi).setHoverChannel?.(widgetId ?? null, port ?? null);
      } else {
        session.setHover(key);
      }
      emitInteractionState(session);
      renderFrame();
    },
    [emitInteractionState, renderFrame],
  );

  const canvasPick = useCanvasPickInteraction({
    resolveTargetsAtClient: resolveFlowPickTargetsAtClient,
    onHoverFocus: (focus) => applyFlowHoverPickKey(focus.targetKey),
    onSelectTarget: (target) => {
      const session = sessionRef.current;
      if (!session) return;
      session.setSelectionJson(JSON.stringify([target.id]));
      emitInteractionState(session);
      evaluate();
      persistFixture();
      renderFrame();
    },
  });

  const pickDeferRef = useRef<{ readonly sx: number; readonly sy: number; readonly shift: boolean; readonly ctrl: boolean; readonly alt: boolean } | null>(null);

  const onPointerDown = useCallback(
    (e: React.PointerEvent<HTMLCanvasElement>) => {
      const session = sessionRef.current;
      if (!session || e.button > 2) return;
      if (e.button === 2) return;
      e.currentTarget.setPointerCapture(e.pointerId);
      const { x, y } = clientToCanvas(e.clientX, e.clientY);
      canvasPick.onCanvasPointerDown({ x: e.clientX, y: e.clientY });
      const pickApi = session as FlowSessionPickApi;
      const targets = parseFlowPickTargetsJson(pickApi.pickTargetsAtScreenJson(x, y));
      if (e.button === 0 && targets.length > 1 && !e.shiftKey && !e.metaKey && !e.ctrlKey) {
        pickDeferRef.current = { sx: x, sy: y, shift: e.shiftKey, ctrl: e.metaKey || e.ctrlKey, alt: e.altKey };
        pointerRef.current = { active: true, pan: false, id: e.pointerId, shift: e.shiftKey, ctrl: e.metaKey || e.ctrlKey, alt: e.altKey, x, y };
        return;
      }
      pickDeferRef.current = null;
      pointerRef.current = { active: true, pan: false, id: e.pointerId, shift: e.shiftKey, ctrl: e.metaKey || e.ctrlKey, alt: e.altKey, x, y };
      try {
        pointerGestureFixtureRef.current = session.fixtureJson();
      } catch {
        pointerGestureFixtureRef.current = null;
      }
      session.pointerDownScreen(x, y, e.button, e.shiftKey, e.metaKey || e.ctrlKey, e.altKey, false);
      emitInteractionState(session);
      renderFrame();
    },
    [canvasPick, clientToCanvas, emitInteractionState, renderFrame],
  );

  const onPointerMove = useCallback(
    (e: React.PointerEvent<HTMLCanvasElement>) => {
      const session = sessionRef.current;
      if (!session) return;
      if (pointerRef.current.active && pointerRef.current.id !== e.pointerId) return;
      const { x, y } = clientToCanvas(e.clientX, e.clientY);
      pointerRef.current.x = x;
      pointerRef.current.y = y;
      pointerRef.current.shift = e.shiftKey;
      pointerRef.current.ctrl = e.metaKey || e.ctrlKey;
      pointerRef.current.alt = e.altKey;
      if (pickDeferRef.current) {
        const defer = pickDeferRef.current;
        const distance = Math.hypot(x - defer.sx, y - defer.sy);
        if (distance > 4) {
          pickDeferRef.current = null;
          session.pointerDownScreen(defer.sx, defer.sy, 0, defer.shift, defer.ctrl, defer.alt, false);
        } else if (!canvasPick.pickMenuOpen) {
          canvasPick.onCanvasPointerMove({ x: e.clientX, y: e.clientY });
        }
        return;
      }
      if (!canvasPick.pickMenuOpen) {
        canvasPick.onCanvasPointerMove({ x: e.clientX, y: e.clientY });
      }
      session.pointerMoveScreen(x, y, e.shiftKey, e.metaKey || e.ctrlKey, e.altKey);
      emitInteractionState(session);
      if (pointerRef.current.active) {
        if (session.widgetDragActive()) {
          evaluate();
        }
        renderFrame();
      }
    },
    [canvasPick, clientToCanvas, emitInteractionState, evaluate, renderFrame],
  );

  const onPointerLeave = useCallback(
    (e: React.PointerEvent<HTMLCanvasElement>) => {
      const session = sessionRef.current;
      if (!session || pointerRef.current.active) return;
      canvasPick.onCanvasPointerLeave();
      session.setHover(null);
      emitInteractionState(session);
    },
    [canvasPick, emitInteractionState],
  );

  const onPointerUp = useCallback(
    (e: React.PointerEvent<HTMLCanvasElement>) => {
      const session = sessionRef.current;
      if (!session || pointerRef.current.id !== e.pointerId) return;
      const { x, y } = clientToCanvas(e.clientX, e.clientY);
      if (pickDeferRef.current) {
        canvasPick.onCanvasPointerUp(
          { x: e.clientX, y: e.clientY },
          { shift: e.shiftKey, ctrl: e.ctrlKey, meta: e.metaKey, alt: e.altKey },
        );
        pickDeferRef.current = null;
        pointerRef.current = { active: false, pan: false, id: -1, shift: false, ctrl: false, alt: false, x: 0, y: 0 };
        return;
      }
      session.pointerUpScreen(x, y, e.shiftKey, e.metaKey || e.ctrlKey, e.altKey);
      const clusterApi = session as FlowSessionClusterApi;
      const explodeId = clusterApi.takePendingClusterExplode?.();
      if (explodeId) {
        clusterApi.explodeCluster(explodeId);
      }
      pointerRef.current = { active: false, pan: false, id: -1, shift: false, ctrl: false, alt: false, x: 0, y: 0 };
      emitInteractionState(session);
      const gestureBefore = pointerGestureFixtureRef.current;
      pointerGestureFixtureRef.current = null;
      let shouldEvaluate = false;
      let shouldPersist = false;
      if (gestureBefore !== null) {
        try {
          const fixtureAfter = session.fixtureJson();
          shouldPersist = fixtureAfter !== gestureBefore;
          const gestureDirty = flowTreeDirtyNeuronIds(gestureBefore, fixtureAfter);
          shouldEvaluate = gestureDirty.structural || gestureDirty.ids.length > 0;
        } catch {
          /* fixture not ready */
        }
      }
      if (shouldEvaluate) evaluate();
      if (shouldPersist) persistFixture();
      renderFrame();
    },
    [canvasPick, clientToCanvas, emitInteractionState, evaluate, persistFixture, renderFrame],
  );

  useEffect(() => {
    const refreshDragWithAlt = (alt: boolean) => {
      const session = sessionRef.current;
      const ptr = pointerRef.current;
      if (!session || !ptr.active) return;
      ptr.alt = alt;
      session.pointerMoveScreen(ptr.x, ptr.y, ptr.shift, ptr.ctrl, alt);
      emitInteractionState(session);
      if (session.widgetDragActive()) {
        evaluate();
      }
      renderFrame();
    };
    const onKeyDown = (e: KeyboardEvent) => {
      if (e.key !== "Alt" || !e.altKey) return;
      refreshDragWithAlt(true);
    };
    const onKeyUp = (e: KeyboardEvent) => {
      if (e.key !== "Alt") return;
      refreshDragWithAlt(false);
    };
    window.addEventListener("keydown", onKeyDown);
    window.addEventListener("keyup", onKeyUp);
    return () => {
      window.removeEventListener("keydown", onKeyDown);
      window.removeEventListener("keyup", onKeyUp);
    };
  }, [emitInteractionState, evaluate, renderFrame]);

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
        case "graphEdit": {
          const ops = Array.isArray(args.ops) ? (args.ops as FlowGraphEditOp[]) : [];
          console.log(`[DEBUG] flow graphEdit ops=${ops.length}`);
          runFlowGraphEdit(session, ops);
          emitInteractionState(session);
          evaluate();
          persistFixture();
          renderFrame();
          break;
        }
        case "collapse": {
          const ids = Array.isArray(args.ids) ? args.ids.filter((value): value is string => typeof value === "string") : [];
          if (ids.length >= 2) {
            (session as FlowSessionClusterApi).collapseSelection(JSON.stringify(ids));
            emitInteractionState(session);
            evaluate();
            persistFixture();
            renderFrame();
          }
          break;
        }
        case "explode": {
          const id = typeof args.id === "string" ? args.id : null;
          if (id) {
            (session as FlowSessionClusterApi).explodeCluster(id);
            emitInteractionState(session);
            evaluate();
            persistFixture();
            renderFrame();
          }
          break;
        }
        case "setSelection": {
          const ids = Array.isArray(args.ids) ? args.ids.filter((id): id is string => typeof id === "string") : [];
          session.setSelection(JSON.stringify(ids));
          emitInteractionState(session);
          renderFrame();
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
      let clusterNodeIds: string[] = [];
      try {
        const fixture = JSON.parse(session.fixtureJson()) as FlowFixtureV1;
        clusterNodeIds = fixture.widgets.filter((widget) => widget.kind === "cluster").map((widget) => widget.id);
        if (hoveredNodeId) {
          const widget = fixture.widgets.find((entry) => entry.id === hoveredNodeId);
          isImageWidget = widget?.kind === "inputImage";
        }
      } catch {
        /* fixture not ready */
      }
      const menuCtx: FlowCanvasContextMenuContext = {
        hoveredNodeId,
        selectedNodeIds: hoveredNodeId && !selectedNodeIds.includes(hoveredNodeId) ? [hoveredNodeId] : selectedNodeIds,
        clusterNodeIds,
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
      flowNotePaletteWidgetDragClient(e.clientX, e.clientY);
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
      flowNotePaletteWidgetDragClient(e.clientX, e.clientY);
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
      flowWidgetPaletteDragGhostRef.current = null;
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
    flowWidgetPaletteDragGhostRef.current = (clientX, clientY, descriptor) => {
      const session = sessionRef.current;
      if (!session) return;
      if (!descriptor) {
        session.clearGhostWidget();
        renderFrame();
        return;
      }
      const mapped = flowWidgetDropPointerToWorldRef.current?.(clientX, clientY);
      if (!mapped) {
        session.clearGhostWidget();
        renderFrame();
        return;
      }
      try {
        session.setGhostWidget(descriptor, mapped.world.x, mapped.world.y);
        renderFrame();
      } catch {
        session.clearGhostWidget();
        renderFrame();
      }
    };
    return () => {
      flowWidgetDropPointerToWorldRef.current = null;
      flowWidgetPaletteDragGhostRef.current = null;
      sessionRef.current?.clearGhostWidget();
      renderFrame();
    };
  }, [fixtureDragDrop, renderFrame]);

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
      className={cn(canvasViewportClass, "overflow-visible", className, fixtureDragActive && "ring-2 ring-inset ring-accent")}
      onPointerDownCapture={onContainerPointerDownCapture}
      onDragEnter={onDragEnter}
      onDragLeave={onDragLeave}
      onDragOver={onDragOver}
      onDrop={onDrop}
    >
      <canvas
        ref={canvasRef}
        className="absolute inset-0 z-0 block h-full w-full touch-none"
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
        className="pointer-events-none absolute inset-0 z-40 block h-full w-full"
        data-testid="flow-text-overlay"
      />
      <FlowParamOverlay
        state={paramOverlayState}
        width={viewportRef.current.width}
        height={viewportRef.current.height}
        onParamChange={onNeuronParamChange}
      />
      <FlowVariableOverlay
        fixtureJson={variableFixtureJson}
        schemas={schemaCatalogue}
        width={viewportRef.current.width}
        height={viewportRef.current.height}
        onNameChange={onVariableNameChange}
        onSchemaChange={onVariableSchemaChange}
      />
      <FlowStepperOverlay
        state={stepperOverlayState}
        width={viewportRef.current.width}
        height={viewportRef.current.height}
        onFieldChange={onStepperFieldChange}
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
      {fixtureDragDrop ? (
        <>
          <FlowWidgetPaletteDragPreviewBridge
            canvasRef={canvasRef}
            containerRef={containerRef}
            enabled={fixtureDragDrop}
            setFixtureDragActive={setFixtureDragActive}
          />
          <FlowWidgetPaletteDragEscapeBridge enabled={fixtureDragDrop} />
        </>
      ) : null}
      <ContextMenuController
        items={surfaceContextMenu?.items ?? []}
        onOpenChange={(open) => {
          if (!open) setSurfaceContextMenu(null);
        }}
        open={surfaceContextMenu !== null}
        position={surfaceContextMenu ? { x: surfaceContextMenu.clientX, y: surfaceContextMenu.clientY } : null}
      />
      <CanvasPickMenu
        request={canvasPick.pickMenu}
        hoveredKey={canvasPick.menuHoveredKey}
        onHoverKey={canvasPick.onMenuHoverKey}
        onPick={canvasPick.onMenuPick}
        onDismiss={canvasPick.dismissPickMenu}
      />
      {selectionBounds ? (
        <FlowSelectionBoundsOverlay rect={selectionBounds} selectionCount={selectionBoundsCount} onAlign={alignSelection} />
      ) : null}
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

    it("flowOverlayLabelFill uses element default, emphasized when selected or hovered, secondary when highlighted", () => {
      const element = resolveSemanticColorHex("border-element-color", "gray");
      const emphasized = resolveSemanticColorHex("border-emphasized-color", "dark");
      const secondary = resolveColorHex(tokenVar("secondary"), "secondary");
      const idle = flowElementInteractionChrome([], { ids: [], removedIds: [] });
      const selected = flowElementInteractionChrome(["node-a"], { ids: [], removedIds: [] });
      const previewSelected = flowElementInteractionChrome([], { ids: ["node-a"], removedIds: [] });
      const previewHighlighted = flowElementInteractionChrome(["node-a"], { ids: [], removedIds: ["node-a"] });
      expect(flowOverlayLabelFill("node-a", false, null, idle, [])).toBe(element);
      expect(flowOverlayLabelFill("node-a", false, "node-a", idle, [])).toBe(emphasized);
      expect(flowOverlayLabelFill("node-a", false, null, selected, [])).toBe(emphasized);
      expect(flowOverlayLabelFill("node-a", false, null, previewSelected, [])).toBe(emphasized);
      expect(flowOverlayLabelFill("node-a", false, null, previewHighlighted, [])).toBe(secondary);
      expect(flowOverlayLabelFill("node-a", false, null, idle, ["node-a"])).toBe(element);
      expect(flowOverlayLabelFill("__ghost__", true, null, idle, [])).toBe(secondary);
    });
  });

  describe("flow react fixture", () => {
    it("default fixture serializes", () => {
      const json = flowFixtureToJson(FLOW_DEFAULT_FIXTURE);
      expect(json).toContain("flow.fixture/v1");
      expect(json).toContain("math.add");
    });
  });

  describe("flowTreeDirtyNeuronIds", () => {
    const chainFixture: FlowFixtureV1 = {
      schema: "flow.fixture/v1",
      camera: { x: 0, y: 0, zoom: 1 },
      layout: { slider: { x: 0, y: 0 }, add: { x: 120, y: 0 }, pass: { x: 240, y: 0 }, preview: { x: 360, y: 0 } },
      widgets: [
        { kind: "inputSlider", id: "slider", value: 3 },
        { kind: "neuron", id: "add", neuronKind: "math.add" },
        { kind: "neuron", id: "pass", neuronKind: "math.passThrough" },
        { kind: "outputPreview", id: "preview" },
      ],
      synapses: [
        { id: "s1", from: "slider", to: "add", fromPort: "number", toPort: "a" },
        { id: "s2", from: "add", to: "pass", fromPort: "sum", toPort: "number" },
        { id: "s3", from: "pass", to: "preview", fromPort: "number", toPort: "" },
      ],
    };

    it("returns structural when previous fixture is missing", () => {
      const result = flowTreeDirtyNeuronIds(null, flowFixtureToJson(FLOW_DEFAULT_FIXTURE));
      expect(result.structural).toBe(true);
      expect(result.ids).toEqual([]);
      expect(result.path).toEqual(["slider", "add", "preview"]);
    });

    it("ignores layout and camera changes", () => {
      const base = flowFixtureToJson(chainFixture);
      const moved: FlowFixtureV1 = {
        ...chainFixture,
        camera: { x: 40, y: -20, zoom: 1.5 },
        layout: { slider: { x: 10, y: 20 }, add: { x: 130, y: 40 }, pass: { x: 250, y: 60 }, preview: { x: 370, y: 80 } },
      };
      expect(flowTreeDirtyNeuronIds(base, flowFixtureToJson(moved))).toEqual({ ids: [], path: [], structural: false });
    });

    it("ignores identical fixture snapshots for selection-only gestures", () => {
      const base = flowFixtureToJson(chainFixture);
      expect(flowTreeDirtyNeuronIds(base, base)).toEqual({ ids: [], path: [], structural: false });
    });

    it("deleting a downstream child does not dirty upstream parents", () => {
      const base = flowFixtureToJson(chainFixture);
      const afterDelete: FlowFixtureV1 = {
        ...chainFixture,
        widgets: chainFixture.widgets.filter((widget) => widget.id !== "pass"),
        synapses: chainFixture.synapses.filter((synapse) => synapse.from !== "pass" && synapse.to !== "pass"),
      };
      const result = flowTreeDirtyNeuronIds(base, flowFixtureToJson(afterDelete));
      expect(result.structural).toBe(false);
      expect(result.ids).not.toContain("add");
      expect(result.ids).not.toContain("slider");
      expect(result.path).not.toContain("add");
      expect(result.path).not.toContain("slider");
    });

    it("deleting a leaf preview leaves upstream compute nodes clean", () => {
      const base = flowFixtureToJson(chainFixture);
      const afterDelete: FlowFixtureV1 = {
        ...chainFixture,
        widgets: chainFixture.widgets.filter((widget) => widget.id !== "preview"),
        synapses: chainFixture.synapses.filter((synapse) => synapse.to !== "preview"),
      };
      expect(flowTreeDirtyNeuronIds(base, flowFixtureToJson(afterDelete))).toEqual({ ids: [], path: [], structural: false });
    });

    it("marks downstream neurons when slider value changes", () => {
      const base = flowFixtureToJson(chainFixture);
      const changed: FlowFixtureV1 = {
        ...chainFixture,
        widgets: chainFixture.widgets.map((widget) => (widget.kind === "inputSlider" ? { ...widget, value: 7 } : widget)),
      };
      expect(flowTreeDirtyNeuronIds(base, flowFixtureToJson(changed))).toEqual({
        ids: ["add", "pass"],
        path: ["slider", "add", "pass", "preview"],
        structural: false,
      });
    });

    it("marks target and downstream on reconnect", () => {
      const base = flowFixtureToJson(chainFixture);
      const reconnected: FlowFixtureV1 = {
        ...chainFixture,
        synapses: [
          { id: "s1", from: "slider", to: "add", fromPort: "number", toPort: "b" },
          { id: "s2", from: "add", to: "pass", fromPort: "sum", toPort: "number" },
          { id: "s3", from: "pass", to: "preview", fromPort: "number", toPort: "" },
        ],
      };
      expect(flowTreeDirtyNeuronIds(base, flowFixtureToJson(reconnected))).toEqual({
        ids: ["add", "pass"],
        path: ["add", "pass", "preview"],
        structural: false,
      });
    });

    it("marks downstream when a node is added upstream", () => {
      const base = flowFixtureToJson(chainFixture);
      const extended: FlowFixtureV1 = {
        ...chainFixture,
        widgets: [...chainFixture.widgets, { kind: "neuron", id: "extra", neuronKind: "math.passThrough" }],
        synapses: [
          { id: "s0", from: "extra", to: "add", fromPort: "number", toPort: "b" },
          ...chainFixture.synapses,
        ],
      };
      const extendedResult = flowTreeDirtyNeuronIds(base, flowFixtureToJson(extended));
      expect(extendedResult.ids).toEqual(expect.arrayContaining(["add", "pass", "extra"]));
      expect(extendedResult.path).toEqual(expect.arrayContaining(["extra", "add", "pass", "preview"]));
    });
  });

  describe("flowComputeProgressPayload", () => {
    it("marks upstream node active and downstream nodes stale", () => {
      const path = ["slider", "add", "pass", "preview"];
      expect(flowComputeProgressPayload(path, 0)).toEqual({ active: "slider", stale: ["add", "pass", "preview"] });
      expect(flowComputeProgressPayload(path, 1)).toEqual({ active: "add", stale: ["pass", "preview"] });
      expect(flowComputeProgressPayload(path, 3)).toEqual({ active: "preview", stale: [] });
    });
  });

  describe("flowEvalAnimationPath", () => {
    it("omits output preview and action widgets from computing animation", () => {
      const fixture: FlowFixtureV1 = {
        schema: "flow.fixture/v1",
        camera: { x: 0, y: 0, zoom: 1 },
        widgets: [
          { kind: "inputSlider", id: "slider", value: 1 },
          { kind: "neuron", id: "volume", neuronKind: "brep.measure.volume" },
          { kind: "outputPreview", id: "preview" },
        ],
        synapses: [
          { id: "s1", from: "slider", to: "volume", fromPort: "number", toPort: "geometry" },
          { id: "s2", from: "volume", to: "preview", fromPort: "volume", toPort: "" },
        ],
      };
      const path = flowFixtureComputePath(fixture);
      expect(path).toEqual(["slider", "volume", "preview"]);
      expect(flowEvalAnimationPath(path, fixture)).toEqual(["slider", "volume"]);
    });
  });

  describe("flowDirtyComputePathReady", () => {
    it("waits for upstream inputs on disconnected measure nodes", () => {
      const fixture: FlowFixtureV1 = {
        schema: "flow.fixture/v1",
        camera: { x: 0, y: 0, zoom: 1 },
        widgets: [
          { kind: "neuron", id: "box", neuronKind: "brep.prim3d.box" },
          { kind: "neuron", id: "volume", neuronKind: "brep.measure.volume" },
        ],
        synapses: [],
      };
      const json = flowFixtureToJson(fixture);
      expect(flowDirtyComputePathReady(json, ["box"])).toBe(true);
      expect(flowDirtyComputePathReady(json, ["volume"])).toBe(false);
      const connected: FlowFixtureV1 = {
        ...fixture,
        synapses: [{ id: "s1", from: "box", to: "volume", fromPort: "solid", toPort: "geometry" }],
      };
      expect(flowDirtyComputePathReady(flowFixtureToJson(connected), ["volume"])).toBe(true);
    });
  });

  describe("flow channel compatibility", () => {
    it("flowChannelCompatible requires input operators subset of output operators", () => {
      const output = { name: "geometry", code: "G", abbreviation: "Geo", fullName: "Geometry", operators: ["brep.bool.fuse", "brep.xform.translate"] };
      const compatible = { name: "geometry", code: "G", abbreviation: "Geo", fullName: "Geometry", operators: ["brep.bool.fuse"] };
      const incompatible = { name: "geometry", code: "G", abbreviation: "Geo", fullName: "Geometry", operators: ["brep.solid.extrude"] };
      expect(flowChannelCompatible(output, compatible)).toBe(true);
      expect(flowChannelCompatible(output, incompatible)).toBe(false);
      expect(flowChannelCompatible(output, { name: "any", code: "A", abbreviation: "Any", fullName: "Any", operators: [] })).toBe(true);
    });

    it("flowChannelCompatible requires output cardinality within input range", () => {
      const scalar = { name: "value", code: "V", abbreviation: "Val", fullName: "Value", operators: [], cardinality: "!" };
      const optional = { name: "value", code: "V", abbreviation: "Val", fullName: "Value", operators: [], cardinality: "?" };
      const many = { name: "list", code: "L", abbreviation: "Lst", fullName: "List", operators: [], cardinality: "*" };
      const oneOrMore = { name: "list", code: "L", abbreviation: "Lst", fullName: "List", operators: [], cardinality: "+" };
      expect(flowChannelCompatible(scalar, oneOrMore)).toBe(true);
      expect(flowChannelCompatible(optional, scalar)).toBe(false);
      expect(flowChannelCompatible(many, scalar)).toBe(false);
      expect(flowChannelCompatible(many, oneOrMore)).toBe(false);
      expect(flowChannelCompatible(oneOrMore, many)).toBe(true);
    });
  });

  describe("formatFlowEvalValue", () => {
    it("renders null channel values", () => {
      expect(formatFlowEvalValue(null)).toBe("null");
    });

    it("renders schema-component error lists", () => {
      const errors = {
        $schema: "list",
        "0": { $schema: "text", value: "missing field x" },
      };
      expect(formatFlowEvalValue(errors)).toBe("missing field x");
      expect(readFlowEvalErrors({ errors })).toEqual(["missing field x"]);
    });
  });

  describe("flow proximity connect", () => {
    it("exports default proximity distance for window options", () => {
      expect(FLOW_DEFAULT_PROXIMITY_DISTANCE).toBe(48);
    });
  });

  describe("flow store", () => {
    it("ephemeral store never loads or saves", () => {
      const store = createEphemeralFlowStore();
      store.save(flowFixtureToJson(FLOW_DEFAULT_FIXTURE));
      expect(store.load()).toBeNull();
      store.clear();
      expect(store.load()).toBeNull();
    });

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
            schemas: [],
            operators: [
              {
                id: "math.add",
                module: "math",
                name: "Add",
                abbreviation: "Add",
                icon: "emoji:➕",
                summary: "Sum",
                inputs: [
                  { name: "a", code: "A", abbreviation: "a", fullName: "A", operators: ["math.add"] },
                  { name: "b", code: "B", abbreviation: "b", fullName: "B", operators: ["math.add"], default: { $schema: "number", value: 0 } },
                ],
                outputs: [{ name: "sum", code: "S", abbreviation: "Sum", fullName: "Sum", operators: ["math.add"] }],
              },
            ],
            widgets: [],
            commands: [],
            settings: [],
          },
        }),
      );
      expect(manifest.id).toBe("math");
      expect(manifest.contributes.operators[0]?.id).toBe("math.add");
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
    it("derives drag-sized slider ranges from a value", () => {
      expect(flowSensibleSliderRange(3)).toEqual({ min: 0, max: 10, step: 1 });
      expect(flowSensibleSliderRange(-3)).toEqual({ min: -10, max: 10, step: 1 });
    });

    it("parses a single number into a sensible slider range", () => {
      const spec = flowParseSpotlightSliderQuery("5");
      expect(spec).toEqual({ value: 5, min: 0, max: 10, step: 1, label: "5" });
    });

    it("parses decimal single numbers with matching step", () => {
      const spec = flowParseSpotlightSliderQuery("1.3");
      expect(spec).toEqual({ value: 1.3, min: 0, max: 10, step: 0.1, label: "1.3" });
    });

    it("parses decimal single numbers with wider sensible max", () => {
      const spec = flowParseSpotlightSliderQuery("10.2");
      expect(spec?.value).toBe(10.2);
      expect(spec?.min).toBe(0);
      expect(spec?.max).toBe(20);
      expect(spec?.step).toBe(0.1);
    });

    it("preserves typed decimal precision for trailing zeros", () => {
      const spec = flowParseSpotlightSliderQuery("1.30");
      expect(spec).toEqual({ value: 1.3, min: 0, max: 10, step: 0.01, label: "1.30" });
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

  describe("nestable catalogue", () => {
    it("nests kinds by authored group path", () => {
      const section = nestNeuronKindsIntoCatalogueSection("brep", "Brep", [
        {
          id: "brep.prim3d.box",
          module: "brep",
          name: "Box",
          abbreviation: "Box",
          icon: "emoji:📦",
          summary: "Axis-aligned box",
          inputs: [],
          outputs: ["geometry"],
          group: ["Primitives 3D"],
        },
        {
          id: "brep.curve.line",
          module: "brep",
          name: "Line",
          abbreviation: "Line",
          icon: "emoji:〰️",
          summary: "Line edge",
          inputs: [],
          outputs: ["geometry"],
          group: ["Curves"],
        },
      ]);
      expect(section.groups?.map((group) => group.title).sort()).toEqual(["Curves", "Primitives 3D"]);
      expect(section.groups?.find((group) => group.title === "Primitives 3D")?.items[0]?.neuronKind).toBe("brep.prim3d.box");
    });

    it("flattens nested catalogue items for spotlight search", () => {
      const sections: CatalogueSection[] = [
        {
          id: "brep",
          title: "Brep",
          items: [],
          groups: [
            {
              id: "brep.primitives-3d",
              title: "Primitives 3D",
              items: [{ kind: "neuron", neuronKind: "brep.prim3d.box", name: "Box", abbreviation: "Box", icon: "emoji:📦", summary: "Box" }],
            },
          ],
        },
      ];
      expect(flattenCatalogueItems(sections).map((item) => item.neuronKind)).toContain("brep.prim3d.box");
    });

    it("builds recursive workbench tree sections", () => {
      const tree = buildCatalogueKindsTreeSections(
        [
          {
            id: "brep",
            title: "Brep",
            items: [],
            groups: [
              {
                id: "brep.primitives-3d",
                title: "Primitives 3D",
                items: [{ kind: "neuron", neuronKind: "brep.prim3d.box", name: "Box", abbreviation: "Box", icon: "emoji:📦", summary: "Box" }],
              },
            ],
          },
        ],
        "test-kinds",
      );
      const group = tree[0]?.items[0];
      expect(group?.label).toBe("Primitives 3D");
      expect(group?.items?.[0]?.draggable).toBe(true);
      expect(group?.items?.[0]?.dragData).toBeDefined();
    });

    it("tolerates omitted items arrays after wasm json round-trip", () => {
      const parsed = parseFlowCatalogueSections(
        JSON.stringify([
          {
            id: "brep",
            title: "Brep",
            groups: [{ id: "brep.solid", title: "Solid", items: [{ kind: "neuron", neuronKind: "brep.solid.extrude", name: "Extrude", abbreviation: "Ext", icon: "emoji:🧱", summary: "Extrude" }] }],
          },
        ]),
      );
      const tree = buildCatalogueKindsTreeSections(parsed, "test-kinds");
      expect(tree[0]?.items[0]?.label).toBe("Solid");
      expect(tree[0]?.items[0]?.items?.[0]?.label).toBe("Extrude");
    });
  });

  describe("flow catalogue lod display", () => {
    const item: CatalogueItem = {
      kind: "neuron",
      neuronKind: "math.multiply",
      name: "Multiply",
      abbreviation: "Mul",
      icon: "emoji:✖️",
      summary: "Product",
    };

    it("uses abbreviation at compact and detail tiers", () => {
      expect(flowCatalogueItemLodDisplay(item, "compact").primary).toBe("Mul");
      expect(flowCatalogueItemLodDisplay(item, "detail").primary).toBe("Mul");
    });

    it("uses name at normal and micro tiers", () => {
      expect(flowCatalogueItemLodDisplay(item, "normal").primary).toBe("Multiply");
      expect(flowCatalogueItemLodDisplay(item, "micro").primary).toBe("Multiply");
    });

    it("shows icon at overview and summary inline at micro", () => {
      expect(flowCatalogueItemLodDisplay(item, "overview").showIcon).toBe(true);
      expect(flowCatalogueItemLodDisplay(item, "micro").detail).toBe("Product");
      expect(flowCatalogueItemLodDisplay(item, "normal").detail).toBeNull();
    });
  });

  describe("flow spotlight suggestion scroll", () => {
    it("enables overflow scrolling only when expanded", () => {
      expect(flowSpotlightSuggestionListScrollClass(false, "normal")).toContain("overflow-hidden");
      expect(flowSpotlightSuggestionListScrollClass(false, "normal")).not.toContain("overflow-y-auto");
      expect(flowSpotlightSuggestionListScrollClass(true, "normal")).toContain("overflow-y-auto");
      expect(flowSpotlightSuggestionListScrollClass(true, "normal")).toContain("max-h-[min(24rem,70vh)]");
    });

    it("uses tighter caps at overview lod", () => {
      expect(flowSpotlightSuggestionListScrollClass(true, "overview")).toContain("max-h-[min(12rem,50vh)]");
    });
  });

  describe("flow catalogue suggestions", () => {
    const sections: CatalogueSection[] = [
      {
        id: "math",
        title: "Math",
        items: [
          { kind: "neuron", neuronKind: "math.add", name: "Add", abbreviation: "Add", icon: "emoji:➕", summary: "Sum" },
          { kind: "neuron", neuronKind: "math.multiply", name: "Multiply", abbreviation: "Mul", icon: "emoji:✖️", summary: "Product" },
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

    it("ranks brep schema component for brep query", () => {
      const brepSections: CatalogueSection[] = [
        {
          id: "brep",
          title: "Brep",
          items: [],
          groups: [
            {
              id: "brep.schemas",
              title: "Schemas",
              items: [
                {
                  kind: "neuron",
                  neuronKind: "brep.brep",
                  name: "Brep",
                  abbreviation: "Brep",
                  icon: "emoji:🧊",
                  summary: "Construct, deconstruct, or modify a brep",
                },
              ],
            },
            {
              id: "brep.primitives",
              title: "Primitives 3D",
              items: [{ kind: "neuron", neuronKind: "brep.prim3d.box", name: "Box", abbreviation: "Box", icon: "emoji:📦", summary: "Box solid" }],
            },
          ],
        },
      ];
      const ranked = flowRankCatalogueSuggestions(brepSections, "brep");
      expect(ranked[0]?.neuronKind).toBe("brep.brep");
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

  describe("flow widget palette drag preview", () => {
    it("flowReadActivePaletteDragEncoded prefers pointer payload over html5 ref", () => {
      flowWidgetPalettePointerDragRef.encoded = '{"kind":"neuron","neuronKind":"math.add"}';
      flowWidgetPaletteDragEncodedRef.current = '{"kind":"inputSlider"}';
      expect(flowReadActivePaletteDragEncoded()).toContain("math.add");
      flowWidgetPalettePointerDragRef.encoded = null;
      flowWidgetPaletteDragEncodedRef.current = null;
    });

    it("flowNotePaletteWidgetDragClient updates client ref and invokes ghost sync", () => {
      const calls: Array<{ clientX: number; clientY: number; descriptor: string | null }> = [];
      flowWidgetPaletteDragGhostRef.current = (clientX, clientY, descriptor) => {
        calls.push({ clientX, clientY, descriptor });
      };
      beginFlowWidgetPalettePointerDrag('{"kind":"neuron","neuronKind":"math.add"}');
      flowNotePaletteWidgetDragClient(48, 72);
      expect(flowWidgetPaletteDragClientRef).toEqual({ clientX: 48, clientY: 72 });
      expect(calls.at(-1)?.descriptor).toContain("math.add");
      abortFlowWidgetPaletteDrag();
      flowWidgetPaletteDragGhostRef.current = null;
    });

    it("abortFlowWidgetPaletteDrag clears active drag state", () => {
      beginFlowWidgetPalettePointerDrag('{"kind":"neuron","neuronKind":"math.add"}');
      abortFlowWidgetPaletteDrag();
      expect(flowReadActivePaletteDragEncoded()).toBeNull();
      expect(flowWidgetPaletteDragRef.active).toBe(false);
    });
  });

  describe("flow context menu", () => {
    const baseCtx: FlowCanvasContextMenuContext = {
      hoveredNodeId: null,
      selectedNodeIds: [],
      clusterNodeIds: [],
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

    it("multi-selection menu includes collapse to cluster", () => {
      const items = buildFlowContextMenuItems(
        { ...baseCtx, isBackground: false, selectedNodeIds: ["a", "b"] },
        () => {},
      );
      expect(items.some((item) => item.id === "flow.ctx.collapse")).toBe(true);
    });

    it("cluster menu includes explode", () => {
      const items = buildFlowContextMenuItems(
        { ...baseCtx, isBackground: false, selectedNodeIds: ["cluster_1"], clusterNodeIds: ["cluster_1"] },
        () => {},
      );
      expect(items.some((item) => item.id === "flow.ctx.explode")).toBe(true);
    });

    it("image widget menu includes replace image", () => {
      const items = buildFlowContextMenuItems(
        { ...baseCtx, hoveredNodeId: "img", isBackground: false, isImageWidget: true, selectedNodeIds: ["img"] },
        () => {},
      );
      expect(items.some((item) => item.id === "flow.ctx.replaceImage")).toBe(true);
    });
  });

  describe("flow selection bounds", () => {
    it("parses selection union bounds screen json", () => {
      expect(parseFlowSelectionUnionBoundsScreen("null")).toBeNull();
      expect(parseFlowSelectionUnionBoundsScreen(JSON.stringify({ x: 10, y: 20, width: 100, height: 50 }))).toEqual({
        x: 10,
        y: 20,
        width: 100,
        height: 50,
      });
      expect(parseFlowSelectionUnionBoundsScreen("{}")).toBeNull();
    });

    it("compares selection union bounds for stable overlay updates", () => {
      const rect = { x: 1, y: 2, width: 3, height: 4 };
      expect(flowSelectionUnionBoundsEqual(rect, { ...rect })).toBe(true);
      expect(flowSelectionUnionBoundsEqual(rect, { ...rect, x: 2 })).toBe(false);
      expect(flowSelectionUnionBoundsEqual(null, null)).toBe(true);
    });

    it("maps pointer hits to align modes outside the selection chrome", () => {
      const rect = { x: 100, y: 80, width: 200, height: 120 };
      const regions = flowSelectionAlignHitRegions(rect, 2);
      expect(regions.length).toBe(6);
      expect(regions.map((region) => region.mode)).toEqual([
        "alignLeft",
        "alignHorizontal",
        "alignRight",
        "alignTop",
        "alignVertical",
        "alignBottom",
      ]);
      expect(regions[0]!.rect.y + regions[0]!.rect.height).toBeCloseTo(rect.y, 5);
      expect(regions[3]!.rect.x).toBeGreaterThanOrEqual(rect.x + rect.width);
      const centerX = rect.x + rect.width / 2;
      const centerY = rect.y + rect.height / 2;
      const topRowMidX = (regions[0]!.rect.x + regions[2]!.rect.x + regions[2]!.rect.width) / 2;
      const rightColMidY = (regions[3]!.rect.y + regions[5]!.rect.y + regions[5]!.rect.height) / 2;
      expect(topRowMidX).toBeCloseTo(centerX, 5);
      expect(rightColMidY).toBeCloseTo(centerY, 5);
      const left = regions[0]!.rect;
      expect(flowPointerHitsSelectionAlign(left.x + 4, left.y + 4, rect, 2)).toBe("alignLeft");
      expect(flowPointerHitsSelectionAlign(rect.x + rect.width / 2, rect.y + rect.height / 2, rect, 2)).toBeNull();
      const distributeRegions = flowSelectionAlignHitRegions(rect, 3);
      expect(distributeRegions.length).toBe(8);
      expect(distributeRegions[6]!.mode).toBe("distributeVertical");
      expect(distributeRegions[6]!.rect.x + distributeRegions[6]!.rect.width).toBeLessThanOrEqual(rect.x);
      expect(distributeRegions[6]!.rect.y + distributeRegions[6]!.rect.height / 2).toBeCloseTo(centerY, 5);
      expect(distributeRegions[7]!.mode).toBe("distributeHorizontal");
      expect(distributeRegions[7]!.rect.y).toBeCloseTo(rect.y + rect.height, 5);
      expect(distributeRegions[7]!.rect.x + distributeRegions[7]!.rect.width / 2).toBeCloseTo(centerX, 5);
    });

    it("aligns selected widgets through FlowSession wasm", async () => {
      await ensureFlowWasmLoaded();
      const session = new FlowSession();
      const fixture: FlowFixtureV1 = {
        ...FLOW_DEFAULT_FIXTURE,
        layout: {
          slider: { x: -80, y: 10 },
          add: { x: 160, y: -20 },
          preview: { x: 320, y: 0 },
        },
      };
      session.loadFixtureJson(flowFixtureToJson(fixture));
      session.setSelection(JSON.stringify(["slider", "add"]));
      session.alignSelection("alignTop");
      const layout = (JSON.parse(session.fixtureJson()) as FlowFixtureV1).layout ?? {};
      expect(layout.slider?.y).toBe(layout.add?.y);
      session.alignSelection("alignLeft");
      const leftAligned = (JSON.parse(session.fixtureJson()) as FlowFixtureV1).layout ?? {};
      expect(leftAligned.add?.x).not.toBe(160);
      expect(leftAligned.slider?.x).toBe(-80);
    });

    it("undoes and redoes align selection through FlowSession wasm", async () => {
      await ensureFlowWasmLoaded();
      const session = new FlowSession();
      const fixture: FlowFixtureV1 = {
        ...FLOW_DEFAULT_FIXTURE,
        layout: {
          slider: { x: -80, y: 10 },
          add: { x: 160, y: -20 },
          preview: { x: 320, y: 0 },
        },
      };
      session.loadFixtureJson(flowFixtureToJson(fixture));
      const before = (JSON.parse(session.fixtureJson()) as FlowFixtureV1).layout ?? {};
      session.setSelection(JSON.stringify(["slider", "add"]));
      session.alignSelection("alignTop");
      const aligned = (JSON.parse(session.fixtureJson()) as FlowFixtureV1).layout ?? {};
      expect(aligned.slider?.y).toBe(aligned.add?.y);
      expect(session.canUndo()).toBe(true);
      expect(session.undo()).toBe(true);
      const undone = (JSON.parse(session.fixtureJson()) as FlowFixtureV1).layout ?? {};
      expect(undone.slider?.y).toBe(before.slider?.y);
      expect(undone.add?.y).toBe(before.add?.y);
      expect(session.canRedo()).toBe(true);
      expect(session.redo()).toBe(true);
      const redone = (JSON.parse(session.fixtureJson()) as FlowFixtureV1).layout ?? {};
      expect(redone.slider?.y).toBe(aligned.slider?.y);
      expect(redone.add?.y).toBe(aligned.add?.y);
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

  describe("flow graph edit", () => {
    it("runFlowGraphEdit dispatches primitive ops to session", () => {
      const calls: string[] = [];
      const session = {
        addWidget: () => calls.push("addWidget"),
        connectPorts: () => calls.push("connectPorts"),
        disconnect: () => calls.push("disconnect"),
        insertBetween: () => calls.push("insertBetween"),
        moveWidget: () => calls.push("moveWidget"),
        makeSpace: () => calls.push("makeSpace"),
        previewOffWidgetIds: () => "[]",
        setPreviewOff: () => calls.push("setPreviewOff"),
        setSliderValue: () => calls.push("setSliderValue"),
        setNeuronParams: () => calls.push("setNeuronParams"),
      } as unknown as FlowSession;
      runFlowGraphEdit(session, [
        { op: "makeSpace", anchor: "a", dx: 120, dy: 0 },
        { op: "addWidget", descriptor: "{}", x: 0, y: 0 },
        { op: "insertBetween", anchor: "a", anchorOutPort: "solid", mid: "b", midInPort: "geometry", midOutPort: "geometry" },
        { op: "setNeuronParams", id: "b", paramsJson: "{}" },
      ]);
      expect(calls).toEqual(["makeSpace", "addWidget", "insertBetween", "setNeuronParams"]);
    });
  });
}
// #endregion 🧪Tests
