// #region 🧲Header
/** @emoji ✏️ Draw React host: SVG infinite canvas, kernel-backed booleans and trace. */
// #endregion 🧲Header

// #region 🔌Adapters
import React, { useCallback, useEffect, useMemo, useRef, useState } from "react";
import {
	cn,
	marqueeCoverageFromGesture,
	SelectionMarquee,
	selectionMergeIds,
	screenRectFromPoints,
	type SelectionMarqueeCoverage,
	type SelectionMarqueePoint,
	type SelectionMarqueeRect,
	type SelectionMergeMode,
} from "@semio-tech/ui-react";
import {
	createDefaultDrawingWasmBridge,
	type DrawingExportBridge,
	type PathSegment as KernelPathSegment,
} from "@semio-tech/geometry-drawing-js";
import {
	findDrawLayer,
	flattenDrawDocumentToSceneNodes,
	layerToPathSegments,
	resolveDrawLayerAtScreenPoint,
	resolveDrawMarqueeLayerHits,
	type DrawBooleanOp,
	type DrawCamera,
	type DrawDocument,
	type DrawHoverPayload,
	type DrawImageAsset,
	type DrawKindHover,
	type DrawSceneNode,
	type DrawToolId,
	type FillStyle,
	type PathSegment,
	type StrokeStyle,
} from "@semio-tech/draw-core";

const DRAW_MARQUEE_THRESHOLD_PX = 4;

function rgbaCss(color: readonly [number, number, number, number]): string {
	return `rgba(${Math.round(color[0] * 255)},${Math.round(color[1] * 255)},${Math.round(color[2] * 255)},${color[3]})`;
}

function segmentsToPathD(segments: readonly PathSegment[]): string {
	let d = "";
	for (const segment of segments) {
		if (segment.kind === "move") d += `M ${segment.to[0]} ${segment.to[1]} `;
		else if (segment.kind === "line") d += `L ${segment.to[0]} ${segment.to[1]} `;
		else if (segment.kind === "quad") d += `Q ${segment.ctrl[0]} ${segment.ctrl[1]} ${segment.to[0]} ${segment.to[1]} `;
		else if (segment.kind === "cubic")
			d += `C ${segment.ctrl1[0]} ${segment.ctrl1[1]} ${segment.ctrl2[0]} ${segment.ctrl2[1]} ${segment.to[0]} ${segment.to[1]} `;
		else if (segment.kind === "arc")
			d += `A ${segment.rx} ${segment.ry} ${segment.rotation} ${segment.largeArc ? 1 : 0} ${segment.sweep ? 1 : 0} ${segment.to[0]} ${segment.to[1]} `;
		else if (segment.kind === "close") d += "Z ";
	}
	return d.trim();
}

function decodeDrawImageAsset(asset: DrawImageAsset): Promise<{ width: number; height: number; luma: Uint8Array } | null> {
	if (typeof document === "undefined") return null;
	return new Promise((resolve) => {
		const img = new Image();
		const dataUrl = asset.data.startsWith("data:") ? asset.data : `data:${asset.mime};base64,${asset.data}`;
		img.onload = () => {
			const canvas = document.createElement("canvas");
			canvas.width = asset.width ?? img.naturalWidth;
			canvas.height = asset.height ?? img.naturalHeight;
			const ctx = canvas.getContext("2d");
			if (!ctx) {
				resolve(null);
				return;
			}
			ctx.drawImage(img, 0, 0, canvas.width, canvas.height);
			const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
			const luma = new Uint8Array(canvas.width * canvas.height);
			for (let i = 0; i < luma.length; i += 1) {
				const offset = i * 4;
				const r = imageData.data[offset] ?? 0;
				const g = imageData.data[offset + 1] ?? 0;
				const b = imageData.data[offset + 2] ?? 0;
				const a = imageData.data[offset + 3] ?? 255;
				luma[i] = Math.round((r * 0.299 + g * 0.587 + b * 0.114) * (a / 255));
			}
			resolve({ width: canvas.width, height: canvas.height, luma });
		};
		img.onerror = () => resolve(null);
		img.src = dataUrl;
	});
}

let drawBridgePromise: Promise<DrawingExportBridge> | null = null;

export async function ensureDrawKernelBridge(): Promise<DrawingExportBridge> {
	if (!drawBridgePromise) drawBridgePromise = createDefaultDrawingWasmBridge();
	return drawBridgePromise;
}

async function resolveSceneNodeSegments(
	doc: DrawDocument,
	node: DrawSceneNode,
	bridge: DrawingExportBridge,
	cache: Map<string, PathSegment[]>,
): Promise<PathSegment[]> {
	const cached = cache.get(node.id);
	if (cached) return cached;
	if (!node.needsKernel) {
		cache.set(node.id, [...node.segments]);
		return [...node.segments];
	}
	if (node.kernelKind === "boolean" && node.kernelPayload && typeof node.kernelPayload === "object") {
		const payload = node.kernelPayload as { op: DrawBooleanOp; children: string[] };
		const childSegments: KernelPathSegment[][] = [];
		for (const childId of payload.children) {
			const childLayer = findDrawLayer(doc, childId);
			if (!childLayer) continue;
			childSegments.push(layerToPathSegments(childLayer) as KernelPathSegment[]);
		}
		if (childSegments.length === 0) return [];
		let acc = childSegments[0]!;
		for (let i = 1; i < childSegments.length; i += 1) {
			acc = bridge.booleanPaths(acc, childSegments[i]!, payload.op) as PathSegment[];
		}
		cache.set(node.id, acc);
		console.log("[DEBUG] draw boolean resolved", { id: node.id, op: payload.op, points: acc.length });
		return acc;
	}
	if (node.kernelKind === "trace" && node.kernelPayload && typeof node.kernelPayload === "object") {
		const payload = node.kernelPayload as { sourceKey: string; params: { threshold: number; simplifyEpsilon: number } };
		const asset = doc.assets?.[payload.sourceKey];
		if (!asset) return [];
		const decoded = await decodeDrawImageAsset(asset);
		if (!decoded) return [];
		const traced = bridge.traceBitmap(
			decoded.width,
			decoded.height,
			decoded.luma,
			payload.params.threshold,
			payload.params.simplifyEpsilon,
		) as PathSegment[];
		cache.set(node.id, traced);
		console.log("[DEBUG] draw trace resolved", { id: node.id, segments: traced.length });
		return traced;
	}
	return [];
}

export type DrawViewMode = "composite" | "navigator";

export interface DrawCanvasProps {
	readonly document: DrawDocument;
	readonly viewMode?: DrawViewMode;
	readonly camera?: DrawCamera;
	readonly selectedIds?: readonly string[];
	readonly hoveredId?: string | null;
	readonly kindHover?: DrawKindHover | null;
	readonly activeTool?: DrawToolId;
	readonly className?: string;
	readonly onCameraChange?: (camera: DrawCamera) => void;
	readonly onHover?: (payload: DrawHoverPayload) => void;
	readonly onSelect?: (ids: readonly string[]) => void;
	readonly onDocumentChange?: (document: DrawDocument) => void;
}

function DrawPathShape({
	segments,
	fill,
	stroke,
	opacity,
	selected,
	hovered,
}: {
	readonly segments: readonly PathSegment[];
	readonly fill?: FillStyle;
	readonly stroke?: StrokeStyle;
	readonly opacity: number;
	readonly selected: boolean;
	readonly hovered: boolean;
}): React.JSX.Element | null {
	const d = segmentsToPathD(segments);
	if (!d) return null;
	const fillValue = fill?.kind === "solid" ? rgbaCss(fill.color) : "none";
	const strokeValue = stroke ? rgbaCss(stroke.color) : selected ? "#3b82f6" : hovered ? "#60a5fa" : "none";
	const strokeWidth = stroke?.width ?? (selected || hovered ? 2 : 0);
	return (
		<path
			d={d}
			fill={fillValue}
			stroke={strokeValue}
			strokeWidth={strokeWidth}
			opacity={opacity}
			vectorEffect="non-scaling-stroke"
		/>
	);
}

export function DrawCanvas({
	document: doc,
	viewMode = "composite",
	camera: cameraProp,
	selectedIds = [],
	hoveredId = null,
	activeTool = "selectDirect",
	className,
	onCameraChange,
	onHover,
	onSelect,
}: DrawCanvasProps): React.JSX.Element {
	const containerRef = useRef<HTMLDivElement>(null);
	const [camera, setCamera] = useState<DrawCamera>(cameraProp ?? doc.camera);
	const [resolved, setResolved] = useState<ReadonlyArray<{ node: DrawSceneNode; segments: PathSegment[] }>>([]);
	const [marquee, setMarquee] = useState<
		| { readonly coverage: SelectionMarqueeCoverage; readonly shape: "rect"; readonly rect: SelectionMarqueeRect }
		| { readonly coverage: SelectionMarqueeCoverage; readonly shape: "polygon"; readonly points: readonly SelectionMarqueePoint[] }
		| null
	>(null);
	const dragRef = useRef<{ kind: "pan" | "marquee"; startX: number; startY: number; merge: SelectionMergeMode } | null>(null);

	useEffect(() => {
		if (cameraProp) setCamera(cameraProp);
	}, [cameraProp]);

	const sceneNodes = useMemo(() => flattenDrawDocumentToSceneNodes(doc), [doc]);

	useEffect(() => {
		let cancelled = false;
		void (async () => {
			const bridge = await ensureDrawKernelBridge();
			const cache = new Map<string, PathSegment[]>();
			const next: Array<{ node: DrawSceneNode; segments: PathSegment[] }> = [];
			for (const node of sceneNodes) {
				const segments = await resolveSceneNodeSegments(doc, node, bridge, cache);
				next.push({ node, segments });
			}
			if (!cancelled) setResolved(next);
		})();
		return () => {
			cancelled = true;
		};
	}, [doc, sceneNodes]);

	const emitCamera = useCallback(
		(next: DrawCamera) => {
			setCamera(next);
			onCameraChange?.(next);
		},
		[onCameraChange],
	);

	const onWheel = useCallback(
		(event: React.WheelEvent) => {
			event.preventDefault();
			const factor = event.deltaY < 0 ? 1.1 : 0.9;
			emitCamera({ ...camera, zoom: Math.max(0.05, Math.min(32, camera.zoom * factor)) });
		},
		[camera, emitCamera],
	);

	const onPointerDown = useCallback(
		(event: React.PointerEvent) => {
			const rect = containerRef.current?.getBoundingClientRect();
			if (!rect) return;
			const x = event.clientX - rect.left;
			const y = event.clientY - rect.top;
			if (event.button === 1 || activeTool === "transformMove") {
				dragRef.current = { kind: "pan", startX: event.clientX, startY: event.clientY, merge: "default" };
				return;
			}
			if (activeTool === "selectMarquee") {
				dragRef.current = {
					kind: "marquee",
					startX: x,
					startY: y,
					merge: event.shiftKey ? "add" : event.altKey ? "subtract" : "default",
				};
				setMarquee({ coverage: "partial", shape: "rect", rect: screenRectFromPoints({ x, y }, { x, y }) });
				return;
			}
			const hit = resolveDrawLayerAtScreenPoint(doc, camera, { width: rect.width, height: rect.height }, { x, y });
			onSelect?.(hit ? [hit] : []);
			console.log("[DEBUG] draw canvas select", { hit, x, y });
		},
		[activeTool, camera, doc, onSelect],
	);

	const onPointerMove = useCallback(
		(event: React.PointerEvent) => {
			const rect = containerRef.current?.getBoundingClientRect();
			if (!rect) return;
			const drag = dragRef.current;
			if (!drag) {
				const x = event.clientX - rect.left;
				const y = event.clientY - rect.top;
				const hit = resolveDrawLayerAtScreenPoint(doc, camera, { width: rect.width, height: rect.height }, { x, y });
				onHover?.({ id: hit, kind: hit ? { domain: "layer", kindId: hit } : null });
				return;
			}
			if (drag.kind === "pan") {
				const dx = (event.clientX - drag.startX) / camera.zoom;
				const dy = (event.clientY - drag.startY) / camera.zoom;
				emitCamera({ ...camera, x: camera.x - dx, y: camera.y - dy });
				dragRef.current = { ...drag, startX: event.clientX, startY: event.clientY };
				return;
			}
			const x = event.clientX - rect.left;
			const y = event.clientY - rect.top;
			setMarquee({ coverage: "partial", shape: "rect", rect: screenRectFromPoints({ x: drag.startX, y: drag.startY }, { x, y }) });
		},
		[camera, doc, emitCamera, onHover],
	);

	const onPointerUp = useCallback(
		(event: React.PointerEvent) => {
			const rect = containerRef.current?.getBoundingClientRect();
			const drag = dragRef.current;
			dragRef.current = null;
			if (!rect || !drag || drag.kind !== "marquee" || !marquee || marquee.shape !== "rect") {
				setMarquee(null);
				return;
			}
			const x = event.clientX - rect.left;
			const y = event.clientY - rect.top;
			const width = Math.abs(x - drag.startX);
			const height = Math.abs(y - drag.startY);
			if (width < DRAW_MARQUEE_THRESHOLD_PX && height < DRAW_MARQUEE_THRESHOLD_PX) {
				setMarquee(null);
				return;
			}
			const rectMarquee = screenRectFromPoints({ x: drag.startX, y: drag.startY }, { x, y });
			const crossing = marqueeCoverageFromGesture(rectMarquee, { x: drag.startX, y: drag.startY }, { x, y }) === "crossing";
			const hits = resolveDrawMarqueeLayerHits(doc, camera, { width: rect.width, height: rect.height }, rectMarquee, crossing);
			onSelect?.(selectionMergeIds(selectedIds, hits, drag.merge));
			setMarquee(null);
		},
		[camera, doc, marquee, onSelect, selectedIds],
	);

	const transform = `translate(${camera.x * -camera.zoom + (containerRef.current?.clientWidth ?? 0) / 2}, ${camera.y * -camera.zoom + (containerRef.current?.clientHeight ?? 0) / 2}) scale(${camera.zoom})`;

	return (
		<div
			ref={containerRef}
			className={cn("relative h-full w-full overflow-hidden bg-neutral-950 touch-none", className)}
			onWheel={onWheel}
			onPointerDown={onPointerDown}
			onPointerMove={onPointerMove}
			onPointerUp={onPointerUp}
			onPointerLeave={() => onHover?.({ id: null, kind: null })}
		>
			<svg className="h-full w-full" viewBox={`0 0 ${containerRef.current?.clientWidth ?? 1024} ${containerRef.current?.clientHeight ?? 768}`}>
				<g transform={transform}>
					{viewMode === "composite" ? (
						resolved.map(({ node, segments }) => (
							<DrawPathShape
								key={node.id}
								segments={segments}
								fill={node.fill}
								stroke={node.stroke}
								opacity={node.opacity}
								selected={selectedIds.includes(node.id)}
								hovered={hoveredId === node.id}
							/>
						))
					) : (
						<rect x={-512} y={-512} width={1024} height={1024} fill="none" stroke="#334155" strokeWidth={1} vectorEffect="non-scaling-stroke" />
					)}
				</g>
			</svg>
			{marquee ? <SelectionMarquee marquee={marquee} /> : null}
		</div>
	);
}

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("@semio-tech/draw-react", () => {
		it("encodes path d strings", () => {
			const d = segmentsToPathD([
				{ kind: "move", to: [0, 0] },
				{ kind: "line", to: [10, 0] },
				{ kind: "close" },
			]);
			expect(d).toContain("M 0 0");
			expect(d).toContain("Z");
		});
	});
}
// #endregion 🧪Tests
