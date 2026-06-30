// #region 🧲Header
/** @emoji 🖼️ Raster React host: WASM {@link RasterSession}, {@link RasterCanvas}, per-layer views. */
// #endregion 🧲Header

// #region 🔌Adapters
import React, { useCallback, useEffect, useLayoutEffect, useRef, useState, type ReactNode } from "react";
import {
	cn,
	marqueeCoverageFromGesture,
	marqueeModeFromModifiers,
	SelectionMarquee,
	selectionMergeIds,
	screenRectFromPoints,
	type SelectionMarqueeCoverage,
	type SelectionMarqueePoint,
	type SelectionMarqueeMethod,
	type SelectionMarqueeRect,
	type SelectionMergeMode,
} from "@semio-tech/ui-react";
import {
	decodeRasterImageAsset,
	rasterDocumentToSyncJson,
	resolveRasterLayerAtScreenPoint,
	resolveRasterMarqueeLayerHits,
	type RasterCamera,
	type RasterDocument,
	type RasterHoverPayload,
	type RasterKindHover,
	type RasterToolId,
} from "@semio-tech/raster-core";
import initRasterWasm, { RasterSession } from "../rs/pkg/raster.js";

const rasterWasmLoadedSync = Boolean(import.meta.vitest || (typeof process !== "undefined" && process.env.VITEST));

if (rasterWasmLoadedSync) {
	const { readFileSync } = await import("node:fs");
	const { initSync } = await import("../rs/pkg/raster.js");
	const wasmPath = new URL("../rs/pkg/raster_bg.wasm", import.meta.url).pathname;
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
	onCameraChange,
	onHover,
	onSelect,
}) => {
	const renderer = React.useMemo(() => new RasterRenderer(), []);
	const canvasRef = useRef<HTMLCanvasElement | null>(null);
	const containerRef = useRef<HTMLDivElement | null>(null);
	const viewportRef = useRef({ width: 1, height: 1 });
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

	useEffect(() => () => renderer.dispose(), [renderer]);

	useLayoutEffect(() => {
		renderer.setViewMode(viewMode, isolatedLayerId);
		renderer.invalidate();
	}, [renderer, viewMode, isolatedLayerId]);

	useEffect(() => {
		renderer.syncDocument(document);
		renderer.invalidate();
	}, [renderer, document]);

	useEffect(() => {
		if (!gpuAttached) return;
		renderer.syncDocument(document);
		if (camera) renderer.syncCamera(camera);
		renderer.invalidate();
	}, [gpuAttached, renderer, document, camera]);

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

	useEffect(() => {
		if (!camera) return;
		renderer.syncCamera(camera);
		renderer.invalidate();
	}, [renderer, camera]);

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
				setGpuAttached(true);
				renderer.invalidate();
			} catch (error) {
				attachStarted = false;
				const message = error instanceof Error ? error.message : String(error);
				if (message.includes("already attached")) {
					setAttachError(null);
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
	}, [renderer]);

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

	const handleWheel = useCallback(
		(event: WheelEvent) => {
			event.preventDefault();
			const point = clientPointFromCanvas(canvasRef.current, event);
			renderer.session.wheelScreen(point.x, point.y, event.deltaY);
			onCameraChange?.(renderer.mirrorCameraFromSession());
			renderer.invalidate();
		},
		[renderer, onCameraChange],
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
				setMarqueeOverlay(null);
				(event.target as HTMLElement).setPointerCapture(event.pointerId);
				return;
			}
			if (event.button === 1 || (!isRasterSelectionTool(activeTool) && event.button === 0)) {
				renderer.session.pointerDownScreen(point.x, point.y, event.button);
				(event.target as HTMLElement).setPointerCapture(event.pointerId);
			}
		},
		[activeTool, clientPoint, renderer, selectionMethod],
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
			const hits = resolveRasterMarqueeLayerHits(document, camera ?? document.camera, viewportRef.current, rect, coverage === "partial");
			onSelect?.(selectionMergeIds(mergeMode, selectedIds, hits));
		},
		[camera, document, onSelect, selectedIds, selectionMethod],
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
			renderer.session.pointerMoveScreen(point.x, point.y);
			onCameraChange?.(renderer.mirrorCameraFromSession());
			renderer.invalidate();
		},
		[clientPoint, onCameraChange, renderer, selectionMethod, updateMarqueeOverlay],
	);

	const handlePointerUp = useCallback(
		(event: React.PointerEvent) => {
			const point = clientPoint(event);
			const marquee = marqueeRef.current;
			if (marquee.tracking) {
				const distance = Math.hypot(point.x - marquee.start.x, point.y - marquee.start.y);
				const mergeMode = marqueeModeFromModifiers(event);
				if (marquee.active && distance >= RASTER_MARQUEE_THRESHOLD_PX) {
					commitMarqueeSelection(point, mergeMode);
				} else if (activeTool === "selectWand") {
					const hit = resolveRasterLayerAtScreenPoint(document, camera ?? document.camera, viewportRef.current, point);
					onSelect?.(selectionMergeIds(mergeMode, selectedIds, hit ? [hit] : []));
				} else if (distance < RASTER_MARQUEE_THRESHOLD_PX && selectionMethod) {
					const hit = resolveRasterLayerAtScreenPoint(document, camera ?? document.camera, viewportRef.current, point);
					onSelect?.(selectionMergeIds(mergeMode, selectedIds, hit ? [hit] : []));
				}
				marquee.tracking = false;
				marquee.active = false;
				marquee.points = [];
				setMarqueeOverlay(null);
				return;
			}
			renderer.session.pointerUpScreen(point.x, point.y);
			onCameraChange?.(renderer.mirrorCameraFromSession());
			renderer.invalidate();
		},
		[
			activeTool,
			camera,
			clientPoint,
			commitMarqueeSelection,
			document,
			onCameraChange,
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
			{marqueeOverlay?.shape === "rect" ? (
				<SelectionMarquee coverage={marqueeOverlay.coverage} shape="rect" rect={marqueeOverlay.rect} />
			) : null}
			{marqueeOverlay?.shape === "polygon" ? (
				<SelectionMarquee coverage={marqueeOverlay.coverage} shape="polygon" points={marqueeOverlay.points} />
			) : null}
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
			const fixtureModules = (await import("../play/fixture-slugs.ts")) as typeof import("../play/fixture-slugs.ts");
			const glob = await import.meta.glob<string>("../fixture/*.raster.json", { eager: true, import: "default" });
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
