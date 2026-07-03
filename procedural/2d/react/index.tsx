// #region 🧲Header
/// <reference types="vite/client" />
/// <reference types="vitest/importMeta" />
/** @emoji 🖊️ `@semio-tech/procedural-2d-react` — flow-based draw editor with infinite-cavas preview. */
// #endregion 🧲Header

// #region 🔌Adapters
import {
	FlowCanvas,
	FlowExtensionHost,
	createEphemeralFlowStore,
	type CatalogueSection,
	type FlowCanvasCommandRequest,
	type FlowCanvasContextMenuContext,
	type FlowFixture,
	type FlowReorganizeRequest,
} from "@semio-tech/flow-react";
import {
	canvasDrawingPngExportPort,
	createDefaultDrawingWasmBridge,
	ensureDrawingWasmLoaded,
	isDrawingRef,
	paintDrawingScene,
	type DrawingExportBridge,
	type DrawingScene,
} from "@semio-tech/kernel-2d-js";
import { CavasEventBindingController } from "@semio-tech/infinite-cavas-react-renderer";
import type { ContextMenuItem } from "@semio-tech/ui-react";
import {
	SelectionMarquee,
	canvasHostRootClass,
	cn,
	marqueeCoverageFromGesture,
	marqueeModeFromModifiers,
	reactHostPort,
	screenRectContainsRect,
	screenRectFromPoints,
	screenRectIntersectsRect,
	selectionMergeIds,
	type SelectionMarqueeCoverage,
	type SelectionMergeMode,
} from "@semio-tech/ui-react";
import { clearColorResolveCache, resolveSemanticColorHex } from "@semio-tech/ui-styling";
import { type ReactNode } from "react";
// #endregion 🔌Adapters

// #region 🔖DrawFlowModule
if (!import.meta.env.VITEST && typeof window !== "undefined") {
	await ensureDrawingWasmLoaded();
}

let procedural2dBridgePromise: Promise<DrawingExportBridge> | null = null;

export async function ensureProcedural2dDrawingBridge(): Promise<DrawingExportBridge> {
	procedural2dBridgePromise ??= createDefaultDrawingWasmBridge();
	return procedural2dBridgePromise;
}

/** @emoji 🔌 Flow extension host with `@semio-tech/flow-module-draw` loaded through the normal module path. */
export class Procedural2dExtensionHost extends FlowExtensionHost {
	private bridge: DrawingExportBridge | null = null;

	async activateDefaults(): Promise<void> {
		await super.activateDefaults();
		if (!this.isActive("draw")) {
			await this.activate("draw");
		}
		this.bridge = await ensureProcedural2dDrawingBridge();
	}

	getDrawingBridge(): DrawingExportBridge {
		if (!this.bridge) throw new Error("draw wasm not ready");
		return this.bridge;
	}

	tryGetDrawingBridge(): DrawingExportBridge | null {
		return this.bridge;
	}
}

export const procedural2dExtensionHost = new Procedural2dExtensionHost();

/** @emoji 🔌 Resolves the drawing WASM bridge after extension defaults are activated. */
export function useProcedural2dDrawingBridge(host: Procedural2dExtensionHost = procedural2dExtensionHost): DrawingExportBridge | null {
	const [bridge, setBridge] = reactHostPort.useState<DrawingExportBridge | null>(host.tryGetDrawingBridge());
	reactHostPort.useEffect(() => {
		let cancelled = false;
		void host.activateDefaults().then(() => {
			if (cancelled) return;
			setBridge(host.tryGetDrawingBridge());
		});
		return () => {
			cancelled = true;
		};
	}, [host]);
	return bridge;
}
// #endregion 🔖DrawFlowModule

// #region 🔖Fixture
export const PROCEDURAL_DEFAULT_FIXTURE: FlowFixture = {
	schema: "flow.fixture",
	camera: { x: 0, y: 0, zoom: 1 },
	widgets: [
		{ kind: "neuron", id: "rect", neuronKind: "draw.shape.rect" },
		{ kind: "neuron", id: "fill", neuronKind: "draw.style.fill" },
		{ kind: "outputPreview", id: "preview" },
	],
	synapses: [
		{ id: "s1", from: "rect", to: "fill", fromPort: "draw.drawing", toPort: "drawing" },
		{ id: "s2", from: "fill", to: "preview", fromPort: "draw.drawing", toPort: "" },
	],
};

export function proceduralFixtureToJson(fixture: FlowFixture = PROCEDURAL_DEFAULT_FIXTURE): string {
	return JSON.stringify(fixture);
}

const PROCEDURAL_FLOW_STORE = createEphemeralFlowStore();
// #endregion 🔖Fixture

// #region 🔖PreviewTypes
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
			readonly kind: "drawing";
			readonly handle: string;
			readonly scene?: DrawingScene;
	  }
	| { readonly widgetId: string; readonly port: string; readonly direction: ProceduralChannelDirection; readonly kind: "point"; readonly position: readonly [number, number] };

export interface ProceduralFixtureEdge {
	readonly source: string;
	readonly target: string;
}

export type ProceduralPreviewShowMode = "everything" | "selected";
export type ProceduralSelectionMode = SelectionMergeMode;
export type ProceduralSelectionMethod = "rectangle" | "lasso";

const DRAWING_REF_PATTERN = /^drawing-/;

function collectDrawingRefsFromValue(value: unknown, refs: string[]): void {
	if (typeof value === "string" && DRAWING_REF_PATTERN.test(value)) {
		refs.push(value);
		return;
	}
	if (Array.isArray(value)) {
		for (const nested of value) collectDrawingRefsFromValue(nested, refs);
		return;
	}
	if (!value || typeof value !== "object") return;
	const record = value as Record<string, unknown>;
	if (record.$schema === "draw.drawing" && typeof record.handle === "string" && DRAWING_REF_PATTERN.test(record.handle)) {
		refs.push(record.handle);
		return;
	}
	for (const nested of Object.values(record)) {
		collectDrawingRefsFromValue(nested, refs);
	}
}

const proceduralPreviewExtractors: Array<(context: { widgetId: string; port: string; direction: ProceduralChannelDirection; value: unknown }) => readonly ProceduralPreviewItem[]> = [
	(context) => {
		const refs: string[] = [];
		collectDrawingRefsFromValue(context.value, refs);
		return [...new Set(refs)].map((handle) => ({
			widgetId: context.widgetId,
			port: context.port,
			direction: context.direction,
			kind: "drawing" as const,
			handle,
		}));
	},
];

function previewItemsFromChannelValue(widgetId: string, port: string, direction: ProceduralChannelDirection, value: unknown): ProceduralPreviewItem[] {
	if (value && typeof value === "object" && !Array.isArray(value) && typeof (value as Record<string, unknown>).error === "string") {
		return [];
	}
	const items: ProceduralPreviewItem[] = [];
	for (const extractor of proceduralPreviewExtractors) {
		items.push(...extractor({ widgetId, port, direction, value }));
	}
	return items;
}

/** @emoji 🔗 Resolves hovered/selected channels to output drawing channels for 2D emphasis. */
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
	return items.filter((entry) => entry.direction === "out" || (entry.direction === "in" && entry.kind === "drawing"));
}

/** @emoji 🔍 Collects drawing preview items from channel-structured flow eval JSON. */
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
// #endregion 🔖PreviewTypes

// #region 🔖CanvasRender
interface Camera2d {
	readonly x: number;
	readonly y: number;
	readonly zoom: number;
}

const PROCEDURAL_2D_PREVIEW_MARQUEE_THRESHOLD_PX = 4;
const DEFAULT_CAMERA: Camera2d = { x: 0, y: 0, zoom: 1 };

function sceneBounds(scene: DrawingScene): { minX: number; minY: number; maxX: number; maxY: number } {
	let minX = 0;
	let minY = 0;
	let maxX = scene.width;
	let maxY = scene.height;
	for (const entry of scene.nodes) {
		const node = entry.node;
		if (node.kind === "rect") {
			minX = Math.min(minX, node.x);
			minY = Math.min(minY, node.y);
			maxX = Math.max(maxX, node.x + node.width);
			maxY = Math.max(maxY, node.y + node.height);
		} else if (node.kind === "circle") {
			minX = Math.min(minX, node.cx - node.r);
			minY = Math.min(minY, node.cy - node.r);
			maxX = Math.max(maxX, node.cx + node.r);
			maxY = Math.max(maxY, node.cy + node.r);
		}
	}
	return { minX, minY, maxX, maxY };
}

function worldToScreen(point: readonly [number, number], camera: Camera2d, width: number, height: number): { x: number; y: number } {
	const cx = width * 0.5;
	const cy = height * 0.5;
	return {
		x: cx + (point[0] - camera.x) * camera.zoom,
		y: cy + (point[1] - camera.y) * camera.zoom,
	};
}

function paintSceneOnCanvas(ctx: CanvasRenderingContext2D, scene: DrawingScene, camera: Camera2d, width: number, height: number, highlight: boolean): void {
	ctx.save();
	ctx.translate(width * 0.5, height * 0.5);
	ctx.scale(camera.zoom, camera.zoom);
	ctx.translate(-camera.x, -camera.y);
	if (highlight) {
		ctx.shadowColor = "rgba(59,130,246,0.8)";
		ctx.shadowBlur = 8 / camera.zoom;
	}
	paintDrawingScene(ctx, scene, { clear: false });
	ctx.restore();
}

function zoomCameraAtScreenPoint(camera: Camera2d, screen: { x: number; y: number }, width: number, height: number, factor: number): Camera2d {
	const cx = width * 0.5;
	const cy = height * 0.5;
	const worldX = camera.x + (screen.x - cx) / camera.zoom;
	const worldY = camera.y + (screen.y - cy) / camera.zoom;
	const zoom = Math.min(32, Math.max(0.05, camera.zoom * factor));
	return {
		x: worldX - (screen.x - cx) / zoom,
		y: worldY - (screen.y - cy) / zoom,
		zoom,
	};
}

export interface Procedural2dPreviewProps {
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
	readonly onHover?: (channel: ProceduralChannelRef | null) => void;
	readonly onSelect?: (channel: ProceduralChannelRef) => void;
	readonly onSelectionChange?: (ids: readonly string[], mode: ProceduralSelectionMode) => void;
	readonly kernel?: DrawingExportBridge;
	readonly className?: string;
}

/** @emoji 🖼️ Infinite-cavas-style 2D preview for procedural draw graphs. */
export function Procedural2dPreview({
	items,
	selectedNodeIds = [],
	selectedChannels = [],
	preselectNodeIds = [],
	preselectRemovedNodeIds = [],
	hoveredNodeId = null,
	hoveredChannel = null,
	selectedGeometryTargets = [],
	fixtureEdges = [],
	previewOffNodeIds = [],
	showMode = "everything",
	selectionMode = "default",
	selectionMethod = "rectangle",
	onHover,
	onSelect,
	onSelectionChange,
	kernel,
	className,
}: Procedural2dPreviewProps): ReactNode {
	const containerRef = reactHostPort.useRef<HTMLDivElement>(null);
	const canvasRef = reactHostPort.useRef<HTMLCanvasElement>(null);
	const [camera, setCamera] = reactHostPort.useState<Camera2d>(DEFAULT_CAMERA);
	const [resolvedKernel, setResolvedKernel] = reactHostPort.useState<DrawingExportBridge | null>(kernel ?? null);
	const [canvasBackground, setCanvasBackground] = reactHostPort.useState(() => resolveSemanticColorHex("--canvas", "light-8-9"));
	const [marqueeOverlay, setMarqueeOverlay] = reactHostPort.useState<{
		coverage: SelectionMarqueeCoverage;
		shape: "rect" | "polygon";
		rect?: { x: number; y: number; width: number; height: number };
		points?: readonly { x: number; y: number }[];
	} | null>(null);

	reactHostPort.useEffect(() => {
		if (kernel) {
			setResolvedKernel(kernel);
			return;
		}
		let cancelled = false;
		void ensureProcedural2dDrawingBridge().then((bridge) => {
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
			}).filter((entry) => entry.kind === "drawing" && isDrawingRef(entry.handle)),
		[fixtureEdges, hoveredChannel, hoveredNodeId, items, selectedChannels, selectedGeometryTargets, selectedNodeIds, showMode],
	);

	const scenes = reactHostPort.useMemo(() => {
		const map = new Map<string, DrawingScene>();
		for (const item of visibleItems) {
			if (item.kind !== "drawing") continue;
			if (item.scene) {
				map.set(item.widgetId, item.scene);
				continue;
			}
			if (!resolvedKernel) continue;
			try {
				map.set(item.widgetId, resolvedKernel.renderScene(item.handle));
			} catch {
				/* ignore */
			}
		}
		return map;
	}, [resolvedKernel, visibleItems]);

	reactHostPort.useEffect(() => {
		const canvas = canvasRef.current;
		const container = containerRef.current;
		if (!canvas || !container) return;
		const paint = () => {
			const rect = container.getBoundingClientRect();
			const dpr = typeof window !== "undefined" ? window.devicePixelRatio || 1 : 1;
			canvas.width = Math.max(1, Math.floor(rect.width * dpr));
			canvas.height = Math.max(1, Math.floor(rect.height * dpr));
			canvas.style.width = `${rect.width}px`;
			canvas.style.height = `${rect.height}px`;
			const ctx = canvas.getContext("2d");
			if (!ctx) return;
			ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
			ctx.fillStyle = canvasBackground;
			ctx.fillRect(0, 0, rect.width, rect.height);
			for (const item of visibleItems) {
				if (item.kind !== "drawing") continue;
				const scene = scenes.get(item.widgetId);
				if (!scene) continue;
				const selected = selectedNodeIds.includes(item.widgetId);
				const hovered = hoveredNodeId === item.widgetId;
				const previewOff = showMode !== "selected" && previewOffNodeIds.includes(item.widgetId);
				if (previewOff) continue;
				paintSceneOnCanvas(ctx, scene, camera, rect.width, rect.height, selected || hovered);
			}
		};
		paint();
		const observer = new ResizeObserver(paint);
		observer.observe(container);
		return () => observer.disconnect();
	}, [camera, canvasBackground, hoveredNodeId, previewOffNodeIds, scenes, selectedNodeIds, showMode, visibleItems]);

	reactHostPort.useEffect(() => {
		const container = containerRef.current;
		if (!container) return;
		const onWheel = (event: WheelEvent) => {
			event.preventDefault();
			const rect = container.getBoundingClientRect();
			const factor = event.deltaY < 0 ? 1.1 : 0.9;
			const screen = { x: event.clientX - rect.left, y: event.clientY - rect.top };
			setCamera((prev) => zoomCameraAtScreenPoint(prev, screen, rect.width, rect.height, factor));
		};
		container.addEventListener("wheel", onWheel, { passive: false });
		return () => container.removeEventListener("wheel", onWheel);
	}, []);

	reactHostPort.useEffect(() => {
		const container = containerRef.current;
		if (!container) return;
		const panRef = { active: false, lastX: 0, lastY: 0 };
		const onPointerDown = (event: PointerEvent) => {
			if (event.button !== 1) return;
			event.preventDefault();
			panRef.active = true;
			panRef.lastX = event.clientX;
			panRef.lastY = event.clientY;
			container.setPointerCapture(event.pointerId);
		};
		const onPointerMove = (event: PointerEvent) => {
			if (!panRef.active) return;
			const dx = event.clientX - panRef.lastX;
			const dy = event.clientY - panRef.lastY;
			panRef.lastX = event.clientX;
			panRef.lastY = event.clientY;
			setCamera((prev) => ({
				...prev,
				x: prev.x - dx / prev.zoom,
				y: prev.y - dy / prev.zoom,
			}));
		};
		const onPointerUp = (event: PointerEvent) => {
			if (!panRef.active) return;
			panRef.active = false;
			container.releasePointerCapture(event.pointerId);
		};
		const bindings = new CavasEventBindingController();
		bindings.listen(container, "pointerdown", onPointerDown as EventListener);
		bindings.listen(container, "pointermove", onPointerMove as EventListener);
		bindings.listen(container, "pointerup", onPointerUp as EventListener);
		bindings.listen(container, "pointercancel", onPointerUp as EventListener);
		return () => bindings.dispose();
	}, []);

	const clientToLocal = reactHostPort.useCallback((clientX: number, clientY: number) => {
		const rect = containerRef.current?.getBoundingClientRect();
		if (!rect) return { x: 0, y: 0 };
		return { x: clientX - rect.left, y: clientY - rect.top };
	}, []);

	const screenBoundsForItem = reactHostPort.useCallback(
		(widgetId: string): { left: number; top: number; right: number; bottom: number } | null => {
			const scene = scenes.get(widgetId);
			const container = containerRef.current;
			if (!scene || !container) return null;
			const rect = container.getBoundingClientRect();
			const bounds = sceneBounds(scene);
			const tl = worldToScreen([bounds.minX, bounds.minY], camera, rect.width, rect.height);
			const br = worldToScreen([bounds.maxX, bounds.maxY], camera, rect.width, rect.height);
			return { left: tl.x, top: tl.y, right: br.x, bottom: br.y };
		},
		[camera, scenes],
	);

	const resolveMarqueeHits = reactHostPort.useCallback(
		(points: readonly { x: number; y: number }[], crossing: boolean): string[] => {
			const marqueeRect = screenRectFromPoints(points);
			if (!marqueeRect) return [];
			const hits: string[] = [];
			for (const entry of visibleItems) {
				const bounds = screenBoundsForItem(entry.widgetId);
				if (!bounds) continue;
				const target = { x: bounds.left, y: bounds.top, width: bounds.right - bounds.left, height: bounds.bottom - bounds.top };
				const marquee = { x: marqueeRect.x, y: marqueeRect.y, width: marqueeRect.width, height: marqueeRect.height };
				if (crossing ? screenRectIntersectsRect(marquee, target) : screenRectContainsRect(marquee, target)) {
					hits.push(entry.widgetId);
				}
			}
			return hits;
		},
		[screenBoundsForItem, visibleItems],
	);

	reactHostPort.useEffect(() => {
		const container = containerRef.current;
		if (!container || !onSelectionChange) return;
		const marqueeRef = {
			tracking: false,
			active: false,
			start: { x: 0, y: 0 },
			points: [] as { x: number; y: number }[],
			initial: [...selectedNodeIds],
		};
		const reset = () => {
			marqueeRef.tracking = false;
			marqueeRef.active = false;
			marqueeRef.points = [];
			setMarqueeOverlay(null);
		};
		const onPointerDown = (event: PointerEvent) => {
			if (event.button !== 0) return;
			marqueeRef.tracking = true;
			marqueeRef.active = false;
			marqueeRef.start = clientToLocal(event.clientX, event.clientY);
			marqueeRef.points = [marqueeRef.start];
			marqueeRef.initial = [...selectedNodeIds];
		};
		const onPointerMove = (event: PointerEvent) => {
			if (!marqueeRef.tracking) return;
			const point = clientToLocal(event.clientX, event.clientY);
			const distance = Math.hypot(point.x - marqueeRef.start.x, point.y - marqueeRef.start.y);
			if (!marqueeRef.active && distance >= PROCEDURAL_2D_PREVIEW_MARQUEE_THRESHOLD_PX) marqueeRef.active = true;
			if (!marqueeRef.active) return;
			if (selectionMethod === "lasso") marqueeRef.points = [...marqueeRef.points, point];
			const points = selectionMethod === "lasso" ? marqueeRef.points : [marqueeRef.start, point];
			const coverage = marqueeCoverageFromGesture({
				method: selectionMethod,
				startX: marqueeRef.start.x,
				endX: point.x,
				path: points,
			});
			const rect = screenRectFromPoints(points);
			setMarqueeOverlay(
				selectionMethod === "lasso"
					? { coverage, shape: "polygon", points }
					: { coverage, shape: "rect", rect: rect ?? undefined },
			);
		};
		const onPointerUp = (event: PointerEvent) => {
			if (!marqueeRef.tracking) return;
			const point = clientToLocal(event.clientX, event.clientY);
			const distance = Math.hypot(point.x - marqueeRef.start.x, point.y - marqueeRef.start.y);
			const mode = marqueeModeFromModifiers(event);
			if (marqueeRef.active && distance >= PROCEDURAL_2D_PREVIEW_MARQUEE_THRESHOLD_PX) {
				const points = selectionMethod === "lasso" ? [...marqueeRef.points, point] : [marqueeRef.start, point];
				const coverage = marqueeCoverageFromGesture({ method: selectionMethod, startX: marqueeRef.start.x, endX: point.x, path: points });
				const hits = resolveMarqueeHits(points, coverage === "partial");
				onSelectionChange(selectionMergeIds(mode, marqueeRef.initial, hits), mode);
			} else if (distance < PROCEDURAL_2D_PREVIEW_MARQUEE_THRESHOLD_PX) {
				for (const entry of visibleItems) {
					const bounds = screenBoundsForItem(entry.widgetId);
					if (!bounds) continue;
					if (point.x >= bounds.left && point.x <= bounds.right && point.y >= bounds.top && point.y <= bounds.bottom) {
						onSelect?.({ widgetId: entry.widgetId, port: entry.port, direction: entry.direction });
						onSelectionChange(selectionMergeIds(mode, selectedNodeIds, [entry.widgetId]), mode);
						break;
					}
				}
			}
			reset();
		};
		const bindings = new CavasEventBindingController();
		bindings.listen(container, "pointerdown", onPointerDown as EventListener);
		bindings.listen(window, "pointermove", onPointerMove as EventListener);
		bindings.listen(window, "pointerup", onPointerUp as EventListener);
		bindings.listen(window, "pointercancel", onPointerUp as EventListener);
		return () => bindings.dispose();
	}, [clientToLocal, onSelect, onSelectionChange, resolveMarqueeHits, screenBoundsForItem, selectedNodeIds, selectionMethod, visibleItems]);

	return (
		<div ref={containerRef} className={cn(canvasHostRootClass, "relative h-full w-full overflow-hidden", className)}>
			<canvas ref={canvasRef} className="absolute inset-0 h-full w-full touch-none" />
			{marqueeOverlay?.shape === "rect" && marqueeOverlay.rect ? (
				<SelectionMarquee coverage={marqueeOverlay.coverage} shape="rect" rect={marqueeOverlay.rect} />
			) : null}
			{marqueeOverlay?.shape === "polygon" && marqueeOverlay.points ? (
				<SelectionMarquee coverage={marqueeOverlay.coverage} shape="polygon" points={marqueeOverlay.points} />
			) : null}
		</div>
	);
}
// #endregion 🔖CanvasRender

// #region 🔖ProceduralEditor
export interface Procedural2dFlowEditorProps {
	readonly fixtureJson?: string;
	readonly className?: string;
	readonly extensionHost?: Procedural2dExtensionHost;
	readonly reorganize?: FlowReorganizeRequest;
	readonly extensionRevision?: number;
	readonly onPreviewText?: (text: string) => void;
	readonly onEvalOutputs?: (outputsJson: string, previewMeshes?: Readonly<Record<string, unknown>>) => void;
	readonly onOutputExport?: (widgetId: string, format: string, resolvedValueJson: string) => void;
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
}

/** @emoji 🧠 Shared flow canvas editor for procedural 2D graphs. */
export function Procedural2dFlowEditor({
	fixtureJson,
	className,
	extensionHost = procedural2dExtensionHost,
	reorganize,
	extensionRevision = 0,
	onPreviewText,
	onEvalOutputs,
	onOutputExport,
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
}: Procedural2dFlowEditorProps): ReactNode {
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
			onOutputExport={onOutputExport}
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
			className={className}
		/>
	);
}

export { canvasDrawingPngExportPort };
// #endregion 🔖ProceduralEditor

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("@semio-tech/procedural-2d-react", () => {
		it("exports default 2d draw fixture json", () => {
			expect(proceduralFixtureToJson()).toContain("draw.shape.rect");
		});

		it("extractChannelPreviewItems collects drawing outputs", () => {
			const items = extractChannelPreviewItems(
				JSON.stringify({ preview: { out: { "": "drawing-42" } } }),
			);
			expect(items).toEqual([
				{ widgetId: "preview", port: "", direction: "out", kind: "drawing", handle: "drawing-42" },
			]);
		});

		it("extractChannelPreviewItems collects draw.drawing schema outputs", () => {
			const items = extractChannelPreviewItems(
				JSON.stringify({ fill: { out: { "draw.drawing": { $schema: "draw.drawing", handle: "drawing-42", kind: "rect" } } } }),
			);
			expect(items).toEqual([
				{ widgetId: "fill", port: "draw.drawing", direction: "out", kind: "drawing", handle: "drawing-42" },
			]);
		});

		it("extractChannelPreviewItems collects drawings nested in list outputs", () => {
			const items = extractChannelPreviewItems(
				JSON.stringify({ get: { out: { value: [{ $schema: "draw.drawing", handle: "drawing-42" }] } } }),
			);
			expect(items).toEqual([
				{ widgetId: "get", port: "value", direction: "out", kind: "drawing", handle: "drawing-42" },
			]);
		});

		it("filterVisiblePreviewItems respects selected show mode", () => {
			const items: ProceduralPreviewItem[] = [
				{ widgetId: "a", port: "drawing", direction: "out", kind: "drawing", handle: "drawing-1" },
				{ widgetId: "b", port: "drawing", direction: "out", kind: "drawing", handle: "drawing-2" },
			];
			const filtered = filterVisiblePreviewItems(items, {
				showMode: "selected",
				selectedNodeIds: ["a"],
				selectedChannels: [],
				hoveredNodeId: null,
				hoveredChannel: null,
			});
			expect(filtered).toHaveLength(1);
			expect(filtered[0]?.widgetId).toBe("a");
		});

		it("evaluates default draw fixture and renders a filled scene", async () => {
			const { FlowSession } = await import("@semio-tech/flow-react");
			const session = new FlowSession();
			session.loadFixtureJson(proceduralFixtureToJson());
			const outputsJson = await session.evaluate();
			expect(outputsJson).not.toMatch(/^\{"error":/);
			expect(outputsJson).not.toContain("unknown kind");
			const parsed = JSON.parse(outputsJson) as Record<string, { out?: Record<string, unknown>; error?: string }>;
			expect(parsed.fill?.error).toBeUndefined();
			expect(parsed.fill?.out?.["draw.drawing"]).toBeTruthy();
			const items = extractChannelPreviewItems(outputsJson);
			expect(items.some((item) => item.kind === "drawing")).toBe(true);
			const bridge = await ensureProcedural2dDrawingBridge();
			const handle =
				items.find((item) => item.widgetId === "fill" && item.direction === "out" && item.kind === "drawing")?.handle ??
				items.find((item) => item.widgetId === "preview" && item.direction === "in" && item.kind === "drawing")?.handle;
			expect(handle).toBeTruthy();
			const scene = bridge.renderScene(handle!);
			expect(scene.nodes.length).toBeGreaterThan(0);
			expect(scene.nodes.some((node) => node.fill?.kind === "solid")).toBe(true);
		});
	});
}
// #endregion 🧪Tests

//#region 🔖PlayHost
import type { ReactElement } from "react";
import type { AppRendererContribution } from "@semio-tech/framework-platform-core";
import { useApp, usePlayController } from "@semio-tech/framework-playground-renderer-react";
import type { Platform } from "@semio-tech/framework-playground-renderer-react";
import { UiPuzzle2dHostSurfaceNode } from "@semio-tech/framework-playground-core";
import type { UiFlowHostSurfaceNode, UiFormsHostSurfaceNode } from "@semio-tech/framework-platform-core";
import { FLOW_WIDGET_DRAG_MIME, flowWidgetPaletteTreeDragController as procedural2dWidgetPaletteTreeDragController } from "@semio-tech/flow-react";
import type { Procedural2dPlayController, Procedural2dPlayHostBridge } from "@semio-tech/procedural-2d-core";
import { FlowGenerateSurface } from "@semio-tech/forms-react";
import { parseFormSpec } from "@semio-tech/forms-core";
import { downloadFlowOutputExport } from "@semio-tech/flow-react";

let buildProcedural2dPlayCanvasContextMenuRef:
	| typeof import("@semio-tech/procedural-2d-core").buildProcedural2dPlayCanvasContextMenu
	| undefined;

function useProcedural2dPlaySnapshotRevision(runtime: Platform, selector: (ctrl: Procedural2dPlayController) => number): number {
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      const ctrl = runtime.getActiveApp()?.controller as Procedural2dPlayController | undefined;
      const unsubscribeChrome = runtime.subscribeChrome(listener);
      const unsubscribeSnapshot = ctrl?.subscribeSnapshot(listener);
      return () => {
        unsubscribeChrome();
        unsubscribeSnapshot?.();
      };
    },
    () => {
      const ctrl = runtime.getActiveApp()?.controller as Procedural2dPlayController | undefined;
      return ctrl ? selector(ctrl) : 0;
    },
    () => 0,
  );
}

function useProcedural2dPlayExtensionRevision(runtime: Platform): number {
  return useProcedural2dPlaySnapshotRevision(runtime, (c) => c.getExtensionRevision());
}

function useProcedural2dPlayInteractionRevision(runtime: Platform): number {
  return useProcedural2dPlaySnapshotRevision(runtime, (c) => c.getInteractionRevision());
}

async function downloadProcedural2dExport(name: string, data: BlobPart, mime: string): Promise<void> {
  const pickerWindow = window as Window & { showSaveFilePicker?: (options?: { suggestedName?: string; types?: { description: string; accept: Record<string, string[]> }[] }) => Promise<FileSystemFileHandle> };
  const ext = name.includes(".") ? name.slice(name.lastIndexOf(".")) : "";
  if (pickerWindow.showSaveFilePicker) {
    const handle = await pickerWindow.showSaveFilePicker({
      suggestedName: name,
      types: [{ description: "Export", accept: { [mime]: [ext] } }],
    });
    const writable = await handle.createWritable();
    await writable.write(data);
    await writable.close();
    return;
  }
  const href = URL.createObjectURL(new Blob([data], { type: mime }));
  const link = document.createElement("a");
  link.href = href;
  link.download = name;
  link.click();
  URL.revokeObjectURL(href);
}

function Procedural2dPlayToolbarHostBridge({ runtime, ctrl }: { readonly runtime: Platform; readonly ctrl: Procedural2dPlayController | undefined }): ReactElement {
  const interactionRevision = useProcedural2dPlayInteractionRevision(runtime);
  const loadInputRef = reactHostPort.useRef<HTMLInputElement>(null);
  const drawingBridge = useProcedural2dDrawingBridge();
  const downloadFixture = reactHostPort.useCallback(async () => {
    const json = ctrl?.getFixtureJson() ?? proceduralFixtureToJson();
    try {
      await downloadProcedural2dExport("procedural2d.fixture.json", `${json}\n`, "application/json");
      console.log("[DEBUG] procedural 2d play downloaded fixture");
    } catch (error) {
      console.log(`[DEBUG] procedural 2d play download failed: ${String(error)}`);
    }
  }, [ctrl]);
  const handleLoadFile = reactHostPort.useCallback(
    (event: reactHostPort.ChangeEvent<HTMLInputElement>) => {
      const file = event.target.files?.[0];
      event.target.value = "";
      if (!file || !ctrl) return;
      void file.text().then((text) => {
        if (!text.includes("flow.fixture")) {
          console.log("[DEBUG] procedural 2d play load rejected: not a flow fixture");
          return;
        }
        ctrl.run("setFixtureJson", { json: text, resetInteraction: true });
        console.log("[DEBUG] procedural 2d play loaded fixture from file");
      });
    },
    [ctrl],
  );
  reactHostPort.useEffect(() => {
    if (!ctrl) return;
    const bridge: Procedural2dPlayHostBridge = {
      getToolbarState: () => ({
        selectionMethod: ctrl.getSelectionMethod(),
        selectionMode: ctrl.getSelectionMode(),
        showMode: ctrl.getShowMode(),
        selectionCount: ctrl.getSelectedNodeIds().length,
        hasStoredFixture: ctrl.hasStoredFixture(),
      }),
      runHostCommand: (command, args) => {
        if (command === "saveDownload") {
          void downloadFixture();
          return;
        }
        if (command === "loadRequest") {
          loadInputRef.current?.click();
          return;
        }
        if (command === "exportSvg" || command === "exportPdf" || command === "exportPng") {
          const handle = (args as { handle?: string } | undefined)?.handle ?? ctrl.getPrimaryDrawingHandle();
          const primaryItem = ctrl.getPreviewItems().find((item) => item.kind === "drawing" && (!handle || item.handle === handle));
          if (!handle && !primaryItem?.scene) {
            console.log(`[DEBUG] procedural 2d play ${command} skipped: no drawing handle or scene`);
            return;
          }
          void (async () => {
            try {
              if (command === "exportPng" && primaryItem?.scene) {
                const png = canvasDrawingPngExportPort.exportPng(primaryItem.scene);
                await downloadProcedural2dExport("procedural2d.export.png", png, "image/png");
              } else if (!drawingBridge || !handle) {
                console.log(`[DEBUG] procedural 2d play ${command} skipped: no drawing bridge`);
                return;
              } else if (command === "exportSvg") {
                const svg = drawingBridge.exportSvg(handle);
                await downloadProcedural2dExport("procedural2d.export.svg", svg, "image/svg+xml");
              } else if (command === "exportPdf") {
                const pdfBase64 = drawingBridge.exportPdf(handle);
                const binary = atob(pdfBase64);
                const bytes = Uint8Array.from(binary, (char) => char.charCodeAt(0));
                await downloadProcedural2dExport("procedural2d.export.pdf", bytes, "application/pdf");
              } else {
                const png = drawingBridge.exportPng(handle);
                await downloadProcedural2dExport("procedural2d.export.png", png, "image/png");
              }
              console.log(`[DEBUG] procedural 2d play ${command} completed`);
            } catch (error) {
              console.log(`[DEBUG] procedural 2d play ${command} failed: ${String(error)}`);
            }
          })();
        }
      },
    };
    ctrl.setHostBridge(bridge);
    return () => ctrl.setHostBridge(null);
  }, [ctrl, downloadFixture, drawingBridge, interactionRevision]);
  return <input ref={loadInputRef} type="file" accept=".json,application/json" className="hidden" onChange={handleLoadFile} />;
}

function Procedural2dPlayPaneSurfaceHost({ node }: { readonly node: UiFlowHostSurfaceNode }): ReactElement {
  const { runtime } = useApp();
  const ctrl = usePlayController<Procedural2dPlayController>();
  const extensionRevision = useProcedural2dPlayExtensionRevision(runtime);
  const interactionRevision = useProcedural2dPlayInteractionRevision(runtime);
  void interactionRevision;
  const onPreviewText = reactHostPort.useCallback(
    (text: string) => {
      console.log(`[DEBUG] procedural 2d play preview: ${text}`);
      ctrl?.run("setPreviewText", { text });
    },
    [ctrl],
  );
  const onEvalOutputs = reactHostPort.useCallback(
    (outputsJson: string, previewMeshes?: Readonly<Record<string, unknown>>) => {
      console.log(`[DEBUG] procedural 2d play eval outputs: ${outputsJson.slice(0, 120)}`);
      ctrl?.run("setEvalOutputs", { outputsJson, previewMeshes });
    },
    [ctrl],
  );
  const onCatalogueReady = reactHostPort.useCallback(
    (sections: readonly import("@semio-tech/flow-react").CatalogueSection[]) => {
      ctrl?.run("setCatalogueSections", { sections: [...sections] });
    },
    [ctrl],
  );
  const onFixtureChange = reactHostPort.useCallback(
    (json: string) => {
      ctrl?.run("setFixtureJson", { json });
    },
    [ctrl],
  );
  const onSelectionChange = reactHostPort.useCallback(
    (ids: readonly string[]) => {
      ctrl?.run("setSelection", { ids: [...ids], mode: "default", fromFlow: true });
    },
    [ctrl],
  );
  const onPreselectChange = reactHostPort.useCallback(
    (snapshot: { readonly ids: readonly string[]; readonly removedIds: readonly string[] }) => {
      ctrl?.run("setPreselect", { ids: [...snapshot.ids], removedIds: [...snapshot.removedIds] });
    },
    [ctrl],
  );
  const onHoverChange = reactHostPort.useCallback(
    (id: string | null) => {
      ctrl?.run("setHover", { id, channel: null });
    },
    [ctrl],
  );
  const onChannelHoverChange = reactHostPort.useCallback(
    (channel: import("@semio-tech/procedural-2d-react").ProceduralChannelRef | null) => {
      ctrl?.run("setHover", { id: channel?.widgetId ?? null, channel });
    },
    [ctrl],
  );
  const onSelectedChannelsChange = reactHostPort.useCallback(
    (channels: readonly import("@semio-tech/procedural-2d-react").ProceduralChannelRef[]) => {
      ctrl?.run("setSelectedChannels", { channels: [...channels] });
    },
    [ctrl],
  );
  const onPreviewOffChange = reactHostPort.useCallback(
    (ids: readonly string[]) => {
      ctrl?.run("setPreviewOff", { ids: [...ids], fromFlow: true });
    },
    [ctrl],
  );
  const onCanvasCommand = reactHostPort.useCallback(
    (command: string, args?: Record<string, unknown>) => {
      ctrl?.run(command, args);
    },
    [ctrl],
  );
  const onOutputExport = reactHostPort.useCallback(
    (widgetId: string, format: string, resolvedValueJson: string) => {
      void downloadFlowOutputExport(format, resolvedValueJson, widgetId).catch((error) => {
        console.log(`[DEBUG] procedural 2d play export failed: ${String(error)}`);
      });
    },
    [],
  );
  return (
    <>
      <Procedural2dPlayToolbarHostBridge runtime={runtime} ctrl={ctrl} />
      <Procedural2dFlowEditor
      fixtureJson={ctrl?.getFixtureJson() ?? proceduralFixtureToJson()}
      reorganize={ctrl?.getReorganize()}
      commandRequest={ctrl?.getCommandRequest()}
      extensionRevision={extensionRevision}
      onPreviewText={onPreviewText}
      onEvalOutputs={onEvalOutputs}
      onOutputExport={onOutputExport}
      onCatalogueReady={onCatalogueReady}
      onFixtureChange={onFixtureChange}
      onSelectionChange={onSelectionChange}
      onPreselectChange={onPreselectChange}
      onHoverChange={onHoverChange}
      onChannelHoverChange={onChannelHoverChange}
      onSelectedChannelsChange={onSelectedChannelsChange}
      onPreviewOffChange={onPreviewOffChange}
      selectedNodeIds={ctrl?.getSelectedNodeIds()}
      selectedChannels={ctrl?.getSelectedChannels()}
      preselectNodeIds={ctrl?.getPreselectNodeIds()}
      preselectRemovedNodeIds={ctrl?.getPreselectRemovedNodeIds()}
      hoveredNodeId={ctrl?.getHoveredNodeId()}
      hoveredChannel={ctrl?.getHoveredChannel()}
      previewOffNodeIds={ctrl?.getPreviewOffNodeIds()}
      selectionMode={ctrl?.getSelectionMode()}
      selectionMethod={ctrl?.getSelectionMethod()}
      contextMenu={(ctx) => buildProcedural2dPlayCanvasContextMenuRef?.(ctx, onCanvasCommand) ?? []}
      className="h-full w-full"
    />
    </>
  );
}

function Procedural2dPreviewSurfaceHost({ node: _node }: { readonly node: UiPuzzle2dHostSurfaceNode }): ReactElement {
  const { runtime } = useApp();
  const ctrl = usePlayController<Procedural2dPlayController>();
  const drawingBridge = useProcedural2dDrawingBridge();
  const interactionRevision = useProcedural2dPlayInteractionRevision(runtime);
  void interactionRevision;
  const onHover = reactHostPort.useCallback(
    (channel: import("@semio-tech/procedural-2d-react").ProceduralChannelRef | null) => {
      ctrl?.run("setHover", { id: channel?.widgetId ?? null, channel });
    },
    [ctrl],
  );
  const onSelect = reactHostPort.useCallback(
    (channel: import("@semio-tech/procedural-2d-react").ProceduralChannelRef) => {
      ctrl?.run("setSelectChannels", { channels: [channel] });
    },
    [ctrl],
  );
  const onSelectionChange = reactHostPort.useCallback(
    (ids: readonly string[], mode: import("@semio-tech/procedural-2d-react").ProceduralSelectionMode) => {
      ctrl?.run("setSelection", { ids: [...ids], mode });
    },
    [ctrl],
  );
  return (
    <div className="absolute inset-0 min-h-0 min-w-0">
      <Procedural2dPreview
        items={ctrl?.getPreviewItems() ?? []}
        selectedNodeIds={ctrl?.getSelectedNodeIds()}
        preselectNodeIds={ctrl?.getPreselectNodeIds()}
        preselectRemovedNodeIds={ctrl?.getPreselectRemovedNodeIds()}
        hoveredNodeId={ctrl?.getHoveredNodeId()}
        hoveredChannel={ctrl?.getHoveredChannel()}
        hoveredGeometryTargets={ctrl?.getHoveredGeometryTargets()}
        selectedChannels={ctrl?.getSelectedChannels()}
        selectedGeometryTargets={ctrl?.getSelectedGeometryTargets()}
        previewOffNodeIds={ctrl?.getPreviewOffNodeIds()}
        showMode={ctrl?.getShowMode() ?? "everything"}
        selectionMode={ctrl?.getSelectionMode()}
        selectionMethod={ctrl?.getSelectionMethod()}
        onHover={onHover}
        onSelect={onSelect}
        onSelectionChange={onSelectionChange}
        kernel={drawingBridge ?? undefined}
        className="h-full w-full"
      />
    </div>
  );
}

function Procedural2dGenerateSurfaceHost({ node }: { readonly node: UiFormsHostSurfaceNode }): ReactElement {
  const ctrl = usePlayController<Procedural2dPlayController>();
  const spec = reactHostPort.useMemo(() => {
    try {
      return parseFormSpec(JSON.parse(ctrl?.getGenerateFormSpecJson() ?? "{}"));
    } catch {
      return parseFormSpec({ schema: "forms.form", id: "empty", version: "1", steps: [{ id: "s", title: "Inputs", questions: [] }] });
    }
  }, [ctrl]);
  return (
    <FlowGenerateSurface
      formSpec={spec}
      generations={[...(ctrl?.getGenerations() ?? [])]}
      selectedGenerationId={ctrl?.getSelectedGenerationId() ?? null}
      previewText={ctrl?.getGeneratePreviewText() ?? "—"}
      onSelectGeneration={(id) => ctrl?.run("selectGeneration", { id })}
      onAddGeneration={() => ctrl?.run("addGeneration")}
      onRemoveGeneration={(id) => ctrl?.run("removeGeneration", { id })}
      onGenerationValuesChange={(id, values) => ctrl?.run("updateGenerationValues", { id, values })}
      onRenameGeneration={(id, name) => ctrl?.run("renameGeneration", { id, name })}
      className="h-full"
    />
  );
}

/** @emoji 🛝 procedural2d app renderer for playground and OS shells. */
export async function procedural2dAppRenderer(): Promise<AppRendererContribution> {
  const {
    PROCEDURAL_2D_PLAY_SURFACE_ID,
    PROCEDURAL_2D_PLAY_SURFACE_ID_GENERATE,
    PROCEDURAL_2D_PLAY_SURFACE_ID_PREVIEW,
    procedural2dPlayWindowBodies,
    procedural2dPlaySidePanelBodies,
    buildProcedural2dPlayCanvasContextMenu,
  } = await import("@semio-tech/procedural-2d-core");
  buildProcedural2dPlayCanvasContextMenuRef = buildProcedural2dPlayCanvasContextMenu;
  return {
    windowBodies: procedural2dPlayWindowBodies,
    sidePanelBodies: procedural2dPlaySidePanelBodies,
    surfaceHosts: {
      [PROCEDURAL_2D_PLAY_SURFACE_ID]: Procedural2dPlayPaneSurfaceHost,
      [PROCEDURAL_2D_PLAY_SURFACE_ID_PREVIEW]: Procedural2dPreviewSurfaceHost,
      [PROCEDURAL_2D_PLAY_SURFACE_ID_GENERATE]: Procedural2dGenerateSurfaceHost,
    },
    treeDragController: (dragByItemId) => {
      const sample = dragByItemId.values().next().value;
      if (sample && FLOW_WIDGET_DRAG_MIME in sample) return procedural2dWidgetPaletteTreeDragController(dragByItemId);
      return undefined;
    },
  };
}
//#endregion 🔖PlayHost
