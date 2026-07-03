// #region 🧲Header
/** @emoji 🧩 Puzzle 2D play app — flat puzzle editor with Jack. */
// #endregion 🧲Header

import {
	createPlaygroundApp,
	createProductPlaygroundPlatform,
	CommandBus,
	Controller,
	registerWindowBody,
	Platform,
	AppRuntime,
	ModeRuntime,
	WindowKindRuntime,
	buildPuzzle2dWindowBody,
	buildPlaygroundKindToggleTools,
	createNamedLayout,
	createWindowLayout,
	type WindowTemplate,
	platformFromViewContext,
	type AppTools,
	type ToolLeaf,
	toolCollection,
	type WindowBodyViewContext,
	type CommandDescriptor,
	type WindowEngagement,
	type WindowEngagementControl,
	type WindowMeasure,
	type UiNode,
	type UiTreeContextMenuItem,
	type UiTreeItemNode,
	type UiTreeSectionNode,
	type UiTreeNode,
	type WindowLayout,
	enforcePlaygroundWindowEngagementInput,
	windowEngagementsEqual,
	normalizeKindWeightGroup,
	syncKindWeightMap,
	type KindWeightMap,
	WireHoverBridge,
	buildWriterWindowBody,
} from "@semio-tech/framework-playground-core";
import { registerOsMediaExportHandler } from "@semio-tech/framework-os-core";
import { rasterizeSvgMarkupToPngDataUrl } from "@semio-tech/kernel-2d-js";
import { createWriterDocument, type WriterDocument } from "@semio-tech/writer-core";
import { wireLiteralFromDagFixtureJson } from "@semio-tech/graph-dsl-core";

import {
	DEFAULT_KIND_CATALOG_BUNDLE,
	PUZZLE_2D_LOD_MODE_AUTOMATIC,
	puzzle2dFixtureMergedKindCatalogs,
	puzzle2dMergeKindCatalogBundle,
	puzzle2dFixtureEdgeDisplayLabel,
	puzzle2dFixtureHandleDisplayLabel,
	puzzle2dFixtureNodeDisplayDescription,
	puzzle2dFixtureNodeDisplayLabel,
	puzzle2dHandleKindOverlayLabel,
	puzzle2dLodAutomaticSelectLabel,
	puzzle2dNodeKindOverlayLabel,
	decodePuzzle2dFixtureFromDrag,
	puzzle2dPlayNodeKindDragData,
	puzzle2dNodeKindCatalogIcon,
	isPuzzle2dDrawLodKind,
	getPuzzle2dLodScale,
	parsePuzzle2dFixture,
	PUZZLE_2D_CAMERA_ZOOM_MAX,
	PUZZLE_2D_CAMERA_ZOOM_MIN,
	DEFAULT_PUZZLE_2D_SUGGESTION_OFFSET_PX,
	DEFAULT_PUZZLE_2D_BRUSH_NODE_SIZE_PX,
	type EdgeKind,
	type HandleKind,
	type KindCatalogBundle,
	type NodeKind,
	type NodeKindHandleTemplate,
	type WireKind,
	type Puzzle2dActiveTool,
	type Puzzle2dDrawLodKind,
	type Puzzle2dFixtureNode,
	type Puzzle2dFixture,
	type Puzzle2dLodModeKind,
	type Puzzle2dSelectionMethod,
	type Puzzle2dSelectionMode,
	type Puzzle2dSelectionTargets,
	type Puzzle2dKindHover,
	type Puzzle2dKindHoverDomain,
	type Puzzle2dHoverPayload,
	type CameraState,
	applyBrushFillPlacementsToFixture,
	clonePuzzle2dFixture,
	PUZZLE_2D_FILL_BUILD_CHUNK_BUDGET,
	PUZZLE_2D_FILL_COUNT_MAX,
	type Puzzle2dBrushPlacePayload,
	type Puzzle2dFillBuildProgress,
	type Puzzle2dRenderer,
} from "../../react/index.tsx";

import { bootstrapElementsSurfaceChromeDocument } from "@semio-tech/ui-react";

export { PUZZLE_2D_FILL_COUNT_MAX };

//#region 🔖Ids
export type Puzzle2dPlayPaneId = "2d-overview" | "2d-detail" | "2d-selection";

export const PUZZLE_2D_PLAY_APP_ID = "puzzle-2d-play";
export const PUZZLE_2D_PLAY_CONTROLLER_ID = "puzzle-2d-play";
export const PUZZLE_2D_PLAY_SURFACE_ID = "puzzle.2d.play";
export const PUZZLE_2D_PLAY_WINDOW_KIND_COMPILED_DAG = "2d-compiled-dag";
export const PUZZLE_2D_PLAY_SURFACE_ID_COMPILED_DAG = "puzzle.2d.play.compiled-dag";
export const PUZZLE_2D_PLAY_BODY_KEY_COMPILED_DAG = "puzzle.2d.play.compiled-dag";

export const PUZZLE_2D_PLAY_BODY_KEY_OVERVIEW = "puzzle.2d.play.overview";
export const PUZZLE_2D_PLAY_BODY_KEY_DETAIL = "puzzle.2d.play.detail";
export const PUZZLE_2D_PLAY_BODY_KEY_SELECTION = "puzzle.2d.play.selection";
export const PUZZLE_2D_PLAY_SETTINGS_BODY_KEY = "puzzle.2d.play.settings";

export const PUZZLE_2D_PLAY_LOD_TIERS: Puzzle2dDrawLodKind[] = getPuzzle2dLodScale().map((lod) => lod.id);

export function puzzle2dPlayLodTierMenuLabel(tier: Puzzle2dDrawLodKind): string {
	const row = getPuzzle2dLodScale().find((lod) => lod.id === tier);
	return row?.name ?? tier.charAt(0).toUpperCase() + tier.slice(1);
}

export const PUZZLE_2D_PLAY_HIERARCHY_TAB_ID = "puzzle-2d-play-hierarchy";
export const PUZZLE_2D_PLAY_KINDS_TAB_ID = "puzzle-2d-play-kinds";
export const PUZZLE_2D_PLAY_ICON_KINDS = "puzzle.2d-play.icon.kinds";

/** @emoji 🖌️ Window engagement possible id for the brush tool. */
export const PUZZLE_2D_ENGAGEMENT_TOOL_BRUSH_ID = "puzzle2d.tool.brush";

/** @emoji 🖱️ Window engagement possible id for the select tool. */
export const PUZZLE_2D_ENGAGEMENT_TOOL_SELECT_ID = "puzzle2d.tool.select";

/** @emoji 🪣 Window engagement possible id for the fill tool. */
export const PUZZLE_2D_ENGAGEMENT_TOOL_FILL_ID = "puzzle2d.tool.fill";

const PUZZLE_2D_SUGGESTION_OFFSET_SLIDER_MIN = 0;
const PUZZLE_2D_SUGGESTION_OFFSET_SLIDER_MAX = 160;
const PUZZLE_2D_SUGGESTION_OFFSET_SLIDER_STEP = 4;

const PUZZLE_2D_FILL_COUNT_SLIDER_MIN = 0;
const PUZZLE_2D_FILL_COUNT_SLIDER_MAX = PUZZLE_2D_FILL_COUNT_MAX;
const PUZZLE_2D_FILL_COUNT_SLIDER_STEP = 1;

/** @emoji 🪣 Cached fill session for O(1) prefix application on the play host. */
export type Puzzle2dFillSessionState = {
	readonly baseFixture: Puzzle2dFixture | null;
	readonly sequence: readonly Puzzle2dBrushPlacePayload[];
	readonly appendedNodes: readonly Puzzle2dFixtureNode[];
	readonly appendedEdges: readonly Puzzle2dFixture["edges"][number][];
	readonly seed: number;
};

/** @emoji 🪣 Latest fill build progress (updated each chunked step). */
export const puzzle2dFillBuildProgressRef: { current: Puzzle2dFillBuildProgress } = {
	current: { count: 0, maxCount: PUZZLE_2D_FILL_COUNT_MAX, done: false },
};

export const puzzle2dFillSessionRef: { current: Puzzle2dFillSessionState } = {
	current: { baseFixture: null, sequence: [], appendedNodes: [], appendedEdges: [], seed: 0 },
};

let puzzle2dFillBuildTimer: ReturnType<typeof setTimeout> | null = null;
let puzzle2dFillSessionReadyEpoch = 0;
const puzzle2dFillSessionReadyListeners = new Set<() => void>();

/** @emoji 🪣 Subscribes to fill session rebuilds. */
export function subscribePuzzle2dFillSessionReady(listener: () => void): () => void {
	puzzle2dFillSessionReadyListeners.add(listener);
	return () => {
		puzzle2dFillSessionReadyListeners.delete(listener);
	};
}

/** @emoji 🪣 Epoch bumped when a fill session is prepared or extended. */
export function getPuzzle2dFillSessionReadyEpoch(): number {
	return puzzle2dFillSessionReadyEpoch;
}

function notifyPuzzle2dFillSessionReady(): void {
	puzzle2dFillSessionReadyEpoch += 1;
	for (const listener of puzzle2dFillSessionReadyListeners) {
		listener();
	}
}

function cancelPuzzle2dFillBuild(): void {
	if (puzzle2dFillBuildTimer !== null) {
		clearTimeout(puzzle2dFillBuildTimer);
		puzzle2dFillBuildTimer = null;
	}
}

function nextPuzzle2dFillSeed(): number {
	return (Date.now() ^ Math.floor(Math.random() * 0x7fffffff)) >>> 0;
}

function puzzle2dFillAppendedSlice(
	core: Puzzle2dFixture,
	sequence: readonly Puzzle2dBrushPlacePayload[],
	catalogs?: KindCatalogBundle,
): Pick<Puzzle2dFillSessionState, "appendedNodes" | "appendedEdges"> {
	if (sequence.length === 0) {
		return { appendedNodes: [], appendedEdges: [] };
	}
	const applied = applyBrushFillPlacementsToFixture(core, sequence, catalogs);
	return {
		appendedNodes: applied.nodes.slice(core.nodes.length),
		appendedEdges: applied.edges.slice(core.edges.length),
	};
}

function composePuzzle2dFillFixture(
	base: Puzzle2dFixture,
	appendedNodes: readonly Puzzle2dFixtureNode[],
	appendedEdges: readonly Puzzle2dFixture["edges"][number][],
	count: number,
): Puzzle2dFixture {
	if (count <= 0) {
		return base;
	}
	return {
		...base,
		nodes: [...base.nodes, ...appendedNodes.slice(0, count)],
		edges: [...base.edges, ...appendedEdges.slice(0, count)],
	};
}

/** @emoji 🪣 Applies a fill prefix count onto the cached base fixture. */
export function applyPuzzle2dFillCount(count: number, catalogs?: KindCatalogBundle): Puzzle2dFixture | null {
	const session = puzzle2dFillSessionRef.current;
	if (!session.baseFixture) {
		return null;
	}
	const available = session.sequence.length;
	const n = Math.max(0, Math.min(PUZZLE_2D_FILL_COUNT_MAX, Math.round(count), available));
	return composePuzzle2dFillFixture(session.baseFixture, session.appendedNodes, session.appendedEdges, n);
}

/** @emoji 🪣 Clears the cached fill session and returns the base fixture when present. */
export function clearPuzzle2dFillSession(renderer?: Puzzle2dRenderer | null): Puzzle2dFixture | null {
	cancelPuzzle2dFillBuild();
	renderer?.endBrushFillSession();
	const base = puzzle2dFillSessionRef.current.baseFixture;
	puzzle2dFillSessionRef.current = { baseFixture: null, sequence: [], appendedNodes: [], appendedEdges: [], seed: 0 };
	puzzle2dFillBuildProgressRef.current = { count: 0, maxCount: PUZZLE_2D_FILL_COUNT_MAX, done: false };
	notifyPuzzle2dFillSessionReady();
	return base;
}

/** @emoji 🪣 Starts a chunked fill session build against a fixture snapshot. */
export function preparePuzzle2dFillSession(
	baseFixture: Puzzle2dFixture,
	renderer: Puzzle2dRenderer | null | undefined,
	kindCatalogs?: KindCatalogBundle,
): void {
	cancelPuzzle2dFillBuild();
	if (!renderer) {
		return;
	}
	const core = clonePuzzle2dFixture(baseFixture);
	const seed = nextPuzzle2dFillSeed();
	puzzle2dFillSessionRef.current = {
		baseFixture: core,
		sequence: [],
		appendedNodes: [],
		appendedEdges: [],
		seed,
	};
	puzzle2dFillBuildProgressRef.current = { count: 0, maxCount: PUZZLE_2D_FILL_COUNT_MAX, done: false };
	notifyPuzzle2dFillSessionReady();
	renderer.beginBrushFillSession(core, PUZZLE_2D_FILL_COUNT_MAX, seed);
	const tick = (): void => {
		const started = performance.now();
		const step = renderer.stepBrushFillSession(PUZZLE_2D_FILL_BUILD_CHUNK_BUDGET);
		const session = puzzle2dFillSessionRef.current;
		const nextSequence = [...session.sequence, ...step.placements];
		const appended = puzzle2dFillAppendedSlice(core, nextSequence, kindCatalogs);
		puzzle2dFillSessionRef.current = {
			...session,
			sequence: nextSequence,
			appendedNodes: appended.appendedNodes,
			appendedEdges: appended.appendedEdges,
		};
		puzzle2dFillBuildProgressRef.current = {
			count: step.count,
			maxCount: PUZZLE_2D_FILL_COUNT_MAX,
			done: step.done,
		};
		console.log(
			`[DEBUG] puzzle2d fill build chunk count=${step.count}/${PUZZLE_2D_FILL_COUNT_MAX} done=${step.done} ms=${(performance.now() - started).toFixed(1)}`,
		);
		notifyPuzzle2dFillSessionReady();
		if (!step.done) {
			puzzle2dFillBuildTimer = setTimeout(tick, 0);
			return;
		}
		renderer.endBrushFillSession();
		puzzle2dFillBuildTimer = null;
	};
	puzzle2dFillBuildTimer = setTimeout(tick, 0);
}

const PUZZLE_2D_PLAY_WINDOW_SPECS: { readonly pane: Puzzle2dPlayPaneId; readonly label: string; readonly bodyKey: string }[] = [
	{ pane: "2d-overview", label: "Overview", bodyKey: PUZZLE_2D_PLAY_BODY_KEY_OVERVIEW },
	{ pane: "2d-detail", label: "Zoom", bodyKey: PUZZLE_2D_PLAY_BODY_KEY_DETAIL },
	{ pane: "2d-selection", label: "Selection", bodyKey: PUZZLE_2D_PLAY_BODY_KEY_SELECTION },
];

const PUZZLE_2D_PANE_TEMPLATES: Record<Puzzle2dPlayPaneId, readonly WindowTemplate[]> = {
	"2d-overview": [
		{ id: "overview-auto", label: "Overview auto LOD", controllerId: PUZZLE_2D_PLAY_CONTROLLER_ID, command: "setLodModeForPane", args: { pane: "2d-overview", value: PUZZLE_2D_LOD_MODE_AUTOMATIC } },
		{ id: "overview-overview", label: "Overview LOD", controllerId: PUZZLE_2D_PLAY_CONTROLLER_ID, command: "setLodModeForPane", args: { pane: "2d-overview", value: "overview" } },
	],
	"2d-detail": [
		{ id: "detail-zoom", label: "Detail zoom LOD", controllerId: PUZZLE_2D_PLAY_CONTROLLER_ID, command: "setLodModeForPane", args: { pane: "2d-detail", value: "detail" } },
	],
	"2d-selection": [
		{ id: "selection-focus", label: "Selection focus", controllerId: PUZZLE_2D_PLAY_CONTROLLER_ID, command: "setLodModeForPane", args: { pane: "2d-selection", value: PUZZLE_2D_LOD_MODE_AUTOMATIC } },
	],
};

/** @emoji 🪟 Maps shell instance id (or pane id on bootstrap) to puzzle 2d pane. */
export function puzzle2dPlayPaneFromShellWindowId(shellWindowId: string): Puzzle2dPlayPaneId | null {
	if (shellWindowId === "2d-overview" || shellWindowId === "2d-detail" || shellWindowId === "2d-selection") {
		return shellWindowId;
	}
	const match = /^win-(2d-(?:overview|detail|selection))-/.exec(shellWindowId);
	if (!match?.[1]) {
		return null;
	}
	const pane = match[1];
	if (pane === "2d-overview" || pane === "2d-detail" || pane === "2d-selection") {
		return pane;
	}
	return null;
}

export function puzzle2dPlayCmd(command: string, args?: Record<string, unknown>): CommandDescriptor {
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
					{ kind: "stack", size: 25, children: [createWindowLayout("2d-detail", "Zoom")] },
					{ kind: "stack", size: 25, children: [createWindowLayout("2d-selection", "Selection")] },
					{ kind: "stack", size: 50, children: [createWindowLayout(PUZZLE_2D_PLAY_WINDOW_KIND_COMPILED_DAG, "DSL")] },
				],
			},
		],
	},
};
//#endregion 🔖Ids

//#region 🔖Puzzle2dPlayHover
function puzzle2dPlayHoverPayloadFromGraphId(fixture: Puzzle2dFixture, graphId: string | null): Puzzle2dHoverPayload {
	if (!graphId) {
		return { clientX: 0, clientY: 0, id: null, kind: null, screenX: 0, screenY: 0, worldX: 0, worldY: 0 };
	}
	const node = fixture.nodes.find((row) => row.id === graphId);
	if (node) {
		return { clientX: 0, clientY: 0, id: graphId, kind: { domain: "node", kindId: node.nodeKind ?? graphId }, screenX: 0, screenY: 0, worldX: 0, worldY: 0 };
	}
	const edge = fixture.edges.find((row) => row.id === graphId);
	if (edge) {
		return { clientX: 0, clientY: 0, id: graphId, kind: { domain: "edge", kindId: edge.edgeKind ?? graphId }, screenX: 0, screenY: 0, worldX: 0, worldY: 0 };
	}
	for (const fixtureNode of fixture.nodes) {
		const handle = fixtureNode.handles.find((row) => row.id === graphId);
		if (handle) {
			return { clientX: 0, clientY: 0, id: graphId, kind: { domain: "handle", kindId: handle.handleKind }, screenX: 0, screenY: 0, worldX: 0, worldY: 0 };
		}
	}
	return { clientX: 0, clientY: 0, id: graphId, kind: null, screenX: 0, screenY: 0, worldX: 0, worldY: 0 };
}

function puzzle2dPlayKindRowHoverHandlers(
	onHover: ((payload: Puzzle2dHoverPayload) => void) | undefined,
	kind: Puzzle2dKindHover,
): Pick<UiTreeItemNode, "onPointerEnter" | "onPointerLeave"> {
	if (!onHover) {
		return {};
	}
	const payload: Puzzle2dHoverPayload = { clientX: 0, clientY: 0, id: null, kind, screenX: 0, screenY: 0, worldX: 0, worldY: 0 };
	return {
		onPointerEnter: () => onHover(payload),
		onPointerLeave: () => onHover({ ...payload, kind: null }),
	};
}

/** @emoji 🌳 Maps transitive kind hover to workbench hierarchy tree item ids. */
export function puzzle2dPlayHierarchyTreeHighlightedIdsForKind(
	fixture: Puzzle2dFixture,
	kindHover: Puzzle2dKindHover | null,
): readonly string[] {
	if (!kindHover?.kindId) {
		return [];
	}
	const ids: string[] = [];
	if (kindHover.domain === "node") {
		for (const node of fixture.nodes) {
			if ((node.nodeKind ?? node.id) === kindHover.kindId) {
				ids.push(`puzzle-2d-play-hierarchy.node.${node.id}`);
			}
		}
		return ids;
	}
	if (kindHover.domain === "handle") {
		for (const node of fixture.nodes) {
			for (const handle of node.handles) {
				if (handle.handleKind === kindHover.kindId) {
					ids.push(`puzzle-2d-play-hierarchy.handle.${handle.id}`);
				}
			}
		}
		return ids;
	}
	if (kindHover.domain === "edge") {
		for (const edge of fixture.edges) {
			if ((edge.edgeKind ?? edge.id) === kindHover.kindId) {
				ids.push(`puzzle-2d-play-hierarchy.edge.${edge.id}`);
			}
		}
		return ids;
	}
	for (const wire of fixture.wires ?? []) {
		if (wire.wireKind === kindHover.kindId) {
			ids.push(`puzzle-2d-play-hierarchy.wire.${wire.id}`);
		}
	}
	return ids;
}

function puzzle2dPlayKindsSectionDomain(sectionId: string): Puzzle2dKindHoverDomain | null {
	if (sectionId === "puzzle-2d-play-kinds.nodes") {
		return "node";
	}
	if (sectionId === "puzzle-2d-play-kinds.handles") {
		return "handle";
	}
	if (sectionId === "puzzle-2d-play-kinds.edges") {
		return "edge";
	}
	if (sectionId === "puzzle-2d-play-kinds.wires") {
		return "wire";
	}
	return null;
}
//#endregion 🔖Puzzle2dPlayHover

//#region 🔖Puzzle2dPlayHierarchy
/** @emoji 🖼️ Default tree-row icons for puzzle 2D entity kinds (Lucide catalog ids). */
export const PUZZLE2D_PLAY_ENTITY_TREE_ICON = {
	node: "shapes",
	handle: "circle-dot",
	edge: "link",
	wire: "plug",
} as const;

type Puzzle2dPlayEntityTreeKind = keyof typeof PUZZLE2D_PLAY_ENTITY_TREE_ICON;

/** @emoji 🖼️ Resolves the tree-row icon id for a puzzle 2D entity kind. */
export function puzzle2dPlayEntityTreeIcon(kind: Puzzle2dPlayEntityTreeKind): string {
	return PUZZLE2D_PLAY_ENTITY_TREE_ICON[kind];
}

function puzzle2dPlayKindSectionTreeIcon(sectionId: string): string | undefined {
	if (sectionId === "puzzle-2d-play-kinds.nodes") {
		return puzzle2dPlayEntityTreeIcon("node");
	}
	if (sectionId === "puzzle-2d-play-kinds.handles") {
		return puzzle2dPlayEntityTreeIcon("handle");
	}
	if (sectionId === "puzzle-2d-play-kinds.edges") {
		return puzzle2dPlayEntityTreeIcon("edge");
	}
	if (sectionId === "puzzle-2d-play-kinds.wires") {
		return puzzle2dPlayEntityTreeIcon("wire");
	}
	return undefined;
}

function puzzle2dPlayNodeHierarchyTreeIcon(node: Puzzle2dFixtureNode, kindCatalogs: KindCatalogBundle): string {
	const kindId = node.nodeKind?.trim();
	if (!kindId) {
		return puzzle2dPlayEntityTreeIcon("node");
	}
	return puzzle2dNodeKindCatalogIcon(kindCatalogs.nodes?.find((row) => row.id === kindId)) ?? puzzle2dPlayEntityTreeIcon("node");
}

function puzzle2dFixtureHandleToNodeId(fixture: Puzzle2dFixture): ReadonlyMap<string, string> {
	const out = new Map<string, string>();
	for (const node of fixture.nodes) {
		for (const handle of node.handles) {
			out.set(handle.id, node.id);
		}
	}
	return out;
}

function puzzle2dFixtureEdgeEndpointNodeId(
	fixture: Puzzle2dFixture,
	endpointId: string,
	handleToNode: ReadonlyMap<string, string>,
): string | undefined {
	const viaHandle = handleToNode.get(endpointId);
	if (viaHandle) {
		return viaHandle;
	}
	return fixture.nodes.some((node) => node.id === endpointId) ? endpointId : undefined;
}

function puzzle2dFixtureChildrenByNodeId(fixture: Puzzle2dFixture): ReadonlyMap<string, readonly string[]> {
	const handleToNode = puzzle2dFixtureHandleToNodeId(fixture);
	const out = new Map<string, string[]>();
	for (const edge of fixture.edges) {
		const parentId = puzzle2dFixtureEdgeEndpointNodeId(fixture, edge.source, handleToNode);
		const childId = puzzle2dFixtureEdgeEndpointNodeId(fixture, edge.target, handleToNode);
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

function puzzle2dFixtureRootNodeIds(fixture: Puzzle2dFixture, childrenByParent: ReadonlyMap<string, readonly string[]>): readonly string[] {
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

/** @emoji ⌨️ Select-all ids for the fixture honoring playground node/edge/handle target toggles. */
export function puzzle2dPlayAllSelectionFromFixture(fixture: Puzzle2dFixture, targets: Puzzle2dSelectionTargets): readonly string[] {
	const ids: string[] = [];
	if (targets.nodes) {
		for (const node of fixture.nodes) {
			ids.push(node.id);
		}
	}
	if (targets.handles) {
		for (const node of fixture.nodes) {
			for (const handle of node.handles) {
				ids.push(handle.id);
			}
		}
	}
	if (targets.edges) {
		for (const edge of fixture.edges) {
			ids.push(edge.id);
		}
	}
	return ids;
}

/** @emoji 🌳 Maps committed graph selection ids to workbench hierarchy tree item ids. */
export function puzzle2dPlayHierarchyTreeSelectedIds(fixture: Puzzle2dFixture, graphSelectionIds: readonly string[]): string[] {
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
	/** @emoji 🖱️ Optional hover sink for hierarchy row pointer enter/leave. */
	readonly onHover?: (payload: Puzzle2dHoverPayload) => void;
	readonly onToggleHidden?: (graphId: string) => void;
	readonly onToggleLocked?: (graphId: string) => void;
};

/** @emoji 🎯 Reads hidden/locked flags for a fixture graph id. */
export function puzzle2dPlayEntityFlagsFromFixture(fixture: Puzzle2dFixture, graphId: string): { hidden: boolean; locked: boolean } {
	for (const node of fixture.nodes) {
		if (node.id === graphId) {
			return { hidden: node.hidden === true, locked: node.locked === true };
		}
		for (const handle of node.handles) {
			if (handle.id === graphId) {
				return { hidden: handle.hidden === true, locked: handle.locked === true };
			}
		}
	}
	const edge = fixture.edges.find((row) => row.id === graphId);
	if (edge) {
		return { hidden: edge.hidden === true, locked: edge.locked === true };
	}
	return { hidden: false, locked: false };
}

/** @emoji 🙈 Sets hidden/locked on every selected fixture row. */
export function puzzle2dPlayApplySelectionFlag(
	fixture: Puzzle2dFixture,
	selectionIds: readonly string[],
	flag: "hidden" | "locked",
	value: boolean,
): Puzzle2dFixture {
	const selected = new Set(selectionIds);
	return {
		...fixture,
		nodes: fixture.nodes.map((node) => {
			const handles = node.handles.map((handle) => (selected.has(handle.id) ? { ...handle, [flag]: value } : handle));
			const nodeSelected = selected.has(node.id);
			if (!nodeSelected && handles === node.handles) {
				return node;
			}
			return {
				...node,
				...(nodeSelected ? { [flag]: value } : {}),
				handles,
			};
		}),
		edges: fixture.edges.map((edge) => (selected.has(edge.id) ? { ...edge, [flag]: value } : edge)),
	};
}

/** @emoji 🔁 Toggles hidden/locked on one hierarchy row. */
export function puzzle2dPlayToggleEntityFlag(fixture: Puzzle2dFixture, graphId: string, flag: "hidden" | "locked"): Puzzle2dFixture {
	const flags = puzzle2dPlayEntityFlagsFromFixture(fixture, graphId);
	return puzzle2dPlayApplySelectionFlag(fixture, [graphId], flag, !(flags[flag] === true));
}

/** @emoji 🗑️ Removes selected nodes, handles, and edges from a fixture. */
export function puzzle2dPlayDeleteSelectionFromFixture(fixture: Puzzle2dFixture, selectionIds: readonly string[]): Puzzle2dFixture {
	const nodeIds = selectionIds.filter((id) => fixture.nodes.some((node) => node.id === id));
	const edgeIds = new Set(selectionIds.filter((id) => fixture.edges.some((edge) => edge.id === id)));
	const handleIds = new Set(
		selectionIds.filter((id) => fixture.nodes.some((node) => node.handles.some((handle) => handle.id === id))),
	);
	let next = fixture;
	for (const nodeId of nodeIds) {
		next = puzzle2dPlayApplyNodeStructuralDeleteToFixture(next, nodeId);
	}
	const nodes = next.nodes.map((node) => ({
		...node,
		handles: node.handles.filter((handle) => !handleIds.has(handle.id)),
	}));
	const edges = next.edges.filter((edge) => !edgeIds.has(edge.id));
	return { ...next, nodes, edges };
}

/** @emoji 📋 Clones selected nodes with fresh ids and offset positions. */
export function puzzle2dPlayDuplicateSelection(
	fixture: Puzzle2dFixture,
	selectionIds: readonly string[],
): { fixture: Puzzle2dFixture; newIds: readonly string[] } {
	const nodeIds = selectionIds.filter((id) => fixture.nodes.some((node) => node.id === id));
	if (nodeIds.length === 0) {
		return { fixture, newIds: [] };
	}
	const existingIds = new Set(fixture.nodes.map((node) => node.id));
	const clones: Puzzle2dFixtureNode[] = [];
	const newIds: string[] = [];
	for (const nodeId of nodeIds) {
		const node = fixture.nodes.find((row) => row.id === nodeId);
		if (!node) {
			continue;
		}
		let newId = `${nodeId}-copy`;
		let suffix = 2;
		while (existingIds.has(newId)) {
			newId = `${nodeId}-copy-${suffix}`;
			suffix += 1;
		}
		existingIds.add(newId);
		const handles = node.handles.map((handle, index) => ({
			...handle,
			id: `${newId}.h${index}`,
		}));
		clones.push({
			...node,
			handles,
			id: newId,
			x: node.x + 32,
			y: node.y + 32,
		});
		newIds.push(newId);
	}
	return { fixture: { ...fixture, nodes: [...fixture.nodes, ...clones] }, newIds };
}

/** @emoji 🧩 Expands selection to every entity sharing a kind id with the current selection. */
export function puzzle2dPlaySelectSameKindIds(fixture: Puzzle2dFixture, selectionIds: readonly string[]): readonly string[] {
	const out = new Set<string>();
	for (const id of selectionIds) {
		const node = fixture.nodes.find((row) => row.id === id);
		if (node?.nodeKind) {
			for (const row of fixture.nodes) {
				if (row.nodeKind === node.nodeKind) {
					out.add(row.id);
				}
			}
			continue;
		}
		for (const rowNode of fixture.nodes) {
			const handle = rowNode.handles.find((row) => row.id === id);
			if (!handle?.handleKind) {
				continue;
			}
			for (const n of fixture.nodes) {
				for (const h of n.handles) {
					if (h.handleKind === handle.handleKind) {
						out.add(h.id);
					}
				}
			}
			break;
		}
		const edge = fixture.edges.find((row) => row.id === id);
		if (edge?.edgeKind) {
			for (const row of fixture.edges) {
				if (row.edgeKind === edge.edgeKind) {
					out.add(row.id);
				}
			}
		}
	}
	return [...out];
}

function puzzle2dPlayHierarchyEntityChrome(
	flags: { readonly hidden?: boolean; readonly locked?: boolean },
	graphId: string,
	options: Puzzle2dPlayHierarchyBuildOptions | undefined,
): Pick<UiTreeItemNode, "isHidden" | "actions" | "contextMenu"> {
	if (!options?.onToggleHidden && !options?.onToggleLocked) {
		return { isHidden: flags.hidden === true };
	}
	const contextMenu: UiTreeContextMenuItem[] = [];
	if (options.onToggleHidden) {
		contextMenu.push({
			id: "hidden",
			label: flags.hidden ? "Show" : "Hide",
			icon: flags.hidden ? "eye" : "eye-off",
			onSelect: () => options.onToggleHidden!(graphId),
		});
	}
	if (options.onToggleLocked) {
		contextMenu.push({
			id: "locked",
			label: flags.locked ? "Unlock" : "Lock",
			icon: flags.locked ? "lock-open" : "lock",
			onSelect: () => options.onToggleLocked!(graphId),
		});
	}
	return {
		isHidden: flags.hidden === true,
		actions: [
			...(options.onToggleHidden
				? [
						{
							id: "hidden",
							icon: flags.hidden ? "eye-off" : "eye",
							title: flags.hidden ? "Show" : "Hide",
							onClick: () => options.onToggleHidden!(graphId),
							revealOnHover: flags.hidden !== true,
						},
					]
				: []),
			...(options.onToggleLocked
				? [
						{
							id: "locked",
							icon: flags.locked ? "lock-open" : "lock",
							title: flags.locked ? "Unlock" : "Lock",
							onClick: () => options.onToggleLocked!(graphId),
							revealOnHover: flags.locked !== true,
						},
					]
				: []),
		],
		contextMenu,
	};
}

function puzzle2dPlayHierarchyHoverHandlers(
	onHover: ((payload: Puzzle2dHoverPayload) => void) | undefined,
	fixture: Puzzle2dFixture,
	graphId: string,
): Pick<UiTreeItemNode, "onPointerEnter" | "onPointerLeave"> {
	if (!onHover) {
		return {};
	}
	return {
		onPointerEnter: () => onHover(puzzle2dPlayHoverPayloadFromGraphId(fixture, graphId)),
		onPointerLeave: () => onHover(puzzle2dPlayHoverPayloadFromGraphId(fixture, null)),
	};
}

function buildPuzzle2dFixtureNodeHierarchyItem(
	fixture: Puzzle2dFixture,
	kindCatalogs: KindCatalogBundle,
	nodeId: string,
	childrenByParent: ReadonlyMap<string, readonly string[]>,
	selectedIds: ReadonlySet<string>,
	visiting: Set<string>,
	omitItemSelection: boolean,
	options?: Puzzle2dPlayHierarchyBuildOptions,
): UiTreeItemNode | null {
	if (visiting.has(nodeId)) {
		return null;
	}
	const node = fixture.nodes.find((row) => row.id === nodeId);
	if (!node) {
		return null;
	}
  visiting.add(nodeId);
  const childItems: UiTreeItemNode[] = [];
  for (const childId of childrenByParent.get(nodeId) ?? []) {
    const childItem = buildPuzzle2dFixtureNodeHierarchyItem(fixture, kindCatalogs, childId, childrenByParent, selectedIds, visiting, omitItemSelection, options);
    if (childItem) {
      childItems.push(childItem);
    }
  }
  visiting.delete(nodeId);
  const onHover = options?.onHover;
  const handleItems: UiTreeItemNode[] = node.handles.map((handle) => ({
    id: `puzzle-2d-play-hierarchy.handle.${handle.id}`,
    label: puzzle2dFixtureHandleDisplayLabel(handle, kindCatalogs),
    icon: puzzle2dPlayEntityTreeIcon("handle"),
    ...(omitItemSelection ? {} : { isSelected: selectedIds.has(handle.id) }),
    command: puzzle2dPlayCmd("hierarchySelect", { id: handle.id }),
    ...puzzle2dPlayHierarchyHoverHandlers(onHover, fixture, handle.id),
    ...puzzle2dPlayHierarchyEntityChrome(handle, handle.id, options),
  }));
  return {
    id: `puzzle-2d-play-hierarchy.node.${nodeId}`,
    label: puzzle2dFixtureNodeDisplayLabel(node, kindCatalogs),
    description: puzzle2dFixtureNodeDisplayDescription(node, kindCatalogs),
    icon: puzzle2dPlayNodeHierarchyTreeIcon(node, kindCatalogs),
    ...(omitItemSelection ? {} : { isSelected: selectedIds.has(nodeId) }),
    defaultOpen: false,
    command: puzzle2dPlayCmd("hierarchySelect", { id: nodeId }),
    ...puzzle2dPlayHierarchyHoverHandlers(onHover, fixture, nodeId),
    ...puzzle2dPlayHierarchyEntityChrome(node, nodeId, options),
    items: [...handleItems, ...childItems],
  };
}

/** @emoji 🎯 Resolves catalog kind from a hovered graph element id. */
export function puzzle2dKindHoverFromGraphId(fixture: Puzzle2dFixture, graphId: string): Puzzle2dKindHover | null {
	return puzzle2dPlayHoverPayloadFromGraphId(fixture, graphId).kind;
}

function puzzle2dPlayKindsSectionIdForDomain(domain: Puzzle2dKindHoverDomain): string {
	switch (domain) {
		case "node":
			return "puzzle-2d-play-kinds.nodes";
		case "handle":
			return "puzzle-2d-play-kinds.handles";
		case "edge":
			return "puzzle-2d-play-kinds.edges";
		case "wire":
			return "puzzle-2d-play-kinds.wires";
	}
}

function puzzle2dPlayKindsCatalogEntries(
	catalogs: KindCatalogBundle | undefined,
	domain: Puzzle2dKindHoverDomain,
): readonly Puzzle2dCatalogKind[] | undefined {
	switch (domain) {
		case "node":
			return catalogs?.nodes;
		case "handle":
			return catalogs?.handles;
		case "edge":
			return catalogs?.edges;
		case "wire":
			return catalogs?.wires;
	}
}

/** @emoji 🏷️ Resolves a catalog kind row id in the kinds tab for object↔kind hover sync. */
export function puzzle2dPlayKindsTreeRowId(catalogs: KindCatalogBundle | undefined, kind: Puzzle2dKindHover): string | null {
	const entries = puzzle2dPlayKindsCatalogEntries(catalogs, kind.domain);
	if (!entries?.length) {
		return null;
	}
	const sectionId = puzzle2dPlayKindsSectionIdForDomain(kind.domain);
	const sorted = [...entries].sort((a, b) => puzzle2dCatalogKindLabel(a).localeCompare(puzzle2dCatalogKindLabel(b)));
	const index = sorted.findIndex((entry) => entry.id === kind.kindId);
	if (index < 0) {
		return null;
	}
	return `${sectionId}.${index}.${kind.kindId}`;
}

/** @emoji 🏷️ Maps hover focus to kinds-tab row ids (kind→object and object→kind, not instance→instance). */
export function puzzle2dPlayKindsTreeHighlightedIds(
	catalogs: KindCatalogBundle | undefined,
	fixture: Puzzle2dFixture,
	graphHoverId: string | null,
	kindHover: Puzzle2dKindHover | null,
): readonly string[] {
	const kind = kindHover ?? (graphHoverId ? puzzle2dKindHoverFromGraphId(fixture, graphHoverId) : null);
	if (!kind) {
		return [];
	}
	const rowId = puzzle2dPlayKindsTreeRowId(catalogs, kind);
	return rowId ? [rowId] : [];
}

/** @emoji 🌳 Maps committed graph hover ids to workbench hierarchy tree item ids. */
export function puzzle2dPlayHierarchyTreeHighlightedIds(
	fixture: Puzzle2dFixture,
	graphHoverId: string | null,
	kindHover: Puzzle2dKindHover | null = null,
): readonly string[] {
	if (graphHoverId) {
		return puzzle2dPlayHierarchyTreeSelectedIds(fixture, [graphHoverId]);
	}
	if (kindHover) {
		return puzzle2dPlayHierarchyTreeHighlightedIdsForKind(fixture, kindHover);
	}
	return [];
}

/** @emoji 🌳 Workbench hierarchy: Nodes and Edges sections. */
export function buildPuzzle2dPlayHierarchySections(
	fixture: Puzzle2dFixture,
	selectionIds: readonly string[],
	kindCatalogs: KindCatalogBundle = puzzle2dFixtureMergedKindCatalogs(fixture),
	options?: Puzzle2dPlayHierarchyBuildOptions,
): UiTreeNode {
	const omitItemSelection = options?.omitItemSelection === true;
	const selectedIds = omitItemSelection ? new Set<string>() : new Set(selectionIds);
	const childrenByParent = puzzle2dFixtureChildrenByNodeId(fixture);
	const rootIds = puzzle2dFixtureRootNodeIds(fixture, childrenByParent);
	const visiting = new Set<string>();
	const nodeItems: UiTreeItemNode[] = [];
	for (const rootId of rootIds) {
		const item = buildPuzzle2dFixtureNodeHierarchyItem(fixture, kindCatalogs, rootId, childrenByParent, selectedIds, visiting, omitItemSelection, options);
		if (item) {
			nodeItems.push(item);
		}
	}
	const edgeItems: UiTreeItemNode[] = fixture.edges.map((edge) => ({
		id: `puzzle-2d-play-hierarchy.edge.${edge.id}`,
		label: puzzle2dFixtureEdgeDisplayLabel(edge, fixture, kindCatalogs),
		icon: puzzle2dPlayEntityTreeIcon("edge"),
		...(omitItemSelection ? {} : { isSelected: selectedIds.has(edge.id) }),
		command: puzzle2dPlayCmd("hierarchySelect", { id: edge.id }),
		...puzzle2dPlayHierarchyHoverHandlers(options?.onHover, fixture, edge.id),
		...puzzle2dPlayHierarchyEntityChrome(edge, edge.id, options),
	}));
	return {
		type: "tree",
		sections: [
			{
				id: "puzzle-2d-play-hierarchy.nodes",
				label: "Nodes",
				defaultOpen: false,
				items: nodeItems.length ? nodeItems : [{ id: "puzzle-2d-play-hierarchy.nodes.empty", label: "(none)" }],
			},
			{
				id: "puzzle-2d-play-hierarchy.edges",
				label: "Edges",
				defaultOpen: false,
				items: edgeItems.length ? edgeItems : [{ id: "puzzle-2d-play-hierarchy.edges.empty", label: "(none)" }],
			},
		],
	} as UiTreeNode;
}
//#endregion 🔖Puzzle2dPlayHierarchy

//#region 🔖Puzzle2dPlayKinds
type Puzzle2dCatalogKind = NodeKind | HandleKind | WireKind | EdgeKind;

function puzzle2dCatalogKindLabel(entry: Puzzle2dCatalogKind): string {
	const display = entry.name?.trim();
	return display && display.length > 0 ? display : entry.id;
}

function puzzle2dCatalogHandleKindLabel(handleKindId: string, handleKinds: readonly HandleKind[] | undefined): string {
	const entry = handleKinds?.find((row) => row.id === handleKindId);
	return entry ? puzzle2dCatalogKindLabel(entry) : handleKindId;
}

function puzzle2dNodeKindHandleTemplateCatalogDescription(template: NodeKindHandleTemplate): string {
	const radius = template.radius ?? 3;
	return `θ ${template.angle.toFixed(2)} · r ${radius.toFixed(1)}`;
}

function puzzle2dPlayNodeKindHandleCatalogItems(
	sectionId: string,
	nodeIndex: number,
	nodeKindId: string,
	templates: readonly NodeKindHandleTemplate[],
	handleKinds: readonly HandleKind[] | undefined,
): readonly UiTreeItemNode[] {
	return templates.map((template, handleIndex) => ({
		id: `${sectionId}.${nodeIndex}.${nodeKindId}.handle.${handleIndex}`,
		label: puzzle2dCatalogHandleKindLabel(template.handleKind, handleKinds),
		description: puzzle2dNodeKindHandleTemplateCatalogDescription(template),
		icon: puzzle2dPlayEntityTreeIcon("handle"),
	}));
}

function puzzle2dPlayKindCatalogSection(
	sectionId: string,
	label: string,
	entries: readonly Puzzle2dCatalogKind[] | undefined,
	handleKinds?: readonly HandleKind[],
	sectionDefaultOpen = true,
	kindCatalogs?: KindCatalogBundle,
	onHover?: (payload: Puzzle2dHoverPayload) => void,
): UiTreeSectionNode | null {
	if (!entries?.length) {
		return null;
	}
	const isNodePalette = sectionId === "puzzle-2d-play-kinds.nodes";
	const sectionDomain = puzzle2dPlayKindsSectionDomain(sectionId);
	const sectionTreeIcon = puzzle2dPlayKindSectionTreeIcon(sectionId);
	const items: UiTreeItemNode[] = [...entries]
		.sort((a, b) => puzzle2dCatalogKindLabel(a).localeCompare(puzzle2dCatalogKindLabel(b)))
		.map((entry, index) => {
			const nodeKind = isNodePalette ? (entry as NodeKind) : null;
			const handleItems = nodeKind?.handles?.length
				? puzzle2dPlayNodeKindHandleCatalogItems(sectionId, index, entry.id, nodeKind.handles, handleKinds)
				: [];
			const kindHover = sectionDomain ? { domain: sectionDomain, kindId: entry.id } satisfies Puzzle2dKindHover : null;
			const nodeKindIcon = isNodePalette ? (puzzle2dNodeKindCatalogIcon(entry as NodeKind) ?? sectionTreeIcon) : sectionTreeIcon;
			return {
				id: `${sectionId}.${index}.${entry.id}`,
				label: puzzle2dCatalogKindLabel(entry),
				description: entry.id,
				icon: nodeKindIcon,
				defaultOpen: handleItems.length === 0,
				...(handleItems.length ? { items: handleItems } : {}),
				...(kindHover ? puzzle2dPlayKindRowHoverHandlers(onHover, kindHover) : {}),
				...(isNodePalette && nodeKind?.handles?.length
					? {
							draggable: true,
							dragData: puzzle2dPlayNodeKindDragData(entry.id, kindCatalogs),
						}
					: {}),
			};
		});
	return { id: sectionId, label, defaultOpen: sectionDefaultOpen, items };
}

/** @emoji 🏷️ Workbench kinds tab: Nodes, Handles, Wires, Edges. */
export function buildPuzzle2dPlayKindsTree(
	catalogs: KindCatalogBundle | undefined,
	options?: { readonly onHover?: (payload: Puzzle2dHoverPayload) => void; readonly highlightedIds?: readonly string[] },
): UiNode {
	const onHover = options?.onHover;
	const sections = [
		puzzle2dPlayKindCatalogSection("puzzle-2d-play-kinds.nodes", "Nodes", catalogs?.nodes, catalogs?.handles, true, catalogs, onHover),
		puzzle2dPlayKindCatalogSection("puzzle-2d-play-kinds.handles", "Handles", catalogs?.handles, undefined, false, undefined, onHover),
		puzzle2dPlayKindCatalogSection("puzzle-2d-play-kinds.wires", "Wires", catalogs?.wires, undefined, true, undefined, onHover),
		puzzle2dPlayKindCatalogSection("puzzle-2d-play-kinds.edges", "Edges", catalogs?.edges, undefined, true, undefined, onHover),
	].filter((section): section is UiTreeSectionNode => section !== null);
	if (!sections.length) {
		return {
			type: "tree",
			sections: [
				{
					id: "puzzle-2d-play-kinds.empty",
					label: "Kinds",
					defaultOpen: false,
					items: [{ id: "puzzle-2d-play-kinds.empty.msg", label: "No kind catalogs in this fixture" }],
				},
			],
		};
	}
	return {
		type: "tree",
		sections,
		...(options?.highlightedIds?.length ? { highlightedIds: options.highlightedIds } : {}),
	};
}
//#endregion 🔖Puzzle2dPlayKinds

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
	readonly puzzle2dActiveTool: Puzzle2dActiveTool;
	readonly puzzle2dSuggestionOffset: number;
	readonly puzzle2dSelectionMethod: Puzzle2dSelectionMethod;
	readonly puzzle2dSelectionMode: Puzzle2dSelectionMode;
	readonly puzzle2dSelectionTargets: Puzzle2dSelectionTargets;
	readonly puzzle2dGridSnapEnabled: boolean;
	readonly puzzle2dRedrawPlaying: boolean;
}

/** @emoji 🔗 Host bridge: toolbar snapshot + commands that need React/fixture context. */
export interface Puzzle2dPlayHostBridge {
	getToolbarState(): Puzzle2dPlayToolbarState;
	getFixtureJson(): string;
	runHostCommand(command: string, args?: unknown): void;
}

/** @emoji 🧰 Playground {@link AppTools} for puzzle 2d play (selection, filter, view, create, actions). */
export function buildPuzzle2dPlayToolbarTools(state: Puzzle2dPlayToolbarState, controllerId: string): AppTools {
	const targetRecord: Record<Puzzle2dPlayTargetKind, boolean> = {
		nodes: state.puzzle2dSelectionTargets.nodes,
		edges: state.puzzle2dSelectionTargets.edges,
		handles: state.puzzle2dSelectionTargets.handles,
	};
	const methodTools: ToolLeaf[] = [
		{
			id: "puzzle2d.select.rectangle",
			kind: "toggle",
			iconId: "square",
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
			iconId: "lasso",
			text: "Lasso",
			order: 1,
			pressed: state.puzzle2dSelectionMethod === "lasso",
			controllerId,
			command: "setSelectionMethod",
			args: { method: "lasso" },
		},
	];
	const modeTools: ToolLeaf[] = [
		{
			id: "puzzle2d.select.mode.default",
			kind: "toggle",
			iconId: "mouse-pointer-2",
			text: "Default",
			order: 0,
			pressed: state.puzzle2dSelectionMode === "default",
			controllerId,
			command: "setSelectionMode",
			args: { mode: "default" },
		},
		{
			id: "puzzle2d.select.mode.additive",
			kind: "toggle",
			iconId: "plus",
			text: "Add",
			order: 1,
			pressed: state.puzzle2dSelectionMode === "additive",
			controllerId,
			command: "setSelectionMode",
			args: { mode: "additive" },
		},
		{
			id: "puzzle2d.select.mode.subtractive",
			kind: "toggle",
			iconId: "minus",
			text: "Subtract",
			order: 2,
			pressed: state.puzzle2dSelectionMode === "subtractive",
			controllerId,
			command: "setSelectionMode",
			args: { mode: "subtractive" },
		},
		{
			id: "puzzle2d.select.mode.invertive",
			kind: "toggle",
			iconId: "arrow-right-left",
			text: "Invert",
			order: 3,
			pressed: state.puzzle2dSelectionMode === "invertive",
			controllerId,
			command: "setSelectionMode",
			args: { mode: "invertive" },
		},
	];
	return [
		toolCollection("selection", "mouse-pointer-2", [
			toolCollection("methods", "square", methodTools, 0),
			toolCollection("mode", "mouse-pointer-2", modeTools, 1),
			toolCollection("targets", "layers", buildPlaygroundKindToggleTools("selection", PUZZLE_2D_PLAY_TARGET_KINDS, puzzle2dPlayTargetLabel, targetRecord, controllerId, "toggleSelectionTarget"), 2),
			{
				id: "puzzle2d.selection.clear",
				kind: "button",
				iconId: "x",
				label: "Clear",
				order: 20,
				controllerId,
				command: "clearSelection",
			},
		]),
		toolCollection("view", "layout-grid", [
			{
				id: "puzzle2d.grid.snap",
				kind: "toggle",
				iconId: "layout-grid",
				text: "Grid snap",
				order: 0,
				pressed: state.puzzle2dGridSnapEnabled,
				controllerId,
				command: "toggleGridSnap",
			},
		]),
		toolCollection("create", "plus", [{ id: "puzzle2d.create.circle", kind: "button", iconId: "circle", label: "Circle", order: 0, controllerId, command: "appendCircle" }]),
		toolCollection("actions", "more-horizontal", [
			{
				id: "puzzle2d.redraw.play",
				kind: "toggle",
				iconId: "play",
				text: "Redraw",
				order: 0,
				pressed: state.puzzle2dRedrawPlaying,
				controllerId,
				command: "toggleRedrawPlaying",
			},
			{ id: "puzzle2d.redraw.handles", kind: "button", iconId: "grip-vertical", label: "Handles", title: "Redraw handles once", order: 1, controllerId, command: "redrawHandlesOnce" },
		]),
	];
}


/** @emoji 🎛 Puzzle 2d play shell controller: per-pane LOD modes + playground toolbar tools. */
export class Puzzle2dPlayShellController extends Controller {
	readonly mainMode = new ModeRuntime("main", "Edit", undefined);
	private lodModeByPane: Record<Puzzle2dPlayPaneId, Puzzle2dLodModeKind>;
	private lodModeByInstance: Record<string, Puzzle2dLodModeKind>;
	private effectiveLodByPane: Record<Puzzle2dPlayPaneId, Puzzle2dDrawLodKind>;
	private engagementInputByPane: Record<Puzzle2dPlayPaneId, string>;
	private lastEngagementRepeatByPane: Record<Puzzle2dPlayPaneId, string>;
	private hostBridge: Puzzle2dPlayHostBridge | null = null;
	private readonly hostChromeNotify: () => void;
	private activeTool: Puzzle2dActiveTool = "select";
	private fillCount = 0;
	private suggestionOffset = DEFAULT_PUZZLE_2D_SUGGESTION_OFFSET_PX;
	private brushEngagementPossibles: { readonly id: string; readonly label: string }[] = [];
	private nodeKindIds: string[] = [];
	private handleKindIds: string[] = [];
	private nodeKindWeights: KindWeightMap = {};
	private handleKindWeights: KindWeightMap = {};
	private kindCatalogs: KindCatalogBundle = DEFAULT_KIND_CATALOG_BUNDLE;
	private readonly wireBridge = new WireHoverBridge();
	private readonly snapshotListeners = new Set<() => void>();

	constructor(commandBus: CommandBus, hostNotify: () => void, hostChromeNotify: () => void) {
		super(PUZZLE_2D_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.hostChromeNotify = hostChromeNotify;
		this.wireBridge.bindPointerFocus(this.pointerFocus);
		this.lodModeByPane = {
			"2d-detail": "detail",
			"2d-overview": PUZZLE_2D_LOD_MODE_AUTOMATIC,
			"2d-selection": PUZZLE_2D_LOD_MODE_AUTOMATIC,
		};
		this.lodModeByInstance = {};
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
		this.lastEngagementRepeatByPane = {
			"2d-detail": "",
			"2d-overview": "",
			"2d-selection": "",
		};
		subscribePuzzle2dFillSessionReady(() => {
			this.emit();
			this.rebuildShellMode();
		});
		this.rebuildShellMode();
	}

	private windowEngagementForPane(pane: Puzzle2dPlayPaneId): WindowEngagement {
		const staticToolPossibles = [
			{ id: PUZZLE_2D_ENGAGEMENT_TOOL_BRUSH_ID, label: "Brush", command: puzzle2dPlayCmd("engagementPossibleSelect", { pane, possibleId: PUZZLE_2D_ENGAGEMENT_TOOL_BRUSH_ID }) },
			{ id: PUZZLE_2D_ENGAGEMENT_TOOL_FILL_ID, label: "Fill", command: puzzle2dPlayCmd("engagementPossibleSelect", { pane, possibleId: PUZZLE_2D_ENGAGEMENT_TOOL_FILL_ID }) },
			{ id: PUZZLE_2D_ENGAGEMENT_TOOL_SELECT_ID, label: "Select", command: puzzle2dPlayCmd("engagementPossibleSelect", { pane, possibleId: PUZZLE_2D_ENGAGEMENT_TOOL_SELECT_ID }) },
			{ id: "puzzle2d.select.rectangle", label: "Rectangle", command: puzzle2dPlayCmd("engagementPossibleSelect", { pane, possibleId: "puzzle2d.select.rectangle" }) },
			{ id: "puzzle2d.select.lasso", label: "Lasso", command: puzzle2dPlayCmd("engagementPossibleSelect", { pane, possibleId: "puzzle2d.select.lasso" }) },
			{ id: "puzzle2d.create.circle", label: "Circle", command: puzzle2dPlayCmd("engagementPossibleSelect", { pane, possibleId: "puzzle2d.create.circle" }) },
			{ id: "puzzle2d.selection.clear", label: "Clear", command: puzzle2dPlayCmd("engagementPossibleSelect", { pane, possibleId: "puzzle2d.selection.clear" }) },
		];
		const toolPossibles =
			this.activeTool === "brush" && this.brushEngagementPossibles.length > 0
				? this.brushEngagementPossibles.map((row) => ({
						id: row.id,
						label: row.label,
						command: puzzle2dPlayCmd("engagementPossibleSelect", { pane, possibleId: row.id }),
					}))
				: staticToolPossibles;
		const sessionActive = this.activeTool === "brush" || this.activeTool === "fill";
		const fillProgress = puzzle2dFillBuildProgressRef.current;
		const fillSliderMax = fillProgress.done
			? PUZZLE_2D_FILL_COUNT_SLIDER_MAX
			: Math.max(fillProgress.count, this.fillCount > 0 ? this.fillCount : 0, 1);
		const fillLabel =
			fillProgress.done || fillProgress.count === 0
				? `Fill ${this.fillCount}`
				: `Fill ${this.fillCount} (building ${fillProgress.count}/${fillProgress.maxCount})`;
		const control =
			this.activeTool === "fill"
				? {
						kind: "slider" as const,
						id: "puzzle2d-fill-count",
						label: fillLabel,
						value: this.fillCount,
						min: PUZZLE_2D_FILL_COUNT_SLIDER_MIN,
						max: fillSliderMax,
						step: PUZZLE_2D_FILL_COUNT_SLIDER_STEP,
						onChange: puzzle2dPlayCmd("engagementControlChange", { pane }),
					}
				: this.activeTool === "brush" && this.brushEngagementPossibles.length > 0
					? this.brushPlacementEngagementControl(pane)
					: undefined;
		return {
			sessionActive,
			options: [
				{ id: PUZZLE_2D_ENGAGEMENT_TOOL_SELECT_ID, label: "Select", pressed: this.activeTool === "select", command: puzzle2dPlayCmd("engagementPossibleSelect", { pane, possibleId: PUZZLE_2D_ENGAGEMENT_TOOL_SELECT_ID }) },
				{ id: PUZZLE_2D_ENGAGEMENT_TOOL_BRUSH_ID, label: "Brush", pressed: this.activeTool === "brush", command: puzzle2dPlayCmd("engagementPossibleSelect", { pane, possibleId: PUZZLE_2D_ENGAGEMENT_TOOL_BRUSH_ID }) },
				{ id: PUZZLE_2D_ENGAGEMENT_TOOL_FILL_ID, label: "Fill", pressed: this.activeTool === "fill", command: puzzle2dPlayCmd("engagementPossibleSelect", { pane, possibleId: PUZZLE_2D_ENGAGEMENT_TOOL_FILL_ID }) },
			],
			input: {
				id: "engagement-input",
				value: this.engagementInputByPane[pane],
				placeholder: this.activeTool === "fill" ? "Fill" : this.activeTool === "brush" ? "Brush" : "Command",
				onChange: puzzle2dPlayCmd("engagementInput", { pane }),
				onSubmit: puzzle2dPlayCmd("engagementSubmit", { pane }),
				onRepeatLast: puzzle2dPlayCmd("engagementRepeatLast", { pane }),
				onAbort: puzzle2dPlayCmd("engagementAbort", { pane }),
			},
			control,
			possibleEngagements: toolPossibles,
		};
	}

	private brushPlacementEngagementControl(pane: Puzzle2dPlayPaneId): WindowEngagementControl {
		const candidates = this.brushEngagementPossibles;
		const selectedValue = candidates[0]!.id;
		const selectCmd = puzzle2dPlayCmd("engagementControlSelect", { pane });
		if (candidates.length <= 6) {
			return {
				kind: "toggleGroup",
				id: "puzzle2d-brush-placement",
				label: "Placement",
				value: selectedValue,
				options: candidates.map((row) => ({ id: row.id, label: row.label })),
				onSelect: selectCmd,
			};
		}
		return {
			kind: "select",
			id: "puzzle2d-brush-placement",
			label: "Placement",
			value: selectedValue,
			placeholder: "Placement",
			items: candidates.map((row) => ({ id: row.id, value: row.id, label: row.label })),
			onChange: selectCmd,
		};
	}

	private kindWeightLabel(kindId: string, catalogSlice: "nodes" | "handles"): string {
		const label =
			catalogSlice === "nodes"
				? puzzle2dNodeKindOverlayLabel(kindId, this.kindCatalogs)
				: puzzle2dHandleKindOverlayLabel(kindId, this.kindCatalogs);
		return label.length > 24 ? `${label.slice(0, 21)}…` : label;
	}

	private kindWeightMeasures(
		prefix: string,
		ids: readonly string[],
		weights: KindWeightMap,
		command: string,
		catalogSlice: "nodes" | "handles",
	): readonly WindowMeasure[] {
		return ids.map((kindId) => {
			const w = weights[kindId] ?? 0;
			return {
				kind: "slider" as const,
				id: `${PUZZLE_2D_PLAY_CONTROLLER_ID}-${prefix}-${kindId}`,
				label: `${this.kindWeightLabel(kindId, catalogSlice)} ${(w * 100).toFixed(0)}%`,
				value: w,
				min: 0,
				max: 1,
				step: 0.01,
				onChange: puzzle2dPlayCmd(command, { kindId }),
			};
		});
	}

	private suggestionMeasuresGroup(): WindowMeasure {
		return {
			kind: "group",
			id: `${PUZZLE_2D_PLAY_CONTROLLER_ID}-suggestion`,
			label: "Suggestion",
			children: [
				{
					kind: "slider",
					id: `${PUZZLE_2D_PLAY_CONTROLLER_ID}-suggestion-offset`,
					label: "Offset",
					value: this.suggestionOffset,
					min: PUZZLE_2D_SUGGESTION_OFFSET_SLIDER_MIN,
					max: PUZZLE_2D_SUGGESTION_OFFSET_SLIDER_MAX,
					step: PUZZLE_2D_SUGGESTION_OFFSET_SLIDER_STEP,
					onChange: puzzle2dPlayCmd("setSuggestionOffset"),
				},
				{
					kind: "group",
					id: `${PUZZLE_2D_PLAY_CONTROLLER_ID}-suggestion-distribution`,
					label: "Distribution",
					defaultOpen: false,
					children: [
						{
							kind: "group",
							id: `${PUZZLE_2D_PLAY_CONTROLLER_ID}-suggestion-distribution-nodes`,
							label: "Nodes",
							defaultOpen: false,
							children: this.kindWeightMeasures("node-kind", this.nodeKindIds, this.nodeKindWeights, "setNodeKindWeight", "nodes"),
						},
						{
							kind: "group",
							id: `${PUZZLE_2D_PLAY_CONTROLLER_ID}-suggestion-distribution-handles`,
							label: "Handles",
							defaultOpen: false,
							children: this.kindWeightMeasures("handle-kind", this.handleKindIds, this.handleKindWeights, "setHandleKindWeight", "handles"),
						},
					],
				},
			],
		};
	}

	private pushBrushKindWeightsToHost(): void {
		this.hostBridge?.runHostCommand("setBrushKindWeights", {
			nodeWeights: this.nodeKindWeights,
			handleWeights: this.handleKindWeights,
		});
	}

	/** @emoji 🎚️ Syncs kind catalogs for suggestion-percentage sliders (uniform weights for new ids). */
	setKindCatalogs(catalogs: KindCatalogBundle | undefined): void {
		this.kindCatalogs = puzzle2dMergeKindCatalogBundle(catalogs);
		const nodes = catalogs?.nodes?.map((row) => row.id).filter((id): id is string => Boolean(id)) ?? [];
		const handles = catalogs?.handles?.map((row) => row.id).filter((id): id is string => Boolean(id)) ?? [];
		const nodeChanged =
			nodes.length !== this.nodeKindIds.length || nodes.some((id, i) => id !== this.nodeKindIds[i]);
		const handleChanged =
			handles.length !== this.handleKindIds.length || handles.some((id, i) => id !== this.handleKindIds[i]);
		if (!nodeChanged && !handleChanged) {
			return;
		}
		this.nodeKindIds = [...nodes];
		this.handleKindIds = [...handles];
		this.nodeKindWeights = syncKindWeightMap(this.nodeKindIds, this.nodeKindWeights);
		this.handleKindWeights = syncKindWeightMap(this.handleKindIds, this.handleKindWeights);
		this.rebuildShellMode();
		this.pushBrushKindWeightsToHost();
		this.emit();
	}

	/** @emoji 🖌️ Mirrors brush candidate rows into window engagement possibles. */
	setBrushEngagementPossibles(rows: readonly { readonly id: string; readonly label: string }[]): void {
		const next = rows.map((row) => ({ id: row.id, label: row.label }));
		if (next.length === this.brushEngagementPossibles.length && next.every((row, i) => row.id === this.brushEngagementPossibles[i]?.id)) {
			return;
		}
		this.brushEngagementPossibles = next;
		this.rebuildShellMode();
		this.emit();
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

	/** @emoji 🖌️ Syncs shell + React host when the viewport tool changes (engagement or bridge command). */
	private setPlayActiveTool(tool: Puzzle2dActiveTool): void {
		const prev = this.activeTool;
		this.activeTool = tool;
		if (tool === "select") {
			this.brushEngagementPossibles = [];
			this.fillCount = 0;
		}
		if (tool === "fill" && prev !== "fill") {
			this.fillCount = 0;
		}
		this.hostBridge?.runHostCommand("setActiveTool", { tool, prevTool: prev });
		for (const pane of ["2d-overview", "2d-detail", "2d-selection"] as const) {
			this.syncWindowEngagementForPane(pane);
		}
	}

	/** @emoji 🖌️ Active viewport tool owned by the play shell (engagement chrome reads this). */
	getActiveTool(): Puzzle2dActiveTool {
		return this.activeTool;
	}

	private applyEngagementCommand(pane: Puzzle2dPlayPaneId, possibleIdOrText: string): boolean {
		const token = puzzle2dPlayEngagementCommandToken(possibleIdOrText);
		const runHost = (command: string, args?: unknown) => {
			this.hostBridge?.runHostCommand(command, args);
		};
		const remember = (key: string): true => {
			this.lastEngagementRepeatByPane = { ...this.lastEngagementRepeatByPane, [pane]: key };
			return true;
		};
		if (possibleIdOrText === "puzzle2d.select.rectangle" || token === "rectangle") {
			runHost("setSelectionMethod", { method: "rectangle" });
			return remember(possibleIdOrText === "puzzle2d.select.rectangle" ? possibleIdOrText : "puzzle2d.select.rectangle");
		}
		if (possibleIdOrText === "puzzle2d.select.lasso" || token === "lasso") {
			runHost("setSelectionMethod", { method: "lasso" });
			return remember(possibleIdOrText === "puzzle2d.select.lasso" ? possibleIdOrText : "puzzle2d.select.lasso");
		}
		if (possibleIdOrText === "puzzle2d.selection.clear" || token === "clear") {
			runHost("clearSelection", {});
			return remember("puzzle2d.selection.clear");
		}
		if (possibleIdOrText === "puzzle2d.create.circle" || token === "circle") {
			runHost("appendCircle", {});
			return remember("puzzle2d.create.circle");
		}
		if (possibleIdOrText === PUZZLE_2D_ENGAGEMENT_TOOL_BRUSH_ID || token === "brush") {
			this.setPlayActiveTool("brush");
			return remember(PUZZLE_2D_ENGAGEMENT_TOOL_BRUSH_ID);
		}
		if (possibleIdOrText === PUZZLE_2D_ENGAGEMENT_TOOL_FILL_ID || token === "fill") {
			this.setPlayActiveTool("fill");
			return remember(PUZZLE_2D_ENGAGEMENT_TOOL_FILL_ID);
		}
		if (possibleIdOrText === PUZZLE_2D_ENGAGEMENT_TOOL_SELECT_ID || token === "select") {
			this.setPlayActiveTool("select");
			return remember(PUZZLE_2D_ENGAGEMENT_TOOL_SELECT_ID);
		}
		if (possibleIdOrText.startsWith("puzzle2d.brush.")) {
			const match = possibleIdOrText.match(/^puzzle2d\.brush\.(.+)\.(\d+)$/);
			if (match) {
				const index = Number(match[2]);
				if (Number.isFinite(index)) {
					runHost("pickBrushCandidate", { index });
				}
			}
			return remember(possibleIdOrText);
		}
		void pane;
		return false;
	}

	/** @emoji ⎋ Ends the active engagement session for a pane (Escape / engagement abort). */
	private abortEngagementForPane(pane: Puzzle2dPlayPaneId): void {
		this.engagementInputByPane = { ...this.engagementInputByPane, [pane]: "" };
		if (this.activeTool === "brush" || this.activeTool === "fill") {
			this.setPlayActiveTool("select");
		}
		this.syncWindowEngagementForPane(pane);
	}

	private repeatLastEngagementForPane(pane: Puzzle2dPlayPaneId): void {
		const last = this.lastEngagementRepeatByPane[pane];
		if (!last) return;
		if (this.applyEngagementCommand(pane, last)) {
			this.engagementInputByPane = { ...this.engagementInputByPane, [pane]: "" };
			this.syncWindowEngagementForPane(pane);
		}
	}

	/** @emoji 🔗 Attaches the React host bridge used for toolbar commands and snapshots. */
	setHostBridge(bridge: Puzzle2dPlayHostBridge | null): void {
		this.hostBridge = bridge;
		this.syncWireText();
		this.rebuildToolbarTools();
	}

	private syncWireText(): void {
		this.wireBridge.setWireText(this.getCompiledWireLiteral());
	}

	/** @emoji 🔄 Refreshes wire DSL text from the live React host fixture. */
	notifyFixtureRevision(): void {
		this.syncWireText();
		this.notifySnapshot();
	}

	subscribeSnapshot(listener: () => void): () => void {
		this.snapshotListeners.add(listener);
		const unsubWire = this.wireBridge.subscribe(listener);
		return () => {
			this.snapshotListeners.delete(listener);
			unsubWire();
		};
	}

	getCompiledWireLiteral(): string {
		const json = this.hostBridge?.getFixtureJson();
		return puzzle2dFixtureToCompiledDagWireLiteral(json ?? puzzle2dFixtureToJson(PUZZLE_2D_PLAY_DEFAULT_FIXTURE));
	}

	getWriterDocumentCompiledDag(): WriterDocument {
		return createWriterDocument({ id: "puzzle-2d-compiled-dag", languageId: "wire", text: this.getCompiledWireLiteral() });
	}

	getWireHoverOccurrences(): readonly { readonly start: number; readonly end: number }[] {
		return this.wireBridge.getWireHoverOccurrences();
	}

	getWireSelectOccurrences(): readonly { readonly start: number; readonly end: number }[] {
		return this.wireBridge.getWireSelectOccurrences();
	}

	getHoverEpoch(): number {
		return this.wireBridge.getHoverEpoch();
	}

	getSelectEpoch(): number {
		return this.wireBridge.getSelectEpoch();
	}

	getGraphHighlightedNodeIds(): readonly string[] {
		return this.wireBridge.getGraphHoveredNodeIds();
	}

	private notifySnapshot(): void {
		for (const listener of this.snapshotListeners) {
			listener();
		}
	}

	/** @emoji 🔄 Rebuilds {@link ModeRuntime.tools} from the latest host toolbar snapshot. */
	private toolbarState(): Puzzle2dPlayToolbarState {
		return (
			this.hostBridge?.getToolbarState() ?? {
				puzzle2dActiveTool: this.activeTool,
				puzzle2dSuggestionOffset: this.suggestionOffset,
				puzzle2dGridSnapEnabled: false,
				puzzle2dRedrawPlaying: false,
				puzzle2dSelectionMethod: "rectangle",
				puzzle2dSelectionMode: "default",
				puzzle2dSelectionTargets: { nodes: true, edges: true, handles: true },
			}
		);
	}

	rebuildToolbarTools(): void {
		this.mainMode.tools = buildPuzzle2dPlayToolbarTools(this.toolbarState(), this.id);
	}

	private windowMeasuresForPane(paneId: Puzzle2dPlayPaneId): readonly WindowMeasure[] {
		return [this.lodMeasureForPane(paneId), this.suggestionMeasuresGroup()];
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
		this.mainMode.windowKinds = [
			...PUZZLE_2D_PLAY_WINDOW_SPECS.map(
				(row) =>
					new WindowKindRuntime(
						row.pane,
						row.label,
						row.bodyKey,
						undefined,
						this.windowMeasuresForPane(row.pane),
						this.windowEngagementForPane(row.pane),
						PUZZLE_2D_PANE_TEMPLATES[row.pane],
					),
			),
			new WindowKindRuntime(PUZZLE_2D_PLAY_WINDOW_KIND_COMPILED_DAG, "DSL", PUZZLE_2D_PLAY_BODY_KEY_COMPILED_DAG),
		];
		for (const windowKind of this.mainMode.windowKinds) {
			enforcePlaygroundWindowEngagementInput(windowKind.engagement, `Puzzle 2D play window "${windowKind.id}"`);
		}
		this.rebuildToolbarTools();
	}

	override run(command: string, args?: unknown): void {
		if (command === "setWireHover") {
			this.wireBridge.setWireHover((args as { offset?: number | null }).offset ?? null);
			this.notifySnapshot();
			this.emit();
			return;
		}
		if (command === "setWireSelect") {
			this.wireBridge.setWireSelect((args as { start: number; end: number } | null) ?? null);
			this.notifySnapshot();
			this.emit();
			return;
		}
		if (command === "setGraphHover") {
			this.wireBridge.setGraphHover((args as { id?: string | null }).id ?? null);
			this.notifySnapshot();
			this.emit();
			return;
		}
		if (command === "setGraphSelect") {
			const ids = (args as { ids?: readonly string[] }).ids ?? [];
			this.wireBridge.setGraphSelect(ids);
			this.notifySnapshot();
			this.emit();
			return;
		}
		if (command === "notifyFixtureRevision") {
			this.notifyFixtureRevision();
			this.emit();
			return;
		}
		switch (command) {
			case "setLodModeForPane": {
				const { pane, paneId, value, instanceId } = args as {
					pane?: Puzzle2dPlayPaneId;
					paneId?: Puzzle2dPlayPaneId;
					value?: string;
					instanceId?: string;
				};
				const resolvedPane = pane ?? paneId;
				const scopeId = instanceId ?? resolvedPane;
				if (!scopeId || typeof value !== "string") break;
				if (value !== PUZZLE_2D_LOD_MODE_AUTOMATIC && !isPuzzle2dDrawLodKind(value)) break;
				const nextMode = value as Puzzle2dLodModeKind;
				this.lodModeByInstance = { ...this.lodModeByInstance, [scopeId]: nextMode };
				if (resolvedPane === "2d-overview" || resolvedPane === "2d-detail" || resolvedPane === "2d-selection") {
					if (scopeId === resolvedPane) {
						this.lodModeByPane = { ...this.lodModeByPane, [resolvedPane]: nextMode };
					}
				}
				this.rebuildShellMode();
				this.emit();
				this.hostChromeNotify();
				return;
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
			case "selectAllSelection":
			case "deleteSelection":
			case "setSelectionFlag":
			case "duplicateSelection":
			case "selectSameKind":
			case "toggleEntityFlag": {
				this.hostBridge?.runHostCommand(command, args);
				break;
			}
			case "hierarchySelect": {
				const id = (args as { id?: string }).id;
				if (typeof id === "string") {
					this.hostBridge?.runHostCommand("hierarchySelect", { id });
				}
				break;
			}
			case "toggleGridSnap":
			case "appendCircle":
			case "toggleRedrawPlaying":
			case "redrawHandlesOnce":
			case "addBrushNode":
			case "pickBrushCandidate": {
				this.hostBridge?.runHostCommand(command, args);
				break;
			}
			case "setActiveTool": {
				const tool = (args as { tool?: Puzzle2dActiveTool }).tool;
				if (tool === "select" || tool === "brush" || tool === "fill") {
					this.setPlayActiveTool(tool);
				}
				break;
			}
			case "setSuggestionOffset": {
				const distance = Number((args as { value?: number }).value);
				if (Number.isFinite(distance)) {
					this.suggestionOffset = Math.max(PUZZLE_2D_SUGGESTION_OFFSET_SLIDER_MIN, Math.min(PUZZLE_2D_SUGGESTION_OFFSET_SLIDER_MAX, distance));
					this.hostBridge?.runHostCommand("setSuggestionOffset", { distance: this.suggestionOffset });
				}
				break;
			}
			case "setNodeKindWeight": {
				const { kindId, value } = args as { kindId?: string; value?: number };
				if (typeof kindId !== "string" || !this.nodeKindIds.includes(kindId)) {
					break;
				}
				const next = Number(value);
				if (!Number.isFinite(next)) {
					break;
				}
				this.nodeKindWeights = normalizeKindWeightGroup(this.nodeKindWeights, kindId, next);
				this.pushBrushKindWeightsToHost();
				break;
			}
			case "setHandleKindWeight": {
				const { kindId, value } = args as { kindId?: string; value?: number };
				if (typeof kindId !== "string" || !this.handleKindIds.includes(kindId)) {
					break;
				}
				const next = Number(value);
				if (!Number.isFinite(next)) {
					break;
				}
				this.handleKindWeights = normalizeKindWeightGroup(this.handleKindWeights, kindId, next);
				this.pushBrushKindWeightsToHost();
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
			case "engagementRepeatLast": {
				const { pane } = args as { pane?: Puzzle2dPlayPaneId };
				if (pane !== "2d-overview" && pane !== "2d-detail" && pane !== "2d-selection") {
					break;
				}
				this.repeatLastEngagementForPane(pane);
				break;
			}
			case "engagementAbort": {
				const { pane } = args as { pane?: Puzzle2dPlayPaneId };
				if (pane !== "2d-overview" && pane !== "2d-detail" && pane !== "2d-selection") {
					break;
				}
				this.abortEngagementForPane(pane);
				break;
			}
			case "engagementControlSelect": {
				const { pane, id, value } = args as { pane?: Puzzle2dPlayPaneId; id?: string; value?: string };
				const selectedId = id ?? value;
				if (pane !== "2d-overview" && pane !== "2d-detail" && pane !== "2d-selection") {
					break;
				}
				if (this.applyEngagementCommand(pane, selectedId ?? "")) {
					this.engagementInputByPane = { ...this.engagementInputByPane, [pane]: "" };
					this.syncWindowEngagementForPane(pane);
				} else {
					this.hostBridge?.runHostCommand("engagementPossibleSelect", { pane, possibleId: selectedId });
				}
				break;
			}
			case "engagementControlChange": {
				const { pane, value } = args as { pane?: Puzzle2dPlayPaneId; value?: number };
				if (pane !== "2d-overview" && pane !== "2d-detail" && pane !== "2d-selection") {
					break;
				}
				if (this.activeTool !== "fill") {
					break;
				}
				const available = puzzle2dFillSessionRef.current.sequence.length;
				const fillProgress = puzzle2dFillBuildProgressRef.current;
				const maxAllowed = fillProgress.done ? PUZZLE_2D_FILL_COUNT_SLIDER_MAX : Math.max(available, 1);
				const count = Math.round(Math.max(PUZZLE_2D_FILL_COUNT_SLIDER_MIN, Math.min(maxAllowed, Number(value) || 0)));
				if (count === this.fillCount) {
					break;
				}
				this.fillCount = count;
				this.hostBridge?.runHostCommand("setFillCount", { count });
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
			case "patchInspectorNodes":
			case "patchInspectorHandles":
			case "patchInspectorEdges":
			case "setPuzzle2dRedrawMode":
			case "setPuzzle2dRedrawHandlesAfterNodes":
			case "setPuzzle2dRedrawProgressiveEnabled":
			case "setPuzzle2dRedrawProgressiveAutoStopMs":
			case "setPuzzle2dRedrawPlayMaxItersPerFrame":
			case "setForceLayoutFullIterations":
			case "setForceLayoutIdealEdgeLength":
			case "setForceLayoutRepulsionStrength":
			case "setForceLayoutGravity":
			case "setTreeLayoutLayerSpacing":
			case "setTreeLayoutSiblingGap":
			case "setTreeLayoutDirection":
			case "applyPuzzle2dRedrawOnce":
			case "applyPuzzle2dRedrawHandlesOnce": {
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

	lodModeForScope(scopeId: string, pane: Puzzle2dPlayPaneId): Puzzle2dLodModeKind {
		return this.lodModeByInstance[scopeId] ?? this.lodModeByPane[pane];
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
	return platformFromViewContext(ctx)?.getActiveApp()?.controller as Puzzle2dPlayShellController | undefined;
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

function buildPuzzle2dPlayCompiledDagDeclarativeBody(_ctx: WindowBodyViewContext): UiNode {
	return buildWriterWindowBody(PUZZLE_2D_PLAY_SURFACE_ID_COMPILED_DAG, PUZZLE_2D_PLAY_CONTROLLER_ID, PUZZLE_2D_PLAY_WINDOW_KIND_COMPILED_DAG);
}
//#endregion 🔖DeclarativeBodies

/** @emoji 🧩 Registers puzzle 2d play window kinds on the supplied controller (layout supplied by host). */
export function attachPuzzle2dPlayWindowKinds(controller: Puzzle2dPlayShellController, layout: unknown): AppRuntime {
	const isWires = import.meta.env?.PUZZLE_PLAY_ENTRY === "wires";
	const app = new AppRuntime(
		PUZZLE_2D_PLAY_APP_ID,
		isWires ? "Wires" : "Puzzle 2D",
		undefined,
		controller,
		layout as never,
		[],
	);
	app.defaultModeId = controller.mainMode.id;
	app.addMode(controller.mainMode);
	return app;
}

/** @emoji 🧩 Builds the puzzle 2d play {@link AppRuntime}; side panels are tree tabs via {@link PlaygroundView} `augmentPanelTabs` only. */
export function buildPuzzle2dPlayAppRuntime(controller: Puzzle2dPlayShellController): AppRuntime {
	const app = attachPuzzle2dPlayWindowKinds(controller, PUZZLE_2D_PLAY_LAYOUT);
	controller.mainMode.namedLayouts = [createNamedLayout("puzzle-2d-default", "Default trio", PUZZLE_2D_PLAY_LAYOUT)];
	app.panelTabs = [];
	app.appSettingsBodyKey = PUZZLE_2D_PLAY_SETTINGS_BODY_KEY;
	return app;
}

/** @emoji 📝 Registers puzzle 2d play declarative window bodies on the playground host (side tabs are host tree panels only). */
export function registerPuzzle2dPlayDeclarativeBodies(): void {
	registerWindowBody(PUZZLE_2D_PLAY_BODY_KEY_OVERVIEW, buildPuzzle2dPlayOverviewDeclarativeBody);
	registerWindowBody(PUZZLE_2D_PLAY_BODY_KEY_DETAIL, buildPuzzle2dPlayDetailDeclarativeBody);
	registerWindowBody(PUZZLE_2D_PLAY_BODY_KEY_SELECTION, buildPuzzle2dPlaySelectionDeclarativeBody);
	registerWindowBody(PUZZLE_2D_PLAY_BODY_KEY_COMPILED_DAG, buildPuzzle2dPlayCompiledDagDeclarativeBody);
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

//#endregion 🔖Extension

export type Puzzle2dPlayStructuralDeleteItem = { kind: "edge" | "node"; id: string };

/** @emoji 🗑️ True when a canvas structural delete should update fixture state (wires stay canvas-only). */
export function puzzle2dPlayForwardsCanvasStructuralDelete(kind: "edge" | "node" | "wire", acceptFromCanvas: boolean): boolean {
	if (!acceptFromCanvas) {
		return false;
	}
	return kind === "edge" || kind === "node";
}

/** @emoji 🗑️ Removes a node and every edge anchored on that node id or its handles. */
export function puzzle2dPlayApplyNodeStructuralDeleteToFixture(fixture: Puzzle2dFixture, nodeId: string): Puzzle2dFixture {
	const node = fixture.nodes.find((row) => row.id === nodeId);
	if (!node) {
		return fixture;
	}
	const handleIds = new Set(node.handles.map((handle) => handle.id));
	return {
		...fixture,
		edges: fixture.edges.filter(
			(edge) => edge.source !== nodeId && edge.target !== nodeId && !handleIds.has(edge.source) && !handleIds.has(edge.target),
		),
		nodes: fixture.nodes.filter((row) => row.id !== nodeId),
	};
}

/** @emoji 🪢 Restores fixture edges from a seed when WASM/resync drained them (nakagin play recovery). */
export function puzzle2dPlayRehydrateFixtureEdgesIfMissing(fixture: Puzzle2dFixture, seed: Puzzle2dFixture): Puzzle2dFixture {
	if (fixture.edges.length > 0 || seed.edges.length === 0 || fixture.nodes.length === 0) {
		return fixture;
	}
	const endpointIds = new Set<string>();
	for (const node of fixture.nodes) {
		endpointIds.add(node.id);
		for (const handle of node.handles) {
			endpointIds.add(handle.id);
		}
	}
	const edges = seed.edges.filter((edge) => endpointIds.has(edge.source) && endpointIds.has(edge.target));
	if (edges.length === 0) {
		return fixture;
	}
	return { ...fixture, edges: edges.map((edge) => ({ ...edge })) };
}

/** @emoji 🗑️ Dedupes authoritative canvas structural deletes and drops ids absent from the fixture (renderer only emits user deletes). */
export function filterPuzzle2dPlayStructuralDeleteBatch(
	batch: readonly Puzzle2dPlayStructuralDeleteItem[],
	fixture: Puzzle2dFixture,
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
	return out;
}

/** @emoji 🗑️ Applies a queued structural-delete batch immediately (brush activation must flush before fixture resync). */
export function flushPuzzle2dPlayStructuralDeleteBatch(
	batch: readonly Puzzle2dPlayStructuralDeleteItem[],
	fixture: Puzzle2dFixture,
	apply: (kind: "edge" | "node", id: string) => void,
): readonly Puzzle2dPlayStructuralDeleteItem[] {
	const pending = filterPuzzle2dPlayStructuralDeleteBatch(batch, fixture);
	for (const item of pending) {
		apply(item.kind, item.id);
	}
	return pending;
}

/** @emoji 📋 Kind catalog rows as unique select options (last row wins per `id`; sorted by label). */
export function puzzle2dPlayKindCatalogSelectItems<T extends { readonly id: string; readonly label?: string; readonly name?: string }>(
	entries: readonly T[] | undefined,
): readonly { readonly value: string; readonly label: string }[] {
	if (!entries?.length) {
		return [];
	}
	const byId = new Map<string, { value: string; label: string }>();
	for (const entry of entries) {
		byId.set(entry.id, { value: entry.id, label: entry.label?.trim() || entry.name?.trim() || entry.id });
	}
	return [...byId.values()].sort((a, b) => a.label.localeCompare(b.label));
}

/** @emoji 🏷 Details inspector tree section title: singular for one id, plural for many. */
export function puzzle2dPlayInspectorKindSectionLabel(kind: "edge" | "handle" | "node", count: number): string {
	if (count === 1) {
		return kind === "node" ? "Node" : kind === "edge" ? "Edge" : "Handle";
	}
	return kind === "node" ? "Nodes" : kind === "edge" ? "Edges" : "Handles";
}

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("puzzle 2d play declarative shell", () => {
		it("resolves pane from shell instance ids", () => {
			expect(puzzle2dPlayPaneFromShellWindowId("2d-overview")).toBe("2d-overview");
			expect(puzzle2dPlayPaneFromShellWindowId("win-2d-detail-abc")).toBe("2d-detail");
			expect(puzzle2dPlayPaneFromShellWindowId("win-unknown")).toBeNull();
		});

		it("setLodModeForPane scopes LOD by instance id", () => {
			const bus = new CommandBus();
			const ctrl = new Puzzle2dPlayShellController(bus, () => {}, () => {});
			ctrl.run("setLodModeForPane", { pane: "2d-detail", value: "detail", instanceId: "win-2d-detail-a" });
			ctrl.run("setLodModeForPane", { pane: "2d-detail", value: "overview", instanceId: "win-2d-detail-b" });
			expect(ctrl.lodModeForScope("win-2d-detail-a", "2d-detail")).toBe("detail");
			expect(ctrl.lodModeForScope("win-2d-detail-b", "2d-detail")).toBe("overview");
			expect(ctrl.lodModeForScope("2d-detail", "2d-detail")).toBe("detail");
		});

		it("requires engagement.input on every window kind", () => {
			const bus = new CommandBus();
			const ctrl = new Puzzle2dPlayShellController(bus, () => {}, () => {});
			for (const windowKind of ctrl.mainMode.windowKinds) {
				expect(windowKind.engagement?.input?.id).toBe("engagement-input");
			}
		});

		it("puzzle2dPlayAllSelectionFromFixture lists every row for enabled targets", () => {
			const fixture = PUZZLE_2D_PLAY_DEFAULT_FIXTURE;
			const all = puzzle2dPlayAllSelectionFromFixture(fixture, { nodes: true, edges: true, handles: true });
			const handleCount = fixture.nodes.reduce((count, node) => count + node.handles.length, 0);
			expect(all.length).toBe(fixture.nodes.length + fixture.edges.length + handleCount);
			expect(all).toContain(fixture.nodes[0]!.id);
			const nodesOnly = puzzle2dPlayAllSelectionFromFixture(fixture, { nodes: true, edges: false, handles: false });
			expect(nodesOnly.length).toBe(fixture.nodes.length);
			expect(nodesOnly.every((id) => fixture.nodes.some((node) => node.id === id))).toBe(true);
		});

		it("selectAllSelection forwards to the host bridge", () => {
			const bus = new CommandBus();
			const ctrl = new Puzzle2dPlayShellController(bus, () => {}, () => {});
			let hostCommand: string | undefined;
			ctrl.setHostBridge({
				getToolbarState: () => ({
					puzzle2dActiveTool: "select",
					puzzle2dSuggestionOffset: DEFAULT_PUZZLE_2D_SUGGESTION_OFFSET_PX,
					puzzle2dGridSnapEnabled: false,
					puzzle2dRedrawPlaying: false,
					puzzle2dSelectionMethod: "rectangle",
					puzzle2dSelectionMode: "default",
					puzzle2dSelectionTargets: { nodes: true, edges: true, handles: true },
				}),
				runHostCommand: (command) => {
					hostCommand = command;
				},
			});
			ctrl.run("selectAllSelection");
			expect(hostCommand).toBe("selectAllSelection");
		});

		it("puzzle2d play registers ctrl+a select-all keybinding", () => {
			const playground = puzzle2dPlayAppDefinition.createPlayground();
			expect(playground.keybindings).toEqual(
				expect.arrayContaining([
					expect.objectContaining({
						key: "ctrl+a,meta+a",
						controllerId: PUZZLE_2D_PLAY_CONTROLLER_ID,
						command: "selectAllSelection",
					}),
				]),
			);
		});

		it("default concrete forest fixture parses with seed node", () => {
			expect(PUZZLE_2D_PLAY_DEFAULT_FIXTURE.nodes.some((node) => node.id === "seed-left-001")).toBe(true);
			expect(PUZZLE_2D_PLAY_DEFAULT_FIXTURE.nodes[0]?.handles.length).toBeGreaterThan(0);
		});

		it("nakagin fixture parses with puzzle 2d graph nodes", () => {
			const nakagin = puzzle2dPlayFixtureForId(PUZZLE_2D_PLAY_EXAMPLE_NAKAGIN_ID);
			expect(nakagin.nodes.length).toBeGreaterThan(0);
			expect(nakagin.edges.length).toBeGreaterThan(0);
			expect(parsePuzzle2dFixture(nakaginFixtureJson as unknown)?.nodes.length).toBe(nakagin.nodes.length);
		});

		it("fixture catalog lists concrete forest and nakagin", () => {
			expect(PUZZLE_2D_PLAY_EXAMPLE_OPTIONS.map((row) => row.id)).toEqual([
				PUZZLE_2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID,
				PUZZLE_2D_PLAY_EXAMPLE_NAKAGIN_ID,
			]);
		});

		it("concrete forest viewport camera centers on the seed node with room to grow", () => {
			const raw = puzzle2dPlayFixtureJson(PUZZLE_2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID) as { nodes: { x: number; y: number }[] };
			const fixture = puzzle2dPlayFixtureForId(PUZZLE_2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
			const authoredNode = raw.nodes[0];
			expect(authoredNode).toBeTruthy();
			const camera = puzzle2dPlayViewportCameraForFixtureId(PUZZLE_2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
			expect(camera.x).toBeCloseTo(authoredNode!.x, 3);
			expect(camera.y).toBeCloseTo(authoredNode!.y, 3);
			expect(camera.zoom).toBeGreaterThan(0.9);
			expect(camera.zoom).toBeLessThan(1.5);
			expect(fixture.nodes[0]?.shape).toBe("circle");
		});

		it("triptych cameras use distinct zoom per pane", () => {
			const fixture = puzzle2dPlayFixtureForId(PUZZLE_2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
			const raw = puzzle2dPlayFixtureJson(PUZZLE_2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
			const cameras = puzzle2dPlayTriptychCamerasFromFixture(fixture, raw);
			expect(cameras["2d-detail"].zoom).toBeGreaterThan(cameras["2d-overview"].zoom);
			expect(cameras["2d-overview"].zoom).toBeGreaterThan(cameras["2d-selection"].zoom);
			expect(cameras["2d-detail"].zoom / cameras["2d-overview"].zoom).toBeCloseTo(
				PUZZLE_2D_PLAY_VIEWPORT_PANE_ZOOM_SCALE["2d-detail"] / PUZZLE_2D_PLAY_VIEWPORT_PANE_ZOOM_SCALE["2d-overview"],
				4,
			);
		});

		it("nakagin hierarchy kind catalog uses specific human-readable node names", () => {
			const catalogNodes = (
				(nakaginFixtureJson as { meta?: { kindCatalogs?: { nodes?: readonly { readonly name: string }[] } } }).meta?.kindCatalogs?.nodes ?? []
			).map((row) => row.name);
			expect(catalogNodes).toEqual(expect.arrayContaining(["Capsule J", "Last Storey Tambour", "Cylindric Tambour"]));
			expect(catalogNodes.some((name) => name === "J" || name === "p" || name === "/" || name === "Tambour Last Storey")).toBe(false);
			expect(catalogNodes.some((name) => name.includes("compose."))).toBe(false);
		});

		it("buildPuzzle2dPlayKindsTree mirrors puzzle 3d catalog slices with 2d names", () => {
			const catalogs = puzzle2dFixtureMergedKindCatalogs(puzzle2dPlayFixtureForId(PUZZLE_2D_PLAY_EXAMPLE_NAKAGIN_ID));
			const tree = buildPuzzle2dPlayKindsTree(catalogs);
			expect(tree.type).toBe("tree");
			if (tree.type !== "tree") return;
			const sectionIds = tree.sections.map((section) => section.id);
			expect(sectionIds).toEqual(
				expect.arrayContaining([
					"puzzle-2d-play-kinds.nodes",
					"puzzle-2d-play-kinds.handles",
					"puzzle-2d-play-kinds.wires",
					"puzzle-2d-play-kinds.edges",
				]),
			);
			const nodes = tree.sections.find((section) => section.id === "puzzle-2d-play-kinds.nodes");
			const capsuleJ = nodes?.items?.find((item) => item.label === "Capsule J");
			expect(capsuleJ?.draggable).toBe(true);
			expect(capsuleJ?.dragData?.["application/x-puzzle-2d-fixture-v1"]).toBeTruthy();
			const dragFixture = decodePuzzle2dFixtureFromDrag(capsuleJ!.dragData!["application/x-puzzle-2d-fixture-v1"]!);
			expect(dragFixture?.nodes[0]?.iconKind).toBe("capsule_J");
			const handles = tree.sections.find((section) => section.id === "puzzle-2d-play-kinds.handles");
			expect(handles?.defaultOpen).toBe(false);
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
			const fixture = parsePuzzle2dFixture({
				schema: "puzzle.2d.fixture",
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

		it("puzzle2dPlayHierarchyTreeHighlightedIds prefers direct instance hover over kind hover", () => {
			const fixture = parsePuzzle2dFixture({
				schema: "puzzle.2d.fixture",
				camera: { x: 0, y: 0, zoom: 1 },
				nodes: [
					{
						id: "a",
						root: true,
						shape: "circle",
						text: "A",
						x: 0,
						y: 0,
						radius: 10,
						handles: [{ id: "h-a", angle: 0, handleKind: "port" }],
					},
					{
						id: "b",
						shape: "circle",
						text: "B",
						x: 10,
						y: 0,
						radius: 10,
						handles: [{ id: "h-b", angle: 0, handleKind: "port" }],
					},
				],
				edges: [],
			});
			expect(fixture).not.toBeNull();
			expect(
				puzzle2dPlayHierarchyTreeHighlightedIds(fixture!, "h-a", { domain: "handle", kindId: "port" }),
			).toEqual(["puzzle-2d-play-hierarchy.handle.h-a"]);
		});

		it("puzzle2dPlayHierarchyTreeHighlightedIdsForKind expands transitive handle kind hover", () => {
			const fixture = parsePuzzle2dFixture({
				schema: "puzzle.2d.fixture",
				camera: { x: 0, y: 0, zoom: 1 },
				nodes: [
					{
						id: "a",
						root: true,
						shape: "circle",
						text: "A",
						x: 0,
						y: 0,
						radius: 10,
						handles: [
							{ id: "h-a", angle: 0, handleKind: "port" },
							{ id: "h-b", angle: 90, handleKind: "other" },
						],
					},
					{
						id: "b",
						shape: "circle",
						text: "B",
						x: 10,
						y: 0,
						radius: 10,
						handles: [{ id: "h-c", angle: 0, handleKind: "port" }],
					},
				],
				edges: [],
			});
			expect(fixture).not.toBeNull();
			expect(puzzle2dPlayHierarchyTreeHighlightedIdsForKind(fixture!, { domain: "handle", kindId: "port" })).toEqual([
				"puzzle-2d-play-hierarchy.handle.h-a",
				"puzzle-2d-play-hierarchy.handle.h-c",
			]);
		});

		it("puzzle2dPlayHierarchyTreeSelectedIds maps graph ids to tree row ids", () => {
			const fixture = parsePuzzle2dFixture({
				schema: "puzzle.2d.fixture",
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

		it("buildPuzzle2dPlayHierarchySections nests children for node-id edges (normal graph)", () => {
			const fixture = parsePuzzle2dFixture({
				schema: "puzzle.2d.fixture",
				camera: { x: 0, y: 0, zoom: 1 },
				nodes: [
					{ id: "root", root: true, shape: "circle", text: "Root", x: 0, y: 0, radius: 10, handles: [] },
					{ id: "child", shape: "circle", text: "Child", x: 10, y: 0, radius: 10, handles: [] },
				],
				edges: [{ id: "e1", source: "root", target: "child" }],
			});
			expect(fixture).not.toBeNull();
			const tree = buildPuzzle2dPlayHierarchySections(fixture!, []);
			const nodesSection = tree.sections.find((section) => section.label === "Nodes");
			const rootItem = nodesSection?.items?.find((row) => row.id === "puzzle-2d-play-hierarchy.node.root");
			expect(rootItem?.items?.some((row) => row.label === "Child")).toBe(true);
			expect(rootItem?.items?.some((row) => row.label === "Handles")).toBe(false);
		});

		it("buildPuzzle2dPlayHierarchySections nests root nodes, handles, and child nodes", () => {
			const fixture = parsePuzzle2dFixture({
				schema: "puzzle.2d.fixture",
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
			const tree = buildPuzzle2dPlayHierarchySections(fixture!, []);
			const nodesSection = tree.sections.find((section) => section.label === "Nodes");
			expect(nodesSection?.items?.[0]?.id).toBe("puzzle-2d-play-hierarchy.node.root");
			expect(nodesSection?.items?.[0]?.label).toBe("Root");
			expect(nodesSection?.items?.[0]?.items?.[0]?.id).toBe("puzzle-2d-play-hierarchy.handle.h-root");
			expect(nodesSection?.items?.[0]?.items?.[1]?.label).toBe("Child");
		});

		it("buildPuzzle2dPlayRuntime wires main mode and empty side tab slots", () => {
			const runtime = buildPuzzle2dPlayRuntime();
			const app = runtime.getActiveApp();
			expect(app?.panelTabs).toEqual([]);
			expect(app?.controller.mainMode.tools ?? []).toEqual([]);
		});

		it("buildPuzzle2dPlayAppRuntime wires appSettingsBodyKey for framework App settings tab", () => {
			const runtime = buildPuzzle2dPlayRuntime();
			expect(runtime.getActiveApp()?.appSettingsBodyKey).toBe(PUZZLE_2D_PLAY_SETTINGS_BODY_KEY);
		});

		it("engagementAbort exits brush and clears command line", () => {
			const bus = new CommandBus();
			let hostTool: Puzzle2dActiveTool | undefined;
			const ctrl = new Puzzle2dPlayShellController(bus, () => {}, () => {});
			ctrl.setHostBridge({
				getToolbarState: () => ({
					puzzle2dActiveTool: "select",
					puzzle2dSuggestionOffset: DEFAULT_PUZZLE_2D_SUGGESTION_OFFSET_PX,
					puzzle2dGridSnapEnabled: false,
					puzzle2dRedrawPlaying: false,
					puzzle2dSelectionMethod: "rectangle",
					puzzle2dSelectionMode: "default",
					puzzle2dSelectionTargets: { nodes: true, edges: true, handles: true },
				}),
				runHostCommand: (command, args) => {
					if (command === "setActiveTool") {
						hostTool = (args as { tool: Puzzle2dActiveTool }).tool;
					}
				},
			});
			bus.dispatch(PUZZLE_2D_PLAY_CONTROLLER_ID, "engagementSubmit", { pane: "2d-overview", value: "Brush" });
			expect(ctrl.getActiveTool()).toBe("brush");
			bus.dispatch(PUZZLE_2D_PLAY_CONTROLLER_ID, "engagementAbort", { pane: "2d-overview" });
			expect(ctrl.getActiveTool()).toBe("select");
			expect(hostTool).toBe("select");
			const engagement = ctrl.mainMode.windowKinds.find((wk) => wk.id === "2d-overview")?.engagement;
			expect(engagement?.sessionActive).toBeFalsy();
			expect(engagement?.input?.value).toBe("");
		});

		it("engagementSubmit Fill activates fill on shell with slider control", () => {
			const bus = new CommandBus();
			let hostTool: Puzzle2dActiveTool | undefined;
			const ctrl = new Puzzle2dPlayShellController(bus, () => {}, () => {});
			ctrl.setHostBridge({
				getToolbarState: () => ({
					puzzle2dActiveTool: "select",
					puzzle2dSuggestionOffset: DEFAULT_PUZZLE_2D_SUGGESTION_OFFSET_PX,
					puzzle2dGridSnapEnabled: false,
					puzzle2dRedrawPlaying: false,
					puzzle2dSelectionMethod: "rectangle",
					puzzle2dSelectionMode: "default",
					puzzle2dSelectionTargets: { nodes: true, edges: true, handles: true },
				}),
				runHostCommand: (command, args) => {
					if (command === "setActiveTool") {
						hostTool = (args as { tool: Puzzle2dActiveTool }).tool;
					}
				},
			});
			bus.dispatch(PUZZLE_2D_PLAY_CONTROLLER_ID, "engagementSubmit", { pane: "2d-overview", value: "Fill" });
			expect(ctrl.getActiveTool()).toBe("fill");
			expect(hostTool).toBe("fill");
			const engagement = ctrl.mainMode.windowKinds.find((wk) => wk.id === "2d-overview")?.engagement;
			expect(engagement?.sessionActive).toBe(true);
			expect(engagement?.control?.kind).toBe("slider");
		});

		it("applyPuzzle2dFillCount composes cached appended nodes without replaying placements", () => {
			const base = PUZZLE_2D_PLAY_EMPTY_FIXTURE;
			const placement = {
				sourceHandleId: "missing",
				nodeKind: "brush.kind",
				x: 0,
				y: 0,
				handles: [{ handleKind: "child", angle: 0 }],
				targetHandleIndex: 0,
			};
			puzzle2dFillSessionRef.current = {
				baseFixture: base,
				sequence: [placement],
				appendedNodes: [{ id: "fill.node", shape: "circle", radius: 20, x: 0, y: 0, handles: [] }],
				appendedEdges: [{ id: "fill.edge", source: "missing", target: "fill.node.h0" }],
				seed: 1,
			};
			const applied = applyPuzzle2dFillCount(1);
			expect(applied?.nodes).toHaveLength(1);
			expect(applied?.edges).toHaveLength(1);
			clearPuzzle2dFillSession();
			expect(puzzle2dFillSessionRef.current.baseFixture).toBeNull();
		});

		it("engagementSubmit Brush activates brush on shell and forwards setActiveTool to host", () => {
			const bus = new CommandBus();
			let hostTool: Puzzle2dActiveTool | undefined;
			const ctrl = new Puzzle2dPlayShellController(bus, () => {}, () => {});
			ctrl.setHostBridge({
				getToolbarState: () => ({
					puzzle2dActiveTool: "select",
					puzzle2dSuggestionOffset: DEFAULT_PUZZLE_2D_SUGGESTION_OFFSET_PX,
					puzzle2dGridSnapEnabled: false,
					puzzle2dRedrawPlaying: false,
					puzzle2dSelectionMethod: "rectangle",
					puzzle2dSelectionMode: "default",
					puzzle2dSelectionTargets: { nodes: true, edges: true, handles: true },
				}),
				runHostCommand: (command, args) => {
					if (command === "setActiveTool") {
						hostTool = (args as { tool: Puzzle2dActiveTool }).tool;
					}
				},
			});
			bus.dispatch(PUZZLE_2D_PLAY_CONTROLLER_ID, "engagementSubmit", { pane: "2d-overview", value: "Brush" });
			expect(ctrl.getActiveTool()).toBe("brush");
			expect(hostTool).toBe("brush");
			const engagement = ctrl.mainMode.windowKinds.find((wk) => wk.id === "2d-overview")?.engagement;
			expect(engagement?.input?.placeholder).toBe("Brush");
			expect(engagement?.sessionActive).toBe(true);
		});

		it("suggestion offset measure is registered on play windows", () => {
			const runtime = buildPuzzle2dPlayRuntime();
			const controller = runtime.getActiveApp()?.controller as Puzzle2dPlayShellController;
			const overview = controller.mainMode.windowKinds.find((wk) => wk.id === "2d-overview");
			const suggestion = overview?.measures?.find((m) => m.kind === "group" && m.label === "Suggestion");
			expect(suggestion?.kind).toBe("group");
			expect(suggestion?.children?.some((m) => m.kind === "slider" && m.id.includes("suggestion-offset") && m.label === "Offset")).toBe(true);
		});

		it("setKindCatalogs seeds nakagin default suggestion ratios for nodes and handles", () => {
			const bus = new CommandBus();
			let pushed: { nodeWeights: Record<string, number>; handleWeights: Record<string, number> } | undefined;
			const ctrl = new Puzzle2dPlayShellController(bus, () => {}, () => {});
			ctrl.setHostBridge({
				getToolbarState: () => ({
					puzzle2dActiveTool: "select",
					puzzle2dSuggestionOffset: DEFAULT_PUZZLE_2D_SUGGESTION_OFFSET_PX,
					puzzle2dGridSnapEnabled: false,
					puzzle2dRedrawPlaying: false,
					puzzle2dSelectionMethod: "rectangle",
					puzzle2dSelectionMode: "default",
					puzzle2dSelectionTargets: { nodes: true, edges: true, handles: true },
				}),
				runHostCommand: (command, args) => {
					if (command === "setBrushKindWeights") {
						pushed = args as typeof pushed;
					}
				},
			});
			ctrl.setKindCatalogs({
				nodes: [
					{ id: "Base", name: "Base" },
					{ id: "Capital", name: "Capital" },
					{ id: "Tambour", name: "Tambour" },
					{ id: "Capsule J", name: "Capsule J" },
				],
				handles: [
					{ id: "core rectangular bottom", name: "core rectangular bottom", color: "#000" },
					{ id: "tambour circular top", name: "tambour circular top", color: "#000" },
					{ id: "door capsule right", name: "door capsule right", color: "#000" },
				],
			});
			expect(pushed).toBeDefined();
			const nw = pushed!.nodeWeights;
			expect(nw.Tambour / nw.Base).toBeCloseTo(15, 4);
			expect(nw.Tambour / nw.Capital).toBeCloseTo(10, 4);
			expect(nw["Capsule J"] / nw.Tambour).toBeCloseTo(8, 4);
			const hw = pushed!.handleWeights;
			expect(hw["tambour circular top"] / hw["core rectangular bottom"]).toBeCloseTo(15, 4);
			expect(hw["door capsule right"] / hw["tambour circular top"]).toBeCloseTo(8, 4);
		});

		it("brush distribution measures label node kinds from catalog names not uuid tails", () => {
			const bus = new CommandBus();
			const ctrl = new Puzzle2dPlayShellController(bus, () => {}, () => {});
			const catalogs = puzzle2dFixtureMergedKindCatalogs(puzzle2dPlayFixtureForId(PUZZLE_2D_PLAY_EXAMPLE_NAKAGIN_ID));
			ctrl.setKindCatalogs(catalogs);
			const collectLabels = (measures: readonly WindowMeasure[] | undefined): string[] => {
				const out: string[] = [];
				for (const row of measures ?? []) {
					if (row.kind === "group") {
						out.push(...collectLabels(row.children));
					} else if ("label" in row && typeof row.label === "string") {
						out.push(row.label);
					}
				}
				return out;
			};
			const overview = ctrl.mainMode.windowKinds.find((wk) => wk.id === "2d-overview");
			const labels = collectLabels(overview?.measures);
			expect(labels.some((label) => label.startsWith("Capsule J "))).toBe(true);
			expect(labels.some((label) => /^[0-9a-f]{8}/i.test(label))).toBe(false);
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

		it("setLodModeForPane bumps runtime and chrome generation", () => {
			const runtime = buildPuzzle2dPlayRuntime();
			const controller = runtime.getActiveApp()?.controller as Puzzle2dPlayShellController;
			const dataGen = runtime.generation;
			const chromeGen = runtime.chromeGeneration;
			controller.commandBus.dispatch(PUZZLE_2D_PLAY_CONTROLLER_ID, "setLodModeForPane", {
				pane: "2d-detail",
				value: "minimap",
			});
			expect(runtime.generation).toBe(dataGen + 1);
			expect(runtime.chromeGeneration).toBe(chromeGen + 1);
			expect(controller.lodModeForScope("2d-detail", "2d-detail")).toBe("minimap");
			const nextDataGen = runtime.generation;
			const nextChromeGen = runtime.chromeGeneration;
			controller.commandBus.dispatch(PUZZLE_2D_PLAY_CONTROLLER_ID, "setLodModeForPane", {
				pane: "2d-detail",
				value: "detail",
			});
			expect(runtime.generation).toBe(nextDataGen + 1);
			expect(runtime.chromeGeneration).toBe(nextChromeGen + 1);
			expect(controller.lodModeForScope("2d-detail", "2d-detail")).toBe("detail");
		});
	});

	describe("puzzle2dPlayForwardsCanvasStructuralDelete", () => {
		it("forwards edge and node deletes when the canvas is ready", () => {
			expect(puzzle2dPlayForwardsCanvasStructuralDelete("edge", true)).toBe(true);
			expect(puzzle2dPlayForwardsCanvasStructuralDelete("node", true)).toBe(true);
		});

		it("drops wire deletes and blocks until the canvas accepts structural deletes", () => {
			expect(puzzle2dPlayForwardsCanvasStructuralDelete("wire", true)).toBe(false);
			expect(puzzle2dPlayForwardsCanvasStructuralDelete("edge", false)).toBe(false);
		});
	});

	describe("puzzle2dPlayApplyNodeStructuralDeleteToFixture", () => {
		it("removes node-id and handle-id edges when deleting a node", () => {
			const fixture: Puzzle2dFixture = {
				schema: "puzzle.2d.fixture",
				camera: { x: 0, y: 0, zoom: 1 },
				nodes: [
					{ id: "root", x: 0, y: 0, radius: 10, handles: [{ id: "h-root", angle: 0, handleKind: "port" }] },
					{ id: "child", x: 40, y: 0, radius: 10, handles: [{ id: "h-child", angle: Math.PI, handleKind: "port" }] },
				],
				edges: [
					{ id: "node-edge", source: "root", target: "child" },
					{ id: "handle-edge", source: "h-root", target: "h-child" },
				],
			};
			const next = puzzle2dPlayApplyNodeStructuralDeleteToFixture(fixture, "root");
			expect(next.nodes.map((node) => node.id)).toEqual(["child"]);
			expect(next.edges).toEqual([]);
		});
	});

	describe("puzzle2dPlayRehydrateFixtureEdgesIfMissing", () => {
		it("restores seed edges only when fixture nodes still exist", () => {
			const seed: Puzzle2dFixture = {
				schema: "puzzle.2d.fixture",
				camera: { x: 0, y: 0, zoom: 1 },
				nodes: [
					{ id: "a", x: 0, y: 0, radius: 10, handles: [{ id: "a:h0", angle: 0, handleKind: "port" }] },
					{ id: "b", x: 40, y: 0, radius: 10, handles: [{ id: "b:h0", angle: Math.PI, handleKind: "port" }] },
				],
				edges: [{ id: "e0", source: "a:h0", target: "b:h0" }],
			};
			const drained = { ...seed, edges: [] as Puzzle2dFixture["edges"] };
			expect(puzzle2dPlayRehydrateFixtureEdgesIfMissing(drained, seed).edges).toEqual(seed.edges);
		});

		it("does not restore edges after delete-all leaves no fixture nodes", () => {
			const seed: Puzzle2dFixture = {
				schema: "puzzle.2d.fixture",
				camera: { x: 0, y: 0, zoom: 1 },
				nodes: [{ id: "a", x: 0, y: 0, radius: 10, handles: [] }],
				edges: [{ id: "e0", source: "a", target: "a" }],
			};
			const cleared = { ...seed, nodes: [], edges: [] };
			expect(puzzle2dPlayRehydrateFixtureEdgesIfMissing(cleared, seed).edges).toEqual([]);
		});
	});

	describe("filterPuzzle2dPlayStructuralDeleteBatch", () => {
		it("keeps real multi-edge node deletes and drops resync-only ghost ids", () => {
			const fixture: Puzzle2dFixture = {
				schema: "puzzle.2d.fixture",
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

		it("keeps mass node deletes from authoritative canvas delete", () => {
			const fixture: Puzzle2dFixture = {
				schema: "puzzle.2d.fixture",
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
			expect(filterPuzzle2dPlayStructuralDeleteBatch(batch, fixture)).toEqual(batch);
		});

		it("keeps sequential and mass edge deletes from authoritative canvas delete", () => {
			const fixture: Puzzle2dFixture = {
				schema: "puzzle.2d.fixture",
				camera: { x: 0, y: 0, zoom: 1 },
				nodes: [
					{ id: "a", x: 0, y: 0, radius: 10, handles: [{ id: "a.h0", angle: 0 }] },
					{ id: "b", x: 40, y: 0, radius: 10, handles: [{ id: "b.h0", angle: Math.PI }] },
				],
				edges: [
					{ id: "e0", source: "a.h0", target: "b.h0" },
					{ id: "e1", source: "a.h0", target: "b.h0" },
					{ id: "e2", source: "a.h0", target: "b.h0" },
				],
			};
			expect(filterPuzzle2dPlayStructuralDeleteBatch([{ kind: "edge", id: "e0" }], fixture)).toEqual([{ kind: "edge", id: "e0" }]);
			const batch = [
				{ kind: "edge" as const, id: "e0" },
				{ kind: "edge" as const, id: "e1" },
				{ kind: "edge" as const, id: "e2" },
			];
			expect(filterPuzzle2dPlayStructuralDeleteBatch(batch, fixture)).toEqual(batch);
		});

		it("keeps paired edge deletes on large nakagin-scale fixtures", () => {
			const fixture: Puzzle2dFixture = {
				schema: "puzzle.2d.fixture",
				camera: { x: 0, y: 0, zoom: 1 },
				nodes: [{ id: "a", x: 0, y: 0, radius: 10, handles: [{ id: "a.h0", angle: 0 }] }],
				edges: Array.from({ length: 30 }, (_, i) => ({ id: `e${i}`, source: "a.h0", target: "a.h0" })),
			};
			const batch = [
				{ kind: "edge" as const, id: "e0" },
				{ kind: "edge" as const, id: "e1" },
			];
			expect(filterPuzzle2dPlayStructuralDeleteBatch(batch, fixture)).toEqual(batch);
		});
	});

	describe("flushPuzzle2dPlayStructuralDeleteBatch", () => {
		it("applies filtered edge deletes before brush fixture resync", () => {
			const fixture: Puzzle2dFixture = {
				schema: "puzzle.2d.fixture",
				camera: { x: 0, y: 0, zoom: 1 },
				nodes: [
					{ id: "a", x: 0, y: 0, radius: 10, handles: [{ id: "a.h0", angle: 0 }] },
					{ id: "b", x: 40, y: 0, radius: 10, handles: [{ id: "b.h0", angle: Math.PI }] },
				],
				edges: [{ id: "e0", source: "a.h0", target: "b.h0" }],
			};
			const applied: Puzzle2dPlayStructuralDeleteItem[] = [];
			let nextFixture = fixture;
			const appliedDeletes = flushPuzzle2dPlayStructuralDeleteBatch([{ kind: "edge", id: "e0" }], nextFixture, (kind, id) => {
				applied.push({ kind, id });
				if (kind === "edge") {
					nextFixture = { ...nextFixture, edges: nextFixture.edges.filter((edge) => edge.id !== id) };
				}
			});
			expect(appliedDeletes).toEqual([{ kind: "edge", id: "e0" }]);
			expect(applied).toEqual([{ kind: "edge", id: "e0" }]);
			expect(nextFixture.edges).toEqual([]);
		});
	});

	it("nakagin fixture yields a populated hierarchy nodes group", () => {
		const tree = buildPuzzle2dPlayHierarchySections(puzzle2dPlayFixtureForId(PUZZLE_2D_PLAY_EXAMPLE_NAKAGIN_ID), []);
		const nodesSection = tree.sections.find((section) => section.label === "Nodes");
		expect(nodesSection?.items?.length).toBeGreaterThan(0);
		expect(nodesSection?.items?.[0]?.label).not.toBe("(none)");
	});

	describe("puzzle2dPlayKindCatalogSelectItems", () => {
		it("dedupes duplicate catalog ids for inspector selects", () => {
			const items = puzzle2dPlayKindCatalogSelectItems([
				{ id: "kind-a", name: "Kind A" },
				{ id: "kind-a", name: "Kind A (duplicate)" },
				{ id: "kind-b", name: "Kind B" },
			]);
			expect(items.map((item) => item.value)).toEqual(["kind-a", "kind-b"]);
			expect(items.find((item) => item.value === "kind-a")?.label).toBe("Kind A (duplicate)");
		});
	});

	describe("puzzle2dPlayInspectorKindSectionLabel", () => {
		it("uses singular titles for one selected id and plural for many", () => {
			expect(puzzle2dPlayInspectorKindSectionLabel("node", 1)).toBe("Node");
			expect(puzzle2dPlayInspectorKindSectionLabel("node", 2)).toBe("Nodes");
			expect(puzzle2dPlayInspectorKindSectionLabel("edge", 1)).toBe("Edge");
			expect(puzzle2dPlayInspectorKindSectionLabel("edge", 3)).toBe("Edges");
			expect(puzzle2dPlayInspectorKindSectionLabel("handle", 1)).toBe("Handle");
			expect(puzzle2dPlayInspectorKindSectionLabel("handle", 4)).toBe("Handles");
		});
	});

	describe("puzzle2d play selection context menu helpers", () => {
		const baseFixture = PUZZLE_2D_PLAY_DEFAULT_FIXTURE;

		it("applySelectionFlag sets hidden on selected nodes", () => {
			const nodeId = baseFixture.nodes[0]!.id;
			const next = puzzle2dPlayApplySelectionFlag(baseFixture, [nodeId], "hidden", true);
			expect(next.nodes.find((row) => row.id === nodeId)?.hidden).toBe(true);
		});

		it("toggleEntityFlag flips locked on a handle row", () => {
			const handleId = baseFixture.nodes[0]!.handles[0]!.id;
			const next = puzzle2dPlayToggleEntityFlag(baseFixture, handleId, "locked");
			expect(puzzle2dPlayEntityFlagsFromFixture(next, handleId).locked).toBe(true);
		});

		it("duplicateSelection clones nodes with new ids", () => {
			const nodeId = baseFixture.nodes[0]!.id;
			const { fixture: next, newIds } = puzzle2dPlayDuplicateSelection(baseFixture, [nodeId]);
			expect(newIds.length).toBe(1);
			expect(next.nodes.some((row) => row.id === newIds[0])).toBe(true);
			expect(newIds[0]).not.toBe(nodeId);
		});

		it("selectSameKindIds expands nodeKind matches", () => {
			const fixture: Puzzle2dFixture = {
				...baseFixture,
				nodes: baseFixture.nodes.map((node, index) => ({ ...node, nodeKind: index === 0 ? "kind-a" : node.nodeKind })),
			};
			const ids = puzzle2dPlaySelectSameKindIds(fixture, [fixture.nodes[0]!.id]);
			expect(ids).toContain(fixture.nodes[0]!.id);
		});
	});
}
//#endregion 🧪Tests

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";

/** @emoji 🧩 S program definition for puzzle 2d. */
export function buildPuzzle2dProgramDefinition(): PlatformDefinition {
	return {
		id: "puzzle.2d",
		name: "Puzzle 2D",
		apiVersion: "1",
		apps: [{ id: "puzzle2d", label: "Puzzle 2D", controllerId: PUZZLE_2D_PLAY_CONTROLLER_ID, modes: [{ id: "edit", label: "Edit" }], defaultModeId: "edit" }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension

//#region 🔖Play
import nakaginFixtureJson from "../../example/nakagin-capsule-tower.2d.json";
import concreteForestFixtureJson from "../../example/concrete-forest.2d.json";

export const PUZZLE_2D_PLAY_EXAMPLE_NAKAGIN_ID = "nakagin";
export const PUZZLE_2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID = "concrete-forest";

export const PUZZLE_2D_PLAY_EXAMPLE_OPTIONS = [
	{ id: PUZZLE_2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID, label: "Concrete Forest" },
	{ id: PUZZLE_2D_PLAY_EXAMPLE_NAKAGIN_ID, label: "Nakagin capsule tower" },
] as const;

/** @emoji 🔒 Resolves a playground example slug (e.g. `concrete`) to a puzzle 2d example id. */
export function resolvePuzzle2dPlayExampleSlug(slug: string): string | undefined {
	const aliases: Record<string, string> = { concrete: PUZZLE_2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID };
	const normalized = aliases[slug] ?? slug;
	return PUZZLE_2D_PLAY_EXAMPLE_OPTIONS.some((row) => row.id === normalized) ? normalized : undefined;
}

const PUZZLE_2D_PLAY_EXAMPLE_JSON_BY_ID: Record<string, unknown> = {
	[PUZZLE_2D_PLAY_EXAMPLE_NAKAGIN_ID]: nakaginFixtureJson,
	[PUZZLE_2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID]: concreteForestFixtureJson,
};

/** @emoji 🧪 Resolves imported puzzle 2d example JSON by catalog id. */
export function puzzle2dPlayFixtureJson(fixtureId: string = PUZZLE_2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID): unknown {
	return PUZZLE_2D_PLAY_EXAMPLE_JSON_BY_ID[fixtureId] ?? concreteForestFixtureJson;
}

/** @emoji 📋 Parses a puzzle 2d play example by catalog id. */
export function puzzle2dPlayFixtureForId(fixtureId: string): Puzzle2dFixture {
	const parsed = parsePuzzle2dFixture(puzzle2dPlayFixtureJson(fixtureId) as unknown);
	if (!parsed) throw new Error(`puzzle 2d example "${fixtureId}" is invalid`);
	return parsed;
}

/** @emoji 📄 Serializes a puzzle 2d fixture for Jack and VCS bridges. */
export function puzzle2dFixtureToJson(fixture: Puzzle2dFixture): string {
	return JSON.stringify(fixture);
}

/** @emoji 🃏 Normalizes a puzzle 2d fixture into board-shaped JSON for Jack queries. */
export function puzzle2dFixtureToJackBoardJson(fixtureOrJson: Puzzle2dFixture | string): string {
	const fixture =
		typeof fixtureOrJson === "string"
			? (parsePuzzle2dFixture(JSON.parse(fixtureOrJson) as unknown) ?? PUZZLE_2D_PLAY_EMPTY_FIXTURE)
			: fixtureOrJson;
	return JSON.stringify({
		schema: fixture.schema,
		nodes: fixture.nodes.map((node) => ({
			id: node.id,
			nodeKind: "node",
			text: puzzle2dFixtureNodeDisplayLabel(node),
		})),
		edges: fixture.edges,
	});
}

/** @emoji 🔌 Renders a puzzle fixture as wire-literal compiled DAG text. */
export function puzzle2dFixtureToCompiledDagWireLiteral(fixtureOrJson: Puzzle2dFixture | string): string {
	const fixture =
		typeof fixtureOrJson === "string"
			? (parsePuzzle2dFixture(JSON.parse(fixtureOrJson) as unknown) ?? PUZZLE_2D_PLAY_EMPTY_FIXTURE)
			: fixtureOrJson;
	return wireLiteralFromDagFixtureJson(
		JSON.stringify({
			nodes: fixture.nodes.map((node) => ({
				id: node.id,
				operatorKind: node.nodeKind ?? "node",
			})),
			edges: fixture.edges.map((edge) => ({
				id: edge.id,
				source: edge.source,
				target: edge.target,
			})),
		}),
	);
}

export const PUZZLE_2D_PLAY_DEFAULT_FIXTURE: Puzzle2dFixture = puzzle2dPlayFixtureForId(PUZZLE_2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);

export const PUZZLE_2D_PLAY_EMPTY_FIXTURE: Puzzle2dFixture = {
	schema: "puzzle.2d.fixture",
	camera: { x: 0, y: 0, zoom: 1 },
	nodes: [],
	edges: [],
};

const PUZZLE_2D_PLAY_VIEWPORT_REF_SHORT_PX = 640;
const PUZZLE_2D_PLAY_VIEWPORT_MARGIN = 0.18;
const PUZZLE_2D_PLAY_VIEWPORT_FRAMING_HALF_SPAN_SCALE = 2.25;
const PUZZLE_2D_PLAY_VIEWPORT_ZOOM_BOOST = 2.5;
const PUZZLE_2D_PLAY_VIEWPORT_PANE_ZOOM_SCALE: Record<Puzzle2dPlayPaneId, number> = {
	"2d-overview": 0.68,
	"2d-detail": 2.15,
	"2d-selection": 0.36,
};

function clampPuzzle2dPlayViewportZoom(value: number): number {
	return Math.min(PUZZLE_2D_CAMERA_ZOOM_MAX, Math.max(PUZZLE_2D_CAMERA_ZOOM_MIN, value));
}

function puzzle2dPlayNodeWorldExtents(node: Record<string, unknown>): { minX: number; minY: number; maxX: number; maxY: number } | null {
	const x = Number(node.x);
	const y = Number(node.y);
	if (!Number.isFinite(x) || !Number.isFinite(y)) {
		return null;
	}
	if (node.shape === "rectangle") {
		const width = Number(node.width);
		const height = Number(node.height);
		if (!Number.isFinite(width) || !Number.isFinite(height) || width <= 0 || height <= 0) {
			return null;
		}
		const hw = width / 2;
		const hh = height / 2;
		return { minX: x - hw, maxX: x + hw, minY: y - hh, maxY: y + hh };
	}
	const radius = Number(node.radius);
	if (!Number.isFinite(radius) || radius <= 0) {
		return null;
	}
	return { minX: x - radius, maxX: x + radius, minY: y - radius, maxY: y + radius };
}

function puzzle2dPlayFixtureWorldBoundsFromNodeRecords(nodes: readonly Record<string, unknown>[]): { cx: number; cy: number; halfSpan: number } {
	let minX = Number.POSITIVE_INFINITY;
	let minY = Number.POSITIVE_INFINITY;
	let maxX = Number.NEGATIVE_INFINITY;
	let maxY = Number.NEGATIVE_INFINITY;
	for (const node of nodes) {
		const extents = puzzle2dPlayNodeWorldExtents(node);
		if (!extents) continue;
		minX = Math.min(minX, extents.minX);
		maxX = Math.max(maxX, extents.maxX);
		minY = Math.min(minY, extents.minY);
		maxY = Math.max(maxY, extents.maxY);
	}
	if (!Number.isFinite(minX)) {
		return { cx: 0, cy: 0, halfSpan: 400 };
	}
	const cx = (minX + maxX) / 2;
	const cy = (minY + maxY) / 2;
	const halfSpan = Math.max(maxX - minX, maxY - minY, 1) / 2;
	return { cx, cy, halfSpan };
}

function puzzle2dPlayFixtureWorldBounds(fixture: Puzzle2dFixture): { cx: number; cy: number; halfSpan: number } {
	return puzzle2dPlayFixtureWorldBoundsFromNodeRecords(fixture.nodes as unknown as Record<string, unknown>[]);
}

function puzzle2dPlayFixtureWorldBoundsFromJson(raw: unknown): { cx: number; cy: number; halfSpan: number } | null {
	if (!raw || typeof raw !== "object") return null;
	const nodes = (raw as Record<string, unknown>).nodes;
	if (!Array.isArray(nodes)) return null;
	const records = nodes.filter((node): node is Record<string, unknown> => Boolean(node) && typeof node === "object");
	if (!records.length) return null;
	return puzzle2dPlayFixtureWorldBoundsFromNodeRecords(records);
}

function puzzle2dPlayViewportCameraFromBounds(
	fixture: Puzzle2dFixture,
	bounds: { cx: number; cy: number; halfSpan: number },
): CameraState {
	const usable = PUZZLE_2D_PLAY_VIEWPORT_REF_SHORT_PX * (1 - 2 * PUZZLE_2D_PLAY_VIEWPORT_MARGIN);
	const worldSpan = Math.max(2 * bounds.halfSpan * PUZZLE_2D_PLAY_VIEWPORT_FRAMING_HALF_SPAN_SCALE, 1);
	const zoom = clampPuzzle2dPlayViewportZoom((usable / worldSpan) * PUZZLE_2D_PLAY_VIEWPORT_ZOOM_BOOST);
	return {
		x: bounds.cx,
		y: bounds.cy,
		zoom,
	};
}

/** @emoji 📷 Viewport camera centered on fixture node bounds with zoom fitted for growth. */
export function puzzle2dPlayViewportCameraFromFixture(fixture: Puzzle2dFixture, rawFixture?: unknown): CameraState {
	const bounds = (rawFixture ? puzzle2dPlayFixtureWorldBoundsFromJson(rawFixture) : null) ?? puzzle2dPlayFixtureWorldBounds(fixture);
	return puzzle2dPlayViewportCameraFromBounds(fixture, bounds);
}

/** @emoji 📷 Viewport camera for a play example catalog id (uses raw JSON bounds before circle normalization). */
export function puzzle2dPlayViewportCameraForFixtureId(fixtureId: string): CameraState {
	const raw = puzzle2dPlayFixtureJson(fixtureId);
	return puzzle2dPlayViewportCameraFromFixture(puzzle2dPlayFixtureForId(fixtureId), raw);
}

function puzzle2dPlayTriptychCameraForPane(
	pane: Puzzle2dPlayPaneId,
	fixture: Puzzle2dFixture,
	bounds: { cx: number; cy: number; halfSpan: number },
	baseZoom: number,
): CameraState {
	const camOffset = fixture.camera;
	const detailNode = fixture.nodes[Math.min(42, Math.max(0, fixture.nodes.length - 1))];
	const zoom = clampPuzzle2dPlayViewportZoom(baseZoom * PUZZLE_2D_PLAY_VIEWPORT_PANE_ZOOM_SCALE[pane]);
	switch (pane) {
		case "2d-overview":
			return { x: bounds.cx + camOffset.x * 0.04, y: bounds.cy + camOffset.y * 0.03, zoom };
		case "2d-detail":
			return {
				x: (detailNode?.x ?? bounds.cx) + camOffset.x * 0.02,
				y: (detailNode?.y ?? bounds.cy) + camOffset.y * 0.02,
				zoom,
			};
		case "2d-selection":
			return {
				x: bounds.cx - bounds.halfSpan * 0.28 + camOffset.x * 0.06,
				y: bounds.cy + bounds.halfSpan * 0.22 + camOffset.y * 0.05,
				zoom,
			};
	}
}

/** @emoji 📷 Default cameras for all puzzle 2d play panes (wide overview, tight detail, regional selection). */
export function puzzle2dPlayTriptychCamerasFromFixture(fixture: Puzzle2dFixture, rawFixture?: unknown): Record<Puzzle2dPlayPaneId, CameraState> {
	const bounds = (rawFixture ? puzzle2dPlayFixtureWorldBoundsFromJson(rawFixture) : null) ?? puzzle2dPlayFixtureWorldBounds(fixture);
	const base = puzzle2dPlayViewportCameraFromBounds(fixture, bounds);
	return {
		"2d-overview": puzzle2dPlayTriptychCameraForPane("2d-overview", fixture, bounds, base.zoom),
		"2d-detail": puzzle2dPlayTriptychCameraForPane("2d-detail", fixture, bounds, base.zoom),
		"2d-selection": puzzle2dPlayTriptychCameraForPane("2d-selection", fixture, bounds, base.zoom),
	};
}




//#region 🔖MediaExport
function puzzle2dFixtureToSvg(fixture: Puzzle2dFixture): string {
	const padding = 40;
	const xs = fixture.nodes.map((node) => node.x);
	const ys = fixture.nodes.map((node) => node.y);
	const radii = fixture.nodes.map((node) => node.radius);
	const minX = Math.min(...xs.map((x, i) => x - (radii[i] ?? 0)), 0) - padding;
	const minY = Math.min(...ys.map((y, i) => y - (radii[i] ?? 0)), 0) - padding;
	const maxX = Math.max(...xs.map((x, i) => x + (radii[i] ?? 0)), 1) + padding;
	const maxY = Math.max(...ys.map((y, i) => y + (radii[i] ?? 0)), 1) + padding;
	const width = Math.max(1, maxX - minX);
	const height = Math.max(1, maxY - minY);
	const nodeById = new Map(fixture.nodes.map((node) => [node.id, node]));
	const edges = fixture.edges
		.map((edge) => {
			const source = nodeById.get(edge.source);
			const target = nodeById.get(edge.target);
			if (!source || !target) return "";
			return `<line x1="${source.x - minX}" y1="${source.y - minY}" x2="${target.x - minX}" y2="${target.y - minY}" stroke="#666" stroke-width="2"/>`;
		})
		.join("");
	const nodes = fixture.nodes
		.map((node) => `<circle cx="${node.x - minX}" cy="${node.y - minY}" r="${node.radius}" fill="#4b7bec" fill-opacity="0.25" stroke="#2d5aa8"/><text x="${node.x - minX + node.radius + 4}" y="${node.y - minY + 4}" font-size="12">${(node.text ?? node.id).replace(/[<>&]/g, "")}</text>`)
		.join("");
	return `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${width} ${height}" width="${width}" height="${height}">${edges}${nodes}</svg>`;
}

/** @emoji 💾 Registers puzzle 2d fixture SVG/PNG export handlers for the OS media graph. */
export function registerPuzzle2dMediaExportHandlers(): void {
	registerOsMediaExportHandler("2d.puzzle", "svg", async (doc) => ({
		data: puzzle2dFixtureToSvg(doc as Puzzle2dFixture),
		mimeType: "image/svg+xml",
		fileName: "puzzle2d.svg",
	}));
	registerOsMediaExportHandler("2d.puzzle", "png", async (doc) => {
		const svg = puzzle2dFixtureToSvg(doc as Puzzle2dFixture);
		const width = Number(svg.match(/width="(\d+)"/)?.[1] ?? 1024);
		const height = Number(svg.match(/height="(\d+)"/)?.[1] ?? 768);
		const dataUrl = await rasterizeSvgMarkupToPngDataUrl(svg, width, height);
		const blob = await fetch(dataUrl).then((response) => response.blob());
		return { data: new Uint8Array(await blob.arrayBuffer()), mimeType: "image/png", fileName: "puzzle2d.png" };
	});
}
//#endregion 🔖MediaExport

/** @emoji 🛝 Puzzle 2d play harness as a single {@link Playground} instance. */


export const puzzle2dPlayAppDefinition = createPlaygroundApp({
	id: PUZZLE_2D_PLAY_APP_ID,
	label: "Puzzle 2D",
	controllerId: PUZZLE_2D_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	devHost: {
		playEntryKind: "2d",
		resolveDedupe: ["react", "react-dom", "three"],
		watchIgnored: ["../rs/lib.rs", "../rs/target/**", "../rs/Cargo.toml", "../rs/Cargo.lock", "../rs/script.ts"],
		optimizeDeps: { include: ["react", "react-dom", "react/jsx-runtime", "react/jsx-dev-runtime", "three", "@react-three/fiber", "@react-three/drei", "lucide-react", "@semio-tech/infinite-cavas-react-renderer"] },
	},
	createRuntime: () => {
		const runtime = new Platform({ id: PUZZLE_2D_PLAY_APP_ID });
			const ctrl = new Puzzle2dPlayShellController(runtime.commandBus, () => runtime.notify(), () => runtime.notifyChrome());
			runtime.addApp(buildPuzzle2dPlayAppRuntime(ctrl));
			return runtime;
	},
	registerBodies: () => {
		registerPuzzle2dPlayDeclarativeBodies();
	},
	keybindings: [
		{ key: "ctrl+a,meta+a", controllerId: PUZZLE_2D_PLAY_CONTROLLER_ID, command: "selectAllSelection" },
	],
	bootRenderer: async (pg) => {
		const { boot2dPlay } = await import("@semio-tech/framework-playground-renderer-react/puzzle/2d");
		boot2dPlay(pg);
	},
});
//#endregion 🔖Play
