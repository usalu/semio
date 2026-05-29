// #region 🧲Header
/** @emoji 🛝 `@framework/playground` — React-neutral playground runtime, one-app shell (selection + filter toolbars, workbench + details), declarative {@link UiNode} bodies, command routing (no DOM). */
// #endregion 🧲Header

//#region 🔖JsonValue
export type JsonPrimitive = string | number | boolean | null;
export type JsonValue = JsonPrimitive | readonly JsonValue[] | { readonly [key: string]: JsonValue };
//#endregion 🔖JsonValue

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

export type UiNode =
	| UiStackNode
	| UiTextNode
	| UiButtonNode
	| UiSeparatorNode
	| UiScene3DHostSurfaceNode
	| UiBoardHostSurfaceNode
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

/** @emoji 🧊 Canonical fullscreen 3D window body: only the infinite scene canvas. */
export function buildScene3dWindowBody(surfaceId: string, controllerId: string): UiScene3DHostSurfaceNode {
	return { type: "scene3d", surfaceId, controllerId };
}

/** @emoji 📋 Canonical fullscreen 2D window body: only the infinite board canvas. */
export function buildBoardWindowBody(surfaceId: string, controllerId: string, paneId: string): UiBoardHostSurfaceNode {
	return { type: "board", surfaceId, controllerId, paneId };
}

/** @emoji ✅ True when a window body is a lone surface or a short error `text` node. */
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
		`Declarative window body "${bodyKey}" must be a single table, scene3d, or board surface (optional none padding stack wrapper). Found "${node.type}".`,
	);
}
//#endregion 🔖UiNode

//#region 🔖WindowMeasure
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
export interface WindowLayoutWindowNode {
	readonly kind: "window";
	readonly windowKindId: string;
	readonly title?: string;
}

export interface WindowLayoutStackNode {
	readonly kind: "stack";
	readonly size?: number;
	readonly children: readonly WindowLayoutWindowNode[];
}

export interface WindowLayoutAxisNode {
	readonly kind: "row" | "column";
	readonly size?: number;
	readonly children: readonly (WindowLayoutAxisNode | WindowLayoutStackNode)[];
}

export interface WindowLayout {
	readonly root: WindowLayoutAxisNode | WindowLayoutStackNode;
}

export function createWindowLayout(windowKindId: string, title?: string): WindowLayoutWindowNode {
	return { kind: "window", windowKindId, ...(title ? { title } : {}) };
}

export function createStackLayout(windowKindIds: string[], titles?: string[]): WindowLayout {
	return {
		root: {
			kind: "stack",
			children: windowKindIds.map((windowKindId, index) => createWindowLayout(windowKindId, titles?.[index])),
		},
	};
}

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
//#endregion 🔖Layout

//#region 🔖Expertise
/** @emoji 🎚 Surface expertise tier for chrome + label resolution. */
export enum Expertise {
	BEGINNER = "beginner",
	NORMAL = "normal",
	EXPERT = "expert",
}
//#endregion 🔖Expertise

//#region 🔖Toolbar
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

export type AppTools = Partial<Record<AppToolCategory, readonly ToolItem[]>>;

export function mergeAppTools(base?: AppTools, extension?: AppTools): AppTools | undefined {
	if (!base && !extension) return undefined;
	const merged: AppTools = {};
	for (const category of APP_TOOL_CATEGORY_ORDER) {
		const combined = [...(base?.[category] ?? []), ...(extension?.[category] ?? [])];
		if (combined.length > 0) (merged as Record<string, readonly ToolItem[]>)[category] = combined;
	}
	return Object.keys(merged).length > 0 ? merged : undefined;
}

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

//#region 🔖Observable
export type PlaygroundSubscriber = () => void;

export class ObservableCell<T> {
	private value: T;
	private readonly listeners = new Set<PlaygroundSubscriber>();

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

	subscribe(listener: PlaygroundSubscriber): () => void {
		this.listeners.add(listener);
		return () => this.listeners.delete(listener);
	}
}
//#endregion 🔖Observable

//#region 🔖CommandBus
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
export class WindowKindRuntime {
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
export class ModeRuntime {
	tools: AppTools = {};
	windowKinds: WindowKindRuntime[] = [];
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
export interface ResolvedAppState {
	readonly id: string;
	readonly activeModeId: string | null;
	readonly label: string;
	readonly iconId: string | undefined;
	readonly tools: AppTools | undefined;
	readonly windowKinds: readonly WindowKindRuntime[];
	readonly defaultLayout: WindowLayout;
	readonly leftTabs: SideTabSpec[];
	readonly rightTabs: SideTabSpec[];
	readonly footerItems: FooterItem[];
}

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
		windowKinds: mergedWindowKinds,
		defaultLayout: mode?.defaultLayout ?? app.defaultLayout,
		leftTabs: mergedLeft,
		rightTabs: mergedRight,
		footerItems: mergeById(app.footerItems, mode?.footerItems) ?? app.footerItems,
	};
}
//#endregion 🔖ResolvedState

//#region 🔖AppRuntime
export class AppRuntime {
	readonly modes: ModeRuntime[] = [];
	defaultModeId?: string;
	private activeModeIdOverride: string | null = null;
	windowKinds: WindowKindRuntime[] = [];
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
export class ProductRuntime {
	readonly commandBus = new CommandBus();
	private readonly listeners = new Set<PlaygroundSubscriber>();
	readonly apps: AppRuntime[] = [];
	activeAppId = "";
	generation = 0;
	mobile: boolean | undefined;
	className = "";
	panelVisibility = { leftSidePanel: false, rightSidePanel: false };

	notify(): void {
		this.generation++;
		for (const listener of this.listeners) listener();
	}

	subscribe(listener: PlaygroundSubscriber): () => void {
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
}
//#endregion 🔖ProductRuntime

//#region 🔖WindowBodyViewContext
export interface WindowBodyViewContext {
	readonly runtime: ProductRuntime;
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

const sidePanelBodyByKey = new Map<string, (ctx: SidePanelBodyViewContext) => UiNode>();

export function registerSidePanelBody(bodyKey: string, build: (ctx: SidePanelBodyViewContext) => UiNode): void {
	sidePanelBodyByKey.set(bodyKey, build);
}

export function getSidePanelBodyFactory(bodyKey: string): ((ctx: SidePanelBodyViewContext) => UiNode) | undefined {
	return sidePanelBodyByKey.get(bodyKey);
}

export function unregisterSidePanelBody(bodyKey: string): void {
	sidePanelBodyByKey.delete(bodyKey);
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
	private runtimeMemo: ProductRuntime | null = null;

	/** @emoji 🚀 Lazily built {@link ProductRuntime} from {@link createRuntime}. */
	get runtime(): ProductRuntime {
		this.runtimeMemo ??= this.createRuntime();
		return this.runtimeMemo;
	}

	abstract createRuntime(): ProductRuntime;
	abstract registerBodies(): void;

	readonly initialPanelVisibility?: PlaygroundPanelVisibility;
	readonly keybindings?: readonly PlaygroundKeybinding[];

	/** @emoji 🧊 Override to register canvas surface hosts (library React adapters). */
	registerSurfaceHosts(): void {}
}

export const PLAYGROUND_LS_THEME = "elements.playground.surface.theme";
export const PLAYGROUND_LS_DEVICE = "elements.playground.surface.device";
export const PLAYGROUND_LS_EXPERTISE = "elements.playground.surface.expertise";

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
	readonly mainSceneSurfaceId: string;
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
export function buildPlaygroundKindToggleTools<K extends string>(
	prefix: "selection" | "filter",
	kinds: readonly K[],
	labels: (kind: K) => string,
	values: Readonly<Record<K, boolean>>,
	controllerId: string,
	command: string,
): ToolItem[] {
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
export function buildPlaygroundBrowseSelectionTools<K extends string>(
	kinds: readonly K[],
	labels: (kind: K) => string,
	selectableKinds: Readonly<Record<K, boolean>>,
	controllerId: string,
): ToolItem[] {
	const toggles = buildPlaygroundKindToggleTools("selection", kinds, labels, selectableKinds, controllerId, "toggleSelectableKind");
	return [...toggles, { id: "playground.selection.separator", kind: "separator", order: kinds.length }, buildPlaygroundClearSelectionTool(controllerId, kinds.length + 1)];
}

/** @emoji 👁️ Standard playground browse filter tools (visibility kind toggles). */
export function buildPlaygroundBrowseFilterTools<K extends string>(
	kinds: readonly K[],
	labels: (kind: K) => string,
	visibleKinds: Readonly<Record<K, boolean>>,
	controllerId: string,
): ToolItem[] {
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

	protected constructor(
		controllerId: string,
		spec: PlaygroundKindSpec<K>,
		commandBus: CommandBus,
		hostNotify: () => void,
	) {
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
export function buildPlaygroundWorkbenchApp(
	ids: PlaygroundIds,
	controller: PlaygroundController<string>,
	options?: BuildPlaygroundWorkbenchAppOptions,
): AppRuntime {
	const layout =
		options?.layout ??
		createDefaultLayout([ids.windowId], "row", [100], [ids.windowLabel]);
	const app = new AppRuntime(
		ids.appId,
		ids.windowLabel,
		undefined,
		controller,
		layout,
		[new WindowKindRuntime(ids.windowId, ids.windowLabel, ids.mainBodyKey)],
	);
	app.defaultModeId = controller.browseMode.id;
	app.addMode(controller.browseMode);
	app.leftTabs = [{ id: `${ids.appId}.workbench`, iconId: ids.workbenchIconId, order: 0, bodyKey: ids.workbenchTabBodyKey }];
	app.rightTabs = [{ id: `${ids.appId}.details`, iconId: ids.detailsIconId, order: 0, bodyKey: ids.detailsTabBodyKey }];
	controller.commandBus.dispatch(controller.id, "setQuery", { query: options?.initialQuery ?? "" });
	return app;
}

export function playgroundControllerFromContext(
	ctx: WindowBodyViewContext | SidePanelBodyViewContext,
): PlaygroundController<string> | undefined {
	return ctx.runtime.getActiveApp()?.controller as PlaygroundController<string> | undefined;
}

/** @emoji 🪟 Declarative main window: lone scene3d surface. */
export function buildPlaygroundMainWindowBody(ids: PlaygroundIds, ctx: WindowBodyViewContext): UiNode {
	if (!playgroundControllerFromContext(ctx)) {
		return { type: "text", value: "Missing playground controller" };
	}
	return buildScene3dWindowBody(ids.mainSceneSurfaceId, ids.controllerId);
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
	registerSidePanelBody(
		ids.workbenchTabBodyKey,
		options?.buildWorkbenchPanel ?? ((ctx) => buildPlaygroundWorkbenchPanelBody(ids, ctx)),
	);
	registerSidePanelBody(
		ids.detailsTabBodyKey,
		options?.buildDetailsPanel ?? ((ctx) => buildPlaygroundDetailsPanelBody(ids, ctx)),
	);
}

/** @emoji 🚀 Creates a {@link ProductRuntime} with one playground app. */
export function createPlaygroundWorkbench(
	ids: PlaygroundIds,
	controller: PlaygroundController<string>,
	options?: BuildPlaygroundWorkbenchAppOptions,
): ProductRuntime {
	const runtime = new ProductRuntime();
	runtime.addApp(buildPlaygroundWorkbenchApp(ids, controller, options));
	return runtime;
}

export interface BootstrapPlaygroundWorkbenchOptions extends BuildPlaygroundWorkbenchAppOptions {
	/** @emoji 📝 When true (default), registers standard playground declarative bodies before returning. */
	readonly registerDeclarativeBodies?: boolean;
	readonly declarativeBodies?: RegisterPlaygroundDeclarativeBodiesOptions;
	/** @emoji 🧱 Reuse an existing product runtime shell (controller must use its {@link CommandBus}). */
	readonly runtime?: ProductRuntime;
}

/** @emoji 🚀 One-shot playground setup: optional declarative registration + one playground app. */
export function bootstrapPlaygroundWorkbench(
	ids: PlaygroundIds,
	controller: PlaygroundController<string>,
	options?: BootstrapPlaygroundWorkbenchOptions,
): ProductRuntime {
	if (options?.registerDeclarativeBodies !== false) {
		registerPlaygroundDeclarativeBodies(ids, options?.declarativeBodies);
	}
	const runtime = options?.runtime ?? new ProductRuntime();
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
		mainSceneSurfaceId: "test.playground.scene/v1",
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

	describe("PlaygroundController", () => {
		it("tracks query and clears selection when kind is hidden", () => {
			const bus = new CommandBus();
			const wb = new ProductRuntime();
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
		it("buildBoardWindowBody is canvas-only", () => {
			const node = buildBoardWindowBody("elements.board/v1", "board-ctrl", "pane-a");
			expect(node).toEqual({ type: "board", surfaceId: "elements.board/v1", controllerId: "board-ctrl", paneId: "pane-a" });
		});
	});

	describe("registerPlaygroundDeclarativeBodies", () => {
		it("registers scene3d main window and table side panels", () => {
			const bus = new CommandBus();
			const wb = new ProductRuntime();
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
			expect(main?.type).toBe("scene3d");
		});
	});
}
//#endregion 🧪Tests
