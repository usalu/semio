// #region 🧲Header
/** 🧱 `@elements/ui-shell` — Framework-free workbench graph: {@link Workbench} → {@link WorkbenchApp} → {@link Mode}; toolbars dispatch via {@link CommandBus}; window bodies addressable by `bodyKey` for the React renderer. */
// #endregion 🧲Header

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

/** @emoji 🗂️ Per-category toolbar maps. */
export type ShellAppTools = Partial<Record<AppToolCategory, readonly ShellToolItem[]>>;

/** @emoji 🔀 Merges toolbar tool maps per category (extension appends within each category). */
export function mergeShellAppTools(base?: ShellAppTools, extension?: ShellAppTools): ShellAppTools | undefined {
	if (!base && !extension) return undefined;
	const merged: ShellAppTools = {};
	for (const category of APP_TOOL_CATEGORY_ORDER) {
		const combined = [...(base?.[category] ?? []), ...(extension?.[category] ?? [])];
		if (combined.length > 0) (merged as Record<string, readonly ShellToolItem[]>)[category] = combined;
	}
	return Object.keys(merged).length > 0 ? merged : undefined;
}

/** @emoji 🧮 Counts toolbar items across populated categories. */
export function countShellAppTools(tools?: ShellAppTools): number {
	if (!tools) return 0;
	return APP_TOOL_CATEGORY_ORDER.reduce((sum, category) => sum + (tools[category]?.length ?? 0), 0);
}

function hasShellAppToolCategoryItems(items: readonly ShellToolItem[] | undefined): boolean {
	return Boolean(items?.some((item) => item.kind !== "separator"));
}

/** @emoji 📂 Lists categories that have at least one non-separator tool. */
export function listPopulatedShellToolCategories(tools?: ShellAppTools): AppToolCategory[] {
	if (!tools) return [];
	return APP_TOOL_CATEGORY_ORDER.filter((category) => hasShellAppToolCategoryItems(tools[category]));
}
//#endregion 🔖Toolbar

//#region 🔖SideTab
/** @emoji 📑 Side panel tab addressing a React-registered `bodyKey` tree host. */
export interface ShellSideTabSpec {
	readonly id: string;
	readonly iconId: string;
	readonly order?: number;
	readonly bodyKey: string;
}
//#endregion 🔖SideTab

//#region 🔖Find
/** @emoji 🔎 Find palette row (label-only; renderer supplies icons). */
export interface ShellFindItem {
	readonly id: string;
	readonly label: string;
	readonly description?: string;
	readonly category?: string;
}
//#endregion 🔖Find

//#region 🔖Footer
/** @emoji 👣 Footer strip item with optional command dispatch. */
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
//#endregion 🔖Footer

//#region 🔖Observable
/** @emoji 📡 Minimal listener set for host invalidation without external reactive libs. */
export type ShellSubscriber = () => void;

/** @emoji 📦 Holds a value and notifies subscribers on `set`. */
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
//#endregion 🔖Observable

//#region 🔖CommandBus
/** @emoji 🚌 Routes toolbar/footer commands to {@link Controller} instances by id. */
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

//#region 🔖WorkbenchWindowKind
/** @emoji 🪟 Declarative window kind; React renderer maps `bodyKey` to a component. */
export class WorkbenchWindowKind {
	constructor(
		readonly id: string,
		readonly label: string,
		readonly bodyKey: string,
		readonly iconId?: string,
	) {}
}
//#endregion 🔖WorkbenchWindowKind

//#region 🔖WorkbenchMode
/** @emoji 🎚 Single app mode: toolbars, window kinds, and side tab specs. */
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
//#endregion 🔖WorkbenchMode

//#region 🔖Merge
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
//#endregion 🔖Merge

//#region 🔖ResolvedState
/** @emoji 📸 Merged view of app + active mode used by the React workbench bridge. */
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

/** @emoji 🧮 Resolves active mode overlays exactly like the retired `resolveAppConfig` merge. */
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
//#endregion 🔖ResolvedState

//#region 🔖WorkbenchApp
/** @emoji 🧩 One registered app with modes, layout, and a primary {@link Controller}. */
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
//#endregion 🔖WorkbenchApp

//#region 🔖Workbench
/** @emoji 🖥️ Root shell: apps, URI chrome, panel toggles, and shared {@link CommandBus}. */
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

/** @emoji 🔎 Command palette row spec resolved in React (icons + `onSelect`). */
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
//#endregion 🔖Workbench

//#region 🔖DeclarativeWindowBody
import type { UiNode } from "./ui-protocol.ts";

/** @emoji 🪟 View context for declarative window bodies: workbench snapshot without DOM or React roots. */
export interface ShellWindowBodyViewContext {
	readonly workbench: Workbench;
	readonly windowKindId: string;
	readonly bodyKey: string;
	readonly activeModeId: string | null;
	readonly generation: number;
}

const declarativeWindowBodyByKey = new Map<string, (ctx: ShellWindowBodyViewContext) => UiNode>();

/** @emoji 📝 Registers a framework-free window body tree for `bodyKey` (host renders DOM). */
export function registerDeclarativeWindowBody(bodyKey: string, build: (ctx: ShellWindowBodyViewContext) => UiNode): void {
	declarativeWindowBodyByKey.set(bodyKey, build);
}

/** @emoji 🔍 Returns the declarative builder registered for `bodyKey`, if any. */
export function getDeclarativeWindowBodyFactory(bodyKey: string): ((ctx: ShellWindowBodyViewContext) => UiNode) | undefined {
	return declarativeWindowBodyByKey.get(bodyKey);
}

/** @emoji 🧹 Removes a declarative window registration (tests / hot reload). */
export function unregisterDeclarativeWindowBody(bodyKey: string): void {
	declarativeWindowBodyByKey.delete(bodyKey);
}
//#endregion 🔖DeclarativeWindowBody

export type {
	JsonPrimitive,
	JsonValue,
	ShellCommandDescriptor,
	ShellStyleSpec,
	UiButtonNode,
	UiNode,
	UiScene3DHostSurfaceNode,
	UiSeparatorNode,
	UiStackNode,
	UiTextNode,
} from "./ui-protocol.ts";
