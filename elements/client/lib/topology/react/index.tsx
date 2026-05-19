import type { ContextMenuItem } from "@elements/ui";
import { Suspense, memo, useMemo, type ReactElement } from "react";

/** 🔭 Topology pairs board WASM with scene R3F under one dual-surface contract; `@elements/board` and `@elements/scene` stay independently testable via their own play apps or hand-built `TopologyDualSurfaceBindings`. */

import {
	boardFixtureMetaKindCatalogBundle,
	parseBoardFixtureV1,
	DEFAULT_BOARD_GRID_FACTOR,
	DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS,
	type BoardCanvasProps,
	type BoardFixtureV1,
	type BoardKindCatalogBundle,
	type BoardKindCompatEntry,
} from "../../board/index.ts";
import { BoardCanvas, Edge, Handle, Node, Wire } from "../../board/index.tsx";
import {
	Canvas3D as Scene,
	ObjectItem as SceneObject,
	Tie as SceneTie,
	Vortex as SceneVortex,
	parseFixtureV1,
	blockedVortexFullIdsFromTies,
	type CanvasProps as SceneCanvasProps,
	type FixtureV1 as SceneFixtureV1,
	type KindCatalogBundle as SceneKindCatalogBundle,
	type KindCompatEntry as SceneKindCompatEntry,
	type LodZoomThresholds as SceneLodZoomThresholds,
	type RelocateMode as SceneRelocateMode,
} from "../../scene/index.tsx";

//#region 🔖TopologyFixture
/** @emoji 📄 Parsed `elements.topology.fixture/v1` manifest (paired board+scene payloads are loaded separately in hosts). */
export interface TopologyFixtureV1 {
	readonly schema: "elements.topology.fixture/v1";
	readonly label?: string;
	readonly meta?: Record<string, unknown>;
}

/** @emoji 🧾 Validates topology fixture JSON. */
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
//#endregion 🔖TopologyFixture

//#region 🔗SharedBindings
/** @emoji 🧷 LOD + grid fields shared by board WASM and scene orbit pseudo-zoom. */
export type TopologyLodGridShared = Pick<BoardCanvasProps, "lodZoomThresholds" | "gridFactor" | "gridSnapEnabled">;

/** @emoji 🧷 Parallel link/selection/camera hooks for board vs scene (payload kinds differ per surface). */
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
		| "lodZoomThresholds"
		| "gridFactor"
		| "gridSnapEnabled"
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

/** @emoji 🧷 Splits shared LOD/grid + catalog rows into board and scene canvas prop slices (scene catalogs are structurally aligned JSON). */
export function buildTopologyDualSurfaceBindings(input: TopologyDualSurfaceBindingInput): TopologyDualSurfaceBindings {
	const sceneLod = input.lodZoomThresholds as SceneLodZoomThresholds | undefined;
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
			lodZoomThresholds: sceneLod,
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

/** @emoji 🔗 Mirrors one logical link callback onto both surfaces with a discriminant. */
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

/** @emoji 🔗 Mirrors proximity-connect telemetry across both surfaces (board WASM vs scene vortex pick). */
export function topologyMirrorProximityHandlers(onBoth: (p: { readonly surface: "board" | "scene" }) => void): Pick<
	TopologyDualSurfaceBindingInput,
	"onBoardProximityConnect" | "onSceneProximityConnect"
> {
	return {
		onBoardProximityConnect: () => onBoth({ surface: "board" }),
		onSceneProximityConnect: () => onBoth({ surface: "scene" }),
	};
}
//#endregion 🔗SharedBindings

//#region 🧾PairedMeta
function isTopologyMetaRecord(value: unknown): value is Record<string, unknown> {
	return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}

/** @emoji 🧾 Reads `kindCompatibility` rows from one fixture meta blob (shared row shape across board + scene JSON). */
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
			entry.specificity === "tie"
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

/** @emoji 🧾 Reads `kindCatalogs` object from one fixture meta (scene JSON shape; cast at dual-surface boundary). */
export function topologyKindCatalogBundleFromSceneMeta(meta: Record<string, unknown> | undefined): SceneKindCatalogBundle | undefined {
	if (!isTopologyMetaRecord(meta)) return undefined;
	const kc = meta.kindCatalogs;
	if (!kc || typeof kc !== "object" || Array.isArray(kc)) return undefined;
	return kc as SceneKindCatalogBundle;
}

/** @emoji 🧾 Picks a single catalog bundle for dual surfaces: board fixture meta wins, else scene meta. */
export function topologyPairedKindCatalogBundle(inp: {
	readonly boardMeta: Record<string, unknown> | undefined;
	readonly sceneMeta: Record<string, unknown> | undefined;
}): BoardKindCatalogBundle | undefined {
	const fromBoard = boardFixtureMetaKindCatalogBundle(inp.boardMeta);
	if (fromBoard) return fromBoard;
	return topologyKindCatalogBundleFromSceneMeta(inp.sceneMeta) as BoardKindCatalogBundle | undefined;
}

/** @emoji 🧾 Picks compatibility rows for dual surfaces: board meta wins when non-empty, else scene meta. */
export function topologyPairedKindCompatibility(inp: {
	readonly boardMeta: Record<string, unknown> | undefined;
	readonly sceneMeta: Record<string, unknown> | undefined;
}): readonly BoardKindCompatEntry[] {
	const fromBoard = topologyKindCompatibilityRowsFromMeta(inp.boardMeta);
	if (fromBoard.length > 0) return fromBoard;
	return topologyKindCompatibilityRowsFromMeta(inp.sceneMeta);
}
//#endregion 🧾PairedMeta

//#region 🎚️PairDefaults
/** @emoji 🎚️ Root layout class shared by `TopologyBoardPane` / `TopologyScenePane` shells (pair-aligned chrome). */
export const TOPOLOGY_PANE_ROOT_CLASS = "flex h-full min-h-0 flex-1 flex-col";

/** @emoji 🎚️ LOD + grid defaults shared by paired board/scene canvases (same knobs as standalone plays). */
export const TOPOLOGY_LOD_GRID_DEFAULTS: TopologyLodGridShared = {
	lodZoomThresholds: DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS,
	gridFactor: DEFAULT_BOARD_GRID_FACTOR,
	gridSnapEnabled: true,
};

/** @emoji 🎚️ Scene-only chrome defaults aligned with the scene play harness in `elements/scene/index.tsx` (topology passes them via `scene` prop). */
export function topologySceneChromeDefaults(): Pick<SceneCanvasProps, "showLodGrid" | "proximityRadius" | "gridSnapEnabled"> {
	return { showLodGrid: true, proximityRadius: 24, gridSnapEnabled: true };
}

/** @emoji 🎚️ Merges paired fixture metas into the shared LOD/grid + catalog fields consumed by `buildTopologyDualSurfaceBindings`. */
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
//#endregion 🎚️PairDefaults

//#region 🗼BoardMarkers
export interface TopologyBoardWireRecord {
	readonly id: string;
	readonly source: string;
	readonly target?: string;
	readonly wireKind?: string;
	readonly endX?: number;
	readonly endY?: number;
	readonly hidden?: boolean;
}

/** @emoji 🗼 Builds a Fragment of board host markers from a board fixture (same static shape walk as board play). */
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
//#endregion 🗼BoardMarkers

//#region 🪟Panes
//#region 🪟PanesBoard
export interface TopologyBoardPaneProps {
	readonly fixture: BoardFixtureV1;
	readonly bindings: TopologyDualSurfaceBindings;
	readonly selectedIds: ReadonlySet<string>;
	readonly wires?: readonly TopologyBoardWireRecord[];
	readonly board?: Omit<BoardCanvasProps, "children">;
}

export const TopologyBoardPane = memo(function TopologyBoardPane(props: TopologyBoardPaneProps) {
	const markers = useMemo(
		() =>
			topologyBoardMarkersFromFixture({
				fixture: props.fixture,
				lockedIds: new Set(),
				selectedIds: props.selectedIds,
				contextMenuById: () => [],
				wires: props.wires ?? [],
			}),
		[props.fixture, props.selectedIds, props.wires],
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
//#endregion 🪟PanesBoard

//#region 🪟PanesScene
export interface TopologyScenePaneProps {
	readonly fixture: SceneFixtureV1;
	readonly bindings: TopologyDualSurfaceBindings;
	readonly relocateMode: SceneRelocateMode;
	readonly selectedObjectId: string | null;
	readonly scene?: Omit<SceneCanvasProps, "children">;
	readonly blockedVortexFullIds?: ReadonlySet<string>;
}

export const TopologyScenePane = memo(function TopologyScenePane(props: TopologyScenePaneProps) {
	const blocked = props.blockedVortexFullIds ?? blockedVortexFullIdsFromTies(props.fixture.ties);
	const { scene: s } = props.bindings;
	const sceneRest = props.scene ?? {};
	return (
		<div className={TOPOLOGY_PANE_ROOT_CLASS} data-topology-scene-root data-topology-surface="scene">
			<Suspense fallback={<div className="flex min-h-0 flex-1 items-center justify-center p-4 text-sm text-muted-foreground">Loading meshes…</div>}>
				<Scene
					className="min-h-0 flex-1"
					camera={sceneRest.camera ?? props.fixture.camera}
					blockedVortexFullIds={blocked}
					{...s}
					{...sceneRest}
					relocateMode={props.relocateMode}
				>
					{props.fixture.objects.map((o) => (
						<SceneObject
							key={o.id}
							id={o.id}
							meshUrl={o.meshUrl}
							origin={o.origin}
							orientation={o.orientation}
							scale={o.scale}
							objectKind={o.objectKind}
							label={o.label}
							selected={props.selectedObjectId === o.id}
							relocate={props.relocateMode}
						>
							{o.vortices.map((v) => (
								<SceneVortex key={v.id} objectId={o.id} objectKind={o.objectKind} {...v} />
							))}
						</SceneObject>
					))}
					{props.fixture.ties.map((t) => (
						<SceneTie key={t.id} {...t} />
					))}
				</Scene>
			</Suspense>
		</div>
	);
});
//#endregion 🪟PanesScene
//#endregion 🪟Panes

export { parseBoardFixtureV1, parseFixtureV1, blockedVortexFullIdsFromTies };
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
}
