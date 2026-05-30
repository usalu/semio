// #region 🧱Header
/** 🧱 `@framework/platform/core` — Renderer-agnostic platform shell: {@link Platform} → {@link AppRuntime} → {@link ModeRuntime}, declarative {@link UiNode} bodies, {@link PluginHost}, {@link SurfaceRouter}, and {@link PlatformDefinition} + {@link SurfaceDefinition} for contribution routing. */
// #endregion 🧱Header

export * from "@framework/core";

import {
	BaseAppRuntime,
	BaseModeRuntime,
	BaseWindowKindRuntime,
	CommandBus,
	Controller,
	Store,
	Platform,
	createTabStackLayout,
	mergeAppTools,
	mergeById,
	mergeSearchItems,
	resolveMode,
	type AppTools,
	type Disposable,
	type FindItem,
	type FooterItem,
	type SurfaceComponent,
	type PlatformSubscriber,
	type SearchItemSpec,
	type SideTabSpec,
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

//#region 🔖ComponentKind
/** @emoji 🧩 Fixed platform component vocabulary wired by renderers (`table`, `puzzle2d`, …). */
export type ComponentKind = "table" | "puzzle2d" | "puzzle3d" | "puzzle5d" | "cad" | "panel";

const CANVAS_COMPONENT_KINDS: readonly ComponentKind[] = ["table", "puzzle2d", "puzzle3d", "puzzle5d", "cad"];
//#endregion 🔖ComponentKind

/** @emoji 📊 Host-bound tabular surface; `paneId` disambiguates multiple table slots in one app. */
export interface UiTableHostSurfaceNode {
	readonly type: "table";
	readonly componentKind: "table";
	readonly surfaceId: string;
	readonly controllerId: string;
	readonly paneId?: string;
	readonly bindingId?: string;
}

/** @emoji 📋 Host-bound 2D puzzle board surface. */
export interface UiPuzzle2dHostSurfaceNode {
	readonly type: "puzzle2d";
	readonly componentKind: "puzzle2d";
	readonly surfaceId: string;
	readonly controllerId: string;
	readonly paneId?: string;
	readonly bindingId?: string;
}

/** @emoji 🧊 Host-bound 3D puzzle scene surface. */
export interface UiPuzzle3dHostSurfaceNode {
	readonly type: "puzzle3d";
	readonly componentKind: "puzzle3d";
	readonly surfaceId: string;
	readonly controllerId: string;
	readonly bindingId?: string;
}

/** @emoji 🌐 Host-bound unified 2D+3D topology surface (`FiveD`). */
export interface UiPuzzle5dHostSurfaceNode {
	readonly type: "puzzle5d";
	readonly componentKind: "puzzle5d";
	readonly surfaceId: string;
	readonly controllerId: string;
	readonly paneId?: string;
	readonly bindingId?: string;
}

/** @emoji 📐 Host-bound CAD spatial surface. */
export interface UiCadHostSurfaceNode {
	readonly type: "cad";
	readonly componentKind: "cad";
	readonly surfaceId: string;
	readonly controllerId: string;
	readonly bindingId?: string;
}

/** @emoji 🧩 Host-bound side-panel surface; renderer maps `surfaceId` to panel body chrome. */
export interface UiPanelHostSurfaceNode {
	readonly type: "panel";
	readonly componentKind: "panel";
	readonly surfaceId: string;
	readonly controllerId: string;
	readonly bindingId?: string;
}

export type UiComponentHostSurfaceNode =
	| UiTableHostSurfaceNode
	| UiPuzzle2dHostSurfaceNode
	| UiPuzzle3dHostSurfaceNode
	| UiPuzzle5dHostSurfaceNode
	| UiCadHostSurfaceNode
	| UiPanelHostSurfaceNode;

export type UiNode =
	| UiStackNode
	| UiTextNode
	| UiButtonNode
	| UiSeparatorNode
	| UiComponentHostSurfaceNode;

/** @emoji 📊 Canonical table window body: only the host-bound table surface. */
export function buildTableWindowBody(surfaceId: string, controllerId: string, paneId?: string, bindingId?: string): UiTableHostSurfaceNode {
	return {
		type: "table",
		componentKind: "table",
		surfaceId,
		controllerId,
		...(paneId ? { paneId } : {}),
		...(bindingId ? { bindingId } : {}),
	};
}

/** @emoji 📋 Canonical 2D puzzle window body. */
export function buildPuzzle2dWindowBody(surfaceId: string, controllerId: string, paneId?: string, bindingId?: string): UiPuzzle2dHostSurfaceNode {
	return {
		type: "puzzle2d",
		componentKind: "puzzle2d",
		surfaceId,
		controllerId,
		...(paneId ? { paneId } : {}),
		...(bindingId ? { bindingId } : {}),
	};
}

/** @emoji 🧊 Canonical 3D puzzle window body. */
export function buildPuzzle3dWindowBody(surfaceId: string, controllerId: string, bindingId?: string): UiPuzzle3dHostSurfaceNode {
	return { type: "puzzle3d", componentKind: "puzzle3d", surfaceId, controllerId, ...(bindingId ? { bindingId } : {}) };
}

/** @emoji 🌐 Canonical 5D topology window body. */
export function buildPuzzle5dWindowBody(surfaceId: string, controllerId: string, paneId?: string, bindingId?: string): UiPuzzle5dHostSurfaceNode {
	return {
		type: "puzzle5d",
		componentKind: "puzzle5d",
		surfaceId,
		controllerId,
		...(paneId ? { paneId } : {}),
		...(bindingId ? { bindingId } : {}),
	};
}

/** @emoji 📐 Canonical CAD window body. */
export function buildCadWindowBody(surfaceId: string, controllerId: string, bindingId?: string): UiCadHostSurfaceNode {
	return { type: "cad", componentKind: "cad", surfaceId, controllerId, ...(bindingId ? { bindingId } : {}) };
}

/** @emoji 🧩 Canonical panel window body. */
export function buildPanelWindowBody(surfaceId: string, controllerId: string, bindingId?: string): UiPanelHostSurfaceNode {
	return { type: "panel", componentKind: "panel", surfaceId, controllerId, ...(bindingId ? { bindingId } : {}) };
}

function isCanvasComponentNode(node: UiNode): boolean {
	if (node.type === "text") return true;
	if (node.type === "panel") return false;
	return CANVAS_COMPONENT_KINDS.includes(node.type as ComponentKind);
}

/** @emoji ✅ True when a window body is a lone canvas component surface or a short error `text` node. */
export function isCanvasOnlyWindowBody(node: UiNode): boolean {
	if (isCanvasComponentNode(node)) return true;
	if (node.type === "stack" && node.padding === "none" && node.children.length === 1) {
		return isCanvasComponentNode(node.children[0]);
	}
	return false;
}

function assertCanvasOnlyWindowBody(bodyKey: string, node: UiNode): void {
	if (isCanvasOnlyWindowBody(node)) return;
	throw new Error(
		`Declarative window body "${bodyKey}" must be a single table, puzzle2d, puzzle3d, puzzle5d, or cad surface (optional none padding stack wrapper). Found "${node.type}". Use ModeRuntime.tools, side tabs, or window measures for chrome.`,
	);
}
//#endregion 🔖UiNode

//#region 🔖ComponentModels
/** @emoji 📊 Column descriptor for {@link TableModel}. */
export interface TableColumnModel {
	readonly id: string;
	readonly label: string;
	readonly width?: number;
}

/** @emoji 📊 Row descriptor for {@link TableModel}. */
export interface TableRowModel {
	readonly id: string;
	readonly cells: Readonly<Record<string, string | number | boolean | null>>;
	readonly navigateUri?: string;
}

/** @emoji 📊 Render-agnostic tabular view-model for {@link Table}. */
export interface TableModel {
	readonly columns: readonly TableColumnModel[];
	readonly rows: readonly TableRowModel[];
	readonly selectedRowIds?: readonly string[];
	readonly sortColumnId?: string | null;
	readonly sortDescending?: boolean;
	readonly emptyMessage?: string;
}

/** @emoji 📋 Node descriptor for {@link Puzzle2dModel}. */
export interface Puzzle2dNodeModel {
	readonly id: string;
	readonly label?: string;
	readonly x?: number;
	readonly y?: number;
}

/** @emoji 📋 Edge descriptor for {@link Puzzle2dModel}. */
export interface Puzzle2dEdgeModel {
	readonly id: string;
	readonly sourceId: string;
	readonly targetId: string;
}

/** @emoji 📋 Render-agnostic 2D board view-model for {@link Puzzle2d}. */
export interface Puzzle2dModel {
	readonly nodes: readonly Puzzle2dNodeModel[];
	readonly edges: readonly Puzzle2dEdgeModel[];
	readonly portColors?: Readonly<Record<string, string>>;
	readonly emptyMessage?: string;
}

/** @emoji 🧊 Render-agnostic 3D scene view-model for {@link Puzzle3d}. */
export interface Puzzle3dModel {
	readonly instanceId?: string;
	readonly emptyMessage?: string;
}

/** @emoji 🌐 Render-agnostic unified topology view-model for {@link Puzzle5d}. */
export interface Puzzle5dModel {
	readonly presentation: "flat" | "volume";
	readonly instanceId: string;
	readonly emptyMessage?: string;
}

/** @emoji 📐 Render-agnostic CAD view-model for {@link Cad}. */
export interface CadModel {
	readonly instanceId?: string;
	readonly emptyMessage?: string;
}

/** @emoji 🧩 Render-agnostic panel body for {@link Panel}. */
export interface PanelModel {
	readonly body: UiNode;
}
//#endregion 🔖ComponentModels

//#region 🔖Component
/** @emoji 🧩 Render-agnostic platform surface backed by a {@link Store} snapshot. */
export abstract class Component<TSnapshot> extends Store<TSnapshot> implements SurfaceComponent {
	readonly componentKind: ComponentKind;
	readonly surfaceId: string;
	readonly controllerId: string;
	private snapshotValue: TSnapshot;

	constructor(componentKind: ComponentKind, surfaceId: string, controllerId: string, initialSnapshot: TSnapshot) {
		super();
		this.componentKind = componentKind;
		this.surfaceId = surfaceId;
		this.controllerId = controllerId;
		this.snapshotValue = initialSnapshot;
	}

	override getSnapshot(): TSnapshot {
		return this.snapshotValue;
	}

	protected setSnapshot(next: TSnapshot): void {
		if (Object.is(this.snapshotValue, next)) return;
		this.snapshotValue = next;
		this.notify();
	}

	abstract buildSnapshot(): TSnapshot;

	refresh(): void {
		this.setSnapshot(this.buildSnapshot());
	}
}

/** @emoji 📊 Table surface component base class. */
export class Table extends Component<TableModel> {
	constructor(surfaceId: string, controllerId: string, initialSnapshot: TableModel = { columns: [], rows: [] }) {
		super("table", surfaceId, controllerId, initialSnapshot);
	}

	buildSnapshot(): TableModel {
		return this.getSnapshot();
	}
}

/** @emoji 📋 2D puzzle board surface component base class. */
export class Puzzle2d extends Component<Puzzle2dModel> {
	constructor(surfaceId: string, controllerId: string, initialSnapshot: Puzzle2dModel = { nodes: [], edges: [] }) {
		super("puzzle2d", surfaceId, controllerId, initialSnapshot);
	}

	buildSnapshot(): Puzzle2dModel {
		return this.getSnapshot();
	}
}

/** @emoji 🧊 3D puzzle scene surface component base class. */
export class Puzzle3d extends Component<Puzzle3dModel> {
	constructor(surfaceId: string, controllerId: string, initialSnapshot: Puzzle3dModel = {}) {
		super("puzzle3d", surfaceId, controllerId, initialSnapshot);
	}

	buildSnapshot(): Puzzle3dModel {
		return this.getSnapshot();
	}
}

/** @emoji 🌐 5D topology surface component base class. */
export class Puzzle5d extends Component<Puzzle5dModel> {
	constructor(surfaceId: string, controllerId: string, initialSnapshot: Puzzle5dModel) {
		super("puzzle5d", surfaceId, controllerId, initialSnapshot);
	}

	buildSnapshot(): Puzzle5dModel {
		return this.getSnapshot();
	}
}

/** @emoji 📐 CAD surface component base class. */
export class Cad extends Component<CadModel> {
	constructor(surfaceId: string, controllerId: string, initialSnapshot: CadModel = {}) {
		super("cad", surfaceId, controllerId, initialSnapshot);
	}

	buildSnapshot(): CadModel {
		return this.getSnapshot();
	}
}

/** @emoji 🧩 Panel surface component base class. */
export class Panel extends Component<PanelModel> {
	constructor(surfaceId: string, controllerId: string, initialSnapshot: PanelModel) {
		super("panel", surfaceId, controllerId, initialSnapshot);
	}

	buildSnapshot(): PanelModel {
		return this.getSnapshot();
	}
}

/** @emoji 🧩 Registers a {@link Component} on a {@link Platform} instance. */
export function registerPlatformComponent(platform: Platform, component: Component<unknown>): void {
	platform.registerComponent(component);
}

/** @emoji 🔍 Typed lookup of a registered {@link Component} by surface id. */
export function getPlatformComponent<T extends Component<unknown>>(platform: Platform, surfaceId: string): T | undefined {
	return platform.getComponent(surfaceId) as T | undefined;
}
//#endregion 🔖Component

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

/** @emoji 🔗 Typed pair used in {@link PlatformDefinition} surface maps. */
export interface SurfaceBinding<TApi, TContribution> {
	readonly api: TApi;
	readonly contributions: TContribution;
}
//#endregion 🔖SurfaceDefinition

//#region 🔖WindowKindRuntime
/** @emoji 🪟 Declarative window kind; React renderer maps `bodyKey` to a component. */
export class WindowKindRuntime extends BaseWindowKindRuntime {
	readonly capabilities: Capability[] = [];
	readonly surfaces: SurfaceDefinition[] = [];
	commands: SearchItemSpec[] = [];

	constructor(
		id: string,
		label: string,
		bodyKey: string,
		iconId?: string,
		measures: readonly WindowMeasure[] = [],
		capabilities?: readonly Capability[],
	) {
		super(id, label, bodyKey, iconId, measures);
		if (capabilities?.length) this.capabilities.push(...capabilities);
	}
}
//#endregion 🔖WindowKindRuntime

//#region 🔖ModeRuntime
/** @emoji 🎚 Single app mode: toolbars, window kinds, and side tab specs. */
export class ModeRuntime extends BaseModeRuntime {
	commands: SearchItemSpec[] = [];
	findItems: FindItem[] = [];
	onFindSelect?: (itemId: string) => void;
	onActiveWindowChange?: (windowKindId: string) => void;
	selection: Record<string, unknown> = {};
	hover: Record<string, unknown> = {};
	options: Record<string, unknown> = {};

	declare windowKinds: WindowKindRuntime[];

	constructor(id: string, label: string, iconId: string | undefined) {
		super(id, label, iconId);
	}
}
//#endregion 🔖ModeRuntime

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

/** @emoji 🧮 Resolves active mode overlays for the platform product shell. */
export function resolveAppState(app: AppRuntime, requestedModeId?: string | null): ResolvedAppState {
	const mode = resolveMode(app, requestedModeId) as ModeRuntime | null;
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
export class AppRuntime extends BaseAppRuntime {
	commands: SearchItemSpec[] = [];
	findItems: FindItem[] = [];
	onFindSelect?: (itemId: string) => void;
	onActiveWindowChange?: (windowKindId: string) => void;
	selection: Record<string, unknown> = {};
	hover: Record<string, unknown> = {};
	options: Record<string, unknown> = {};

	declare modes: ModeRuntime[];
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

	override addMode(mode: ModeRuntime): void {
		super.addMode(mode);
	}

	override resolve(requestedModeId?: string | null): ResolvedAppState {
		const modeId = requestedModeId ?? this.getActiveModeId();
		return resolveAppState(this, modeId);
	}
}
//#endregion 🔖AppRuntime

/** @emoji 🧭 Resolves the command palette rows visible for the active UI/app/mode/window scope. */
export function resolveCommandPaletteItems(platform: Platform, app: ResolvedAppState, activeWindowKindId?: string | null): SearchItemSpec[] {
	const uiCommands = mergeSearchItems(platform.searchItems, platform.commands) ?? platform.commands;
	const windowKind = activeWindowKindId ? app.windowKinds.find((entry) => entry.id === activeWindowKindId) : undefined;
	return mergeSearchItems(mergeSearchItems(uiCommands, app.commands), windowKind?.commands) ?? [];
}

//#region 🔖WindowBodyViewContext
/** @emoji 🪟 View context for declarative window bodies: platform snapshot without DOM or React roots. */
export interface WindowBodyViewContext {
	readonly platform: Platform;
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

//#region 🔖PlatformDefinition
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

export interface PlatformDefinition<TProductApi = unknown> {
	readonly id: string;
	readonly name: string;
	readonly apiVersion: string;
	readonly apps: readonly AppDefinition[];
	createPlatformApi(ctx: PluginContext): TProductApi;
}
//#endregion 🔖PlatformDefinition

//#region 🔖SurfaceContext
/** @emoji 🧩 Activation context for a single {@link SurfaceDefinition} instance. */
export interface SurfaceContext<TSurfaceId extends string = string> {
	readonly surfaceId: TSurfaceId;
	readonly productId: string;
	readonly appId: string;
	readonly modeId: string;
	readonly windowKindId: string;
	readonly platform: Platform;
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
	static flattenFromPlatformDefinition(product: PlatformDefinition, resolveWhen: ContextKeyResolver = matchAllContext): SurfaceRoutingRow[] {
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
		readonly platform: Platform,
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
			this.platform.addApp(app);
			this.disposables.push({
				dispose: () => {
					const index = this.platform.apps.findIndex((entry) => entry.id === spec.id);
					if (index >= 0) this.platform.apps.splice(index, 1);
				},
			});
		}
	}

	subscribe(listener: PlatformSubscriber): PluginSubscription {
		const unsubscribe = this.platform.subscribe(listener);
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

	constructor(readonly platform: Platform) {}

	register(manifest: PluginManifest, module?: PluginModule): void {
		if (module && module.id !== manifest.id) {
			throw new Error(`Plugin module id "${module.id}" does not match manifest id "${manifest.id}".`);
		}
		this.plugins.set(manifest.id, { manifest, module });
	}

	getControllerById(controllerId: string): Controller | undefined {
		for (const app of this.platform.apps) {
			if (app.controller.id === controllerId) return app.controller;
		}
		return undefined;
	}

	async activateAll(getController: (controllerId: string) => Controller | undefined): Promise<void> {
		if (this.activated) return;
		this.activated = true;
		for (const { manifest, module } of this.plugins.values()) {
			const context = new PluginContext(this.platform, manifest);
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

//#region 🔖PlatformPlugin
/** @emoji 🧩 Typed product plugin: manifest target, optional per-surface activation, and selector-based contributions. */
export interface PlatformPlugin<TProductApi = unknown, TSurfaceMap extends Record<string, SurfaceBinding<unknown, unknown>> = Record<string, SurfaceBinding<unknown, unknown>>> {
	readonly id: string;
	readonly target: { readonly product: string; readonly api: string };
	readonly manifest?: PluginManifest;
	activate?(ctx: PluginContext, product: TProductApi): void | Promise<void>;
	deactivate?(): void | Promise<void>;
	surfaces?: { [K in keyof TSurfaceMap]?: (ctx: SurfaceContext<K & string>, surface: TSurfaceMap[K]["api"]) => Disposable | Promise<Disposable> };
	contributes?: { readonly selectors?: readonly ContributionRoute[] };
}

/** @emoji ✅ Identity helper for authoring {@link PlatformPlugin} definitions. */
export function definePlatformPlugin<TProductApi, TSurfaceMap extends Record<string, SurfaceBinding<unknown, unknown>>>(
	plugin: PlatformPlugin<TProductApi, TSurfaceMap>,
): PlatformPlugin<TProductApi, TSurfaceMap> {
	return plugin;
}
//#endregion 🔖PlatformPlugin

//#region 🔖PlatformPluginActivationHost
/** @emoji 🎛 Activates {@link PlatformPlugin} instances: product lifecycle + surface handlers + routed contributions. */
export class PlatformPluginActivationHost<TProductApi = unknown> {
	private readonly disposables: Disposable[] = [];
	private productApi: TProductApi | undefined;

	constructor(
		readonly platform: Platform,
		readonly productId: string,
		readonly createApi: (ctx: PluginContext) => TProductApi,
	) {}

	async activateAll(plugins: readonly PlatformPlugin<TProductApi>[], getController: (controllerId: string) => Controller | undefined): Promise<void> {
		void getController;
		const bootstrapCtx = new PluginContext(this.platform, { id: "__product", contributes: {} });
		this.productApi ??= this.createApi(bootstrapCtx);
		const rows = () => SurfaceRouter.flattenFromRuntimeApps(this.productId, this.platform.apps);
		for (const plugin of plugins) {
			const manifest: PluginManifest = plugin.manifest ?? { id: plugin.id, contributes: {} };
			const ctx = new PluginContext(this.platform, manifest);
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
					platform: this.platform,
					activeModeId: this.platform.getActiveApp()?.getActiveModeId() ?? null,
					generation: this.platform.generation,
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
					platform: this.platform,
					activeModeId: this.platform.getActiveApp()?.getActiveModeId() ?? null,
					generation: this.platform.generation,
				})),
			);
		}
	}

	disposeAll(): void {
		for (const d of this.disposables.splice(0)) d.dispose();
	}
}
//#endregion 🔖PlatformPluginActivationHost

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("canvas-only declarative window bodies", () => {
		it("accepts lone puzzle and table nodes", () => {
			expect(isCanvasOnlyWindowBody(buildPuzzle3dWindowBody("s", "c"))).toBe(true);
			expect(isCanvasOnlyWindowBody(buildPuzzle2dWindowBody("b", "c", "pane"))).toBe(true);
			expect(isCanvasOnlyWindowBody(buildTableWindowBody("t", "c"))).toBe(true);
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
						buildPuzzle5dWindowBody("s", "c"),
					],
				}),
			).toThrow(/table, puzzle2d, puzzle3d, puzzle5d, or cad/);
		});
	});

	describe("Platform", () => {
		it("constructs from PlatformSpec metadata", () => {
			const platform = new Platform({ id: "demo", name: "Demo", defaultActiveAppId: "home" });
			expect(platform.id).toBe("demo");
			expect(platform.name).toBe("Demo");
			expect(platform.activeAppId).toBe("home");
		});
	});

	describe("Component registry", () => {
		it("registers components by surface id and refreshes models", () => {
			class DemoTable extends Table {
				override buildSnapshot(): TableModel {
					return {
						columns: [{ id: "name", label: "Name" }],
						rows: [{ id: "1", cells: { name: "alpha" } }],
					};
				}
			}
			const platform = new Platform({ id: "demo", name: "Demo" });
			const table = new DemoTable("surface/table/v1", "ctrl");
			registerPlatformComponent(platform, table);
			table.refresh();
			const resolved = getPlatformComponent<DemoTable>(platform, "surface/table/v1");
			expect(resolved?.getSnapshot().rows[0]?.cells.name).toBe("alpha");
		});
	});

	describe("PluginHost", () => {
		it("merges contributed apps and declarative window bodies", async () => {
			class TCtrl extends Controller {
				override run(): void {}
			}
			const bus = new CommandBus();
			const rt = new Platform();
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
			expect(factory?.({ platform: rt, windowKindId: "main", bodyKey: "demo.ext.main", activeModeId: null, generation: 0 }).type).toBe("text");
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
			const rt = new Platform();
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
						platform: rt,
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

	describe("definePlatformPlugin lifecycle", () => {
		it("runs activate once and disposes surface contributions", async () => {
			class TCtrl extends Controller {
				override run(): void {}
			}
			const bus = new CommandBus();
			const rt = new Platform();
			const ctrl = new TCtrl("c", bus, () => rt.notify());
			rt.addApp(new AppRuntime("app", "App", undefined, ctrl, createTabStackLayout(["w"]), [new WindowKindRuntime("w", "W", "k", undefined, [], ["x"])]));
			let surfaceActivations = 0;
			const plugin = definePlatformPlugin({
				id: "pl",
				target: { product: "p", api: "^1" },
				surfaces: {
					[`framework.window:app:default:w`]: async () => {
						surfaceActivations++;
						return { dispose: () => undefined };
					},
				},
			});
			const host = new PlatformPluginActivationHost(rt, "p", () => ({}) as Record<string, never>);
			await host.activateAll([plugin], () => ctrl);
			expect(surfaceActivations).toBe(1);
			host.disposeAll();
		});
	});

		describe("resolveCommandPaletteItems", () => {
			it("merges ui, app, mode, and active window kind commands by active scope", () => {
				const runtime = new Platform();
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
