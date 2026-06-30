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
	type SelectionMarqueeCoverage,
	type SelectionMarqueePoint,
	type SelectionMergeMode,
} from "@semio-tech/ui-react";
import {
	rasterDocumentToJson,
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
// #endregion 🔌Adapters

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
		const json = rasterDocumentToJson(doc);
		if (json !== this.documentJson) {
			this.documentJson = json;
			this.session.syncDocumentJson(json);
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
	const canvasRef = useRef<HTMLCanvasElement | null>(null);
	const rendererRef = useRef<RasterRenderer | null>(null);
	const containerRef = useRef<HTMLDivElement | null>(null);
	const panningRef = useRef(false);
	const marqueeRef = useRef<{ start: SelectionMarqueePoint; end: SelectionMarqueePoint } | null>(null);
	const [marqueeCoverage, setMarqueeCoverage] = useState<SelectionMarqueeCoverage | null>(null);
	const [attachError, setAttachError] = useState<string | null>(null);

	useEffect(() => {
		const renderer = new RasterRenderer();
		rendererRef.current = renderer;
		return () => renderer.dispose();
	}, []);

	useLayoutEffect(() => {
		rendererRef.current?.setViewMode(viewMode, isolatedLayerId);
		rendererRef.current?.invalidate();
	}, [viewMode, isolatedLayerId]);

	useEffect(() => {
		const renderer = rendererRef.current;
		if (!renderer) return;
		renderer.syncDocument(document);
		renderer.invalidate();
	}, [document]);

	useEffect(() => {
		rendererRef.current?.syncSelection(selectedIds);
		rendererRef.current?.invalidate();
	}, [selectedIds]);

	useEffect(() => {
		rendererRef.current?.syncHover(hoveredId, kindHover);
		rendererRef.current?.invalidate();
	}, [hoveredId, kindHover]);

	useEffect(() => {
		if (activeTool) rendererRef.current?.syncTool(activeTool);
	}, [activeTool]);

	useEffect(() => {
		if (!camera) return;
		rendererRef.current?.syncCamera(camera);
		rendererRef.current?.invalidate();
	}, [camera]);

	useLayoutEffect(() => {
		const canvas = canvasRef.current;
		const container = containerRef.current;
		const renderer = rendererRef.current;
		if (!canvas || !container || !renderer) return;

		let disposed = false;
		const resize = () => {
			const rect = container.getBoundingClientRect();
			const dpr = window.devicePixelRatio || 1;
			const w = Math.max(1, Math.floor(rect.width));
			const h = Math.max(1, Math.floor(rect.height));
			if (renderer.session.gpuReady()) {
				renderer.setSize(w, h, dpr);
			}
		};

		void (async () => {
			const rect = container.getBoundingClientRect();
			const dpr = window.devicePixelRatio || 1;
			const w = Math.max(1, Math.floor(rect.width));
			const h = Math.max(1, Math.floor(rect.height));
			if (disposed) return;
			try {
				await renderer.attachCanvas(canvas, w, h, dpr);
				if (disposed) return;
				setAttachError(null);
				if (camera) renderer.syncCamera(camera);
				renderer.syncDocument(document);
			} catch (error) {
				const message = error instanceof Error ? error.message : String(error);
				if (message.includes("already attached")) {
					setAttachError(null);
					resize();
					if (camera) renderer.syncCamera(camera);
					renderer.syncDocument(document);
					return;
				}
				setAttachError(message);
				console.error("[DEBUG] raster canvas attach failed", error);
			}
		})();

		const observer = new ResizeObserver(resize);
		observer.observe(container);
		return () => {
			disposed = true;
			observer.disconnect();
		};
	}, []);

	const clientPoint = useCallback((event: React.PointerEvent | React.WheelEvent): { x: number; y: number } => {
		const canvas = canvasRef.current;
		if (!canvas) return { x: 0, y: 0 };
		const rect = canvas.getBoundingClientRect();
		return { x: event.clientX - rect.left, y: event.clientY - rect.top };
	}, []);

	const handleWheel = useCallback(
		(event: React.WheelEvent) => {
			event.preventDefault();
			const renderer = rendererRef.current;
			if (!renderer) return;
			const point = clientPoint(event);
			renderer.session.wheelScreen(point.x, point.y, event.deltaY);
			onCameraChange?.(renderer.mirrorCameraFromSession());
			renderer.invalidate();
		},
		[clientPoint, onCameraChange],
	);

	const handlePointerDown = useCallback(
		(event: React.PointerEvent) => {
			const renderer = rendererRef.current;
			if (!renderer) return;
			const point = clientPoint(event);
			if (event.button === 1 || (activeTool !== "selectMarquee" && event.button === 0)) {
				panningRef.current = event.button === 1;
				renderer.session.pointerDownScreen(point.x, point.y, event.button);
				(event.target as HTMLElement).setPointerCapture(event.pointerId);
			}
			if (activeTool === "selectMarquee" && event.button === 0) {
				marqueeRef.current = { start: point, end: point };
				setMarqueeCoverage(marqueeCoverageFromGesture(point, point, "replace"));
			}
		},
		[activeTool, clientPoint],
	);

	const handlePointerMove = useCallback(
		(event: React.PointerEvent) => {
			const renderer = rendererRef.current;
			if (!renderer) return;
			const point = clientPoint(event);
			if (marqueeRef.current) {
				marqueeRef.current.end = point;
				setMarqueeCoverage(
					marqueeCoverageFromGesture(marqueeRef.current.start, point, marqueeModeFromModifiers(event) as SelectionMergeMode),
				);
			}
			renderer.session.pointerMoveScreen(point.x, point.y);
			onCameraChange?.(renderer.mirrorCameraFromSession());
			renderer.invalidate();
		},
		[clientPoint, onCameraChange],
	);

	const handlePointerUp = useCallback(
		(event: React.PointerEvent) => {
			const renderer = rendererRef.current;
			if (!renderer) return;
			const point = clientPoint(event);
			renderer.session.pointerUpScreen(point.x, point.y);
			panningRef.current = false;
			if (marqueeRef.current) {
				marqueeRef.current = null;
				setMarqueeCoverage(null);
			}
			onCameraChange?.(renderer.mirrorCameraFromSession());
			renderer.invalidate();
		},
		[clientPoint, onCameraChange],
	);

	return (
		<div ref={containerRef} className={cn("relative h-full min-h-0 w-full min-w-0 overflow-hidden", className)}>
			<canvas
				ref={canvasRef}
				className="absolute inset-0 h-full w-full touch-none"
				onWheel={handleWheel}
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
			{marqueeCoverage ? <SelectionMarquee coverage={marqueeCoverage} /> : null}
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
