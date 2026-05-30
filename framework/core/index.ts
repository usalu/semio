// #region 🧱Header
/** 🧱 `@framework/core` — Render-independent shared framework: declarative {@link UiPrimitiveNode}, layout, toolbar, {@link CommandBus}, {@link Platform} shell, and generic body registries. */
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
export interface UiPrimitiveStackNode {
	readonly type: "stack";
	readonly direction: "horizontal" | "vertical";
	readonly gap?: "none" | "tight" | "standard" | "relaxed";
	readonly padding?: "none" | "standard";
	readonly children: readonly UiPrimitiveNode[];
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

/** @emoji 🧩 Primitive declarative nodes shared by every product UI graph. */
export type UiPrimitiveNode = UiPrimitiveStackNode | UiTextNode | UiButtonNode | UiSeparatorNode;
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

export interface WindowMeasureSlider {
	readonly kind: "slider";
	readonly id: string;
	readonly label?: string;
	readonly value: number;
	readonly min: number;
	readonly max: number;
	readonly step?: number;
	readonly onChange: CommandDescriptor;
}

export interface WindowMeasureToggle {
	readonly kind: "toggle";
	readonly id: string;
	readonly label?: string;
	readonly pressed: boolean;
	readonly text?: string;
	readonly onChange: CommandDescriptor;
}

export type WindowMeasure = WindowMeasureSelect | WindowMeasureSlider | WindowMeasureToggle;
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

/** @emoji 🧱 Default row/column of one stack per window kind. */
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
export type AppToolCategory =
	| "history"
	| "hand"
	| "selection"
	| "lasso"
	| "filter"
	| "open"
	| "save"
	| "transform"
	| "create"
	| "view"
	| "actions"
	| "settings";

/** @emoji 📋 Default toolbar category order. */
export const APP_TOOL_CATEGORY_ORDER: readonly AppToolCategory[] = [
	"history",
	"hand",
	"selection",
	"lasso",
	"filter",
	"open",
	"save",
	"transform",
	"create",
	"view",
	"actions",
	"settings",
];

/** @emoji 🎛 Declarative toolbar item; interactions route through {@link CommandBus}. */
export interface ToolItem {
	readonly id: string;
	readonly kind: "button" | "toggle" | "separator";
	readonly iconId?: string;
	readonly label?: string;
	readonly text?: string;
	readonly title?: string;
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
/** @emoji 🔎 Command palette row spec resolved by renderers. */
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

/** @emoji 🔎 Find palette row (label-only; renderer supplies icons). */
export interface FindItem {
	readonly id: string;
	readonly label: string;
	readonly description?: string;
	readonly category?: string;
}
//#endregion 🔖CommandsPalette

//#region 🔖SideTab
/** @emoji 📑 Side panel tab addressing a declarative `bodyKey` tree host. */
export interface SideTabSpec {
	readonly id: string;
	readonly iconId: string;
	readonly order?: number;
	readonly bodyKey: string;
}
//#endregion 🔖SideTab

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
	readonly content?: unknown;
}
//#endregion 🔖Footer

//#region 🔖Merge
/** @emoji 🔀 Merges rows by `id`, favoring the more specific extension. */
export function mergeById<T extends { id: string }>(base: readonly T[] | undefined, extension: readonly T[] | undefined): T[] | undefined {
	if (!base?.length && !extension?.length) return undefined;
	const merged = new Map<string, T>();
	base?.forEach((entry) => merged.set(entry.id, entry));
	extension?.forEach((entry) => merged.set(entry.id, entry));
	return [...merged.values()];
}

/** @emoji 🔀 Merges command palette rows by `id`. */
export function mergeSearchItems(base?: readonly SearchItemSpec[], extension?: readonly SearchItemSpec[]): SearchItemSpec[] | undefined {
	return mergeById(base, extension);
}
//#endregion 🔖Merge

//#region 🔖Observable
/** @emoji 📡 Minimal listener set for host invalidation without external reactive libs. */
export type PlatformSubscriber = () => void;

/** @emoji 📦 Holds a value and notifies subscribers on `set`. */
export class ObservableCell<T> {
	private value: T;
	private readonly listeners = new Set<PlatformSubscriber>();

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

	subscribe(listener: PlatformSubscriber): () => void {
		this.listeners.add(listener);
		return () => this.listeners.delete(listener);
	}
}
//#endregion 🔖Observable

//#region 🔖AppPointerFocus
/** @emoji 🎯 Snapshot of cross-surface selection and hover for one app session. */
export interface AppPointerFocusSnapshot<TKey> {
	readonly selection: readonly TKey[];
	readonly hover: TKey | null;
	readonly hoverSourceId: string | null;
}

/** @emoji 🖱️ Shared selection + hover with per-surface hover ownership (hierarchy, canvas, …). */
export class AppPointerFocusStore<TKey> {
	private selection = new Set<TKey>();
	private hover: TKey | null = null;
	private hoverSourceId: string | null = null;
	readonly cell: ObservableCell<AppPointerFocusSnapshot<TKey>>;

	constructor(initialSelection: readonly TKey[] = []) {
		this.selection = new Set(initialSelection);
		this.cell = new ObservableCell(this.snapshot());
	}

	getSnapshot(): AppPointerFocusSnapshot<TKey> {
		return this.snapshot();
	}

	setSelection(keys: readonly TKey[]): void {
		const next = new Set(keys);
		if (next.size === this.selection.size && keys.every((key) => this.selection.has(key))) {
			return;
		}
		this.selection = next;
		this.publish();
	}

	claimHoverSource(sourceId: string): void {
		if (this.hoverSourceId === sourceId) {
			return;
		}
		this.hoverSourceId = sourceId;
		this.publish();
	}

	setHoverFromSource(sourceId: string, key: TKey | null): void {
		this.hoverSourceId = sourceId;
		if (Object.is(this.hover, key)) {
			this.publish();
			return;
		}
		this.hover = key;
		this.publish();
	}

	clearHoverFromSource(sourceId: string): void {
		if (this.hoverSourceId !== sourceId) {
			return;
		}
		this.hoverSourceId = null;
		if (this.hover === null) {
			return;
		}
		this.hover = null;
		this.publish();
	}

	clearHover(): void {
		if (this.hover === null && this.hoverSourceId === null) {
			return;
		}
		this.hover = null;
		this.hoverSourceId = null;
		this.publish();
	}

	private snapshot(): AppPointerFocusSnapshot<TKey> {
		return {
			selection: [...this.selection],
			hover: this.hover,
			hoverSourceId: this.hoverSourceId,
		};
	}

	private publish(): void {
		this.cell.set(this.snapshot());
	}
}
//#endregion 🔖AppPointerFocus

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

//#region 🔖WindowKindRuntime
/** @emoji 🪟 Declarative window kind; renderers map `bodyKey` to a component tree. */
export class BaseWindowKindRuntime {
	constructor(
		readonly id: string,
		readonly label: string,
		readonly bodyKey: string,
		readonly iconId?: string,
		readonly measures: readonly WindowMeasure[] = [],
	) {}
}
//#endregion 🔖WindowKindRuntime

//#region 🔖ModeRuntime
/** @emoji 🎚 Single app mode: toolbars, window kinds, and side tab specs. */
export class BaseModeRuntime {
	tools: AppTools = {};
	windowKinds: BaseWindowKindRuntime[] = [];
	defaultLayout?: WindowLayout;
	leftTabs: SideTabSpec[] = [];
	rightTabs: SideTabSpec[] = [];
	footerItems: FooterItem[] = [];

	constructor(
		readonly id: string,
		readonly label: string,
		readonly iconId: string | undefined,
	) {}
}
//#endregion 🔖ModeRuntime

//#region 🔖ResolveMode
/** @emoji 🎯 Resolves the active mode for an app given an optional override id. */
export function resolveMode(app: BaseAppRuntime, requestedModeId: string | null | undefined): BaseModeRuntime | null {
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
//#endregion 🔖ResolveMode

//#region 🔖ResolvedState
/** @emoji 📸 Merged view of app + active mode used by product renderers. */
export interface ResolvedAppState {
	readonly id: string;
	readonly activeModeId: string | null;
	readonly label: string;
	readonly iconId: string | undefined;
	readonly tools: AppTools | undefined;
	readonly windowKinds: readonly BaseWindowKindRuntime[];
	readonly defaultLayout: WindowLayout;
	readonly leftTabs: SideTabSpec[];
	readonly rightTabs: SideTabSpec[];
	readonly footerItems: FooterItem[];
}

/** @emoji 🧮 Merges app-level and active-mode overlays into {@link ResolvedAppState}. */
export function resolveBaseAppState(app: BaseAppRuntime, requestedModeId?: string | null): ResolvedAppState {
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
		windowKinds: mergedWindowKinds,
		defaultLayout: mode?.defaultLayout ?? app.defaultLayout,
		leftTabs: mergedLeft,
		rightTabs: mergedRight,
		footerItems: mergeById(app.footerItems, mode?.footerItems) ?? app.footerItems,
	};
}
//#endregion 🔖ResolvedState

//#region 🔖AppRuntime
/** @emoji 🧩 One registered app with modes, layout, and a primary {@link Controller}. */
export class BaseAppRuntime {
	readonly modes: BaseModeRuntime[] = [];
	defaultModeId?: string;
	private activeModeIdOverride: string | null = null;
	windowKinds: BaseWindowKindRuntime[] = [];
	defaultLayout!: WindowLayout;
	tools: AppTools = {};
	leftTabs: SideTabSpec[] = [];
	rightTabs: SideTabSpec[] = [];
	footerItems: FooterItem[] = [];
	readonly controller: Controller;

	constructor(
		readonly id: string,
		readonly label: string,
		readonly iconId: string | undefined,
		controller: Controller,
		layout: WindowLayout,
		windowKinds: readonly BaseWindowKindRuntime[],
	) {
		this.controller = controller;
		this.defaultLayout = layout;
		this.windowKinds = [...windowKinds];
	}

	addMode(mode: BaseModeRuntime): void {
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
		return resolveBaseAppState(this, modeId);
	}
}
//#endregion 🔖AppRuntime

//#region 🔖PlatformSpec
/** @emoji 🧾 Declarative platform bootstrap: metadata + optional default chrome. */
export interface PlatformSpec {
	readonly id: string;
	readonly name: string;
	readonly defaultActiveAppId?: string;
	readonly initialPanelVisibility?: { readonly leftSidePanel: boolean; readonly rightSidePanel: boolean };
	readonly className?: string;
	readonly mobile?: boolean;
	readonly mobileQuery?: string;
	readonly globalTools?: AppTools;
	readonly commands?: readonly SearchItemSpec[];
	readonly searchItems?: readonly SearchItemSpec[];
	readonly globalFooterItems?: readonly FooterItem[];
}
//#endregion 🔖PlatformSpec

//#region 🔖Platform
/** @emoji 🖥️ Root shell: apps, URI chrome, panel toggles, and shared {@link CommandBus}. */
export class Platform {
	readonly commandBus = new CommandBus();
	private readonly listeners = new Set<PlatformSubscriber>();
	readonly apps: BaseAppRuntime[] = [];
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
	readonly id: string;
	readonly name: string;

	constructor(spec?: PlatformSpec) {
		this.id = spec?.id ?? "";
		this.name = spec?.name ?? "";
		if (spec?.initialPanelVisibility) this.initialPanelVisibility = spec.initialPanelVisibility;
		if (spec?.className) this.className = spec.className;
		if (spec?.mobile !== undefined) this.mobile = spec.mobile;
		if (spec?.mobileQuery) this.mobileQuery = spec.mobileQuery;
		if (spec?.globalTools) this.globalTools = spec.globalTools;
		if (spec?.commands?.length) this.commands = [...spec.commands];
		if (spec?.searchItems?.length) this.searchItems = [...spec.searchItems];
		if (spec?.globalFooterItems?.length) this.globalFooterItems = [...spec.globalFooterItems];
		if (spec?.defaultActiveAppId) this.activeAppId = spec.defaultActiveAppId;
	}

	notify(): void {
		this.generation++;
		for (const listener of this.listeners) listener();
	}

	subscribe(listener: PlatformSubscriber): () => void {
		this.listeners.add(listener);
		return () => this.listeners.delete(listener);
	}

	addApp(app: BaseAppRuntime): void {
		this.apps.push(app);
		if (!this.activeAppId) this.activeAppId = app.id;
		this.notify();
	}

	getActiveApp(): BaseAppRuntime | undefined {
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
//#endregion 🔖Platform

//#region 🔖BodyViewContext
/** @emoji 🪟 Shared fields for declarative window and side-panel body builders. */
export interface BodyViewContext {
	readonly windowKindId: string;
	readonly bodyKey: string;
	readonly activeModeId: string | null;
	readonly generation: number;
}

export interface BodyRegistryOptions<TNode> {
	readonly assert?: (bodyKey: string, node: TNode) => void;
	readonly wrap?: (bodyKey: string, build: (ctx: BodyViewContext) => TNode) => (ctx: BodyViewContext) => TNode;
}

/** @emoji 📝 Factory for framework-free declarative body registration maps. */
export function createBodyRegistry<TNode>() {
	const map = new Map<string, (ctx: BodyViewContext) => TNode>();

	return {
		register(bodyKey: string, build: (ctx: BodyViewContext) => TNode, options?: BodyRegistryOptions<TNode>): void {
			const wrapped = options?.wrap ? options.wrap(bodyKey, build) : build;
			map.set(bodyKey, (ctx) => {
				const node = wrapped(ctx);
				options?.assert?.(bodyKey, node);
				return node;
			});
		},
		get(bodyKey: string): ((ctx: BodyViewContext) => TNode) | undefined {
			return map.get(bodyKey);
		},
		unregister(bodyKey: string): void {
			map.delete(bodyKey);
		},
	};
}
//#endregion 🔖BodyViewContext

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("layout factories", () => {
		it("createTabStackLayout builds one stack per window", () => {
			const layout = createTabStackLayout(["a", "b"], ["A", "B"]);
			expect(layout.root.kind).toBe("stack");
			if (layout.root.kind !== "stack") return;
			expect(layout.root.children.map((c) => c.windowKindId)).toEqual(["a", "b"]);
		});

		it("createDefaultLayout builds axis of stacks", () => {
			const layout = createDefaultLayout(["main"], "row", [100], ["Main"]);
			expect(layout.root.kind).toBe("row");
		});
	});

	describe("mergeAppTools", () => {
		it("merges tools per category", () => {
			const base: AppTools = { view: [{ id: "a", kind: "button", label: "A" }] };
			const ext: AppTools = { view: [{ id: "b", kind: "button", label: "B" }] };
			const merged = mergeAppTools(base, ext);
			expect(merged?.view?.length).toBe(2);
		});
	});

	describe("mergeById", () => {
		it("extension overrides base by id", () => {
			const merged = mergeById(
				[{ id: "x", label: "old" }],
				[{ id: "x", label: "new" }],
			);
			expect(merged?.[0].label).toBe("new");
		});
	});

	describe("CommandBus", () => {
		it("dispatches to registered controller", () => {
			class TCtrl extends Controller {
				last: string | undefined;
				constructor(bus: CommandBus) {
					super("c", bus, () => undefined);
				}
				run(command: string): void {
					this.last = command;
				}
			}
			const bus = new CommandBus();
			const ctrl = new TCtrl(bus);
			bus.dispatch("c", "ping");
			expect(ctrl.last).toBe("ping");
			ctrl.dispose();
		});
	});

	describe("ObservableCell", () => {
		it("notifies subscribers on set", () => {
			const cell = new ObservableCell(0);
			let hits = 0;
			cell.subscribe(() => {
				hits++;
			});
			cell.set(1);
			expect(hits).toBe(1);
			cell.set(1);
			expect(hits).toBe(1);
		});
	});

	describe("AppPointerFocusStore", () => {
		it("arbitrates hover by source id", () => {
			const store = new AppPointerFocusStore<string>();
			store.setHoverFromSource("hierarchy", "a");
			expect(store.getSnapshot().hover).toBe("a");
			store.clearHoverFromSource("canvas");
			expect(store.getSnapshot().hover).toBe("a");
			store.clearHoverFromSource("hierarchy");
			expect(store.getSnapshot().hover).toBeNull();
		});

		it("updates selection independently of hover", () => {
			const store = new AppPointerFocusStore<string>(["x"]);
			store.setHoverFromSource("canvas", "y");
			store.setSelection(["z"]);
			expect(store.getSnapshot()).toEqual({ selection: ["z"], hover: "y", hoverSourceId: "canvas" });
		});
	});

	describe("Platform", () => {
		it("constructs from PlatformSpec metadata", () => {
			const platform = new Platform({ id: "demo", name: "Demo", defaultActiveAppId: "home" });
			expect(platform.id).toBe("demo");
			expect(platform.activeAppId).toBe("home");
		});
	});

	describe("resolveBaseAppState", () => {
		it("merges mode window kinds over app window kinds", () => {
			class TCtrl extends Controller {
				run(): void {}
			}
			const bus = new CommandBus();
			const platform = new Platform();
			const ctrl = new TCtrl("c", bus, () => platform.notify());
			const app = new BaseAppRuntime("app", "App", undefined, ctrl, createTabStackLayout(["base"]), [new BaseWindowKindRuntime("base", "Base", "base.body")]);
			const mode = new BaseModeRuntime("inspect", "Inspect", undefined);
			mode.windowKinds = [new BaseWindowKindRuntime("inspect", "Inspect", "inspect.body")];
			app.addMode(mode);
			const resolved = resolveBaseAppState(app, "inspect");
			expect(resolved.windowKinds.map((wk) => wk.id)).toEqual(["base", "inspect"]);
		});
	});
}
//#endregion 🧪Tests
