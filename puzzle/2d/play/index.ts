// #region 🧲Header
// 💻 elements/lib/board/play/index.ts — Board play shell on `@framework/playground`: declarative bodies, LOD measures, toolbar tools (no React).
// #endregion 🧲Header

import {
	CommandBus,
	Controller,
	Playground,
	registerWindowBody,
	ProductRuntime,
	AppRuntime,
	ModeRuntime,
	WindowKindRuntime,
	buildBoardWindowBody,
	buildPlaygroundKindToggleTools,
	createWindowLayout,
	playgroundTreePanelRootItems,
	type AppTools,
	type ToolItem,
	type WindowBodyViewContext,
	type WindowMeasure,
	type UiNode,
	type UiTreeItemNode,
	type UiTreeNode,
	type WindowLayout,
} from "@framework/playground";

import nakaginFixtureJson from "./fixtures/nakagin-capsule-tower.board.json";
import {
	BOARD_LOD_MODE_AUTOMATIC,
	boardFixtureNodeCaption,
	boardLodAutomaticSelectLabel,
	isBoardDrawLodKind,
	parseBoardFixtureV1,
	type BoardDrawLodKind,
	type BoardFixtureNodeV1,
	type BoardFixtureV1,
	type BoardLodModeKind,
	type BoardSelectionMethod,
	type BoardSelectionMode,
	type BoardSelectionTargets,
} from "../react/index.tsx";

//#region 🔖Ids
export type Puzzle2dPlayPaneId = "2d-overview" | "2d-detail" | "2d-selection";

export const PUZZLE_2D_PLAY_APP_ID = "puzzle-2d-play";
export const PUZZLE_2D_PLAY_CONTROLLER_ID = "puzzle-2d-play";
export const PUZZLE_2D_PLAY_BOARD_SURFACE_ID = "puzzle.2d.play.board/v1";

export const PUZZLE_2D_PLAY_BODY_KEY_OVERVIEW = "puzzle.2d.play.overview";
export const PUZZLE_2D_PLAY_BODY_KEY_DETAIL = "puzzle.2d.play.detail";
export const PUZZLE_2D_PLAY_BODY_KEY_SELECTION = "puzzle.2d.play.selection";

export const PUZZLE_2D_PLAY_LOD_TIERS: BoardDrawLodKind[] = ["minimap", "overview", "compact", "normal", "detail", "micro"];

export function boardPlayLodTierMenuLabel(tier: BoardDrawLodKind): string {
	return tier.charAt(0).toUpperCase() + tier.slice(1);
}

export const PUZZLE_2D_PLAY_HIERARCHY_TAB_ID = "puzzle-2d-play-hierarchy";

export const PUZZLE_2D_PLAY_PACKAGE_ROOT = import.meta.url;

export const PUZZLE_2D_PLAY_DEFAULT_FIXTURE: BoardFixtureV1 =
	parseBoardFixtureV1(nakaginFixtureJson as unknown) ?? (nakaginFixtureJson as BoardFixtureV1);

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

//#region 🔖BoardPlayHierarchy
function boardFixtureHandleToNodeId(fixture: BoardFixtureV1): ReadonlyMap<string, string> {
	const out = new Map<string, string>();
	for (const node of fixture.nodes) {
		for (const handle of node.handles) {
			out.set(handle.id, node.id);
		}
	}
	return out;
}

function boardFixtureChildrenByNodeId(fixture: BoardFixtureV1): ReadonlyMap<string, readonly string[]> {
	const handleToNode = boardFixtureHandleToNodeId(fixture);
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

function boardFixtureRootNodeIds(fixture: BoardFixtureV1, childrenByParent: ReadonlyMap<string, readonly string[]>): readonly string[] {
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

function boardFixtureNodeLabel(node: BoardFixtureNodeV1): string {
	const caption = boardFixtureNodeCaption(node);
	return caption?.trim() ? `${node.id} · ${caption}` : node.id;
}

function buildBoardFixtureNodeHierarchyItem(
	fixture: BoardFixtureV1,
	nodeId: string,
	childrenByParent: ReadonlyMap<string, readonly string[]>,
	selectedIds: ReadonlySet<string>,
	onSelect: (id: string) => void,
	visiting: Set<string>,
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
		label: handle.handleKind ? `${handle.id} · ${handle.handleKind}` : handle.id,
		isSelected: selectedIds.has(handle.id),
		onClick: () => onSelect(handle.id),
	}));
	const handlesGroup: UiTreeItemNode = {
		id: `puzzle-2d-play-hierarchy.node.${nodeId}.handles`,
		label: "Handles",
		defaultOpen: true,
		items: handleItems.length ? handleItems : [{ id: `puzzle-2d-play-hierarchy.node.${nodeId}.handles.empty`, label: "(none)" }],
	};
	const childItems: UiTreeItemNode[] = [];
	for (const childId of childrenByParent.get(nodeId) ?? []) {
		const childItem = buildBoardFixtureNodeHierarchyItem(fixture, childId, childrenByParent, selectedIds, onSelect, visiting);
		if (childItem) {
			childItems.push(childItem);
		}
	}
	visiting.delete(nodeId);
	return {
		id: `puzzle-2d-play-hierarchy.node.${nodeId}`,
		label: boardFixtureNodeLabel(node),
		description: node.nodeKind ?? undefined,
		isSelected: selectedIds.has(nodeId),
		defaultOpen: true,
		onClick: () => onSelect(nodeId),
		items: [handlesGroup, ...childItems],
	};
}

/** @emoji 🌳 Nested workbench tree: Board → Nodes (graph) → Handles; flat Edges group. */
export function buildBoardPlayHierarchySections(
	fixture: BoardFixtureV1,
	selectionIds: readonly string[],
	onSelect: (id: string) => void,
): UiTreeNode {
	const selectedIds = new Set(selectionIds);
	const childrenByParent = boardFixtureChildrenByNodeId(fixture);
	const rootIds = boardFixtureRootNodeIds(fixture, childrenByParent);
	const visiting = new Set<string>();
	const nodeItems: UiTreeItemNode[] = [];
	for (const rootId of rootIds) {
		const item = buildBoardFixtureNodeHierarchyItem(fixture, rootId, childrenByParent, selectedIds, onSelect, visiting);
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
		label: edge.id,
		description: `${edge.source} → ${edge.target}`,
		isSelected: selectedIds.has(edge.id),
		onClick: () => onSelect(edge.id),
	}));
	const edgesGroup: UiTreeItemNode = {
		id: "puzzle-2d-play-hierarchy.edges",
		label: "Edges",
		defaultOpen: true,
		items: edgeItems.length ? edgeItems : [{ id: "puzzle-2d-play-hierarchy.edges.empty", label: "(none)" }],
	};
	const boardRoot: UiTreeItemNode = {
		id: "puzzle-2d-play-hierarchy.board",
		label: "Board",
		defaultOpen: true,
		items: [nodesGroup, edgesGroup],
	};
	return playgroundTreePanelRootItems("puzzle-2d-play-hierarchy.root", [boardRoot]) as UiTreeNode;
}
//#endregion 🔖BoardPlayHierarchy

//#region 🔖Controller
const PUZZLE_2D_PLAY_TARGET_KINDS = ["nodes", "edges", "handles"] as const;
type BoardPlayTargetKind = (typeof PUZZLE_2D_PLAY_TARGET_KINDS)[number];

function boardPlayTargetLabel(kind: BoardPlayTargetKind): string {
	if (kind === "nodes") return "Nodes";
	if (kind === "edges") return "Edges";
	return "Handles";
}

/** @emoji 🧰 Snapshot read by {@link buildBoardPlayToolbarTools} (host-owned play state). */
export interface BoardPlayToolbarState {
	readonly boardSelectionMethod: BoardSelectionMethod;
	readonly boardSelectionMode: BoardSelectionMode;
	readonly boardSelectionTargets: BoardSelectionTargets;
	readonly boardGridSnapEnabled: boolean;
	readonly boardRedrawPlaying: boolean;
}

/** @emoji 🔗 Host bridge: toolbar snapshot + commands that need React/fixture context. */
export interface BoardPlayHostBridge {
	getToolbarState(): BoardPlayToolbarState;
	runHostCommand(command: string, args?: unknown): void;
}

/** @emoji 🧰 Playground {@link AppTools} for board play (selection, filter, view, create, actions). */
export function buildBoardPlayToolbarTools(state: BoardPlayToolbarState, controllerId: string): AppTools {
	const targetRecord: Record<BoardPlayTargetKind, boolean> = {
		nodes: state.boardSelectionTargets.nodes,
		edges: state.boardSelectionTargets.edges,
		handles: state.boardSelectionTargets.handles,
	};
	const selectionTools: ToolItem[] = [
		{
			id: "board.select.rectangle",
			kind: "toggle",
			text: "Rectangle",
			order: 0,
			pressed: state.boardSelectionMethod === "rectangle",
			controllerId,
			command: "setSelectionMethod",
			args: { method: "rectangle" },
		},
		{
			id: "board.select.lasso",
			kind: "toggle",
			text: "Lasso",
			order: 1,
			pressed: state.boardSelectionMethod === "lasso",
			controllerId,
			command: "setSelectionMethod",
			args: { method: "lasso" },
		},
		{
			id: "board.select.mode.default",
			kind: "toggle",
			text: "Default",
			order: 2,
			pressed: state.boardSelectionMode === "default",
			controllerId,
			command: "setSelectionMode",
			args: { mode: "default" },
		},
		{
			id: "board.select.mode.additive",
			kind: "toggle",
			text: "Add",
			order: 3,
			pressed: state.boardSelectionMode === "additive",
			controllerId,
			command: "setSelectionMode",
			args: { mode: "additive" },
		},
		{
			id: "board.select.mode.subtractive",
			kind: "toggle",
			text: "Subtract",
			order: 4,
			pressed: state.boardSelectionMode === "subtractive",
			controllerId,
			command: "setSelectionMode",
			args: { mode: "subtractive" },
		},
		{
			id: "board.select.mode.invertive",
			kind: "toggle",
			text: "Invert",
			order: 5,
			pressed: state.boardSelectionMode === "invertive",
			controllerId,
			command: "setSelectionMode",
			args: { mode: "invertive" },
		},
		...buildPlaygroundKindToggleTools("selection", PUZZLE_2D_PLAY_TARGET_KINDS, boardPlayTargetLabel, targetRecord, controllerId, "toggleSelectionTarget"),
		{
			id: "board.selection.clear",
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
				id: "board.grid.snap",
				kind: "toggle",
				text: "Grid snap",
				order: 0,
				pressed: state.boardGridSnapEnabled,
				controllerId,
				command: "toggleGridSnap",
			},
		],
		create: [
			{ id: "board.create.circle", kind: "button", label: "Circle", order: 0, controllerId, command: "appendCircle" },
			{ id: "board.create.rectangle", kind: "button", label: "Rectangle", order: 1, controllerId, command: "appendRectangle" },
		],
		actions: [
			{
				id: "board.redraw.play",
				kind: "toggle",
				text: "Redraw",
				order: 0,
				pressed: state.boardRedrawPlaying,
				controllerId,
				command: "toggleRedrawPlaying",
			},
			{ id: "board.redraw.handles", kind: "button", label: "Handles", title: "Redraw handles once", order: 1, controllerId, command: "redrawHandlesOnce" },
		],
	};
}

/** @emoji 🎛 Board play shell controller: per-pane LOD modes + playground toolbar tools. */
export class BoardPlayShellController extends Controller {
	readonly mainMode = new ModeRuntime("main", "Board", undefined);
	private lodModeByPane: Record<Puzzle2dPlayPaneId, BoardLodModeKind>;
	private effectiveLodByPane: Record<Puzzle2dPlayPaneId, BoardDrawLodKind>;
	private hostBridge: BoardPlayHostBridge | null = null;

	constructor(commandBus: CommandBus, hostNotify: () => void) {
		super(PUZZLE_2D_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.lodModeByPane = {
			"2d-detail": BOARD_LOD_MODE_AUTOMATIC,
			"2d-overview": BOARD_LOD_MODE_AUTOMATIC,
			"2d-selection": BOARD_LOD_MODE_AUTOMATIC,
		};
		this.effectiveLodByPane = {
			"2d-detail": "normal",
			"2d-overview": "normal",
			"2d-selection": "normal",
		};
		this.rebuildShellMode();
	}

	/** @emoji 🔗 Attaches the React host bridge used for toolbar commands and snapshots. */
	setHostBridge(bridge: BoardPlayHostBridge | null): void {
		this.hostBridge = bridge;
		this.rebuildToolbarTools();
	}

	/** @emoji 🔄 Rebuilds {@link ModeRuntime.tools} from the latest host toolbar snapshot. */
	rebuildToolbarTools(): void {
		if (!this.hostBridge) {
			this.mainMode.tools = undefined;
			return;
		}
		this.mainMode.tools = buildBoardPlayToolbarTools(this.hostBridge.getToolbarState(), this.id);
	}

	private lodMeasureForPane(paneId: Puzzle2dPlayPaneId): WindowMeasure {
		return {
			kind: "select",
			id: `${paneId}-lod`,
			label: "LOD",
			value: this.lodModeByPane[paneId],
			items: [
				{ id: "automatic", value: BOARD_LOD_MODE_AUTOMATIC, label: boardLodAutomaticSelectLabel(this.effectiveLodByPane[paneId]) },
				...PUZZLE_2D_PLAY_LOD_TIERS.map((tier) => ({ id: tier, value: tier, label: boardPlayLodTierMenuLabel(tier) })),
			],
			onChange: { controllerId: PUZZLE_2D_PLAY_CONTROLLER_ID, command: "setLodModeForPane", args: { pane: paneId } },
		};
	}

	private rebuildShellMode(): void {
		this.mainMode.windowKinds = [
			new WindowKindRuntime("2d-overview", "Overview", PUZZLE_2D_PLAY_BODY_KEY_OVERVIEW, undefined, [this.lodMeasureForPane("2d-overview")]),
			new WindowKindRuntime("2d-detail", "Zoom", PUZZLE_2D_PLAY_BODY_KEY_DETAIL, undefined, [this.lodMeasureForPane("2d-detail")]),
			new WindowKindRuntime("2d-selection", "Selection", PUZZLE_2D_PLAY_BODY_KEY_SELECTION, undefined, [this.lodMeasureForPane("2d-selection")]),
		];
	}

	override run(command: string, args?: unknown): void {
		switch (command) {
			case "setLodModeForPane": {
				const { pane, value } = args as { pane: Puzzle2dPlayPaneId; value?: string };
				if (pane !== "2d-overview" && pane !== "2d-detail" && pane !== "2d-selection") break;
				if (value === BOARD_LOD_MODE_AUTOMATIC || (typeof value === "string" && isBoardDrawLodKind(value))) {
					this.lodModeByPane = { ...this.lodModeByPane, [pane]: value as BoardLodModeKind };
				}
				break;
			}
			case "setEffectiveLodForPane": {
				const { pane, lod } = args as { pane: Puzzle2dPlayPaneId; lod: BoardDrawLodKind };
				if (!isBoardDrawLodKind(lod)) break;
				if (this.effectiveLodByPane[pane] === lod) break;
				this.effectiveLodByPane = { ...this.effectiveLodByPane, [pane]: lod };
				break;
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
			default:
				break;
		}
		this.rebuildShellMode();
		this.rebuildToolbarTools();
		this.emit();
	}

	getLodModeByPane(): Readonly<Record<Puzzle2dPlayPaneId, BoardLodModeKind>> {
		return this.lodModeByPane;
	}

	getEffectiveLodByPane(): Readonly<Record<Puzzle2dPlayPaneId, BoardDrawLodKind>> {
		return this.effectiveLodByPane;
	}
}
//#endregion 🔖Controller

//#region 🔖DeclarativeBodies
function boardPlayControllerFromContext(ctx: WindowBodyViewContext): BoardPlayShellController | undefined {
	return ctx.runtime.getActiveApp()?.controller as BoardPlayShellController | undefined;
}

function buildBoardPlayDeclarativeBody(paneId: Puzzle2dPlayPaneId): (ctx: WindowBodyViewContext) => UiNode {
	return (ctx) => {
		if (!boardPlayControllerFromContext(ctx)) {
			return { type: "text", value: "Missing board play controller" };
		}
		return buildBoardWindowBody(PUZZLE_2D_PLAY_BOARD_SURFACE_ID, PUZZLE_2D_PLAY_CONTROLLER_ID, paneId);
	};
}

export const buildBoardPlayOverviewDeclarativeBody = buildBoardPlayDeclarativeBody("2d-overview");
export const buildBoardPlayDetailDeclarativeBody = buildBoardPlayDeclarativeBody("2d-detail");
export const buildBoardPlaySelectionDeclarativeBody = buildBoardPlayDeclarativeBody("2d-selection");
//#endregion 🔖DeclarativeBodies

/** @emoji 🧩 Registers board play window kinds on the supplied controller (layout supplied by host). */
export function attachBoardPlayWindowKinds(controller: BoardPlayShellController, layout: unknown): AppRuntime {
	const app = new AppRuntime(PUZZLE_2D_PLAY_APP_ID, "Board", undefined, controller, layout as never, []);
	app.defaultModeId = controller.mainMode.id;
	app.addMode(controller.mainMode);
	return app;
}

/** @emoji 🧩 Builds the board play {@link AppRuntime}; side panels are tree tabs via {@link PlaygroundView} `augmentPanelTabs` only. */
export function buildBoardPlayAppRuntime(controller: BoardPlayShellController): AppRuntime {
	const app = attachBoardPlayWindowKinds(controller, PUZZLE_2D_PLAY_LAYOUT);
	app.leftTabs = [];
	app.rightTabs = [];
	return app;
}

/** @emoji 📝 Registers board play declarative window bodies on the playground host (side tabs are host tree panels only). */
export function registerBoardPlayDeclarativeBodies(): void {
	registerWindowBody(PUZZLE_2D_PLAY_BODY_KEY_OVERVIEW, buildBoardPlayOverviewDeclarativeBody);
	registerWindowBody(PUZZLE_2D_PLAY_BODY_KEY_DETAIL, buildBoardPlayDetailDeclarativeBody);
	registerWindowBody(PUZZLE_2D_PLAY_BODY_KEY_SELECTION, buildBoardPlaySelectionDeclarativeBody);
}

//#region 🔖Extension
/** @emoji 🔌 Host context for optional puzzle-2d-play extension activation. */
export interface BoardPlayPluginContext {
	registerWindowBody(bodyKey: string, factory: (ctx: WindowBodyViewContext) => UiNode): void;
}

/** @emoji 📦 Extension manifest shape for board play (host-agnostic). */
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
	id: "elements.puzzle-2d-play",
	label: "Board Play",
	version: "0.1.0",
	contributes: {
		apps: [
			{
				id: PUZZLE_2D_PLAY_APP_ID,
				label: "Board",
				controllerId: PUZZLE_2D_PLAY_CONTROLLER_ID,
				defaultLayout: PUZZLE_2D_PLAY_LAYOUT,
				defaultModeId: "main",
				windowKinds: [
					{ id: "2d-overview", label: "Overview", bodyKey: PUZZLE_2D_PLAY_BODY_KEY_OVERVIEW },
					{ id: "2d-detail", label: "Zoom", bodyKey: PUZZLE_2D_PLAY_BODY_KEY_DETAIL },
					{ id: "2d-selection", label: "Selection", bodyKey: PUZZLE_2D_PLAY_BODY_KEY_SELECTION },
				],
				modes: [{ id: "main", label: "Board" }],
			},
		],
	},
};

/** @emoji 🔌 Board play plugin: registers declarative bodies on activate. */
export const boardPlayPlugin: { readonly id: string; activate(context: BoardPlayPluginContext): void } = {
	id: PUZZLE_2D_PLAY_EXTENSION_MANIFEST.id,
	activate(context: BoardPlayPluginContext): void {
		context.registerWindowBody(PUZZLE_2D_PLAY_BODY_KEY_OVERVIEW, buildBoardPlayOverviewDeclarativeBody);
		context.registerWindowBody(PUZZLE_2D_PLAY_BODY_KEY_DETAIL, buildBoardPlayDetailDeclarativeBody);
		context.registerWindowBody(PUZZLE_2D_PLAY_BODY_KEY_SELECTION, buildBoardPlaySelectionDeclarativeBody);
	},
};

/** @emoji 🚀 Creates a {@link ProductRuntime} with board play app + declarative bodies registered. */
export function buildBoardPlayRuntime(): ProductRuntime {
	registerBoardPlayDeclarativeBodies();
	const runtime = new ProductRuntime();
	const ctrl = new BoardPlayShellController(runtime.commandBus, () => runtime.notify());
	runtime.addApp(buildBoardPlayAppRuntime(ctrl));
	return runtime;
}

/** @emoji 🛝 Board play harness as a single {@link Playground} instance. */
export class Playground2d extends Playground {
	readonly id = PUZZLE_2D_PLAY_APP_ID;
	readonly initialPanelVisibility = { leftSidePanel: true, rightSidePanel: true };

	createRuntime(): ProductRuntime {
		const runtime = new ProductRuntime();
		const ctrl = new BoardPlayShellController(runtime.commandBus, () => runtime.notify());
		runtime.addApp(buildBoardPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerBoardPlayDeclarativeBodies();
	}
}
//#endregion 🔖Extension

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("board play declarative shell", () => {
		it("declarative overview body references board host surface", () => {
			const runtime = buildBoardPlayRuntime();
			const tree = buildBoardPlayOverviewDeclarativeBody({
				runtime,
				windowKindId: "2d-overview",
				bodyKey: PUZZLE_2D_PLAY_BODY_KEY_OVERVIEW,
				activeModeId: "main",
				generation: 0,
			});
			expect(tree).toEqual(buildBoardWindowBody(PUZZLE_2D_PLAY_BOARD_SURFACE_ID, PUZZLE_2D_PLAY_CONTROLLER_ID, "2d-overview"));
		});

		it("buildBoardPlayHierarchySections nests root nodes, handles, and child nodes", () => {
			const fixture = parseBoardFixtureV1({
				schema: "puzzle.2d.fixture/v1",
				camera: { x: 0, y: 0, zoom: 1 },
				nodes: [
					{
						id: "root",
						root: true,
						shape: "circle",
						x: 0,
						y: 0,
						radius: 10,
						handles: [{ id: "h-root", angle: 0, handleKind: "board.port" }],
					},
					{
						id: "child",
						shape: "circle",
						x: 10,
						y: 0,
						radius: 10,
						handles: [{ id: "h-child", angle: 0, handleKind: "board.port" }],
					},
				],
				edges: [{ id: "e1", source: "h-root", target: "h-child" }],
			});
			expect(fixture).not.toBeNull();
			const tree = buildBoardPlayHierarchySections(fixture!, [], () => {});
			const boardRoot = tree.sections[0]?.items?.[0];
			expect(boardRoot?.label).toBe("Board");
			const nodesGroup = boardRoot?.items?.find((row) => row.label === "Nodes");
			expect(nodesGroup?.items?.[0]?.id).toBe("puzzle-2d-play-hierarchy.node.root");
			expect(nodesGroup?.items?.[0]?.items?.[0]?.label).toBe("Handles");
			expect(nodesGroup?.items?.[0]?.items?.[1]?.id).toBe("puzzle-2d-play-hierarchy.node.child");
		});

		it("buildBoardPlayRuntime wires main mode and empty side tab slots", () => {
			const runtime = buildBoardPlayRuntime();
			const app = runtime.getActiveApp();
			expect(app?.leftTabs).toEqual([]);
			expect(app?.rightTabs).toEqual([]);
			expect(app?.controller.mainMode.tools ?? {}).toEqual({});
		});
	});
}
//#endregion 🧪Tests
