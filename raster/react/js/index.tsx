// #region 🧲Header
/** @emoji 🖼️ Raster React host: WASM {@link RasterSession}, {@link RasterCanvas}, per-layer views. */
// #endregion 🧲Header

// #region 🔌Adapters
import React, { useCallback, useEffect, useLayoutEffect, useRef, useState, type ReactNode } from "react";
import {
	cn,
	CanvasPickMenu,
	marqueeCoverageFromGesture,
	marqueeModeFromModifiers,
	SelectionMarquee,
	selectionMergeIds,
	screenRectFromPoints,
	useCanvasPickInteraction,
	useVelloThemeSync,
	type SelectionMarqueeCoverage,
	type SelectionMarqueePoint,
	type SelectionMarqueeMethod,
	type SelectionMarqueeRect,
	type SelectionMergeMode,
} from "@semio-tech/ui-react";
import { syncSessionVelloTheme } from "@semio-tech/ui-styling";
import { parseCanvasPickTargetKey, type CanvasPickTarget } from "@semio-tech/framework-core";
import {
	decodeRasterImageAsset,
	rasterDocumentToSyncJson,
	resolveRasterPickTargetsAtScreenPoint,
	resolveRasterMarqueeLayerHits,
	rasterCameraEqual,
	rasterNavigatorFitCamera,
	rasterNavigatorViewportOverlay,
	rasterWheelCamera,
	type RasterCamera,
	type RasterDocument,
	type RasterHoverPayload,
	type RasterKindHover,
	type RasterPickTarget,
	type RasterToolId,
	type RasterViewport,
} from "@semio-tech/raster-core";
import initRasterWasm, { RasterSession } from "../../rs/pkg/raster.js";

const rasterWasmLoadedSync = Boolean(import.meta.vitest || (typeof process !== "undefined" && process.env.VITEST));

if (rasterWasmLoadedSync) {
	const { readFileSync } = await import("node:fs");
	const { initSync } = await import("../../rs/pkg/raster.js");
	const wasmPath = new URL("../../rs/pkg/raster_bg.wasm", import.meta.url).pathname;
	initSync({ module: readFileSync(wasmPath) });
} else {
	await initRasterWasm();
}

export async function ensureRasterWasmLoaded(): Promise<void> {
	if (rasterWasmLoadedSync) return;
	await initRasterWasm();
}

export { RasterSession };

const RASTER_MARQUEE_THRESHOLD_PX = 4;
const RASTER_MIN_ATTACH_PX = 64;

type RasterMarqueeOverlay =
	| { readonly coverage: SelectionMarqueeCoverage; readonly shape: "rect"; readonly rect: SelectionMarqueeRect }
	| { readonly coverage: SelectionMarqueeCoverage; readonly shape: "polygon"; readonly points: readonly SelectionMarqueePoint[] };

function rasterSelectionMethod(activeTool: RasterToolId | undefined): SelectionMarqueeMethod | null {
	if (activeTool === "selectMarquee") return "rectangle";
	if (activeTool === "selectLasso") return "lasso";
	return null;
}

function isRasterSelectionTool(activeTool: RasterToolId | undefined): boolean {
	return activeTool === "selectMarquee" || activeTool === "selectLasso" || activeTool === "selectWand";
}

function rasterPickTargetToCanvas(target: RasterPickTarget): CanvasPickTarget {
	return { domain: target.domain, id: target.id, generality: target.generality, label: target.label };
}

function rasterHoverPayloadFromFocusKey(key: string | null): RasterHoverPayload {
	if (!key) return { id: null, kind: null };
	const parsed = parseCanvasPickTargetKey(key);
	if (!parsed) return { id: null, kind: null };
	const domain = parsed.domain === "pixel" ? "layer" : parsed.domain;
	return { id: parsed.id, kind: { domain: domain as RasterKindHover["domain"], kindId: parsed.id } };
}
// #endregion 🔌Adapters

function uploadRasterDocumentAssets(session: RasterSession, doc: RasterDocument): void {
	if (!doc.assets) return;
	for (const [key, asset] of Object.entries(doc.assets)) {
		const bytes = decodeRasterImageAsset(asset);
		session.uploadRasterImageKey(key, bytes);
	}
}

// #region 📐Contracts
export type RasterViewMode = "composite" | "layer" | "mask" | "navigator";

export interface RasterCanvasProps {
	readonly document: RasterDocument;
	readonly viewMode?: RasterViewMode;
	readonly isolatedLayerId?: string | null;
	readonly camera?: RasterCamera;
	readonly selectedIds?: readonly string[];
	readonly hoveredId?: string | null;
	readonly kindHover?: RasterKindHover | null;
	readonly activeTool?: RasterToolId;
	readonly className?: string;
	readonly contentViewport?: RasterViewport;
	readonly onViewportChange?: (viewport: RasterViewport) => void;
	readonly onCameraChange?: (camera: RasterCamera) => void;
	readonly onHover?: (payload: RasterHoverPayload) => void;
	readonly onSelect?: (ids: readonly string[]) => void;
	readonly onDocumentChange?: (document: RasterDocument) => void;
}

export interface RasterRendererOptions {
	readonly viewMode?: RasterViewMode;
	readonly isolatedLayerId?: string | null;
}
// #endregion 📐Contracts

// #region 🔧Renderer
function parseCameraJson(json: string): RasterCamera {
	try {
		const raw = JSON.parse(json) as { x?: number; y?: number; zoom?: number };
		return { x: raw.x ?? 0, y: raw.y ?? 0, zoom: raw.zoom ?? 1 };
	} catch {
		return { x: 0, y: 0, zoom: 1 };
	}
}

/** @emoji 🖼️ Owns one {@link RasterSession} and syncs document JSON each frame. */
export class RasterRenderer {
	readonly session: RasterSession;
	private raf = 0;
	private mounted = false;
	private attachPromise: Promise<void> | null = null;
	private boundCanvas: HTMLCanvasElement | null = null;
	private documentJson = "";
	private viewMode: RasterViewMode = "composite";
	private isolatedLayerId: string | null = null;

	constructor() {
		this.session = new RasterSession();
	}

	setViewMode(mode: RasterViewMode, layerId: string | null = null): void {
		this.viewMode = mode;
		this.isolatedLayerId = layerId;
		this.session.setViewMode(mode, layerId);
	}

	syncDocument(doc: RasterDocument): void {
		const json = rasterDocumentToSyncJson(doc);
		if (json !== this.documentJson) {
			this.documentJson = json;
			this.session.syncDocumentJson(json);
			uploadRasterDocumentAssets(this.session, doc);
		}
	}

	syncSelection(ids: readonly string[]): void {
		this.session.setSelectionIdsJson(JSON.stringify(ids));
	}

	syncHover(id: string | null, kind: RasterKindHover | null): void {
		this.session.setHoveredIdSilent(id);
		void kind;
	}

	syncTool(tool: RasterToolId): void {
		this.session.setActiveTool(tool);
	}

	syncCamera(camera: RasterCamera): void {
		this.session.setCamera(camera.x, camera.y, camera.zoom);
	}

	mirrorCameraFromSession(): RasterCamera {
		return parseCameraJson(this.session.cameraJson());
	}

	invalidate(): void {
		if (!this.mounted) return;
		cancelAnimationFrame(this.raf);
		this.raf = requestAnimationFrame(() => {
			this.session.renderFrame();
		});
	}

	async attachCanvas(canvas: HTMLCanvasElement, width: number, height: number, dpr: number): Promise<void> {
		if (this.boundCanvas === canvas && (this.mounted || this.session.gpuReady())) {
			this.setSize(width, height, dpr);
			return;
		}
		if (this.attachPromise) {
			await this.attachPromise;
			if (this.boundCanvas === canvas && (this.mounted || this.session.gpuReady())) {
				this.setSize(width, height, dpr);
				return;
			}
		}
		this.attachPromise = (async () => {
			if (this.session.gpuReady()) {
				this.mounted = true;
				this.boundCanvas = canvas;
				this.setSize(width, height, dpr);
				return;
			}
			const pw = Math.max(1, Math.round(width * dpr));
			const ph = Math.max(1, Math.round(height * dpr));
			if (canvas.width !== pw || canvas.height !== ph) {
				canvas.width = pw;
				canvas.height = ph;
			}
			await this.session.attachCanvas(canvas, width, height, dpr);
			this.mounted = true;
			this.boundCanvas = canvas;
			console.log("[DEBUG] raster canvas attached", { width, height, dpr, viewMode: this.viewMode });
			this.invalidate();
		})();
		try {
			await this.attachPromise;
		} finally {
			this.attachPromise = null;
		}
	}

	setSize(width: number, height: number, dpr: number): void {
		if (this.boundCanvas) {
			const pw = Math.max(1, Math.round(width * dpr));
			const ph = Math.max(1, Math.round(height * dpr));
			if (this.boundCanvas.width !== pw || this.boundCanvas.height !== ph) {
				this.boundCanvas.width = pw;
				this.boundCanvas.height = ph;
			}
		}
		this.session.setSize(width, height, dpr);
		this.invalidate();
	}

	dispose(): void {
		this.mounted = false;
		this.boundCanvas = null;
		this.attachPromise = null;
		cancelAnimationFrame(this.raf);
	}
}
// #endregion 🔧Renderer

// #region 🖼️Canvas
export const RasterCanvas: React.FC<RasterCanvasProps> = ({
	document,
	viewMode = "composite",
	isolatedLayerId = null,
	camera,
	selectedIds = [],
	hoveredId = null,
	kindHover = null,
	activeTool,
	className,
	contentViewport,
	onViewportChange,
	onCameraChange,
	onHover,
	onSelect,
}) => {
	const renderer = React.useMemo(() => new RasterRenderer(), []);
	const syncVelloTheme = useCallback(() => {
		syncSessionVelloTheme(renderer.session);
		renderer.invalidate();
	}, [renderer]);
	useVelloThemeSync(syncVelloTheme);
	const canvasRef = useRef<HTMLCanvasElement | null>(null);
	const containerRef = useRef<HTMLDivElement | null>(null);
	const viewportRef = useRef({ width: 1, height: 1 });
	const panningRef = useRef(false);
	const panLastRef = useRef<{ x: number; y: number } | null>(null);
	const [viewportSize, setViewportSize] = useState<RasterViewport>({ width: 1, height: 1 });
	const [localCamera, setLocalCamera] = useState<RasterCamera>(() => camera ?? document.camera);
	const isNavigator = viewMode === "navigator";
	const contentCamera = camera ?? document.camera;
	const interactionCamera = isNavigator ? contentCamera : localCamera;
	const navigatorCamera = React.useMemo(
		() => (isNavigator ? rasterNavigatorFitCamera(document, viewportSize) : localCamera),
		[isNavigator, document, viewportSize, localCamera],
	);
	const externalCameraKey = `${contentCamera.x}:${contentCamera.y}:${contentCamera.zoom}`;
	const navigatorOverlay = React.useMemo(() => {
		if (!isNavigator || !contentViewport) return null;
		if (contentViewport.width <= 1 || contentViewport.height <= 1) return null;
		return rasterNavigatorViewportOverlay(contentCamera, contentViewport, navigatorCamera, viewportSize);
	}, [contentCamera, contentViewport, isNavigator, navigatorCamera, viewportSize]);
	const marqueeRef = useRef({
		tracking: false,
		active: false,
		start: { x: 0, y: 0 } as SelectionMarqueePoint,
		points: [] as SelectionMarqueePoint[],
		mergeMode: "default" as SelectionMergeMode,
	});
	const [marqueeOverlay, setMarqueeOverlay] = useState<RasterMarqueeOverlay | null>(null);
	const [attachError, setAttachError] = useState<string | null>(null);
	const [gpuAttached, setGpuAttached] = useState(false);
	const selectionMethod = rasterSelectionMethod(activeTool);

	const emitCamera = useCallback(
		(next: RasterCamera) => {
			if (rasterCameraEqual(contentCamera, next)) return;
			onCameraChange?.(next);
		},
		[contentCamera, onCameraChange],
	);

	useEffect(() => () => renderer.dispose(), [renderer]);

	useLayoutEffect(() => {
		renderer.setViewMode(viewMode, isolatedLayerId);
		renderer.invalidate();
	}, [renderer, viewMode, isolatedLayerId]);

	useEffect(() => {
		renderer.syncDocument(document);
		if (gpuAttached) renderer.invalidate();
	}, [renderer, document, gpuAttached]);

	useEffect(() => {
		if (!gpuAttached || !isNavigator) return;
		renderer.syncCamera(navigatorCamera);
		renderer.invalidate();
	}, [gpuAttached, isNavigator, navigatorCamera, renderer]);

	useEffect(() => {
		if (!gpuAttached || isNavigator) return;
		const wasmCamera = renderer.mirrorCameraFromSession();
		setLocalCamera((previous) => (rasterCameraEqual(previous, contentCamera) ? previous : contentCamera));
		if (rasterCameraEqual(wasmCamera, contentCamera)) return;
		renderer.syncCamera(contentCamera);
		renderer.invalidate();
	}, [externalCameraKey, gpuAttached, isNavigator, renderer, contentCamera]);

	const applyCompositeCamera = useCallback(
		(next: RasterCamera) => {
			setLocalCamera((previous) => (rasterCameraEqual(previous, next) ? previous : next));
			if (!rasterCameraEqual(contentCamera, next)) onCameraChange?.(next);
		},
		[contentCamera, onCameraChange],
	);

	useEffect(() => {
		if (isNavigator || !onViewportChange) return;
		if (viewportSize.width <= 1 || viewportSize.height <= 1) return;
		onViewportChange(viewportSize);
	}, [isNavigator, onViewportChange, viewportSize]);

	useEffect(() => {
		renderer.syncSelection(selectedIds);
		renderer.invalidate();
	}, [renderer, selectedIds]);

	useEffect(() => {
		renderer.syncHover(hoveredId, kindHover);
		renderer.invalidate();
	}, [renderer, hoveredId, kindHover]);

	useEffect(() => {
		if (activeTool) renderer.syncTool(activeTool);
	}, [renderer, activeTool]);

	useLayoutEffect(() => {
		const canvas = canvasRef.current;
		const container = containerRef.current;
		if (!canvas || !container) return;

		let disposed = false;
		let attachStarted = false;
		const resize = () => {
			const rect = container.getBoundingClientRect();
			const dpr = window.devicePixelRatio || 1;
			const w = Math.max(1, Math.floor(rect.width));
			const h = Math.max(1, Math.floor(rect.height));
			viewportRef.current = { width: w, height: h };
			setViewportSize({ width: w, height: h });
			if (!isNavigator) onViewportChange?.({ width: w, height: h });
			if (renderer.session.gpuReady()) {
				renderer.setSize(w, h, dpr);
			}
		};

		const tryAttach = async () => {
			const rect = container.getBoundingClientRect();
			const dpr = window.devicePixelRatio || 1;
			const w = Math.max(1, Math.floor(rect.width));
			const h = Math.max(1, Math.floor(rect.height));
			viewportRef.current = { width: w, height: h };
			if (w < RASTER_MIN_ATTACH_PX || h < RASTER_MIN_ATTACH_PX) return;
			if (disposed || attachStarted || renderer.session.gpuReady()) return;
			attachStarted = true;
			try {
				await renderer.attachCanvas(canvas, w, h, dpr);
				if (disposed) return;
				setAttachError(null);
				const initialCamera = isNavigator
					? rasterNavigatorFitCamera(document, { width: w, height: h })
					: (camera ?? document.camera);
				renderer.syncCamera(initialCamera);
				setGpuAttached(true);
				renderer.invalidate();
			} catch (error) {
				attachStarted = false;
				const message = error instanceof Error ? error.message : String(error);
				if (message.includes("already attached")) {
					setAttachError(null);
					const initialCamera = isNavigator
						? rasterNavigatorFitCamera(document, { width: w, height: h })
						: (camera ?? document.camera);
					renderer.syncCamera(initialCamera);
					setGpuAttached(true);
					resize();
					renderer.invalidate();
					return;
				}
				setAttachError(message);
				console.error("[DEBUG] raster canvas attach failed", error);
			}
		};

		const observer = new ResizeObserver(() => {
			resize();
			void tryAttach();
		});
		observer.observe(container);
		resize();
		void tryAttach();
		return () => {
			disposed = true;
			setGpuAttached(false);
			observer.disconnect();
		};
	}, [camera, document, isNavigator, onViewportChange, renderer]);

	const clientPoint = useCallback((event: React.PointerEvent): { x: number; y: number } => {
		return clientPointFromCanvas(canvasRef.current, event);
	}, []);

	function clientPointFromCanvas(
		canvas: HTMLCanvasElement | null,
		event: { clientX: number; clientY: number },
	): { x: number; y: number } {
		if (!canvas) return { x: 0, y: 0 };
		const rect = canvas.getBoundingClientRect();
		return { x: event.clientX - rect.left, y: event.clientY - rect.top };
	}

	const resolveTargetsAtClient = useCallback(
		(client: { readonly x: number; readonly y: number }) => {
			const screen = clientPointFromCanvas(canvasRef.current, client);
			return resolveRasterPickTargetsAtScreenPoint(document, interactionCamera, viewportRef.current, screen).map(
				rasterPickTargetToCanvas,
			);
		},
		[document, interactionCamera],
	);

	const canvasPick = useCanvasPickInteraction({
		resolveTargetsAtClient,
		onHoverFocus: (focus) => onHover?.(rasterHoverPayloadFromFocusKey(focus.targetKey)),
		onSelectTarget: (target, request) => {
			const mergeMode = marqueeModeFromModifiers({
				shiftKey: request.modifiers?.shift === true,
				ctrlKey: request.modifiers?.ctrl === true,
				metaKey: request.modifiers?.meta === true,
				altKey: request.modifiers?.alt === true,
			});
			onSelect?.(selectionMergeIds(mergeMode, selectedIds, [target.id]));
		},
	});

	const handleWheel = useCallback(
		(event: WheelEvent) => {
			event.preventDefault();
			const point = clientPointFromCanvas(canvasRef.current, event);
			if (isNavigator && contentViewport) {
				emitCamera(rasterWheelCamera(contentCamera, contentViewport, point, event.deltaY));
				renderer.invalidate();
				return;
			}
			renderer.session.wheelScreen(point.x, point.y, event.deltaY);
			applyCompositeCamera(renderer.mirrorCameraFromSession());
			renderer.invalidate();
		},
		[applyCompositeCamera, contentCamera, contentViewport, emitCamera, isNavigator, renderer],
	);

	useEffect(() => {
		const canvas = canvasRef.current;
		if (!canvas) return;
		canvas.addEventListener("wheel", handleWheel, { passive: false });
		return () => canvas.removeEventListener("wheel", handleWheel);
	}, [handleWheel]);

	const handlePointerDown = useCallback(
		(event: React.PointerEvent) => {
			const point = clientPoint(event);
			if (selectionMethod && event.button === 0) {
				marqueeRef.current = {
					tracking: true,
					active: false,
					start: point,
					points: [point],
					mergeMode: marqueeModeFromModifiers(event),
				};
				canvasPick.onCanvasPointerDown({ x: event.clientX, y: event.clientY });
				setMarqueeOverlay(null);
				(event.target as HTMLElement).setPointerCapture(event.pointerId);
				return;
			}
			if (activeTool === "selectWand" && event.button === 0) {
				canvasPick.onCanvasPointerDown({ x: event.clientX, y: event.clientY });
				return;
			}
			if (event.button === 1) {
				panningRef.current = true;
				panLastRef.current = point;
				if (!isNavigator) renderer.session.pointerDownScreen(point.x, point.y, event.button);
				(event.target as HTMLElement).setPointerCapture(event.pointerId);
				return;
			}
			if (!isRasterSelectionTool(activeTool) && event.button === 0) {
				renderer.session.pointerDownScreen(point.x, point.y, event.button);
				(event.target as HTMLElement).setPointerCapture(event.pointerId);
			}
		},
		[activeTool, canvasPick, clientPoint, isNavigator, renderer, selectionMethod],
	);

	const updateMarqueeOverlay = useCallback(
		(point: SelectionMarqueePoint, mergeMode: SelectionMergeMode) => {
			if (!selectionMethod) return;
			const marquee = marqueeRef.current;
			const points = selectionMethod === "lasso" ? marquee.points : [marquee.start, point];
			const coverage = marqueeCoverageFromGesture({
				method: selectionMethod,
				startX: marquee.start.x,
				endX: point.x,
				path: points,
			});
			if (selectionMethod === "lasso") {
				setMarqueeOverlay({ coverage, shape: "polygon", points });
				return;
			}
			const rect = screenRectFromPoints(points);
			if (!rect) return;
			setMarqueeOverlay({ coverage, shape: "rect", rect });
		},
		[selectionMethod],
	);

	const commitMarqueeSelection = useCallback(
		(point: SelectionMarqueePoint, mergeMode: SelectionMergeMode) => {
			const marquee = marqueeRef.current;
			const points = selectionMethod === "lasso" ? [...marquee.points, point] : [marquee.start, point];
			const coverage = marqueeCoverageFromGesture({
				method: selectionMethod ?? "rectangle",
				startX: marquee.start.x,
				endX: point.x,
				path: points,
			});
			const rect = screenRectFromPoints(points);
			if (!rect) return;
			const hits = resolveRasterMarqueeLayerHits(document, localCamera, viewportRef.current, rect, coverage === "partial");
			onSelect?.(selectionMergeIds(mergeMode, selectedIds, hits));
		},
		[document, localCamera, onSelect, selectedIds, selectionMethod],
	);

	const handlePointerMove = useCallback(
		(event: React.PointerEvent) => {
			const point = clientPoint(event);
			const marquee = marqueeRef.current;
			if (marquee.tracking) {
				const distance = Math.hypot(point.x - marquee.start.x, point.y - marquee.start.y);
				if (!marquee.active && distance >= RASTER_MARQUEE_THRESHOLD_PX) marquee.active = true;
				if (marquee.active) {
					if (selectionMethod === "lasso") marquee.points = [...marquee.points, point];
					marquee.mergeMode = marqueeModeFromModifiers(event);
					updateMarqueeOverlay(point, marquee.mergeMode);
				}
				return;
			}
			if (panningRef.current) {
				if (isNavigator && panLastRef.current) {
					const last = panLastRef.current;
					emitCamera({
						...contentCamera,
						x: contentCamera.x - (point.x - last.x) / contentCamera.zoom,
						y: contentCamera.y - (point.y - last.y) / contentCamera.zoom,
					});
					panLastRef.current = point;
				} else {
					renderer.session.pointerMoveScreen(point.x, point.y);
					applyCompositeCamera(renderer.mirrorCameraFromSession());
				}
				renderer.invalidate();
				return;
			}
			if (!isNavigator && !canvasPick.pickMenuOpen && isRasterSelectionTool(activeTool)) {
				canvasPick.onCanvasPointerMove({ x: event.clientX, y: event.clientY });
			}
			if (!isNavigator) {
				renderer.session.pointerMoveScreen(point.x, point.y);
				renderer.invalidate();
			}
		},
		[activeTool, applyCompositeCamera, canvasPick, clientPoint, contentCamera, emitCamera, isNavigator, renderer, selectionMethod, updateMarqueeOverlay],
	);

	const handlePointerUp = useCallback(
		(event: React.PointerEvent) => {
			const point = clientPoint(event);
			const marquee = marqueeRef.current;
			if (activeTool === "selectWand" && !marquee.tracking) {
				canvasPick.onCanvasPointerUp(
					{ x: event.clientX, y: event.clientY },
					{ shift: event.shiftKey, ctrl: event.ctrlKey, meta: event.metaKey, alt: event.altKey },
				);
			}
			if (marquee.tracking) {
				const distance = Math.hypot(point.x - marquee.start.x, point.y - marquee.start.y);
				const mergeMode = marqueeModeFromModifiers(event);
				if (marquee.active && distance >= RASTER_MARQUEE_THRESHOLD_PX) {
					commitMarqueeSelection(point, mergeMode);
				} else if (distance < RASTER_MARQUEE_THRESHOLD_PX && (activeTool === "selectWand" || selectionMethod)) {
					canvasPick.onCanvasPointerUp(
						{ x: event.clientX, y: event.clientY },
						{ shift: event.shiftKey, ctrl: event.ctrlKey, meta: event.metaKey, alt: event.altKey },
					);
				}
				marquee.tracking = false;
				marquee.active = false;
				marquee.points = [];
				setMarqueeOverlay(null);
				return;
			}
			if (panningRef.current) {
				if (!isNavigator) renderer.session.pointerUpScreen(point.x, point.y);
				panningRef.current = false;
				panLastRef.current = null;
				if (!isNavigator) applyCompositeCamera(renderer.mirrorCameraFromSession());
				renderer.invalidate();
				return;
			}
			renderer.session.pointerUpScreen(point.x, point.y);
			renderer.invalidate();
		},
		[
			activeTool,
			applyCompositeCamera,
			canvasPick,
			clientPoint,
			commitMarqueeSelection,
			emitCamera,
			isNavigator,
			onSelect,
			renderer,
			selectedIds,
			selectionMethod,
		],
	);

	return (
		<div ref={containerRef} className={cn("relative h-full min-h-0 w-full min-w-0 overflow-hidden", className)}>
			<canvas
				ref={canvasRef}
				className="absolute inset-0 h-full w-full touch-none"
				onPointerDown={handlePointerDown}
				onPointerMove={handlePointerMove}
				onPointerUp={handlePointerUp}
				onPointerLeave={handlePointerUp}
			/>
			{attachError ? (
				<div className="bg-window/80 text-muted-foreground pointer-events-none absolute inset-0 flex items-center justify-center p-double text-center text-sm">
					Canvas unavailable: {attachError}
				</div>
			) : null}
			{navigatorOverlay ? (
				<div
					className="border-active-base/80 bg-active-base/10 pointer-events-none absolute z-10 border"
					style={{
						left: navigatorOverlay.x,
						top: navigatorOverlay.y,
						width: navigatorOverlay.width,
						height: navigatorOverlay.height,
					}}
				/>
			) : null}
			{marqueeOverlay?.shape === "rect" ? (
				<SelectionMarquee coverage={marqueeOverlay.coverage} shape="rect" rect={marqueeOverlay.rect} />
			) : null}
			{marqueeOverlay?.shape === "polygon" ? (
				<SelectionMarquee coverage={marqueeOverlay.coverage} shape="polygon" points={marqueeOverlay.points} />
			) : null}
			<CanvasPickMenu
				request={canvasPick.pickMenu}
				hoveredKey={canvasPick.menuHoveredKey}
				onHoverKey={canvasPick.onMenuHoverKey}
				onPick={canvasPick.onMenuPick}
				onDismiss={canvasPick.dismissPickMenu}
			/>
		</div>
	);
};

export const RasterLayerView: React.FC<Omit<RasterCanvasProps, "viewMode">> = (props) => (
	<RasterCanvas {...props} viewMode="layer" isolatedLayerId={props.isolatedLayerId ?? props.selectedIds?.[0] ?? null} />
);

export const RasterMaskView: React.FC<Omit<RasterCanvasProps, "viewMode">> = (props) => (
	<RasterCanvas {...props} viewMode="mask" isolatedLayerId={props.isolatedLayerId ?? props.selectedIds?.[0] ?? null} />
);
// #endregion 🖼️Canvas

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("parseCameraJson", () => {
		it("parses session camera json", () => {
			expect(parseCameraJson('{"x":1,"y":2,"zoom":3}')).toEqual({ x: 1, y: 2, zoom: 3 });
		});
	});

	describe("RasterRenderer", () => {
		it("syncDocument accepts every play fixture json", async () => {
			const fixtureModules = (await import("../../core/js/example-slugs.ts")) as typeof import("../../core/js/example-slugs.ts");
			const glob = await import.meta.glob<string>("../../example/*.raster.json", { eager: true, import: "default" });
			const renderer = new RasterRenderer();
			for (const [path, json] of Object.entries(glob)) {
				const id = path.split("/").pop()?.replace(/\.raster\.json$/, "") ?? path;
				const doc = JSON.parse(typeof json === "string" ? json : JSON.stringify(json));
				expect(() => renderer.syncDocument(doc)).not.toThrow();
				void id;
			}
			void fixtureModules;
		});
	});
}
// #endregion 🧪Tests
