// #region 🧱Header
/** 🧱 `@elements/framework` — Renderer-agnostic product shell: {@link ProductRuntime} → {@link AppRuntime} → {@link ModeRuntime}, declarative {@link UiNode} bodies, {@link PluginHost}, {@link SurfaceRouter}, and {@link ProductDefinition} + {@link SurfaceDefinition} for contribution routing. */
// #endregion 🧱Header

//#region 🔖JsonValue
export type JsonPrimitive = string | number | boolean | null;
export type JsonValue = JsonPrimitive | readonly JsonValue[] | { readonly [key: string]: JsonValue };
//#endregion 🔖JsonValue

//#region 🔖Disposable
/** @emoji 🧹 Reversible teardown handle for surfaces and plugin activation. */
export interface Disposable {
	dispose(): void;
}
//#endregion 🔖Disposable

//#region 🔖Commands
/** @emoji 🎯 Semantic command routed through the host to {@link CommandBus.dispatch}. */
export interface CommandDescriptor {
	readonly controllerId: string;
	readonly command: string;
	readonly args?: JsonValue;
}
//#endregion 🔖Commands

//#region 🔖Style
/** @emoji 🎨 Tokenized chrome hints mapped by the renderer. */
export interface StyleSpec {
	readonly variant?: "default" | "subtle" | "danger" | "success";
	readonly size?: "small" | "medium" | "large";
	readonly density?: "compact" | "normal" | "comfortable";
}
//#endregion 🔖Style

//#region 🔖UiNode
export interface UiStackNode {
	readonly type: "stack";
	readonly direction: "horizontal" | "vertical";
	readonly gap?: "none" | "tight" | "standard" | "relaxed";
	readonly padding?: "none" | "standard";
	readonly children: readonly UiNode[];
}

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

/** @emoji 🧊 Host-bound 3D surface; renderer maps `surfaceId` to the canvas implementation. */
export interface UiScene3DHostSurfaceNode {
	readonly type: "scene3d";
	readonly surfaceId: string;
	readonly controllerId: string;
}

/** @emoji 📋 Host-bound 2D board canvas; `paneId` selects the window slot. */
export interface UiBoardHostSurfaceNode {
	readonly type: "board";
	readonly surfaceId: string;
	readonly controllerId: string;
	readonly paneId: string;
}

/** @emoji 📊 Host-bound tabular surface; `paneId` disambiguates multiple table slots in one app. */
export interface UiTableHostSurfaceNode {
	readonly type: "table";
	readonly surfaceId: string;
	readonly controllerId: string;
	readonly paneId?: string;
}

/** @emoji 🧩 Host-bound side-panel surface; renderer maps `surfaceId` to panel body chrome. */
export interface UiPanelHostSurfaceNode {
	readonly type: "panel";
	readonly surfaceId: string;
	readonly controllerId: string;
}

export type UiNode =
	| UiStackNode
	| UiTextNode
	| UiButtonNode
	| UiSeparatorNode
	| UiScene3DHostSurfaceNode
	| UiBoardHostSurfaceNode
	| UiTableHostSurfaceNode
	| UiPanelHostSurfaceNode;

/** @emoji 🧊 Canonical fullscreen 3D window body: only the infinite scene canvas. */
export function buildScene3dWindowBody(surfaceId: string, controllerId: string): UiScene3DHostSurfaceNode {
	return { type: "scene3d", surfaceId, controllerId };
}

/** @emoji 📋 Canonical fullscreen 2D window body: only the infinite board canvas. */
export function buildBoardWindowBody(surfaceId: string, controllerId: string, paneId: string): UiBoardHostSurfaceNode {
	return { type: "board", surfaceId, controllerId, paneId };
}

/** @emoji 📊 Canonical table window body: only the host-bound table surface. */
export function buildTableWindowBody(surfaceId: string, controllerId: string, paneId?: string): UiTableHostSurfaceNode {
	return paneId ? { type: "table", surfaceId, controllerId, paneId } : { type: "table", surfaceId, controllerId };
}

/** @emoji ✅ True when a window body is a lone surface (`table` / `scene3d` / `board`) or a short error `text` node. */
export function isCanvasOnlyWindowBody(node: UiNode): boolean {
	if (node.type === "text" || node.type === "scene3d" || node.type === "board" || node.type === "table") return true;
	if (node.type === "stack" && node.padding === "none" && node.children.length === 1) {
		const child = node.children[0];
		return child.type === "scene3d" || child.type === "board" || child.type === "table";
	}
	return false;
}

function assertCanvasOnlyWindowBody(bodyKey: string, node: UiNode): void {
	if (isCanvasOnlyWindowBody(node)) return;
	throw new Error(
		`Declarative window body "${bodyKey}" must be a single table, scene3d, or board surface (optional none padding stack wrapper). Found "${node.type}". Use ModeRuntime.tools, side tabs, or window measures for chrome.`,
	);
}
//#endregion 🔖UiNode

//#region 🔖WindowMeasure
/** @emoji 📐 Framework-free window measure; host maps `onChange` to {@link CommandBus.dispatch}. */
export interface WindowMeasureSelect {
	readonly kind: "select";
	readonly id: string;
	readonly label?: string;
	readonly value: string;
	readonly items: readonly { readonly id: string; readonly value: string; readonly label: string }[];
	readonly onChange: CommandDescriptor;
}

export type WindowMeasure = WindowMeasureSelect;
//#endregion 🔖WindowMeasure

//#region 🔖Layout
/** @emoji 🪟 Single window slot in the abstract layout tree. */
export interface WindowLayoutWindowNode {
	readonly kind: "window";
	readonly windowKindId: string;
	readonly title?: string;
}

/** @emoji 📚 Tab stack node grouping window slots. */
export interface WindowLayoutStackNode {
	readonly kind: "stack";
	readonly size?: number;
	readonly children: readonly WindowLayoutWindowNode[];
}

/** @emoji ↔️ Row/column branch in the abstract layout tree. */
export interface WindowLayoutAxisNode {
	readonly kind: "row" | "column";
	readonly size?: number;
	readonly children: readonly (WindowLayoutAxisNode | WindowLayoutStackNode)[];
}

/** @emoji 🧭 Root layout wrapper owned by an app. */
export interface WindowLayout {
	readonly root: WindowLayoutAxisNode | WindowLayoutStackNode;
}
//#endregion 🔖Layout

//#region 🔖LayoutFactories
/** @emoji 🪟 Single window slot helper for {@link WindowLayout} trees. */
export function createWindowLayout(windowKindId: string, title?: string): WindowLayoutWindowNode {
	return { kind: "window", windowKindId, ...(title ? { title } : {}) };
}

/** @emoji 📚 Stack layout from ordered window kind ids. */
export function createStackLayout(windowKindIds: string[], titles?: string[]): WindowLayout {
	return {
		root: {
			kind: "stack",
			children: windowKindIds.map((windowKindId, index) => createWindowLayout(windowKindId, titles?.[index])),
		},
	};
}

/** @emoji 🧱 Default row/column of one stack per window kind (Golden-style ownership). */
export function createDefaultLayout(windowIds: string[], direction: "row" | "column" = "row", sizes?: number[], titles?: string[]): WindowLayout {
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

/** @emoji 📑 Single stack with every window as a tab group. */
export function createTabStackLayout(windowIds: string[], titles?: string[]): WindowLayout {
	return createStackLayout(windowIds, titles);
}
//#endregion 🔖LayoutFactories

//#region 🔖Expertise
/** @emoji 🎚 Surface expertise tier for chrome + label resolution. */
export enum Expertise {
	BEGINNER = "beginner",
	NORMAL = "normal",
	EXPERT = "expert",
}
//#endregion 🔖Expertise

//#region 🔖Toolbar
/** @emoji 🧰 Toolbar category ids shared by every app registration surface. */
export type AppToolCategory = "history" | "hand" | "selection" | "lasso" | "filter" | "open" | "create" | "view" | "actions" | "settings";

/** @emoji 📋 Default toolbar category order. */
export const APP_TOOL_CATEGORY_ORDER: readonly AppToolCategory[] = ["history", "hand", "selection", "lasso", "filter", "open", "create", "view", "actions", "settings"];

/** @emoji 🎛 Declarative toolbar item; interactions route through {@link CommandBus}. */
export interface ToolItem {
	readonly id: string;
	readonly kind: "button" | "toggle" | "separator";
	readonly iconId?: string;
	readonly label?: string;
	readonly text?: string;
	readonly order?: number;
	readonly pressed?: boolean;
	readonly disabled?: boolean;
	readonly controllerId?: string;
	readonly command?: string;
	readonly args?: unknown;
}

/** @emoji 🗂️ Per-category toolbar maps. */
export type AppTools = Partial<Record<AppToolCategory, readonly ToolItem[]>>;

/** @emoji 🔀 Merges toolbar tool maps per category (extension appends within each category). */
export function mergeAppTools(base?: AppTools, extension?: AppTools): AppTools | undefined {
	if (!base && !extension) return undefined;
	const merged: AppTools = {};
	for (const category of APP_TOOL_CATEGORY_ORDER) {
		const combined = [...(base?.[category] ?? []), ...(extension?.[category] ?? [])];
		if (combined.length > 0) (merged as Record<string, readonly ToolItem[]>)[category] = combined;
	}
	return Object.keys(merged).length > 0 ? merged : undefined;
}

/** @emoji 🧮 Counts toolbar items across populated categories. */
export function countAppTools(tools?: AppTools): number {
	if (!tools) return 0;
	return APP_TOOL_CATEGORY_ORDER.reduce((sum, category) => sum + (tools[category]?.length ?? 0), 0);
}

function hasAppToolCategoryItems(items: readonly ToolItem[] | undefined): boolean {
	return Boolean(items?.some((item) => item.kind !== "separator"));
}

/** @emoji 📂 Lists categories that have at least one non-separator tool. */
export function listPopulatedToolCategories(tools?: AppTools): AppToolCategory[] {
	if (!tools) return [];
	return APP_TOOL_CATEGORY_ORDER.filter((category) => hasAppToolCategoryItems(tools[category]));
}
//#endregion 🔖Toolbar

//#region 🔖CommandsPalette
/** @emoji 🔎 Command palette row spec resolved in React (icons + `onSelect`). */
export interface SearchItemSpec {
	readonly id: string;
	readonly label: string;
	readonly description?: string;
	readonly category?: string;
	readonly iconId?: string;
	readonly controllerId: string;
	readonly command: string;
	readonly args?: unknown;
}

/** @emoji 🔀 Merges command palette rows by `id`, favoring the more specific extension. */
export function mergeSearchItems(base?: readonly SearchItemSpec[], extension?: readonly SearchItemSpec[]): SearchItemSpec[] | undefined {
	return mergeById(base, extension);
}
//#endregion 🔖CommandsPalette

//#region 🔖SideTab
/** @emoji 📑 Side panel tab addressing a React-registered `bodyKey` tree host. */
export interface SideTabSpec {
	readonly id: string;
	readonly iconId: string;
	readonly order?: number;
	readonly bodyKey: string;
}
//#endregion 🔖SideTab

//#region 🔖Find
/** @emoji 🔎 Find palette row (label-only; renderer supplies icons). */
export interface FindItem {
	readonly id: string;
	readonly label: string;
	readonly description?: string;
	readonly category?: string;
}
//#endregion 🔖Find

//#region 🔖Footer
/** @emoji 👣 Footer strip item with optional command dispatch. */
export interface FooterItem {
	readonly id: string;
	readonly text?: string;
	readonly order?: number;
	readonly iconId?: string;
	readonly className?: string;
	readonly disabled?: boolean;
	readonly controllerId?: string;
	readonly command?: string;
	readonly args?: unknown;
}
//#endregion 🔖Footer

//#region 🔖Observable
/** @emoji 📡 Minimal listener set for host invalidation without external reactive libs. */
export type ProductSubscriber = () => void;

/** @emoji 📦 Holds a value and notifies subscribers on `set`. */
export class ObservableCell<T> {
	private value: T;
	private readonly listeners = new Set<ProductSubscriber>();

	constructor(initial: T) {
		this.value = initial;
	}

	get(): T {
		return this.value;
	}

	set(next: T): void {
		if (Object.is(this.value, next)) return;
		this.value = next;
		for (const listener of this.listeners) listener();
	}

	update(updater: (previous: T) => T): void {
		this.set(updater(this.value));
	}

	subscribe(listener: ProductSubscriber): () => void {
		this.listeners.add(listener);
		return () => this.listeners.delete(listener);
	}
}
//#endregion 🔖Observable

//#region 🔖CommandBus
/** @emoji 🚦 Routes toolbar/footer commands to {@link Controller} instances by id. */
export class CommandBus {
	private readonly controllers = new Map<string, Controller>();

	register(controller: Controller): void {
		this.controllers.set(controller.id, controller);
	}

	unregister(controllerId: string): void {
		this.controllers.delete(controllerId);
	}

	dispatch(controllerId: string, command: string, args?: unknown): void {
		this.controllers.get(controllerId)?.run(command, args);
	}
}

/** @emoji 🎮 Base class for imperative app controllers participating in {@link CommandBus}. */
export abstract class Controller {
	readonly id: string;
	readonly commandBus: CommandBus;
	private readonly hostNotify: () => void;

	protected constructor(id: string, commandBus: CommandBus, hostNotify: () => void) {
		this.id = id;
		this.commandBus = commandBus;
		this.hostNotify = hostNotify;
		commandBus.register(this);
	}

	protected emit(): void {
		this.hostNotify();
	}

	dispose(): void {
		this.commandBus.unregister(this.id);
	}

	abstract run(command: string, args?: unknown): void;
}
//#endregion 🔖CommandBus

//#region 🔖ContextKeys
/** @emoji 🔑 Opaque context bag for `SurfaceSelector.when` resolution (products inject evaluators). */
export type ContextKey = string;

export type ContextKeyResolver = (when: string | undefined) => boolean;

export const matchAllContext: ContextKeyResolver = (when) => when === undefined || when === "" || when === "*";
//#endregion 🔖ContextKeys

//#region 🔖Capability
/** @emoji 🏷 Semantic affordance string attached to surfaces and matched by plugin selectors. */
export type Capability = string;

/** @emoji ✅ True when `required` ⊆ `available` as a set. */
export function capabilitiesSatisfy(available: readonly Capability[], required: readonly Capability[]): boolean {
	for (const c of required) {
		if (!available.includes(c)) return false;
	}
	return true;
}
//#endregion 🔖Capability

//#region 🔖SurfaceDefinition
/** @emoji 🪟 Extension-capable area: typed API factory + contribution application. */
export interface SurfaceDefinition<TApi = unknown, TContribution = unknown> {
	readonly id: string;
	readonly appId: string;
	readonly modeId: string;
	readonly windowKindId: string;
	readonly kind: "window" | "toolbar" | "panel" | "overlay" | "tool" | "menu" | "inspector" | "analysis" | string;
	readonly capabilities: readonly Capability[];
	createApi(ctx: SurfaceContext): TApi;
	applyContribution(contribution: TContribution, ctx: SurfaceContext, api: TApi): Disposable;
}

/** @emoji 🔗 Typed pair used in {@link ProductDefinition} surface maps. */
export interface SurfaceBinding<TApi, TContribution> {
	readonly api: TApi;
	readonly contributions: TContribution;
}
//#endregion 🔖SurfaceDefinition

//#region 🔖WindowKindRuntime
/** @emoji 🪟 Declarative window kind; React renderer maps `bodyKey` to a component. */
export class WindowKindRuntime {
	readonly capabilities: Capability[] = [];
	readonly surfaces: SurfaceDefinition[] = [];
	commands: SearchItemSpec[] = [];

	constructor(
		readonly id: string,
		readonly label: string,
		readonly bodyKey: string,
		readonly iconId?: string,
		readonly measures: readonly WindowMeasure[] = [],
		capabilities?: readonly Capability[],
	) {
		if (capabilities?.length) this.capabilities.push(...capabilities);
	}
}
//#endregion 🔖WindowKindRuntime

//#region 🔖ModeRuntime
/** @emoji 🎚 Single app mode: toolbars, window kinds, and side tab specs. */
export class ModeRuntime {
	tools: AppTools = {};
	commands: SearchItemSpec[] = [];
	windowKinds: WindowKindRuntime[] = [];
	defaultLayout?: WindowLayout;
	leftTabs: SideTabSpec[] = [];
	rightTabs: SideTabSpec[] = [];
	footerItems: FooterItem[] = [];
	findItems: FindItem[] = [];
	onFindSelect?: (itemId: string) => void;
	onActiveWindowChange?: (windowKindId: string) => void;
	selection: Record<string, unknown> = {};
	hover: Record<string, unknown> = {};
	options: Record<string, unknown> = {};

	constructor(
		readonly id: string,
		readonly label: string,
		readonly iconId: string | undefined,
	) {}
}
//#endregion 🔖ModeRuntime

//#region 🔖Merge
function mergeById<T extends { id: string }>(base: readonly T[] | undefined, extension: readonly T[] | undefined): T[] | undefined {
	if (!base?.length && !extension?.length) return undefined;
	const merged = new Map<string, T>();
	base?.forEach((entry) => merged.set(entry.id, entry));
	extension?.forEach((entry) => merged.set(entry.id, entry));
	return [...merged.values()];
}

function resolveMode(app: AppRuntime, requestedModeId: string | null | undefined): ModeRuntime | null {
	if (!app.modes.length) return null;
	if (requestedModeId) {
		const matching = app.modes.find((mode) => mode.id === requestedModeId);
		if (matching) return matching;
	}
	if (app.defaultModeId) {
		const matching = app.modes.find((mode) => mode.id === app.defaultModeId);
		if (matching) return matching;
	}
	return app.modes[0] ?? null;
}
//#endregion 🔖Merge

//#region 🔖ResolvedState
/** @emoji 📸 Merged view of app + active mode used by the React product bridge. */
export interface ResolvedAppState {
	readonly id: string;
	readonly activeModeId: string | null;
	readonly label: string;
	readonly iconId: string | undefined;
	readonly tools: AppTools | undefined;
	readonly commands: SearchItemSpec[];
	readonly windowKinds: readonly WindowKindRuntime[];
	readonly defaultLayout: WindowLayout;
	readonly leftTabs: SideTabSpec[];
	readonly rightTabs: SideTabSpec[];
	readonly footerItems: FooterItem[];
	readonly findItems: FindItem[];
	readonly onFindSelect?: (itemId: string) => void;
	readonly onActiveWindowChange?: (windowKindId: string) => void;
	readonly selection: Record<string, unknown>;
	readonly hover: Record<string, unknown>;
	readonly options: Record<string, unknown>;
}

/** @emoji 🧮 Resolves active mode overlays exactly like the prior `resolveWorkbenchAppState` merge. */
export function resolveAppState(app: AppRuntime, requestedModeId?: string | null): ResolvedAppState {
	const mode = resolveMode(app, requestedModeId);
	const mergedWindowKinds = mergeById(app.windowKinds, mode?.windowKinds) ?? app.windowKinds;
	const mergedLeft = mergeById(app.leftTabs, mode?.leftTabs) ?? app.leftTabs;
	const mergedRight = mergeById(app.rightTabs, mode?.rightTabs) ?? app.rightTabs;
	return {
		id: app.id,
		activeModeId: mode?.id ?? null,
		label: mode?.label ?? app.label,
		iconId: mode?.iconId ?? app.iconId,
		tools: mergeAppTools(app.tools, mode?.tools),
		commands: mergeSearchItems(app.commands, mode?.commands) ?? app.commands,
		windowKinds: mergedWindowKinds,
		defaultLayout: mode?.defaultLayout ?? app.defaultLayout,
		leftTabs: mergedLeft,
		rightTabs: mergedRight,
		footerItems: mergeById(app.footerItems, mode?.footerItems) ?? app.footerItems,
		findItems: mergeById(app.findItems, mode?.findItems) ?? app.findItems,
		onFindSelect: mode?.onFindSelect ?? app.onFindSelect,
		onActiveWindowChange: mode?.onActiveWindowChange ?? app.onActiveWindowChange,
		selection: { ...app.selection, ...(mode?.selection ?? {}) },
		hover: { ...app.hover, ...(mode?.hover ?? {}) },
		options: { ...app.options, ...(mode?.options ?? {}) },
	};
}
//#endregion 🔖ResolvedState

//#region 🔖AppRuntime
/** @emoji 🧩 One registered app with modes, layout, and a primary {@link Controller}. */
export class AppRuntime {
	readonly modes: ModeRuntime[] = [];
	defaultModeId?: string;
	private activeModeIdOverride: string | null = null;
	windowKinds: WindowKindRuntime[] = [];
	defaultLayout!: WindowLayout;
	tools: AppTools = {};
	commands: SearchItemSpec[] = [];
	leftTabs: SideTabSpec[] = [];
	rightTabs: SideTabSpec[] = [];
	footerItems: FooterItem[] = [];
	findItems: FindItem[] = [];
	onFindSelect?: (itemId: string) => void;
	onActiveWindowChange?: (windowKindId: string) => void;
	selection: Record<string, unknown> = {};
	hover: Record<string, unknown> = {};
	options: Record<string, unknown> = {};
	readonly controller: Controller;

	constructor(
		readonly id: string,
		readonly label: string,
		readonly iconId: string | undefined,
		controller: Controller,
		layout: WindowLayout,
		windowKinds: readonly WindowKindRuntime[],
	) {
		this.controller = controller;
		this.defaultLayout = layout;
		this.windowKinds = [...windowKinds];
	}

	addMode(mode: ModeRuntime): void {
		this.modes.push(mode);
	}

	getActiveModeId(): string | null {
		if (this.activeModeIdOverride) return this.activeModeIdOverride;
		return resolveMode(this, null)?.id ?? null;
	}

	setActiveModeId(modeId: string | null): void {
		this.activeModeIdOverride = modeId;
	}

	resolve(requestedModeId?: string | null): ResolvedAppState {
		const modeId = requestedModeId ?? this.getActiveModeId();
		return resolveAppState(this, modeId);
	}
}
//#endregion 🔖AppRuntime

//#region 🔖ProductRuntime
/** @emoji 🖥️ Root shell: apps, URI chrome, panel toggles, and shared {@link CommandBus}. */
export class ProductRuntime {
	readonly commandBus = new CommandBus();
	private readonly listeners = new Set<ProductSubscriber>();
	readonly apps: AppRuntime[] = [];
	activeAppId = "";
	generation = 0;
	uri = "/";
	canGoBack = false;
	canGoForward = false;
	canGoUp = false;
	onNavigate?: (uri: string) => void;
	onGoBack?: () => void;
	onGoForward?: () => void;
	onGoUp?: () => void;
	globalTools: AppTools | undefined;
	commands: readonly SearchItemSpec[] = [];
	globalFooterItems: FooterItem[] = [];
	searchItems: readonly SearchItemSpec[] = [];
	mobile: boolean | undefined;
	mobileQuery = "(max-width: 767px)";
	className = "";
	panelVisibility = { leftSidePanel: false, rightSidePanel: false };
	initialPanelVisibility?: { leftSidePanel: boolean; rightSidePanel: boolean };

	notify(): void {
		this.generation++;
		for (const listener of this.listeners) listener();
	}

	subscribe(listener: ProductSubscriber): () => void {
		this.listeners.add(listener);
		return () => this.listeners.delete(listener);
	}

	addApp(app: AppRuntime): void {
		this.apps.push(app);
		if (!this.activeAppId) this.activeAppId = app.id;
		this.notify();
	}

	getActiveApp(): AppRuntime | undefined {
		return this.apps.find((app) => app.id === this.activeAppId) ?? this.apps[0];
	}

	setActiveAppId(id: string): void {
		this.activeAppId = id;
		this.notify();
	}

	setPanelVisibility(next: { leftSidePanel: boolean; rightSidePanel: boolean }): void {
		this.panelVisibility = next;
		this.notify();
	}
}
//#endregion 🔖ProductRuntime

/** @emoji 🧭 Resolves the command palette rows visible for the active UI/app/mode/window scope. */
export function resolveCommandPaletteItems(runtime: ProductRuntime, app: ResolvedAppState, activeWindowKindId?: string | null): SearchItemSpec[] {
	const uiCommands = mergeSearchItems(runtime.searchItems, runtime.commands) ?? runtime.commands;
	const windowKind = activeWindowKindId ? app.windowKinds.find((entry) => entry.id === activeWindowKindId) : undefined;
	return mergeSearchItems(mergeSearchItems(uiCommands, app.commands), windowKind?.commands) ?? [];
}

//#region 🔖WindowBodyViewContext
/** @emoji 🪟 View context for declarative window bodies: product snapshot without DOM or React roots. */
export interface WindowBodyViewContext {
	readonly runtime: ProductRuntime;
	readonly windowKindId: string;
	readonly bodyKey: string;
	readonly activeModeId: string | null;
	readonly generation: number;
}

const windowBodyByKey = new Map<string, (ctx: WindowBodyViewContext) => UiNode>();

/** @emoji 📝 Registers a framework-free window body tree for `bodyKey` (host renders DOM). */
export function registerWindowBody(bodyKey: string, build: (ctx: WindowBodyViewContext) => UiNode): void {
	windowBodyByKey.set(bodyKey, (ctx) => {
		const node = build(ctx);
		assertCanvasOnlyWindowBody(bodyKey, node);
		return node;
	});
}

/** @emoji 🔍 Returns the declarative builder registered for `bodyKey`, if any. */
export function getWindowBodyFactory(bodyKey: string): ((ctx: WindowBodyViewContext) => UiNode) | undefined {
	return windowBodyByKey.get(bodyKey);
}

/** @emoji 🧹 Removes a declarative window registration (tests / hot reload). */
export function unregisterWindowBody(bodyKey: string): void {
	windowBodyByKey.delete(bodyKey);
}
//#endregion 🔖WindowBodyViewContext

//#region 🔖SidePanelBodyViewContext
/** @emoji 📑 View context for declarative side-panel tab bodies (same snapshot fields as window bodies). */
export type SidePanelBodyViewContext = WindowBodyViewContext;

const sidePanelBodyByKey = new Map<string, (ctx: SidePanelBodyViewContext) => UiNode>();

/** @emoji 📝 Registers a framework-free side-panel tree for `bodyKey`. */
export function registerSidePanelBody(bodyKey: string, build: (ctx: SidePanelBodyViewContext) => UiNode): void {
	sidePanelBodyByKey.set(bodyKey, build);
}

/** @emoji 🔍 Returns the declarative side-panel builder for `bodyKey`, if any. */
export function getSidePanelBodyFactory(bodyKey: string): ((ctx: SidePanelBodyViewContext) => UiNode) | undefined {
	return sidePanelBodyByKey.get(bodyKey);
}

/** @emoji 🧹 Removes a declarative side-panel registration (tests). */
export function unregisterSidePanelBody(bodyKey: string): void {
	sidePanelBodyByKey.delete(bodyKey);
}
//#endregion 🔖SidePanelBodyViewContext

//#region 🔖ProductDefinition
/** @emoji 🧭 Static product graph: apps, modes, window kinds, and surfaces (serializable + typed). */
export interface WindowKindDefinition {
	readonly id: string;
	readonly appId: string;
	readonly modeId: string;
	readonly kind: "table" | "diagram" | "scene" | string;
	readonly label: string;
	readonly capabilities: readonly Capability[];
	readonly bodyKey?: string;
	readonly iconId?: string;
	readonly measures?: readonly WindowMeasure[];
	readonly surfaces: readonly SurfaceDefinition[];
}

export interface ModeDefinition {
	readonly id: string;
	readonly label: string;
	readonly iconId?: string;
	readonly windowKinds: readonly WindowKindDefinition[];
	readonly defaultLayout?: WindowLayout;
	readonly tools?: AppTools;
	readonly leftTabs?: readonly SideTabSpec[];
	readonly rightTabs?: readonly SideTabSpec[];
}

export interface AppDefinition {
	readonly id: string;
	readonly label: string;
	readonly iconId?: string;
	readonly controllerId: string;
	readonly modes: readonly ModeDefinition[];
	readonly defaultModeId?: string;
}

export interface ProductDefinition<TProductApi = unknown> {
	readonly id: string;
	readonly name: string;
	readonly apiVersion: string;
	readonly apps: readonly AppDefinition[];
	createProductApi(ctx: PluginContext): TProductApi;
}
//#endregion 🔖ProductDefinition

//#region 🔖SurfaceContext
/** @emoji 🧩 Activation context for a single {@link SurfaceDefinition} instance. */
export interface SurfaceContext<TSurfaceId extends string = string> {
	readonly surfaceId: TSurfaceId;
	readonly productId: string;
	readonly appId: string;
	readonly modeId: string;
	readonly windowKindId: string;
	readonly runtime: ProductRuntime;
	readonly activeModeId: string | null;
	readonly generation: number;
}
//#endregion 🔖SurfaceContext

//#region 🔖SurfaceSelector
/** @emoji 🧭 Declarative filter for routing contributions to surfaces. */
export interface SurfaceSelector {
	readonly product?: string;
	readonly app?: string;
	readonly mode?: string;
	readonly windowKind?: string;
	readonly surface?: string;
	readonly kind?: string;
	readonly capabilities?: readonly Capability[];
	readonly when?: string;
}

/** @emoji ✅ True when `selector` matches the routing row derived from a surface definition. */
export function matchesSurface(selector: SurfaceSelector, row: SurfaceRoutingRow, resolveWhen: ContextKeyResolver = matchAllContext): boolean {
	if (selector.product && selector.product !== row.productId) return false;
	if (selector.app && selector.app !== row.appId) return false;
	if (selector.mode && selector.mode !== row.modeId) return false;
	if (selector.windowKind && selector.windowKind !== row.windowKindId) return false;
	if (selector.surface && selector.surface !== row.surfaceId) return false;
	if (selector.kind && selector.kind !== row.surfaceKind) return false;
	if (selector.capabilities?.length && !capabilitiesSatisfy(row.capabilities, selector.capabilities)) return false;
	if (!resolveWhen(selector.when)) return false;
	return true;
}

/** @emoji 📇 Flattened surface identity used by {@link SurfaceRouter}. */
export interface SurfaceRoutingRow {
	readonly productId: string;
	readonly appId: string;
	readonly modeId: string;
	readonly windowKindId: string;
	readonly surfaceId: string;
	readonly surfaceKind: string;
	readonly capabilities: readonly Capability[];
	readonly surface: SurfaceDefinition;
}
//#endregion 🔖SurfaceSelector

//#region 🔖ContributionRoute
/** @emoji 🛤 One plugin-authored routing rule: selector + opaque contribution payload. */
export interface ContributionRoute {
	readonly pluginId: string;
	readonly where: SurfaceSelector;
	readonly contribution: unknown;
}
//#endregion 🔖ContributionRoute

//#region 🔖ContributionRegistry
/** @emoji 📚 Collects {@link ContributionRoute} rows before {@link SurfaceRouter} applies them. */
export class ContributionRegistry {
	private readonly routes: ContributionRoute[] = [];

	add(route: ContributionRoute): void {
		this.routes.push(route);
	}

	list(): readonly ContributionRoute[] {
		return this.routes;
	}

	clear(): void {
		this.routes.length = 0;
	}
}
//#endregion 🔖ContributionRegistry

//#region 🔖SurfaceRouter
/** @emoji 🧭 Walks product graph + runtime apps and applies contributions to matching surfaces. */
export class SurfaceRouter {
	static flattenFromProductDefinition(product: ProductDefinition, resolveWhen: ContextKeyResolver = matchAllContext): SurfaceRoutingRow[] {
		const rows: SurfaceRoutingRow[] = [];
		for (const app of product.apps) {
			for (const mode of app.modes) {
				for (const wk of mode.windowKinds) {
					for (const surface of wk.surfaces) {
						const caps = [...new Set([...wk.capabilities, ...surface.capabilities])];
						rows.push({
							productId: product.id,
							appId: app.id,
							modeId: mode.id,
							windowKindId: wk.id,
							surfaceId: surface.id,
							surfaceKind: surface.kind,
							capabilities: caps,
							surface,
						});
					}
				}
			}
		}
		void resolveWhen;
		return rows;
	}

	static flattenFromRuntimeApps(productId: string, apps: readonly AppRuntime[], resolveWhen: ContextKeyResolver = matchAllContext): SurfaceRoutingRow[] {
		const rows: SurfaceRoutingRow[] = [];
		for (const app of apps) {
			const modeId = app.getActiveModeId();
			const resolved = app.resolve(modeId);
			for (const wk of resolved.windowKinds) {
				const implicitWindowSurfaceId = `framework.window:${app.id}:${resolved.activeModeId ?? "default"}:${wk.id}`;
				const implicit: SurfaceDefinition = {
					id: implicitWindowSurfaceId,
					appId: app.id,
					modeId: resolved.activeModeId ?? "default",
					windowKindId: wk.id,
					kind: "window",
					capabilities: [...wk.capabilities],
					createApi: () => ({}),
					applyContribution: () => ({ dispose: () => undefined }),
				};
				rows.push({
					productId,
					appId: app.id,
					modeId: resolved.activeModeId ?? "default",
					windowKindId: wk.id,
					surfaceId: implicit.id,
					surfaceKind: implicit.kind,
					capabilities: [...wk.capabilities],
					surface: implicit,
				});
				for (const surface of wk.surfaces) {
					const caps = [...new Set([...wk.capabilities, ...surface.capabilities])];
					rows.push({
						productId,
						appId: app.id,
						modeId: resolved.activeModeId ?? "default",
						windowKindId: wk.id,
						surfaceId: surface.id,
						surfaceKind: surface.kind,
						capabilities: caps,
						surface,
					});
				}
			}
		}
		void resolveWhen;
		return rows;
	}

	static applyRoutes(
		routes: readonly ContributionRoute[],
		rows: readonly SurfaceRoutingRow[],
		buildContext: (row: SurfaceRoutingRow) => SurfaceContext,
		resolveWhen: ContextKeyResolver = matchAllContext,
	): Disposable {
		const disposables: Disposable[] = [];
		for (const route of routes) {
			for (const row of rows) {
				const selector: SurfaceSelector = { ...route.where, product: route.where.product ?? row.productId };
				if (!matchesSurface(selector, row, resolveWhen)) continue;
				const ctx = buildContext(row);
				const api = row.surface.createApi(ctx);
				disposables.push(row.surface.applyContribution(route.contribution, ctx, api));
			}
		}
		return {
			dispose: () => {
				for (const d of disposables.splice(0)) d.dispose();
			},
		};
	}
}
//#endregion 🔖SurfaceRouter

//#region 🔖PluginContext
/** @emoji 🔌 Disposable returned from {@link PluginContext.subscribe}. */
export interface PluginSubscription {
	dispose(): void;
}

/** @emoji 🧰 Activation context: product runtime, manifest, and registration helpers (VS Code `ExtensionContext` analogue). */
export class PluginContext {
	private readonly disposables: PluginSubscription[] = [];

	constructor(
		readonly runtime: ProductRuntime,
		readonly manifest: PluginManifest,
	) {}

	registerWindowBody(bodyKey: string, build: (ctx: WindowBodyViewContext) => UiNode): void {
		registerWindowBody(bodyKey, build);
		this.disposables.push({
			dispose: () => unregisterWindowBody(bodyKey),
		});
	}

	registerSidePanelBody(bodyKey: string, build: (ctx: SidePanelBodyViewContext) => UiNode): void {
		registerSidePanelBody(bodyKey, build);
		this.disposables.push({
			dispose: () => unregisterSidePanelBody(bodyKey),
		});
	}

	addContributedApps(getController: (controllerId: string) => Controller | undefined): void {
		for (const spec of this.manifest.contributes.apps ?? []) {
			const controller = getController(spec.controllerId);
			if (!controller) continue;
			const windowKinds = spec.windowKinds.map((wk) => {
				const windowKind = new WindowKindRuntime(wk.id, wk.label, wk.bodyKey, wk.iconId, wk.measures);
				if (wk.commands?.length) windowKind.commands = [...wk.commands];
				return windowKind;
			});
			const app = new AppRuntime(spec.id, spec.label, spec.iconId, controller, spec.defaultLayout, windowKinds);
			if (spec.defaultModeId) app.defaultModeId = spec.defaultModeId;
			if (spec.tools) app.tools = spec.tools;
			if (spec.commands?.length) app.commands = [...spec.commands];
			if (spec.leftTabs?.length) app.leftTabs = [...spec.leftTabs];
			if (spec.rightTabs?.length) app.rightTabs = [...spec.rightTabs];
			if (spec.footerItems?.length) app.footerItems = [...spec.footerItems];
			if (spec.findItems?.length) app.findItems = [...spec.findItems];
			for (const modeSpec of spec.modes ?? []) {
				const mode = new ModeRuntime(modeSpec.id, modeSpec.label, modeSpec.iconId);
				if (modeSpec.tools) mode.tools = modeSpec.tools;
				if (modeSpec.commands?.length) mode.commands = [...modeSpec.commands];
				if (modeSpec.windowKinds?.length) {
					mode.windowKinds = modeSpec.windowKinds.map((wk) => {
						const windowKind = new WindowKindRuntime(wk.id, wk.label, wk.bodyKey, wk.iconId, wk.measures);
						if (wk.commands?.length) windowKind.commands = [...wk.commands];
						return windowKind;
					});
				}
				if (modeSpec.defaultLayout) mode.defaultLayout = modeSpec.defaultLayout;
				app.addMode(mode);
			}
			this.runtime.addApp(app);
			this.disposables.push({
				dispose: () => {
					const index = this.runtime.apps.findIndex((entry) => entry.id === spec.id);
					if (index >= 0) this.runtime.apps.splice(index, 1);
				},
			});
		}
	}

	subscribe(listener: ProductSubscriber): PluginSubscription {
		const unsubscribe = this.runtime.subscribe(listener);
		const sub: PluginSubscription = {
			dispose: () => unsubscribe(),
		};
		this.disposables.push(sub);
		return sub;
	}

	disposeAll(): void {
		for (const disposable of this.disposables.splice(0)) disposable.dispose();
	}
}
//#endregion 🔖PluginContext

//#region 🔖PluginManifest
/** @emoji 🧩 Static app contribution merged by {@link PluginHost} before {@link AppRuntime} construction. */
export interface PluginManifestAppContribute {
	readonly id: string;
	readonly label: string;
	readonly iconId?: string;
	readonly controllerId: string;
	readonly windowKinds: readonly {
		readonly id: string;
		readonly label: string;
		readonly bodyKey: string;
		readonly iconId?: string;
		readonly measures?: readonly WindowMeasure[];
		readonly commands?: readonly SearchItemSpec[];
	}[];
	readonly defaultLayout: WindowLayout;
	readonly defaultModeId?: string;
	readonly modes?: readonly {
		readonly id: string;
		readonly label: string;
		readonly iconId?: string;
		readonly tools?: AppTools;
		readonly commands?: readonly SearchItemSpec[];
		readonly windowKinds?: readonly { readonly id: string; readonly label: string; readonly bodyKey: string; readonly iconId?: string; readonly measures?: readonly WindowMeasure[]; readonly commands?: readonly SearchItemSpec[] }[];
		readonly defaultLayout?: WindowLayout;
	}[];
	readonly tools?: AppTools;
	readonly commands?: readonly SearchItemSpec[];
	readonly leftTabs?: readonly SideTabSpec[];
	readonly rightTabs?: readonly SideTabSpec[];
	readonly footerItems?: readonly FooterItem[];
	readonly findItems?: readonly FindItem[];
}

/** @emoji 📦 VS Code–style `contributes` block (serializable); runtime bodies register in {@link PluginModule.activate}. */
export interface PluginManifestContributes {
	readonly apps?: readonly PluginManifestAppContribute[];
	readonly commands?: readonly {
		readonly id: string;
		readonly controllerId: string;
		readonly command: string;
		readonly title?: string;
	}[];
}

/** @emoji 🧾 Extension package descriptor (id + contributes); optional {@link PluginModule}. */
export interface PluginManifest {
	readonly id: string;
	readonly label?: string;
	readonly version?: string;
	readonly target?: { readonly product: string; readonly api: string };
	readonly contributes: PluginManifestContributes;
}

/** @emoji 🧩 Runtime plugin module (`activate` / `deactivate`). */
export interface PluginModule {
	readonly id: string;
	activate(context: PluginContext): void | Promise<void>;
	deactivate?(): void | Promise<void>;
}

/** @emoji 🏗 Loads plugin manifests, activates modules, and owns contributed {@link AppRuntime} rows. */
export class PluginHost {
	private readonly plugins = new Map<string, { manifest: PluginManifest; module?: PluginModule }>();
	private readonly contexts = new Map<string, PluginContext>();
	private activated = false;

	constructor(readonly runtime: ProductRuntime) {}

	register(manifest: PluginManifest, module?: PluginModule): void {
		if (module && module.id !== manifest.id) {
			throw new Error(`Plugin module id "${module.id}" does not match manifest id "${manifest.id}".`);
		}
		this.plugins.set(manifest.id, { manifest, module });
	}

	getControllerById(controllerId: string): Controller | undefined {
		for (const app of this.runtime.apps) {
			if (app.controller.id === controllerId) return app.controller;
		}
		return undefined;
	}

	async activateAll(getController: (controllerId: string) => Controller | undefined): Promise<void> {
		if (this.activated) return;
		this.activated = true;
		for (const { manifest, module } of this.plugins.values()) {
			const context = new PluginContext(this.runtime, manifest);
			this.contexts.set(manifest.id, context);
			context.addContributedApps(getController);
			if (module) await module.activate(context);
		}
	}

	async deactivateAll(): Promise<void> {
		for (const [id, { module }] of [...this.plugins.entries()].reverse()) {
			await module?.deactivate?.();
			this.contexts.get(id)?.disposeAll();
			this.contexts.delete(id);
		}
		this.activated = false;
	}
}
//#endregion 🔖PluginManifest

//#region 🔖ProductPlugin
/** @emoji 🧩 Typed product plugin: manifest target, optional per-surface activation, and selector-based contributions. */
export interface ProductPlugin<TProductApi = unknown, TSurfaceMap extends Record<string, SurfaceBinding<unknown, unknown>> = Record<string, SurfaceBinding<unknown, unknown>>> {
	readonly id: string;
	readonly target: { readonly product: string; readonly api: string };
	readonly manifest?: PluginManifest;
	activate?(ctx: PluginContext, product: TProductApi): void | Promise<void>;
	deactivate?(): void | Promise<void>;
	surfaces?: { [K in keyof TSurfaceMap]?: (ctx: SurfaceContext<K & string>, surface: TSurfaceMap[K]["api"]) => Disposable | Promise<Disposable> };
	contributes?: { readonly selectors?: readonly ContributionRoute[] };
}

/** @emoji ✅ Identity helper for authoring {@link ProductPlugin} definitions. */
export function defineProductPlugin<TProductApi, TSurfaceMap extends Record<string, SurfaceBinding<unknown, unknown>>>(
	plugin: ProductPlugin<TProductApi, TSurfaceMap>,
): ProductPlugin<TProductApi, TSurfaceMap> {
	return plugin;
}
//#endregion 🔖ProductPlugin

//#region 🔖PluginActivationHost
/** @emoji 🎛 Activates {@link ProductPlugin} instances: product lifecycle + surface handlers + routed contributions. */
export class PluginActivationHost<TProductApi = unknown> {
	private readonly disposables: Disposable[] = [];
	private productApi: TProductApi | undefined;

	constructor(
		readonly runtime: ProductRuntime,
		readonly productId: string,
		readonly createApi: (ctx: PluginContext) => TProductApi,
	) {}

	async activateAll(plugins: readonly ProductPlugin<TProductApi>[], getController: (controllerId: string) => Controller | undefined): Promise<void> {
		void getController;
		const bootstrapCtx = new PluginContext(this.runtime, { id: "__product", contributes: {} });
		this.productApi ??= this.createApi(bootstrapCtx);
		const rows = () => SurfaceRouter.flattenFromRuntimeApps(this.productId, this.runtime.apps);
		for (const plugin of plugins) {
			const manifest: PluginManifest = plugin.manifest ?? { id: plugin.id, contributes: {} };
			const ctx = new PluginContext(this.runtime, manifest);
			await plugin.activate?.(ctx, this.productApi!);
			const flat = rows();
			for (const row of flat) {
				const handler = plugin.surfaces?.[row.surfaceId as keyof typeof plugin.surfaces];
				if (!handler) continue;
				const sctx: SurfaceContext = {
					surfaceId: row.surfaceId,
					productId: this.productId,
					appId: row.appId,
					modeId: row.modeId,
					windowKindId: row.windowKindId,
					runtime: this.runtime,
					activeModeId: this.runtime.getActiveApp()?.getActiveModeId() ?? null,
					generation: this.runtime.generation,
				};
				const result = await handler(sctx as SurfaceContext<string>, {} as never);
				if (result && typeof (result as Disposable).dispose === "function") {
					this.disposables.push(result as Disposable);
				}
			}
			const registry = new ContributionRegistry();
			for (const route of plugin.contributes?.selectors ?? []) {
				registry.add({ ...route, pluginId: plugin.id });
			}
			this.disposables.push(
				SurfaceRouter.applyRoutes(registry.list(), flat, (row) => ({
					surfaceId: row.surfaceId,
					productId: this.productId,
					appId: row.appId,
					modeId: row.modeId,
					windowKindId: row.windowKindId,
					runtime: this.runtime,
					activeModeId: this.runtime.getActiveApp()?.getActiveModeId() ?? null,
					generation: this.runtime.generation,
				})),
			);
		}
	}

	disposeAll(): void {
		for (const d of this.disposables.splice(0)) d.dispose();
	}
}
//#endregion 🔖PluginActivationHost

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("canvas-only declarative window bodies", () => {
		it("accepts lone scene3d and board nodes", () => {
			expect(isCanvasOnlyWindowBody(buildScene3dWindowBody("s", "c"))).toBe(true);
			expect(isCanvasOnlyWindowBody(buildBoardWindowBody("b", "c", "pane"))).toBe(true);
			expect(isCanvasOnlyWindowBody({ type: "text", value: "loading" })).toBe(true);
		});

		it("rejects window bodies with toolbar buttons", () => {
			expect(() =>
				assertCanvasOnlyWindowBody("bad", {
					type: "stack",
					direction: "vertical",
					padding: "none",
					children: [
						{
							type: "button",
							label: "Nope",
							command: { controllerId: "c", command: "x" },
						},
						buildScene3dWindowBody("s", "c"),
					],
				}),
			).toThrow(/table, scene3d, or board/);
		});
	});

	describe("PluginHost", () => {
		it("merges contributed apps and declarative window bodies", async () => {
			class TCtrl extends Controller {
				override run(): void {}
			}
			const bus = new CommandBus();
			const rt = new ProductRuntime();
			const ctrl = new TCtrl("ext-ctrl", bus, () => rt.notify());
			const host = new PluginHost(rt);
			host.register(
				{
					id: "demo.ext",
					contributes: {
						apps: [
							{
								id: "demo-app",
								label: "Demo",
								controllerId: "ext-ctrl",
								windowKinds: [{ id: "main", label: "Main", bodyKey: "demo.ext.main" }],
								defaultLayout: createTabStackLayout(["main"], ["Main"]),
							},
						],
					},
				},
				{
					id: "demo.ext",
					activate(ctx) {
						ctx.registerWindowBody("demo.ext.main", () => ({
							type: "text",
							value: "hello",
						}));
					},
				},
			);
			await host.activateAll((id) => (id === "ext-ctrl" ? ctrl : undefined));
			expect(rt.apps.some((app) => app.id === "demo-app")).toBe(true);
			const factory = getWindowBodyFactory("demo.ext.main");
			expect(factory?.({ runtime: rt, windowKindId: "main", bodyKey: "demo.ext.main", activeModeId: null, generation: 0 }).type).toBe("text");
		});
	});

	describe("matchesSurface", () => {
		const row: SurfaceRoutingRow = {
			productId: "p",
			appId: "a",
			modeId: "m",
			windowKindId: "wk",
			surfaceId: "s1",
			surfaceKind: "diagram",
			capabilities: ["design.model.read", "energy.overlay"],
			surface: {
				id: "s1",
				appId: "a",
				modeId: "m",
				windowKindId: "wk",
				kind: "diagram",
				capabilities: ["energy.overlay"],
				createApi: () => ({}),
				applyContribution: () => ({ dispose: () => undefined }),
			},
		};

		it("matches by app/mode/windowKind/surface/kind", () => {
			expect(matchesSurface({ app: "a", mode: "m", windowKind: "wk", surface: "s1", kind: "diagram" }, row)).toBe(true);
			expect(matchesSurface({ app: "other" }, row)).toBe(false);
			expect(matchesSurface({ kind: "scene" }, row)).toBe(false);
		});

		it("matches capabilities as subset", () => {
			expect(matchesSurface({ capabilities: ["energy.overlay"] }, row)).toBe(true);
			expect(matchesSurface({ capabilities: ["energy.overlay", "missing"] }, row)).toBe(false);
		});
	});

	describe("capability-only routing across implicit window surfaces", () => {
		it("routes contributions to every compatible implicit window surface", () => {
			class TCtrl extends Controller {
				override run(): void {}
			}
			const bus = new CommandBus();
			const rt = new ProductRuntime();
			const ctrl = new TCtrl("c", bus, () => rt.notify());
			const wk = new WindowKindRuntime("main", "Main", "demo.body", undefined, [], ["foo.overlay"]);
			rt.addApp(new AppRuntime("app", "App", undefined, ctrl, createTabStackLayout(["main"]), [wk]));
			const flat = SurfaceRouter.flattenFromRuntimeApps("prod", rt.apps);
			let applied = 0;
			const disposable = SurfaceRouter.applyRoutes(
				[{ pluginId: "p1", where: { capabilities: ["foo.overlay"] }, contribution: {} }],
				flat,
				(row) =>
					({
						surfaceId: row.surfaceId,
						productId: "prod",
						appId: row.appId,
						modeId: row.modeId,
						windowKindId: row.windowKindId,
						runtime: rt,
						activeModeId: null,
						generation: 0,
					}) as SurfaceContext,
			);
			for (const r of flat) {
				if (matchesSurface({ capabilities: ["foo.overlay"] }, r)) applied++;
			}
			expect(applied).toBe(1);
			disposable.dispose();
		});
	});

	describe("defineProductPlugin lifecycle", () => {
		it("runs activate once and disposes surface contributions", async () => {
			class TCtrl extends Controller {
				override run(): void {}
			}
			const bus = new CommandBus();
			const rt = new ProductRuntime();
			const ctrl = new TCtrl("c", bus, () => rt.notify());
			rt.addApp(new AppRuntime("app", "App", undefined, ctrl, createTabStackLayout(["w"]), [new WindowKindRuntime("w", "W", "k", undefined, [], ["x"])]));
			let surfaceActivations = 0;
			const plugin = defineProductPlugin({
				id: "pl",
				target: { product: "p", api: "^1" },
				surfaces: {
					[`framework.window:app:default:w`]: async () => {
						surfaceActivations++;
						return { dispose: () => undefined };
					},
				},
			});
			const host = new PluginActivationHost(rt, "p", () => ({}) as Record<string, never>);
			await host.activateAll([plugin], () => ctrl);
			expect(surfaceActivations).toBe(1);
			host.disposeAll();
		});
	});

		describe("resolveCommandPaletteItems", () => {
			it("merges ui, app, mode, and active window kind commands by active scope", () => {
				const runtime = new ProductRuntime();
				runtime.commands = [{ id: "ui", label: "UI", controllerId: "ctrl", command: "ui" }];
				runtime.searchItems = [{ id: "legacy", label: "Legacy", controllerId: "ctrl", command: "legacy" }];
				class TCtrl extends Controller {
					constructor() {
						super("ctrl", runtime.commandBus, () => runtime.notify());
					}
					run(): void {}
				}
				const baseWindow = new WindowKindRuntime("base", "Base", "base.body");
				baseWindow.commands = [{ id: "base-window", label: "Base Window", controllerId: "ctrl", command: "base-window" }];
				const app = new AppRuntime("app", "App", undefined, new TCtrl(), createTabStackLayout(["base"]), [baseWindow]);
				app.commands = [{ id: "app", label: "App", controllerId: "ctrl", command: "app" }];
				const inspect = new ModeRuntime("inspect", "Inspect", undefined);
				inspect.commands = [{ id: "mode", label: "Mode", controllerId: "ctrl", command: "mode" }];
				const inspectWindow = new WindowKindRuntime("inspect", "Inspect", "inspect.body");
				inspectWindow.commands = [{ id: "inspect-window", label: "Inspect Window", controllerId: "ctrl", command: "inspect-window" }];
				inspect.windowKinds = [inspectWindow];
				app.addMode(inspect);
				const resolved = resolveAppState(app, "inspect");

				expect(resolveCommandPaletteItems(runtime, resolved, "inspect").map((item) => item.id)).toEqual(["legacy", "ui", "app", "mode", "inspect-window"]);
				expect(resolveCommandPaletteItems(runtime, resolved, "base").map((item) => item.id)).toEqual(["legacy", "ui", "app", "mode", "base-window"]);
			});
		});
}
//#endregion 🧪Tests
