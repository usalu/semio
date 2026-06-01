// #region 🧲Header
// 💻 puzzle/2d/play/index.ts — Puzzle 2D play shell on `@framework/playground/core`: declarative bodies, LOD measures, toolbar tools (no React).
// #endregion 🧲Header

import {
	CommandBus,
	Controller,
	Playground,
	registerWindowBody,
	Platform,
	AppRuntime,
	ModeRuntime,
	WindowKindRuntime,
	buildPuzzle2dWindowBody,
	buildPlaygroundKindToggleTools,
	createWindowLayout,
	playgroundTreePanelRootItems,
	type AppTools,
	type ToolItem,
	type WindowBodyViewContext,
	type CommandDescriptor,
	type WindowEngagement,
	type WindowMeasure,
	type UiNode,
	type UiTreeItemNode,
	type UiTreeNode,
	type WindowLayout,
	enforcePlaygroundWindowEngagementInput,
	windowEngagementsEqual,
} from "@framework/playground/core";

import nakaginFixtureJson from "../fixture/nakagin-capsule-tower.2d.json";
import {
	DEFAULT_KIND_CATALOG_BUNDLE,
	PUZZLE_2D_LOD_MODE_AUTOMATIC,
	fixtureMetaKindCatalogBundle,
	mergeKindCatalogBundleByRowId,
	puzzle2dFixtureEdgeDisplayLabel,
	puzzle2dFixtureHandleDisplayLabel,
	puzzle2dFixtureNodeDisplayDescription,
	puzzle2dFixtureNodeDisplayLabel,
	puzzle2dLodAutomaticSelectLabel,
	isPuzzle2dDrawLodKind,
	parsePuzzle2dFixtureV1,
	type KindCatalogBundle,
	type Puzzle2dDrawLodKind,
	type Puzzle2dFixtureNodeV1,
	type Puzzle2dFixtureV1,
	type Puzzle2dLodModeKind,
	type Puzzle2dSelectionMethod,
	type Puzzle2dSelectionMode,
	type Puzzle2dSelectionTargets,
} from "../react/index.tsx";

//#region 🔖Ids
export type Puzzle2dPlayPaneId = "2d-overview" | "2d-detail" | "2d-selection";

export const PUZZLE_2D_PLAY_APP_ID = "puzzle-2d-play";
export const PUZZLE_2D_PLAY_CONTROLLER_ID = "puzzle-2d-play";
export const PUZZLE_2D_PLAY_SURFACE_ID = "puzzle.2d.play/v1";

export const PUZZLE_2D_PLAY_BODY_KEY_OVERVIEW = "puzzle.2d.play.overview";
export const PUZZLE_2D_PLAY_BODY_KEY_DETAIL = "puzzle.2d.play.detail";
export const PUZZLE_2D_PLAY_BODY_KEY_SELECTION = "puzzle.2d.play.selection";

export const PUZZLE_2D_PLAY_LOD_TIERS: Puzzle2dDrawLodKind[] = ["minimap", "overview", "compact", "normal", "detail", "micro"];

export function puzzle2dPlayLodTierMenuLabel(tier: Puzzle2dDrawLodKind): string {
	return tier.charAt(0).toUpperCase() + tier.slice(1);
}

export const PUZZLE_2D_PLAY_HIERARCHY_TAB_ID = "puzzle-2d-play-hierarchy";

const PUZZLE_2D_PLAY_WINDOW_SPECS: { readonly pane: Puzzle2dPlayPaneId; readonly label: string; readonly bodyKey: string }[] = [
	{ pane: "2d-overview", label: "Overview", bodyKey: PUZZLE_2D_PLAY_BODY_KEY_OVERVIEW },
	{ pane: "2d-detail", label: "Zoom", bodyKey: PUZZLE_2D_PLAY_BODY_KEY_DETAIL },
	{ pane: "2d-selection", label: "Selection", bodyKey: PUZZLE_2D_PLAY_BODY_KEY_SELECTION },
];

function puzzle2dPlayCmd(command: string, args?: Record<string, unknown>): CommandDescriptor {
	return { controllerId: PUZZLE_2D_PLAY_CONTROLLER_ID, command, args: args as never };
}

/** @emoji ⌨️ Lowercase PascalCase engagement token for command matching (mirrors ui {@link normalizeEngagementCommandText}). */
function puzzle2dPlayEngagementCommandToken(text: string): string {
	const words = text
		.replace(/[^a-zA-Z0-9]+/g, " ")
		.trim()
		.split(/\s+/)
		.filter(Boolean)
		.flatMap((word) => word.split(/(?=[A-Z])/))
		.filter(Boolean);
	return words.map((word) => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase()).join("").toLowerCase();
}

export const PUZZLE_2D_PLAY_PACKAGE_ROOT = import.meta.url;

export const PUZZLE_2D_PLAY_DEFAULT_FIXTURE: Puzzle2dFixtureV1 =
	parsePuzzle2dFixtureV1(nakaginFixtureJson as unknown) ?? (nakaginFixtureJson as Puzzle2dFixtureV1);

export const PUZZLE_2D_PLAY_LAYOUT: WindowLayout = {
	root: {
		kind: "row",
		children: [
			{
				kind: "stack",
				size: 50,
				children: [createWindowLayout("2d-overview", "Overview")],
			},
			{
				kind: "column",
				size: 50,
				children: [
					{ kind: "stack", size: 50, children: [createWindowLayout("2d-detail", "Zoom")] },
					{ kind: "stack", size: 50, children: [createWindowLayout("2d-selection", "Selection")] },
				],
			},
		],
	},
};
//#endregion 🔖Ids

//#region 🔖Puzzle2dPlayHierarchy
function puzzle2dFixtureHandleToNodeId(fixture: Puzzle2dFixtureV1): ReadonlyMap<string, string> {
	const out = new Map<string, string>();
	for (const node of fixture.nodes) {
		for (const handle of node.handles) {
			out.set(handle.id, node.id);
		}
	}
	return out;
}

function puzzle2dFixtureChildrenByNodeId(fixture: Puzzle2dFixtureV1): ReadonlyMap<string, readonly string[]> {
	const handleToNode = puzzle2dFixtureHandleToNodeId(fixture);
	const out = new Map<string, string[]>();
	for (const edge of fixture.edges) {
		const parentId = handleToNode.get(edge.source);
		const childId = handleToNode.get(edge.target);
		if (!parentId || !childId || parentId === childId) {
			continue;
		}
		const next = out.get(parentId) ?? [];
		next.push(childId);
		out.set(parentId, next);
	}
	for (const [parentId, childIds] of out) {
		out.set(
			parentId,
			[...new Set(childIds)].sort((a, b) => a.localeCompare(b)),
		);
	}
	return out;
}

function puzzle2dFixtureRootNodeIds(fixture: Puzzle2dFixtureV1, childrenByParent: ReadonlyMap<string, readonly string[]>): readonly string[] {
	const explicitRoots = fixture.nodes.filter((node) => node.root).map((node) => node.id);
	if (explicitRoots.length > 0) {
		return [...new Set(explicitRoots)].sort((a, b) => a.localeCompare(b));
	}
	const childIds = new Set<string>();
	for (const ids of childrenByParent.values()) {
		for (const id of ids) {
			childIds.add(id);
		}
	}
	const inferred = fixture.nodes.map((node) => node.id).filter((id) => !childIds.has(id));
	return inferred.length > 0 ? inferred.sort((a, b) => a.localeCompare(b)) : fixture.nodes.map((node) => node.id).sort((a, b) => a.localeCompare(b));
}

/** @emoji 🌳 Maps committed graph selection ids to workbench hierarchy tree item ids. */
export function puzzle2dPlayHierarchyTreeSelectedIds(fixture: Puzzle2dFixtureV1, graphSelectionIds: readonly string[]): string[] {
	const out: string[] = [];
	for (const id of graphSelectionIds) {
		if (fixture.nodes.some((node) => node.id === id)) {
			out.push(`puzzle-2d-play-hierarchy.node.${id}`);
			continue;
		}
		if (fixture.edges.some((edge) => edge.id === id)) {
			out.push(`puzzle-2d-play-hierarchy.edge.${id}`);
			continue;
		}
		if (fixture.nodes.some((node) => node.handles.some((handle) => handle.id === id))) {
			out.push(`puzzle-2d-play-hierarchy.handle.${id}`);
		}
	}
	return out;
}

/** @emoji 🌳 Resolves a hierarchy tree row id back to a graph object id (node, handle, or edge). */
export function puzzle2dPlayHierarchyGraphIdFromTreeItemId(treeItemId: string): string | null {
	const handlePrefix = "puzzle-2d-play-hierarchy.handle.";
	const edgePrefix = "puzzle-2d-play-hierarchy.edge.";
	const nodePrefix = "puzzle-2d-play-hierarchy.node.";
	if (treeItemId.startsWith(handlePrefix)) {
		return treeItemId.slice(handlePrefix.length);
	}
	if (treeItemId.startsWith(edgePrefix)) {
		return treeItemId.slice(edgePrefix.length);
	}
	if (treeItemId.startsWith(nodePrefix) && !treeItemId.includes(".handles")) {
		return treeItemId.slice(nodePrefix.length);
	}
	return null;
}

export type Puzzle2dPlayHierarchyBuildOptions = {
	/** @emoji 🌳 When true, omit per-item `isSelected`; drive highlight via Tree `selectedIds` instead. */
	readonly omitItemSelection?: boolean;
	/** @emoji 🖱️ Optional graph-id hover sink for hierarchy row pointer enter/leave. */
	readonly onHover?: (id: string | null) => void;
};

function puzzle2dPlayHierarchyHoverHandlers(
	onHover: ((id: string | null) => void) | undefined,
	graphId: string,
): Pick<UiTreeItemNode, "onPointerEnter" | "onPointerLeave"> {
	if (!onHover) {
		return {};
	}
	return {
		onPointerEnter: () => onHover(graphId),
		onPointerLeave: () => onHover(null),
	};
}

function buildPuzzle2dFixtureNodeHierarchyItem(
	fixture: Puzzle2dFixtureV1,
	kindCatalogs: KindCatalogBundle,
	nodeId: string,
	childrenByParent: ReadonlyMap<string, readonly string[]>,
	selectedIds: ReadonlySet<string>,
	onSelect: (id: string) => void,
	visiting: Set<string>,
	omitItemSelection: boolean,
	onHover?: (id: string | null) => void,
): UiTreeItemNode | null {
	if (visiting.has(nodeId)) {
		return null;
	}
	const node = fixture.nodes.find((row) => row.id === nodeId);
	if (!node) {
		return null;
	}
	visiting.add(nodeId);
	const handleItems: UiTreeItemNode[] = node.handles.map((handle) => ({
		id: `puzzle-2d-play-hierarchy.handle.${handle.id}`,
		label: puzzle2dFixtureHandleDisplayLabel(handle, kindCatalogs),
		...(omitItemSelection ? {} : { isSelected: selectedIds.has(handle.id) }),
		onClick: () => onSelect(handle.id),
		...puzzle2dPlayHierarchyHoverHandlers(onHover, handle.id),
	}));
	const handlesGroup: UiTreeItemNode = {
		id: `puzzle-2d-play-hierarchy.node.${nodeId}.handles`,
		label: "Handles",
		defaultOpen: true,
		items: handleItems.length ? handleItems : [{ id: `puzzle-2d-play-hierarchy.node.${nodeId}.handles.empty`, label: "(none)" }],
	};
	const childItems: UiTreeItemNode[] = [];
	for (const childId of childrenByParent.get(nodeId) ?? []) {
		const childItem = buildPuzzle2dFixtureNodeHierarchyItem(fixture, kindCatalogs, childId, childrenByParent, selectedIds, onSelect, visiting, omitItemSelection, onHover);
		if (childItem) {
			childItems.push(childItem);
		}
	}
	visiting.delete(nodeId);
	return {
		id: `puzzle-2d-play-hierarchy.node.${nodeId}`,
		label: puzzle2dFixtureNodeDisplayLabel(node, kindCatalogs),
		description: puzzle2dFixtureNodeDisplayDescription(node, kindCatalogs),
		...(omitItemSelection ? {} : { isSelected: selectedIds.has(nodeId) }),
		defaultOpen: true,
		onClick: () => onSelect(nodeId),
		...puzzle2dPlayHierarchyHoverHandlers(onHover, nodeId),
		items: [handlesGroup, ...childItems],
	};
}

/** @emoji 🌳 Maps committed graph hover ids to workbench hierarchy tree item ids. */
export function puzzle2dPlayHierarchyTreeHighlightedIds(fixture: Puzzle2dFixtureV1, graphHoverId: string | null): readonly string[] {
	if (!graphHoverId) {
		return [];
	}
	return puzzle2dPlayHierarchyTreeSelectedIds(fixture, [graphHoverId]);
}

/** @emoji 🌳 Nested workbench tree: Puzzle 2D → nodes (graph) → handles; flat edges group. */
export function buildPuzzle2dPlayHierarchySections(
	fixture: Puzzle2dFixtureV1,
	selectionIds: readonly string[],
	onSelect: (id: string) => void,
	kindCatalogs: KindCatalogBundle = mergeKindCatalogBundleByRowId(DEFAULT_KIND_CATALOG_BUNDLE, fixtureMetaKindCatalogBundle(fixture) ?? {}),
	options?: Puzzle2dPlayHierarchyBuildOptions,
): UiTreeNode {
	const omitItemSelection = options?.omitItemSelection === true;
	const onHover = options?.onHover;
	const selectedIds = omitItemSelection ? new Set<string>() : new Set(selectionIds);
	const childrenByParent = puzzle2dFixtureChildrenByNodeId(fixture);
	const rootIds = puzzle2dFixtureRootNodeIds(fixture, childrenByParent);
	const visiting = new Set<string>();
	const nodeItems: UiTreeItemNode[] = [];
	for (const rootId of rootIds) {
		const item = buildPuzzle2dFixtureNodeHierarchyItem(fixture, kindCatalogs, rootId, childrenByParent, selectedIds, onSelect, visiting, omitItemSelection, onHover);
		if (item) {
			nodeItems.push(item);
		}
	}
	const nodesGroup: UiTreeItemNode = {
		id: "puzzle-2d-play-hierarchy.nodes",
		label: "Nodes",
		defaultOpen: true,
		items: nodeItems.length ? nodeItems : [{ id: "puzzle-2d-play-hierarchy.nodes.empty", label: "(none)" }],
	};
	const edgeItems: UiTreeItemNode[] = fixture.edges.map((edge) => ({
		id: `puzzle-2d-play-hierarchy.edge.${edge.id}`,
		label: puzzle2dFixtureEdgeDisplayLabel(edge, fixture, kindCatalogs),
		...(omitItemSelection ? {} : { isSelected: selectedIds.has(edge.id) }),
		onClick: () => onSelect(edge.id),
		...puzzle2dPlayHierarchyHoverHandlers(onHover, edge.id),
	}));
	const edgesGroup: UiTreeItemNode = {
		id: "puzzle-2d-play-hierarchy.edges",
		label: "Edges",
		defaultOpen: true,
		items: edgeItems.length ? edgeItems : [{ id: "puzzle-2d-play-hierarchy.edges.empty", label: "(none)" }],
	};
	const puzzle2dRoot: UiTreeItemNode = {
		id: "puzzle-2d-play-hierarchy.puzzle2d",
		label: "Puzzle 2D",
		defaultOpen: true,
		items: [nodesGroup, edgesGroup],
	};
	return playgroundTreePanelRootItems("puzzle-2d-play-hierarchy.root", [puzzle2dRoot]) as UiTreeNode;
}
//#endregion 🔖Puzzle2dPlayHierarchy

//#region 🔖Controller
const PUZZLE_2D_PLAY_TARGET_KINDS = ["nodes", "edges", "handles"] as const;
type Puzzle2dPlayTargetKind = (typeof PUZZLE_2D_PLAY_TARGET_KINDS)[number];

function puzzle2dPlayTargetLabel(kind: Puzzle2dPlayTargetKind): string {
	if (kind === "nodes") return "Nodes";
	if (kind === "edges") return "Edges";
	return "Handles";
}

/** @emoji 🧰 Snapshot read by {@link buildPuzzle2dPlayToolbarTools} (host-owned play state). */
export interface Puzzle2dPlayToolbarState {
	readonly puzzle2dSelectionMethod: Puzzle2dSelectionMethod;
	readonly puzzle2dSelectionMode: Puzzle2dSelectionMode;
	readonly puzzle2dSelectionTargets: Puzzle2dSelectionTargets;
	readonly puzzle2dGridSnapEnabled: boolean;
	readonly puzzle2dRedrawPlaying: boolean;
}

/** @emoji 🔗 Host bridge: toolbar snapshot + commands that need React/fixture context. */
export interface Puzzle2dPlayHostBridge {
	getToolbarState(): Puzzle2dPlayToolbarState;
	runHostCommand(command: string, args?: unknown): void;
}

/** @emoji 🧰 Playground {@link AppTools} for puzzle 2d play (selection, filter, view, create, actions). */
export function buildPuzzle2dPlayToolbarTools(state: Puzzle2dPlayToolbarState, controllerId: string): AppTools {
	const targetRecord: Record<Puzzle2dPlayTargetKind, boolean> = {
		nodes: state.puzzle2dSelectionTargets.nodes,
		edges: state.puzzle2dSelectionTargets.edges,
		handles: state.puzzle2dSelectionTargets.handles,
	};
	const selectionTools: ToolItem[] = [
		{
			id: "puzzle2d.select.rectangle",
			kind: "toggle",
			text: "Rectangle",
			order: 0,
			pressed: state.puzzle2dSelectionMethod === "rectangle",
			controllerId,
			command: "setSelectionMethod",
			args: { method: "rectangle" },
		},
		{
			id: "puzzle2d.select.lasso",
			kind: "toggle",
			text: "Lasso",
			order: 1,
			pressed: state.puzzle2dSelectionMethod === "lasso",
			controllerId,
			command: "setSelectionMethod",
			args: { method: "lasso" },
		},
		{
			id: "puzzle2d.select.mode.default",
			kind: "toggle",
			text: "Default",
			order: 2,
			pressed: state.puzzle2dSelectionMode === "default",
			controllerId,
			command: "setSelectionMode",
			args: { mode: "default" },
		},
		{
			id: "puzzle2d.select.mode.additive",
			kind: "toggle",
			text: "Add",
			order: 3,
			pressed: state.puzzle2dSelectionMode === "additive",
			controllerId,
			command: "setSelectionMode",
			args: { mode: "additive" },
		},
		{
			id: "puzzle2d.select.mode.subtractive",
			kind: "toggle",
			text: "Subtract",
			order: 4,
			pressed: state.puzzle2dSelectionMode === "subtractive",
			controllerId,
			command: "setSelectionMode",
			args: { mode: "subtractive" },
		},
		{
			id: "puzzle2d.select.mode.invertive",
			kind: "toggle",
			text: "Invert",
			order: 5,
			pressed: state.puzzle2dSelectionMode === "invertive",
			controllerId,
			command: "setSelectionMode",
			args: { mode: "invertive" },
		},
		...buildPlaygroundKindToggleTools("selection", PUZZLE_2D_PLAY_TARGET_KINDS, puzzle2dPlayTargetLabel, targetRecord, controllerId, "toggleSelectionTarget"),
		{
			id: "puzzle2d.selection.clear",
			kind: "button",
			label: "Clear",
			order: 20,
			controllerId,
			command: "clearSelection",
		},
	];
	return {
		selection: selectionTools,
		view: [
			{
				id: "puzzle2d.grid.snap",
				kind: "toggle",
				text: "Grid snap",
				order: 0,
				pressed: state.puzzle2dGridSnapEnabled,
				controllerId,
				command: "toggleGridSnap",
			},
		],
		create: [
			{ id: "puzzle2d.create.circle", kind: "button", label: "Circle", order: 0, controllerId, command: "appendCircle" },
			{ id: "puzzle2d.create.rectangle", kind: "button", label: "Rectangle", order: 1, controllerId, command: "appendRectangle" },
		],
		actions: [
			{
				id: "puzzle2d.redraw.play",
				kind: "toggle",
				text: "Redraw",
				order: 0,
				pressed: state.puzzle2dRedrawPlaying,
				controllerId,
				command: "toggleRedrawPlaying",
			},
			{ id: "puzzle2d.redraw.handles", kind: "button", label: "Handles", title: "Redraw handles once", order: 1, controllerId, command: "redrawHandlesOnce" },
		],
	};
}

/** @emoji 🎛 Puzzle 2d play shell controller: per-pane LOD modes + playground toolbar tools. */
export class Puzzle2dPlayShellController extends Controller {
	readonly mainMode = new ModeRuntime("main", "Puzzle 2D", undefined);
	private lodModeByPane: Record<Puzzle2dPlayPaneId, Puzzle2dLodModeKind>;
	private effectiveLodByPane: Record<Puzzle2dPlayPaneId, Puzzle2dDrawLodKind>;
	private engagementInputByPane: Record<Puzzle2dPlayPaneId, string>;
	private hostBridge: Puzzle2dPlayHostBridge | null = null;
	private readonly hostChromeNotify: () => void;

	constructor(commandBus: CommandBus, hostNotify: () => void, hostChromeNotify: () => void) {
		super(PUZZLE_2D_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.hostChromeNotify = hostChromeNotify;
		this.lodModeByPane = {
			"2d-detail": PUZZLE_2D_LOD_MODE_AUTOMATIC,
			"2d-overview": PUZZLE_2D_LOD_MODE_AUTOMATIC,
			"2d-selection": PUZZLE_2D_LOD_MODE_AUTOMATIC,
		};
		this.effectiveLodByPane = {
			"2d-detail": "normal",
			"2d-overview": "normal",
			"2d-selection": "normal",
		};
		this.engagementInputByPane = {
			"2d-detail": "",
			"2d-overview": "",
			"2d-selection": "",
		};
		this.rebuildShellMode();
	}

	private windowEngagementForPane(pane: Puzzle2dPlayPaneId): WindowEngagement {
		return {
			input: {
				id: "engagement-input",
				value: this.engagementInputByPane[pane],
				placeholder: "Command",
				onChange: puzzle2dPlayCmd("engagementInput", { pane }),
				onSubmit: puzzle2dPlayCmd("engagementSubmit", { pane }),
				onAbort: puzzle2dPlayCmd("engagementAbort", { pane }),
			},
			possibleEngagements: [
				{ id: "puzzle2d.select.rectangle", label: "Rectangle", command: puzzle2dPlayCmd("engagementPossibleSelect", { pane, possibleId: "puzzle2d.select.rectangle" }) },
				{ id: "puzzle2d.select.lasso", label: "Lasso", command: puzzle2dPlayCmd("engagementPossibleSelect", { pane, possibleId: "puzzle2d.select.lasso" }) },
				{ id: "puzzle2d.create.circle", label: "Circle", command: puzzle2dPlayCmd("engagementPossibleSelect", { pane, possibleId: "puzzle2d.create.circle" }) },
				{ id: "puzzle2d.create.rectangle", label: "RectangleShape", command: puzzle2dPlayCmd("engagementPossibleSelect", { pane, possibleId: "puzzle2d.create.rectangle" }) },
				{ id: "puzzle2d.selection.clear", label: "Clear", command: puzzle2dPlayCmd("engagementPossibleSelect", { pane, possibleId: "puzzle2d.selection.clear" }) },
			],
		};
	}

	private syncWindowEngagementForPane(pane: Puzzle2dPlayPaneId): void {
		const existing = this.mainMode.windowKinds.find((wk) => wk.id === pane);
		const next = this.windowEngagementForPane(pane);
		if (existing) {
			if (windowEngagementsEqual(existing.engagement, next)) {
				return;
			}
			existing.engagement = next;
			this.mainMode.windowKinds = [...this.mainMode.windowKinds];
		} else {
			this.rebuildShellMode();
		}
		this.emit();
	}

	private applyEngagementCommand(pane: Puzzle2dPlayPaneId, possibleIdOrText: string): boolean {
		const token = puzzle2dPlayEngagementCommandToken(possibleIdOrText);
		const runHost = (command: string, args?: unknown) => {
			this.hostBridge?.runHostCommand(command, args);
		};
		if (possibleIdOrText === "puzzle2d.select.rectangle" || token === "rectangle") {
			runHost("setSelectionMethod", { method: "rectangle" });
			return true;
		}
		if (possibleIdOrText === "puzzle2d.select.lasso" || token === "lasso") {
			runHost("setSelectionMethod", { method: "lasso" });
			return true;
		}
		if (possibleIdOrText === "puzzle2d.selection.clear" || token === "clear") {
			runHost("clearSelection", {});
			return true;
		}
		if (possibleIdOrText === "puzzle2d.create.circle" || token === "circle") {
			runHost("appendCircle", {});
			return true;
		}
		if (possibleIdOrText === "puzzle2d.create.rectangle" || token === "rectangleshape" || token === "rectangle") {
			runHost("appendRectangle", {});
			return true;
		}
		void pane;
		return false;
	}

	/** @emoji 🔗 Attaches the React host bridge used for toolbar commands and snapshots. */
	setHostBridge(bridge: Puzzle2dPlayHostBridge | null): void {
		this.hostBridge = bridge;
		this.rebuildToolbarTools();
	}

	/** @emoji 🔄 Rebuilds {@link ModeRuntime.tools} from the latest host toolbar snapshot. */
	rebuildToolbarTools(): void {
		if (!this.hostBridge) {
			this.mainMode.tools = undefined;
			return;
		}
		this.mainMode.tools = buildPuzzle2dPlayToolbarTools(this.hostBridge.getToolbarState(), this.id);
	}

	private lodMeasureForPane(paneId: Puzzle2dPlayPaneId): WindowMeasure {
		return {
			kind: "select",
			id: `${paneId}-lod`,
			label: "LOD",
			value: this.lodModeByPane[paneId],
			items: [
				{ id: "automatic", value: PUZZLE_2D_LOD_MODE_AUTOMATIC, label: puzzle2dLodAutomaticSelectLabel(this.effectiveLodByPane[paneId]) },
				...PUZZLE_2D_PLAY_LOD_TIERS.map((tier) => ({ id: tier, value: tier, label: puzzle2dPlayLodTierMenuLabel(tier) })),
			],
			onChange: { controllerId: PUZZLE_2D_PLAY_CONTROLLER_ID, command: "setLodModeForPane", args: { pane: paneId } },
		};
	}

	private rebuildShellMode(): void {
		this.mainMode.windowKinds = PUZZLE_2D_PLAY_WINDOW_SPECS.map(
			(row) =>
				new WindowKindRuntime(row.pane, row.label, row.bodyKey, undefined, [this.lodMeasureForPane(row.pane)], this.windowEngagementForPane(row.pane)),
		);
		for (const windowKind of this.mainMode.windowKinds) {
			enforcePlaygroundWindowEngagementInput(windowKind.engagement, `Puzzle 2D play window "${windowKind.id}"`);
		}
	}

	override run(command: string, args?: unknown): void {
		switch (command) {
			case "setLodModeForPane": {
				const { pane, value } = args as { pane: Puzzle2dPlayPaneId; value?: string };
				if (pane !== "2d-overview" && pane !== "2d-detail" && pane !== "2d-selection") break;
				if (value === PUZZLE_2D_LOD_MODE_AUTOMATIC || (typeof value === "string" && isPuzzle2dDrawLodKind(value))) {
					this.lodModeByPane = { ...this.lodModeByPane, [pane]: value as Puzzle2dLodModeKind };
				}
				break;
			}
			case "setEffectiveLodForPane": {
				const { pane, lod } = args as { pane: Puzzle2dPlayPaneId; lod: Puzzle2dDrawLodKind };
				if (!isPuzzle2dDrawLodKind(lod)) break;
				if (this.effectiveLodByPane[pane] === lod) return;
				this.effectiveLodByPane = { ...this.effectiveLodByPane, [pane]: lod };
				this.rebuildShellMode();
				this.hostChromeNotify();
				return;
			}
			case "setSelectionMethod":
			case "setSelectionMode":
			case "toggleSelectionTarget":
			case "clearSelection":
			case "toggleGridSnap":
			case "appendCircle":
			case "appendRectangle":
			case "toggleRedrawPlaying":
			case "redrawHandlesOnce": {
				this.hostBridge?.runHostCommand(command, args);
				break;
			}
			case "engagementInput": {
				const { pane, value } = args as { pane?: Puzzle2dPlayPaneId; value?: string };
				if (pane !== "2d-overview" && pane !== "2d-detail" && pane !== "2d-selection") {
					break;
				}
				this.engagementInputByPane = { ...this.engagementInputByPane, [pane]: String(value ?? "") };
				this.syncWindowEngagementForPane(pane);
				break;
			}
			case "engagementSubmit": {
				const { pane, value } = args as { pane?: Puzzle2dPlayPaneId; value?: string };
				if (pane !== "2d-overview" && pane !== "2d-detail" && pane !== "2d-selection") {
					break;
				}
				if (this.applyEngagementCommand(pane, String(value ?? ""))) {
					this.engagementInputByPane = { ...this.engagementInputByPane, [pane]: "" };
					this.syncWindowEngagementForPane(pane);
				} else {
					this.hostBridge?.runHostCommand(command, args);
				}
				break;
			}
			case "engagementAbort": {
				const { pane } = args as { pane?: Puzzle2dPlayPaneId };
				if (pane !== "2d-overview" && pane !== "2d-detail" && pane !== "2d-selection") {
					break;
				}
				this.engagementInputByPane = { ...this.engagementInputByPane, [pane]: "" };
				this.syncWindowEngagementForPane(pane);
				break;
			}
			case "engagementPossibleSelect": {
				const { pane, possibleId } = args as { pane?: Puzzle2dPlayPaneId; possibleId?: string };
				if (pane !== "2d-overview" && pane !== "2d-detail" && pane !== "2d-selection") {
					break;
				}
				if (this.applyEngagementCommand(pane, possibleId ?? "")) {
					this.engagementInputByPane = { ...this.engagementInputByPane, [pane]: "" };
					this.syncWindowEngagementForPane(pane);
				} else {
					this.hostBridge?.runHostCommand(command, args);
				}
				break;
			}
			default:
				break;
		}
		this.rebuildShellMode();
		this.rebuildToolbarTools();
		this.emit();
	}

	getLodModeByPane(): Readonly<Record<Puzzle2dPlayPaneId, Puzzle2dLodModeKind>> {
		return this.lodModeByPane;
	}

	getEffectiveLodByPane(): Readonly<Record<Puzzle2dPlayPaneId, Puzzle2dDrawLodKind>> {
		return this.effectiveLodByPane;
	}
}
//#endregion 🔖Controller

//#region 🔖DeclarativeBodies
function puzzle2dPlayControllerFromContext(ctx: WindowBodyViewContext): Puzzle2dPlayShellController | undefined {
	return ctx.runtime.getActiveApp()?.controller as Puzzle2dPlayShellController | undefined;
}

function buildPuzzle2dPlayDeclarativeBody(paneId: Puzzle2dPlayPaneId): (ctx: WindowBodyViewContext) => UiNode {
	return (ctx) => {
		if (!puzzle2dPlayControllerFromContext(ctx)) {
			return { type: "text", value: "Missing puzzle 2d play controller" };
		}
		return buildPuzzle2dWindowBody(PUZZLE_2D_PLAY_SURFACE_ID, PUZZLE_2D_PLAY_CONTROLLER_ID, paneId);
	};
}

export const buildPuzzle2dPlayOverviewDeclarativeBody = buildPuzzle2dPlayDeclarativeBody("2d-overview");
export const buildPuzzle2dPlayDetailDeclarativeBody = buildPuzzle2dPlayDeclarativeBody("2d-detail");
export const buildPuzzle2dPlaySelectionDeclarativeBody = buildPuzzle2dPlayDeclarativeBody("2d-selection");
//#endregion 🔖DeclarativeBodies

/** @emoji 🧩 Registers puzzle 2d play window kinds on the supplied controller (layout supplied by host). */
export function attachPuzzle2dPlayWindowKinds(controller: Puzzle2dPlayShellController, layout: unknown): AppRuntime {
	const app = new AppRuntime(PUZZLE_2D_PLAY_APP_ID, "Puzzle 2D play", undefined, controller, layout as never, []);
	app.defaultModeId = controller.mainMode.id;
	app.addMode(controller.mainMode);
	return app;
}

/** @emoji 🧩 Builds the puzzle 2d play {@link AppRuntime}; side panels are tree tabs via {@link PlaygroundView} `augmentPanelTabs` only. */
export function buildPuzzle2dPlayAppRuntime(controller: Puzzle2dPlayShellController): AppRuntime {
	const app = attachPuzzle2dPlayWindowKinds(controller, PUZZLE_2D_PLAY_LAYOUT);
	app.panelTabs = [];
	return app;
}

/** @emoji 📝 Registers puzzle 2d play declarative window bodies on the playground host (side tabs are host tree panels only). */
export function registerPuzzle2dPlayDeclarativeBodies(): void {
	registerWindowBody(PUZZLE_2D_PLAY_BODY_KEY_OVERVIEW, buildPuzzle2dPlayOverviewDeclarativeBody);
	registerWindowBody(PUZZLE_2D_PLAY_BODY_KEY_DETAIL, buildPuzzle2dPlayDetailDeclarativeBody);
	registerWindowBody(PUZZLE_2D_PLAY_BODY_KEY_SELECTION, buildPuzzle2dPlaySelectionDeclarativeBody);
}

//#region 🔖Extension
/** @emoji 🔌 Host context for optional puzzle-2d-play extension activation. */
export interface Puzzle2dPlayPluginContext {
	registerWindowBody(bodyKey: string, factory: (ctx: WindowBodyViewContext) => UiNode): void;
}

/** @emoji 📦 Extension manifest shape for puzzle 2d play (host-agnostic). */
export interface Puzzle2dPlayExtensionManifest {
	readonly id: string;
	readonly label: string;
	readonly version: string;
	readonly contributes: {
		readonly apps?: readonly {
			readonly id: string;
			readonly label: string;
			readonly controllerId: string;
			readonly defaultLayout: WindowLayout;
			readonly defaultModeId: string;
			readonly windowKinds: readonly { readonly id: string; readonly label: string; readonly bodyKey: string }[];
			readonly modes: readonly { readonly id: string; readonly label: string }[];
		}[];
	};
}

export const PUZZLE_2D_PLAY_EXTENSION_MANIFEST: Puzzle2dPlayExtensionManifest = {
	id: "puzzle.2d.play",
	label: "Puzzle 2D Play",
	version: "0.1.0",
	contributes: {
		apps: [
			{
				id: PUZZLE_2D_PLAY_APP_ID,
				label: "Puzzle 2D",
				controllerId: PUZZLE_2D_PLAY_CONTROLLER_ID,
				defaultLayout: PUZZLE_2D_PLAY_LAYOUT,
				defaultModeId: "main",
				windowKinds: [
					{ id: "2d-overview", label: "Overview", bodyKey: PUZZLE_2D_PLAY_BODY_KEY_OVERVIEW },
					{ id: "2d-detail", label: "Zoom", bodyKey: PUZZLE_2D_PLAY_BODY_KEY_DETAIL },
					{ id: "2d-selection", label: "Selection", bodyKey: PUZZLE_2D_PLAY_BODY_KEY_SELECTION },
				],
				modes: [{ id: "main", label: "Main" }],
			},
		],
	},
};

/** @emoji 🔌 Puzzle 2d play plugin: registers declarative bodies on activate. */
export const puzzle2dPlayPlugin: { readonly id: string; activate(context: Puzzle2dPlayPluginContext): void } = {
	id: PUZZLE_2D_PLAY_EXTENSION_MANIFEST.id,
	activate(context: Puzzle2dPlayPluginContext): void {
		context.registerWindowBody(PUZZLE_2D_PLAY_BODY_KEY_OVERVIEW, buildPuzzle2dPlayOverviewDeclarativeBody);
		context.registerWindowBody(PUZZLE_2D_PLAY_BODY_KEY_DETAIL, buildPuzzle2dPlayDetailDeclarativeBody);
		context.registerWindowBody(PUZZLE_2D_PLAY_BODY_KEY_SELECTION, buildPuzzle2dPlaySelectionDeclarativeBody);
	},
};

/** @emoji 🚀 Creates a {@link Platform} with puzzle 2d play app + declarative bodies registered. */
export function buildPuzzle2dPlayRuntime(): Platform {
	registerPuzzle2dPlayDeclarativeBodies();
	const runtime = new Platform();
	const ctrl = new Puzzle2dPlayShellController(runtime.commandBus, () => runtime.notify(), () => runtime.notifyChrome());
	runtime.addApp(buildPuzzle2dPlayAppRuntime(ctrl));
	return runtime;
}

/** @emoji 🛝 Puzzle 2d play harness as a single {@link Playground} instance. */
export class Playground2d extends Playground {
	readonly id = PUZZLE_2D_PLAY_APP_ID;
	readonly initialPanelVisibility = { leftSidePanel: true, rightSidePanel: true };

	createRuntime(): Platform {
		const runtime = new Platform({ id: this.id, initialPanelVisibility: this.initialPanelVisibility });
		const ctrl = new Puzzle2dPlayShellController(runtime.commandBus, () => runtime.notify(), () => runtime.notifyChrome());
		runtime.addApp(buildPuzzle2dPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerPuzzle2dPlayDeclarativeBodies();
	}
}
//#endregion 🔖Extension

export type Puzzle2dPlayStructuralDeleteItem = { kind: "edge" | "node"; id: string };

/** @emoji 🗑️ Dedupes structural deletes and drops ids absent from the fixture (descriptor resync bursts), keeping real multi-edge node deletes. */
export function filterPuzzle2dPlayStructuralDeleteBatch(
	batch: readonly Puzzle2dPlayStructuralDeleteItem[],
	fixture: Puzzle2dFixtureV1,
): Puzzle2dPlayStructuralDeleteItem[] {
	const seen = new Set<string>();
	const out: Puzzle2dPlayStructuralDeleteItem[] = [];
	for (const item of batch) {
		const key = `${item.kind}:${item.id}`;
		if (seen.has(key)) {
			continue;
		}
		seen.add(key);
		const exists =
			item.kind === "node" ? fixture.nodes.some((n) => n.id === item.id) : fixture.edges.some((e) => e.id === item.id);
		if (!exists) {
			continue;
		}
		out.push(item);
	}
	const nodeDeletes = out.filter((item) => item.kind === "node");
	const nodeCount = fixture.nodes.length;
	if (nodeCount > 0 && nodeDeletes.length >= 2 && nodeDeletes.length > nodeCount / 2) {
		return out.filter((item) => item.kind === "edge");
	}
	return out;
}

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("puzzle 2d play declarative shell", () => {
		it("default nakagin fixture parses with puzzle 2d graph nodes", () => {
			expect(PUZZLE_2D_PLAY_DEFAULT_FIXTURE.nodes.length).toBeGreaterThan(0);
			expect(PUZZLE_2D_PLAY_DEFAULT_FIXTURE.edges.length).toBeGreaterThan(0);
			expect(parsePuzzle2dFixtureV1(nakaginFixtureJson as unknown)?.nodes.length).toBe(
				PUZZLE_2D_PLAY_DEFAULT_FIXTURE.nodes.length,
			);
		});

		it("declarative overview body references puzzle2d host surface", () => {
			const runtime = buildPuzzle2dPlayRuntime();
			const tree = buildPuzzle2dPlayOverviewDeclarativeBody({
				runtime,
				windowKindId: "2d-overview",
				bodyKey: PUZZLE_2D_PLAY_BODY_KEY_OVERVIEW,
				activeModeId: "main",
				generation: 0,
			});
			expect(tree).toEqual(buildPuzzle2dWindowBody(PUZZLE_2D_PLAY_SURFACE_ID, PUZZLE_2D_PLAY_CONTROLLER_ID, "2d-overview"));
		});

		it("puzzle2dPlayHierarchyTreeHighlightedIds maps graph ids to tree row ids", () => {
			const fixture = parsePuzzle2dFixtureV1({
				schema: "puzzle.2d.fixture/v1",
				camera: { x: 0, y: 0, zoom: 1 },
				nodes: [
					{
						id: "root",
						root: true,
						shape: "circle",
						text: "Root",
						x: 0,
						y: 0,
						radius: 10,
						handles: [{ id: "h-root", angle: 0, handleKind: "port" }],
					},
				],
				edges: [{ id: "e1", source: "h-root", target: "h-root" }],
			});
			expect(fixture).not.toBeNull();
			expect(puzzle2dPlayHierarchyTreeHighlightedIds(fixture!, "root")).toEqual(["puzzle-2d-play-hierarchy.node.root"]);
			expect(puzzle2dPlayHierarchyTreeHighlightedIds(fixture!, "h-root")).toEqual(["puzzle-2d-play-hierarchy.handle.h-root"]);
			expect(puzzle2dPlayHierarchyTreeHighlightedIds(fixture!, "e1")).toEqual(["puzzle-2d-play-hierarchy.edge.e1"]);
			expect(puzzle2dPlayHierarchyTreeHighlightedIds(fixture!, null)).toEqual([]);
		});

		it("puzzle2dPlayHierarchyTreeSelectedIds maps graph ids to tree row ids", () => {
			const fixture = parsePuzzle2dFixtureV1({
				schema: "puzzle.2d.fixture/v1",
				camera: { x: 0, y: 0, zoom: 1 },
				nodes: [
					{
						id: "root",
						root: true,
						shape: "circle",
						text: "Root",
						x: 0,
						y: 0,
						radius: 10,
						handles: [{ id: "h-root", angle: 0, handleKind: "port" }],
					},
				],
				edges: [{ id: "e1", source: "h-root", target: "h-root" }],
			});
			expect(fixture).not.toBeNull();
			expect(puzzle2dPlayHierarchyTreeSelectedIds(fixture!, ["root"])).toEqual(["puzzle-2d-play-hierarchy.node.root"]);
			expect(puzzle2dPlayHierarchyTreeSelectedIds(fixture!, ["h-root"])).toEqual(["puzzle-2d-play-hierarchy.handle.h-root"]);
			expect(puzzle2dPlayHierarchyTreeSelectedIds(fixture!, ["e1"])).toEqual(["puzzle-2d-play-hierarchy.edge.e1"]);
			expect(puzzle2dPlayHierarchyGraphIdFromTreeItemId("puzzle-2d-play-hierarchy.node.root")).toBe("root");
		});

		it("buildPuzzle2dPlayHierarchySections nests root nodes, handles, and child nodes", () => {
			const fixture = parsePuzzle2dFixtureV1({
				schema: "puzzle.2d.fixture/v1",
				camera: { x: 0, y: 0, zoom: 1 },
				nodes: [
					{
						id: "root",
						root: true,
						shape: "circle",
						text: "Root",
						x: 0,
						y: 0,
						radius: 10,
						handles: [{ id: "h-root", angle: 0, handleKind: "port" }],
					},
					{
						id: "child",
						shape: "circle",
						text: "Child",
						x: 10,
						y: 0,
						radius: 10,
						handles: [{ id: "h-child", angle: 0, handleKind: "port" }],
					},
				],
				edges: [{ id: "e1", source: "h-root", target: "h-child" }],
			});
			expect(fixture).not.toBeNull();
			const tree = buildPuzzle2dPlayHierarchySections(fixture!, [], () => {});
			const puzzle2dRoot = tree.sections[0]?.items?.[0];
			expect(puzzle2dRoot?.label).toBe("Puzzle 2D");
			const nodesGroup = puzzle2dRoot?.items?.find((row) => row.label === "Nodes");
			expect(nodesGroup?.items?.[0]?.id).toBe("puzzle-2d-play-hierarchy.node.root");
			expect(nodesGroup?.items?.[0]?.label).toBe("Root");
			expect(nodesGroup?.items?.[0]?.items?.[0]?.label).toBe("Handles");
			expect(nodesGroup?.items?.[0]?.items?.[1]?.label).toBe("Child");
		});

		it("buildPuzzle2dPlayRuntime wires main mode and empty side tab slots", () => {
			const runtime = buildPuzzle2dPlayRuntime();
			const app = runtime.getActiveApp();
			expect(app?.panelTabs).toEqual([]);
			expect(app?.controller.mainMode.tools ?? {}).toEqual({});
		});

		it("setEffectiveLodForPane bumps chrome generation only", () => {
			const runtime = buildPuzzle2dPlayRuntime();
			const controller = runtime.getActiveApp()?.controller as Puzzle2dPlayShellController;
			const dataGen = runtime.generation;
			const chromeGen = runtime.chromeGeneration;
			controller.commandBus.dispatch(PUZZLE_2D_PLAY_CONTROLLER_ID, "setEffectiveLodForPane", {
				pane: "2d-overview",
				lod: "detail",
			});
			expect(runtime.generation).toBe(dataGen);
			expect(runtime.chromeGeneration).toBe(chromeGen + 1);
		});
	});

	describe("filterPuzzle2dPlayStructuralDeleteBatch", () => {
		it("keeps real multi-edge node deletes and drops resync-only ghost ids", () => {
			const fixture: Puzzle2dFixtureV1 = {
				schema: "puzzle.2d.fixture/v1",
				camera: { x: 0, y: 0, zoom: 1 },
				nodes: [
					{ id: "hub", x: 0, y: 0, radius: 20, handles: [{ id: "hub.h0", angle: 0 }] },
					{ id: "leaf", x: 100, y: 0, radius: 20, handles: [{ id: "leaf.h0", angle: Math.PI }] },
				],
				edges: [
					{ id: "e0", source: "hub.h0", target: "leaf.h0" },
					{ id: "e1", source: "hub.h0", target: "leaf.h0" },
					{ id: "e2", source: "hub.h0", target: "leaf.h0" },
					{ id: "e3", source: "hub.h0", target: "leaf.h0" },
					{ id: "e4", source: "hub.h0", target: "leaf.h0" },
					{ id: "e5", source: "hub.h0", target: "leaf.h0" },
				],
			};
			const batch = [
				{ kind: "edge" as const, id: "e0" },
				{ kind: "edge" as const, id: "e1" },
				{ kind: "edge" as const, id: "e2" },
				{ kind: "edge" as const, id: "e3" },
				{ kind: "edge" as const, id: "e4" },
				{ kind: "edge" as const, id: "e5" },
				{ kind: "node" as const, id: "leaf" },
			];
			expect(filterPuzzle2dPlayStructuralDeleteBatch(batch, fixture)).toEqual(batch);
			expect(
				filterPuzzle2dPlayStructuralDeleteBatch(
					[
						{ kind: "edge", id: "ghost-edge" },
						{ kind: "node", id: "ghost-node" },
						{ kind: "edge", id: "e0" },
					],
					fixture,
				),
			).toEqual([{ kind: "edge", id: "e0" }]);
		});

		it("drops resync bursts that would delete most of the fixture graph", () => {
			const fixture: Puzzle2dFixtureV1 = {
				schema: "puzzle.2d.fixture/v1",
				camera: { x: 0, y: 0, zoom: 1 },
				nodes: [
					{ id: "a", x: 0, y: 0, radius: 10, handles: [] },
					{ id: "b", x: 10, y: 0, radius: 10, handles: [] },
					{ id: "c", x: 20, y: 0, radius: 10, handles: [] },
				],
				edges: [],
			};
			const batch = [
				{ kind: "node" as const, id: "a" },
				{ kind: "node" as const, id: "b" },
				{ kind: "node" as const, id: "c" },
			];
			expect(filterPuzzle2dPlayStructuralDeleteBatch(batch, fixture)).toEqual([]);
		});
	});

	it("nakagin default fixture yields a populated hierarchy nodes group", () => {
		const tree = buildPuzzle2dPlayHierarchySections(PUZZLE_2D_PLAY_DEFAULT_FIXTURE, [], () => {});
		const nodesGroup = tree.sections[0]?.items?.[0]?.items?.find((row) => row.label === "Nodes");
		expect(nodesGroup?.items?.length).toBeGreaterThan(0);
		expect(nodesGroup?.items?.[0]?.label).not.toBe("(none)");
	});
}
//#endregion 🧪Tests

//#region 🔖Boot
if (
	typeof document !== "undefined" &&
	document.getElementById("root") != null &&
	!import.meta.vitest &&
	import.meta.env.PUZZLE_PLAY_ENTRY === "2d"
) {
	void (async () => {
		await import("./globals.css");
		const { boot2dPlay } = await import("@framework/playground/renderer/react/puzzle/2d");
		boot2dPlay(new Playground2d());
	})();
}
//#endregion 🔖Boot
