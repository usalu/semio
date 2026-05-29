// #region 🧲Header
/** @emoji 🔗 `@puzzle/5d/react` — paired board + scene surfaces + play harness (monolith). */
// #endregion 🧲Header

// #region 🔌Adapters
import { reactHostPort, type ContextMenuItem } from "@ui/react";
import type { ReactElement } from "react";

/** ┬¡ãÆ├Â┬í Topology pairs board WASM with scene R3F under one dual-surface contract; `@puzzle/2d/react` and `@puzzle/3d/react` stay independently testable via their own play apps or hand-built `TopologyDualSurfaceBindings`. */

import {
	boardFixtureMetaKindCatalogBundle,
	parseBoardFixtureV1,
	DEFAULT_BOARD_GRID_FACTOR,
	DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS,
	type BoardCanvasProps,
	type BoardFixtureV1,
	type BoardForceGraphLayoutOptions,
	type BoardKindCatalogBundle,
	type BoardKindCompatEntry,
	type CameraState as BoardCameraState,
} from "@puzzle/2d/react";
import { BoardCanvas, Edge, Handle, Node, Wire } from "@puzzle/2d/react";
import {
	Canvas3D as Scene,
	SceneAttractions,
	SceneObjectStateProvider,
	SceneObjects,
	parseFixtureV1,
	blockedVortexFullIdsFromAttractions,
	useSceneObjectConnect,
	useSceneObjectRelocate,
	type CanvasProps as SceneCanvasProps,
	type FixtureV1 as SceneFixtureV1,
	type KindCatalogBundle as SceneKindCatalogBundle,
	type KindCompatEntry as SceneKindCompatEntry,
	type RelocateMode as SceneRelocateMode,
} from "../../3d/react/index.tsx";
// #endregion 🔌Adapters

//#region ┬¡ãÆ├Â├╗TopologyFixture
/** @emoji ┬¡ãÆ├┤├ñ Parsed `elements.topology.fixture/v1` manifest (paired board+scene payloads are loaded separately in hosts). */
export interface TopologyFixtureV1 {
	readonly schema: "elements.topology.fixture/v1";
	readonly label?: string;
	readonly meta?: Record<string, unknown>;
}

/** @emoji ┬¡ãÆ┬║┬Ñ Validates topology fixture JSON. */
export function parseTopologyFixtureV1(raw: unknown): TopologyFixtureV1 | null {
	if (!raw || typeof raw !== "object") return null;
	const r = raw as Record<string, unknown>;
	if (r.schema !== "elements.topology.fixture/v1") return null;
	return {
		schema: "elements.topology.fixture/v1",
		...(typeof r.label === "string" ? { label: r.label } : {}),
		...(r.meta && typeof r.meta === "object" ? { meta: r.meta as Record<string, unknown> } : {}),
	};
}
//#endregion ┬¡ãÆ├Â├╗TopologyFixture

//#region ┬¡ãÆ├Â├╣SharedBindings
/** @emoji 🔗 LOD + grid fields shared by board WASM (thresholds) and scene (grid only). */
export type TopologyLodGridShared = Pick<BoardCanvasProps, "lodZoomThresholds" | "gridFactor" | "gridSnapEnabled">;

/** @emoji ┬¡ãÆ┬║├Ç Parallel link/selection/camera hooks for board vs scene (payload kinds differ per surface). */
export interface TopologyDualSurfaceBindingInput extends TopologyLodGridShared {
	readonly kindCatalogs?: BoardKindCatalogBundle;
	readonly kindCompatibility?: readonly BoardKindCompatEntry[];
	readonly onBoardConnect?: BoardCanvasProps["onConnect"];
	readonly onSceneConnect?: SceneCanvasProps["onConnect"];
	readonly onBoardIndirectConnect?: BoardCanvasProps["onIndirectConnect"];
	readonly onSceneIndirectConnect?: SceneCanvasProps["onIndirectConnect"];
	readonly onBoardProximityConnect?: BoardCanvasProps["onProximityConnect"];
	readonly onSceneProximityConnect?: SceneCanvasProps["onProximityConnect"];
	readonly onBoardLinkCompatibleNodes?: BoardCanvasProps["onLinkCompatibleNodes"];
	readonly onSceneAttractionCompatibleObjects?: SceneCanvasProps["onAttractionCompatibleObjects"];
	readonly onBoardLinkTargetRing?: BoardCanvasProps["onLinkTargetRing"];
	readonly onSceneAttractionTargetRing?: SceneCanvasProps["onAttractionTargetRing"];
	readonly onBoardSelect?: BoardCanvasProps["onSelect"];
	readonly onSceneSelect?: SceneCanvasProps["onSelect"];
	readonly onBoardCamera?: BoardCanvasProps["onCamera"];
	readonly onSceneCamera?: SceneCanvasProps["onCamera"];
	readonly onBoardHover?: BoardCanvasProps["onHover"];
	readonly onSceneLodChange?: SceneCanvasProps["onLodChange"];
}

export interface TopologyDualSurfaceBindings {
	readonly board: Pick<
		BoardCanvasProps,
		| "lodZoomThresholds"
		| "gridFactor"
		| "gridSnapEnabled"
		| "kindCatalogs"
		| "kindCompatibility"
		| "onConnect"
		| "onIndirectConnect"
		| "onProximityConnect"
		| "onLinkCompatibleNodes"
		| "onLinkTargetRing"
		| "onSelect"
		| "onCamera"
		| "onHover"
	>;
	readonly scene: Pick<
		SceneCanvasProps,
		| "gridFactor"
		| "gridSnapEnabled"
		| "automaticLod"
		| "depthVariableLod"
		| "lod"
		| "kindCatalogs"
		| "kindCompatibility"
		| "onConnect"
		| "onIndirectConnect"
		| "onProximityConnect"
		| "onAttractionCompatibleObjects"
		| "onAttractionTargetRing"
		| "onSelect"
		| "onCamera"
		| "onLodChange"
	>;
}

/** @emoji ┬¡ãÆ┬║├Ç Splits shared LOD/grid + catalog rows into board and scene canvas prop slices (scene catalogs are structurally aligned JSON). */
export function buildTopologyDualSurfaceBindings(input: TopologyDualSurfaceBindingInput): TopologyDualSurfaceBindings {
	const sceneCatalogs = input.kindCatalogs as SceneKindCatalogBundle | undefined;
	const sceneCompat = input.kindCompatibility as readonly SceneKindCompatEntry[] | undefined;
	return {
		board: {
			lodZoomThresholds: input.lodZoomThresholds,
			gridFactor: input.gridFactor,
			gridSnapEnabled: input.gridSnapEnabled,
			kindCatalogs: input.kindCatalogs,
			kindCompatibility: input.kindCompatibility,
			onConnect: input.onBoardConnect,
			onIndirectConnect: input.onBoardIndirectConnect,
			onProximityConnect: input.onBoardProximityConnect,
			onLinkCompatibleNodes: input.onBoardLinkCompatibleNodes,
			onLinkTargetRing: input.onBoardLinkTargetRing,
			onSelect: input.onBoardSelect,
			onCamera: input.onBoardCamera,
			onHover: input.onBoardHover,
		},
		scene: {
			gridFactor: input.gridFactor,
			gridSnapEnabled: input.gridSnapEnabled,
			kindCatalogs: sceneCatalogs,
			kindCompatibility: sceneCompat,
			onConnect: input.onSceneConnect,
			onIndirectConnect: input.onSceneIndirectConnect,
			onProximityConnect: input.onSceneProximityConnect,
			onAttractionCompatibleObjects: input.onSceneAttractionCompatibleObjects,
			onAttractionTargetRing: input.onSceneAttractionTargetRing,
			onSelect: input.onSceneSelect,
			onCamera: input.onSceneCamera as SceneCanvasProps["onCamera"],
			onLodChange: input.onSceneLodChange,
		},
	};
}

/** @emoji ┬¡ãÆ├Â├╣ Mirrors one logical link callback onto both surfaces with a discriminant. */
export function topologyMirrorConnectHandlers(onBoth: (p: {
	readonly source: string;
	readonly target: string;
	readonly surface: "board" | "scene";
}) => void): Pick<TopologyDualSurfaceBindingInput, "onBoardConnect" | "onSceneConnect"> {
	return {
		onBoardConnect: (payload) => onBoth({ source: payload.source, target: payload.target, surface: "board" }),
		onSceneConnect: (payload) =>
			onBoth({ source: payload.attracting, target: payload.attracted, surface: "scene" }),
	};
}

/** @emoji ┬¡ãÆ├Â├╣ Mirrors proximity-connect telemetry across both surfaces (board WASM vs scene vortex pick). */
export function topologyMirrorProximityHandlers(onBoth: (p: { readonly surface: "board" | "scene" }) => void): Pick<
	TopologyDualSurfaceBindingInput,
	"onBoardProximityConnect" | "onSceneProximityConnect"
> {
	return {
		onBoardProximityConnect: () => onBoth({ surface: "board" }),
		onSceneProximityConnect: () => onBoth({ surface: "scene" }),
	};
}
//#endregion ┬¡ãÆ├Â├╣SharedBindings

//#region ┬¡ãÆ├▓┬®┬┤┬®├àBoardLayout
/** @emoji ┬¡ãÆ├Â├╣ Default separator for board handle ids (`piece::connector`). */
export const TOPOLOGY_BOARD_HANDLE_ID_SEPARATOR = "::";

/** @emoji ┬¡ãÆ├Â├╣ Builds a compound board handle id from two parts. */
export function topologyBoardCompoundId(left: string, right: string, separator: string = TOPOLOGY_BOARD_HANDLE_ID_SEPARATOR): string {
	return `${left}${separator}${right}`;
}

/** @emoji ┬¡ãÆ├Â├¼ Parses a compound board handle id into left/right parts. */
export function topologyParseBoardCompoundId(
	value: string,
	separator: string = TOPOLOGY_BOARD_HANDLE_ID_SEPARATOR,
): { left: string; right: string } | null {
	const separatorIndex = value.indexOf(separator);
	if (separatorIndex <= 0 || separatorIndex >= value.length - separator.length) return null;
	return {
		left: value.slice(0, separatorIndex),
		right: value.slice(separatorIndex + separator.length),
	};
}

/** @emoji ┬¡ãÆ├┤├ë Evenly distributes connector angles around a node rim (starts at top). */
export function topologyBoardConnectorAngle(index: number, total: number): number {
	return -Math.PI / 2 + (index * Math.PI * 2) / Math.max(total, 1);
}

export type TopologyKitBoardSide = "top" | "right" | "bottom" | "left";

/** @emoji ┬¡ãÆ├┤├ë Kit diagram snap side to board handle angle (rectangle vs circle rim). */
export function topologyKitBoardHandleAngle(side: TopologyKitBoardSide, shape: "circle" | "rectangle"): number {
	if (shape === "rectangle") {
		if (side === "top") return 0;
		if (side === "right") return Math.PI / 2;
		if (side === "bottom") return Math.PI;
		return (3 * Math.PI) / 2;
	}
	if (side === "right") return 0;
	if (side === "bottom") return Math.PI / 2;
	if (side === "left") return Math.PI;
	return -Math.PI / 2;
}

/** @emoji ┬¡ãÆ├┤├¼ Node center from top-left layout position and frame size. */
export function topologyBoardCenterFromTopLeft(
	position: { readonly x: number; readonly y: number },
	frame: { readonly width: number; readonly height: number },
): { x: number; y: number } {
	return { x: position.x + frame.width / 2, y: position.y + frame.height / 2 };
}

/** @emoji ┬¡ãÆ├▓┬®┬┤┬®├à Diagram force-slider weights shared by sketchpad kit/design hosts. */
export interface TopologyDiagramForceWeights {
	readonly centerStrength: number;
	readonly linkDistance: number;
	readonly chargeStrength: number;
}

/** @emoji ┬¡ãÆ├▓┬®┬┤┬®├à Maps diagram force sliders to {@link layoutBoardFixtureForceGraph} options. */
export function topologyDiagramForceGraphOptions(weights: TopologyDiagramForceWeights): BoardForceGraphLayoutOptions {
	return {
		centerX: 0,
		centerY: 0,
		gravity: weights.centerStrength,
		idealEdgeLength: weights.linkDistance,
		iterations: 280,
		randomSeed: 1,
		repulsionStrength: Math.abs(weights.chargeStrength),
	};
}

/** @emoji ┬¡ãÆ├┤├Ç Centers the board camera on the average of node centers. */
export function topologyBoardCameraFromCenters(centers: readonly { x: number; y: number }[]): BoardCameraState {
	if (centers.length === 0) return { x: 0, y: 0, zoom: 1 };
	const avgX = centers.reduce((sum, point) => sum + point.x, 0) / centers.length;
	const avgY = centers.reduce((sum, point) => sum + point.y, 0) / centers.length;
	return { x: -avgX, y: -avgY, zoom: 1 };
}

/** @emoji ┬¡ãÆ├┤├¼ Writes WASM layout node centers back into top-left layout positions. */
export function topologyApplyBoardFixtureCentersToTopLeft<T extends { readonly id: string; readonly position: { x: number; y: number } }>(
	items: readonly T[],
	fixture: BoardFixtureV1,
	frameForItem: (item: T) => { width: number; height: number },
): T[] {
	const centerById = new Map(fixture.nodes.map((node) => [node.id, { x: node.x, y: node.y }]));
	return items.map((item) => {
		const center = centerById.get(item.id);
		if (!center) return item;
		const frame = frameForItem(item);
		return {
			...item,
			position: { x: center.x - frame.width / 2, y: center.y - frame.height / 2 },
		};
	});
}
//#endregion ┬¡ãÆ├▓┬®┬┤┬®├àBoardLayout

//#region ┬¡ãÆ┬║┬ÑPairedMeta
function isTopologyMetaRecord(value: unknown): value is Record<string, unknown> {
	return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}

/** @emoji ┬¡ãÆ┬║┬Ñ Reads `kindCompatibility` rows from one fixture meta blob (shared row shape across board + scene JSON). */
export function topologyKindCompatibilityRowsFromMeta(meta: Record<string, unknown> | undefined): BoardKindCompatEntry[] {
	if (!isTopologyMetaRecord(meta)) return [];
	const arr = meta.kindCompatibility;
	if (!Array.isArray(arr)) return [];
	const out: BoardKindCompatEntry[] = [];
	for (const entry of arr) {
		if (!isTopologyMetaRecord(entry)) continue;
		const source = typeof entry.source === "string" ? entry.source.trim() : "";
		const target = typeof entry.target === "string" ? entry.target.trim() : "";
		if (!source || !target) continue;
		const specificity =
			entry.specificity === "general" ||
			entry.specificity === "node" ||
			entry.specificity === "edge" ||
			entry.specificity === "handle" ||
			entry.specificity === "wire" ||
			entry.specificity === "object" ||
			entry.specificity === "attraction"
				? entry.specificity
				: undefined;
		out.push({
			source,
			target,
			...(entry.bidirectional === true ? { bidirectional: true } : {}),
			...(entry.important === true ? { important: true } : {}),
			...(specificity ? { specificity } : {}),
		});
	}
	return out;
}

/** @emoji ┬¡ãÆ┬║┬Ñ Reads `kindCatalogs` object from one fixture meta (scene JSON shape; cast at dual-surface boundary). */
export function topologyKindCatalogBundleFromSceneMeta(meta: Record<string, unknown> | undefined): SceneKindCatalogBundle | undefined {
	if (!isTopologyMetaRecord(meta)) return undefined;
	const kc = meta.kindCatalogs;
	if (!kc || typeof kc !== "object" || Array.isArray(kc)) return undefined;
	return kc as SceneKindCatalogBundle;
}

/** @emoji ┬¡ãÆ┬║┬Ñ Picks a single catalog bundle for dual surfaces: board fixture meta wins, else scene meta. */
export function topologyPairedKindCatalogBundle(inp: {
	readonly boardMeta: Record<string, unknown> | undefined;
	readonly sceneMeta: Record<string, unknown> | undefined;
}): BoardKindCatalogBundle | undefined {
	const fromBoard = boardFixtureMetaKindCatalogBundle(inp.boardMeta);
	if (fromBoard) return fromBoard;
	return topologyKindCatalogBundleFromSceneMeta(inp.sceneMeta) as BoardKindCatalogBundle | undefined;
}

/** @emoji ┬¡ãÆ┬║┬Ñ Picks compatibility rows for dual surfaces: board meta wins when non-empty, else scene meta. */
export function topologyPairedKindCompatibility(inp: {
	readonly boardMeta: Record<string, unknown> | undefined;
	readonly sceneMeta: Record<string, unknown> | undefined;
}): readonly BoardKindCompatEntry[] {
	const fromBoard = topologyKindCompatibilityRowsFromMeta(inp.boardMeta);
	if (fromBoard.length > 0) return fromBoard;
	return topologyKindCompatibilityRowsFromMeta(inp.sceneMeta);
}
//#endregion ┬¡ãÆ┬║┬ÑPairedMeta

//#region ┬¡ãÆ├ä├£┬┤┬®├àPairDefaults
/** @emoji ┬¡ãÆ├ä├£┬┤┬®├à Root layout class shared by `TopologyBoardPane` / `TopologyScenePane` shells (pair-aligned chrome). */
export const TOPOLOGY_PANE_ROOT_CLASS = "flex h-full min-h-0 flex-1 flex-col";

/** @emoji ┬¡ãÆ├ä├£┬┤┬®├à LOD + grid defaults shared by paired board/scene canvases (same knobs as standalone plays). */
export const TOPOLOGY_LOD_GRID_DEFAULTS: TopologyLodGridShared = {
	lodZoomThresholds: DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS,
	gridFactor: DEFAULT_BOARD_GRID_FACTOR,
	gridSnapEnabled: true,
};

/** @emoji ┬¡ãÆ├ä├£┬┤┬®├à Scene-only chrome defaults aligned with the scene play harness (stable reference for memoized panes). */
export const TOPOLOGY_SCENE_CHROME_DEFAULTS: Pick<
	SceneCanvasProps,
	"showLodGrid" | "proximityRadius" | "gridSnapEnabled"
> = { showLodGrid: true, proximityRadius: 24, gridSnapEnabled: true };

/** @deprecated Use {@link TOPOLOGY_SCENE_CHROME_DEFAULTS} ├ö├ç├Â kept for callers that still invoke a factory. */
export function topologySceneChromeDefaults(): Pick<SceneCanvasProps, "showLodGrid" | "proximityRadius" | "gridSnapEnabled"> {
	return TOPOLOGY_SCENE_CHROME_DEFAULTS;
}

/** @emoji ┬¡ãÆ├ä├£┬┤┬®├à Merges paired fixture metas into the shared LOD/grid + catalog fields consumed by `buildTopologyDualSurfaceBindings`. */
export function topologySharedKindsFromPairedMetas(inp: {
	readonly boardMeta: Record<string, unknown> | undefined;
	readonly sceneMeta: Record<string, unknown> | undefined;
}): Pick<
	TopologyDualSurfaceBindingInput,
	"lodZoomThresholds" | "gridFactor" | "gridSnapEnabled" | "kindCatalogs" | "kindCompatibility"
> {
	return {
		...TOPOLOGY_LOD_GRID_DEFAULTS,
		kindCatalogs: topologyPairedKindCatalogBundle(inp),
		kindCompatibility: topologyPairedKindCompatibility(inp),
	};
}
//#endregion ┬¡ãÆ├ä├£┬┤┬®├àPairDefaults

//#region ┬¡ãÆ├╣ÔòØBoardMarkers
export interface TopologyBoardWireRecord {
	readonly id: string;
	readonly source: string;
	readonly target?: string;
	readonly wireKind?: string;
	readonly endX?: number;
	readonly endY?: number;
	readonly hidden?: boolean;
}

/** @emoji ┬¡ãÆ├╣ÔòØ Builds a Fragment of board host markers from a board fixture (same static shape walk as board play). */
export function topologyBoardMarkersFromFixture(props: {
	readonly fixture: BoardFixtureV1;
	readonly lockedIds: ReadonlySet<string>;
	readonly selectedIds: ReadonlySet<string>;
	readonly contextMenuById: (id: string | null) => ContextMenuItem[];
	readonly wires: readonly TopologyBoardWireRecord[];
}): ReactElement {
	const { contextMenuById, fixture, lockedIds, selectedIds, wires } = props;
	return (
		<>
			{fixture.nodes.map((node) =>
				node.shape === "rectangle" ? (
					<Node
						contextMenu={contextMenuById(node.id)}
						draggable={!lockedIds.has(node.id)}
						height={node.height}
						id={node.id}
						key={node.id}
						{...(node.hidden === true ? { hidden: true } : {})}
						{...(node.nodeKind !== undefined ? { nodeKind: node.nodeKind } : {})}
						shape="rectangle"
						selected={selectedIds.has(node.id)}
						text={node.text}
						textAlignment={node.textAlignment}
						textAutofit={node.textAutofit === true}
						textFontFamily={node.textFontFamily}
						textFontSize={node.textFontSize}
						width={node.width}
						x={node.x}
						y={node.y}
						{...(node.iconKind ? { iconKind: node.iconKind } : {})}
					>
						{node.handles.map((handle) => (
							<Handle
								angle={handle.angle}
								color={handle.color}
								contextMenu={contextMenuById(handle.id)}
								handleKind={handle.handleKind}
								{...(handle.hidden === true ? { hidden: true } : {})}
								id={handle.id}
								key={handle.id}
								radius={handle.radius}
								selected={selectedIds.has(handle.id)}
								{...(handle.iconKind ? { iconKind: handle.iconKind } : {})}
							/>
						))}
					</Node>
				) : (
					<Node
						contextMenu={contextMenuById(node.id)}
						draggable={!lockedIds.has(node.id)}
						id={node.id}
						key={node.id}
						{...(node.hidden === true ? { hidden: true } : {})}
						{...(node.nodeKind !== undefined ? { nodeKind: node.nodeKind } : {})}
						radius={node.radius}
						selected={selectedIds.has(node.id)}
						text={node.text}
						textAlignment={node.textAlignment}
						textAutofit={node.textAutofit === true}
						textFontFamily={node.textFontFamily}
						textFontSize={node.textFontSize}
						x={node.x}
						y={node.y}
						{...(node.iconKind ? { iconKind: node.iconKind } : {})}
					>
						{node.handles.map((handle) => (
							<Handle
								angle={handle.angle}
								color={handle.color}
								contextMenu={contextMenuById(handle.id)}
								handleKind={handle.handleKind}
								{...(handle.hidden === true ? { hidden: true } : {})}
								id={handle.id}
								key={handle.id}
								radius={handle.radius}
								selected={selectedIds.has(handle.id)}
								{...(handle.iconKind ? { iconKind: handle.iconKind } : {})}
							/>
						))}
					</Node>
				),
			)}
			{fixture.edges.map((edge) => (
				<Edge
					contextMenu={contextMenuById(edge.id)}
					edgeKind={edge.edgeKind}
					{...(edge.hidden === true ? { hidden: true } : {})}
					id={edge.id}
					key={edge.id}
					selected={selectedIds.has(edge.id)}
					source={edge.source}
					target={edge.target}
				/>
			))}
			{wires.map((wire) => (
				<Wire
					contextMenu={contextMenuById(wire.id)}
					{...(typeof wire.endX === "number" ? { endX: wire.endX } : {})}
					{...(typeof wire.endY === "number" ? { endY: wire.endY } : {})}
					{...(wire.hidden === true ? { hidden: true } : {})}
					id={wire.id}
					key={wire.id}
					selected={selectedIds.has(wire.id)}
					source={wire.source}
					{...(wire.target ? { target: wire.target } : {})}
					{...(wire.wireKind ? { wireKind: wire.wireKind } : {})}
				/>
			))}
		</>
	);
}
//#endregion ┬¡ãÆ├╣ÔòØBoardMarkers

//#region ┬¡ãÆ┬¼ãÆPanes
//#region ┬¡ãÆ┬¼ãÆPanesBoard
export interface TopologyBoardPaneProps {
	readonly fixture: BoardFixtureV1;
	readonly bindings: TopologyDualSurfaceBindings;
	readonly selectedIds: ReadonlySet<string>;
	readonly lockedIds?: ReadonlySet<string>;
	readonly wires?: readonly TopologyBoardWireRecord[];
	readonly board?: Omit<BoardCanvasProps, "children">;
}

export const TopologyBoardPane = reactHostPort.memo(function TopologyBoardPane(props: TopologyBoardPaneProps) {
	const lockedIds = props.lockedIds ?? new Set<string>();
	const markers = reactHostPort.useMemo(
		() =>
			topologyBoardMarkersFromFixture({
				fixture: props.fixture,
				lockedIds,
				selectedIds: props.selectedIds,
				contextMenuById: () => [],
				wires: props.wires ?? [],
			}),
		[props.fixture, lockedIds, props.selectedIds, props.wires],
	);
	const { board: b } = props.bindings;
	const boardExtra = props.board ?? {};
	const mergedCatalogs =
		boardExtra.kindCatalogs ?? b.kindCatalogs ?? boardFixtureMetaKindCatalogBundle(props.fixture.meta);
	return (
		<div className={TOPOLOGY_PANE_ROOT_CLASS} data-topology-board-root data-topology-surface="board">
			<BoardCanvas
				camera={boardExtra.camera ?? props.fixture.camera}
				className="min-h-0 flex-1"
				{...b}
				{...boardExtra}
				{...(mergedCatalogs ? { kindCatalogs: mergedCatalogs } : {})}
			>
				{markers}
			</BoardCanvas>
		</div>
	);
});
//#endregion ┬¡ãÆ┬¼ãÆPanesBoard

//#region ┬¡ãÆ┬¼ãÆPanesScene
export interface TopologyScenePaneProps {
	readonly fixture: SceneFixtureV1;
	readonly bindings: TopologyDualSurfaceBindings;
	readonly relocateMode: SceneRelocateMode;
	readonly selectedObjectId: string | null;
	readonly scene?: Omit<SceneCanvasProps, "children">;
	readonly blockedVortexFullIds?: ReadonlySet<string>;
}

const TopologySceneCanvas = reactHostPort.memo(function TopologySceneCanvas(
	props: TopologyScenePaneProps & { readonly blocked: ReadonlySet<string> },
) {
	const onRelocate = useSceneObjectRelocate();
	const onConnect = useSceneObjectConnect();
	const { scene: s } = props.bindings;
	const { camera: liveCamera, onRelocate: _externalRelocate, ...sceneRest } = props.scene ?? {};
	return (
		<Scene
			className="min-h-0 flex-1"
			camera={liveCamera ?? props.fixture.camera}
			blockedVortexFullIds={props.blocked}
			{...s}
			{...sceneRest}
			relocateMode={props.relocateMode}
			onRelocate={onRelocate}
			onConnect={onConnect}
		>
			<SceneObjects selectedObjectId={props.selectedObjectId} relocate={props.relocateMode} />
			<SceneAttractions />
		</Scene>
	);
});

export const TopologyScenePane = reactHostPort.memo(function TopologyScenePane(props: TopologyScenePaneProps) {
	const blocked = props.blockedVortexFullIds ?? blockedVortexFullIdsFromAttractions(props.fixture.attractions);
	return (
		<div className={TOPOLOGY_PANE_ROOT_CLASS} data-topology-scene-root data-topology-surface="scene">
			<reactHostPort.Suspense fallback={<div className="flex min-h-0 flex-1 items-center justify-center p-4 text-sm text-muted-foreground">Loading meshes├ö├ç┬¬</div>}>
				<SceneObjectStateProvider
					fixture={props.fixture}
					onConnect={props.bindings.scene.onConnect}
					onRelocate={props.scene?.onRelocate}
				>
					<TopologySceneCanvas {...props} blocked={blocked} />
				</SceneObjectStateProvider>
			</reactHostPort.Suspense>
		</div>
	);
});
//#endregion ┬¡ãÆ┬¼ãÆPanesScene
//#endregion ┬¡ãÆ┬¼ãÆPanes

export { parseBoardFixtureV1, parseFixtureV1, blockedVortexFullIdsFromAttractions };
export { DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS, DEFAULT_BOARD_GRID_FACTOR };

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
	describe("parseTopologyFixtureV1", () => {
		it("accepts manifest", () => {
			const t = parseTopologyFixtureV1({
				schema: "elements.topology.fixture/v1",
				label: "x",
			});
			expect(t?.schema).toBe("elements.topology.fixture/v1");
			expect(t?.label).toBe("x");
		});
	});
	describe("buildTopologyDualSurfaceBindings", () => {
		it("forwards lod keys to both slices", () => {
			const b = buildTopologyDualSurfaceBindings({
				...TOPOLOGY_LOD_GRID_DEFAULTS,
			});
			expect(b.board.gridSnapEnabled).toBe(true);
			expect(b.scene.gridSnapEnabled).toBe(true);
		});
	});
	describe("topologyPairedKindCompatibility", () => {
		it("prefers board meta rows when present", () => {
			const rows = topologyPairedKindCompatibility({
				boardMeta: { kindCompatibility: [{ source: "a", target: "b" }] },
				sceneMeta: { kindCompatibility: [{ source: "x", target: "y" }] },
			});
			expect(rows.some((r) => r.source === "a")).toBe(true);
		});
		it("falls back to scene meta when board has no rows", () => {
			const rows = topologyPairedKindCompatibility({
				boardMeta: {},
				sceneMeta: { kindCompatibility: [{ source: "x", target: "y" }] },
			});
			expect(rows.some((r) => r.source === "x")).toBe(true);
		});
	});
	describe("topologyMirrorProximityHandlers", () => {
		it("invokes both surfaces", () => {
			const seen: string[] = [];
			const m = topologyMirrorProximityHandlers((p) => {
				seen.push(p.surface);
			});
			m.onBoardProximityConnect?.({ source: "a", target: "b" } as never);
			m.onSceneProximityConnect?.({ source: "a", target: "b" } as never);
			expect(seen).toEqual(["board", "scene"]);
		});
	});
	describe("topologySharedKindsFromPairedMetas", () => {
		it("includes lod defaults", () => {
			const s = topologySharedKindsFromPairedMetas({ boardMeta: undefined, sceneMeta: undefined });
			expect(s.gridSnapEnabled).toBe(true);
		});
	});
	describe("topologyBoardCompoundId", () => {
		it("round-trips handle ids", () => {
			const id = topologyBoardCompoundId("piece-a", "conn-b");
			expect(topologyParseBoardCompoundId(id)).toEqual({ left: "piece-a", right: "conn-b" });
		});
	});
	describe("topologyKitBoardHandleAngle", () => {
		it("maps rectangle sides to axis angles", () => {
			expect(topologyKitBoardHandleAngle("top", "rectangle")).toBe(0);
			expect(topologyKitBoardHandleAngle("right", "rectangle")).toBeCloseTo(Math.PI / 2);
		});
	});
	describe("topologyBoardCenterFromTopLeft", () => {
		it("offsets by half frame", () => {
			expect(topologyBoardCenterFromTopLeft({ x: 10, y: 20 }, { width: 40, height: 60 })).toEqual({ x: 30, y: 50 });
		});
	});
	describe("topologyApplyBoardFixtureCentersToTopLeft", () => {
		it("converts centers to top-left using frame size", () => {
			const fixture: BoardFixtureV1 = {
				schema: "elements.board.fixture/v1",
				camera: { x: 0, y: 0, zoom: 1 },
				nodes: [{ id: "n1", shape: "rectangle", width: 40, height: 20, x: 50, y: 30, handles: [] }],
				edges: [],
			};
			const next = topologyApplyBoardFixtureCentersToTopLeft(
				[{ id: "n1", position: { x: 0, y: 0 } }],
				fixture,
				() => ({ width: 40, height: 20 }),
			);
			expect(next[0]?.position).toEqual({ x: 30, y: 20 });
		});
	});
	describe("topologyDiagramForceGraphOptions", () => {
		it("maps charge strength to repulsion", () => {
			const o = topologyDiagramForceGraphOptions({ centerStrength: 0.1, linkDistance: 120, chargeStrength: -400 });
			expect(o.repulsionStrength).toBe(400);
			expect(o.idealEdgeLength).toBe(120);
		});
	});
}
