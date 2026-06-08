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
	readonly iconId: string;
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
	readonly iconId: string;
	readonly label?: string;
	readonly pressed: boolean;
	readonly text?: string;
	readonly onChange: CommandDescriptor;
}

/** @emoji 🌳 Collapsible branch in the window measures rail; {@link children} are leaves or nested groups. */
export interface WindowMeasureGroup {
	readonly kind: "group";
	readonly id: string;
	readonly label: string;
	readonly defaultOpen?: boolean;
	readonly children: readonly WindowMeasure[];
}

export type WindowMeasure = WindowMeasureSelect | WindowMeasureSlider | WindowMeasureToggle | WindowMeasureGroup;
//#endregion 🔖WindowMeasure

//#region 🔖KindWeights
export type KindWeightMap = Readonly<Record<string, number>>;

/** @emoji ⚖️ Equal weights for every id; sum is 1 (or empty map when no ids). */
export function uniformKindWeights(ids: readonly string[]): KindWeightMap {
	if (ids.length === 0) {
		return {};
	}
	const w = 1 / ids.length;
	return Object.fromEntries(ids.map((id) => [id, w]));
}

/** @emoji 🏗️ Relative brush-suggestion weight for a single base-tier kind (before group normalization). */
export const PUZZLE_KIND_SUGGESTION_WEIGHT_BASE = 1;

/** @emoji 🥁 Tambour kinds are suggested 15× as often as base kinds (per-kind weight ratio). */
export const PUZZLE_KIND_SUGGESTION_WEIGHT_TAMBOUR = 15;

/** @emoji 🏛️ Capital kinds are suggested 10× less often than tambour kinds (tambour:capital = 10:1). */
export const PUZZLE_KIND_SUGGESTION_WEIGHT_CAPITAL = PUZZLE_KIND_SUGGESTION_WEIGHT_TAMBOUR / 10;

/** @emoji 💊 Capsule kinds are suggested 8× as often as tambour kinds (per-kind weight ratio). */
export const PUZZLE_KIND_SUGGESTION_WEIGHT_CAPSULE = PUZZLE_KIND_SUGGESTION_WEIGHT_TAMBOUR * 8;

/** @emoji 🎚️ Classifies a catalog kind id into a default brush suggestion tier (nakagin tower vocabulary). */
export function puzzleKindSuggestionRelativeWeight(kindId: string): number {
	const k = kindId.toLowerCase();
	if (k.includes("capsule")) {
		return PUZZLE_KIND_SUGGESTION_WEIGHT_CAPSULE;
	}
	if (k.includes("tambour")) {
		return PUZZLE_KIND_SUGGESTION_WEIGHT_TAMBOUR;
	}
	if (k.includes("capital") || k.startsWith("roof ")) {
		return PUZZLE_KIND_SUGGESTION_WEIGHT_CAPITAL;
	}
	if (k === "base" || k === "base blob" || k.startsWith("core ")) {
		return PUZZLE_KIND_SUGGESTION_WEIGHT_BASE;
	}
	return PUZZLE_KIND_SUGGESTION_WEIGHT_BASE;
}

/** @emoji 🎚️ Default puzzle brush weights for catalog ids (capsule > tambour > capital > base; sums to 1). */
export function defaultPuzzleKindWeights(ids: readonly string[]): KindWeightMap {
	if (ids.length === 0) {
		return {};
	}
	const raw = ids.map((id) => puzzleKindSuggestionRelativeWeight(id));
	const sum = raw.reduce((acc, w) => acc + w, 0);
	if (sum <= 0) {
		return uniformKindWeights(ids);
	}
	return Object.fromEntries(ids.map((id, i) => [id, raw[i]! / sum]));
}

/** @emoji 🎚️ Renormalizes one kind-weight group after a slider move so the group still sums to 1. */
export function normalizeKindWeightGroup(weights: KindWeightMap, changedId: string, newValue: number): KindWeightMap {
	const clamped = Math.max(0, Math.min(1, newValue));
	const ids = Object.keys(weights);
	if (ids.length === 0) {
		return { [changedId]: clamped };
	}
	const rest = ids.filter((id) => id !== changedId);
	const budget = 1 - clamped;
	if (rest.length === 0) {
		return { [changedId]: 1 };
	}
	const restSum = rest.reduce((acc, id) => acc + (weights[id] ?? 0), 0);
	const next: Record<string, number> = { [changedId]: clamped };
	if (restSum <= 0) {
		const each = budget / rest.length;
		for (const id of rest) {
			next[id] = each;
		}
	} else {
		for (const id of rest) {
			next[id] = (budget * (weights[id] ?? 0)) / restSum;
		}
	}
	return next;
}

/** @emoji 🔀 Merges catalog ids with existing weights (uniform for new ids, drops unknown). */
export function syncKindWeightMap(ids: readonly string[], existing: KindWeightMap): KindWeightMap {
	if (ids.length === 0) {
		return {};
	}
	const prev = ids.map((id) => existing[id] ?? 0);
	const sum = prev.reduce((a, b) => a + b, 0);
	if (sum <= 0) {
		return defaultPuzzleKindWeights(ids);
	}
	return Object.fromEntries(ids.map((id, i) => [id, prev[i]! / sum]));
}

export type WeightedOrderRng = () => number;

/** @emoji 🎲 Weighted sampling without replacement; higher weight → earlier in the list. */
export function weightedOrder<T>(items: readonly T[], weightOf: (item: T) => number, rng: WeightedOrderRng = Math.random): readonly T[] {
	if (items.length < 2) {
		return [...items];
	}
	const remaining = [...items];
	const out: T[] = [];
	while (remaining.length > 0) {
		const weights = remaining.map((item) => Math.max(0, weightOf(item)));
		const total = weights.reduce((a, b) => a + b, 0);
		let pick = 0;
		if (total > 0) {
			let r = rng() * total;
			for (let i = 0; i < remaining.length; i += 1) {
				r -= weights[i]!;
				if (r <= 0) {
					pick = i;
					break;
				}
			}
		} else {
			pick = Math.floor(rng() * remaining.length);
		}
		out.push(remaining[pick]!);
		remaining.splice(pick, 1);
	}
	return out;
}
//#endregion 🔖KindWeights

//#region 🔖Layout
/** @emoji 🪟 Single window slot in the abstract layout tree. */
export interface WindowLayoutWindowNode {
	readonly kind: "window";
	readonly windowKindId: string;
	readonly title?: string;
	readonly instanceId?: string;
	readonly templateId?: string;
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

//#region 🔖WindowTemplate
/** @emoji 📐 Preset for spawning a window of a kind (camera, projection, …). */
export interface WindowTemplate {
	readonly id: string;
	readonly label: string;
	readonly iconId?: string;
	readonly controllerId?: string;
	readonly command?: string;
	readonly args?: unknown;
	readonly children?: readonly WindowTemplate[];
}
//#endregion 🔖WindowTemplate

//#region 🔖NamedLayout
/** @emoji 🧭 Reusable window arrangement (builtin catalog or user-saved). */
export interface NamedLayout {
	readonly id: string;
	readonly label: string;
	readonly iconId?: string;
	readonly layout: WindowLayout;
	readonly origin: "builtin" | "user";
	readonly groupPath?: readonly string[];
}

/** @emoji 🧭 Factory for a catalog or saved {@link NamedLayout}. */
export function createNamedLayout(
	id: string,
	label: string,
	layout: WindowLayout,
	origin: NamedLayout["origin"] = "builtin",
	iconId?: string,
	groupPath?: readonly string[],
): NamedLayout {
	return {
		id,
		label,
		layout,
		origin,
		...(iconId ? { iconId } : {}),
		...(groupPath?.length ? { groupPath } : {}),
	};
}

/** @emoji 🔀 Merges named layouts by `id`; extension entries override base. */
export function mergeNamedLayouts(base: readonly NamedLayout[] | undefined, extension: readonly NamedLayout[] | undefined): NamedLayout[] {
	return mergeById(base, extension) ?? [];
}
//#endregion 🔖NamedLayout

//#region 🔖LayoutFactories
/** @emoji 🪟 Single window slot helper for {@link WindowLayout} trees. */
export function createWindowLayout(
	windowKindId: string,
	title?: string,
	options?: { readonly instanceId?: string; readonly templateId?: string },
): WindowLayoutWindowNode {
	return {
		kind: "window",
		windowKindId,
		...(title ? { title } : {}),
		...(options?.instanceId ? { instanceId: options.instanceId } : {}),
		...(options?.templateId ? { templateId: options.templateId } : {}),
	};
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
	| "transfer"
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
	"transfer",
	"transform",
	"create",
	"view",
	"actions",
	"settings",
];

/** @emoji 🎛 Declarative toolbar item; interactions route through {@link CommandBus}. */
export type ToolItem =
	| {
			readonly id: string;
			readonly kind: "separator";
			readonly order?: number;
			readonly disabled?: boolean;
	  }
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
/** @emoji 📐 Fixed platform shell panel slot (left: display/workbench/overview; right: details/settings/chat). */
export type PanelKind = "display" | "overview" | "workbench" | "details" | "settings" | "chat";

export const LEFT_PANEL_KINDS: readonly PanelKind[] = ["display", "workbench", "overview"];
export const RIGHT_PANEL_KINDS: readonly PanelKind[] = ["details", "settings", "chat"];
export const PANEL_KINDS: readonly PanelKind[] = [...LEFT_PANEL_KINDS, ...RIGHT_PANEL_KINDS];

/** @emoji ↔️ Returns whether a panel kind is rendered on the left or right side of the canvas. */
export function panelSide(kind: PanelKind): "left" | "right" {
	return LEFT_PANEL_KINDS.includes(kind) ? "left" : "right";
}

/** @emoji 📑 Side panel tab addressing a declarative `bodyKey` tree host. */
export interface SideTabSpec {
	readonly id: string;
	readonly iconId: string;
	readonly panel: PanelKind;
	readonly order?: number;
	readonly bodyKey: string;
	/** @emoji 🏷️ Panel section title; omit for content-only panel chrome. */
	readonly label?: string;
}
//#endregion 🔖SideTab

//#region 🔖Footer
/** @emoji 👣 Footer strip item with optional command dispatch. */
export interface FooterItem {
	readonly id: string;
	readonly iconId: string;
	readonly text?: string;
	readonly order?: number;
	readonly className?: string;
	readonly disabled?: boolean;
	readonly controllerId?: string;
	readonly command?: string;
	readonly args?: unknown;
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

//#region 🔖Store
/** @emoji 📡 Minimal listener set for host invalidation without external reactive libs. */
export type PlatformSubscriber = () => void;

/** @emoji 🗄️ Renderer-neutral observable state; backends (memory, disk, worker, …) live in subclasses. */
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
//#endregion 🔖Store

//#region 🔖DisplayStore
/** @emoji 💾 Key-value persistence port for user-saved display layouts (renderer supplies the backend). */
export interface StoragePort {
	get(key: string): string | null;
	set(key: string, value: string): void;
	remove(key: string): void;
}

function namedLayoutStorageKey(appId: string): string {
	return `semio.display.layouts.${appId}`;
}

/** @emoji 🧭 Observable store of user-saved {@link NamedLayout}s for one app. */
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
					Boolean(entry) &&
					typeof entry === "object" &&
					typeof (entry as NamedLayout).id === "string" &&
					typeof (entry as NamedLayout).label === "string" &&
					(entry as NamedLayout).origin === "user" &&
					Boolean((entry as NamedLayout).layout),
			);
		} catch {
			return [];
		}
	}

	private persist(): void {
		this.storage.set(namedLayoutStorageKey(this.appId), JSON.stringify(this.layouts));
	}
}
//#endregion 🔖DisplayStore

//#region 🔖Observable
/** @emoji 📦 In-memory cell store; notifies subscribers on `set`. */
export class ObservableCell<T> extends Store<T> {
	private value: T;

	constructor(initial: T) {
		super();
		this.value = initial;
	}

	get(): T {
		return this.value;
	}

	getSnapshot(): T {
		return this.value;
	}

	set(next: T): void {
		if (Object.is(this.value, next)) return;
		this.value = next;
		this.notify();
	}

	update(updater: (previous: T) => T): void {
		this.set(updater(this.value));
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
export class AppPointerFocusStore<TKey> extends Store<AppPointerFocusSnapshot<TKey>> {
	private selection = new Set<TKey>();
	private hover: TKey | null = null;
	private hoverSourceId: string | null = null;
	private snapshot: AppPointerFocusSnapshot<TKey> = {
		selection: [],
		hover: null,
		hoverSourceId: null,
	};

	constructor(initialSelection: readonly TKey[] = []) {
		super();
		this.selection = new Set(initialSelection);
		if (initialSelection.length > 0) {
			this.rebuildSnapshot();
		}
	}

	override getSnapshot(): AppPointerFocusSnapshot<TKey> {
		return this.snapshot;
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
		if (this.hoverSourceId === sourceId && Object.is(this.hover, key)) {
			return;
		}
		this.hoverSourceId = sourceId;
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

	private rebuildSnapshot(): void {
		this.snapshot = {
			selection: [...this.selection],
			hover: this.hover,
			hoverSourceId: this.hoverSourceId,
		};
	}

	private publish(): void {
		const prev = this.snapshot;
		const nextSelection = [...this.selection];
		if (
			prev.hover === this.hover &&
			prev.hoverSourceId === this.hoverSourceId &&
			prev.selection.length === nextSelection.length &&
			prev.selection.every((key, index) => key === nextSelection[index])
		) {
			return;
		}
		this.rebuildSnapshot();
		this.notify();
	}
}
//#endregion 🔖AppPointerFocus

//#region 🔖PuzzlePlayHover
/** @emoji 🧩 Puzzle 2D catalog-kind hover domain (instance → kind "is a"). */
export type Puzzle2dKindHoverDomain = "node" | "handle" | "edge" | "wire";

/** @emoji 📷 Puzzle 3D catalog-kind hover domain (instance → kind "is a"). */
export type Puzzle3dKindHoverDomain = "object" | "vortex" | "attraction";

/** @emoji 🖱️ Active transitive hover kind for puzzle 2D instances. */
export interface Puzzle2dKindHover {
	readonly domain: Puzzle2dKindHoverDomain;
	readonly kindId: string;
}

/** @emoji 🖱️ Active transitive hover kind for puzzle 3D instances. */
export interface Puzzle3dKindHover {
	readonly domain: Puzzle3dKindHoverDomain;
	readonly kindId: string;
}

/** @emoji 🖱️ Compares two puzzle 2D kind hovers for equality. */
export function puzzle2dKindHoversEqual(a: Puzzle2dKindHover | null, b: Puzzle2dKindHover | null): boolean {
	if (a === b) {
		return true;
	}
	if (!a || !b) {
		return false;
	}
	return a.domain === b.domain && a.kindId === b.kindId;
}

/** @emoji 🖱️ Compares two puzzle 3D kind hovers for equality. */
export function puzzle3dKindHoversEqual(a: Puzzle3dKindHover | null, b: Puzzle3dKindHover | null): boolean {
	if (a === b) {
		return true;
	}
	if (!a || !b) {
		return false;
	}
	return a.domain === b.domain && a.kindId === b.kindId;
}
//#endregion 🔖PuzzlePlayHover

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
	private readonly ownedStores = new Map<string, Store<unknown>>();

	protected constructor(id: string, commandBus: CommandBus, hostNotify: () => void) {
		this.id = id;
		this.commandBus = commandBus;
		this.hostNotify = hostNotify;
		commandBus.register(this);
	}

	protected emit(): void {
		this.hostNotify();
	}

	/** @emoji 🗄️ Registers a store owned by this controller (replaces same id). */
	protected provideStore<TSnapshot>(id: string, store: Store<TSnapshot>): Store<TSnapshot> {
		const previous = this.ownedStores.get(id);
		if (previous && previous !== store) previous.dispose();
		this.ownedStores.set(id, store as Store<unknown>);
		return store;
	}

	/** @emoji 🔍 Resolves a controller-owned store by id. */
	getStore<TSnapshot>(id: string): Store<TSnapshot> | undefined {
		return this.ownedStores.get(id) as Store<TSnapshot> | undefined;
	}

	/** @emoji 🗑️ Disposes and unregisters a controller-owned store by id. */
	protected revokeStore(id: string): void {
		const store = this.ownedStores.get(id);
		store?.dispose();
		this.ownedStores.delete(id);
	}

	/** @emoji 📚 All stores currently provided by this controller. */
	get stores(): ReadonlyMap<string, Store<unknown>> {
		return this.ownedStores;
	}

	dispose(): void {
		for (const store of this.ownedStores.values()) store.dispose();
		this.ownedStores.clear();
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
		readonly templates: readonly WindowTemplate[] = [],
	) {}
}
//#endregion 🔖WindowKindRuntime

//#region 🔖ModeRuntime
/** @emoji 🎚 Single app mode: toolbars, window kinds, and side tab specs. */
export class BaseModeRuntime {
	tools: AppTools = {};
	windowKinds: BaseWindowKindRuntime[] = [];
	namedLayouts: NamedLayout[] = [];
	defaultLayout?: WindowLayout;
	panelTabs: SideTabSpec[] = [];
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
	readonly namedLayouts: readonly NamedLayout[];
	readonly defaultLayout: WindowLayout;
	readonly panelTabs: SideTabSpec[];
	readonly footerItems: FooterItem[];
}

/** @emoji 🧮 Merges app-level and active-mode overlays into {@link ResolvedAppState}. */
export function resolveBaseAppState(app: BaseAppRuntime, requestedModeId?: string | null): ResolvedAppState {
	const mode = resolveMode(app, requestedModeId);
	const mergedWindowKinds = mergeById(app.windowKinds, mode?.windowKinds) ?? app.windowKinds;
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

//#region 🔖AppRuntime
/** @emoji 🧩 One registered app with modes, layout, and a primary {@link Controller}. */
export class BaseAppRuntime {
	readonly modes: BaseModeRuntime[] = [];
	defaultModeId?: string;
	private activeModeIdOverride: string | null = null;
	windowKinds: BaseWindowKindRuntime[] = [];
	namedLayouts: NamedLayout[] = [];
	defaultLayout!: WindowLayout;
	tools: AppTools = {};
	panelTabs: SideTabSpec[] = [];
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
		this.invalidateResolvedState();
	}

	getActiveModeId(): string | null {
		if (this.activeModeIdOverride) return this.activeModeIdOverride;
		return resolveMode(this, null)?.id ?? null;
	}

	setActiveModeId(modeId: string | null): void {
		if (this.activeModeIdOverride === modeId) return;
		this.activeModeIdOverride = modeId;
		this.invalidateResolvedState();
	}

	private resolvedCache: { modeId: string | null; state: ResolvedAppState } | null = null;

	invalidateResolvedState(): void {
		this.resolvedCache = null;
	}

	resolve(requestedModeId?: string | null): ResolvedAppState {
		const modeId = requestedModeId ?? this.getActiveModeId();
		if (this.resolvedCache?.modeId === modeId) {
			return this.resolvedCache.state;
		}
		const state = resolveBaseAppState(this, modeId);
		this.resolvedCache = { modeId, state };
		return state;
	}
}
//#endregion 🔖AppRuntime

//#region 🔖PlatformSpec
/** @emoji 🧾 Declarative platform bootstrap: metadata + optional default chrome. */
export interface PlatformSpec {
	readonly id: string;
	readonly name: string;
	readonly defaultActiveAppId?: string;
	/** Side panels start hidden unless set; see {@link resolveInitialPanelVisibility}. */
	readonly initialPanelVisibility?: PanelVisibility;
	readonly className?: string;
	readonly mobile?: boolean;
	readonly mobileQuery?: string;
	readonly globalTools?: AppTools;
	readonly commands?: readonly SearchItemSpec[];
	readonly searchItems?: readonly SearchItemSpec[];
	readonly globalFooterItems?: readonly FooterItem[];
}
//#endregion 🔖PlatformSpec

//#region 🔖PanelVisibility
/** @emoji 📐 Left/right side panel open state for {@link Platform} and product shells. */
export interface PanelVisibility {
	readonly leftSidePanel: boolean;
	readonly rightSidePanel: boolean;
}

/** @emoji 🪟 Default open side panels for product/playground shells (glass workbench + details). */
export const PRODUCT_SHELL_DEFAULT_PANEL_VISIBILITY: PanelVisibility = {
	leftSidePanel: true,
	rightSidePanel: true,
};

/** @emoji 📐 Resolves initial panel visibility: prop override, then platform spec; default both hidden. */
export function resolveInitialPanelVisibility(
	prop?: Partial<PanelVisibility>,
	platform?: Pick<Platform, "initialPanelVisibility">,
): PanelVisibility {
	return {
		leftSidePanel: prop?.leftSidePanel ?? platform?.initialPanelVisibility?.leftSidePanel ?? false,
		rightSidePanel: prop?.rightSidePanel ?? platform?.initialPanelVisibility?.rightSidePanel ?? false,
	};
}
//#endregion 🔖PanelVisibility

//#region 🔖Navigation
/** @emoji 🧭 One navigable destination (label + URI) for breadcrumb trails and separator alternatives. */
export interface NavigationDestination {
	readonly id: string;
	readonly label: unknown;
	readonly uri: string;
}

/** @emoji 🧭 One breadcrumb level: chosen node plus sibling alternatives for the following separator dropdown. */
export interface NavigationLevel {
	readonly node: NavigationDestination;
	readonly alternatives: readonly NavigationDestination[];
}
//#endregion 🔖Navigation

//#region 🔖Platform
/** @emoji 🖥️ Root shell: apps, URI chrome, panel toggles, and shared {@link CommandBus}. */
export class Platform {
	readonly commandBus = new CommandBus();
	private readonly listeners = new Set<PlatformSubscriber>();
	private readonly chromeListeners = new Set<PlatformSubscriber>();
	readonly apps: BaseAppRuntime[] = [];
	activeAppId = "";
	generation = 0;
	chromeGeneration = 0;
	uri = "/";
	canGoBack = false;
	canGoForward = false;
	canGoUp = false;
	onNavigate?: (uri: string) => void;
	/** @emoji 🔗 Product hook: apply URI to platform state (active app, stores) when navigation changes. */
	applyUri?: (uri: string) => void;
	/** @emoji 🧭 Optional navigation trail for the current URI; default is path segments without alternatives. */
	navigation?: (uri: string) => readonly NavigationLevel[];
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
	panelVisibility: PanelVisibility = { leftSidePanel: false, rightSidePanel: false };
	initialPanelVisibility?: PanelVisibility;
	readonly id: string;
	readonly name: string;

	constructor(spec?: PlatformSpec) {
		this.id = spec?.id ?? "";
		this.name = spec?.name ?? "";
		if (spec?.initialPanelVisibility) {
			this.initialPanelVisibility = { ...spec.initialPanelVisibility };
			this.panelVisibility = { ...spec.initialPanelVisibility };
		}
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
		for (const app of this.apps) {
			app.invalidateResolvedState();
		}
		for (const listener of this.listeners) listener();
	}

	/** @emoji 🪟 Bumps shell chrome only (panels, active app/mode) without waking data subscribers. */
	notifyChrome(): void {
		this.chromeGeneration++;
		for (const listener of this.chromeListeners) listener();
	}

	subscribe(listener: PlatformSubscriber): () => void {
		this.listeners.add(listener);
		return () => this.listeners.delete(listener);
	}

	subscribeChrome(listener: PlatformSubscriber): () => void {
		this.chromeListeners.add(listener);
		return () => this.chromeListeners.delete(listener);
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
		if (this.activeAppId === id) return;
		this.activeAppId = id;
		this.notifyChrome();
	}

	setPanelVisibility(next: PanelVisibility): void {
		if (!this.assignPanelVisibility(next)) return;
		this.notifyChrome();
	}

	/** @emoji 🪟 Updates {@link Platform.panelVisibility} without notifying subscribers (local React shell owns UI). */
	assignPanelVisibility(next: PanelVisibility): boolean {
		const left = next.leftSidePanel;
		const right = next.rightSidePanel;
		if (this.panelVisibility.leftSidePanel === left && this.panelVisibility.rightSidePanel === right) return false;
		this.panelVisibility = { leftSidePanel: left, rightSidePanel: right };
		return true;
	}

	private readonly componentsBySurfaceId = new Map<string, SurfaceComponent>();

	/** @emoji 🧩 Registers a render-agnostic surface component keyed by {@link SurfaceComponent.surfaceId}. */
	registerComponent(component: SurfaceComponent): void {
		this.componentsBySurfaceId.set(component.surfaceId, component);
		this.notify();
	}

	/** @emoji 🔍 Resolves a registered surface component by id. */
	getComponent(surfaceId: string): SurfaceComponent | undefined {
		return this.componentsBySurfaceId.get(surfaceId);
	}

	/** @emoji 🧹 Removes a surface component (tests / hot reload). */
	unregisterComponent(surfaceId: string): void {
		if (!this.componentsBySurfaceId.delete(surfaceId)) return;
		this.notify();
	}
}
//#endregion 🔖Platform

//#region 🔖SurfaceComponent
/** @emoji 🧩 Minimal surface component handle stored on {@link Platform}. */
export interface SurfaceComponent extends Store<unknown> {
	readonly surfaceId: string;
	readonly componentKind: string;
}
//#endregion 🔖SurfaceComponent

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
			const base: AppTools = { view: [{ id: "a", kind: "button", iconId: "circle-dot", label: "A" }] };
			const ext: AppTools = { view: [{ id: "b", kind: "button", iconId: "circle-dot", label: "B" }] };
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

	describe("display panel types", () => {
		it("LEFT_PANEL_KINDS places display left of workbench", () => {
			expect(LEFT_PANEL_KINDS).toEqual(["display", "workbench", "overview"]);
		});

		it("createWindowLayout carries template and instance ids", () => {
			const node = createWindowLayout("kind-a", "Title", { instanceId: "inst-1", templateId: "top" });
			expect(node.instanceId).toBe("inst-1");
			expect(node.templateId).toBe("top");
		});

		it("NamedLayoutStore persists user layouts via StoragePort", () => {
			const memory = new Map<string, string>();
			const storage: StoragePort = {
				get: (key) => memory.get(key) ?? null,
				set: (key, value) => {
					memory.set(key, value);
				},
				remove: (key) => {
					memory.delete(key);
				},
			};
			const store = new NamedLayoutStore("app-a", storage);
			const layout = createStackLayout(["main"]);
			store.save(createNamedLayout("user-1", "Mine", layout, "user"));
			expect(store.getSnapshot()).toHaveLength(1);
			const reloaded = new NamedLayoutStore("app-a", storage);
			expect(reloaded.getSnapshot()[0]?.label).toBe("Mine");
			reloaded.remove("user-1");
			expect(reloaded.getSnapshot()).toHaveLength(0);
		});
	});

	describe("kind weight helpers", () => {
		it("normalizeKindWeightGroup keeps group sum at 1", () => {
			const base = { a: 0.5, b: 0.3, c: 0.2 };
			const next = normalizeKindWeightGroup(base, "a", 0.8);
			const sum = Object.values(next).reduce((acc, v) => acc + v, 0);
			expect(sum).toBeCloseTo(1, 5);
			expect(next.a).toBeCloseTo(0.8, 5);
		});

		it("weightedOrder favors high weights at the front with fixed rng", () => {
			const items = ["a", "b", "c"];
			const ordered = weightedOrder(items, (id) => (id === "a" ? 100 : 1), () => 0);
			expect(ordered[0]).toBe("a");
		});

		it("defaultPuzzleKindWeights sums to 1 and encodes tambour/base/capital/capsule ratios", () => {
			const weights = defaultPuzzleKindWeights(["Base", "Capital", "Tambour", "Capsule J"]);
			const sum = Object.values(weights).reduce((acc, v) => acc + v, 0);
			expect(sum).toBeCloseTo(1, 8);
			const base = weights.Base ?? 0;
			const capital = weights.Capital ?? 0;
			const tambour = weights.Tambour ?? 0;
			const capsule = weights["Capsule J"] ?? 0;
			expect(tambour / base).toBeCloseTo(15, 5);
			expect(tambour / capital).toBeCloseTo(10, 5);
			expect(capsule / tambour).toBeCloseTo(8, 5);
		});

		it("syncKindWeightMap uses default puzzle weights when existing group is empty", () => {
			const synced = syncKindWeightMap(["Base", "Tambour"], {});
			expect(synced.Base! / synced.Tambour!).toBeCloseTo(1 / 15, 5);
		});

		it("puzzleKindSuggestionRelativeWeight maps handle and vortex catalog ids", () => {
			expect(puzzleKindSuggestionRelativeWeight("door capsule right")).toBe(PUZZLE_KIND_SUGGESTION_WEIGHT_CAPSULE);
			expect(puzzleKindSuggestionRelativeWeight("tambour circular top")).toBe(PUZZLE_KIND_SUGGESTION_WEIGHT_TAMBOUR);
			expect(puzzleKindSuggestionRelativeWeight("roof rectangular top")).toBe(PUZZLE_KIND_SUGGESTION_WEIGHT_CAPITAL);
			expect(puzzleKindSuggestionRelativeWeight("core rectangular bottom")).toBe(PUZZLE_KIND_SUGGESTION_WEIGHT_BASE);
		});
	});

	describe("Controller stores", () => {
		it("provideStore registers and disposes owned stores", () => {
			class TCtrl extends Controller {
				override run(): void {}
			}
			class CountStore extends Store<number> {
				value = 0;
				override getSnapshot(): number {
					return this.value;
				}
				disposed = false;
				override dispose(): void {
					this.disposed = true;
					super.dispose();
				}
			}
			const bus = new CommandBus();
			const ctrl = new TCtrl("c", bus, () => {});
			const store = new CountStore();
			ctrl.provideStore("count", store);
			expect(ctrl.getStore<number>("count")).toBe(store);
			expect(ctrl.stores.size).toBe(1);
			ctrl.dispose();
			expect(store.disposed).toBe(true);
		});

		it("revokeStore disposes and removes a store", () => {
			class TCtrl extends Controller {
				override run(): void {}
			}
			class CountStore extends Store<number> {
				value = 0;
				override getSnapshot(): number {
					return this.value;
				}
				disposed = false;
				override dispose(): void {
					this.disposed = true;
					super.dispose();
				}
			}
			const bus = new CommandBus();
			const ctrl = new TCtrl("c", bus, () => {});
			const store = new CountStore();
			ctrl.provideStore("count", store);
			(ctrl as { revokeStore(id: string): void }).revokeStore("count");
			expect(ctrl.getStore<number>("count")).toBeUndefined();
			expect(store.disposed).toBe(true);
			ctrl.dispose();
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

		it("returns a stable getSnapshot reference until focus changes", () => {
			const store = new AppPointerFocusStore<string>();
			const first = store.getSnapshot();
			expect(store.getSnapshot()).toBe(first);
			store.setHoverFromSource("canvas", "y");
			const second = store.getSnapshot();
			expect(second).not.toBe(first);
			expect(store.getSnapshot()).toBe(second);
		});
	});

	describe("Platform", () => {
		it("constructs from PlatformSpec metadata", () => {
			const platform = new Platform({ id: "demo", name: "Demo", defaultActiveAppId: "home" });
			expect(platform.id).toBe("demo");
			expect(platform.activeAppId).toBe("home");
		});

		it("defaults side panels hidden", () => {
			const platform = new Platform({ id: "demo", name: "Demo" });
			expect(platform.panelVisibility).toEqual({ leftSidePanel: false, rightSidePanel: false });
		});

		it("applies initialPanelVisibility from PlatformSpec", () => {
			const platform = new Platform({
				id: "demo",
				name: "Demo",
				initialPanelVisibility: { leftSidePanel: true, rightSidePanel: false },
			});
			expect(platform.initialPanelVisibility).toEqual({ leftSidePanel: true, rightSidePanel: false });
			expect(platform.panelVisibility).toEqual({ leftSidePanel: true, rightSidePanel: false });
		});

		it("notifyChrome does not bump data generation", () => {
			const platform = new Platform({ id: "demo", name: "Demo" });
			const dataGen = platform.generation;
			const chromeGen = platform.chromeGeneration;
			platform.setPanelVisibility({ leftSidePanel: true, rightSidePanel: false });
			expect(platform.generation).toBe(dataGen);
			expect(platform.chromeGeneration).toBe(chromeGen + 1);
			platform.setActiveAppId("missing");
			expect(platform.generation).toBe(dataGen);
		});

		it("notify bumps data generation but not chrome generation", () => {
			const platform = new Platform({ id: "demo", name: "Demo" });
			const chromeGen = platform.chromeGeneration;
			platform.notify();
			expect(platform.generation).toBe(1);
			expect(platform.chromeGeneration).toBe(chromeGen);
		});

		it("setPanelVisibility is a no-op when visibility unchanged", () => {
			const platform = new Platform({
				id: "demo",
				name: "Demo",
				initialPanelVisibility: { leftSidePanel: true, rightSidePanel: false },
			});
			const chromeGen = platform.chromeGeneration;
			platform.setPanelVisibility({ leftSidePanel: true, rightSidePanel: false });
			expect(platform.chromeGeneration).toBe(chromeGen);
		});

		it("assignPanelVisibility updates state without notifying subscribers", () => {
			const platform = new Platform({ id: "demo", name: "Demo" });
			const dataGen = platform.generation;
			const chromeGen = platform.chromeGeneration;
			let dataCalls = 0;
			let chromeCalls = 0;
			platform.subscribe(() => {
				dataCalls++;
			});
			platform.subscribeChrome(() => {
				chromeCalls++;
			});
			expect(platform.assignPanelVisibility({ leftSidePanel: true, rightSidePanel: false })).toBe(true);
			expect(platform.panelVisibility).toEqual({ leftSidePanel: true, rightSidePanel: false });
			expect(platform.generation).toBe(dataGen);
			expect(platform.chromeGeneration).toBe(chromeGen);
			expect(dataCalls).toBe(0);
			expect(chromeCalls).toBe(0);
			expect(platform.assignPanelVisibility({ leftSidePanel: true, rightSidePanel: false })).toBe(false);
		});

		it("registers render-agnostic surface components by surface id", () => {
			const platform = new Platform({ id: "demo", name: "Demo" });
			let snapshot = "a";
			platform.registerComponent({
				surfaceId: "surface/demo",
				componentKind: "table",
				subscribe: (listener) => {
					listener();
					return () => undefined;
				},
				getSnapshot: () => snapshot,
				dispose: () => undefined,
			});
			expect(platform.getComponent("surface/demo")?.getSnapshot()).toBe("a");
			snapshot = "b";
			expect(platform.getComponent("surface/demo")?.getSnapshot()).toBe("b");
			platform.unregisterComponent("surface/demo");
			expect(platform.getComponent("surface/demo")).toBeUndefined();
		});
	});

	describe("resolveInitialPanelVisibility", () => {
		it("prefers prop override over platform spec", () => {
			const platform = new Platform({
				id: "demo",
				name: "Demo",
				initialPanelVisibility: { leftSidePanel: true, rightSidePanel: true },
			});
			expect(resolveInitialPanelVisibility({ leftSidePanel: false }, platform)).toEqual({
				leftSidePanel: false,
				rightSidePanel: true,
			});
		});

		it("defaults both panels hidden when unset", () => {
			expect(resolveInitialPanelVisibility(undefined, new Platform({ id: "d", name: "D" }))).toEqual({
				leftSidePanel: false,
				rightSidePanel: false,
			});
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
