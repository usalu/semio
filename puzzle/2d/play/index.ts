// #region 🧲Header
// 💻 elements/lib/board/play/index.ts — Board play shell on `@framework/playground`: declarative bodies, LOD measures, toolbar tools (no React).
// #endregion 🧲Header

import {
	CommandBus,
	Controller,
	registerWindowBody,
	ProductRuntime,
	AppRuntime,
	ModeRuntime,
	WindowKindRuntime,
	buildBoardWindowBody,
	buildPlaygroundKindToggleTools,
	createWindowLayout,
	type AppTools,
	type ToolItem,
	type WindowBodyViewContext,
	type WindowMeasure,
	type UiNode,
	type WindowLayout,
} from "@framework/playground";

import { playgroundTreePanelRootItems } from "@framework/playground-renderer-react";
import type { TreeDataItem, TreeDataSection } from "@ui/react";

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
} from "../index";

//#region 🔖Ids
export type BoardPlayPaneId = "board-overview" | "board-detail" | "board-selection";

export const BOARD_PLAY_APP_ID = "elements-board-play";
export const BOARD_PLAY_CONTROLLER_ID = "board-play";
export const BOARD_PLAY_BOARD_SURFACE_ID = "elements.board.play.board/v1";

export const BOARD_PLAY_BODY_KEY_OVERVIEW = "elements.board.play.overview";
export const BOARD_PLAY_BODY_KEY_DETAIL = "elements.board.play.detail";
export const BOARD_PLAY_BODY_KEY_SELECTION = "elements.board.play.selection";

export const BOARD_PLAY_LOD_TIERS: BoardDrawLodKind[] = ["minimap", "overview", "compact", "normal", "detail", "micro"];

export function boardPlayLodTierMenuLabel(tier: BoardDrawLodKind): string {
	return tier.charAt(0).toUpperCase() + tier.slice(1);
}

export const BOARD_PLAY_HIERARCHY_TAB_ID = "board-play-hierarchy";

export const BOARD_PLAY_PACKAGE_ROOT = import.meta.url;

export const BOARD_PLAY_DEFAULT_FIXTURE: BoardFixtureV1 =
	parseBoardFixtureV1(nakaginFixtureJson as unknown) ?? (nakaginFixtureJson as BoardFixtureV1);

export const BOARD_PLAY_LAYOUT: WindowLayout = {
	root: {
		kind: "row",
		children: [
			{
				kind: "stack",
				size: 50,
				children: [createWindowLayout("board-overview", "Overview")],
			},
			{
				kind: "column",
				size: 50,
				children: [
					{ kind: "stack", size: 50, children: [createWindowLayout("board-detail", "Zoom")] },
					{ kind: "stack", size: 50, children: [createWindowLayout("board-selection", "Selection")] },
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
): TreeDataItem | null {
	if (visiting.has(nodeId)) {
		return null;
	}
	const node = fixture.nodes.find((row) => row.id === nodeId);
	if (!node) {
		return null;
	}
	visiting.add(nodeId);
	const handleItems: TreeDataItem[] = node.handles.map((handle) => ({
		id: `board-play-hierarchy.handle.${handle.id}`,
		label: handle.handleKind ? `${handle.id} · ${handle.handleKind}` : handle.id,
		isSelected: selectedIds.has(handle.id),
		onClick: () => onSelect(handle.id),
	}));
	const handlesGroup: TreeDataItem = {
		id: `board-play-hierarchy.node.${nodeId}.handles`,
		label: "Handles",
		defaultOpen: true,
		items: handleItems.length ? handleItems : [{ id: `board-play-hierarchy.node.${nodeId}.handles.empty`, label: "(none)" }],
	};
	const childItems: TreeDataItem[] = [];
	for (const childId of childrenByParent.get(nodeId) ?? []) {
		const childItem = buildBoardFixtureNodeHierarchyItem(fixture, childId, childrenByParent, selectedIds, onSelect, visiting);
		if (childItem) {
			childItems.push(childItem);
		}
	}
	visiting.delete(nodeId);
	return {
		id: `board-play-hierarchy.node.${nodeId}`,
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
): TreeDataSection[] {
	const selectedIds = new Set(selectionIds);
	const childrenByParent = boardFixtureChildrenByNodeId(fixture);
	const rootIds = boardFixtureRootNodeIds(fixture, childrenByParent);
	const visiting = new Set<string>();
	const nodeItems: TreeDataItem[] = [];
	for (const rootId of rootIds) {
		const item = buildBoardFixtureNodeHierarchyItem(fixture, rootId, childrenByParent, selectedIds, onSelect, visiting);
		if (item) {
			nodeItems.push(item);
		}
	}
	const nodesGroup: TreeDataItem = {
		id: "board-play-hierarchy.nodes",
		label: "Nodes",
		defaultOpen: true,
		items: nodeItems.length ? nodeItems : [{ id: "board-play-hierarchy.nodes.empty", label: "(none)" }],
	};
	const edgeItems: TreeDataItem[] = fixture.edges.map((edge) => ({
		id: `board-play-hierarchy.edge.${edge.id}`,
		label: edge.id,
		description: `${edge.source} → ${edge.target}`,
		isSelected: selectedIds.has(edge.id),
		onClick: () => onSelect(edge.id),
	}));
	const edgesGroup: TreeDataItem = {
		id: "board-play-hierarchy.edges",
		label: "Edges",
		defaultOpen: true,
		items: edgeItems.length ? edgeItems : [{ id: "board-play-hierarchy.edges.empty", label: "(none)" }],
	};
	const boardRoot: TreeDataItem = {
		id: "board-play-hierarchy.board",
		label: "Board",
		defaultOpen: true,
		items: [nodesGroup, edgesGroup],
	};
	return playgroundTreePanelRootItems("board-play-hierarchy.root", [boardRoot]);
}
//#endregion 🔖BoardPlayHierarchy

//#region 🔖Controller
const BOARD_PLAY_TARGET_KINDS = ["nodes", "edges", "handles"] as const;
type BoardPlayTargetKind = (typeof BOARD_PLAY_TARGET_KINDS)[number];

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
		...buildPlaygroundKindToggleTools("selection", BOARD_PLAY_TARGET_KINDS, boardPlayTargetLabel, targetRecord, controllerId, "toggleSelectionTarget"),
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
	private lodModeByPane: Record<BoardPlayPaneId, BoardLodModeKind>;
	private effectiveLodByPane: Record<BoardPlayPaneId, BoardDrawLodKind>;
	private hostBridge: BoardPlayHostBridge | null = null;

	constructor(commandBus: CommandBus, hostNotify: () => void) {
		super(BOARD_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.lodModeByPane = {
			"board-detail": BOARD_LOD_MODE_AUTOMATIC,
			"board-overview": BOARD_LOD_MODE_AUTOMATIC,
			"board-selection": BOARD_LOD_MODE_AUTOMATIC,
		};
		this.effectiveLodByPane = {
			"board-detail": "normal",
			"board-overview": "normal",
			"board-selection": "normal",
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

	private lodMeasureForPane(paneId: BoardPlayPaneId): WindowMeasure {
		return {
			kind: "select",
			id: `${paneId}-lod`,
			label: "LOD",
			value: this.lodModeByPane[paneId],
			items: [
				{ id: "automatic", value: BOARD_LOD_MODE_AUTOMATIC, label: boardLodAutomaticSelectLabel(this.effectiveLodByPane[paneId]) },
				...BOARD_PLAY_LOD_TIERS.map((tier) => ({ id: tier, value: tier, label: boardPlayLodTierMenuLabel(tier) })),
			],
			onChange: { controllerId: BOARD_PLAY_CONTROLLER_ID, command: "setLodModeForPane", args: { pane: paneId } },
		};
	}

	private rebuildShellMode(): void {
		this.mainMode.windowKinds = [
			new WindowKindRuntime("board-overview", "Overview", BOARD_PLAY_BODY_KEY_OVERVIEW, undefined, [this.lodMeasureForPane("board-overview")]),
			new WindowKindRuntime("board-detail", "Zoom", BOARD_PLAY_BODY_KEY_DETAIL, undefined, [this.lodMeasureForPane("board-detail")]),
			new WindowKindRuntime("board-selection", "Selection", BOARD_PLAY_BODY_KEY_SELECTION, undefined, [this.lodMeasureForPane("board-selection")]),
		];
	}

	override run(command: string, args?: unknown): void {
		switch (command) {
			case "setLodModeForPane": {
				const { pane, value } = args as { pane: BoardPlayPaneId; value?: string };
				if (pane !== "board-overview" && pane !== "board-detail" && pane !== "board-selection") break;
				if (value === BOARD_LOD_MODE_AUTOMATIC || (typeof value === "string" && isBoardDrawLodKind(value))) {
					this.lodModeByPane = { ...this.lodModeByPane, [pane]: value as BoardLodModeKind };
				}
				break;
			}
			case "setEffectiveLodForPane": {
				const { pane, lod } = args as { pane: BoardPlayPaneId; lod: BoardDrawLodKind };
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

	getLodModeByPane(): Readonly<Record<BoardPlayPaneId, BoardLodModeKind>> {
		return this.lodModeByPane;
	}

	getEffectiveLodByPane(): Readonly<Record<BoardPlayPaneId, BoardDrawLodKind>> {
		return this.effectiveLodByPane;
	}
}
//#endregion 🔖Controller

//#region 🔖DeclarativeBodies
function boardPlayControllerFromContext(ctx: WindowBodyViewContext): BoardPlayShellController | undefined {
	return ctx.runtime.getActiveApp()?.controller as BoardPlayShellController | undefined;
}

function buildBoardPlayDeclarativeBody(paneId: BoardPlayPaneId): (ctx: WindowBodyViewContext) => UiNode {
	return (ctx) => {
		if (!boardPlayControllerFromContext(ctx)) {
			return { type: "text", value: "Missing board play controller" };
		}
		return buildBoardWindowBody(BOARD_PLAY_BOARD_SURFACE_ID, BOARD_PLAY_CONTROLLER_ID, paneId);
	};
}

export const buildBoardPlayOverviewDeclarativeBody = buildBoardPlayDeclarativeBody("board-overview");
export const buildBoardPlayDetailDeclarativeBody = buildBoardPlayDeclarativeBody("board-detail");
export const buildBoardPlaySelectionDeclarativeBody = buildBoardPlayDeclarativeBody("board-selection");
//#endregion 🔖DeclarativeBodies

/** @emoji 🧩 Registers board play window kinds on the supplied controller (layout supplied by host). */
export function attachBoardPlayWindowKinds(controller: BoardPlayShellController, layout: unknown): AppRuntime {
	const app = new AppRuntime(BOARD_PLAY_APP_ID, "Board", undefined, controller, layout as never, []);
	app.defaultModeId = controller.mainMode.id;
	app.addMode(controller.mainMode);
	return app;
}

/** @emoji 🧩 Builds the board play {@link AppRuntime}; side panels are tree tabs via {@link PlaygroundView} `augmentPanelTabs` only. */
export function buildBoardPlayAppRuntime(controller: BoardPlayShellController): AppRuntime {
	const app = attachBoardPlayWindowKinds(controller, BOARD_PLAY_LAYOUT);
	app.leftTabs = [];
	app.rightTabs = [];
	return app;
}

/** @emoji 📝 Registers board play declarative window bodies on the playground host (side tabs are host tree panels only). */
export function registerBoardPlayDeclarativeBodies(): void {
	registerWindowBody(BOARD_PLAY_BODY_KEY_OVERVIEW, buildBoardPlayOverviewDeclarativeBody);
	registerWindowBody(BOARD_PLAY_BODY_KEY_DETAIL, buildBoardPlayDetailDeclarativeBody);
	registerWindowBody(BOARD_PLAY_BODY_KEY_SELECTION, buildBoardPlaySelectionDeclarativeBody);
}

//#region 🔖Extension
/** @emoji 🔌 Host context for optional board-play extension activation. */
export interface BoardPlayPluginContext {
	registerWindowBody(bodyKey: string, factory: (ctx: WindowBodyViewContext) => UiNode): void;
}

/** @emoji 📦 Extension manifest shape for board play (host-agnostic). */
export interface BoardPlayExtensionManifest {
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

export const BOARD_PLAY_EXTENSION_MANIFEST: BoardPlayExtensionManifest = {
	id: "elements.board-play",
	label: "Board Play",
	version: "0.1.0",
	contributes: {
		apps: [
			{
				id: BOARD_PLAY_APP_ID,
				label: "Board",
				controllerId: BOARD_PLAY_CONTROLLER_ID,
				defaultLayout: BOARD_PLAY_LAYOUT,
				defaultModeId: "main",
				windowKinds: [
					{ id: "board-overview", label: "Overview", bodyKey: BOARD_PLAY_BODY_KEY_OVERVIEW },
					{ id: "board-detail", label: "Zoom", bodyKey: BOARD_PLAY_BODY_KEY_DETAIL },
					{ id: "board-selection", label: "Selection", bodyKey: BOARD_PLAY_BODY_KEY_SELECTION },
				],
				modes: [{ id: "main", label: "Board" }],
			},
		],
	},
};

/** @emoji 🔌 Board play plugin: registers declarative bodies on activate. */
export const boardPlayPlugin: { readonly id: string; activate(context: BoardPlayPluginContext): void } = {
	id: BOARD_PLAY_EXTENSION_MANIFEST.id,
	activate(context: BoardPlayPluginContext): void {
		context.registerWindowBody(BOARD_PLAY_BODY_KEY_OVERVIEW, buildBoardPlayOverviewDeclarativeBody);
		context.registerWindowBody(BOARD_PLAY_BODY_KEY_DETAIL, buildBoardPlayDetailDeclarativeBody);
		context.registerWindowBody(BOARD_PLAY_BODY_KEY_SELECTION, buildBoardPlaySelectionDeclarativeBody);
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
//#endregion 🔖Extension

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("board play declarative shell", () => {
		it("declarative overview body references board host surface", () => {
			const runtime = buildBoardPlayRuntime();
			const tree = buildBoardPlayOverviewDeclarativeBody({
				runtime,
				windowKindId: "board-overview",
				bodyKey: BOARD_PLAY_BODY_KEY_OVERVIEW,
				activeModeId: "main",
				generation: 0,
			});
			expect(tree).toEqual(buildBoardWindowBody(BOARD_PLAY_BOARD_SURFACE_ID, BOARD_PLAY_CONTROLLER_ID, "board-overview"));
		});

		it("buildBoardPlayHierarchySections nests root nodes, handles, and child nodes", () => {
			const fixture = parseBoardFixtureV1({
				schema: "elements.board.fixture/v1",
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
			const sections = buildBoardPlayHierarchySections(fixture!, [], () => {});
			const boardRoot = sections[0]?.items?.[0];
			expect(boardRoot?.label).toBe("Board");
			const nodesGroup = boardRoot?.items?.find((row) => row.label === "Nodes");
			expect(nodesGroup?.items?.[0]?.id).toBe("board-play-hierarchy.node.root");
			expect(nodesGroup?.items?.[0]?.items?.[0]?.label).toBe("Handles");
			expect(nodesGroup?.items?.[0]?.items?.[1]?.id).toBe("board-play-hierarchy.node.child");
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
