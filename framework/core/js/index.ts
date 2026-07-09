// #region 🧲Header
/** @emoji 🧭 `@semio-tech/framework-core` — shared canvas pick helpers, layout factories, and inspector utilities for UI renderers. */
// #endregion 🧲Header

export const CANVAS_HOVER_SOURCE_CANVAS = "canvas";
export const CANVAS_HOVER_SOURCE_PICK_MENU = "pick-menu";
export const CANVAS_HOVER_SOURCE_CATALOG = "catalog";
export const CANVAS_HOVER_SOURCE_DOCUMENT = "document";

export const FRAMEWORK_PANEL_TAB_DOCUMENT_ID = "framework.panel.document";
export const FRAMEWORK_PANEL_TAB_CATALOGUE_ID = "framework.panel.catalogue";
export const FRAMEWORK_PANEL_TAB_INSPECTION_ID = "framework.panel.inspection";
export const FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL = "Document";
export const FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL = "Catalogue";
export const FRAMEWORK_PANEL_TAB_INSPECTION_LABEL = "Inspection";
export const FRAMEWORK_PANEL_TAB_DOCUMENT_ICON_ID = "framework.panel.document";
export const FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID = "framework.panel.catalogue";
export const FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID = "framework.panel.inspection";
export const FRAMEWORK_PANEL_TAB_PARAMETERS_ID = "framework.panel.parameters";
export const FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL = "Parameters";
export const FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID = "framework.panel.parameters";

export const UI_INSPECTOR_MIXED_PLACEHOLDER = "Mixed";

export type CanvasPickTarget = {
  readonly domain: string;
  readonly id: string;
  readonly generality: number;
  readonly label: string;
  readonly kind?: string;
};

export type CanvasPickRequest = {
  readonly targets: readonly CanvasPickTarget[];
  readonly client: { readonly x: number; readonly y: number };
  readonly modifiers?: Readonly<Record<string, boolean>>;
};

export type CanvasHoverFocus = {
  readonly sourceId: string;
  readonly target: CanvasPickTarget | null;
};

export type CommandDescriptor = {
  readonly controllerId: string;
  readonly command: string;
  readonly args?: unknown;
};

export type WindowLayoutWindowNode = {
  readonly kind: "window";
  readonly windowKindId: string;
  readonly title?: string;
  readonly instanceId?: string;
  readonly templateId?: string;
};

export type WindowLayoutStackNode = {
  readonly kind: "stack";
  readonly size?: number;
  readonly children: readonly WindowLayoutWindowNode[];
};

export type WindowLayoutAxisNode = {
  readonly kind: "row" | "column";
  readonly size?: number;
  readonly children: readonly (WindowLayoutAxisNode | WindowLayoutStackNode)[];
};

export type WindowLayout = {
  readonly root: WindowLayoutAxisNode | WindowLayoutStackNode;
};

export type NamedLayout = {
  readonly id: string;
  readonly label: string;
  readonly iconId?: string;
  readonly layout: WindowLayout;
  readonly origin: "builtin" | "user";
  readonly groupPath?: readonly string[];
};

export enum Expertise {
  BEGINNER = "beginner",
  NORMAL = "normal",
  EXPERT = "expert",
}

export type ToolLeaf =
  | { readonly id: string; readonly kind: "separator"; readonly order?: number; readonly disabled?: boolean }
  | {
      readonly id: string;
      readonly kind: "button";
      readonly iconId: string;
      readonly label?: string;
      readonly text?: string;
      readonly title?: string;
      readonly order?: number;
      readonly disabled?: boolean;
      readonly controllerId?: string;
      readonly command?: string;
      readonly args?: unknown;
    }
  | {
      readonly id: string;
      readonly kind: "toggle";
      readonly iconId: string;
      readonly label?: string;
      readonly text?: string;
      readonly title?: string;
      readonly order?: number;
      readonly pressed?: boolean;
      readonly disabled?: boolean;
      readonly controllerId?: string;
      readonly command?: string;
      readonly args?: unknown;
    };

export type ToolNode =
  | ToolLeaf
  | {
      readonly id: string;
      readonly kind: "collection";
      readonly iconId: string;
      readonly label?: string;
      readonly text?: string;
      readonly title?: string;
      readonly order?: number;
      readonly disabled?: boolean;
      readonly children: readonly ToolNode[];
    };

export type UiSectionNode = {
  readonly type: "section";
  readonly id: string;
  readonly label?: string;
  readonly defaultOpen?: boolean;
  readonly children: readonly UiNode[];
};

export type UiTreeItemNode = {
  readonly id: string;
  readonly label: string;
  readonly description?: string;
  readonly icon?: string;
  readonly selected?: boolean;
  readonly defaultOpen?: boolean;
  readonly command?: CommandDescriptor;
  readonly draggable?: boolean;
  readonly dragData?: Readonly<Record<string, string>>;
  readonly items?: readonly UiTreeItemNode[];
  readonly control?: UiControlNode;
  readonly isHidden?: boolean;
};

export type UiTreeSectionNode = {
  readonly id: string;
  readonly label?: string;
  readonly defaultOpen?: boolean;
  readonly items: readonly UiTreeItemNode[];
};

export type UiTreeNode = {
  readonly type: "tree";
  readonly sections: readonly UiTreeSectionNode[];
  readonly selectedIds?: readonly string[];
  readonly highlightedIds?: readonly string[];
  readonly selectionChange?: CommandDescriptor;
};

export type UiControlNode = UiInputNode | UiSelectNode | UiToggleNode | UiVec3Node | UiButtonNode | UiKeyValueNode | UiSliderNode | UiNumberStepperNode | UiRingNode | UiIconSelectNode;

export type UiInputNode = {
  readonly type: "input";
  readonly id: string;
  readonly inputKind: "text" | "number";
  readonly value: string;
  readonly placeholder?: string;
  readonly commit?: "change" | "blur";
  readonly onChange: CommandDescriptor;
};

export type UiSelectNode = {
  readonly type: "select";
  readonly id: string;
  readonly value: string;
  readonly items: readonly { readonly value: string; readonly label: string }[];
  readonly placeholder?: string;
  readonly onChange: CommandDescriptor;
};

export type UiToggleNode = {
  readonly type: "toggle";
  readonly id: string;
  readonly iconId: string;
  readonly pressed: boolean;
  readonly text?: string;
  readonly onChange: CommandDescriptor;
};

export type UiVec3Node = {
  readonly type: "vec3";
  readonly id: string;
  readonly value: readonly [number, number, number] | null;
  readonly onChange: CommandDescriptor;
};

export type UiKeyValueNode = {
  readonly type: "keyValue";
  readonly entries: readonly { readonly label: string; readonly value: string }[];
};

export type UiSliderNode = {
  readonly type: "slider";
  readonly id: string;
  readonly value: number;
  readonly min: number;
  readonly max: number;
  readonly step: number;
  readonly onChange: CommandDescriptor;
};

export type UiNumberStepperNode = {
  readonly type: "numberStepper";
  readonly id: string;
  readonly value: number;
  readonly step: number;
  readonly uniform: boolean;
  readonly onAbsolute: CommandDescriptor;
  readonly onDelta: CommandDescriptor;
};

export type UiRingNode = {
  readonly type: "ring";
  readonly id: string;
  readonly orbId: string;
  readonly t: number;
  readonly disabled?: boolean;
  readonly onChange: CommandDescriptor;
};

export type UiIconSelectNode = {
  readonly type: "iconSelect";
  readonly id: string;
  readonly value: string;
  readonly uniform: boolean;
  readonly classifierKind: "puzzle2d";
  readonly onChange: CommandDescriptor;
};

export type UiFieldNode = {
  readonly type: "field";
  readonly id: string;
  readonly label: string;
  readonly child: UiControlNode;
};

export type UiButtonNode = {
  readonly type: "button";
  readonly id?: string;
  readonly iconId: string;
  readonly label: string;
  readonly command: CommandDescriptor;
};

export type UiTextNode = {
  readonly type: "text";
  readonly value: string;
  readonly emphasize?: boolean;
};

export type UiNode =
  | { readonly type: "stack"; readonly direction: string; readonly gap?: string; readonly padding?: string; readonly children: readonly UiNode[] }
  | UiTextNode
  | UiButtonNode
  | { readonly type: "separator" }
  | UiSectionNode
  | UiInputNode
  | UiSelectNode
  | UiToggleNode
  | UiVec3Node
  | UiKeyValueNode
  | UiSliderNode
  | UiNumberStepperNode
  | UiRingNode
  | UiIconSelectNode
  | UiFieldNode
  | UiTreeNode;

export type UiInspectorFieldGroup = {
  readonly id: string;
  readonly label: string;
  readonly defaultOpen?: boolean;
  readonly fields: readonly UiNode[];
};

export function canvasPickTargetKey(target: CanvasPickTarget): string {
  return `${target.domain}:${target.id}`;
}

/** @emoji 🪪 Parses a pick target key into domain and id. */
export function parseCanvasPickTargetKey(key: string): { readonly domain: string; readonly id: string } | null {
  const colon = key.indexOf(":");
  if (colon < 0) return null;
  return { domain: key.slice(0, colon), id: key.slice(colon + 1) };
}

export function sortCanvasPickTargetsGeneralFirst(targets: readonly CanvasPickTarget[]): readonly CanvasPickTarget[] {
  return [...targets].sort((left, right) => left.generality - right.generality || left.label.localeCompare(right.label));
}

export function pickMostSpecificCanvasTarget(targets: readonly CanvasPickTarget[]): CanvasPickTarget | null {
  if (targets.length === 0) return null;
  return [...targets].sort((left, right) => right.generality - left.generality)[0] ?? null;
}

export function canvasHoverFocusFromTarget(sourceId: string, target: CanvasPickTarget | null): CanvasHoverFocus {
  return { sourceId, target };
}

export function createWindowLayout(windowKindId: string, title?: string, options?: { readonly instanceId?: string; readonly templateId?: string }): WindowLayoutWindowNode {
  return {
    kind: "window",
    windowKindId,
    ...(title ? { title } : {}),
    ...(options?.instanceId ? { instanceId: options.instanceId } : {}),
    ...(options?.templateId ? { templateId: options.templateId } : {}),
  };
}

export function createStackLayout(windowKindIds: readonly string[], titles?: readonly string[]): WindowLayout {
  return {
    root: {
      kind: "stack",
      children: windowKindIds.map((windowKindId, index) => createWindowLayout(windowKindId, titles?.[index])),
    },
  };
}

export function createDefaultLayout(windowIds: readonly string[], direction: "row" | "column" = "row", sizes?: readonly number[], titles?: readonly string[]): WindowLayout {
  return {
    root: {
      kind: direction,
      children: windowIds.map((id, index) => ({
        kind: "stack" as const,
        ...(sizes?.[index] !== undefined ? { size: sizes[index] } : {}),
        children: [createWindowLayout(id, titles?.[index] ?? id)],
      })),
    },
  };
}

export function createTabStackLayout(windowIds: readonly string[], titles?: readonly string[]): WindowLayout {
  return createStackLayout(windowIds, titles);
}

export function createNamedLayout(id: string, label: string, layout: WindowLayout, origin: NamedLayout["origin"] = "builtin", iconId?: string, groupPath?: readonly string[]): NamedLayout {
  return {
    id,
    label,
    layout,
    origin,
    ...(iconId ? { iconId } : {}),
    ...(groupPath?.length ? { groupPath } : {}),
  };
}

export function mergeById<T extends { id: string }>(base: readonly T[] | undefined, extension: readonly T[] | undefined): T[] | undefined {
  if (!base?.length && !extension?.length) return undefined;
  const merged = new Map<string, T>();
  base?.forEach((entry) => merged.set(entry.id, entry));
  extension?.forEach((entry) => merged.set(entry.id, entry));
  return [...merged.values()];
}

export function mergeNamedLayouts(base: readonly NamedLayout[] | undefined, extension: readonly NamedLayout[] | undefined): NamedLayout[] {
  return mergeById(base, extension) ?? [];
}

export type PlatformSubscriber = () => void;

export abstract class Store<TSnapshot> {
  private readonly listeners = new Set<PlatformSubscriber>();
  private disposed = false;

  abstract getSnapshot(): TSnapshot;

  subscribe(listener: PlatformSubscriber): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  protected notify(): void {
    if (this.disposed) return;
    for (const listener of this.listeners) listener();
  }

  dispose(): void {
    this.disposed = true;
    this.listeners.clear();
  }
}

export interface StoragePort {
  get(key: string): string | null;
  set(key: string, value: string): void;
  remove(key: string): void;
}

function namedLayoutStorageKey(appId: string): string {
  return `compose.display.layouts.${appId}`;
}

export class NamedLayoutStore extends Store<readonly NamedLayout[]> {
  private layouts: NamedLayout[] = [];

  constructor(
    private readonly appId: string,
    private readonly storage: StoragePort,
  ) {
    super();
    this.layouts = this.readPersisted();
  }

  getSnapshot(): readonly NamedLayout[] {
    return this.layouts;
  }

  save(layout: NamedLayout): void {
    const next = mergeNamedLayouts(
      this.layouts.filter((entry) => entry.id !== layout.id),
      [layout],
    );
    this.layouts = next;
    this.persist();
    this.notify();
  }

  remove(layoutId: string): void {
    const next = this.layouts.filter((entry) => entry.id !== layoutId);
    if (next.length === this.layouts.length) return;
    this.layouts = next;
    this.persist();
    this.notify();
  }

  private readPersisted(): NamedLayout[] {
    const raw = this.storage.get(namedLayoutStorageKey(this.appId));
    if (!raw) return [];
    try {
      const parsed = JSON.parse(raw) as unknown;
      if (!Array.isArray(parsed)) return [];
      return parsed.filter(
        (entry): entry is NamedLayout =>
          Boolean(entry) && typeof entry === "object" && typeof (entry as NamedLayout).id === "string" && typeof (entry as NamedLayout).label === "string" && (entry as NamedLayout).origin === "user" && Boolean((entry as NamedLayout).layout),
      );
    } catch {
      return [];
    }
  }

  private persist(): void {
    this.storage.set(namedLayoutStorageKey(this.appId), JSON.stringify(this.layouts));
  }
}

export function createBrowserStoragePort(): StoragePort {
  return {
    get: (key) => {
      try {
        return typeof localStorage !== "undefined" ? localStorage.getItem(key) : null;
      } catch {
        return null;
      }
    },
    set: (key, value) => {
      try {
        if (typeof localStorage !== "undefined") localStorage.setItem(key, value);
      } catch {
        /* ignore */
      }
    },
    remove: (key) => {
      try {
        if (typeof localStorage !== "undefined") localStorage.removeItem(key);
      } catch {
        /* ignore */
      }
    },
  };
}

export function uiInspectorAllEqual<T>(values: readonly T[]): boolean {
  if (values.length <= 1) return true;
  const first = values[0];
  for (let index = 1; index < values.length; index += 1) {
    if (values[index] !== first) return false;
  }
  return true;
}

export function uiInspectorMixedText(values: readonly string[]): { readonly value: string; readonly placeholder?: string } {
  const uniform = uiInspectorAllEqual(values);
  return { value: uniform ? (values[0] ?? "") : "", placeholder: uniform ? undefined : UI_INSPECTOR_MIXED_PLACEHOLDER };
}

export function uiInspectorMixedNumber(values: readonly number[]): { readonly value: number; readonly uniform: boolean } {
  const uniform = uiInspectorAllEqual(values);
  return { value: uniform ? (values[0] ?? 0) : Number.NaN, uniform };
}

export function uiInspectorMixedSelect(values: readonly string[]): { readonly value: string; readonly placeholder?: string } {
  return uiInspectorMixedText(values);
}

export function uiInspectorMixedToggle(values: readonly boolean[]): { readonly pressed: boolean; readonly uniform: boolean } {
  const uniform = uiInspectorAllEqual(values);
  return { pressed: uniform ? (values[0] ?? false) : false, uniform };
}

export function uiInspectorMixedSlider(values: readonly number[]): { readonly value: number; readonly uniform: boolean } {
  return uiInspectorMixedNumber(values);
}

export function uiInspectorMixedVec3(values: readonly (readonly [number, number, number])[]): { readonly value: readonly [number, number, number] | null; readonly uniform: boolean } {
  const uniform = uiInspectorAllEqual(values.map((row) => JSON.stringify(row)));
  return { value: uniform && values[0] ? values[0] : null, uniform };
}

export function uiInspectorGroupsToTree(groups: readonly UiInspectorFieldGroup[]): UiTreeNode {
  return uiDeclarativeSectionsToTree(
    groups
      .filter((group) => group.fields.length > 0)
      .map((group) => ({
        type: "section" as const,
        id: group.id,
        label: group.label,
        defaultOpen: group.defaultOpen ?? true,
        children: group.fields,
      })),
  );
}

export function uiDeclarativeSectionsToTree(sections: readonly UiSectionNode[]): UiTreeNode {
  const treeSections: UiTreeSectionNode[] = sections.map((section) => ({
    id: section.id,
    label: section.label,
    defaultOpen: section.defaultOpen ?? true,
    items: section.children.map((child, index) => uiDeclarativeChildToTreeItem(child, `${section.id}.${index}`)),
  }));
  return {
    type: "tree",
    sections: treeSections.length ? treeSections : [{ id: "empty", items: [{ id: "empty", label: "—" }] }],
  };
}

function uiDeclarativeChildToTreeItem(node: UiNode, fallbackId: string): UiTreeItemNode {
  if (node.type === "text") return { id: `${fallbackId}.text`, label: node.value };
  if (node.type === "field") {
    if (node.child.type === "text") return { id: node.id, label: node.label, description: node.child.value };
    return { id: node.id, label: node.label, control: node.child };
  }
  if (node.type === "button") return { id: node.id ?? fallbackId, label: node.label, control: node };
  if (node.type === "input" || node.type === "select" || node.type === "toggle" || node.type === "vec3" || node.type === "keyValue" || node.type === "slider" || node.type === "numberStepper" || node.type === "ring" || node.type === "iconSelect") {
    return { id: "id" in node ? String(node.id) : fallbackId, label: "", control: node };
  }
  if (node.type === "separator") return { id: `${fallbackId}.sep`, label: "—" };
  return { id: fallbackId, label: node.type };
}

//#region PluginRuntime
export type PluginViewState = {
  readonly activeModeId?: string;
  readonly activeWindowKindId?: string;
  readonly selectionJson?: string;
  readonly panelJson?: string;
  readonly contributionsJson?: string;
};

export type PluginUiNode = Record<string, unknown> & { readonly type: string };

export type PluginContribution = {
  readonly kind: "formsQuestionKind";
  readonly appId: string;
  readonly questionKind: string;
  readonly label: string;
  readonly iconId: string;
  readonly defaultValueJson?: string;
  readonly paramsBodyKey: string;
  readonly previewBodyKey: string;
};

export type PluginContributionEntry = {
  readonly pluginId: string;
  readonly contribution: PluginContribution;
};

export type PluginManifest = {
  readonly pluginId: string;
  readonly label: string;
  readonly version: string;
  readonly apps: readonly Record<string, unknown>[];
  readonly programs: readonly {
    readonly programId: string;
    readonly appId: string;
    readonly label: string;
    readonly yields: string;
  }[];
  readonly examples: readonly { readonly id: string; readonly label: string; readonly documentJson: string; readonly appId: string }[];
  readonly contributions?: readonly PluginContribution[];
};

export type PluginWasmHandle = {
  readonly pluginId: string;
  readonly manifest: PluginManifest;
  readonly createApp: (appId: string) => Promise<number>;
  readonly destroyApp: (instanceId: number) => Promise<void>;
  readonly handleCommand: (instanceId: number, commandJson: string, viewState: PluginViewState) => Promise<string[]>;
  readonly render: (instanceId: number, bodyKey: string, viewState: PluginViewState) => Promise<PluginUiNode>;
  readonly renderWithDocument?: (instanceId: number, bodyKey: string, viewState: PluginViewState, documentJson: string) => Promise<PluginUiNode>;
  readonly tools: (instanceId: number, viewState: PluginViewState) => Promise<readonly Record<string, unknown>[]>;
  readonly windowEngagements: (instanceId: number, viewState: PluginViewState) => Promise<Readonly<Record<string, Record<string, unknown>>>>;
  readonly windowMeasures: (instanceId: number, viewState: PluginViewState) => Promise<Readonly<Record<string, readonly Record<string, unknown>[]>>>;
  readonly dispose: () => void;
};

export function buildContributionsJson(loaded: ReadonlyArray<{ readonly pluginId: string; readonly manifest: PluginManifest }>): string {
  const entries: PluginContributionEntry[] = [];
  for (const entry of loaded) {
    for (const contribution of entry.manifest.contributions ?? []) {
      entries.push({ pluginId: entry.pluginId, contribution });
    }
  }
  return JSON.stringify(entries);
}

export const PLAYGROUND_PLUGIN_REGISTRY_IDS: Readonly<Record<string, string>> = {
  procedural3d: "procedural",
  procedural2d: "procedural",
  puzzle2d: "puzzle",
  puzzle3d: "puzzle",
  puzzle5d: "puzzle",
  "trinity-rewrite": "trinity",
  gis2d: "gis",
  "reasoning-wires": "reasoning-mindmap",
};

export const PLAYGROUND_DEFAULT_APP_IDS: Readonly<Record<string, string>> = {
  procedural3d: "procedural3d-play",
  procedural2d: "procedural2d-play",
  puzzle2d: "puzzle2d-play",
  puzzle3d: "puzzle3d-play",
  puzzle5d: "puzzle5d-play",
  "trinity-rewrite": "trinity-rewrite-play",
  gis2d: "gis2d-play",
  "reasoning-wires": "reasoning-wires-play",
};

export function resolveLayoutForMode(
  app: { readonly defaultLayout?: WindowLayout; readonly namedLayouts?: readonly NamedLayout[]; readonly modes: readonly { readonly id: string; readonly layoutId?: string }[] },
  modeId: string,
): WindowLayout | undefined {
  const mode = app.modes.find((entry) => entry.id === modeId);
  if (mode?.layoutId) {
    const named = app.namedLayouts?.find((entry) => entry.id === mode.layoutId);
    if (named) return named.layout;
  }
  return app.defaultLayout;
}

export function resolvePluginRegistryId(playgroundPluginId: string): string {
  return PLAYGROUND_PLUGIN_REGISTRY_IDS[playgroundPluginId] ?? playgroundPluginId;
}

export function resolvePlaygroundDefaultAppId(playgroundPluginId: string): string | undefined {
  return PLAYGROUND_DEFAULT_APP_IDS[playgroundPluginId];
}

export function contributorPluginIdsFor(registryId: string): readonly string[] {
  if (registryId === "forms" || registryId === "procedural") return ["forms-module-procedural"];
  return [];
}

export function expandPluginRegistry(plugins: readonly PluginRegistryEntry[], primaryPluginId?: string, studioMode = false): readonly PluginRegistryEntry[] {
  if (studioMode || !primaryPluginId) return plugins;
  const registryId = resolvePluginRegistryId(primaryPluginId);
  const extraIds = new Set(contributorPluginIdsFor(registryId));
  return [...plugins.filter((entry) => entry.pluginId === registryId), ...plugins.filter((entry) => entry.pluginId !== registryId && extraIds.has(entry.pluginId))];
}

export type ExternalSlotResolverContext = {
  readonly plugins: ReadonlyMap<string, PluginWasmHandle>;
  readonly contributorInstances: Map<string, number>;
  readonly viewState: PluginViewState;
};

export async function ensureContributorInstance(pluginId: string, appId: string, context: ExternalSlotResolverContext): Promise<number | null> {
  const existing = context.contributorInstances.get(pluginId);
  if (existing != null) return existing;
  const handle = context.plugins.get(pluginId);
  if (!handle) return null;
  const instanceId = await handle.createApp(appId);
  context.contributorInstances.set(pluginId, instanceId);
  return instanceId;
}

export async function resolveExternalSlots(node: PluginUiNode, context: ExternalSlotResolverContext): Promise<PluginUiNode> {
  if (node.type === "externalSlot") {
    const pluginId = String(node.pluginId ?? "");
    const appId = String(node.appId ?? pluginId);
    const bodyKey = String(node.bodyKey ?? "");
    const paramsJson = String(node.paramsJson ?? "{}");
    const handle = context.plugins.get(pluginId);
    if (!handle) {
      return { type: "text", value: `Extension unavailable: ${pluginId}` };
    }
    const instanceId = await ensureContributorInstance(pluginId, appId, context);
    if (instanceId == null) {
      return { type: "text", value: `Extension unavailable: ${pluginId}` };
    }
    const rendered = handle.renderWithDocument ? await handle.renderWithDocument(instanceId, bodyKey, context.viewState, paramsJson) : await handle.render(instanceId, bodyKey, context.viewState);
    return resolveExternalSlots(rendered, context);
  }
  if (node.type === "stack" && Array.isArray(node.children)) {
    const children = await Promise.all(node.children.map((child) => resolveExternalSlots(child as PluginUiNode, context)));
    return { ...node, children };
  }
  if (node.type === "section" && Array.isArray(node.children)) {
    const children = await Promise.all(node.children.map((child) => resolveExternalSlots(child as PluginUiNode, context)));
    return { ...node, children };
  }
  return node;
}

export type PluginRegistryEntry = {
  readonly pluginId: string;
  readonly moduleUrl: string;
};

type KernelOperationPayload = {
  readonly diff?: {
    readonly payload?: unknown;
  };
};

type CommandResultPayload = {
  readonly operations?: readonly KernelOperationPayload[];
};

/** @emoji 🔧 Normalizes plugin command responses into legacy JSON patch op strings. */
export function patchOpsFromCommandResponse(raw: string): string[] {
  let parsed: unknown;
  try {
    parsed = JSON.parse(raw);
  } catch {
    return [];
  }
  if (Array.isArray(parsed)) {
    return parsed.map((entry) => (typeof entry === "string" ? entry : JSON.stringify(entry)));
  }
  if (parsed && typeof parsed === "object") {
    const result = parsed as CommandResultPayload;
    if (Array.isArray(result.operations)) {
      return result.operations
        .map((operation) => operation?.diff?.payload)
        .filter((payload): payload is Record<string, unknown> => payload != null && typeof payload === "object")
        .map((payload) => JSON.stringify(payload));
    }
  }
  return [];
}

export const DEFAULT_PLUGIN_REGISTRY: readonly PluginRegistryEntry[] = [{ pluginId: "draw", moduleUrl: "/plugin-modules/draw/draw_plugin.js" }];

//#region SerializedPluginWasm
/** @emoji 🔒 Serializes wasm plugin entry points — the host keeps instances in one RefCell. */
export function withSerializedPluginWasmHandle(handle: PluginWasmHandle): PluginWasmHandle {
  let tail: Promise<void> = Promise.resolve();
  const runSerialized = <T>(fn: () => Promise<T>): Promise<T> => {
    const job = tail.then(async () => {
      for (let attempt = 0; attempt < 8; attempt += 1) {
        try {
          return await fn();
        } catch (error) {
          const message = error instanceof Error ? error.message : String(error);
          if (!message.includes("plugin instance busy") && !message.includes("plugin busy")) throw error;
          await new Promise((resolve) => setTimeout(resolve, attempt + 1));
        }
      }
      return fn();
    });
    tail = job.then(
      () => undefined,
      () => undefined,
    );
    return job;
  };
  return {
    pluginId: handle.pluginId,
    manifest: handle.manifest,
    createApp: (appId) => runSerialized(() => handle.createApp(appId)),
    destroyApp: (instanceId) => runSerialized(() => handle.destroyApp(instanceId)),
    handleCommand: (instanceId, commandJson, viewState) => runSerialized(() => handle.handleCommand(instanceId, commandJson, viewState)),
    render: (instanceId, bodyKey, viewState) => runSerialized(() => handle.render(instanceId, bodyKey, viewState)),
    renderWithDocument: handle.renderWithDocument ? (instanceId, bodyKey, viewState, documentJson) => runSerialized(() => handle.renderWithDocument!(instanceId, bodyKey, viewState, documentJson)) : undefined,
    tools: (instanceId, viewState) => runSerialized(() => handle.tools(instanceId, viewState)),
    windowEngagements: (instanceId, viewState) => runSerialized(() => handle.windowEngagements(instanceId, viewState)),
    windowMeasures: (instanceId, viewState) => runSerialized(() => handle.windowMeasures(instanceId, viewState)),
    dispose: handle.dispose,
  };
}
//#endregion SerializedPluginWasm

const pluginModuleHandleCache = new Map<string, Promise<PluginWasmHandle>>();

export async function loadPluginModule(pluginId: string, moduleUrl: string): Promise<PluginWasmHandle> {
  const cached = pluginModuleHandleCache.get(moduleUrl);
  if (cached) return cached;
  const pending = loadPluginModuleUncached(pluginId, moduleUrl);
  pluginModuleHandleCache.set(moduleUrl, pending);
  try {
    return await pending;
  } catch (error) {
    pluginModuleHandleCache.delete(moduleUrl);
    throw error;
  }
}

async function loadPluginModuleUncached(pluginId: string, moduleUrl: string): Promise<PluginWasmHandle> {
  const module = (await import(/* @vite-ignore */ moduleUrl)) as {
    default?: () => Promise<void> | void;
    createPluginApi?: () => Promise<{
      manifest: () => Promise<string>;
      createApp: (appId: string) => Promise<number>;
      destroyApp?: (instanceId: number) => Promise<void>;
      handleCommand: (instanceId: number, commandJson: string, contextJson: string) => Promise<string>;
      render: (instanceId: number, bodyKey: string, viewStateJson: string) => Promise<string>;
      renderWithDocument?: (instanceId: number, bodyKey: string, viewStateJson: string, documentJson: string) => Promise<string>;
      tools?: (instanceId: number, viewStateJson: string) => Promise<string>;
      windowEngagements?: (instanceId: number, viewStateJson: string) => Promise<string>;
      windowMeasures?: (instanceId: number, viewStateJson: string) => Promise<string>;
    }>;
    semio_plugin_manifest?: () => string;
    semio_plugin_create_app?: (appId: string) => number;
    semio_plugin_destroy_app?: (instanceId: number) => void;
    semio_plugin_handle_command?: (instanceId: number, commandJson: string, viewStateJson: string) => string;
    semio_plugin_render?: (instanceId: number, bodyKey: string, viewStateJson: string) => string;
    semio_plugin_tools?: (instanceId: number, viewStateJson: string) => string;
    semio_plugin_window_engagements?: (instanceId: number, viewStateJson: string) => string;
    semio_plugin_window_measures?: (instanceId: number, viewStateJson: string) => string;
  };
  if (module.default) await module.default();
  if (module.createPluginApi) {
    const api = await module.createPluginApi();
    const manifest = JSON.parse(await api.manifest()) as PluginManifest;
    return withSerializedPluginWasmHandle({
      pluginId,
      manifest,
      createApp: (appId) => api.createApp(appId),
      destroyApp: async (instanceId) => {
        await api.destroyApp?.(instanceId);
      },
      handleCommand: async (instanceId, commandJson, viewState) => {
        const raw = await api.handleCommand(instanceId, commandJson, JSON.stringify(viewState));
        return patchOpsFromCommandResponse(raw);
      },
      render: async (instanceId, bodyKey, viewState) => JSON.parse(await api.render(instanceId, bodyKey, JSON.stringify(viewState))) as PluginUiNode,
      renderWithDocument: api.renderWithDocument ? async (instanceId, bodyKey, viewState, documentJson) => JSON.parse(await api.renderWithDocument!(instanceId, bodyKey, JSON.stringify(viewState), documentJson)) as PluginUiNode : undefined,
      tools: async (instanceId, viewState) => {
        if (!api.tools) return [];
        return JSON.parse(await api.tools(instanceId, JSON.stringify(viewState))) as Record<string, unknown>[];
      },
      windowEngagements: async (instanceId, viewState) => {
        if (!api.windowEngagements) return {};
        return JSON.parse(await api.windowEngagements(instanceId, JSON.stringify(viewState))) as Record<string, Record<string, unknown>>;
      },
      windowMeasures: async (instanceId, viewState) => {
        if (!api.windowMeasures) return {};
        return JSON.parse(await api.windowMeasures(instanceId, JSON.stringify(viewState))) as Record<string, readonly Record<string, unknown>[]>;
      },
      dispose() {},
    });
  }
  if (!module.semio_plugin_manifest) {
    throw new Error(`[DEBUG] plugin ${pluginId} missing semio_plugin_manifest export`);
  }
  const manifest = JSON.parse(module.semio_plugin_manifest()) as PluginManifest;
  return withSerializedPluginWasmHandle({
    pluginId,
    manifest,
    async createApp(appId: string) {
      const create = module.semio_plugin_create_app;
      if (!create) throw new Error(`plugin ${pluginId} missing create_app`);
      return create(appId);
    },
    async destroyApp(instanceId: number) {
      module.semio_plugin_destroy_app?.(instanceId);
    },
    async handleCommand(instanceId: number, commandJson: string, viewState: PluginViewState) {
      const handle = module.semio_plugin_handle_command;
      if (!handle) return [];
      const raw = handle(instanceId, commandJson, JSON.stringify(viewState));
      return patchOpsFromCommandResponse(raw);
    },
    async render(instanceId: number, bodyKey: string, viewState: PluginViewState) {
      const render = module.semio_plugin_render;
      if (!render) throw new Error(`plugin ${pluginId} missing render`);
      return JSON.parse(render(instanceId, bodyKey, JSON.stringify(viewState))) as PluginUiNode;
    },
    async tools(instanceId: number, viewState: PluginViewState) {
      const tools = module.semio_plugin_tools;
      if (!tools) return [];
      return JSON.parse(tools(instanceId, JSON.stringify(viewState))) as Record<string, unknown>[];
    },
    async windowEngagements(instanceId: number, viewState: PluginViewState) {
      const engagements = module.semio_plugin_window_engagements;
      if (!engagements) return {};
      return JSON.parse(engagements(instanceId, JSON.stringify(viewState))) as Record<string, Record<string, unknown>>;
    },
    async windowMeasures(instanceId: number, viewState: PluginViewState) {
      const measures = module.semio_plugin_window_measures;
      if (!measures) return {};
      return JSON.parse(measures(instanceId, JSON.stringify(viewState))) as Record<string, readonly Record<string, unknown>[]>;
    },
    dispose() {},
  });
}

export async function loadPluginWasm(pluginId: string, moduleUrl: string): Promise<PluginWasmHandle> {
  return loadPluginModule(pluginId, moduleUrl);
}

export function pluginHandleForBridge(handle: PluginWasmHandle) {
  return {
    manifest: () => JSON.stringify(handle.manifest),
    createApp: (appId: string) => handle.createApp(appId),
    destroyApp: (instanceId: number) => handle.destroyApp(instanceId),
    handleCommand: (instanceId: number, commandJson: string, viewStateJson: string) => handle.handleCommand(instanceId, commandJson, JSON.parse(viewStateJson) as PluginViewState).then((ops) => JSON.stringify(ops)),
    render: (instanceId: number, bodyKey: string, viewStateJson: string) => handle.render(instanceId, bodyKey, JSON.parse(viewStateJson) as PluginViewState).then((node) => JSON.stringify(node)),
    renderWithDocument: handle.renderWithDocument
      ? (instanceId: number, bodyKey: string, viewStateJson: string, documentJson: string) => handle.renderWithDocument!(instanceId, bodyKey, JSON.parse(viewStateJson) as PluginViewState, documentJson).then((node) => JSON.stringify(node))
      : undefined,
    tools: (instanceId: number, viewStateJson: string) => handle.tools(instanceId, JSON.parse(viewStateJson) as PluginViewState).then((nodes) => JSON.stringify(nodes)),
    windowEngagements: (instanceId: number, viewStateJson: string) => handle.windowEngagements(instanceId, JSON.parse(viewStateJson) as PluginViewState).then((engagements) => JSON.stringify(engagements)),
    windowMeasures: (instanceId: number, viewStateJson: string) => handle.windowMeasures(instanceId, JSON.parse(viewStateJson) as PluginViewState).then((measures) => JSON.stringify(measures)),
  };
}
//#endregion PluginRuntime
