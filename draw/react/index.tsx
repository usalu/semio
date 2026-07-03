// #region 🧲Header
/** @emoji ✏️ Draw React host: SVG infinite canvas, kernel-backed booleans and trace. */
// #endregion 🧲Header

// #region 🔌Adapters
import React, { useCallback, useEffect, useMemo, useRef, useState } from "react";
import {
	cn,
	CanvasPickMenu,
	marqueeCoverageFromGesture,
	marqueeModeFromModifiers,
	SelectionMarquee,
	selectionMergeIds,
	screenRectFromPoints,
	useCanvasPickInteraction,
	type SelectionMarqueeCoverage,
	type SelectionMarqueeMethod,
	type SelectionMarqueePoint,
	type SelectionMarqueeRect,
	type SelectionMergeMode,
} from "@semio-tech/ui-react";
import { parseCanvasPickTargetKey, type CanvasPickTarget } from "@semio-tech/framework-core";
import { resolveSemanticColorHex } from "@semio-tech/ui-styling";
import {
	createDefaultDrawingWasmBridge,
	type DrawingExportBridge,
	type PathSegment as KernelPathSegment,
} from "@semio-tech/kernel-2d-js";
import {
	applyDrawEditOp,
	createDrawPathLayer,
	createDrawShapeLayer,
	createDrawGroupLayer,
	createDrawTraceLayer,
	defaultDrawTraceParams,
	defaultDrawDocument,
	findDrawLayer,
	flattenDrawDocumentToSceneNodes,
	drawLayerDescendantLeafIds,
	layerToPathSegments,
	resolveDrawLayerAtScreenPoint,
	resolveDrawPickTargetsAtScreenPoint,
	resolveDrawMarqueeLayerHits,
	filterPathSegmentsByContourArea,
	scalePathSegments,
	splitPathSegmentsByContour,
	resolveDrawDocumentArtboard,
	transformPathSegments,
	type DrawBooleanOp,
	type DrawBlendMode,
	type DrawCamera,
	type DrawDocument,
	type DrawHoverPayload,
	type DrawImageAsset,
	type DrawKindHover,
	type DrawSceneNode,
	type DrawScreenRect,
	type DrawToolId,
	type DrawPickTarget,
	type FillStyle,
	type PathSegment,
	type StrokeStyle,
	type Vec2,
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
	if (typeof document === "undefined") return Promise.resolve(null);
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
			const localSegments = layerToPathSegments(childLayer) as KernelPathSegment[];
			childSegments.push(transformPathSegments(localSegments, childLayer.transform) as KernelPathSegment[]);
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
		const artboard = resolveDrawDocumentArtboard(doc);
		const scaled =
			artboard && decoded.width > 0 && decoded.height > 0
				? scalePathSegments(traced, artboard.width / decoded.width, artboard.height / decoded.height)
				: traced;
		const filtered = filterPathSegmentsByContourArea(scaled, 6);
		cache.set(node.id, filtered);
		console.log("[DEBUG] draw trace resolved", { id: node.id, segments: filtered.length });
		return filtered;
	}
	return [];
}

export interface DrawCanvasProps {
	readonly document: DrawDocument;
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
	readonly onCommit?: (document: DrawDocument, selectLayerId?: string) => void;
}

function drawSelectionMethod(activeTool: DrawToolId | undefined): SelectionMarqueeMethod | null {
	if (activeTool === "selectMarquee") return "rectangle";
	if (activeTool === "selectLasso") return "lasso";
	return null;
}

function isDrawSelectionTool(activeTool: DrawToolId | undefined): boolean {
	return activeTool === "selectMarquee" || activeTool === "selectLasso" || activeTool === "selectDirect";
}

function isDrawAuthoringTool(activeTool: DrawToolId | undefined): boolean {
	return (
		activeTool === "pen" ||
		activeTool === "shapeRect" ||
		activeTool === "shapeEllipse" ||
		activeTool === "shapeLine" ||
		activeTool === "shapePolygon" ||
		activeTool === "trace"
	);
}

function drawHoverPayloadFromPickTarget(doc: DrawDocument, target: DrawPickTarget | null): DrawHoverPayload {
	if (!target) return { id: null, kind: null };
	const layerId = target.layerId ?? target.id;
	return { id: layerId, kind: drawKindHoverForLayerId(doc, layerId) };
}

function drawPickTargetToCanvas(target: DrawPickTarget): CanvasPickTarget {
	return { domain: target.domain, id: target.id, generality: target.generality, label: target.label, meta: target.layerId ? { layerId: target.layerId } : undefined };
}

function drawLayerIdFromCanvasPickTarget(target: CanvasPickTarget): string {
	const layerId = target.meta?.layerId;
	return typeof layerId === "string" ? layerId : target.id;
}

function drawHoverPayloadFromFocusKey(doc: DrawDocument, key: string | null): DrawHoverPayload {
	if (!key) return { id: null, kind: null };
	const parsed = parseCanvasPickTargetKey(key);
	if (!parsed) return { id: null, kind: null };
	if (parsed.domain === "controlPoint") {
		const layerId = key.split(":")[0] ?? parsed.id;
		return drawHoverPayloadFromPickTarget(doc, { domain: parsed.domain, id: parsed.id, generality: 4, layerId });
	}
	return drawHoverPayloadFromPickTarget(doc, { domain: parsed.domain, id: parsed.id, generality: 0 });
}

function drawKindHoverForLayerId(doc: DrawDocument, layerId: string): DrawKindHover {
	const layer = findDrawLayer(doc, layerId);
	const domain =
		layer?.kind === "group"
			? "group"
			: layer?.kind === "boolean"
				? "boolean"
				: layer?.kind === "trace"
					? "trace"
					: layer?.kind === "shape"
						? "shape"
						: "layer";
	return { domain, kindId: layerId };
}

function drawBlendModeCss(mode: DrawBlendMode): React.CSSProperties["mixBlendMode"] {
	switch (mode) {
		case "colorDodge":
			return "color-dodge";
		case "colorBurn":
			return "color-burn";
		case "hardLight":
			return "hard-light";
		case "softLight":
			return "soft-light";
		default:
			return mode;
	}
}

function DrawPathShape({
	segments,
	fill,
	stroke,
	opacity,
	kernelKind,
}: {
	readonly segments: readonly PathSegment[];
	readonly fill?: FillStyle;
	readonly stroke?: StrokeStyle;
	readonly opacity: number;
	readonly kernelKind?: "boolean" | "trace";
}): React.JSX.Element | null {
	const contours = splitPathSegmentsByContour(segments);
	const traceFillFromStroke = kernelKind === "trace" && !fill && stroke;
	const fillValue =
		fill?.kind === "solid" ? rgbaCss(fill.color) : traceFillFromStroke && stroke ? rgbaCss(stroke.color) : "none";
	const strokeValue = traceFillFromStroke ? "none" : stroke ? rgbaCss(stroke.color) : "none";
	const strokeWidth = traceFillFromStroke ? 0 : stroke?.width ?? 0;
	const paint = { fill: fillValue, stroke: strokeValue, strokeWidth, opacity, vectorEffect: "non-scaling-stroke" as const, fillRule: "evenodd" as const };
	const paths = contours
		.map((contour, index) => {
			const d = segmentsToPathD(contour);
			if (!d) return null;
			return <path key={index} d={d} {...paint} />;
		})
		.filter((node): node is React.JSX.Element => node !== null);
	if (paths.length === 0) return null;
	return <g>{paths}</g>;
}

function DrawLayerInteractionHighlight({
	node,
	segments,
	selected,
	hovered,
	stroke,
	activeColor,
	hoverColor,
	haloColor,
}: {
	readonly node: DrawSceneNode;
	readonly segments: readonly PathSegment[];
	readonly selected: boolean;
	readonly hovered: boolean;
	readonly stroke?: StrokeStyle;
	readonly activeColor: string;
	readonly hoverColor: string;
	readonly haloColor: string;
}): React.JSX.Element | null {
	const accent = selected ? activeColor : hovered ? hoverColor : null;
	if (!accent) return null;
	const dash = hovered && !selected ? "4 2" : undefined;
	const strokeWidth = Math.max(2.5, (stroke?.width ?? 0) + 2);
	const haloWidth = strokeWidth + 2.5;
	if (node.text) {
		const width = Math.max(8, node.text.content.length * node.text.size * 0.6);
		const height = Math.max(8, node.text.size * 1.2);
		return (
			<g pointerEvents="none">
				{selected ? <rect x={0} y={0} width={width} height={height} fill={activeColor} fillOpacity={0.12} stroke="none" /> : null}
				<rect x={0} y={0} width={width} height={height} fill="none" stroke={haloColor} strokeWidth={haloWidth} vectorEffect="non-scaling-stroke" />
				<rect x={0} y={0} width={width} height={height} fill="none" stroke={accent} strokeWidth={strokeWidth} strokeDasharray={dash} vectorEffect="non-scaling-stroke" />
			</g>
		);
	}
	if (node.image) {
		const width = node.image.width;
		const height = node.image.height;
		return (
			<g pointerEvents="none">
				{selected ? <rect x={0} y={0} width={width} height={height} fill={activeColor} fillOpacity={0.12} stroke="none" /> : null}
				<rect x={0} y={0} width={width} height={height} fill="none" stroke={haloColor} strokeWidth={haloWidth} vectorEffect="non-scaling-stroke" />
				<rect x={0} y={0} width={width} height={height} fill="none" stroke={accent} strokeWidth={strokeWidth} strokeDasharray={dash} vectorEffect="non-scaling-stroke" />
			</g>
		);
	}
	const contours = splitPathSegmentsByContour(segments);
	const paths = contours
		.map((contour, index) => {
			const d = segmentsToPathD(contour);
			if (!d) return null;
			return (
				<g key={index} pointerEvents="none">
					{selected ? <path d={d} fill={activeColor} fillOpacity={0.12} stroke="none" fillRule="evenodd" /> : null}
					<path d={d} fill="none" stroke={haloColor} strokeWidth={haloWidth} vectorEffect="non-scaling-stroke" fillRule="evenodd" />
					<path d={d} fill="none" stroke={accent} strokeWidth={strokeWidth} strokeDasharray={dash} vectorEffect="non-scaling-stroke" fillRule="evenodd" />
				</g>
			);
		})
		.filter((node): node is React.JSX.Element => node !== null);
	if (paths.length === 0) return null;
	return <g>{paths}</g>;
}

function DrawTextShape({
	content,
	size,
	fill,
	opacity,
}: {
	readonly content: string;
	readonly size: number;
	readonly fill?: FillStyle;
	readonly opacity: number;
}): React.JSX.Element {
	const fillValue = fill?.kind === "solid" ? rgbaCss(fill.color) : "#e2e8f0";
	return (
		<g opacity={opacity}>
			<text x={0} y={size} fontSize={size} fill={fillValue} fontFamily="ui-monospace, monospace">
				{content}
			</text>
		</g>
	);
}

function DrawImageShape({
	src,
	width,
	height,
	opacity,
}: {
	readonly src: string;
	readonly width: number;
	readonly height: number;
	readonly opacity: number;
}): React.JSX.Element | null {
	if (!src) return null;
	return (
		<g opacity={opacity}>
			<image href={src} x={0} y={0} width={width} height={height} />
		</g>
	);
}

function DrawPreviewPath({
	segments,
	stroke,
}: {
	readonly segments: readonly PathSegment[];
	readonly stroke: string;
}): React.JSX.Element | null {
	const d = segmentsToPathD(segments);
	if (!d) return null;
	return <path d={d} fill={`${stroke}26`} stroke={stroke} strokeWidth={1.5} vectorEffect="non-scaling-stroke" />;
}

type DrawDragState =
	| { readonly kind: "pan"; readonly startX: number; readonly startY: number }
	| {
			readonly kind: "marquee";
			readonly method: SelectionMarqueeMethod;
			readonly startX: number;
			readonly startY: number;
			readonly points: SelectionMarqueePoint[];
			readonly merge: SelectionMergeMode;
			readonly active: boolean;
	  }
	| { readonly kind: "shapeRect" | "shapeEllipse" | "shapeLine"; readonly startWorld: Vec2 }
	| { readonly kind: "pen" | "shapePolygon"; readonly points: Vec2[] }
	| null;

export function DrawCanvas({
	document: doc,
	camera: cameraProp,
	selectedIds = [],
	hoveredId = null,
	kindHover = null,
	activeTool = "selectDirect",
	className,
	onCameraChange,
	onHover,
	onSelect,
	onCommit,
}: DrawCanvasProps): React.JSX.Element {
	const containerRef = useRef<HTMLDivElement>(null);
	const [camera, setCamera] = useState<DrawCamera>(cameraProp ?? doc.camera);
	const [resolved, setResolved] = useState<ReadonlyArray<{ node: DrawSceneNode; segments: PathSegment[] }>>([]);
	const [marquee, setMarquee] = useState<
		| { readonly coverage: SelectionMarqueeCoverage; readonly shape: "rect"; readonly rect: SelectionMarqueeRect }
		| { readonly coverage: SelectionMarqueeCoverage; readonly shape: "polygon"; readonly points: readonly SelectionMarqueePoint[] }
		| null
	>(null);
	const [previewSegments, setPreviewSegments] = useState<PathSegment[]>([]);
	const dragRef = useRef<DrawDragState>(null);
	const selectionMethod = drawSelectionMethod(activeTool);

	const effectiveHoveredId = hoveredId ?? kindHover?.kindId ?? null;
	const activeColor = useMemo(() => resolveSemanticColorHex("--active-base", "gray"), []);
	const hoverColor = useMemo(() => resolveSemanticColorHex("--color-changed-hovered", "gray"), []);
	const haloColor = useMemo(() => resolveSemanticColorHex("--foreground", "gray"), []);
	const selectedLeafIds = useMemo(() => {
		const ids = new Set<string>();
		for (const layerId of selectedIds) for (const leafId of drawLayerDescendantLeafIds(doc, layerId)) ids.add(leafId);
		return ids;
	}, [doc, selectedIds]);
	const hoveredLeafIds = useMemo(() => {
		if (!effectiveHoveredId) return new Set<string>();
		return new Set(drawLayerDescendantLeafIds(doc, effectiveHoveredId));
	}, [doc, effectiveHoveredId]);

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

	const viewport = useCallback((): { width: number; height: number } => {
		const rect = containerRef.current?.getBoundingClientRect();
		return { width: rect?.width ?? 1024, height: rect?.height ?? 768 };
	}, []);

	const clientFromScreen = useCallback((screen: SelectionMarqueePoint): { x: number; y: number } => {
		const rect = containerRef.current?.getBoundingClientRect();
		return { x: (rect?.left ?? 0) + screen.x, y: (rect?.top ?? 0) + screen.y };
	}, []);

	const resolveTargetsAtClient = useCallback(
		(client: { readonly x: number; readonly y: number }): readonly CanvasPickTarget[] => {
			const rect = containerRef.current?.getBoundingClientRect();
			const screen = { x: client.x - (rect?.left ?? 0), y: client.y - (rect?.top ?? 0) };
			return resolveDrawPickTargetsAtScreenPoint(doc, camera, viewport(), screen, {
				includeControlPoints: activeTool === "selectDirect",
			}).map(drawPickTargetToCanvas);
		},
		[activeTool, camera, doc, viewport],
	);

	const canvasPick = useCanvasPickInteraction({
		resolveTargetsAtClient,
		onHoverFocus: (focus) => onHover?.(drawHoverPayloadFromFocusKey(doc, focus.targetKey)),
		onSelectTarget: (target, request) => {
			const layerId = drawLayerIdFromCanvasPickTarget(target);
			const merge = marqueeModeFromModifiers({
				shiftKey: request.modifiers?.shift === true,
				ctrlKey: request.modifiers?.ctrl === true,
				metaKey: request.modifiers?.meta === true,
				altKey: request.modifiers?.alt === true,
			});
			onSelect?.(selectionMergeIds(merge, selectedIds, [layerId]));
		},
	});

	const screenPoint = useCallback((event: React.PointerEvent | PointerEvent): SelectionMarqueePoint => {
		const rect = containerRef.current?.getBoundingClientRect();
		return { x: event.clientX - (rect?.left ?? 0), y: event.clientY - (rect?.top ?? 0) };
	}, []);

	const screenToWorld = useCallback(
		(point: SelectionMarqueePoint): Vec2 => {
			const vp = viewport();
			return [
				(point.x - vp.width / 2) / camera.zoom + camera.x,
				(point.y - vp.height / 2) / camera.zoom + camera.y,
			];
		},
		[camera, viewport],
	);

	const emitCamera = useCallback(
		(next: DrawCamera) => {
			setCamera(next);
			onCameraChange?.(next);
		},
		[onCameraChange],
	);

	const commitDocument = useCallback(
		(next: DrawDocument, selectLayerId?: string) => {
			const withTool = applyDrawEditOp(next, { op: "setActiveTool", tool: "selectDirect" });
			onCommit?.(withTool, selectLayerId);
		},
		[onCommit],
	);

	const updateMarqueeOverlay = useCallback(
		(point: SelectionMarqueePoint, method: SelectionMarqueeMethod, startX: number, startY: number, points: SelectionMarqueePoint[]) => {
			const pathPoints = method === "lasso" ? points : [{ x: startX, y: startY }, point];
			const coverage = marqueeCoverageFromGesture({ method, startX, endX: point.x, path: pathPoints });
			if (method === "lasso") {
				setMarquee({ coverage, shape: "polygon", points });
				return;
			}
			const rect = screenRectFromPoints(pathPoints);
			if (!rect) return;
			setMarquee({ coverage, shape: "rect", rect });
		},
		[],
	);

	const commitMarqueeSelection = useCallback(
		(point: SelectionMarqueePoint, drag: Extract<DrawDragState, { kind: "marquee" }>) => {
			const vp = viewport();
			const pathPoints = drag.method === "lasso" ? [...drag.points, point] : [{ x: drag.startX, y: drag.startY }, point];
			const coverage = marqueeCoverageFromGesture({
				method: drag.method,
				startX: drag.startX,
				endX: point.x,
				path: pathPoints,
			});
			const rect = screenRectFromPoints(pathPoints);
			if (!rect) return;
			const hits = resolveDrawMarqueeLayerHits(doc, camera, vp, rect, coverage === "partial");
			onSelect?.(selectionMergeIds(drag.merge, selectedIds, hits));
		},
		[camera, doc, onSelect, selectedIds, viewport],
	);

	const shapePreviewFromWorld = useCallback((kind: "shapeRect" | "shapeEllipse" | "shapeLine", start: Vec2, end: Vec2): PathSegment[] => {
		if (kind === "shapeLine") {
			return [
				{ kind: "move", to: start },
				{ kind: "line", to: end },
			];
		}
		const x = Math.min(start[0], end[0]);
		const y = Math.min(start[1], end[1]);
		const width = Math.abs(end[0] - start[0]);
		const height = Math.abs(end[1] - start[1]);
		if (kind === "shapeRect") {
			return [
				{ kind: "move", to: [x, y] },
				{ kind: "line", to: [x + width, y] },
				{ kind: "line", to: [x + width, y + height] },
				{ kind: "line", to: [x, y + height] },
				{ kind: "close" },
			];
		}
		const cx = x + width / 2;
		const cy = y + height / 2;
		const rx = width / 2;
		const ry = height / 2;
		const k = 0.5522847498;
		return [
			{ kind: "move", to: [cx, cy - ry] },
			{ kind: "cubic", ctrl1: [cx + rx * k, cy - ry], ctrl2: [cx + rx, cy - ry * k], to: [cx + rx, cy] },
			{ kind: "cubic", ctrl1: [cx + rx, cy + ry * k], ctrl2: [cx + rx * k, cy + ry], to: [cx, cy + ry] },
			{ kind: "cubic", ctrl1: [cx - rx * k, cy + ry], ctrl2: [cx - rx, cy + ry * k], to: [cx - rx, cy] },
			{ kind: "cubic", ctrl1: [cx - rx, cy - ry * k], ctrl2: [cx - rx * k, cy - ry], to: [cx, cy - ry] },
			{ kind: "close" },
		];
	}, []);

	const commitShapeDrag = useCallback(
		(kind: "shapeRect" | "shapeEllipse" | "shapeLine", start: Vec2, end: Vec2) => {
			const x = Math.min(start[0], end[0]);
			const y = Math.min(start[1], end[1]);
			const width = Math.abs(end[0] - start[0]);
			const height = Math.abs(end[1] - start[1]);
			if (width < 1 && height < 1) return;
			let layer;
			if (kind === "shapeLine") {
				layer = createDrawShapeLayer("Line", { shapeKind: "line", line: { x1: start[0], y1: start[1], x2: end[0], y2: end[1] } });
			} else if (kind === "shapeEllipse") {
				layer = createDrawShapeLayer("Ellipse", {
					shapeKind: "ellipse",
					ellipse: { cx: x + width / 2, cy: y + height / 2, rx: width / 2, ry: height / 2 },
				});
			} else {
				layer = createDrawShapeLayer("Rectangle", { shapeKind: "rect", rect: { x, y, width, height } });
			}
			const next = applyDrawEditOp(doc, { op: "addShapeLayer", layer });
			commitDocument(next, layer.id);
		},
		[commitDocument, doc],
	);

	const commitPolyline = useCallback(
		(kind: "pen" | "shapePolygon", points: Vec2[]) => {
			if (points.length < 2) return;
			if (kind === "pen") {
				const segments: PathSegment[] = [{ kind: "move", to: points[0]! }];
				for (let i = 1; i < points.length; i += 1) segments.push({ kind: "line", to: points[i]! });
				const layer = createDrawPathLayer("Path", segments);
				const next = applyDrawEditOp(doc, { op: "addPathLayer", layer });
				commitDocument(next, layer.id);
				return;
			}
			const layer = createDrawShapeLayer("Polygon", { shapeKind: "polygon", polygon: { points } });
			const next = applyDrawEditOp(doc, { op: "addShapeLayer", layer });
			commitDocument(next, layer.id);
		},
		[commitDocument, doc],
	);

	const commitTraceAt = useCallback(
		(screen: SelectionMarqueePoint) => {
			const hit = resolveDrawLayerAtScreenPoint(doc, camera, viewport(), screen);
			const hitLayer = hit ? findDrawLayer(doc, hit) : null;
			let sourceKey: string | undefined;
			if (hitLayer?.kind === "image") sourceKey = hitLayer.imageKey;
			else sourceKey = Object.keys(doc.assets ?? {})[0];
			if (!sourceKey) return;
			const layer = createDrawTraceLayer("Trace", sourceKey, defaultDrawTraceParams());
			const next = applyDrawEditOp(doc, { op: "addTraceLayer", layer });
			commitDocument(next, layer.id);
		},
		[camera, commitDocument, doc, viewport],
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
			const point = screenPoint(event);
			if (event.button === 1 || activeTool === "transformMove") {
				dragRef.current = { kind: "pan", startX: event.clientX, startY: event.clientY };
				return;
			}
			if (selectionMethod && event.button === 0) {
				dragRef.current = {
					kind: "marquee",
					method: selectionMethod,
					startX: point.x,
					startY: point.y,
					points: [point],
					merge: marqueeModeFromModifiers(event),
					active: false,
				};
				setMarquee(null);
				return;
			}
			if (activeTool === "shapeRect" || activeTool === "shapeEllipse" || activeTool === "shapeLine") {
				dragRef.current = { kind: activeTool, startWorld: screenToWorld(point) };
				setPreviewSegments([]);
				return;
			}
			if (activeTool === "pen" || activeTool === "shapePolygon") {
				const world = screenToWorld(point);
				const existing = dragRef.current;
				if (existing && (existing.kind === "pen" || existing.kind === "shapePolygon") && existing.kind === activeTool) {
					const points = [...existing.points, world];
					dragRef.current = { kind: activeTool, points };
					const segments: PathSegment[] = [{ kind: "move", to: points[0]! }];
					for (let i = 1; i < points.length; i += 1) segments.push({ kind: "line", to: points[i]! });
					if (activeTool === "shapePolygon" && points.length > 2) segments.push({ kind: "close" });
					setPreviewSegments(segments);
					return;
				}
				dragRef.current = { kind: activeTool, points: [world] };
				setPreviewSegments([{ kind: "move", to: world }]);
				return;
			}
			if (activeTool === "trace" && event.button === 0) {
				commitTraceAt(point);
				return;
			}
			if (activeTool === "selectDirect" && event.button === 0) {
				canvasPick.onCanvasPointerDown(clientFromScreen(point));
				return;
			}
		},
		[activeTool, canvasPick, clientFromScreen, commitTraceAt, screenPoint, screenToWorld, selectionMethod],
	);

	const onPointerMove = useCallback(
		(event: React.PointerEvent) => {
			const point = screenPoint(event);
			const drag = dragRef.current;
			if (!drag) {
				if (!canvasPick.pickMenuOpen) canvasPick.onCanvasPointerMove(clientFromScreen(point));
				return;
			}
			if (drag.kind === "pan") {
				const dx = (event.clientX - drag.startX) / camera.zoom;
				const dy = (event.clientY - drag.startY) / camera.zoom;
				emitCamera({ ...camera, x: camera.x - dx, y: camera.y - dy });
				dragRef.current = { kind: "pan", startX: event.clientX, startY: event.clientY };
				return;
			}
			if (drag.kind === "marquee") {
				const distance = Math.hypot(point.x - drag.startX, point.y - drag.startY);
				const active = drag.active || distance >= DRAW_MARQUEE_THRESHOLD_PX;
				const points = drag.method === "lasso" && active ? [...drag.points, point] : drag.points;
				const nextDrag = { ...drag, active, points, merge: marqueeModeFromModifiers(event) };
				dragRef.current = nextDrag;
				if (active) updateMarqueeOverlay(point, drag.method, drag.startX, drag.startY, points);
				return;
			}
			if (drag.kind === "shapeRect" || drag.kind === "shapeEllipse" || drag.kind === "shapeLine") {
				const end = screenToWorld(point);
				setPreviewSegments(shapePreviewFromWorld(drag.kind, drag.startWorld, end));
				return;
			}
			if (drag.kind === "pen" || drag.kind === "shapePolygon") {
				const segments: PathSegment[] = [{ kind: "move", to: drag.points[0]! }];
				for (let i = 1; i < drag.points.length; i += 1) segments.push({ kind: "line", to: drag.points[i]! });
				const world = screenToWorld(point);
				segments.push({ kind: "line", to: world });
				if (drag.kind === "shapePolygon" && drag.points.length > 1) segments.push({ kind: "close" });
				setPreviewSegments(segments);
			}
		},
		[camera, canvasPick, clientFromScreen, emitCamera, screenPoint, screenToWorld, shapePreviewFromWorld, updateMarqueeOverlay],
	);

	const onPointerUp = useCallback(
		(event: React.PointerEvent) => {
			const point = screenPoint(event);
			const drag = dragRef.current;
			if (activeTool === "selectDirect" && !drag) {
				canvasPick.onCanvasPointerUp(clientFromScreen(point), {
					shift: event.shiftKey,
					ctrl: event.ctrlKey,
					meta: event.metaKey,
					alt: event.altKey,
				});
				return;
			}
			if (!drag) return;
			if (drag.kind === "pen" || drag.kind === "shapePolygon") return;
			dragRef.current = null;
			if (drag.kind === "marquee") {
				const distance = Math.hypot(point.x - drag.startX, point.y - drag.startY);
				const merge = marqueeModeFromModifiers(event);
				if (drag.active && distance >= DRAW_MARQUEE_THRESHOLD_PX) {
					commitMarqueeSelection(point, drag);
				} else if (activeTool === "selectDirect" || selectionMethod) {
					canvasPick.onCanvasPointerUp(clientFromScreen(point), {
						shift: event.shiftKey,
						ctrl: event.ctrlKey,
						meta: event.metaKey,
						alt: event.altKey,
					});
				}
				setMarquee(null);
				return;
			}
			if (drag.kind === "shapeRect" || drag.kind === "shapeEllipse" || drag.kind === "shapeLine") {
				commitShapeDrag(drag.kind, drag.startWorld, screenToWorld(point));
				setPreviewSegments([]);
				return;
			}
		},
		[activeTool, camera, canvasPick, clientFromScreen, commitMarqueeSelection, commitShapeDrag, screenPoint, screenToWorld, selectionMethod],
	);

	const onDoubleClick = useCallback(
		(event: React.MouseEvent) => {
			const drag = dragRef.current;
			if (!drag || (drag.kind !== "pen" && drag.kind !== "shapePolygon")) return;
			event.preventDefault();
			commitPolyline(drag.kind, drag.points);
			dragRef.current = null;
			setPreviewSegments([]);
		},
		[commitPolyline],
	);

	useEffect(() => {
		const onKeyDown = (event: KeyboardEvent) => {
			const drag = dragRef.current;
			if (event.key === "Escape") {
				dragRef.current = null;
				setPreviewSegments([]);
				setMarquee(null);
				return;
			}
			if (event.key === "Enter" && drag && (drag.kind === "pen" || drag.kind === "shapePolygon")) {
				commitPolyline(drag.kind, drag.points);
				dragRef.current = null;
				setPreviewSegments([]);
			}
		};
		window.addEventListener("keydown", onKeyDown);
		return () => window.removeEventListener("keydown", onKeyDown);
	}, [commitPolyline]);

	const transform = `translate(${camera.x * -camera.zoom + (containerRef.current?.clientWidth ?? 0) / 2}, ${camera.y * -camera.zoom + (containerRef.current?.clientHeight ?? 0) / 2}) scale(${camera.zoom})`;

	return (
		<div
			ref={containerRef}
			className={cn("relative h-full w-full overflow-hidden bg-neutral-950 touch-none", className)}
			onWheel={onWheel}
			onPointerDown={onPointerDown}
			onPointerMove={onPointerMove}
			onPointerUp={onPointerUp}
			onDoubleClick={onDoubleClick}
			onPointerLeave={() => {
				canvasPick.onCanvasPointerLeave();
				if (!canvasPick.pickMenuOpen) onHover?.({ id: null, kind: null });
			}}
		>
			<svg className="h-full w-full" viewBox={`0 0 ${containerRef.current?.clientWidth ?? 1024} ${containerRef.current?.clientHeight ?? 768}`}>
				<g transform={transform}>
					{resolved.map(({ node, segments }) => {
						const matrix = `matrix(${node.transform.join(" ")})`;
						return (
							<g key={node.id} transform={matrix} style={{ mixBlendMode: drawBlendModeCss(node.blendMode) }} opacity={node.opacity}>
								{node.text ? (
									<DrawTextShape content={node.text.content} size={node.text.size} fill={node.fill} opacity={1} />
								) : node.image ? (
									<DrawImageShape src={node.image.src} width={node.image.width} height={node.image.height} opacity={1} />
								) : (
									<DrawPathShape segments={segments} fill={node.fill} stroke={node.stroke} opacity={1} kernelKind={node.kernelKind} />
								)}
							</g>
						);
					})}
					{resolved.map(({ node, segments }) => {
						const selected = selectedLeafIds.has(node.id);
						const hovered = hoveredLeafIds.has(node.id);
						if (!selected && !hovered) return null;
						const matrix = `matrix(${node.transform.join(" ")})`;
						return (
							<g key={`hl-${node.id}`} transform={matrix} opacity={node.opacity} pointerEvents="none">
								<DrawLayerInteractionHighlight
									node={node}
									segments={segments}
									selected={selected}
									hovered={hovered}
									stroke={node.stroke}
									activeColor={activeColor}
									hoverColor={hoverColor}
									haloColor={haloColor}
								/>
							</g>
						);
					})}
					{previewSegments.length > 0 ? <DrawPreviewPath segments={previewSegments} stroke={hoverColor} /> : null}
				</g>
			</svg>
			{marquee ? <SelectionMarquee {...marquee} /> : null}
			<CanvasPickMenu
				request={canvasPick.pickMenu}
				hoveredKey={canvasPick.menuHoveredKey}
				onHoverKey={canvasPick.onMenuHoverKey}
				onPick={canvasPick.onMenuPick}
				onDismiss={canvasPick.dismissPickMenu}
			/>
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

		it("highlights group descendants via leaf id expansion", () => {
			const child = createDrawPathLayer("Child");
			const group = { ...createDrawGroupLayer("Group"), children: [child] };
			const doc = { ...defaultDrawDocument("group"), layers: [group] };
			const leaves = drawLayerDescendantLeafIds(doc, group.id);
			expect(leaves).toEqual([child.id]);
		});
	});
}
// #endregion 🧪Tests
