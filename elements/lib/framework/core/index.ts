// #region ­ƒº▓Header
/** 🧱 `@elements/framework` — Framework-free workbench graph, declarative {@link UiNode} bodies, {@link ShellExtensionHost} (VS Code–style `contributes` + `activate`), and {@link Workbench} → {@link WorkbenchApp} → {@link WorkbenchMode}. */
// #endregion ­ƒº▓Header

//#region ­ƒöûJsonValue
export type JsonPrimitive = string | number | boolean | null;
export type JsonValue = JsonPrimitive | readonly JsonValue[] | { readonly [key: string]: JsonValue };
//#endregion ­ƒöûJsonValue

//#region ­ƒöûCommands
/** @emoji ­ƒÄ» Semantic command routed through the host to {@link CommandBus.dispatch}. */
export interface ShellCommandDescriptor {
	readonly controllerId: string;
	readonly command: string;
	readonly args?: JsonValue;
}
//#endregion ­ƒöûCommands

//#region ­ƒöûStyle
/** @emoji ­ƒÄ¿ Tokenized chrome hints mapped by the renderer. */
export interface ShellStyleSpec {
	readonly variant?: "default" | "subtle" | "danger" | "success";
	readonly size?: "small" | "medium" | "large";
	readonly density?: "compact" | "normal" | "comfortable";
}
//#endregion ­ƒöûStyle

//#region ­ƒöûUiNode
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
	readonly command: ShellCommandDescriptor;
	readonly style?: ShellStyleSpec;
}

export interface UiSeparatorNode {
	readonly type: "separator";
}

/** @emoji ­ƒºè Host-bound 3D surface; renderer maps `surfaceId` to the canvas implementation. */
export interface UiScene3DHostSurfaceNode {
	readonly type: "scene3d";
	readonly surfaceId: string;
	readonly controllerId: string;
}

/** @emoji ­ƒôï Host-bound 2D board canvas; `paneId` selects the window slot. */
export interface UiBoardHostSurfaceNode {
	readonly type: "board";
	readonly surfaceId: string;
	readonly controllerId: string;
	readonly paneId: string;
}

/** @emoji ­ƒôè Host-bound tabular surface; `paneId` disambiguates multiple table slots in one app. */
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

/** @emoji ­ƒºè Canonical fullscreen 3D window body: only the infinite scene canvas. */
export function buildScene3dWindowBody(surfaceId: string, controllerId: string): UiScene3DHostSurfaceNode {
	return { type: "scene3d", surfaceId, controllerId };
}

/** @emoji ­ƒôï Canonical fullscreen 2D window body: only the infinite board canvas. */
export function buildBoardWindowBody(surfaceId: string, controllerId: string, paneId: string): UiBoardHostSurfaceNode {
	return { type: "board", surfaceId, controllerId, paneId };
}

/** @emoji ­ƒôè Canonical table window body: only the host-bound table surface. */
export function buildTableWindowBody(surfaceId: string, controllerId: string, paneId?: string): UiTableHostSurfaceNode {
	return paneId ? { type: "table", surfaceId, controllerId, paneId } : { type: "table", surfaceId, controllerId };
}

/** @emoji Ô£à True when a window body is a lone surface (`table` / `scene3d` / `board`) or a short error `text` node. */
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
		`Declarative window body "${bodyKey}" must be a single table, scene3d, or board surface (optional none padding stack wrapper). Found "${node.type}". Use WorkbenchMode.tools, side tabs, or window measures for chrome.`,
	);
}
//#endregion ­ƒöûUiNode

//#region ­ƒöûShellWindowMeasure
/** @emoji ­ƒôÉ Framework-free window measure; host maps `onChange` to {@link CommandBus.dispatch}. */
export interface ShellWindowMeasureSelect {
	readonly kind: "select";
	readonly id: string;
	readonly label?: string;
	readonly value: string;
	readonly items: readonly { readonly id: string; readonly value: string; readonly label: string }[];
	readonly onChange: ShellCommandDescriptor;
}

export type ShellWindowMeasure = ShellWindowMeasureSelect;
//#endregion ­ƒöûShellWindowMeasure

//#region ­ƒöûLayout
/** @emoji ­ƒ¬ƒ Single window slot in the abstract layout tree. */
export interface WindowLayoutWindowNode {
	readonly kind: "window";
	readonly windowKindId: string;
	readonly title?: string;
}

/** @emoji ­ƒôÜ Tab stack node grouping window slots. */
export interface WindowLayoutStackNode {
	readonly kind: "stack";
	readonly size?: number;
	readonly children: readonly WindowLayoutWindowNode[];
}

/** @emoji Ôåö´©Å Row/column branch in the abstract layout tree. */
export interface WindowLayoutAxisNode {
	readonly kind: "row" | "column";
	readonly size?: number;
	readonly children: readonly (WindowLayoutAxisNode | WindowLayoutStackNode)[];
}

/** @emoji ­ƒº¡ Root layout wrapper owned by an app. */
export interface WindowLayout {
	readonly root: WindowLayoutAxisNode | WindowLayoutStackNode;
}
//#endregion ­ƒöûLayout

//#region ­ƒöûLayoutFactories
/** @emoji ­ƒ¬ƒ Single window slot helper for {@link WindowLayout} trees. */
export function createWindowLayout(windowKindId: string, title?: string): WindowLayoutWindowNode {
	return { kind: "window", windowKindId, ...(title ? { title } : {}) };
}

/** @emoji ­ƒôÜ Stack layout from ordered window kind ids. */
export function createStackLayout(windowKindIds: string[], titles?: string[]): WindowLayout {
	return {
		root: {
			kind: "stack",
			children: windowKindIds.map((windowKindId, index) => createWindowLayout(windowKindId, titles?.[index])),
		},
	};
}

/** @emoji ­ƒº▒ Default row/column of one stack per window kind (Golden-style ownership). */
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

/** @emoji ­ƒôæ Single stack with every window as a tab group. */
export function createTabStackLayout(windowIds: string[], titles?: string[]): WindowLayout {
	return createStackLayout(windowIds, titles);
}
//#endregion ­ƒöûLayoutFactories

//#region ­ƒöûExpertise
/** @emoji ­ƒÄÜ Surface expertise tier for chrome + label resolution. */
export enum Expertise {
	BEGINNER = "beginner",
	NORMAL = "normal",
	EXPERT = "expert",
}
//#endregion ­ƒöûExpertise

//#region ­ƒöûToolbar
/** @emoji ­ƒº░ Toolbar category ids shared by every app registration surface. */
export type AppToolCategory = "history" | "hand" | "selection" | "lasso" | "filter" | "open" | "create" | "view" | "actions" | "settings";

/** @emoji ­ƒôï Default toolbar category order. */
export const APP_TOOL_CATEGORY_ORDER: readonly AppToolCategory[] = ["history", "hand", "selection", "lasso", "filter", "open", "create", "view", "actions", "settings"];

/** @emoji ­ƒÄø Declarative toolbar item; interactions route through {@link CommandBus}. */
export interface ShellToolItem {
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

/** @emoji ­ƒùé´©Å Per-category toolbar maps. */
export type ShellAppTools = Partial<Record<AppToolCategory, readonly ShellToolItem[]>>;

/** @emoji ­ƒöÇ Merges toolbar tool maps per category (extension appends within each category). */
export function mergeShellAppTools(base?: ShellAppTools, extension?: ShellAppTools): ShellAppTools | undefined {
	if (!base && !extension) return undefined;
	const merged: ShellAppTools = {};
	for (const category of APP_TOOL_CATEGORY_ORDER) {
		const combined = [...(base?.[category] ?? []), ...(extension?.[category] ?? [])];
		if (combined.length > 0) (merged as Record<string, readonly ShellToolItem[]>)[category] = combined;
	}
	return Object.keys(merged).length > 0 ? merged : undefined;
}

/** @emoji ­ƒº« Counts toolbar items across populated categories. */
export function countShellAppTools(tools?: ShellAppTools): number {
	if (!tools) return 0;
	return APP_TOOL_CATEGORY_ORDER.reduce((sum, category) => sum + (tools[category]?.length ?? 0), 0);
}

function hasShellAppToolCategoryItems(items: readonly ShellToolItem[] | undefined): boolean {
	return Boolean(items?.some((item) => item.kind !== "separator"));
}

/** @emoji ­ƒôé Lists categories that have at least one non-separator tool. */
export function listPopulatedShellToolCategories(tools?: ShellAppTools): AppToolCategory[] {
	if (!tools) return [];
	return APP_TOOL_CATEGORY_ORDER.filter((category) => hasShellAppToolCategoryItems(tools[category]));
}
//#endregion ­ƒöûToolbar

//#region ­ƒöûSideTab
/** @emoji ­ƒôæ Side panel tab addressing a React-registered `bodyKey` tree host. */
export interface ShellSideTabSpec {
	readonly id: string;
	readonly iconId: string;
	readonly order?: number;
	readonly bodyKey: string;
}
//#endregion ­ƒöûSideTab

//#region ­ƒöûFind
/** @emoji ­ƒöÄ Find palette row (label-only; renderer supplies icons). */
export interface ShellFindItem {
	readonly id: string;
	readonly label: string;
	readonly description?: string;
	readonly category?: string;
}
//#endregion ­ƒöûFind

//#region ­ƒöûFooter
/** @emoji ­ƒæú Footer strip item with optional command dispatch. */
export interface ShellFooterItem {
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
//#endregion ­ƒöûFooter

//#region ­ƒöûObservable
/** @emoji ­ƒôí Minimal listener set for host invalidation without external reactive libs. */
export type ShellSubscriber = () => void;

/** @emoji ­ƒôª Holds a value and notifies subscribers on `set`. */
export class ObservableCell<T> {
	private value: T;
	private readonly listeners = new Set<ShellSubscriber>();

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

	subscribe(listener: ShellSubscriber): () => void {
		this.listeners.add(listener);
		return () => this.listeners.delete(listener);
	}
}
//#endregion ­ƒöûObservable

//#region ­ƒöûCommandBus
/** @emoji ­ƒÜî Routes toolbar/footer commands to {@link Controller} instances by id. */
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

/** @emoji ­ƒÄ« Base class for imperative app controllers participating in {@link CommandBus}. */
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
//#endregion ­ƒöûCommandBus

//#region ­ƒöûWorkbenchWindowKind
/** @emoji ­ƒ¬ƒ Declarative window kind; React renderer maps `bodyKey` to a component. */
export class WorkbenchWindowKind {
	constructor(
		readonly id: string,
		readonly label: string,
		readonly bodyKey: string,
		readonly iconId?: string,
		readonly measures: readonly ShellWindowMeasure[] = [],
	) {}
}
//#endregion ­ƒöûWorkbenchWindowKind

//#region ­ƒöûWorkbenchMode
/** @emoji ­ƒÄÜ Single app mode: toolbars, window kinds, and side tab specs. */
export class WorkbenchMode {
	tools: ShellAppTools = {};
	windowKinds: WorkbenchWindowKind[] = [];
	defaultLayout?: WindowLayout;
	leftTabs: ShellSideTabSpec[] = [];
	rightTabs: ShellSideTabSpec[] = [];
	footerItems: ShellFooterItem[] = [];
	findItems: ShellFindItem[] = [];
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
//#endregion ­ƒöûWorkbenchMode

//#region ­ƒöûMerge
function mergeById<T extends { id: string }>(base: readonly T[] | undefined, extension: readonly T[] | undefined): T[] | undefined {
	if (!base?.length && !extension?.length) return undefined;
	const merged = new Map<string, T>();
	base?.forEach((entry) => merged.set(entry.id, entry));
	extension?.forEach((entry) => merged.set(entry.id, entry));
	return [...merged.values()];
}

function resolveMode(app: WorkbenchApp, requestedModeId: string | null | undefined): WorkbenchMode | null {
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
//#endregion ­ƒöûMerge

//#region ­ƒöûResolvedState
/** @emoji ­ƒô© Merged view of app + active mode used by the React workbench bridge. */
export interface ResolvedWorkbenchAppState {
	readonly id: string;
	readonly activeModeId: string | null;
	readonly label: string;
	readonly iconId: string | undefined;
	readonly tools: ShellAppTools | undefined;
	readonly windowKinds: readonly WorkbenchWindowKind[];
	readonly defaultLayout: WindowLayout;
	readonly leftTabs: ShellSideTabSpec[];
	readonly rightTabs: ShellSideTabSpec[];
	readonly footerItems: ShellFooterItem[];
	readonly findItems: ShellFindItem[];
	readonly onFindSelect?: (itemId: string) => void;
	readonly onActiveWindowChange?: (windowKindId: string) => void;
	readonly selection: Record<string, unknown>;
	readonly hover: Record<string, unknown>;
	readonly options: Record<string, unknown>;
}

/** @emoji ­ƒº« Resolves active mode overlays exactly like the retired `resolveAppConfig` merge. */
export function resolveWorkbenchAppState(app: WorkbenchApp, requestedModeId?: string | null): ResolvedWorkbenchAppState {
	const mode = resolveMode(app, requestedModeId);
	const mergedWindowKinds = mergeById(app.windowKinds, mode?.windowKinds) ?? app.windowKinds;
	const mergedLeft = mergeById(app.leftTabs, mode?.leftTabs) ?? app.leftTabs;
	const mergedRight = mergeById(app.rightTabs, mode?.rightTabs) ?? app.rightTabs;
	return {
		id: app.id,
		activeModeId: mode?.id ?? null,
		label: mode?.label ?? app.label,
		iconId: mode?.iconId ?? app.iconId,
		tools: mergeShellAppTools(app.tools, mode?.tools),
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
//#endregion ­ƒöûResolvedState

//#region ­ƒöûWorkbenchApp
/** @emoji ­ƒº® One registered app with modes, layout, and a primary {@link Controller}. */
export class WorkbenchApp {
	readonly modes: WorkbenchMode[] = [];
	defaultModeId?: string;
	private activeModeIdOverride: string | null = null;
	windowKinds: WorkbenchWindowKind[] = [];
	defaultLayout!: WindowLayout;
	tools: ShellAppTools = {};
	leftTabs: ShellSideTabSpec[] = [];
	rightTabs: ShellSideTabSpec[] = [];
	footerItems: ShellFooterItem[] = [];
	findItems: ShellFindItem[] = [];
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
		windowKinds: readonly WorkbenchWindowKind[],
	) {
		this.controller = controller;
		this.defaultLayout = layout;
		this.windowKinds = [...windowKinds];
	}

	addMode(mode: WorkbenchMode): void {
		this.modes.push(mode);
	}

	getActiveModeId(): string | null {
		if (this.activeModeIdOverride) return this.activeModeIdOverride;
		return resolveMode(this, null)?.id ?? null;
	}

	setActiveModeId(modeId: string | null): void {
		this.activeModeIdOverride = modeId;
	}

	resolve(requestedModeId?: string | null): ResolvedWorkbenchAppState {
		const modeId = requestedModeId ?? this.getActiveModeId();
		return resolveWorkbenchAppState(this, modeId);
	}
}
//#endregion ­ƒöûWorkbenchApp

//#region ­ƒöûWorkbench
/** @emoji ­ƒûÑ´©Å Root shell: apps, URI chrome, panel toggles, and shared {@link CommandBus}. */
export class Workbench {
	readonly commandBus = new CommandBus();
	private readonly listeners = new Set<ShellSubscriber>();
	readonly apps: WorkbenchApp[] = [];
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
	globalTools: ShellAppTools | undefined;
	globalFooterItems: ShellFooterItem[] = [];
	searchItems: readonly ShellSearchItemSpec[] = [];
	mobile: boolean | undefined;
	mobileQuery = "(max-width: 767px)";
	className = "";
	panelVisibility = { leftSidePanel: false, rightSidePanel: false };
	initialPanelVisibility?: { leftSidePanel: boolean; rightSidePanel: boolean };

	notify(): void {
		this.generation++;
		for (const listener of this.listeners) listener();
	}

	subscribe(listener: ShellSubscriber): () => void {
		this.listeners.add(listener);
		return () => this.listeners.delete(listener);
	}

	addApp(app: WorkbenchApp): void {
		this.apps.push(app);
		if (!this.activeAppId) this.activeAppId = app.id;
		this.notify();
	}

	getActiveApp(): WorkbenchApp | undefined {
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

/** @emoji ­ƒöÄ Command palette row spec resolved in React (icons + `onSelect`). */
export interface ShellSearchItemSpec {
	readonly id: string;
	readonly label: string;
	readonly description?: string;
	readonly category?: string;
	readonly iconId?: string;
	readonly controllerId: string;
	readonly command: string;
	readonly args?: unknown;
}
//#endregion ­ƒöûWorkbench

//#region ­ƒöûDeclarativeWindowBody
/** @emoji ­ƒ¬ƒ View context for declarative window bodies: workbench snapshot without DOM or React roots. */
export interface ShellWindowBodyViewContext {
	readonly workbench: Workbench;
	readonly windowKindId: string;
	readonly bodyKey: string;
	readonly activeModeId: string | null;
	readonly generation: number;
}

const declarativeWindowBodyByKey = new Map<string, (ctx: ShellWindowBodyViewContext) => UiNode>();

/** @emoji ­ƒôØ Registers a framework-free window body tree for `bodyKey` (host renders DOM). */
export function registerDeclarativeWindowBody(bodyKey: string, build: (ctx: ShellWindowBodyViewContext) => UiNode): void {
	declarativeWindowBodyByKey.set(bodyKey, (ctx) => {
		const node = build(ctx);
		assertCanvasOnlyWindowBody(bodyKey, node);
		return node;
	});
}

/** @emoji ­ƒöì Returns the declarative builder registered for `bodyKey`, if any. */
export function getDeclarativeWindowBodyFactory(bodyKey: string): ((ctx: ShellWindowBodyViewContext) => UiNode) | undefined {
	return declarativeWindowBodyByKey.get(bodyKey);
}

/** @emoji ­ƒº╣ Removes a declarative window registration (tests / hot reload). */
export function unregisterDeclarativeWindowBody(bodyKey: string): void {
	declarativeWindowBodyByKey.delete(bodyKey);
}
//#endregion ­ƒöûDeclarativeWindowBody

//#region ­ƒöûDeclarativeSidePanelBody
/** @emoji ­ƒôæ View context for declarative side-panel tab bodies (same snapshot fields as window bodies). */
export type ShellSidePanelBodyViewContext = ShellWindowBodyViewContext;

const declarativeSidePanelBodyByKey = new Map<string, (ctx: ShellSidePanelBodyViewContext) => UiNode>();

/** @emoji ­ƒôØ Registers a framework-free side-panel tree for `bodyKey`. */
export function registerDeclarativeSidePanelBody(bodyKey: string, build: (ctx: ShellSidePanelBodyViewContext) => UiNode): void {
	declarativeSidePanelBodyByKey.set(bodyKey, build);
}

/** @emoji ­ƒöì Returns the declarative side-panel builder for `bodyKey`, if any. */
export function getDeclarativeSidePanelBodyFactory(bodyKey: string): ((ctx: ShellSidePanelBodyViewContext) => UiNode) | undefined {
	return declarativeSidePanelBodyByKey.get(bodyKey);
}

/** @emoji ­ƒº╣ Removes a declarative side-panel registration (tests). */
export function unregisterDeclarativeSidePanelBody(bodyKey: string): void {
	declarativeSidePanelBodyByKey.delete(bodyKey);
}
//#endregion ­ƒöûDeclarativeSidePanelBody

//#region ­ƒöûExtensionContributes
/** @emoji ­ƒº® Static app contribution merged by {@link ShellExtensionHost} before {@link WorkbenchApp} construction. */
export interface ShellExtensionAppContribute {
	readonly id: string;
	readonly label: string;
	readonly iconId?: string;
	readonly controllerId: string;
	readonly windowKinds: readonly {
		readonly id: string;
		readonly label: string;
		readonly bodyKey: string;
		readonly iconId?: string;
		readonly measures?: readonly ShellWindowMeasure[];
	}[];
	readonly defaultLayout: WindowLayout;
	readonly defaultModeId?: string;
	readonly modes?: readonly {
		readonly id: string;
		readonly label: string;
		readonly iconId?: string;
		readonly tools?: ShellAppTools;
		readonly windowKinds?: readonly WorkbenchWindowKind[];
		readonly defaultLayout?: WindowLayout;
	}[];
	readonly tools?: ShellAppTools;
	readonly leftTabs?: readonly ShellSideTabSpec[];
	readonly rightTabs?: readonly ShellSideTabSpec[];
	readonly footerItems?: readonly ShellFooterItem[];
	readonly findItems?: readonly ShellFindItem[];
}

/** @emoji ­ƒôª VS CodeÔÇôstyle `contributes` block (serializable); runtime bodies register in {@link ShellExtension.activate}. */
export interface ShellExtensionContributes {
	readonly apps?: readonly ShellExtensionAppContribute[];
	readonly commands?: readonly {
		readonly id: string;
		readonly controllerId: string;
		readonly command: string;
		readonly title?: string;
	}[];
}

/** @emoji ­ƒº¥ Extension package descriptor (id + contributes); optional {@link ShellExtension} module. */
export interface ShellExtensionManifest {
	readonly id: string;
	readonly label?: string;
	readonly version?: string;
	readonly contributes: ShellExtensionContributes;
}

/** @emoji ­ƒöî Disposable returned from {@link ShellExtensionContext.subscribe}. */
export interface ShellExtensionSubscription {
	dispose(): void;
}

/** @emoji ­ƒº░ Activation context: workbench, manifest, and registration helpers (VS Code `ExtensionContext` analogue). */
export class ShellExtensionContext {
	private readonly disposables: ShellExtensionSubscription[] = [];

	constructor(
		readonly workbench: Workbench,
		readonly manifest: ShellExtensionManifest,
	) {}

	/** @emoji ­ƒôØ Registers a declarative window body scoped to this extension activation. */
	registerDeclarativeWindowBody(bodyKey: string, build: (ctx: ShellWindowBodyViewContext) => UiNode): void {
		registerDeclarativeWindowBody(bodyKey, build);
		this.disposables.push({
			dispose: () => unregisterDeclarativeWindowBody(bodyKey),
		});
	}

	/** @emoji ­ƒôØ Registers a declarative side-panel body scoped to this extension activation. */
	registerDeclarativeSidePanelBody(bodyKey: string, build: (ctx: ShellSidePanelBodyViewContext) => UiNode): void {
		registerDeclarativeSidePanelBody(bodyKey, build);
		this.disposables.push({
			dispose: () => unregisterDeclarativeSidePanelBody(bodyKey),
		});
	}

	/** @emoji Ô×ò Adds a {@link WorkbenchApp} built from static {@link ShellExtensionAppContribute} rows. */
	addContributedApps(getController: (controllerId: string) => Controller | undefined): void {
		for (const spec of this.manifest.contributes.apps ?? []) {
			const controller = getController(spec.controllerId);
			if (!controller) continue;
			const windowKinds = spec.windowKinds.map((wk) => new WorkbenchWindowKind(wk.id, wk.label, wk.bodyKey, wk.iconId, wk.measures));
			const app = new WorkbenchApp(spec.id, spec.label, spec.iconId, controller, spec.defaultLayout, windowKinds);
			if (spec.defaultModeId) app.defaultModeId = spec.defaultModeId;
			if (spec.tools) app.tools = spec.tools;
			if (spec.leftTabs?.length) app.leftTabs = [...spec.leftTabs];
			if (spec.rightTabs?.length) app.rightTabs = [...spec.rightTabs];
			if (spec.footerItems?.length) app.footerItems = [...spec.footerItems];
			if (spec.findItems?.length) app.findItems = [...spec.findItems];
			for (const modeSpec of spec.modes ?? []) {
				const mode = new WorkbenchMode(modeSpec.id, modeSpec.label, modeSpec.iconId);
				if (modeSpec.tools) mode.tools = modeSpec.tools;
				if (modeSpec.windowKinds?.length) mode.windowKinds = [...modeSpec.windowKinds];
				if (modeSpec.defaultLayout) mode.defaultLayout = modeSpec.defaultLayout;
				app.addMode(mode);
			}
			this.workbench.addApp(app);
			this.disposables.push({
				dispose: () => {
					const index = this.workbench.apps.findIndex((entry) => entry.id === spec.id);
					if (index >= 0) this.workbench.apps.splice(index, 1);
				},
			});
		}
	}

	subscribe(listener: ShellSubscriber): ShellExtensionSubscription {
		const unsubscribe = this.workbench.subscribe(listener);
		const sub: ShellExtensionSubscription = {
			dispose: () => unsubscribe(),
		};
		this.disposables.push(sub);
		return sub;
	}

	disposeAll(): void {
		for (const disposable of this.disposables.splice(0)) disposable.dispose();
	}
}

/** @emoji ­ƒº® Runtime extension module (`activate` / `deactivate`). */
export interface ShellExtension {
	readonly id: string;
	activate(context: ShellExtensionContext): void | Promise<void>;
	deactivate?(): void | Promise<void>;
}

/** @emoji ­ƒÅù´©Å Loads extension manifests, activates modules, and owns contributed {@link WorkbenchApp} rows. */
export class ShellExtensionHost {
	private readonly extensions = new Map<string, { manifest: ShellExtensionManifest; module?: ShellExtension }>();
	private readonly contexts = new Map<string, ShellExtensionContext>();
	private activated = false;

	constructor(readonly workbench: Workbench) {}

	register(manifest: ShellExtensionManifest, module?: ShellExtension): void {
		if (module && module.id !== manifest.id) {
			throw new Error(`Extension module id "${module.id}" does not match manifest id "${manifest.id}".`);
		}
		this.extensions.set(manifest.id, { manifest, module });
	}

	getControllerById(controllerId: string): Controller | undefined {
		for (const app of this.workbench.apps) {
			if (app.controller.id === controllerId) return app.controller;
		}
		return undefined;
	}

	async activateAll(getController: (controllerId: string) => Controller | undefined): Promise<void> {
		if (this.activated) return;
		this.activated = true;
		for (const { manifest, module } of this.extensions.values()) {
			const context = new ShellExtensionContext(this.workbench, manifest);
			this.contexts.set(manifest.id, context);
			context.addContributedApps(getController);
			if (module) await module.activate(context);
		}
	}

	async deactivateAll(): Promise<void> {
		for (const [id, { module }] of [...this.extensions.entries()].reverse()) {
			await module?.deactivate?.();
			this.contexts.get(id)?.disposeAll();
			this.contexts.delete(id);
		}
		this.activated = false;
	}
}
//#endregion ­ƒöûExtensionContributes

//#region ­ƒº¬Tests
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

	describe("ShellExtensionHost", () => {
		it("merges contributed apps and declarative window bodies", async () => {
			class TCtrl extends Controller {
				override run(): void {}
			}
			const bus = new CommandBus();
			const wb = new Workbench();
			const ctrl = new TCtrl("ext-ctrl", bus, () => wb.notify());
			const host = new ShellExtensionHost(wb);
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
						ctx.registerDeclarativeWindowBody("demo.ext.main", () => ({
							type: "text",
							value: "hello",
						}));
					},
				},
			);
			await host.activateAll((id) => (id === "ext-ctrl" ? ctrl : undefined));
			expect(wb.apps.some((app) => app.id === "demo-app")).toBe(true);
			const factory = getDeclarativeWindowBodyFactory("demo.ext.main");
			expect(factory?.({ workbench: wb, windowKindId: "main", bodyKey: "demo.ext.main", activeModeId: null, generation: 0 }).type).toBe("text");
		});
	});
}
//#endregion ­ƒº¬Tests
