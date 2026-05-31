// #region 🧲Header
/** @emoji 🛝 `@framework/playground/core` — React-neutral playground runtime, one-app shell (selection + filter toolbars, workbench + details), declarative {@link UiNode} bodies, command routing (no DOM). */
// #endregion 🧲Header

export * from "@framework/core";

import {
  BaseAppRuntime,
  BaseModeRuntime,
  BaseModeRuntime as ModeRuntime,
  BaseWindowKindRuntime,
  mergeAppTools,
  CommandBus,
  Controller,
  createDefaultLayout,
  createTabStackLayout,
  mergeById,
  Platform,
  resolveMode,
  type AppTools,
  type CommandDescriptor,
  type FooterItem,
  type SideTabSpec,
  type ToolItem,
  type WindowLayout,
  type WindowMeasure,
} from "@framework/core";

//#region 🔖UiNode
export interface UiStackNode {
  readonly type: "stack";
  readonly direction: "horizontal" | "vertical";
  readonly gap?: "none" | "tight" | "standard" | "relaxed";
  readonly padding?: "none" | "standard";
  readonly children: readonly UiNode[];
}

export type { UiButtonNode, UiSeparatorNode, UiTextNode } from "@framework/core";

export interface UiTextNode {
  readonly type: "text";
  readonly value: string;
  readonly emphasize?: boolean;
  readonly dataAttributes?: Readonly<Record<string, string>>;
}

export interface UiButtonNode {
  readonly type: "button";
  readonly id?: string;
  readonly label: string;
  readonly command: CommandDescriptor;
  readonly style?: StyleSpec;
}

export interface UiSeparatorNode {
  readonly type: "separator";
}

export type {
  UiPuzzle2dHostSurfaceNode,
  UiPuzzle3dHostSurfaceNode,
  UiPuzzle5dHostSurfaceNode,
  UiCadHostSurfaceNode,
} from "@framework/platform/core";

/** @emoji 📋 Playground alias for {@link UiPuzzle2dHostSurfaceNode}. */
export type UiPuzzle2dHostSurfaceNode = import("@framework/platform/core").UiPuzzle2dHostSurfaceNode;

/** @emoji 📊 Host-bound tabular surface; `paneId` disambiguates multiple table slots in one app. */
export interface UiTableHostSurfaceNode {
  readonly type: "table";
  readonly surfaceId: string;
  readonly controllerId: string;
  readonly paneId?: string;
}

/** @emoji 📂 Collapsible panel section for side-panel declarative trees. */
export interface UiSectionNode {
  readonly type: "section";
  readonly id: string;
  readonly label?: string;
  readonly defaultOpen?: boolean;
  readonly children: readonly UiNode[];
}

/** @emoji 🏷️ Labeled field wrapping one declarative control. */
export interface UiFieldNode {
  readonly type: "field";
  readonly id: string;
  readonly label: string;
  readonly child: UiNode;
}

/** @emoji ✏️ Text or number input bound to a command. */
export interface UiInputNode {
  readonly type: "input";
  readonly id: string;
  readonly inputKind: "text" | "number";
  readonly value: string;
  readonly placeholder?: string;
  readonly commit?: "change" | "blur";
  readonly onChange: CommandDescriptor;
}

/** @emoji 📋 Select control bound to a command (`value` in args). */
export interface UiSelectNode {
  readonly type: "select";
  readonly id: string;
  readonly value: string;
  readonly items: readonly { readonly value: string; readonly label: string }[];
  readonly placeholder?: string;
  readonly onChange: CommandDescriptor;
}

/** @emoji 🔘 Toggle control bound to a command (`pressed` in args). */
export interface UiToggleNode {
  readonly type: "toggle";
  readonly id: string;
  readonly pressed: boolean;
  readonly text?: string;
  readonly onChange: CommandDescriptor;
}

/** @emoji 📐 Three-axis numeric row; `value` null renders mixed placeholder. */
export interface UiVec3Node {
  readonly type: "vec3";
  readonly id: string;
  readonly value: readonly [number, number, number] | null;
  readonly onChange: CommandDescriptor;
}

/** @emoji 📋 Read-only label/value rows. */
export interface UiKeyValueNode {
  readonly type: "keyValue";
  readonly entries: readonly { readonly label: string; readonly value: string }[];
}

/** @emoji 🌿 One tree row; optional nested items and selection command. */
export interface UiTreeItemNode {
  readonly id: string;
  readonly label: string;
  readonly description?: string;
  readonly selected?: boolean;
  readonly defaultOpen?: boolean;
  readonly command?: CommandDescriptor;
  /** @emoji 🖱️ When true, row is draggable when the panel tree supplies a drag controller. */
  readonly draggable?: boolean;
  /** @emoji 📤 Extra `dataTransfer` MIME entries for in-app drags (e.g. puzzle fixture palette). */
  readonly dragData?: Readonly<Record<string, string>>;
  readonly items?: readonly UiTreeItemNode[];
}

/** @emoji 🌲 Tree section for {@link UiTreeNode}. */
export interface UiTreeSectionNode {
  readonly id: string;
  readonly label?: string;
  readonly defaultOpen?: boolean;
  readonly items: readonly UiTreeItemNode[];
}

/** @emoji 🌲 Workbench/details tree panel body. */
export interface UiTreeNode {
  readonly type: "tree";
  readonly sections: readonly UiTreeSectionNode[];
}

/** @emoji 🖱️ Collects declarative tree item `dragData` by row id (depth-first across sections). */
export function collectUiTreeItemDragData(sections: readonly UiTreeSectionNode[]): Map<string, Record<string, string>> {
  const out = new Map<string, Record<string, string>>();
  const visitItems = (items: readonly UiTreeItemNode[]): void => {
    for (const item of items) {
      if (item.dragData) {
        out.set(item.id, item.dragData);
      }
      if (item.items?.length) {
        visitItems(item.items);
      }
    }
  };
  for (const section of sections) {
    visitItems(section.items);
  }
  return out;
}

export type UiNode =
  | UiStackNode
  | UiTextNode
  | UiButtonNode
  | UiSeparatorNode
  | UiPuzzle3dHostSurfaceNode
  | UiPuzzle2dHostSurfaceNode
  | import("@framework/platform/core").UiPuzzle5dHostSurfaceNode
  | import("@framework/platform/core").UiCadHostSurfaceNode
  | UiTableHostSurfaceNode
  | UiSectionNode
  | UiFieldNode
  | UiInputNode
  | UiSelectNode
  | UiToggleNode
  | UiVec3Node
  | UiKeyValueNode
  | UiTreeNode;

/** @emoji 🌲 Single-root tree body for a side panel (no duplicate section title). */
export function playgroundTreePanelRootItems(sectionId: string, items: readonly UiTreeItemNode[]): UiTreeNode {
  if (!items.length) {
    throw new Error("playgroundTreePanelRootItems requires at least one root item.");
  }
  return {
    type: "tree",
    sections: [{ id: sectionId, defaultOpen: true, items }],
  };
}

import {
  buildPuzzle2dWindowBody,
  buildPuzzle3dWindowBody,
  isCanvasOnlyWindowBody,
} from "@framework/platform/core";

export {
  buildPuzzle2dWindowBody,
  buildPuzzle3dWindowBody,
  buildPuzzle5dWindowBody,
  buildCadWindowBody,
  isCanvasOnlyWindowBody,
} from "@framework/platform/core";

function assertCanvasOnlyWindowBody(bodyKey: string, node: UiNode): void {
  if (isCanvasOnlyWindowBody(node)) return;
  throw new Error(`Declarative window body "${bodyKey}" must be a single canvas component surface (optional none padding stack wrapper). Found "${node.type}".`);
}
//#endregion 🔖UiNode

//#region 🔖WindowEngagement
/** @emoji 💬 One floating engagement option button; dispatches {@link command} when pressed. */
export interface WindowEngagementOption {
  readonly id: string;
  readonly label?: string;
  readonly iconId?: string;
  readonly pressed?: boolean;
  readonly disabled?: boolean;
  readonly command?: CommandDescriptor;
}

/** @emoji 💬 Engagement input line; {@link onChange}/{@link onSubmit} dispatch with `{ value }` merged into args. */
export interface WindowEngagementInput {
  readonly id?: string;
  readonly value?: string;
  readonly placeholder?: string;
  readonly disabled?: boolean;
  readonly onChange?: CommandDescriptor;
  readonly onSubmit?: CommandDescriptor;
}

/** @emoji 💬 One engagement status cell rendered as muted text. */
export interface WindowEngagementStatus {
  readonly id: string;
  readonly text: string;
}

/** @emoji 🔎 One engagement autocomplete row mirrored from {@link EngagementSpec.possibleEngagements}. */
export interface WindowEngagementPossible {
  readonly id: string;
  readonly label: string;
  readonly detail?: string;
  readonly command?: CommandDescriptor;
}

/** @emoji 💬 React-neutral floating window engagement (options/input/status) resolved to a UI panel by the renderer. */
export interface WindowEngagement {
  readonly options?: readonly WindowEngagementOption[];
  readonly input?: WindowEngagementInput;
  readonly status?: readonly WindowEngagementStatus[];
  readonly possibleEngagements?: readonly WindowEngagementPossible[];
}
//#endregion 🔖WindowEngagement

//#region 🔖WindowKindRuntime
export class WindowKindRuntime extends BaseWindowKindRuntime {
  /** @emoji 💬 Optional floating engagement (options/input/status); mutable so controllers can rebuild it per snapshot. */
  engagement?: WindowEngagement;

  constructor(
    id: string,
    label: string,
    bodyKey: string,
    iconId?: string,
    measures: readonly WindowMeasure[] = [],
    engagement?: WindowEngagement,
  ) {
    super(id, label, bodyKey, iconId, measures);
    this.engagement = engagement;
  }
}
//#endregion 🔖WindowKindRuntime

export { ModeRuntime };

//#region 🔖AppRuntime
/** @emoji 🧩 Playground app runtime (reuses shared {@link BaseAppRuntime} shell). */
export class AppRuntime extends BaseAppRuntime {
  declare windowKinds: WindowKindRuntime[];

  constructor(
    id: string,
    label: string,
    iconId: string | undefined,
    controller: import("@framework/core").Controller,
    layout: WindowLayout,
    windowKinds: readonly WindowKindRuntime[],
  ) {
    super(id, label, iconId, controller, layout, windowKinds);
  }

  override resolve(requestedModeId?: string | null): ResolvedAppState {
    const modeId = requestedModeId ?? this.getActiveModeId();
    return resolveAppState(this, modeId);
  }
}
//#endregion 🔖AppRuntime

//#region 🔖ResolvedState
export type { ResolvedAppState } from "@framework/core";

/** @emoji 🧮 Resolves playground app + active mode overlays. */
export function resolveAppState(app: AppRuntime, requestedModeId?: string | null): ResolvedAppState {
  const mode = resolveMode(app, requestedModeId);
  const mergedWindowKinds = (mergeById(app.windowKinds, mode?.windowKinds) ?? app.windowKinds) as WindowKindRuntime[];
  const mergedPanelTabs = mergeById(app.panelTabs, mode?.panelTabs) ?? app.panelTabs;
  return {
    id: app.id,
    activeModeId: mode?.id ?? null,
    label: mode?.label ?? app.label,
    iconId: mode?.iconId ?? app.iconId,
    tools: mergeAppTools(app.tools, mode?.tools),
    windowKinds: mergedWindowKinds,
    defaultLayout: mode?.defaultLayout ?? app.defaultLayout,
    panelTabs: mergedPanelTabs,
    footerItems: mergeById(app.footerItems, mode?.footerItems) ?? app.footerItems,
  };
}
//#endregion 🔖ResolvedState

//#region 🔖WindowBodyViewContext
export interface WindowBodyViewContext {
  readonly runtime: Platform;
  readonly windowKindId: string;
  readonly bodyKey: string;
  readonly activeModeId: string | null;
  readonly generation: number;
}

const windowBodyByKey = new Map<string, (ctx: WindowBodyViewContext) => UiNode>();

export function registerWindowBody(bodyKey: string, build: (ctx: WindowBodyViewContext) => UiNode): void {
  windowBodyByKey.set(bodyKey, (ctx) => {
    const node = build(ctx);
    assertCanvasOnlyWindowBody(bodyKey, node);
    return node;
  });
}

export function getWindowBodyFactory(bodyKey: string): ((ctx: WindowBodyViewContext) => UiNode) | undefined {
  return windowBodyByKey.get(bodyKey);
}

export function unregisterWindowBody(bodyKey: string): void {
  windowBodyByKey.delete(bodyKey);
}
//#endregion 🔖WindowBodyViewContext

//#region 🔖SidePanelBodyViewContext
export type SidePanelBodyViewContext = WindowBodyViewContext;

/** @emoji 🌲 `nested` wraps the body in a shell tree section; `treeRoot` mounts a declarative tree as the tab root. */
export type SidePanelBodyMount = "nested" | "treeRoot";

const sidePanelBodyByKey = new Map<string, (ctx: SidePanelBodyViewContext) => UiNode>();
const sidePanelBodyMountByKey = new Map<string, SidePanelBodyMount>();

export function registerSidePanelBody(
  bodyKey: string,
  build: (ctx: SidePanelBodyViewContext) => UiNode,
  options?: { readonly mount?: SidePanelBodyMount },
): void {
  sidePanelBodyByKey.set(bodyKey, build);
  if (options?.mount) {
    sidePanelBodyMountByKey.set(bodyKey, options.mount);
  } else {
    sidePanelBodyMountByKey.delete(bodyKey);
  }
}

export function getSidePanelBodyFactory(bodyKey: string): ((ctx: SidePanelBodyViewContext) => UiNode) | undefined {
  return sidePanelBodyByKey.get(bodyKey);
}

/** @emoji 🌲 How a declarative side-panel body mounts in the workbench shell tree. */
export function getSidePanelBodyMount(bodyKey: string): SidePanelBodyMount {
  return sidePanelBodyMountByKey.get(bodyKey) ?? "nested";
}

export function unregisterSidePanelBody(bodyKey: string): void {
  sidePanelBodyByKey.delete(bodyKey);
  sidePanelBodyMountByKey.delete(bodyKey);
}
//#endregion 🔖SidePanelBodyViewContext

//#region 🔖Playground
export interface PlaygroundPanelVisibility {
  readonly leftSidePanel: boolean;
  readonly rightSidePanel: boolean;
}

/** @emoji ⌨️ Document key routed to {@link CommandBus.dispatch} when focus is not in a field. */
export interface PlaygroundKeybinding {
  readonly key: string;
  readonly controllerId: string;
  readonly command: string;
  readonly args?: JsonValue;
}

/** @emoji 🛝 React-free playground definition: runtime, declarative bodies, optional surface host registration. */
export abstract class Playground {
  abstract readonly id: string;
  private runtimeMemo: Platform | null = null;

  /** @emoji 🚀 Lazily built {@link Platform} from {@link createRuntime}. */
  get runtime(): Platform {
    this.runtimeMemo ??= this.createRuntime();
    return this.runtimeMemo;
  }

  abstract createRuntime(): Platform;
  abstract registerBodies(): void;

  readonly initialPanelVisibility?: PlaygroundPanelVisibility;
  readonly keybindings?: readonly PlaygroundKeybinding[];

  /** @emoji 🧊 Override to register canvas surface hosts (library React adapters). */
  registerSurfaceHosts(): void {}
}

export const PLAYGROUND_LS_THEME = "framework.playground.surface.theme";
export const PLAYGROUND_LS_DEVICE = "framework.playground.surface.device";
export const PLAYGROUND_LS_EXPERTISE = "framework.playground.surface.expertise";

export type PlaygroundSurfaceTheme = "system" | "light" | "dark";
export type PlaygroundSurfaceDevice = "desktop" | "tablet" | "mobile";

/** @emoji 🌓 Parses persisted playground surface theme. */
export function parsePlaygroundStoredTheme(raw: string | null): PlaygroundSurfaceTheme {
  if (raw === "light" || raw === "dark" || raw === "system") return raw;
  return "system";
}

/** @emoji 📱 Parses persisted playground surface device. */
export function parsePlaygroundStoredDevice(raw: string | null): PlaygroundSurfaceDevice {
  if (raw === "desktop" || raw === "tablet" || raw === "mobile") return raw;
  return "desktop";
}

/** @emoji 🎚 Parses persisted playground surface expertise. */
export function parsePlaygroundStoredExpertise(raw: string | null): Expertise {
  if (raw === Expertise.BEGINNER || raw === Expertise.NORMAL || raw === Expertise.EXPERT) return raw;
  return Expertise.NORMAL;
}
//#endregion 🔖Playground

//#region 🔖Ids
/** @emoji 🏷 Stable ids for a single-app playground (main window + workbench + details tabs). */
export interface PlaygroundIds {
  readonly appId: string;
  readonly controllerId: string;
  readonly windowId: string;
  readonly windowLabel: string;
  readonly mainBodyKey: string;
  readonly workbenchTabBodyKey: string;
  readonly detailsTabBodyKey: string;
  readonly workbenchIconId: string;
  readonly detailsIconId: string;
  readonly mainPuzzle3dViewportSurfaceId: string;
  readonly workbenchPanelSurfaceId: string;
  readonly detailsPanelSurfaceId: string;
}

export interface PlaygroundKindSpec<K extends string> {
  readonly kinds: readonly K[];
  readonly label: (kind: K) => string;
}

export type PlaygroundFocusFilter<K extends string> = "all" | K;
//#endregion 🔖Ids

//#region 🔖Toolbar
function createAllKindsEnabled<K extends string>(kinds: readonly K[]): Record<K, boolean> {
  return Object.fromEntries(kinds.map((kind) => [kind, true])) as Record<K, boolean>;
}

/** @emoji 🎚 Kind toggle row for playground `selection` or `filter` toolbar zones. */
export function buildPlaygroundKindToggleTools<K extends string>(prefix: "selection" | "filter", kinds: readonly K[], labels: (kind: K) => string, values: Readonly<Record<K, boolean>>, controllerId: string, command: string): ToolItem[] {
  return kinds.map((kind, order) => ({
    id: `playground.${prefix}.${kind}`,
    kind: "toggle" as const,
    text: labels(kind),
    order,
    pressed: values[kind],
    controllerId,
    command,
    args: { kind },
  }));
}

/** @emoji 🧹 Clear-selection button for the playground `selection` toolbar zone. */
export function buildPlaygroundClearSelectionTool(controllerId: string, order: number): ToolItem {
  return {
    id: "playground.selection.clear",
    kind: "button",
    label: "Clear",
    order,
    controllerId,
    command: "setSelectedId",
    args: { id: null },
  };
}

/** @emoji 🎯 Standard playground browse selection tools (kind toggles + clear). */
export function buildPlaygroundBrowseSelectionTools<K extends string>(kinds: readonly K[], labels: (kind: K) => string, selectableKinds: Readonly<Record<K, boolean>>, controllerId: string): ToolItem[] {
  const toggles = buildPlaygroundKindToggleTools("selection", kinds, labels, selectableKinds, controllerId, "toggleSelectableKind");
  return [...toggles, { id: "playground.selection.separator", kind: "separator", order: kinds.length }, buildPlaygroundClearSelectionTool(controllerId, kinds.length + 1)];
}

/** @emoji 👁️ Standard playground browse filter tools (visibility kind toggles). */
export function buildPlaygroundBrowseFilterTools<K extends string>(kinds: readonly K[], labels: (kind: K) => string, visibleKinds: Readonly<Record<K, boolean>>, controllerId: string): ToolItem[] {
  return buildPlaygroundKindToggleTools("filter", kinds, labels, visibleKinds, controllerId, "toggleVisibleKind");
}
//#endregion 🔖Toolbar

//#region 🔖Controller
/** @emoji 🎛 Base playground controller: selection/filter kind toggles, query, selected id, focused kind. */
export abstract class PlaygroundController<K extends string> extends Controller {
  readonly browseMode = new ModeRuntime("browse", "Browse", undefined);
  protected readonly kinds: readonly K[];
  protected readonly kindLabel: (kind: K) => string;
  readonly selectableKinds: Record<K, boolean>;
  readonly visibleKinds: Record<K, boolean>;
  focusedKind: PlaygroundFocusFilter<K> = "all";
  query = "";
  selectedId: string | null = null;
  private snapshotListeners = new Set<() => void>();

  protected constructor(controllerId: string, spec: PlaygroundKindSpec<K>, commandBus: CommandBus, hostNotify: () => void) {
    super(controllerId, commandBus, hostNotify);
    this.kinds = spec.kinds;
    this.kindLabel = spec.label;
    this.selectableKinds = createAllKindsEnabled(spec.kinds);
    this.visibleKinds = createAllKindsEnabled(spec.kinds);
    this.rebuildBrowseModeTools();
  }

  /** @emoji 🔔 Subscribes to browse-state updates without shell generation bumps. */
  subscribeSnapshot(listener: () => void): () => void {
    this.snapshotListeners.add(listener);
    return () => this.snapshotListeners.delete(listener);
  }

  protected notifySnapshot(): void {
    for (const listener of this.snapshotListeners) {
      listener();
    }
  }

  protected rebuildBrowseModeTools(): void {
    this.browseMode.tools = {
      selection: buildPlaygroundBrowseSelectionTools(this.kinds, this.kindLabel, this.selectableKinds, this.id),
      filter: buildPlaygroundBrowseFilterTools(this.kinds, this.kindLabel, this.visibleKinds, this.id),
    };
  }

  protected syncShell(): void {
    this.rebuildBrowseModeTools();
    this.emit();
  }

  /** @emoji ✅ Domain hook: whether `id` may be selected given current kind toggles. */
  protected abstract canSelectId(id: string): boolean;

  /** @emoji 🔄 Domain hook: clear selection when it becomes invalid (e.g. hidden kind). */
  protected ensureSelectionValidity(): void {
    if (this.selectedId !== null && !this.canSelectId(this.selectedId)) {
      this.selectedId = null;
    }
  }

  protected handlePlaygroundCommand(command: string, args?: unknown): "shell" | "snapshot" | false {
    switch (command) {
      case "toggleSelectableKind": {
        const { kind } = args as { kind: K };
        this.selectableKinds[kind] = !this.selectableKinds[kind];
        return "shell";
      }
      case "toggleVisibleKind": {
        const { kind } = args as { kind: K };
        this.visibleKinds[kind] = !this.visibleKinds[kind];
        return "shell";
      }
      case "setSelectedId": {
        const { id } = args as { id: string | null };
        if (!id || this.canSelectId(id)) this.selectedId = id;
        return "snapshot";
      }
      case "setFocusedKind": {
        this.focusedKind = (args as { kind: PlaygroundFocusFilter<K> }).kind;
        return "snapshot";
      }
      case "setQuery": {
        this.query = (args as { query: string }).query;
        return "snapshot";
      }
      default:
        return false;
    }
  }

  protected finishPlaygroundCommand(notify: "shell" | "snapshot"): void {
    this.ensureSelectionValidity();
    if (notify === "shell") {
      this.syncShell();
      this.notifySnapshot();
      return;
    }
    this.notifySnapshot();
  }
}
//#endregion 🔖Controller

//#region 🔖Runtime
export interface BuildPlaygroundWorkbenchAppOptions {
  readonly layout?: WindowLayout;
  readonly initialQuery?: string;
}

/** @emoji 🧩 Registers the standard playground app (browse mode, left workbench + right details tabs). */
export function buildPlaygroundWorkbenchApp(ids: PlaygroundIds, controller: PlaygroundController<string>, options?: BuildPlaygroundWorkbenchAppOptions): AppRuntime {
  const layout = options?.layout ?? createDefaultLayout([ids.windowId], "row", [100], [ids.windowLabel]);
  const app = new AppRuntime(ids.appId, ids.windowLabel, undefined, controller, layout, [new WindowKindRuntime(ids.windowId, ids.windowLabel, ids.mainBodyKey)]);
  app.defaultModeId = controller.browseMode.id;
  app.addMode(controller.browseMode);
  app.panelTabs = [
    { id: `${ids.appId}.workbench`, iconId: ids.workbenchIconId, panel: "workbench", order: 0, bodyKey: ids.workbenchTabBodyKey },
    { id: `${ids.appId}.details`, iconId: ids.detailsIconId, panel: "details", order: 0, bodyKey: ids.detailsTabBodyKey },
  ];
  controller.commandBus.dispatch(controller.id, "setQuery", { query: options?.initialQuery ?? "" });
  return app;
}

export function playgroundControllerFromContext(ctx: WindowBodyViewContext | SidePanelBodyViewContext): PlaygroundController<string> | undefined {
  return ctx.runtime.getActiveApp()?.controller as PlaygroundController<string> | undefined;
}

/** @emoji 🪟 Declarative main window: lone puzzle3d viewport surface. */
export function buildPlaygroundMainWindowBody(ids: PlaygroundIds, ctx: WindowBodyViewContext): UiNode {
  if (!playgroundControllerFromContext(ctx)) {
    return { type: "text", value: "Missing playground controller" };
  }
  return buildPuzzle3dWindowBody(ids.mainPuzzle3dViewportSurfaceId, ids.controllerId);
}

/** @emoji 📋 Declarative workbench side tab: host-bound table surface. */
export function buildPlaygroundWorkbenchPanelBody(ids: PlaygroundIds, ctx: SidePanelBodyViewContext): UiNode {
  if (!playgroundControllerFromContext(ctx)) {
    return { type: "text", value: "Missing playground controller" };
  }
  return { type: "table", surfaceId: ids.workbenchPanelSurfaceId, controllerId: ids.controllerId };
}

/** @emoji 🔎 Declarative details side tab: host-bound table surface. */
export function buildPlaygroundDetailsPanelBody(ids: PlaygroundIds, ctx: SidePanelBodyViewContext): UiNode {
  if (!playgroundControllerFromContext(ctx)) {
    return { type: "text", value: "Missing playground controller" };
  }
  return { type: "table", surfaceId: ids.detailsPanelSurfaceId, controllerId: ids.controllerId };
}

export interface RegisterPlaygroundDeclarativeBodiesOptions {
  readonly buildMainWindow?: (ctx: WindowBodyViewContext) => UiNode;
  readonly buildWorkbenchPanel?: (ctx: SidePanelBodyViewContext) => UiNode;
  readonly buildDetailsPanel?: (ctx: SidePanelBodyViewContext) => UiNode;
}

export interface PlaygroundSidePanelBodyRegistration {
  readonly bodyKey: string;
  readonly build: (ctx: SidePanelBodyViewContext) => UiNode;
}

/** @emoji 📝 Registers multiple side-panel declarative trees. */
export function registerPlaygroundSidePanelBodies(tabs: readonly PlaygroundSidePanelBodyRegistration[]): void {
  for (const tab of tabs) {
    registerSidePanelBody(tab.bodyKey, tab.build);
  }
}

/** @emoji 📝 Registers playground window + side-panel declarative trees on the framework host. */
export function registerPlaygroundDeclarativeBodies(ids: PlaygroundIds, options?: RegisterPlaygroundDeclarativeBodiesOptions): void {
  registerWindowBody(ids.mainBodyKey, options?.buildMainWindow ?? ((ctx) => buildPlaygroundMainWindowBody(ids, ctx)));
  registerSidePanelBody(ids.workbenchTabBodyKey, options?.buildWorkbenchPanel ?? ((ctx) => buildPlaygroundWorkbenchPanelBody(ids, ctx)));
  registerSidePanelBody(ids.detailsTabBodyKey, options?.buildDetailsPanel ?? ((ctx) => buildPlaygroundDetailsPanelBody(ids, ctx)));
}

/** @emoji 🚀 Creates a {@link Platform} with one playground app. */
export function createPlaygroundWorkbench(ids: PlaygroundIds, controller: PlaygroundController<string>, options?: BuildPlaygroundWorkbenchAppOptions): Platform {
  const runtime = new Platform();
  runtime.addApp(buildPlaygroundWorkbenchApp(ids, controller, options));
  return runtime;
}

export interface BootstrapPlaygroundWorkbenchOptions extends BuildPlaygroundWorkbenchAppOptions {
  /** @emoji 📝 When true (default), registers standard playground declarative bodies before returning. */
  readonly registerDeclarativeBodies?: boolean;
  readonly declarativeBodies?: RegisterPlaygroundDeclarativeBodiesOptions;
  /** @emoji 🧱 Reuse an existing product runtime shell (controller must use its {@link CommandBus}). */
  readonly runtime?: Platform;
}

/** @emoji 🚀 One-shot playground setup: optional declarative registration + one playground app. */
export function bootstrapPlaygroundWorkbench(ids: PlaygroundIds, controller: PlaygroundController<string>, options?: BootstrapPlaygroundWorkbenchOptions): Platform {
  if (options?.registerDeclarativeBodies !== false) {
    registerPlaygroundDeclarativeBodies(ids, options?.declarativeBodies);
  }
  const runtime = options?.runtime ?? new Platform();
  runtime.addApp(buildPlaygroundWorkbenchApp(ids, controller, options));
  return runtime;
}
//#endregion 🔖Runtime

//#region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  const TEST_IDS: PlaygroundIds = {
    appId: "test-playground",
    controllerId: "test-playground-ctrl",
    windowId: "main",
    windowLabel: "Main",
    mainBodyKey: "test.playground.main",
    workbenchTabBodyKey: "test.playground.workbench",
    detailsTabBodyKey: "test.playground.details",
    workbenchIconId: "test.playground.icon.workbench",
    detailsIconId: "test.playground.icon.details",
    mainPuzzle3dViewportSurfaceId: "test.playground.puzzle3d/v1",
    workbenchPanelSurfaceId: "test.playground.panel.workbench/v1",
    detailsPanelSurfaceId: "test.playground.panel.details/v1",
  };

  class DemoPlaygroundController extends PlaygroundController<"a" | "b"> {
    private readonly selectable = new Set<string>(["entity-a", "entity-b"]);

    constructor(bus: CommandBus, notify: () => void) {
      super(TEST_IDS.controllerId, { kinds: ["a", "b"], label: (k) => k.toUpperCase() }, bus, notify);
    }

    protected canSelectId(id: string): boolean {
      return this.selectable.has(id) && this.selectableKinds[id === "entity-a" ? "a" : "b"] && this.visibleKinds[id === "entity-a" ? "a" : "b"];
    }

    override run(command: string, args?: unknown): void {
      const notify = this.handlePlaygroundCommand(command, args);
      if (notify) {
        this.finishPlaygroundCommand(notify);
      }
    }
  }

  describe("WindowKindRuntime engagement", () => {
    it("defaults to no engagement and accepts a neutral engagement descriptor", () => {
      const plain = new WindowKindRuntime("w", "W", "body");
      expect(plain.engagement).toBeUndefined();
      const engaged = new WindowKindRuntime("w", "W", "body", undefined, [], {
        options: [{ id: "confirm", label: "Confirm", command: { controllerId: "ctrl", command: "confirm" } }],
        input: { id: "in", value: "", placeholder: "type" },
        status: [{ id: "state", text: "idle" }],
      });
      expect(engaged.engagement?.options?.[0]?.command?.command).toBe("confirm");
      expect(engaged.engagement?.status?.[0]?.text).toBe("idle");
      engaged.engagement = { status: [{ id: "state", text: "active" }] };
      expect(engaged.engagement?.status?.[0]?.text).toBe("active");
    });
  });

  describe("PlaygroundController", () => {
    it("tracks query and clears selection when kind is hidden", () => {
      const bus = new CommandBus();
      const wb = new Platform();
      const ctrl = new DemoPlaygroundController(bus, () => wb.notify());
      wb.addApp(buildPlaygroundWorkbenchApp(TEST_IDS, ctrl));
      bus.dispatch(TEST_IDS.controllerId, "setSelectedId", { id: "entity-a" });
      bus.dispatch(TEST_IDS.controllerId, "setQuery", { query: "find" });
      expect(ctrl.query).toBe("find");
      expect(ctrl.selectedId).toBe("entity-a");
      bus.dispatch(TEST_IDS.controllerId, "toggleVisibleKind", { kind: "a" });
      expect(ctrl.selectedId).toBeNull();
    });
  });

  describe("getSidePanelBodyMount", () => {
    it("defaults to nested and remembers treeRoot registration", () => {
      const key = "test.playground.mount";
      registerSidePanelBody(key, () => ({ type: "text", value: "x" }));
      expect(getSidePanelBodyMount(key)).toBe("nested");
      registerSidePanelBody(key, () => ({ type: "tree", sections: [] }), { mount: "treeRoot" });
      expect(getSidePanelBodyMount(key)).toBe("treeRoot");
      unregisterSidePanelBody(key);
    });
  });

  describe("collectUiTreeItemDragData", () => {
    it("collects dragData from nested declarative tree items by id", () => {
      const sections: readonly UiTreeSectionNode[] = [
        {
          id: "objects",
          label: "Objects",
          items: [
            {
              id: "objects.0.Base",
              label: "Base",
              dragData: { "application/x-test": "payload" },
            },
            {
              id: "group",
              label: "Group",
              items: [{ id: "group.child", label: "Child", dragData: { "application/x-child": "c" } }],
            },
          ],
        },
      ];
      const map = collectUiTreeItemDragData(sections);
      expect(map.get("objects.0.Base")).toEqual({ "application/x-test": "payload" });
      expect(map.get("group.child")).toEqual({ "application/x-child": "c" });
      expect(map.size).toBe(2);
    });
  });

  describe("bootstrapPlaygroundWorkbench", () => {
    it("registers declarative bodies and adds one app", () => {
      const bus = new CommandBus();
      const ctrl = new DemoPlaygroundController(bus, () => undefined);
      const wb = bootstrapPlaygroundWorkbench(TEST_IDS, ctrl);
      expect(wb.apps.length).toBeGreaterThan(0);
      expect(getWindowBodyFactory(TEST_IDS.mainBodyKey)).toBeTypeOf("function");
    });
  });

  describe("canonical window bodies", () => {
    it("buildPuzzle2dWindowBody is canvas-only", () => {
      const node = buildPuzzle2dWindowBody("puzzle.2d/v1", "puzzle2d-ctrl", "pane-a");
      expect(node).toEqual({ type: "puzzle2d", componentKind: "puzzle2d", surfaceId: "puzzle.2d/v1", controllerId: "puzzle2d-ctrl", paneId: "pane-a" });
    });
  });

  describe("registerPlaygroundDeclarativeBodies", () => {
    it("registers puzzle3d main window and table side panels", () => {
      const bus = new CommandBus();
      const wb = new Platform();
      const ctrl = new DemoPlaygroundController(bus, () => wb.notify());
      wb.addApp(buildPlaygroundWorkbenchApp(TEST_IDS, ctrl));
      registerPlaygroundDeclarativeBodies(TEST_IDS);
      const ctx: WindowBodyViewContext = {
        runtime: wb,
        windowKindId: TEST_IDS.windowId,
        bodyKey: TEST_IDS.mainBodyKey,
        activeModeId: "browse",
        generation: wb.generation,
      };
      const main = getWindowBodyFactory(TEST_IDS.mainBodyKey)?.(ctx);
      expect(main?.type).toBe("puzzle3d");
    });
  });
}
//#endregion 🧪Tests
