// #region 🧲Header
/// <reference types="vite/client" />
/// <reference types="vitest/importMeta" />
/** @emoji 🔧 `@procedural/react` — flow-based brep editor with R3F viewport. */
// #endregion 🧲Header

// #region 🔌Adapters
import {
	applyGumballPose,
	borderNormalClass,
	canvasHostRootClass,
	canvasViewportClass,
	cn,
	gumballHandleKindToTransformMode,
	gumballPointerConsumesCanvasEventRef,
	reactHostPort,
	sceneHostPort,
	UnifiedGumball,
	type GumballHandleKind,
	type GumballPose,
} from "@ui/react";
import { clearColorResolveCache, resolveSemanticColorHex } from "@ui/styling";
import {
	createDefaultBrepWasmBridge,
	ensureBrepWasmLoaded,
	isRenderableMeshTransfer,
	meshTransferToGeometryData,
	type BrepWasmBridge,
	type GeometryRef,
	type MeshTransfer,
	type Vec3,
} from "@geometry/brep/js";
import {
	FlowCanvas,
	createEphemeralFlowStore,
	type DagDrawLodKind,
	FlowExtensionHost,
	createFlowEvalBridge,
	type CatalogueSection,
	type FlowCanvasCommandRequest,
	type FlowCanvasContextMenuContext,
	type FlowFixtureV1,
	type FlowModuleCommandV1,
	type FlowReorganizeRequest,
} from "@flow/react";
import type { ContextMenuItem } from "@ui/react";
import { meshStyleColors, resolveMeshStyle, type MeshStyleKind } from "@puzzle/3d/react";
import {
	applyOrbitProjectionToCameraState,
	DEFAULT_LOD_GRID_FACTOR,
	DEFAULT_MANUAL_LOD,
	WORLD_LOCKED_OPACITY_SCALE,
	worldEntityRenderMode,
	WorldCameraInvalidator,
	WorldCanvas,
	WorldLayer,
	WorldLodBridge,
	WorldOrbitCameraViewRig,
	WorldOrbitGated,
	WorldOrbitProjectionSwitch,
	WorldOrbitViewControls,
	WorldOrbitViewSnapGateProvider,
	WorldEventBindingController,
	type OrbitCameraProjection,
	type WorldCameraState,
} from "@infinite/world/r3f";
import {
	SelectionMarquee,
	marqueeCoverageFromGesture,
	marqueeModeFromModifiers,
	screenRectContainsRect,
	screenRectFromPoints,
	screenRectIntersectsRect,
	selectionMergeIds,
	type SelectionMergeMode,
	type SelectionMarqueeCoverage,
} from "@ui/react";
import { type ReactNode } from "react";

const THREE = sceneHostPort.three;
// #endregion 🔌Adapters

// #region 🔖BrepWasmBridge
if (!import.meta.env.VITEST) {
	await ensureBrepWasmLoaded();
}
// #endregion 🔖BrepWasmBridge

// #region 🔖BrepFlowModule
let proceduralBrepBridgePromise: Promise<BrepWasmBridge> | null = null;

export async function ensureProceduralBrepBridge(): Promise<BrepWasmBridge> {
	proceduralBrepBridgePromise ??= createDefaultBrepWasmBridge();
	return proceduralBrepBridgePromise;
}

/** @emoji 🔌 Flow extension host with `@flow/module-brep` loaded through the normal module path. */
export class ProceduralExtensionHost extends FlowExtensionHost {
	private bridge: BrepWasmBridge | null = null;

	async activateDefaults(): Promise<void> {
		await super.activateDefaults();
		if (!this.isActive("brep")) {
			await this.activate("brep");
		}
		this.bridge = await ensureProceduralBrepBridge();
	}

	getBrepBridge(): BrepWasmBridge {
		if (!this.bridge) throw new Error("brep wasm not ready");
		return this.bridge;
	}

	tryGetBrepBridge(): BrepWasmBridge | null {
		return this.bridge;
	}

	/** @emoji 🧭 Back-compat alias for preview hosts that still call `getBrepKernel`. */
	getBrepKernel(): BrepWasmBridge {
		return this.getBrepBridge();
	}
}

export const proceduralExtensionHost = new ProceduralExtensionHost();

/** @emoji 🔌 Resolves the brep WASM bridge after extension defaults are activated. */
export function useProceduralBrepBridge(host: ProceduralExtensionHost = proceduralExtensionHost): BrepWasmBridge | null {
	const [bridge, setBridge] = reactHostPort.useState<BrepWasmBridge | null>(host.tryGetBrepBridge());
	reactHostPort.useEffect(() => {
		let cancelled = false;
		void host.activateDefaults().then(() => {
			if (cancelled) return;
			setBridge(host.tryGetBrepBridge());
		});
		return () => {
			cancelled = true;
		};
	}, [host]);
	return bridge;
}
// #endregion 🔖BrepFlowModule


// #region 🔖Fixture
export const PROCEDURAL_DEFAULT_FIXTURE: FlowFixtureV1 = {
	schema: "flow.fixture/v1",
	camera: { x: 0, y: 0, zoom: 1 },
	widgets: [
		{ kind: "neuron", id: "box", neuronKind: "brep.prim3d.box" },
		{ kind: "neuron", id: "fillet", neuronKind: "brep.solid.fillet" },
		{ kind: "neuron", id: "offset", neuronKind: "math.vector" },
		{ kind: "neuron", id: "move", neuronKind: "brep.xform.translate" },
		{ kind: "outputPreview", id: "preview" },
	],
	synapses: [
		{ id: "s1", from: "box", to: "fillet", fromPort: "solid", toPort: "geometry" },
		{ id: "s2", from: "fillet", to: "move", fromPort: "solid", toPort: "geometry" },
		{ id: "s3", from: "offset", to: "move", fromPort: "vector", toPort: "offset" },
		{ id: "s4", from: "move", to: "preview", fromPort: "geometry", toPort: "" },
	],
};

export function proceduralFixtureToJson(fixture: FlowFixtureV1 = PROCEDURAL_DEFAULT_FIXTURE): string {
	return JSON.stringify(fixture);
}

const PROCEDURAL_FLOW_STORE = createEphemeralFlowStore();
// #endregion 🔖Fixture

// #region 🔖ProceduralPreview
export type ProceduralChannelDirection = "in" | "out";

export interface ProceduralChannelRef {
	readonly widgetId: string;
	readonly port: string;
	readonly direction: ProceduralChannelDirection;
}

export type ProceduralPreviewItem =
	| {
			readonly widgetId: string;
			readonly port: string;
			readonly direction: ProceduralChannelDirection;
			readonly kind: "geometry";
			readonly handle: GeometryRef;
			readonly mesh?: MeshTransfer;
	  }
	| { readonly widgetId: string; readonly port: string; readonly direction: ProceduralChannelDirection; readonly kind: "point"; readonly position: Vec3 }
	| { readonly widgetId: string; readonly port: string; readonly direction: ProceduralChannelDirection; readonly kind: "vector"; readonly directionVec: Vec3 };

export interface ProceduralPreviewExtractContext {
	readonly widgetId: string;
	readonly port: string;
	readonly direction: ProceduralChannelDirection;
	readonly value: unknown;
}

export type ProceduralPreviewExtractor = (context: ProceduralPreviewExtractContext) => readonly ProceduralPreviewItem[];

const proceduralPreviewExtractors: ProceduralPreviewExtractor[] = [];

/** @emoji 📋 Registers a channel-value preview extractor for procedural flow eval. */
export function registerPreviewExtractor(extractor: ProceduralPreviewExtractor): void {
	proceduralPreviewExtractors.push(extractor);
}

function runPreviewExtractors(context: ProceduralPreviewExtractContext): ProceduralPreviewItem[] {
	const items: ProceduralPreviewItem[] = [];
	for (const extractor of proceduralPreviewExtractors) {
		items.push(...extractor(context));
	}
	return items;
}

function previewItemsFromChannelValue(widgetId: string, port: string, direction: ProceduralChannelDirection, value: unknown): ProceduralPreviewItem[] {
	if (value && typeof value === "object" && !Array.isArray(value) && typeof (value as Record<string, unknown>).error === "string") {
		return [];
	}
	return runPreviewExtractors({ widgetId, port, direction, value });
}

export interface ProceduralFixtureEdge {
	readonly source: string;
	readonly target: string;
}

/** @emoji 🔗 Resolves hovered/selected channels to output geometry channels for 3D emphasis. */
export function resolveGeometryTargets(
	channels: readonly ProceduralChannelRef[],
	nodeFallbackId: string | null,
	previewItems: readonly ProceduralPreviewItem[],
	edges: readonly ProceduralFixtureEdge[],
): ProceduralChannelRef[] {
	const seen = new Set<string>();
	const targets: ProceduralChannelRef[] = [];
	const push = (channel: ProceduralChannelRef) => {
		const key = `${channel.widgetId}:${channel.port}:${channel.direction}`;
		if (seen.has(key)) return;
		seen.add(key);
		targets.push(channel);
	};
	for (const channel of channels) {
		if (channel.direction === "out") {
			push(channel);
			continue;
		}
		const targetKey = `${channel.widgetId}:${channel.port}`;
		const edge = edges.find((entry) => entry.target === targetKey);
		if (!edge) continue;
		const colon = edge.source.indexOf(":");
		if (colon <= 0) continue;
		push({ widgetId: edge.source.slice(0, colon), port: edge.source.slice(colon + 1), direction: "out" });
	}
	if (channels.length === 0 && nodeFallbackId) {
		for (const item of previewItems) {
			if (item.widgetId === nodeFallbackId && item.direction === "out") {
				push({ widgetId: item.widgetId, port: item.port, direction: "out" });
			}
		}
	}
	return targets;
}

function geometryTargetMatches(item: ProceduralPreviewItem, targets: readonly ProceduralChannelRef[]): boolean {
	return item.direction === "out" && targets.some((target) => item.widgetId === target.widgetId && item.port === target.port);
}

function resolveSelectedPreviewTargets(
	items: readonly ProceduralPreviewItem[],
	options: {
		readonly selectedNodeIds: readonly string[];
		readonly selectedChannels: readonly ProceduralChannelRef[];
		readonly selectedGeometryTargets: readonly ProceduralChannelRef[];
		readonly edges: readonly ProceduralFixtureEdge[];
	},
): ProceduralChannelRef[] {
	if (options.selectedGeometryTargets.length > 0) return [...options.selectedGeometryTargets];
	if (options.selectedChannels.length > 0) {
		return resolveGeometryTargets(options.selectedChannels, null, items, options.edges);
	}
	if (options.selectedNodeIds.length > 0) {
		const targets: ProceduralChannelRef[] = [];
		for (const widgetId of options.selectedNodeIds) {
			targets.push(...resolveGeometryTargets([], widgetId, items, options.edges));
		}
		return targets;
	}
	return [];
}

export function filterVisiblePreviewItems(
	items: readonly ProceduralPreviewItem[],
	options: {
		readonly showMode: ProceduralPreviewShowMode;
		readonly selectedNodeIds: readonly string[];
		readonly selectedChannels: readonly ProceduralChannelRef[];
		readonly selectedGeometryTargets?: readonly ProceduralChannelRef[];
		readonly edges?: readonly ProceduralFixtureEdge[];
		readonly hoveredNodeId: string | null;
		readonly hoveredChannel: ProceduralChannelRef | null;
	},
): ProceduralPreviewItem[] {
	const { showMode } = options;
	if (showMode === "selected") {
		const targets = resolveSelectedPreviewTargets(items, {
			selectedNodeIds: options.selectedNodeIds,
			selectedChannels: options.selectedChannels,
			selectedGeometryTargets: options.selectedGeometryTargets ?? [],
			edges: options.edges ?? [],
		});
		if (targets.length === 0) return [];
		return items.filter((entry) => geometryTargetMatches(entry, targets));
	}
	return items.filter((entry) => entry.direction === "out");
}

registerPreviewExtractor((context) => {
	const refs: GeometryRef[] = [];
	collectGeometryRefsFromValue(context.value, refs);
	return [...new Set(refs)].map((handle) => ({
		widgetId: context.widgetId,
		port: context.port,
		direction: context.direction,
		kind: "geometry" as const,
		handle,
	}));
});

registerPreviewExtractor((context) => {
	const point = parseChannelPoint(context.value);
	if (!point) return [];
	return [{ widgetId: context.widgetId, port: context.port, direction: context.direction, kind: "point", position: point }];
});

registerPreviewExtractor((context) => {
	const vector = parseChannelVector(context.value);
	if (!vector) return [];
	return [{ widgetId: context.widgetId, port: context.port, direction: context.direction, kind: "vector", directionVec: vector }];
});

export type ProceduralPreviewShowMode = "everything" | "selected";
export type ProceduralSelectionMode = SelectionMergeMode;
export type ProceduralSelectionMethod = "rectangle" | "lasso";
export type ProceduralTransformGranularity = "compact" | "full";
export type ProceduralGumballTransformOp = "translate" | "rotate" | "scale";

export type ProceduralGumballTransformDelta =
	| { readonly op: "translate"; readonly offset: readonly [number, number, number] }
	| { readonly op: "rotate"; readonly angle: number }
	| { readonly op: "scale"; readonly factor: number };

export type ProceduralGumballTransformPhase = "start" | "live" | "end";

export interface ProceduralGumballTransformRequest {
	readonly widgetId: string;
	readonly delta: ProceduralGumballTransformDelta;
	readonly granularity: ProceduralTransformGranularity;
	readonly phase?: ProceduralGumballTransformPhase;
}

function gumballDeltaFromPoses(
	mode: ProceduralGumballTransformRequest["delta"]["op"],
	before: GumballPose,
	after: GumballPose,
): ProceduralGumballTransformRequest["delta"] {
	if (mode === "translate") {
		return {
			op: "translate",
			offset: [
				after.position[0] - before.position[0],
				after.position[1] - before.position[1],
				after.position[2] - before.position[2],
			],
		};
	}
	if (mode === "rotate") {
		const qb = new THREE.Quaternion(...before.quaternion);
		const qa = new THREE.Quaternion(...after.quaternion);
		const eulerBefore = new THREE.Euler().setFromQuaternion(qb, "XYZ");
		const eulerAfter = new THREE.Euler().setFromQuaternion(qa, "XYZ");
		return { op: "rotate", angle: eulerAfter.z - eulerBefore.z };
	}
	const beforeScale = before.scale[0] || 1;
	return { op: "scale", factor: after.scale[0] / beforeScale };
}

function ProceduralPreviewGumball({
	item,
	kernel,
	transformGranularity,
	onGumballTransform,
	onInteractionChange,
}: {
	readonly item: Extract<ProceduralPreviewItem, { kind: "geometry" }>;
	readonly kernel: BrepWasmBridge;
	readonly transformGranularity: ProceduralTransformGranularity;
	readonly onGumballTransform?: (request: ProceduralGumballTransformRequest) => void;
	readonly onInteractionChange?: (widgetId: string | null) => void;
}): ReactNode {
	const [target, setTarget] = reactHostPort.useState<THREE.Object3D | null>(null);
	const dragBeforeRef = reactHostPort.useRef<GumballPose | null>(null);
	const center = reactHostPort.useMemo(() => {
		const bounds = worldBoundsForPreviewItem(item, kernel);
		if (!bounds) return [0, 0, 0] as Vec3;
		return [
			(bounds.min[0] + bounds.max[0]) * 0.5,
			(bounds.min[1] + bounds.max[1]) * 0.5,
			(bounds.min[2] + bounds.max[2]) * 0.5,
		] as Vec3;
	}, [item, kernel]);
	const setGumballInteractionActive = reactHostPort.useCallback(
		(active: boolean) => {
			proceduralGumballDragActiveRef.current = active;
			gumballPointerConsumesCanvasEventRef.current = active;
			onInteractionChange?.(active ? item.widgetId : null);
		},
		[item.widgetId, onInteractionChange],
	);
	const emitGumballTransform = reactHostPort.useCallback(
		(phase: ProceduralGumballTransformPhase, kind: GumballHandleKind, before: GumballPose, after: GumballPose) => {
			const mode = gumballHandleKindToTransformMode(kind);
			const delta = gumballDeltaFromPoses(mode, before, after);
			console.log(`[DEBUG] procedural gumball ${item.widgetId} ${mode} ${phase}`, delta);
			onGumballTransform?.({ widgetId: item.widgetId, delta, granularity: transformGranularity, phase });
		},
		[item.widgetId, onGumballTransform, transformGranularity],
	);
	if (!onGumballTransform) return null;
	return (
		<group position={center}>
			<group ref={(node) => setTarget(node)} />
			{target ? (
				<UnifiedGumball
					target={target}
					onDragStart={(kind: GumballHandleKind, pose: GumballPose) => {
						dragBeforeRef.current = pose;
						setGumballInteractionActive(true);
						emitGumballTransform("start", kind, pose, pose);
					}}
					onDraggingChanged={(active) => {
						setGumballInteractionActive(active);
					}}
					onDrag={(kind: GumballHandleKind, after: GumballPose) => {
						const before = dragBeforeRef.current;
						if (!before) return;
						emitGumballTransform("live", kind, before, after);
						applyGumballPose(target, before);
					}}
					onDragEnd={(kind: GumballHandleKind, before: GumballPose, after: GumballPose) => {
						emitGumballTransform("end", kind, before, after);
						applyGumballPose(target, before);
						dragBeforeRef.current = null;
						setGumballInteractionActive(false);
					}}
				/>
			) : null}
		</group>
	);
}

export interface ProceduralPreviewProps {
	readonly items: readonly ProceduralPreviewItem[];
	readonly selectedNodeIds?: readonly string[];
	readonly selectedChannels?: readonly ProceduralChannelRef[];
	readonly preselectNodeIds?: readonly string[];
	readonly preselectRemovedNodeIds?: readonly string[];
	readonly hoveredNodeId?: string | null;
	readonly hoveredChannel?: ProceduralChannelRef | null;
	readonly hoveredGeometryTargets?: readonly ProceduralChannelRef[];
	readonly selectedGeometryTargets?: readonly ProceduralChannelRef[];
	readonly fixtureEdges?: readonly ProceduralFixtureEdge[];
	readonly previewOffNodeIds?: readonly string[];
	readonly showMode?: ProceduralPreviewShowMode;
	readonly selectionMode?: ProceduralSelectionMode;
	readonly selectionMethod?: ProceduralSelectionMethod;
	readonly transformGranularity?: ProceduralTransformGranularity;
	readonly onGumballTransform?: (request: ProceduralGumballTransformRequest) => void;
	readonly gumballActiveWidgetIds?: readonly string[];
	readonly onHover?: (channel: ProceduralChannelRef | null) => void;
	readonly onSelect?: (channel: ProceduralChannelRef) => void;
	readonly onSelectionChange?: (ids: readonly string[], mode: ProceduralSelectionMode) => void;
	readonly kernel?: BrepWasmBridge;
	readonly tolerance?: number;
	readonly className?: string;
}

const PROCEDURAL_PREVIEW_MARQUEE_THRESHOLD_PX = 4;

/** @emoji 🎛 True while a procedural preview gumball drag is active (blocks marquee selection). */
export const proceduralGumballDragActiveRef = { current: false };

type PreviewLayerChrome = {
	readonly selected: boolean;
	readonly highlighted: boolean;
	readonly hovered: boolean;
	readonly previewOff: boolean;
	readonly locked: boolean;
	readonly interactionHighlighted: boolean;
	readonly pickEnabled: boolean;
	readonly onHover?: (channel: ProceduralChannelRef | null) => void;
	readonly onPick?: (channel: ProceduralChannelRef, mode: ProceduralSelectionMode) => void;
};

interface BrepMeshBuffers {
	readonly surface: THREE.BufferGeometry | null;
	readonly lines: THREE.BufferGeometry | null;
	readonly points: THREE.BufferGeometry | null;
}

function buildMeshBuffers(data: ReturnType<typeof meshTransferToGeometryData>): BrepMeshBuffers {
	let surface: THREE.BufferGeometry | null = null;
	if (data.position.length > 0 && data.index.length > 0) {
		const geometry = new THREE.BufferGeometry();
		geometry.setAttribute("position", new THREE.Float32BufferAttribute(data.position, 3));
		geometry.setAttribute("normal", new THREE.Float32BufferAttribute(data.normal, 3));
		geometry.setIndex(new THREE.BufferAttribute(data.index, 1));
		for (const g of data.faceGroups) geometry.addGroup(g.start, g.count, 0);
		surface = geometry;
	}
	let lines: THREE.BufferGeometry | null = null;
	if (data.edges.length > 0) {
		const geometry = new THREE.BufferGeometry();
		geometry.setAttribute("position", new THREE.Float32BufferAttribute(data.edges, 3));
		lines = geometry;
	}
	let points: THREE.BufferGeometry | null = null;
	if (data.points.length > 0) {
		const geometry = new THREE.BufferGeometry();
		geometry.setAttribute("position", new THREE.Float32BufferAttribute(data.points, 3));
		points = geometry;
	}
	return { surface, lines, points };
}

const PROCEDURAL_PREVIEW_POINT_RADIUS = 0.08;
const PROCEDURAL_PREVIEW_BOUNDS_PAD = 0.05;
const PROCEDURAL_PREVIEW_LINE_PICK_MIN = 0.08;
const PROCEDURAL_GEOMETRY_REF_PATTERN = /^(vertex|edge|wire|face|shell|solid|compound|curve|surface|drawing)-/;

export function previewPickProxyFromBounds(bounds: { min: Vec3; max: Vec3 }): { position: Vec3; size: Vec3 } {
	const [minX, minY, minZ] = bounds.min;
	const [maxX, maxY, maxZ] = bounds.max;
	const minPick = PROCEDURAL_PREVIEW_LINE_PICK_MIN;
	return {
		position: [(minX + maxX) * 0.5, (minY + maxY) * 0.5, (minZ + maxZ) * 0.5],
		size: [Math.max(maxX - minX, minPick), Math.max(maxY - minY, minPick), Math.max(maxZ - minZ, minPick)],
	};
}

function previewLineColor(colors: NonNullable<ReturnType<typeof meshStyleColors>>, hovered: boolean, hasSurface: boolean): string {
	if (hovered && !hasSurface) {
		return meshStyleColors("selected")?.lineColor ?? meshStyleColors("highlighted")?.lineColor ?? colors.lineColor;
	}
	return colors.lineColor;
}

function parseVec3Loose(input: unknown): Vec3 | null {
	if (Array.isArray(input) && input.length >= 3) {
		const x = Number(input[0]);
		const y = Number(input[1]);
		const z = Number(input[2]);
		if (Number.isFinite(x) && Number.isFinite(y) && Number.isFinite(z)) return [x, y, z];
		return null;
	}
	if (!input || typeof input !== "object" || Array.isArray(input)) return null;
	const dict = input as Record<string, unknown>;
	const x = Number(dict.x);
	const y = Number(dict.y);
	const z = Number(dict.z);
	if (!Number.isFinite(x) || !Number.isFinite(y) || !Number.isFinite(z)) return null;
	return [x, y, z];
}

function parsePreviewVec3(input: unknown): Vec3 | null {
	return parseVec3Loose(input);
}

function channelSchema(value: unknown): string | null {
	if (!value || typeof value !== "object" || Array.isArray(value)) return null;
	const schema = (value as Record<string, unknown>).$schema;
	return typeof schema === "string" ? schema : null;
}

/** @emoji 📍 Resolves a flow channel value to a point position for 3D preview. */
function parseChannelPoint(value: unknown): Vec3 | null {
	if (Array.isArray(value)) return parsePreviewVec3(value);
	if (!value || typeof value !== "object") return null;
	const dict = value as Record<string, unknown>;
	if (channelSchema(value) === "vector") return null;
	const nested = dict.point ?? dict.position ?? dict.center;
	if (nested !== undefined) return parsePreviewVec3(nested);
	if (channelSchema(value) === "point") return parsePreviewVec3(dict);
	return null;
}

/** @emoji ➡️ Resolves a flow channel value to a vector direction for 3D preview. */
function parseChannelVector(value: unknown): Vec3 | null {
	if (Array.isArray(value)) return parsePreviewVec3(value);
	if (!value || typeof value !== "object") return null;
	const dict = value as Record<string, unknown>;
	if (channelSchema(value) === "point") return null;
	const nested = dict.vector ?? dict.direction ?? dict.normal ?? dict.tangent;
	if (nested !== undefined) return parsePreviewVec3(nested);
	if (channelSchema(value) === "vector") return parsePreviewVec3(dict);
	return null;
}

function collectGeometryRefsFromValue(value: unknown, refs: GeometryRef[]): void {
	if (typeof value === "string" && PROCEDURAL_GEOMETRY_REF_PATTERN.test(value)) {
		refs.push(value as GeometryRef);
		return;
	}
	if (!value || typeof value !== "object" || Array.isArray(value)) return;
	for (const nested of Object.values(value as Record<string, unknown>)) {
		collectGeometryRefsFromValue(nested, refs);
	}
}

function previewLayerPaint(chrome: Pick<PreviewLayerChrome, "previewOff" | "hovered" | "selected" | "highlighted" | "locked" | "interactionHighlighted">): {
	readonly renderMode: ReturnType<typeof worldEntityRenderMode>;
	readonly style: MeshStyleKind;
	readonly colors: NonNullable<ReturnType<typeof meshStyleColors>>;
	readonly opacity: number;
} {
	const interactionHighlighted = chrome.interactionHighlighted;
	const locked = chrome.locked;
	const renderMode = worldEntityRenderMode(
		{ hidden: chrome.previewOff, locked },
		{
			hovered: chrome.hovered && !locked,
			selected: chrome.selected || chrome.highlighted || interactionHighlighted,
			revealed: chrome.hovered,
		},
	);
	const style = resolveMeshStyle({
		selected: renderMode.showSelectedOutline || interactionHighlighted,
		highlighted: chrome.highlighted || interactionHighlighted,
		hovered: renderMode.asHover || chrome.hovered || interactionHighlighted,
	});
	const colors = meshStyleColors(style) ?? meshStyleColors("neutral")!;
	const opacity = colors.opacity * (renderMode.dim ? WORLD_LOCKED_OPACITY_SCALE : 1);
	return { renderMode, style, colors, opacity };
}

function previewItemChannel(item: ProceduralPreviewItem): ProceduralChannelRef {
	return { widgetId: item.widgetId, port: item.port, direction: item.direction };
}

export function previewOffForItem(
	showMode: ProceduralPreviewShowMode,
	widgetId: string,
	previewOffNodeIds: readonly string[],
): boolean {
	return showMode !== "selected" && previewOffNodeIds.includes(widgetId);
}

function createPreviewPointerHandlers(
	channel: ProceduralChannelRef,
	onHover?: (channel: ProceduralChannelRef | null) => void,
	onPick?: (channel: ProceduralChannelRef, mode: ProceduralSelectionMode) => void,
	pickEnabled = true,
) {
	if (!pickEnabled || (!onHover && !onPick)) return {};
	return {
		onPointerDown: (event: { stopPropagation: () => void; nativeEvent: PointerEvent }) => {
			if (event.nativeEvent.button !== 0) return;
			event.stopPropagation();
		},
		onPointerOver: (event: { stopPropagation: () => void }) => {
			event.stopPropagation();
			onHover?.(channel);
		},
		onPointerOut: (event: { stopPropagation: () => void }) => {
			event.stopPropagation();
			onHover?.(null);
		},
		onClick: (event: { stopPropagation: () => void; shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => {
			event.stopPropagation();
			const mode = marqueeModeFromModifiers(event);
			onPick?.(channel, mode);
		},
	};
}

function boundsFromMeshTransfer(mesh: MeshTransfer, pad: number): { min: Vec3; max: Vec3 } | null {
	const positions = mesh.position;
	if (!positions.length) return null;
	let minX = Number.POSITIVE_INFINITY;
	let minY = Number.POSITIVE_INFINITY;
	let minZ = Number.POSITIVE_INFINITY;
	let maxX = Number.NEGATIVE_INFINITY;
	let maxY = Number.NEGATIVE_INFINITY;
	let maxZ = Number.NEGATIVE_INFINITY;
	for (let i = 0; i < positions.length; i += 3) {
		const x = positions[i]!;
		const y = positions[i + 1]!;
		const z = positions[i + 2]!;
		minX = Math.min(minX, x);
		minY = Math.min(minY, y);
		minZ = Math.min(minZ, z);
		maxX = Math.max(maxX, x);
		maxY = Math.max(maxY, y);
		maxZ = Math.max(maxZ, z);
	}
	if (!Number.isFinite(minX)) return null;
	return {
		min: [minX - pad, minY - pad, minZ - pad],
		max: [maxX + pad, maxY + pad, maxZ + pad],
	};
}

function worldBoundsForPreviewItem(item: ProceduralPreviewItem, _kernel: BrepWasmBridge): { min: Vec3; max: Vec3 } | null {
	const pad = PROCEDURAL_PREVIEW_BOUNDS_PAD;
	if (item.kind === "geometry") {
		if (item.mesh && isRenderableMeshTransfer(item.mesh)) {
			return boundsFromMeshTransfer(item.mesh, pad);
		}
		return null;
	}
	if (item.kind === "point") {
		const [x, y, z] = item.position;
		return { min: [x - pad, y - pad, z - pad], max: [x + pad, y + pad, z + pad] };
	}
	const [x, y, z] = item.directionVec;
	return {
		min: [Math.min(0, x) - pad, Math.min(0, y) - pad, Math.min(0, z) - pad],
		max: [Math.max(0, x) + pad, Math.max(0, y) + pad, Math.max(0, z) + pad],
	};
}

function previewItemKey(item: ProceduralPreviewItem): string {
	if (item.kind === "geometry") return `${item.widgetId}:${item.direction}:${item.port}:geometry:${item.handle}`;
	if (item.kind === "point") return `${item.widgetId}:${item.direction}:${item.port}:point`;
	return `${item.widgetId}:${item.direction}:${item.port}:vector`;
}

function BrepPreviewLayer({
	item,
	kernel,
	tolerance,
	...chrome
}: {
	readonly item: ProceduralPreviewItem;
	readonly kernel: BrepWasmBridge;
	readonly tolerance: number;
} & PreviewLayerChrome): ReactNode {
	const { renderMode, colors, opacity } = previewLayerPaint(chrome);
	const handlers = createPreviewPointerHandlers(previewItemChannel(item), chrome.onHover, chrome.onPick, chrome.pickEnabled);
	const [buffers, setBuffers] = reactHostPort.useState<BrepMeshBuffers>({ surface: null, lines: null, points: null });
	const invalidate = sceneHostPort.fiber.useThree((state) => state.invalidate);
	const geometryRef = item.kind === "geometry" ? item.handle : null;
	const lineOnlyGeometry = item.kind === "geometry" && !buffers.surface && Boolean(buffers.lines);
	const pickProxy = reactHostPort.useMemo(() => {
		if (!lineOnlyGeometry) return null;
		const bounds = worldBoundsForPreviewItem(item, kernel);
		return bounds ? previewPickProxyFromBounds(bounds) : null;
	}, [item, kernel, lineOnlyGeometry]);
	const lineColor = previewLineColor(colors, renderMode.asHover, Boolean(buffers.surface));
	const arrow = reactHostPort.useMemo(() => {
		if (item.kind !== "vector") return null;
		const tip = new THREE.Vector3(item.directionVec[0], item.directionVec[1], item.directionVec[2]);
		const length = tip.length();
		if (length < 1e-6) return null;
		const unit = tip.clone().normalize();
		const shaftEnd = unit.clone().multiplyScalar(length * 0.85);
		const shaft = new THREE.BufferGeometry().setFromPoints([new THREE.Vector3(0, 0, 0), shaftEnd]);
		const headHeight = length * 0.15;
		const headRadius = headHeight * 0.35;
		const head = new THREE.ConeGeometry(headRadius, headHeight, 10);
		const quaternion = new THREE.Quaternion().setFromUnitVectors(new THREE.Vector3(0, 1, 0), unit);
		const headPosition = unit.clone().multiplyScalar(length - headHeight * 0.5);
		return { shaft, head, headPosition, quaternion };
	}, [item]);

	reactHostPort.useEffect(() => {
		if (!geometryRef) return;
		if (item.kind === "geometry" && item.mesh && isRenderableMeshTransfer(item.mesh)) {
			setBuffers(buildMeshBuffers(meshTransferToGeometryData(item.mesh)));
			invalidate();
			return;
		}
		let cancelled = false;
		void (async () => {
			await ensureBrepWasmLoaded();
			const mesh = await kernel.tessellateGeometry(geometryRef, tolerance);
			if (cancelled) return;
			if (!isRenderableMeshTransfer(mesh)) {
				setBuffers({ surface: null, lines: null, points: null });
				invalidate();
				return;
			}
			setBuffers(buildMeshBuffers(meshTransferToGeometryData(mesh)));
			invalidate();
		})();
		return () => {
			cancelled = true;
		};
	}, [geometryRef, invalidate, item, kernel, tolerance]);

	if (!renderMode.visible) return null;

	if (item.kind === "point") {
		const radius = renderMode.asHover ? PROCEDURAL_PREVIEW_POINT_RADIUS * 1.25 : PROCEDURAL_PREVIEW_POINT_RADIUS;
		return (
			<group position={item.position} {...handlers}>
				<mesh>
					<sphereGeometry args={[radius, 16, 12]} />
					<meshStandardMaterial
						color={colors.meshColor}
						emissive={colors.emissiveColor}
						emissiveIntensity={colors.emissiveIntensity}
						metalness={0}
						roughness={1}
						transparent={opacity < 1}
						opacity={opacity}
					/>
				</mesh>
			</group>
		);
	}

	if (item.kind === "vector") {
		if (!arrow) return null;
		return (
			<group {...handlers}>
				<line geometry={arrow.shaft}>
					<lineBasicMaterial color={colors.lineColor} linewidth={1} transparent={opacity < 1} opacity={opacity} />
				</line>
				<mesh geometry={arrow.head} position={arrow.headPosition} quaternion={arrow.quaternion}>
					<meshStandardMaterial
						color={colors.meshColor}
						emissive={colors.emissiveColor}
						emissiveIntensity={colors.emissiveIntensity}
						metalness={0}
						roughness={1}
						transparent={opacity < 1}
						opacity={opacity}
					/>
				</mesh>
			</group>
		);
	}

	return (
		<group {...(pickProxy ? {} : handlers)}>
			{pickProxy ? (
				<mesh position={pickProxy.position} {...handlers}>
					<boxGeometry args={pickProxy.size} />
					<meshBasicMaterial transparent opacity={0} depthWrite={false} side={THREE.DoubleSide} />
				</mesh>
			) : null}
			{buffers.surface ? (
				<mesh geometry={buffers.surface} raycast={chrome.pickEnabled ? undefined : () => null}>
					<meshStandardMaterial
						color={colors.meshColor}
						emissive={colors.emissiveColor}
						emissiveIntensity={colors.emissiveIntensity}
						metalness={chrome.locked ? 0.15 : 0}
						roughness={chrome.locked ? 0.35 : 1}
						transparent={opacity < 1}
						opacity={opacity}
						side={THREE.DoubleSide}
					/>
				</mesh>
			) : null}
			{buffers.lines ? (
				<lineSegments geometry={buffers.lines} raycast={() => null}>
					<lineBasicMaterial
						color={lineColor}
						linewidth={renderMode.asHover && !buffers.surface ? 2 : 1}
						transparent={opacity < 1}
						opacity={opacity}
					/>
				</lineSegments>
			) : null}
			{buffers.points ? (
				<points geometry={buffers.points}>
					<pointsMaterial color={lineColor} size={renderMode.asHover ? 0.16 : 0.12} transparent={opacity < 1} opacity={opacity} />
				</points>
			) : null}
		</group>
	);
}

export const PROCEDURAL_PREVIEW_DEFAULT_CAMERA: WorldCameraState = {
	position: [8, 8, 6],
	target: [0, 0, 0],
	zoom: 1,
	up: [0, 0, 1],
	projection: "perspective",
};

export function proceduralPreviewCameraSeed(seed: number): string {
	return `procedural-preview-camera-${seed}`;
}

type ScreenBounds = { readonly left: number; readonly top: number; readonly right: number; readonly bottom: number };

function projectWorldBoundsToScreen(bounds: { min: Vec3; max: Vec3 }, camera: THREE.Camera, width: number, height: number): ScreenBounds | null {
	const corners: Vec3[] = [
		[bounds.min[0], bounds.min[1], bounds.min[2]],
		[bounds.max[0], bounds.min[1], bounds.min[2]],
		[bounds.min[0], bounds.max[1], bounds.min[2]],
		[bounds.max[0], bounds.max[1], bounds.min[2]],
		[bounds.min[0], bounds.min[1], bounds.max[2]],
		[bounds.max[0], bounds.min[1], bounds.max[2]],
		[bounds.min[0], bounds.max[1], bounds.max[2]],
		[bounds.max[0], bounds.max[1], bounds.max[2]],
	];
	const vector = new THREE.Vector3();
	let left = Number.POSITIVE_INFINITY;
	let top = Number.POSITIVE_INFINITY;
	let right = Number.NEGATIVE_INFINITY;
	let bottom = Number.NEGATIVE_INFINITY;
	for (const corner of corners) {
		vector.set(corner[0], corner[1], corner[2]).project(camera);
		const x = ((vector.x + 1) / 2) * width;
		const y = ((-vector.y + 1) / 2) * height;
		left = Math.min(left, x);
		top = Math.min(top, y);
		right = Math.max(right, x);
		bottom = Math.max(bottom, y);
	}
	if (!Number.isFinite(left) || !Number.isFinite(top) || !Number.isFinite(right) || !Number.isFinite(bottom)) return null;
	return { left, top, right, bottom };
}

function ProceduralPreviewCameraBridge({
	onCamera,
}: {
	readonly onCamera: (camera: THREE.Camera, size: { width: number; height: number }) => void;
}): null {
	const invalidate = sceneHostPort.fiber.useThree((state) => state.invalidate);
	const camera = sceneHostPort.fiber.useThree((state) => state.camera);
	const size = sceneHostPort.fiber.useThree((state) => state.size);
	reactHostPort.useEffect(() => {
		onCamera(camera, size);
		invalidate();
	}, [camera, invalidate, onCamera, size, size.height, size.width]);
	return null;
}

function ProceduralPreviewSceneInvalidator(props: { readonly token: string }): null {
	const invalidate = sceneHostPort.fiber.useThree((state) => state.invalidate);
	reactHostPort.useLayoutEffect(() => {
		invalidate();
		let frame = 0;
		let raf = 0;
		const tick = () => {
			invalidate();
			frame += 1;
			if (frame < 3) raf = requestAnimationFrame(tick);
		};
		raf = requestAnimationFrame(tick);
		return () => cancelAnimationFrame(raf);
	}, [invalidate, props.token]);
	return null;
}

function ProceduralPreviewMarqueeBridge({
	containerRef,
	selectionMethod,
	selectedNodeIds,
	resolveMarqueeHits,
	commitSelection,
	onMarqueeOverlay,
	onLivePreselect,
}: {
	readonly containerRef: React.RefObject<HTMLDivElement | null>;
	readonly selectionMethod: ProceduralSelectionMethod;
	readonly selectedNodeIds: readonly string[];
	readonly resolveMarqueeHits: (points: readonly { x: number; y: number }[], crossing: boolean) => string[];
	readonly commitSelection: (ids: readonly string[], mode: ProceduralSelectionMode) => void;
	readonly onMarqueeOverlay: (overlay: {
		coverage: SelectionMarqueeCoverage;
		shape: "rect" | "polygon";
		rect?: { x: number; y: number; width: number; height: number };
		points?: readonly { x: number; y: number }[];
	} | null) => void;
	readonly onLivePreselect: (snapshot: { ids: string[]; removedIds: string[] }) => void;
}): null {
	const gl = sceneHostPort.fiber.useThree((state) => state.gl);
	const invalidate = sceneHostPort.fiber.useThree((state) => state.invalidate);
	const marqueeRef = reactHostPort.useRef<{ tracking: boolean; active: boolean; start: { x: number; y: number }; points: { x: number; y: number }[]; initial: string[] }>({
		tracking: false,
		active: false,
		start: { x: 0, y: 0 },
		points: [],
		initial: [],
	});
	const resolveMarqueeHitsRef = reactHostPort.useRef(resolveMarqueeHits);
	const commitSelectionRef = reactHostPort.useRef(commitSelection);
	const onMarqueeOverlayRef = reactHostPort.useRef(onMarqueeOverlay);
	const onLivePreselectRef = reactHostPort.useRef(onLivePreselect);
	const selectionMethodRef = reactHostPort.useRef(selectionMethod);
	const selectedNodeIdsRef = reactHostPort.useRef(selectedNodeIds);
	resolveMarqueeHitsRef.current = resolveMarqueeHits;
	commitSelectionRef.current = commitSelection;
	onMarqueeOverlayRef.current = onMarqueeOverlay;
	onLivePreselectRef.current = onLivePreselect;
	selectionMethodRef.current = selectionMethod;
	selectedNodeIdsRef.current = selectedNodeIds;

	const clientToLocal = reactHostPort.useCallback(
		(clientX: number, clientY: number) => {
			const host = containerRef.current;
			if (!host) return { x: clientX, y: clientY };
			const rect = host.getBoundingClientRect();
			return { x: clientX - rect.left, y: clientY - rect.top };
		},
		[containerRef],
	);

	reactHostPort.useEffect(() => {
		const canvas = gl.domElement;
		if (!canvas) return;
		const resetGesture = () => {
			marqueeRef.current = { tracking: false, active: false, start: { x: 0, y: 0 }, points: [], initial: [] };
			onMarqueeOverlayRef.current(null);
			onLivePreselectRef.current({ ids: [], removedIds: [] });
		};
		const gumballBlocksSelection = () => gumballPointerConsumesCanvasEventRef.current || proceduralGumballDragActiveRef.current;
		const onPointerDown = (event: PointerEvent) => {
			if (event.button !== 0) return;
			if (gumballBlocksSelection()) return;
			if ((event.target as HTMLElement | null)?.closest("[data-world-projection-switch]")) return;
			const point = clientToLocal(event.clientX, event.clientY);
			marqueeRef.current = { tracking: true, active: false, start: point, points: [point], initial: [...selectedNodeIdsRef.current] };
			onMarqueeOverlayRef.current(null);
			onLivePreselectRef.current({ ids: [], removedIds: [] });
		};
		const onPointerMove = (event: PointerEvent) => {
			if (gumballBlocksSelection()) {
				if (marqueeRef.current.tracking) resetGesture();
				return;
			}
			if (!marqueeRef.current.tracking) return;
			const point = clientToLocal(event.clientX, event.clientY);
			const start = marqueeRef.current.start;
			const distance = Math.hypot(point.x - start.x, point.y - start.y);
			if (!marqueeRef.current.active && distance < PROCEDURAL_PREVIEW_MARQUEE_THRESHOLD_PX) return;
			marqueeRef.current.active = true;
			const points = selectionMethodRef.current === "lasso" ? [...marqueeRef.current.points, point] : [start, point];
			marqueeRef.current.points = points;
			const coverage = marqueeCoverageFromGesture({ method: selectionMethodRef.current, startX: start.x, endX: point.x, path: points });
			if (selectionMethodRef.current === "lasso" && points.length >= 3) {
				onMarqueeOverlayRef.current({ coverage, shape: "polygon", points });
			} else {
				const rect = screenRectFromPoints(points);
				if (rect) onMarqueeOverlayRef.current({ coverage, shape: "rect", rect });
			}
			const mode = marqueeModeFromModifiers(event);
			const hits = resolveMarqueeHitsRef.current(points, coverage === "partial");
			const merged = selectionMergeIds(mode, marqueeRef.current.initial, hits);
			const removed = marqueeRef.current.initial.filter((id) => !merged.includes(id));
			onLivePreselectRef.current({ ids: merged.filter((id) => !marqueeRef.current.initial.includes(id)), removedIds: removed });
			invalidate();
		};
		const onPointerUp = (event: PointerEvent) => {
			if (gumballBlocksSelection()) {
				if (marqueeRef.current.tracking) resetGesture();
				return;
			}
			if (!marqueeRef.current.tracking) return;
			const point = clientToLocal(event.clientX, event.clientY);
			const start = marqueeRef.current.start;
			const distance = Math.hypot(point.x - start.x, point.y - start.y);
			const mode = marqueeModeFromModifiers(event);
			if (marqueeRef.current.active && distance >= PROCEDURAL_PREVIEW_MARQUEE_THRESHOLD_PX) {
				const points = selectionMethodRef.current === "lasso" ? [...marqueeRef.current.points, point] : [start, point];
				const coverage = marqueeCoverageFromGesture({ method: selectionMethodRef.current, startX: start.x, endX: point.x, path: points });
				const hits = resolveMarqueeHitsRef.current(points, coverage === "partial");
				const next = selectionMergeIds(mode, marqueeRef.current.initial, hits);
				commitSelectionRef.current(next, mode);
			}
			resetGesture();
			invalidate();
		};
		const bindings = new WorldEventBindingController();
		bindings.listen(canvas, "pointerdown", onPointerDown as EventListener, true);
		bindings.listen(window, "pointermove", onPointerMove as EventListener);
		bindings.listen(window, "pointerup", onPointerUp as EventListener, true);
		bindings.listen(window, "pointercancel", onPointerUp as EventListener, true);
		return () => bindings.dispose();
	}, [clientToLocal, gl, invalidate]);

	return null;
}

export function ProceduralPreview({
	items,
	selectedNodeIds = [],
	selectedChannels = [],
	preselectNodeIds = [],
	preselectRemovedNodeIds = [],
	hoveredNodeId = null,
	hoveredChannel = null,
	hoveredGeometryTargets = [],
	selectedGeometryTargets = [],
	fixtureEdges = [],
	previewOffNodeIds = [],
	showMode = "everything",
	selectionMode = "default",
	selectionMethod = "rectangle",
	transformGranularity = "full",
	onGumballTransform,
	gumballActiveWidgetIds = [],
	onHover,
	onSelect,
	onSelectionChange,
	kernel,
	tolerance = 0.02,
	className,
}: ProceduralPreviewProps): ReactNode {
	const containerRef = reactHostPort.useRef<HTMLDivElement>(null);
	const cameraRef = reactHostPort.useRef<THREE.Camera | null>(null);
	const sizeRef = reactHostPort.useRef({ width: 1, height: 1 });
	const lodRef = reactHostPort.useRef(DEFAULT_MANUAL_LOD);
	const [camera, setCamera] = reactHostPort.useState<WorldCameraState>(PROCEDURAL_PREVIEW_DEFAULT_CAMERA);
	const [cameraSeed, setCameraSeed] = reactHostPort.useState(0);
	const cameraSeedKey = proceduralPreviewCameraSeed(cameraSeed);
	const projection = camera.projection ?? "perspective";
	const [marqueeOverlay, setMarqueeOverlay] = reactHostPort.useState<{
		coverage: SelectionMarqueeCoverage;
		shape: "rect" | "polygon";
		rect?: { x: number; y: number; width: number; height: number };
		points?: readonly { x: number; y: number }[];
	} | null>(null);
	const [livePreselect, setLivePreselect] = reactHostPort.useState<{ ids: string[]; removedIds: string[] }>({ ids: [], removedIds: [] });
	const [gumballInteractionWidgetId, setGumballInteractionWidgetId] = reactHostPort.useState<string | null>(null);
	const gumballHighlightIds = reactHostPort.useMemo(() => new Set(gumballActiveWidgetIds), [gumballActiveWidgetIds]);
	const gumballDragActive = gumballInteractionWidgetId !== null;
	const [canvasBackground, setCanvasBackground] = reactHostPort.useState(() => resolveSemanticColorHex("--canvas", "light-8-9"));
	const [resolvedKernel, setResolvedKernel] = reactHostPort.useState<BrepWasmBridge | null>(kernel ?? null);

	reactHostPort.useEffect(() => {
		if (kernel) {
			setResolvedKernel(kernel);
			return;
		}
		let cancelled = false;
		void ensureProceduralBrepBridge().then((bridge) => {
			if (!cancelled) setResolvedKernel(bridge);
		});
		return () => {
			cancelled = true;
		};
	}, [kernel]);

	reactHostPort.useEffect(() => {
		if (typeof document === "undefined") return;
		const sync = () => {
			clearColorResolveCache();
			setCanvasBackground(resolveSemanticColorHex("--canvas", "light-8-9"));
		};
		sync();
		const obs = new MutationObserver(sync);
		obs.observe(document.documentElement, { attributes: true, attributeFilter: ["class", "style", "data-theme", "data-ui-theme"] });
		return () => obs.disconnect();
	}, []);

	const visibleItems = reactHostPort.useMemo(
		() =>
			filterVisiblePreviewItems(items, {
				showMode,
				selectedNodeIds,
				selectedChannels,
				selectedGeometryTargets,
				edges: fixtureEdges,
				hoveredNodeId,
				hoveredChannel,
			}),
		[fixtureEdges, hoveredChannel, hoveredNodeId, items, selectedChannels, selectedGeometryTargets, selectedNodeIds, showMode],
	);

	const gumballAnchorId = reactHostPort.useMemo(() => {
		if (!onGumballTransform) return null;
		if (gumballDragActive) {
			const transformGeometry = gumballActiveWidgetIds.find((id) =>
				items.some((entry) => entry.widgetId === id && entry.kind === "geometry"),
			);
			if (transformGeometry) return transformGeometry;
			if (gumballInteractionWidgetId) return gumballInteractionWidgetId;
		}
		if (selectedNodeIds.length !== 1) return null;
		return selectedNodeIds[0]!;
	}, [gumballActiveWidgetIds, gumballDragActive, gumballInteractionWidgetId, items, onGumballTransform, selectedNodeIds]);

	const gumballItem = reactHostPort.useMemo(() => {
		if (!gumballAnchorId) return null;
		return (
			visibleItems.find(
				(entry): entry is Extract<ProceduralPreviewItem, { kind: "geometry" }> => entry.widgetId === gumballAnchorId && entry.kind === "geometry",
			) ?? null
		);
	}, [gumballAnchorId, visibleItems]);

	const effectiveSelected = reactHostPort.useMemo(() => new Set(selectedNodeIds), [selectedNodeIds]);
	const effectivePreselect = reactHostPort.useMemo(() => new Set(livePreselect.ids.length ? livePreselect.ids : preselectNodeIds), [livePreselect.ids, preselectNodeIds]);
	const effectivePreselectRemoved = reactHostPort.useMemo(
		() => new Set(livePreselect.removedIds.length ? livePreselect.removedIds : preselectRemovedNodeIds),
		[livePreselect.removedIds, preselectRemovedNodeIds],
	);

	const handleCamera = reactHostPort.useCallback((camera: THREE.Camera, size: { width: number; height: number }) => {
		cameraRef.current = camera;
		sizeRef.current = { width: size.width, height: size.height };
	}, []);

	const screenBoundsForItem = reactHostPort.useCallback(
		(item: ProceduralPreviewItem): ScreenBounds | null => {
			const camera = cameraRef.current;
			if (!camera) return null;
			if (!resolvedKernel) return null;
			const bounds = worldBoundsForPreviewItem(item, resolvedKernel);
			if (!bounds) return null;
			return projectWorldBoundsToScreen(bounds, camera, sizeRef.current.width, sizeRef.current.height);
		},
		[resolvedKernel],
	);

	const resolveMarqueeHits = reactHostPort.useCallback(
		(points: readonly { x: number; y: number }[], crossing: boolean): string[] => {
			const marqueeRect = screenRectFromPoints(points);
			if (!marqueeRect) return [];
			const hits: string[] = [];
			for (const entry of visibleItems) {
				const bounds = screenBoundsForItem(entry);
				if (!bounds) continue;
				const target = { x: bounds.left, y: bounds.top, width: bounds.right - bounds.left, height: bounds.bottom - bounds.top };
				const marquee = { x: marqueeRect.x, y: marqueeRect.y, width: marqueeRect.width, height: marqueeRect.height };
				const contained = screenRectContainsRect(marquee, target);
				const intersects = screenRectIntersectsRect(marquee, target);
				if (crossing ? intersects : contained) hits.push(entry.widgetId);
			}
			return hits;
		},
		[screenBoundsForItem, visibleItems],
	);

	const commitSelection = reactHostPort.useCallback(
		(ids: readonly string[], mode: ProceduralSelectionMode) => {
			onSelectionChange?.(ids, mode);
		},
		[onSelectionChange],
	);

	const onPick = reactHostPort.useCallback(
		(channel: ProceduralChannelRef, mode: ProceduralSelectionMode) => {
			onSelect?.(channel);
			const next = selectionMergeIds(mode, selectedNodeIds, [channel.widgetId]);
			commitSelection(next, mode);
		},
		[commitSelection, onSelect, selectedNodeIds],
	);

	const resolvedHoverTargets = reactHostPort.useMemo(() => {
		if (hoveredGeometryTargets.length > 0) return hoveredGeometryTargets;
		if (hoveredChannel?.direction === "out") return [hoveredChannel];
		if (hoveredNodeId) {
			return items
				.filter((entry) => entry.widgetId === hoveredNodeId && entry.direction === "out")
				.map((entry) => previewItemChannel(entry));
		}
		return [];
	}, [hoveredChannel, hoveredGeometryTargets, hoveredNodeId, items]);

	const resolvedSelectedTargets = reactHostPort.useMemo(() => {
		if (selectedGeometryTargets.length > 0) return selectedGeometryTargets;
		return selectedChannels.filter((channel) => channel.direction === "out");
	}, [selectedChannels, selectedGeometryTargets]);

	const hasChannelSelection = selectedChannels.length > 0 || selectedGeometryTargets.length > 0;

	const onProjectionChange = reactHostPort.useCallback((nextProjection: OrbitCameraProjection) => {
		setCamera((prev) => applyOrbitProjectionToCameraState(prev, nextProjection));
		setCameraSeed((seed) => seed + 1);
	}, []);

	const onViewportGizmoCameraChange = reactHostPort.useCallback((next: WorldCameraState) => {
		setCamera(next);
		setCameraSeed((seed) => seed + 1);
	}, []);

	const sceneInvalidationToken = `${resolvedKernel ? 1 : 0}:${visibleItems.length}:${cameraSeedKey}`;

	if (!resolvedKernel) return null;

	return (
		<div ref={containerRef} className={cn("absolute inset-0", canvasHostRootClass, className)}>
			<WorldCanvas
				className="h-full w-full"
				frameloop={gumballDragActive ? "always" : "demand"}
				background={canvasBackground}
				overlay={<WorldOrbitProjectionSwitch projection={projection} onProjectionChange={onProjectionChange} />}
			>
				<WorldLodBridge
					lodRef={lodRef}
					distanceReference={100}
					gridFactor={DEFAULT_LOD_GRID_FACTOR}
					gridSnapEnabled={false}
					showLodGrid
					automaticLod
					depthVariableLod={false}
					manualLod={DEFAULT_MANUAL_LOD}
					gridDatum={[0, 0, 0]}
				>
					<WorldOrbitViewSnapGateProvider>
						<WorldOrbitCameraViewRig state={camera} seedKey={cameraSeedKey} perspectiveFov={45} />
						<WorldOrbitGated controlsKey={cameraSeedKey} projection={projection} zoom={camera.zoom} />
						<WorldOrbitViewControls onCameraChange={onViewportGizmoCameraChange} />
						<ProceduralPreviewCameraBridge onCamera={handleCamera} />
						<ProceduralPreviewSceneInvalidator token={sceneInvalidationToken} />
						<ProceduralPreviewMarqueeBridge
							containerRef={containerRef}
							selectionMethod={selectionMethod}
							selectedNodeIds={selectedNodeIds}
							resolveMarqueeHits={resolveMarqueeHits}
							commitSelection={commitSelection}
							onMarqueeOverlay={setMarqueeOverlay}
							onLivePreselect={setLivePreselect}
						/>
						<WorldCameraInvalidator />
						<ambientLight intensity={0.45} />
						<directionalLight position={[12, 18, 10]} intensity={1.1} />
						<WorldLayer order={10} name="procedural.preview">
							{visibleItems.map((entry) => {
								const interactionHighlighted = gumballHighlightIds.has(entry.widgetId) || gumballInteractionWidgetId === entry.widgetId;
								const locked = gumballDragActive && !interactionHighlighted;
								const chrome: PreviewLayerChrome = {
									selected:
										geometryTargetMatches(entry, resolvedSelectedTargets) ||
										(!hasChannelSelection && (effectiveSelected.has(entry.widgetId) || effectivePreselect.has(entry.widgetId))),
									highlighted: effectivePreselectRemoved.has(entry.widgetId),
									hovered: geometryTargetMatches(entry, resolvedHoverTargets),
									previewOff: previewOffForItem(showMode, entry.widgetId, previewOffNodeIds),
									locked,
									interactionHighlighted,
									pickEnabled: !gumballDragActive,
									onHover,
									onPick,
								};
								return <BrepPreviewLayer key={previewItemKey(entry)} item={entry} kernel={resolvedKernel} tolerance={tolerance} {...chrome} />;
							})}
							{gumballItem ? (
								<ProceduralPreviewGumball
									key="procedural-gumball"
									item={gumballItem}
									kernel={resolvedKernel}
									transformGranularity={transformGranularity}
									onGumballTransform={onGumballTransform}
									onInteractionChange={setGumballInteractionWidgetId}
								/>
							) : null}
						</WorldLayer>
					</WorldOrbitViewSnapGateProvider>
				</WorldLodBridge>
			</WorldCanvas>
			{marqueeOverlay?.shape === "rect" && marqueeOverlay.rect ? (
				<SelectionMarquee coverage={marqueeOverlay.coverage} shape="rect" rect={marqueeOverlay.rect} />
			) : null}
			{marqueeOverlay?.shape === "polygon" && marqueeOverlay.points ? (
				<SelectionMarquee coverage={marqueeOverlay.coverage} shape="polygon" points={marqueeOverlay.points} />
			) : null}
		</div>
	);
}

/** @emoji 🔍 Collects geometry, point, and vector preview items from channel-structured flow eval JSON. */
export function extractChannelPreviewItems(channelJson: string): ProceduralPreviewItem[] {
	const items: ProceduralPreviewItem[] = [];
	try {
		const parsed = JSON.parse(channelJson) as unknown;
		if (!parsed || typeof parsed !== "object" || Array.isArray(parsed)) return items;
		if ("error" in (parsed as Record<string, unknown>) && Object.keys(parsed as object).length === 1) return items;
		for (const [widgetId, entry] of Object.entries(parsed as Record<string, unknown>)) {
			if (!entry || typeof entry !== "object" || Array.isArray(entry)) continue;
			const channels = entry as { in?: Record<string, unknown>; out?: Record<string, unknown> };
			for (const [port, value] of Object.entries(channels.in ?? {})) {
				items.push(...previewItemsFromChannelValue(widgetId, port, "in", value));
			}
			for (const [port, value] of Object.entries(channels.out ?? {})) {
				items.push(...previewItemsFromChannelValue(widgetId, port, "out", value));
			}
		}
	} catch {
		/* ignore */
	}
	return items;
}

/** @emoji 🔍 Back-compat alias for channel preview extraction. */
export const extractPreviewItems = extractChannelPreviewItems;
// #endregion 🔖ProceduralPreview

// #region 🔖ProceduralEditor
export interface ProceduralFlowEditorProps {
	readonly fixtureJson?: string;
	readonly className?: string;
	readonly extensionHost?: ProceduralExtensionHost;
	readonly reorganize?: FlowReorganizeRequest;
	readonly extensionRevision?: number;
	readonly onPreviewText?: (text: string) => void;
	readonly onEvalOutputs?: (outputsJson: string, previewMeshes?: Readonly<Record<string, unknown>>) => void;
	readonly onCatalogueReady?: (sections: readonly CatalogueSection[]) => void;
	readonly onFixtureChange?: (fixtureJson: string) => void;
	readonly onSelectionChange?: (ids: readonly string[]) => void;
	readonly onPreselectChange?: (snapshot: { readonly ids: readonly string[]; readonly removedIds: readonly string[] }) => void;
	readonly onHoverChange?: (id: string | null) => void;
	readonly onChannelHoverChange?: (channel: ProceduralChannelRef | null) => void;
	readonly onSelectedChannelsChange?: (channels: readonly ProceduralChannelRef[]) => void;
	readonly selectedNodeIds?: readonly string[];
	readonly preselectNodeIds?: readonly string[];
	readonly preselectRemovedNodeIds?: readonly string[];
	readonly hoveredNodeId?: string | null;
	readonly hoveredChannel?: ProceduralChannelRef | null;
	readonly selectedChannels?: readonly ProceduralChannelRef[];
	readonly previewOffNodeIds?: readonly string[];
	readonly selectionMode?: ProceduralSelectionMode;
	readonly selectionMethod?: ProceduralSelectionMethod;
	readonly contextMenu?: (ctx: FlowCanvasContextMenuContext) => readonly ContextMenuItem[];
	readonly commandRequest?: FlowCanvasCommandRequest;
	readonly onPreviewOffChange?: (ids: readonly string[]) => void;
	readonly automaticLod?: boolean;
	readonly lod?: DagDrawLodKind;
	readonly onLodChange?: (lod: DagDrawLodKind) => void;
	readonly proximityDistance?: number;
}

export function ProceduralFlowEditor({
	fixtureJson,
	className,
	extensionHost = proceduralExtensionHost,
	reorganize,
	extensionRevision = 0,
	onPreviewText,
	onEvalOutputs,
	onCatalogueReady,
	onFixtureChange,
	onSelectionChange,
	onPreselectChange,
	onHoverChange,
	onChannelHoverChange,
	onSelectedChannelsChange,
	selectedNodeIds,
	preselectNodeIds,
	preselectRemovedNodeIds,
	hoveredNodeId,
	hoveredChannel,
	selectedChannels,
	previewOffNodeIds,
	selectionMode,
	selectionMethod,
	contextMenu,
	commandRequest,
	onPreviewOffChange,
	automaticLod,
	lod,
	onLodChange,
	proximityDistance,
}: ProceduralFlowEditorProps): ReactNode {
	const hostRef = reactHostPort.useRef(extensionHost);

	reactHostPort.useEffect(() => {
		hostRef.current = extensionHost;
		void extensionHost.activateDefaults();
	}, [extensionHost]);

	return (
		<FlowCanvas
			fixtureJson={fixtureJson}
			store={PROCEDURAL_FLOW_STORE}
			fixtureDragDrop
			reorganize={reorganize}
			extensionRevision={extensionRevision}
			extensionHost={extensionHost}
			onPreviewText={onPreviewText}
			onEvalOutputs={onEvalOutputs}
			onCatalogueReady={onCatalogueReady}
			onFixtureChange={onFixtureChange}
			onSelectionChange={onSelectionChange}
			onPreselectChange={onPreselectChange}
			onHoverChange={onHoverChange}
			onChannelHoverChange={onChannelHoverChange}
			onSelectedChannelsChange={onSelectedChannelsChange}
			selectedNodeIds={selectedNodeIds}
			preselectNodeIds={preselectNodeIds}
			preselectRemovedNodeIds={preselectRemovedNodeIds}
			hoveredNodeId={hoveredNodeId}
			hoveredChannel={hoveredChannel}
			selectedChannels={selectedChannels}
			previewOffNodeIds={previewOffNodeIds}
			selectionMode={selectionMode}
			selectionMethod={selectionMethod}
			contextMenu={contextMenu}
			commandRequest={commandRequest}
			onPreviewOffChange={onPreviewOffChange}
			automaticLod={automaticLod}
			lod={lod}
			onLodChange={onLodChange}
			proximityDistance={proximityDistance}
			className={className ?? "h-full w-full"}
		/>
	);
}

export { createFlowEvalBridge, type FlowModuleCommandV1, type FlowReorganizeRequest };
// #endregion 🔖ProceduralEditor

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it, beforeAll } = import.meta.vitest;
	const { createRoot } = await import("react-dom/client");
	const { act } = await import("react");
	const {
		ProceduralExtensionHost,
		ProceduralPreview,
		PROCEDURAL_PREVIEW_DEFAULT_CAMERA,
		extractPreviewItems,
		proceduralPreviewCameraSeed,
	} = await import("./index.tsx");
	const { applyOrbitProjectionToCameraState } = await import("@infinite/world/r3f");
	const { ensureBrepWasmLoaded, isRenderableMeshTransfer } = await import("@geometry/brep/js");

	function numberDict(value: number) {
		return { $schema: "number", value };
	}

	function geometryDict(handle: string) {
		return { $schema: "geometry", handle, kind: "solid" };
	}

	function channelPayload<T extends Record<string, unknown>>(out: T, channel: string): T {
		const payload = out[channel];
		return (typeof payload === "object" && payload !== null ? payload : out) as T;
	}

	describe("@procedural/react", () => {
		let host: ProceduralExtensionHost;
		let bridge: Awaited<ReturnType<typeof host.getBrepBridge>>;

		beforeAll(async () => {
			await ensureBrepWasmLoaded();
			host = new ProceduralExtensionHost();
			await host.activateDefaults();
			bridge = host.getBrepBridge();
			if (typeof globalThis.ResizeObserver === "undefined") {
				globalThis.ResizeObserver = class {
					observe() {}
					unobserve() {}
					disconnect() {}
				} as typeof ResizeObserver;
			}
			(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;
		});

		it("brep.prim3d.box evaluates to geometry handle", () => {
			const out = JSON.parse(host.evaluate("brep.prim3d.box", JSON.stringify({}))) as { solid?: { handle?: string }; error?: string };
			expect(out.error).toBeUndefined();
			expect(channelPayload(out, "solid").handle).toMatch(/^solid-/);
		});

		it("box fillet translate chain tessellates", async () => {
			const box = channelPayload(
				JSON.parse(host.evaluate("brep.prim3d.box", JSON.stringify({}))) as { solid: { handle: string } },
				"solid",
			);
			const fillet = channelPayload(
				JSON.parse(
					host.evaluate(
						"brep.solid.fillet",
						JSON.stringify({ geometry: geometryDict(box.handle), radius: numberDict(0.1) }),
					),
				) as { solid: { handle: string } },
				"solid",
			);
			const moved = channelPayload(
				JSON.parse(
					host.evaluate(
						"brep.xform.translate",
						JSON.stringify({
							geometry: geometryDict(fillet.handle),
							offset: { $schema: "vector", x: 1, y: 0, z: 0 },
						}),
					),
				) as { geometry: { handle: string } },
				"geometry",
			);
			const mesh = await bridge.tessellateGeometry(moved.handle as import("@geometry/brep/js").GeometryRef, 0.05);
			expect(isRenderableMeshTransfer(mesh)).toBe(true);
		});

		it("brep.bool.cut evaluates without error", () => {
			const base = channelPayload(
				JSON.parse(host.evaluate("brep.prim3d.box", JSON.stringify({ width: numberDict(2), depth: numberDict(2), height: numberDict(2) }))) as {
					solid: { handle: string };
				},
				"solid",
			);
			const tool = channelPayload(
				JSON.parse(host.evaluate("brep.prim3d.box", JSON.stringify({ width: numberDict(1), depth: numberDict(1), height: numberDict(1) }))) as {
					solid: { handle: string };
				},
				"solid",
			);
			const cut = channelPayload(
				JSON.parse(
					host.evaluate("brep.bool.cut", JSON.stringify({ a: geometryDict(base.handle), b: geometryDict(tool.handle) })),
				) as { solid?: { handle?: string }; error?: string },
				"solid",
			);
			expect((cut as { error?: string }).error).toBeUndefined();
			expect(cut.handle).toMatch(/^solid-/);
		});

		it("extractChannelPreviewItems collects geometry outputs", () => {
			const box = channelPayload(
				JSON.parse(host.evaluate("brep.prim3d.box", JSON.stringify({}))) as { solid: { handle: string } },
				"solid",
			);
			const items = extractPreviewItems(
				JSON.stringify({
					box: { in: {}, out: { solid: geometryDict(box.handle) } },
				}),
			);
			expect(items).toContainEqual({ widgetId: "box", port: "solid", direction: "out", kind: "geometry", handle: box.handle });
		});

		it("extractChannelPreviewItems collects schema point and vector outputs", () => {
			const pointNode = channelPayload(
				JSON.parse(host.evaluate("math.point", JSON.stringify({ x: numberDict(2), y: numberDict(0), z: numberDict(1) }))) as {
					point: { $schema?: string; x?: number; y?: number; z?: number };
				},
				"point",
			);
			const vectorNode = channelPayload(
				JSON.parse(host.evaluate("math.vector", JSON.stringify({ x: numberDict(0), y: numberDict(3), z: numberDict(0) }))) as {
					vector: { $schema?: string; x?: number; y?: number; z?: number };
				},
				"vector",
			);
			const items = extractPreviewItems(
				JSON.stringify({
					pt: { in: {}, out: { point: pointNode } },
					vec: { in: {}, out: { vector: vectorNode } },
				}),
			);
			expect(items).toContainEqual({ widgetId: "pt", port: "point", direction: "out", kind: "point", position: [2, 0, 1] });
			expect(items).toContainEqual({ widgetId: "vec", port: "vector", direction: "out", kind: "vector", directionVec: [0, 3, 0] });
		});

		it("previewOffForItem is ignored in show selected mode", async () => {
			const { previewOffForItem } = await import("./index.tsx");
			expect(previewOffForItem("everything", "sphere", ["sphere"])).toBe(true);
			expect(previewOffForItem("selected", "sphere", ["sphere"])).toBe(false);
		});

		it("filterVisiblePreviewItems resolves input channel selection to upstream geometry", async () => {
			const { filterVisiblePreviewItems } = await import("./index.tsx");
			const sphereHandle = "solid-sphere";
			const torusHandle = "solid-torus";
			const cutHandle = "solid-cut";
			const items = [
				{ widgetId: "sphere", port: "solid", direction: "out" as const, kind: "geometry" as const, handle: sphereHandle },
				{ widgetId: "torus", port: "solid", direction: "out" as const, kind: "geometry" as const, handle: torusHandle },
				{ widgetId: "cut", port: "solid", direction: "out" as const, kind: "geometry" as const, handle: cutHandle },
			];
			const edges = [
				{ source: "sphere:solid", target: "cut:a" },
				{ source: "torus:solid", target: "cut:b" },
			];
			const visibleForA = filterVisiblePreviewItems(items, {
				showMode: "selected",
				selectedNodeIds: ["cut"],
				selectedChannels: [{ widgetId: "cut", port: "a", direction: "in" }],
				edges,
				hoveredNodeId: null,
				hoveredChannel: null,
			});
			expect(visibleForA).toEqual([items[0]]);
			const visibleForB = filterVisiblePreviewItems(items, {
				showMode: "selected",
				selectedNodeIds: ["cut"],
				selectedChannels: [{ widgetId: "cut", port: "b", direction: "in" }],
				edges,
				hoveredNodeId: null,
				hoveredChannel: null,
			});
			expect(visibleForB).toEqual([items[1]]);
			const visibleForOut = filterVisiblePreviewItems(items, {
				showMode: "selected",
				selectedNodeIds: ["cut"],
				selectedChannels: [{ widgetId: "cut", port: "solid", direction: "out" }],
				edges,
				hoveredNodeId: null,
				hoveredChannel: null,
			});
			expect(visibleForOut).toEqual([items[2]]);
		});

		it("procedural host exposes brep operators with capability-typed ports", async () => {
			const sections = host.catalogueSections();
			const brep = sections.find((section) => section.id === "brep");
			expect(brep?.groups?.some((group) => group.title === "Primitives 3D")).toBe(true);
			expect(brep?.groups?.some((group) => group.title === "Schemas")).toBe(true);
			const kinds = JSON.parse(host.kindInfosJson()) as Array<{ id: string; inputs: Array<{ name: string; operators: string[] }> }>;
			const box = kinds.find((item) => item.id === "brep.prim3d.box");
			expect(box?.inputs.some((port) => port.name === "width")).toBe(true);
			expect(box?.inputs[0]?.operators?.length).toBeGreaterThan(0);
			expect(kinds.some((item) => item.id === "brep.geometry")).toBe(true);
			expect(kinds.some((item) => item.id === "brep.brep")).toBe(true);
			expect(kinds.some((item) => item.id === "math.vector")).toBe(true);
		});

		it("procedural host exposes bim building model operators", async () => {
			const sections = host.catalogueSections();
			const bim = sections.find((section) => section.id === "bim");
			expect(bim?.groups?.some((group) => group.title === "Elements")).toBe(true);
			const wall = channelPayload(
				JSON.parse(host.evaluate("bim.element.wall", JSON.stringify({}))) as { wall: { $schema?: string } },
				"wall",
			);
			expect(wall.$schema).toBe("wall");
			const slab = channelPayload(
				JSON.parse(
					host.evaluate(
						"bim.element.slab",
						JSON.stringify({
							width: { $schema: "number", value: 10 },
							depth: { $schema: "number", value: 8 },
							thickness: { $schema: "number", value: 0.25 },
						}),
					),
				) as { slab: { $schema?: string } },
				"slab",
			);
			const story = channelPayload(
				JSON.parse(
					host.evaluate(
						"bim.assemble.story",
						JSON.stringify({
							height: { $schema: "number", value: 3 },
							slab,
							elements: {},
						}),
					),
				) as { story: { $schema?: string } },
				"story",
			);
			const building = channelPayload(
				JSON.parse(
					host.evaluate(
						"bim.assemble.building",
						JSON.stringify({
							name: { $schema: "text", value: "Tower" },
							stories: { 0: story },
						}),
					),
				) as { building: { $schema?: string; name?: string } },
				"building",
			);
			const area = channelPayload(
				JSON.parse(host.evaluate("bim.measure.floorArea", JSON.stringify({ building }))) as {
					floorArea: { $schema?: string; value?: number };
				},
				"floorArea",
			);
			expect(building.$schema).toBe("building");
			expect(area.$schema).toBe("number");
			expect(area.value).toBe(80);
		});

		it("procedural preview mounts the infinite-world viewport stack", async () => {
			const mount = document.createElement("div");
			document.body.appendChild(mount);
			const root = createRoot(mount);
			await act(async () => {
				root.render(<ProceduralPreview items={[]} kernel={bridge} />);
			});
			expect(mount.querySelector("[data-world-projection-switch]")).not.toBeNull();
			root.unmount();
			mount.remove();
		});

		it("procedural preview default camera is z-up perspective", () => {
			expect(PROCEDURAL_PREVIEW_DEFAULT_CAMERA).toMatchObject({
				position: [8, 8, 6],
				target: [0, 0, 0],
				projection: "perspective",
			});
			expect(proceduralPreviewCameraSeed(0)).toContain("procedural-preview-camera");
			const next = applyOrbitProjectionToCameraState(PROCEDURAL_PREVIEW_DEFAULT_CAMERA, "orthographic");
			expect(next.projection).toBe("orthographic");
		});
	});
}
// #endregion 🧪Tests
