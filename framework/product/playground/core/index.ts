// #region 🧲Header
/** @emoji 🛝 `@framework/playground/core` — React-neutral playground runtime, one-app shell (selection + filter toolbars, workbench + details), declarative {@link UiNode} bodies, command routing (no DOM). */
// #endregion 🧲Header

export * from "@framework/core";

import type { OrbitCameraViewLayoutArrangement, OrbitCameraViewLayoutDescriptor, OrbitCameraViewLayoutPane } from "@infinite/world/r3f";
import {
  BaseAppRuntime,
  BaseModeRuntime,
  BaseModeRuntime as ModeRuntime,
  BaseWindowKindRuntime,
  mergeAppTools,
  CommandBus,
  Controller,
  createDefaultLayout,
  createNamedLayout,
  createTabStackLayout,
  createWindowLayout,
  mergeById,
  mergeNamedLayouts,
  Platform,
  PRODUCT_SHELL_DEFAULT_PANEL_VISIBILITY,
  resolveMode,
  type AppTools,
  type CommandDescriptor,
  type FooterItem,
  type NamedLayout,
  type SideTabSpec,
  type ToolItem,
  type WindowLayout,
  type WindowLayoutWindowNode,
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
  readonly iconId: string;
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

import {
  collectUiTreeItemDragData,
  getSidePanelBodyFactory,
  registerSidePanelBody,
  sidePanelTreeRootItems,
  unregisterSidePanelBody,
  type UiTreeNode,
  type UiTreeSectionNode,
} from "@framework/platform/core";

export type {
  UiControlNode,
  UiSectionNode,
  UiFieldNode,
  UiInputNode,
  UiKeyValueNode,
  UiSelectNode,
  UiToggleNode,
  UiTreeItemNode,
  UiTreeItemAction,
  UiTreeContextMenuItem,
  UiTreeNode,
  UiTreeSectionNode,
  UiVec3Node,
  SidePanelTreeSelection,
} from "@framework/platform/core";

export { collectUiTreeItemDragData, sidePanelTreeRootItems, uiDeclarativeSectionsToTree } from "@framework/platform/core";

/** @emoji 🎯 Playground alias for {@link SidePanelTreeSelection}. */
export type PlaygroundTreePanelSelection = import("@framework/platform/core").SidePanelTreeSelection;

/** @emoji 🌲 Playground alias for {@link sidePanelTreeRootItems}. */
export { sidePanelTreeRootItems as playgroundTreePanelRootItems };

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
  | import("@framework/platform/core").UiTreeNode;

import {
  buildPuzzle2dWindowBody,
  buildPuzzle3dWindowBody,
  buildMapWindowBody,
  buildFlowWindowBody,
  buildDagWindowBody,
  isCanvasOnlyWindowBody,
} from "@framework/platform/core";

export {
  buildPuzzle2dWindowBody,
  buildPuzzle3dWindowBody,
  buildPuzzle5dWindowBody,
  buildCadWindowBody,
  buildMapWindowBody,
  buildFlowWindowBody,
  buildDagWindowBody,
  buildPanelWindowBody,
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
  /** @emoji 🔁 Repeats the last finalized engagement when Space is pressed with an empty command (non-empty command uses {@link onSubmit}). */
  readonly onRepeatLast?: CommandDescriptor;
  readonly onAbort?: CommandDescriptor;
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

/** @emoji 🔘 One discrete option on an engagement ring control. */
export interface WindowEngagementRingOption {
  readonly id: string;
  readonly label: string;
  readonly disabled?: boolean;
}

/** @emoji 🎚 Engagement slider control (neutral). */
export interface WindowEngagementSliderControl {
  readonly kind: "slider";
  readonly id?: string;
  readonly label?: string;
  readonly value: number;
  readonly min: number;
  readonly max: number;
  readonly step?: number;
  readonly unit?: string;
  readonly disabled?: boolean;
  readonly onChange?: CommandDescriptor;
  readonly onCommit?: CommandDescriptor;
}

/** @emoji 🔢 Engagement stepper control (neutral). */
export interface WindowEngagementStepperControl {
  readonly kind: "stepper";
  readonly id?: string;
  readonly label?: string;
  readonly value: number;
  readonly min?: number;
  readonly max?: number;
  readonly step?: number;
  readonly unit?: string;
  readonly disabled?: boolean;
  readonly onChange?: CommandDescriptor;
  readonly onCommit?: CommandDescriptor;
}

/** @emoji 🧫 Engagement ring control (neutral). */
export interface WindowEngagementRingControl {
  readonly kind: "ring";
  readonly id?: string;
  readonly label?: string;
  readonly value?: string;
  readonly options: readonly WindowEngagementRingOption[];
  readonly disabled?: boolean;
  readonly onSelect?: CommandDescriptor;
}

/** @emoji 🎛 Optional engagement UI control for playground windows. */
export type WindowEngagementControl = WindowEngagementSliderControl | WindowEngagementStepperControl | WindowEngagementRingControl;

/** @emoji 💬 React-neutral floating window engagement (options/input/status) resolved to a UI panel by the renderer. */
export interface WindowEngagement {
  /** @emoji 🎯 Ongoing engagement session (step input + step options stay visible). */
  readonly sessionActive?: boolean;
  readonly options?: readonly WindowEngagementOption[];
  readonly input?: WindowEngagementInput;
  readonly control?: WindowEngagementControl;
  readonly controls?: readonly WindowEngagementControl[];
  readonly status?: readonly WindowEngagementStatus[];
  readonly possibleEngagements?: readonly WindowEngagementPossible[];
}

function windowEngagementControlDigest(control: WindowEngagementControl | undefined): string {
  if (!control) return "";
  if (control.kind === "ring") {
    const options = control.options.map((row) => `${row.id}\u0001${row.label}\u0001${row.disabled ? 1 : 0}`).join("\u0002");
    return `ring\u0001${control.id ?? ""}\u0001${control.label ?? ""}\u0001${control.value ?? ""}\u0001${control.disabled ? 1 : 0}\u0001${options}\u0001${engagementCommandDigest(control.onSelect)}`;
  }
  const bounds =
    control.kind === "slider"
      ? `${control.min}\u0001${control.max}`
      : `${control.min ?? ""}\u0001${control.max ?? ""}`;
  return `${control.kind}\u0001${control.id ?? ""}\u0001${control.label ?? ""}\u0001${control.value}\u0001${bounds}\u0001${control.step ?? ""}\u0001${control.unit ?? ""}\u0001${control.disabled ? 1 : 0}\u0001${engagementCommandDigest(control.onChange)}\u0001${engagementCommandDigest(control.onCommit)}`;
}

function engagementCommandDigest(cmd: CommandDescriptor | undefined): string {
  if (!cmd) return "";
  return `${cmd.controllerId}\u0005${cmd.command}\u0005${cmd.args === undefined ? "" : JSON.stringify(cmd.args)}`;
}

/** @emoji 🔑 Stable digest for {@link WindowEngagement} equality (skips redundant shell updates). */
export function windowEngagementDigest(engagement: WindowEngagement | undefined): string {
  if (!engagement) return "";
  const options = (engagement.options ?? [])
    .map((row) => `${row.id}\u0001${row.label}\u0001${row.pressed ? 1 : 0}\u0001${row.disabled ? 1 : 0}\u0001${engagementCommandDigest(row.command)}`)
    .join("\u0002");
  const input = engagement.input
    ? `${engagement.input.id}\u0001${engagement.input.value}\u0001${engagement.input.placeholder ?? ""}\u0001${engagement.input.disabled ? 1 : 0}\u0001${engagementCommandDigest(engagement.input.onChange)}\u0001${engagementCommandDigest(engagement.input.onSubmit)}\u0001${engagementCommandDigest(engagement.input.onRepeatLast)}\u0001${engagementCommandDigest(engagement.input.onAbort)}`
    : "";
  const status = (engagement.status ?? []).map((row) => `${row.id}\u0001${row.text}`).join("\u0002");
  const possibles = (engagement.possibleEngagements ?? [])
    .map((row) => `${row.id}\u0001${row.label}\u0001${row.detail ?? ""}\u0001${engagementCommandDigest(row.command)}`)
    .join("\u0002");
  const session = engagement.sessionActive ? "1" : "0";
  const control = windowEngagementControlDigest(engagement.control);
  const controls = (engagement.controls ?? []).map((row) => windowEngagementControlDigest(row)).join("\u0004");
  return [session, options, input, status, possibles, control, controls].join("\u0003");
}

/** @emoji ⚖️ Returns whether two neutral engagement snapshots are equivalent for shell sync. */
export function windowEngagementsEqual(left: WindowEngagement | undefined, right: WindowEngagement | undefined): boolean {
  return windowEngagementDigest(left) === windowEngagementDigest(right);
}

/** @emoji 💬 Enforces CAD-style window engagement: a command {@link WindowEngagementInput} must be present. */
export function enforcePlaygroundWindowEngagementInput(engagement: WindowEngagement | undefined, contextLabel: string): void {
  if (!engagement?.input) {
    throw new Error(`${contextLabel} must declare engagement.input (command line).`);
  }
}

/** @emoji 💬 Ensures every playground window kind that declares engagement exposes a command {@link WindowEngagementInput}. */
export function enforceWindowKindsEngagementInput(windowKinds: readonly WindowKindRuntime[], contextLabel: string): void {
  for (const windowKind of windowKinds) {
    if (windowKind.engagement === undefined) {
      continue;
    }
    enforcePlaygroundWindowEngagementInput(windowKind.engagement, `${contextLabel} window "${windowKind.id}"`);
  }
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
    templates: readonly import("@framework/core").WindowTemplate[] = [],
  ) {
    super(id, label, bodyKey, iconId, measures, templates);
    this.engagement = engagement;
  }
}
//#endregion 🔖WindowKindRuntime

//#region 🔖OrbitViewLayouts
function orbitViewLayoutPaneToWindow(windowKindId: string, pane: OrbitCameraViewLayoutPane): WindowLayoutWindowNode {
	return createWindowLayout(windowKindId, pane.title ?? undefined, { templateId: pane.view });
}

function orbitViewArrangementToLayout(windowKindId: string, arrangement: OrbitCameraViewLayoutArrangement): WindowLayout {
	switch (arrangement.kind) {
		case "stack":
			return {
				root: {
					kind: "stack",
					children: arrangement.panes.map((pane) => orbitViewLayoutPaneToWindow(windowKindId, pane)),
				},
			};
		case "row":
		case "column":
			return {
				root: {
					kind: arrangement.kind,
					children: arrangement.panes.map((pane) => ({
						kind: "stack" as const,
						...(pane.size !== undefined ? { size: pane.size } : {}),
						children: [orbitViewLayoutPaneToWindow(windowKindId, pane)],
					})),
				},
			};
		case "grid":
			return {
				root: {
					kind: "column",
					children: arrangement.rows.map((row) => ({
						kind: "row" as const,
						...(row.size !== undefined ? { size: row.size } : {}),
						children: row.panes.map((pane) => ({
							kind: "stack" as const,
							children: [orbitViewLayoutPaneToWindow(windowKindId, pane)],
						})),
					})),
				},
			};
	}
}

/** @emoji 🧭 Maps orbit-view layout descriptors into playground {@link NamedLayout} entries. */
export function namedLayoutsFromOrbitViewDescriptors(windowKindId: string, descriptors: readonly OrbitCameraViewLayoutDescriptor[]): NamedLayout[] {
	return descriptors.map((descriptor) =>
		createNamedLayout(descriptor.id, descriptor.label, orbitViewArrangementToLayout(windowKindId, descriptor.arrangement), "builtin", undefined, descriptor.groupPath),
	);
}
//#endregion 🔖OrbitViewLayouts

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
    namedLayouts: mergeNamedLayouts(app.namedLayouts, mode?.namedLayouts),
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
export type { SidePanelBodyViewContext } from "@framework/platform/core";

export {
  getSidePanelBodyFactory,
  registerSidePanelBody,
  unregisterSidePanelBody,
} from "@framework/platform/core";
//#endregion 🔖SidePanelBodyViewContext

//#region 🔖Playground
export interface PlaygroundPanelVisibility {
  readonly leftSidePanel: boolean;
  readonly rightSidePanel: boolean;
}

/** @emoji 🖥️ Product/playground {@link Platform} with glass workbench panels open by default. */
export function createProductPlaygroundPlatform(id: string, name?: string): Platform {
  return new Platform({ id, name: name ?? id, initialPanelVisibility: PRODUCT_SHELL_DEFAULT_PANEL_VISIBILITY });
}

/** @emoji 🧩 Builds a playground {@link AppRuntime} with mode window kinds wired for golden windows. */
export function createPlayAppRuntime(
  id: string,
  label: string,
  controller: import("@framework/core").Controller,
  layout: WindowLayout,
  mode: ModeRuntime,
  iconId?: string,
): AppRuntime {
  const app = new AppRuntime(id, label, iconId, controller, layout, mode.windowKinds);
  app.defaultModeId = mode.id;
  app.addMode(mode);
  return app;
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

//#region 🔖Fixture
/** @emoji ∅ Sentinel id for the navbar “No fixture” row (shared with {@link NAVBAR_NO_FIXTURE_ID} in `@ui/react`). */
export const PLAYGROUND_NO_FIXTURE_ID = "__none__";

/** @emoji 🧪 One selectable playground fixture (kit, graph, shape source, …). */
export interface PlaygroundFixtureOption {
  readonly id: string;
  readonly label: string;
}

/** @emoji ∅ Standard “No fixture” navbar row. */
export const PLAYGROUND_NO_FIXTURE_OPTION: PlaygroundFixtureOption = {
  id: PLAYGROUND_NO_FIXTURE_ID,
  label: "No fixture",
};

/** @emoji 📋 Active fixture plus choices for the navbar center dropdown. */
export interface PlaygroundFixtureCatalog {
  readonly activeFixtureId: string;
  readonly options: readonly PlaygroundFixtureOption[];
}

/** @emoji 🎛 Optional controller surface for {@link PlaygroundView} navbar fixture selection. */
export interface PlaygroundFixtureHost {
  getFixtureCatalog(): PlaygroundFixtureCatalog | null;
}

/** @emoji ∅ True when the navbar selection is empty / “No fixture”. */
export function isPlaygroundNoFixtureId(fixtureId: string | null | undefined): boolean {
  return fixtureId == null || fixtureId === "" || fixtureId === PLAYGROUND_NO_FIXTURE_ID;
}

const playgroundFixtureCatalogWithNoOptionCache = new Map<string, PlaygroundFixtureCatalog>();

function playgroundFixtureCatalogCacheKey(activeFixtureId: string, options: readonly PlaygroundFixtureOption[]): string {
  const normalizedId = isPlaygroundNoFixtureId(activeFixtureId) ? PLAYGROUND_NO_FIXTURE_ID : activeFixtureId;
  return `${normalizedId}\0${options.map((row) => `${row.id}\u0001${row.label}`).join("\0")}`;
}

/** @emoji 📋 Prepends {@link PLAYGROUND_NO_FIXTURE_OPTION} and normalizes the active id. */
export function playgroundFixtureCatalogWithNoOption(
  activeFixtureId: string,
  options: readonly PlaygroundFixtureOption[],
): PlaygroundFixtureCatalog {
  const key = playgroundFixtureCatalogCacheKey(activeFixtureId, options);
  const cached = playgroundFixtureCatalogWithNoOptionCache.get(key);
  if (cached) {
    return cached;
  }
  const withoutNone = options.filter((row) => row.id !== PLAYGROUND_NO_FIXTURE_ID);
  const catalog: PlaygroundFixtureCatalog = {
    activeFixtureId: isPlaygroundNoFixtureId(activeFixtureId) ? PLAYGROUND_NO_FIXTURE_ID : activeFixtureId,
    options: [PLAYGROUND_NO_FIXTURE_OPTION, ...withoutNone],
  };
  playgroundFixtureCatalogWithNoOptionCache.set(key, catalog);
  return catalog;
}

/** @emoji 🔎 Reads a fixture catalog from a controller when it implements {@link PlaygroundFixtureHost}. */
export function resolvePlaygroundFixtureCatalog(controller: Controller | undefined): PlaygroundFixtureCatalog | null {
  if (!controller) return null;
  const host = controller as Controller & PlaygroundFixtureHost;
  if (typeof host.getFixtureCatalog !== "function") return null;
  const catalog = host.getFixtureCatalog();
  if (!catalog) return null;
  return playgroundFixtureCatalogWithNoOption(catalog.activeFixtureId, catalog.options);
}
//#endregion 🔖Fixture

//#region 🔖Toolbar
function createAllKindsEnabled<K extends string>(kinds: readonly K[]): Record<K, boolean> {
  return Object.fromEntries(kinds.map((kind) => [kind, true])) as Record<K, boolean>;
}

/** @emoji 🎚 Kind toggle row for playground `selection` or `filter` toolbar zones. */
export function buildPlaygroundKindToggleTools<K extends string>(prefix: "selection" | "filter", kinds: readonly K[], labels: (kind: K) => string, values: Readonly<Record<K, boolean>>, controllerId: string, command: string): ToolItem[] {
  return kinds.map((kind, order) => ({
    id: `playground.${prefix}.${kind}`,
    kind: "toggle" as const,
    iconId: `playground.${prefix}.${kind}`,
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
    iconId: "x",
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
    { id: `${ids.appId}.workbench`, iconId: ids.workbenchIconId, panel: "workbench", order: 0, bodyKey: ids.workbenchTabBodyKey, label: "Workbench" },
    { id: `${ids.appId}.details`, iconId: ids.detailsIconId, panel: "details", order: 0, bodyKey: ids.detailsTabBodyKey, label: "Details" },
  ];
  controller.commandBus.dispatch(controller.id, "setQuery", { query: options?.initialQuery ?? "" });
  return app;
}

/** @emoji 🧭 Resolves the platform snapshot from playground (`runtime`) or product shell (`platform`) body context. */
export function platformFromViewContext(ctx: SidePanelBodyViewContext | WindowBodyViewContext): Platform | undefined {
  const snapshot = ctx as SidePanelBodyViewContext & WindowBodyViewContext;
  return snapshot.platform ?? snapshot.runtime;
}

export function playgroundControllerFromContext(ctx: WindowBodyViewContext | SidePanelBodyViewContext): PlaygroundController<string> | undefined {
  return platformFromViewContext(ctx)?.getActiveApp()?.controller as PlaygroundController<string> | undefined;
}

/** @emoji 🪟 Declarative main window: lone puzzle3d viewport surface. */
export function buildPlaygroundMainWindowBody(ids: PlaygroundIds, ctx: WindowBodyViewContext): UiNode {
  if (!playgroundControllerFromContext(ctx)) {
    return { type: "text", value: "Missing playground controller" };
  }
  return buildPuzzle3dWindowBody(ids.mainPuzzle3dViewportSurfaceId, ids.controllerId);
}

/** @emoji 📋 Declarative workbench side tab: host-bound table surface in a single tree item. */
export function buildPlaygroundWorkbenchPanelBody(ids: PlaygroundIds, ctx: SidePanelBodyViewContext): UiTreeNode {
  if (!playgroundControllerFromContext(ctx)) {
    return sidePanelTreeRootItems("playground.workbench.missing", [{ id: "missing", label: "Missing playground controller" }]);
  }
  return {
    type: "tree",
    sections: [
      {
        id: "playground.workbench.table",
        label: "Workbench",
        defaultOpen: true,
        items: [
          {
            id: "playground.workbench.table.host",
            label: "",
            control: {
              type: "table",
              componentKind: "table",
              surfaceId: ids.workbenchPanelSurfaceId,
              controllerId: ids.controllerId,
            },
          },
        ],
      },
    ],
  };
}

/** @emoji 🔎 Declarative details side tab: host-bound table surface in a single tree item. */
export function buildPlaygroundDetailsPanelBody(ids: PlaygroundIds, ctx: SidePanelBodyViewContext): UiTreeNode {
  if (!playgroundControllerFromContext(ctx)) {
    return sidePanelTreeRootItems("playground.details.missing", [{ id: "missing", label: "Missing playground controller" }]);
  }
  return {
    type: "tree",
    sections: [
      {
        id: "playground.details.table",
        label: "Details",
        defaultOpen: true,
        items: [
          {
            id: "playground.details.table.host",
            label: "",
            control: {
              type: "table",
              componentKind: "table",
              surfaceId: ids.detailsPanelSurfaceId,
              controllerId: ids.controllerId,
            },
          },
        ],
      },
    ],
  };
}

export interface RegisterPlaygroundDeclarativeBodiesOptions {
  readonly buildMainWindow?: (ctx: WindowBodyViewContext) => UiNode;
  readonly buildWorkbenchPanel?: (ctx: SidePanelBodyViewContext) => UiNode;
  readonly buildDetailsPanel?: (ctx: SidePanelBodyViewContext) => UiNode;
}

export interface PlaygroundSidePanelBodyRegistration {
  readonly bodyKey: string;
  readonly build: (ctx: SidePanelBodyViewContext) => UiTreeNode;
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

  describe("enforceWindowKindsEngagementInput", () => {
    it("allows window kinds without engagement", () => {
      expect(() => enforceWindowKindsEngagementInput([new WindowKindRuntime("w", "W", "body")], "Test app")).not.toThrow();
    });

    it("throws when a window kind declares engagement without input", () => {
      expect(() =>
        enforceWindowKindsEngagementInput(
          [new WindowKindRuntime("w", "W", "body", undefined, [], { options: [{ id: "a", label: "A" }] })],
          "Test app",
        ),
      ).toThrow(/engagement\.input/);
    });
  });

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

    it("windowEngagementDigest includes control value and command routing", () => {
      const left: WindowEngagement = {
        input: { id: "engagement-input", onChange: { controllerId: "ctrl", command: "engagementInput" } },
        control: { kind: "stepper", value: 1, min: 0, onChange: { controllerId: "ctrl", command: "engagementControlChange" } },
      };
      const right: WindowEngagement = {
        input: { id: "engagement-input", onChange: { controllerId: "ctrl", command: "engagementInput" } },
        control: { kind: "stepper", value: 2, min: 0, onChange: { controllerId: "ctrl", command: "engagementControlChange" } },
      };
      expect(windowEngagementsEqual(left, left)).toBe(true);
      expect(windowEngagementsEqual(left, right)).toBe(false);
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

  describe("registerSidePanelBody", () => {
    it("rejects non-tree panel bodies", () => {
      const key = "test.playground.panel.invalid";
      registerSidePanelBody(key, () => ({ type: "text", value: "x" }) as UiTreeNode);
      expect(() => getSidePanelBodyFactory(key)!({} as SidePanelBodyViewContext)).toThrow(/must be type "tree"/);
      unregisterSidePanelBody(key);
    });
    it("accepts tree panel bodies with sections", () => {
      const key = "test.playground.panel.tree";
      registerSidePanelBody(key, () => ({
        type: "tree",
        sections: [{ id: "s", items: [{ id: "i", label: "Item" }] }],
      }));
      expect(getSidePanelBodyFactory(key)?.({} as SidePanelBodyViewContext).type).toBe("tree");
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

  describe("playgroundFixtureCatalogWithNoOption", () => {
    it("prepends No fixture and normalizes empty active ids", () => {
      const catalog = playgroundFixtureCatalogWithNoOption("", [{ id: "a", label: "Alpha" }]);
      expect(catalog.activeFixtureId).toBe(PLAYGROUND_NO_FIXTURE_ID);
      expect(catalog.options[0]).toEqual(PLAYGROUND_NO_FIXTURE_OPTION);
      expect(catalog.options).toHaveLength(2);
    });
  });

  describe("resolvePlaygroundFixtureCatalog", () => {
    it("returns null when the controller does not host fixtures", () => {
      const bus = new CommandBus();
      const ctrl = new DemoPlaygroundController(bus, () => undefined);
      expect(resolvePlaygroundFixtureCatalog(ctrl)).toBeNull();
    });

    it("returns catalog with No fixture when the controller implements PlaygroundFixtureHost", () => {
      class FixtureDemoController extends Controller implements PlaygroundFixtureHost {
        activeFixtureId = "a";

        constructor(bus: CommandBus) {
          super("fixture-demo", bus, () => undefined);
        }

        getFixtureCatalog(): PlaygroundFixtureCatalog {
          return {
            activeFixtureId: this.activeFixtureId,
            options: [
              { id: "a", label: "Alpha" },
              { id: "b", label: "Beta" },
            ],
          };
        }

        run(): void {}
      }
      const bus = new CommandBus();
      const ctrl = new FixtureDemoController(bus);
      expect(resolvePlaygroundFixtureCatalog(ctrl)?.activeFixtureId).toBe("a");
      expect(resolvePlaygroundFixtureCatalog(ctrl)?.options[0]?.id).toBe(PLAYGROUND_NO_FIXTURE_ID);
      expect(resolvePlaygroundFixtureCatalog(ctrl)?.options).toHaveLength(3);
    });

    it("playgroundFixtureCatalogWithNoOption returns a stable catalog reference for identical inputs", () => {
      const options = [
        { id: "a", label: "Alpha" },
        { id: "b", label: "Beta" },
      ] as const;
      const first = playgroundFixtureCatalogWithNoOption("a", options);
      const second = playgroundFixtureCatalogWithNoOption("a", options);
      expect(second).toBe(first);
    });

    it("returns only No fixture when the host lists no presets", () => {
      class EmptyFixtureController extends Controller implements PlaygroundFixtureHost {
        constructor(bus: CommandBus) {
          super("empty-fixture", bus, () => undefined);
        }

        getFixtureCatalog(): PlaygroundFixtureCatalog {
          return { activeFixtureId: PLAYGROUND_NO_FIXTURE_ID, options: [] };
        }

        run(): void {}
      }
      const bus = new CommandBus();
      const catalog = resolvePlaygroundFixtureCatalog(new EmptyFixtureController(bus));
      expect(catalog?.options).toEqual([PLAYGROUND_NO_FIXTURE_OPTION]);
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
    it("createProductPlaygroundPlatform opens glass side panels by default", () => {
      const platform = createProductPlaygroundPlatform("procedural-play", "Procedural");
      expect(platform.initialPanelVisibility).toEqual(PRODUCT_SHELL_DEFAULT_PANEL_VISIBILITY);
      expect(platform.panelVisibility).toEqual(PRODUCT_SHELL_DEFAULT_PANEL_VISIBILITY);
    });

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
