// #region 🧲Header
/// <reference types="vite/client" />
/// <reference types="vitest/importMeta" />
/** @emoji ✏️ `@semio-tech/draw-core` — non-destructive vector document model, edit ops, hover/selection mapping. */
// #endregion 🧲Header

import { DRAWLAYERS_LAYER_IDS, type DrawLayersLayerKindId } from "@semio-tech/graph-manifest";
import {
	createDocumentVcsEnvelope,
	type DocumentVcsEnvelope,
	materializeDocumentProjection,
} from "@semio-tech/vcs-core/internal";

// #region 📐Types
export type Vec2 = readonly [number, number];

export type PathSegment =
	| { readonly kind: "move"; readonly to: Vec2 }
	| { readonly kind: "line"; readonly to: Vec2 }
	| { readonly kind: "quad"; readonly ctrl: Vec2; readonly to: Vec2 }
	| { readonly kind: "cubic"; readonly ctrl1: Vec2; readonly ctrl2: Vec2; readonly to: Vec2 }
	| {
			readonly kind: "arc";
			readonly rx: number;
			readonly ry: number;
			readonly rotation: number;
			readonly largeArc: boolean;
			readonly sweep: boolean;
			readonly to: Vec2;
	  }
	| { readonly kind: "close" };

export type FillStyle =
	| { readonly kind: "solid"; readonly color: readonly [number, number, number, number] }
	| {
			readonly kind: "linearGradient";
			readonly x1: number;
			readonly y1: number;
			readonly x2: number;
			readonly y2: number;
			readonly stops: readonly GradientStop[];
	  }
	| { readonly kind: "radialGradient"; readonly cx: number; readonly cy: number; readonly r: number; readonly stops: readonly GradientStop[] };

export interface GradientStop {
	readonly offset: number;
	readonly color: readonly [number, number, number, number];
}

export interface StrokeStyle {
	readonly color: readonly [number, number, number, number];
	readonly width: number;
	readonly cap: "butt" | "round" | "square";
	readonly join: "miter" | "round" | "bevel";
	readonly dash?: readonly number[];
}

export const DRAW_BLEND_MODES = [
	"normal",
	"multiply",
	"screen",
	"overlay",
	"darken",
	"lighten",
	"colorDodge",
	"colorBurn",
	"hardLight",
	"softLight",
	"difference",
	"exclusion",
	"hue",
	"saturation",
	"color",
	"luminosity",
] as const;

export type DrawBlendMode = (typeof DRAW_BLEND_MODES)[number];

export const DRAW_BOOLEAN_OPS = ["union", "difference", "intersection", "xor"] as const;
export type DrawBooleanOp = (typeof DRAW_BOOLEAN_OPS)[number];

export const DRAW_SHAPE_KINDS = ["rect", "ellipse", "circle", "line", "polygon"] as const;
export type DrawShapeKind = (typeof DRAW_SHAPE_KINDS)[number];

export type DrawLayerKindId = DrawLayersLayerKindId;
export { DRAWLAYERS_LAYER_IDS as DRAW_LAYER_KIND_IDS };

export const DRAW_TOOL_IDS = [
	"selectMarquee",
	"selectLasso",
	"selectDirect",
	"pen",
	"shapeRect",
	"shapeEllipse",
	"shapeLine",
	"shapePolygon",
	"booleanCombine",
	"trace",
	"transformMove",
] as const;

export type DrawToolId = (typeof DRAW_TOOL_IDS)[number];

export interface DrawCamera {
	readonly x: number;
	readonly y: number;
	readonly zoom: number;
}

export interface DrawTransform {
	readonly x: number;
	readonly y: number;
	readonly scaleX: number;
	readonly scaleY: number;
	readonly rotation: number;
}

export interface DrawAttributes {
	readonly fill?: FillStyle;
	readonly stroke?: StrokeStyle;
}

export interface DrawTraceParams {
	readonly threshold: number;
	readonly simplifyEpsilon: number;
}

export interface DrawImageAsset {
	readonly mime: string;
	readonly data: string;
	readonly width?: number;
	readonly height?: number;
}

export interface DrawLayerBase {
	readonly id: string;
	readonly name: string;
	readonly visible: boolean;
	readonly locked: boolean;
	readonly opacity: number;
	readonly blendMode: DrawBlendMode;
	readonly transform: DrawTransform;
	readonly attributes: DrawAttributes;
}

export interface DrawShapeLayer extends DrawLayerBase {
	readonly kind: "shape";
	readonly shapeKind: DrawShapeKind;
	readonly rect?: { readonly x: number; readonly y: number; readonly width: number; readonly height: number };
	readonly ellipse?: { readonly cx: number; readonly cy: number; readonly rx: number; readonly ry: number };
	readonly circle?: { readonly cx: number; readonly cy: number; readonly r: number };
	readonly line?: { readonly x1: number; readonly y1: number; readonly x2: number; readonly y2: number };
	readonly polygon?: { readonly points: readonly Vec2[] };
}

export interface DrawPathLayer extends DrawLayerBase {
	readonly kind: "path";
	readonly segments: readonly PathSegment[];
}

export interface DrawTextLayer extends DrawLayerBase {
	readonly kind: "text";
	readonly x: number;
	readonly y: number;
	readonly content: string;
	readonly size: number;
}

export interface DrawImageLayer extends DrawLayerBase {
	readonly kind: "image";
	readonly imageKey: string;
	readonly width: number;
	readonly height: number;
}

export interface DrawGroupLayer extends DrawLayerBase {
	readonly kind: "group";
	readonly children: readonly DrawLayerNode[];
}

export interface DrawBooleanLayer extends DrawLayerBase {
	readonly kind: "boolean";
	readonly op: DrawBooleanOp;
	readonly children: readonly string[];
}

export interface DrawTraceLayer extends DrawLayerBase {
	readonly kind: "trace";
	readonly sourceKey: string;
	readonly params: DrawTraceParams;
}

export type DrawLayerNode =
	| DrawShapeLayer
	| DrawPathLayer
	| DrawTextLayer
	| DrawImageLayer
	| DrawGroupLayer
	| DrawBooleanLayer
	| DrawTraceLayer;

export interface DrawArtboard {
	readonly width: number;
	readonly height: number;
}

export interface DrawDocument {
	readonly schema: "draw.document";
	readonly id: string;
	readonly title?: string;
	readonly camera: DrawCamera;
	readonly layers: readonly DrawLayerNode[];
	readonly assets?: Readonly<Record<string, DrawImageAsset>>;
	readonly artboard?: DrawArtboard;
	readonly activeTool?: DrawToolId;
}

export type DrawKindHoverDomain = "layer" | "group" | "boolean" | "trace" | "shape";

export interface DrawKindHover {
	readonly domain: DrawKindHoverDomain;
	readonly kindId: string;
}

export interface DrawHoverPayload {
	readonly id: string | null;
	readonly kind: DrawKindHover | null;
}

/** @emoji 🪪 Encodes draw hover/selection focus as `draw:${kind}:${id}`. */
export function encodeDrawPointerFocusKey(kind: string, id: string): string {
	return `draw:${kind}:${id}`;
}

/** @emoji 🪪 Decodes a draw pointer-focus key. */
export function decodeDrawPointerFocusKey(key: string): { readonly kind: string; readonly id: string } | null {
	if (!key.startsWith("draw:")) return null;
	const rest = key.slice("draw:".length);
	const colon = rest.indexOf(":");
	if (colon < 0) return null;
	return { kind: rest.slice(0, colon), id: rest.slice(colon + 1) };
}

/** @emoji 🖱️ Builds {@link DrawHoverPayload} from a pointer-focus hover key. */
export function drawHoverPayloadFromPointerFocusKey(key: string | null): DrawHoverPayload {
	if (!key) return { id: null, kind: null };
	const decoded = decodeDrawPointerFocusKey(key);
	if (!decoded) return { id: key, kind: null };
	return {
		id: decoded.id,
		kind: { domain: decoded.kind as DrawKindHoverDomain, kindId: decoded.id },
	};
}

export type DrawEditOp =
	| { readonly op: "setLayerVisible"; readonly layerId: string; readonly visible: boolean }
	| { readonly op: "setLayerLocked"; readonly layerId: string; readonly locked: boolean }
	| { readonly op: "setLayerOpacity"; readonly layerId: string; readonly opacity: number }
	| { readonly op: "setLayerBlendMode"; readonly layerId: string; readonly blendMode: DrawBlendMode }
	| { readonly op: "setLayerName"; readonly layerId: string; readonly name: string }
	| { readonly op: "setLayerTransform"; readonly layerId: string; readonly transform: DrawTransform }
	| { readonly op: "setFill"; readonly layerId: string; readonly fill: FillStyle | undefined }
	| { readonly op: "setStroke"; readonly layerId: string; readonly stroke: StrokeStyle | undefined }
	| { readonly op: "setBooleanOp"; readonly layerId: string; readonly booleanOp: DrawBooleanOp }
	| { readonly op: "setTraceParams"; readonly layerId: string; readonly params: DrawTraceParams }
	| { readonly op: "addShapeLayer"; readonly parentId?: string; readonly index?: number; readonly layer: DrawShapeLayer }
	| { readonly op: "addPathLayer"; readonly parentId?: string; readonly index?: number; readonly layer: DrawPathLayer }
	| { readonly op: "addTextLayer"; readonly parentId?: string; readonly index?: number; readonly layer: DrawTextLayer }
	| { readonly op: "addImageLayer"; readonly parentId?: string; readonly index?: number; readonly layer: DrawImageLayer }
	| { readonly op: "addGroupLayer"; readonly parentId?: string; readonly index?: number; readonly layer: DrawGroupLayer }
	| { readonly op: "addBooleanLayer"; readonly parentId?: string; readonly index?: number; readonly layer: DrawBooleanLayer }
	| { readonly op: "addTraceLayer"; readonly parentId?: string; readonly index?: number; readonly layer: DrawTraceLayer }
	| { readonly op: "duplicateLayer"; readonly layerId: string }
	| { readonly op: "removeLayer"; readonly layerId: string }
	| { readonly op: "reorderLayer"; readonly layerId: string; readonly parentId?: string; readonly index: number }
	| { readonly op: "setActiveTool"; readonly tool: DrawToolId }
	| { readonly op: "setCamera"; readonly camera: DrawCamera }
	| { readonly op: "setDocument"; readonly document: DrawDocument };
// #endregion 📐Types

// #region 🔧Helpers
let drawIdCounter = 0;

export function createDrawId(prefix = "layer"): string {
	drawIdCounter += 1;
	return `${prefix}-${drawIdCounter}`;
}

export function defaultDrawTransform(): DrawTransform {
	return { x: 0, y: 0, scaleX: 1, scaleY: 1, rotation: 0 };
}

export function defaultDrawTraceParams(): DrawTraceParams {
	return { threshold: 0.5, simplifyEpsilon: 1.5 };
}

export interface DrawLayerLocation {
	readonly parentId?: string;
	readonly index: number;
}

export function createDrawPathLayer(name = "Path", segments: readonly PathSegment[] = []): DrawPathLayer {
	return {
		kind: "path",
		id: createDrawId("path"),
		name,
		visible: true,
		locked: false,
		opacity: 1,
		blendMode: "normal",
		transform: defaultDrawTransform(),
		attributes: {},
		segments,
	};
}

export function createDrawGroupLayer(name = "Group"): DrawGroupLayer {
	return {
		kind: "group",
		id: createDrawId("group"),
		name,
		visible: true,
		locked: false,
		opacity: 1,
		blendMode: "normal",
		transform: defaultDrawTransform(),
		attributes: {},
		children: [],
	};
}

export function createDrawBooleanLayer(name = "Boolean", op: DrawBooleanOp = "union", children: readonly string[] = []): DrawBooleanLayer {
	return {
		kind: "boolean",
		id: createDrawId("boolean"),
		name,
		visible: true,
		locked: false,
		opacity: 1,
		blendMode: "normal",
		transform: defaultDrawTransform(),
		attributes: {},
		op,
		children: [...children],
	};
}

export function createDrawTraceLayer(name = "Trace", sourceKey: string, params = defaultDrawTraceParams()): DrawTraceLayer {
	return {
		kind: "trace",
		id: createDrawId("trace"),
		name,
		visible: true,
		locked: false,
		opacity: 1,
		blendMode: "normal",
		transform: defaultDrawTransform(),
		attributes: {},
		sourceKey,
		params,
	};
}

export type DrawShapeGeometry =
	| { readonly shapeKind: "rect"; readonly rect: { readonly x: number; readonly y: number; readonly width: number; readonly height: number } }
	| { readonly shapeKind: "ellipse"; readonly ellipse: { readonly cx: number; readonly cy: number; readonly rx: number; readonly ry: number } }
	| { readonly shapeKind: "circle"; readonly circle: { readonly cx: number; readonly cy: number; readonly r: number } }
	| { readonly shapeKind: "line"; readonly line: { readonly x1: number; readonly y1: number; readonly x2: number; readonly y2: number } }
	| { readonly shapeKind: "polygon"; readonly polygon: { readonly points: readonly Vec2[] } };

export function createDrawShapeLayer(name = "Shape", geometry: DrawShapeGeometry): DrawShapeLayer {
	return {
		kind: "shape",
		id: createDrawId("shape"),
		name,
		visible: true,
		locked: false,
		opacity: 1,
		blendMode: "normal",
		transform: defaultDrawTransform(),
		attributes: {},
		...geometry,
	};
}

export function createDrawTextLayer(name = "Text", content = "Text", size = 24, x = 0, y = 0): DrawTextLayer {
	return {
		kind: "text",
		id: createDrawId("text"),
		name,
		visible: true,
		locked: false,
		opacity: 1,
		blendMode: "normal",
		transform: defaultDrawTransform(),
		attributes: { fill: { kind: "solid", color: [0, 0, 0, 1] } },
		x,
		y,
		content,
		size,
	};
}

export function createDrawImageLayer(name = "Image", imageKey: string, width = 256, height = 256): DrawImageLayer {
	return {
		kind: "image",
		id: createDrawId("image"),
		name,
		visible: true,
		locked: false,
		opacity: 1,
		blendMode: "normal",
		transform: defaultDrawTransform(),
		attributes: {},
		imageKey,
		width,
		height,
	};
}

export function defaultDrawDocument(id = "empty", title?: string): DrawDocument {
	return {
		schema: "draw.document",
		id,
		title,
		camera: { x: 0, y: 0, zoom: 1 },
		layers: [createDrawPathLayer("Layer 1")],
		activeTool: "selectDirect",
	};
}

export function cloneDrawLayerNode(node: DrawLayerNode, nameSuffix = " copy"): DrawLayerNode {
	const id = createDrawId(node.kind);
	if (node.kind === "group") {
		return {
			...node,
			id,
			name: `${node.name}${nameSuffix}`,
			children: node.children.map((child) => cloneDrawLayerNode(child, "")),
		};
	}
	if (node.kind === "boolean") {
		return { ...node, id, name: `${node.name}${nameSuffix}`, children: [...node.children] };
	}
	return { ...node, id, name: `${node.name}${nameSuffix}` };
}

export function parseDrawDocument(raw: unknown): DrawDocument {
	if (!raw || typeof raw !== "object" || Array.isArray(raw)) throw new Error("draw document must be an object");
	const record = raw as DrawDocument;
	if (record.schema !== "draw.document") throw new Error(`unsupported draw schema: ${String((raw as { schema?: string }).schema)}`);
	if (!Array.isArray(record.layers)) throw new Error("draw document layers must be an array");
	validateDrawLayerNodes(record.layers);
	return record;
}

function validateDrawLayerNodes(layers: readonly DrawLayerNode[]): void {
	for (const layer of layers) {
		if (!(DRAWLAYERS_LAYER_IDS as readonly string[]).includes(layer.kind)) {
			throw new Error(`unknown draw layer kind: ${String(layer.kind)}`);
		}
		if (layer.kind === "group") {
			validateDrawLayerNodes(layer.children);
		}
	}
}

export function drawDocumentToJson(doc: DrawDocument): string {
	return JSON.stringify(doc, null, 2);
}

export function drawDocumentFromJson(json: string): DrawDocument {
	return parseDrawDocument(JSON.parse(json));
}

export function drawKindHoversEqual(a: DrawKindHover | null, b: DrawKindHover | null): boolean {
	if (a === b) return true;
	if (!a || !b) return false;
	return a.domain === b.domain && a.kindId === b.kindId;
}

export function hexToRgba(hex: string, alpha = 1): [number, number, number, number] {
	const normalized = hex.replace("#", "");
	const value =
		normalized.length === 3
			? normalized
					.split("")
					.map((c) => c + c)
					.join("")
			: normalized;
	const r = Number.parseInt(value.slice(0, 2), 16) / 255;
	const g = Number.parseInt(value.slice(2, 4), 16) / 255;
	const b = Number.parseInt(value.slice(4, 6), 16) / 255;
	return [r, g, b, alpha];
}

export function rgbaToHex(color: readonly [number, number, number, number]): string {
	const r = Math.round(Math.max(0, Math.min(1, color[0])) * 255)
		.toString(16)
		.padStart(2, "0");
	const g = Math.round(Math.max(0, Math.min(1, color[1])) * 255)
		.toString(16)
		.padStart(2, "0");
	const b = Math.round(Math.max(0, Math.min(1, color[2])) * 255)
		.toString(16)
		.padStart(2, "0");
	return `#${r}${g}${b}`;
}

function ellipsePathSegments(cx: number, cy: number, rx: number, ry: number): PathSegment[] {
	const k = 0.5522847498;
	const crx = rx * k;
	const cry = ry * k;
	return [
		{ kind: "move", to: [cx, cy - ry] },
		{ kind: "cubic", ctrl1: [cx + crx, cy - ry], ctrl2: [cx + rx, cy - cry], to: [cx + rx, cy] },
		{ kind: "cubic", ctrl1: [cx + rx, cy + cry], ctrl2: [cx + crx, cy + ry], to: [cx, cy + ry] },
		{ kind: "cubic", ctrl1: [cx - crx, cy + ry], ctrl2: [cx - rx, cy + cry], to: [cx - rx, cy] },
		{ kind: "cubic", ctrl1: [cx - rx, cy - cry], ctrl2: [cx - crx, cy - ry], to: [cx, cy - ry] },
		{ kind: "close" },
	];
}

export function drawImageAssetDataUrl(asset: DrawImageAsset): string {
	return asset.data.startsWith("data:") ? asset.data : `data:${asset.mime};base64,${asset.data}`;
}

export function layerToPathSegments(layer: DrawLayerNode): PathSegment[] {
	if (layer.kind === "path") return [...layer.segments];
	if (layer.kind === "shape") {
		if (layer.shapeKind === "rect" && layer.rect) {
			const { x, y, width, height } = layer.rect;
			return [
				{ kind: "move", to: [x, y] },
				{ kind: "line", to: [x + width, y] },
				{ kind: "line", to: [x + width, y + height] },
				{ kind: "line", to: [x, y + height] },
				{ kind: "close" },
			];
		}
		if (layer.shapeKind === "line" && layer.line) {
			return [
				{ kind: "move", to: [layer.line.x1, layer.line.y1] },
				{ kind: "line", to: [layer.line.x2, layer.line.y2] },
			];
		}
		if (layer.shapeKind === "polygon" && layer.polygon?.points.length) {
			const segments: PathSegment[] = [{ kind: "move", to: layer.polygon.points[0]! }];
			for (let i = 1; i < layer.polygon.points.length; i += 1) {
				segments.push({ kind: "line", to: layer.polygon.points[i]! });
			}
			segments.push({ kind: "close" });
			return segments;
		}
		if (layer.shapeKind === "ellipse" && layer.ellipse) {
			return ellipsePathSegments(layer.ellipse.cx, layer.ellipse.cy, layer.ellipse.rx, layer.ellipse.ry);
		}
		if (layer.shapeKind === "circle" && layer.circle) {
			return ellipsePathSegments(layer.circle.cx, layer.circle.cy, layer.circle.r, layer.circle.r);
		}
	}
	return [];
}

export interface DrawSceneNode {
	readonly id: string;
	readonly transform: readonly [number, number, number, number, number, number];
	readonly segments: readonly PathSegment[];
	readonly fill?: FillStyle;
	readonly stroke?: StrokeStyle;
	readonly opacity: number;
	readonly blendMode: DrawBlendMode;
	readonly visible: boolean;
	readonly needsKernel: boolean;
	readonly kernelKind?: "boolean" | "trace";
	readonly kernelPayload?: unknown;
	readonly text?: { readonly content: string; readonly size: number };
	readonly image?: { readonly src: string; readonly width: number; readonly height: number };
}

export function drawTransformToMatrix(transform: DrawTransform): [number, number, number, number, number, number] {
	const cos = Math.cos(transform.rotation);
	const sin = Math.sin(transform.rotation);
	const a = transform.scaleX * cos;
	const b = transform.scaleX * sin;
	const c = -transform.scaleY * sin;
	const d = transform.scaleY * cos;
	return [a, b, c, d, transform.x, transform.y];
}

/** 🧭 Best-effort inverse of {@link drawTransformToMatrix}; shear is not represented in {@link DrawTransform}. */
export function drawMatrixToTransform(matrix: readonly [number, number, number, number, number, number]): DrawTransform {
	const [a, b, c, d, e, f] = matrix;
	const scaleX = Math.hypot(a, b);
	const rotation = Math.atan2(b, a);
	const det = a * d - b * c;
	const scaleY = scaleX !== 0 ? det / scaleX : 0;
	return { x: e, y: f, scaleX, scaleY, rotation };
}

function drawMapPointByMatrix(matrix: readonly [number, number, number, number, number, number], point: Vec2): Vec2 {
	const [a, b, c, d, e, f] = matrix;
	return [a * point[0] + c * point[1] + e, b * point[0] + d * point[1] + f];
}

export function transformPathSegments(segments: readonly PathSegment[], transform: DrawTransform): PathSegment[] {
	const matrix = drawTransformToMatrix(transform);
	return segments.map((segment) => {
		if (segment.kind === "close") return segment;
		if (segment.kind === "move" || segment.kind === "line") return { ...segment, to: drawMapPointByMatrix(matrix, segment.to) };
		if (segment.kind === "quad") return { ...segment, ctrl: drawMapPointByMatrix(matrix, segment.ctrl), to: drawMapPointByMatrix(matrix, segment.to) };
		if (segment.kind === "cubic")
			return {
				...segment,
				ctrl1: drawMapPointByMatrix(matrix, segment.ctrl1),
				ctrl2: drawMapPointByMatrix(matrix, segment.ctrl2),
				to: drawMapPointByMatrix(matrix, segment.to),
			};
		return { ...segment, to: drawMapPointByMatrix(matrix, segment.to) };
	});
}

export function scalePathSegments(segments: readonly PathSegment[], scaleX: number, scaleY: number): PathSegment[] {
	if (scaleX === 1 && scaleY === 1) return [...segments];
	return transformPathSegments(segments, { x: 0, y: 0, scaleX, scaleY, rotation: 0 });
}

export function splitPathSegmentsByContour(segments: readonly PathSegment[]): PathSegment[][] {
	const contours: PathSegment[][] = [];
	let current: PathSegment[] = [];
	for (const segment of segments) {
		if (segment.kind === "move" && current.length > 0) {
			contours.push(current);
			current = [];
		}
		current.push(segment);
	}
	if (current.length > 0) contours.push(current);
	return contours.length > 0 ? contours : [[]];
}

export function filterPathSegmentsByContourArea(segments: readonly PathSegment[], minArea: number): PathSegment[] {
	if (minArea <= 0) return [...segments];
	const kept: PathSegment[] = [];
	for (const contour of splitPathSegmentsByContour(segments)) {
		const bounds = pathBounds(contour);
		if (!bounds || bounds.width * bounds.height < minArea) continue;
		kept.push(...contour);
	}
	return kept;
}

export function resolveDrawDocumentArtboard(doc: DrawDocument): DrawArtboard | null {
	if (doc.artboard && doc.artboard.width > 0 && doc.artboard.height > 0) return doc.artboard;
	let maxX = 0;
	let maxY = 0;
	for (const layer of flattenDrawLayers(doc.layers)) {
		if (layer.kind === "trace" || layer.kind === "boolean" || layer.kind === "group") continue;
		const bounds = drawLayerWorldBounds(layer);
		if (!bounds) continue;
		maxX = Math.max(maxX, bounds.x + bounds.width);
		maxY = Math.max(maxY, bounds.y + bounds.height);
	}
	if (maxX <= 0 || maxY <= 0) return null;
	return { width: maxX, height: maxY };
}

export function flattenDrawDocumentToSceneNodes(doc: DrawDocument): DrawSceneNode[] {
	const out: DrawSceneNode[] = [];
	const walk = (layers: readonly DrawLayerNode[]) => {
		for (const layer of layers) {
			if (!layer.visible) continue;
			if (layer.kind === "group") {
				walk(layer.children);
				continue;
			}
			if (layer.kind === "boolean") {
				out.push({
					id: layer.id,
					transform: drawTransformToMatrix(layer.transform),
					segments: [],
					fill: layer.attributes.fill,
					stroke: layer.attributes.stroke,
					opacity: layer.opacity,
					blendMode: layer.blendMode,
					visible: layer.visible,
					needsKernel: true,
					kernelKind: "boolean",
					kernelPayload: { op: layer.op, children: layer.children },
				});
				continue;
			}
			if (layer.kind === "trace") {
				out.push({
					id: layer.id,
					transform: drawTransformToMatrix(layer.transform),
					segments: [],
					fill: layer.attributes.fill,
					stroke: layer.attributes.stroke,
					opacity: layer.opacity,
					blendMode: layer.blendMode,
					visible: layer.visible,
					needsKernel: true,
					kernelKind: "trace",
					kernelPayload: { sourceKey: layer.sourceKey, params: layer.params },
				});
				continue;
			}
			if (layer.kind === "text") {
				out.push({
					id: layer.id,
					transform: drawTransformToMatrix(layer.transform),
					segments: [],
					fill: layer.attributes.fill,
					stroke: layer.attributes.stroke,
					opacity: layer.opacity,
					blendMode: layer.blendMode,
					visible: layer.visible,
					needsKernel: false,
					text: { content: layer.content, size: layer.size },
				});
				continue;
			}
			if (layer.kind === "image") {
				const asset = doc.assets?.[layer.imageKey];
				out.push({
					id: layer.id,
					transform: drawTransformToMatrix(layer.transform),
					segments: [],
					fill: layer.attributes.fill,
					stroke: layer.attributes.stroke,
					opacity: layer.opacity,
					blendMode: layer.blendMode,
					visible: layer.visible,
					needsKernel: false,
					image: asset
						? { src: drawImageAssetDataUrl(asset), width: layer.width, height: layer.height }
						: { src: "", width: layer.width, height: layer.height },
				});
				continue;
			}
			const segments = layerToPathSegments(layer);
			if (segments.length === 0) continue;
			out.push({
				id: layer.id,
				transform: drawTransformToMatrix(layer.transform),
				segments,
				fill: layer.attributes.fill,
				stroke: layer.attributes.stroke,
				opacity: layer.opacity,
				blendMode: layer.blendMode,
				visible: layer.visible,
				needsKernel: false,
			});
		}
	};
	walk(doc.layers);
	return out;
}
// #endregion 🔧Helpers

// #region 🌳TreeIds
export const DRAW_PLAY_TREE_PREFIX = "draw-play-layers";

export function findDrawLayerLocation(doc: DrawDocument, layerId: string): DrawLayerLocation | null {
	const search = (layers: readonly DrawLayerNode[], parentId?: string): DrawLayerLocation | null => {
		for (let index = 0; index < layers.length; index += 1) {
			const layer = layers[index]!;
			if (layer.id === layerId) return { parentId, index };
			if (layer.kind === "group") {
				const nested = search(layer.children, layer.id);
				if (nested) return nested;
			}
		}
		return null;
	};
	return search(doc.layers);
}

export function drawPlayBooleanChildRowId(booleanId: string, childId: string): string {
	return `${DRAW_PLAY_TREE_PREFIX}.ref.${booleanId}::${childId}`;
}

export function drawPlayLayerIdFromBooleanChildRowId(rowId: string): string | null {
	const refMatch = rowId.match(/^draw-play-layers\.ref\.[^:]+::(.+)$/);
	return refMatch?.[1] ?? null;
}

export function drawPlayLayerIdFromTreeRowId(rowId: string): string | null {
	const refId = drawPlayLayerIdFromBooleanChildRowId(rowId);
	if (refId) return refId;
	const layerMatch = rowId.match(/^draw-play-layers\.(layer|group|boolean|trace|path|shape|text|image)\.(.+)$/);
	return layerMatch?.[2] ?? null;
}

export function resolveDrawPlayReorderTarget(
	doc: DrawDocument,
	targetRowId: string,
	dropPosition: "before" | "after" | "inside",
): DrawLayerLocation | null {
	const layerId = drawPlayLayerIdFromTreeRowId(targetRowId);
	if (!layerId) {
		if (dropPosition === "inside") return { index: doc.layers.length };
		return null;
	}
	const layer = findDrawLayer(doc, layerId);
	if (!layer) return null;
	if (dropPosition === "inside" && layer.kind === "group") {
		return { parentId: layer.id, index: layer.children.length };
	}
	const location = findDrawLayerLocation(doc, layerId);
	if (!location) return null;
	if (dropPosition === "before") return location;
	return { parentId: location.parentId, index: location.index + 1 };
}

export function drawPlayLayersTreeRowId(layer: DrawLayerNode): string {
	const segment =
		layer.kind === "group"
			? "group"
			: layer.kind === "boolean"
				? "boolean"
				: layer.kind === "trace"
					? "trace"
					: layer.kind === "path"
						? "path"
						: layer.kind === "shape"
							? "shape"
							: layer.kind === "text"
								? "text"
								: "image";
	return `${DRAW_PLAY_TREE_PREFIX}.${segment}.${layer.id}`;
}

export function findDrawLayer(doc: DrawDocument, layerId: string): DrawLayerNode | null {
	for (const layer of doc.layers) {
		const found = findDrawLayerInNode(layer, layerId);
		if (found) return found;
	}
	return null;
}

function findDrawLayerInNode(node: DrawLayerNode, layerId: string): DrawLayerNode | null {
	if (node.id === layerId) return node;
	if (node.kind === "group") {
		for (const child of node.children) {
			const found = findDrawLayerInNode(child, layerId);
			if (found) return found;
		}
	}
	return null;
}

export function drawLayerDescendantLeafIds(doc: DrawDocument, layerId: string): string[] {
	const layer = findDrawLayer(doc, layerId);
	if (!layer) return [];
	if (layer.kind !== "group") return [layerId];
	const out: string[] = [];
	const walk = (layers: readonly DrawLayerNode[]) => {
		for (const child of layers) {
			if (child.kind === "group") walk(child.children);
			else out.push(child.id);
		}
	};
	walk(layer.children);
	return out;
}

export function flattenDrawLayers(layers: readonly DrawLayerNode[]): DrawLayerNode[] {
	const out: DrawLayerNode[] = [];
	const walk = (nodes: readonly DrawLayerNode[]) => {
		for (const node of nodes) {
			out.push(node);
			if (node.kind === "group") walk(node.children);
		}
	};
	walk(layers);
	return out;
}

function drawKindHoverDomainForLayer(layer: DrawLayerNode): DrawKindHoverDomain {
	if (layer.kind === "group") return "group";
	if (layer.kind === "boolean") return "boolean";
	if (layer.kind === "trace") return "trace";
	if (layer.kind === "shape") return "shape";
	return "layer";
}

export function drawPlayHoverPayloadFromTreeRowId(doc: DrawDocument, rowId: string | null): DrawHoverPayload {
	if (!rowId) return { id: null, kind: null };
	const refId = drawPlayLayerIdFromBooleanChildRowId(rowId);
	if (refId) {
		const layer = findDrawLayer(doc, refId);
		if (layer) return { id: layer.id, kind: { domain: drawKindHoverDomainForLayer(layer), kindId: layer.id } };
	}
	const layerMatch = rowId.match(/^draw-play-layers\.(layer|group|boolean|trace|path|shape|text|image)\.(.+)$/);
	if (layerMatch) {
		const layer = findDrawLayer(doc, layerMatch[2]!);
		if (layer) return { id: layer.id, kind: { domain: drawKindHoverDomainForLayer(layer), kindId: layer.id } };
	}
	return { id: null, kind: null };
}

export function drawPlayLayersTreeHighlightedIds(doc: DrawDocument, hoveredId: string | null, kindHover: DrawKindHover | null): readonly string[] {
	const out: string[] = [];
	const pushLayer = (layer: DrawLayerNode) => {
		out.push(drawPlayLayersTreeRowId(layer));
		for (const root of doc.layers) {
			const walk = (node: DrawLayerNode) => {
				if (node.kind === "boolean") {
					if (node.children.includes(layer.id)) out.push(drawPlayBooleanChildRowId(node.id, layer.id));
				}
				if (node.kind === "group") node.children.forEach(walk);
			};
			walk(root);
		}
	};
	if (hoveredId) {
		const layer = findDrawLayer(doc, hoveredId);
		if (layer) pushLayer(layer);
	}
	if (kindHover?.kindId) {
		for (const layer of flattenDrawLayers(doc.layers)) {
			if (layer.id === kindHover.kindId) pushLayer(layer);
		}
	}
	return out;
}

export interface DrawViewport {
	readonly width: number;
	readonly height: number;
}

export interface DrawScreenRect {
	readonly x: number;
	readonly y: number;
	readonly width: number;
	readonly height: number;
}

export function drawWorldToScreen(
	camera: DrawCamera,
	viewport: DrawViewport,
	world: { readonly x: number; readonly y: number },
): { x: number; y: number } {
	return {
		x: (world.x - camera.x) * camera.zoom + viewport.width / 2,
		y: (world.y - camera.y) * camera.zoom + viewport.height / 2,
	};
}

export function drawScreenToWorld(
	camera: DrawCamera,
	viewport: DrawViewport,
	screen: { readonly x: number; readonly y: number },
): { x: number; y: number } {
	return {
		x: (screen.x - viewport.width / 2) / camera.zoom + camera.x,
		y: (screen.y - viewport.height / 2) / camera.zoom + camera.y,
	};
}

function drawTransformWorldPoint(transform: DrawTransform, local: { readonly x: number; readonly y: number }): { x: number; y: number } {
	const sx = local.x * transform.scaleX;
	const sy = local.y * transform.scaleY;
	const cos = Math.cos(transform.rotation);
	const sin = Math.sin(transform.rotation);
	return {
		x: transform.x + sx * cos - sy * sin,
		y: transform.y + sx * sin + sy * cos,
	};
}

function pathBounds(segments: readonly PathSegment[]): DrawScreenRect | null {
	let minX = Number.POSITIVE_INFINITY;
	let minY = Number.POSITIVE_INFINITY;
	let maxX = Number.NEGATIVE_INFINITY;
	let maxY = Number.NEGATIVE_INFINITY;
	for (const segment of segments) {
		if ("to" in segment) {
			minX = Math.min(minX, segment.to[0]);
			minY = Math.min(minY, segment.to[1]);
			maxX = Math.max(maxX, segment.to[0]);
			maxY = Math.max(maxY, segment.to[1]);
		}
	}
	if (!Number.isFinite(minX)) return null;
	return { x: minX, y: minY, width: maxX - minX, height: maxY - minY };
}

function drawLayerLocalBounds(layer: DrawLayerNode): DrawScreenRect | null {
	if (layer.kind === "text") {
		const width = Math.max(8, layer.content.length * layer.size * 0.6);
		const height = Math.max(8, layer.size * 1.2);
		return { x: layer.x, y: layer.y, width, height };
	}
	if (layer.kind === "image") return { x: 0, y: 0, width: layer.width, height: layer.height };
	const segments = layerToPathSegments(layer);
	if (!segments.length) return { x: -64, y: -64, width: 128, height: 128 };
	return pathBounds(segments);
}

export function drawLayerWorldBounds(layer: DrawLayerNode): DrawScreenRect | null {
	const local = drawLayerLocalBounds(layer);
	if (!local) return null;
	const corners = [
		{ x: local.x, y: local.y },
		{ x: local.x + local.width, y: local.y },
		{ x: local.x + local.width, y: local.y + local.height },
		{ x: local.x, y: local.y + local.height },
	].map((point) => drawTransformWorldPoint(layer.transform, point));
	const xs = corners.map((point) => point.x);
	const ys = corners.map((point) => point.y);
	return { x: Math.min(...xs), y: Math.min(...ys), width: Math.max(...xs) - Math.min(...xs), height: Math.max(...ys) - Math.min(...ys) };
}

function drawScreenRectContainsPoint(rect: DrawScreenRect, point: { readonly x: number; readonly y: number }): boolean {
	return point.x >= rect.x && point.x <= rect.x + rect.width && point.y >= rect.y && point.y <= rect.y + rect.height;
}

function drawScreenRectIntersects(a: DrawScreenRect, b: DrawScreenRect): boolean {
	return a.x <= b.x + b.width && a.x + a.width >= b.x && a.y <= b.y + b.height && a.y + a.height >= b.y;
}

function drawScreenRectContains(outer: DrawScreenRect, inner: DrawScreenRect): boolean {
	return (
		inner.x >= outer.x &&
		inner.y >= outer.y &&
		inner.x + inner.width <= outer.x + outer.width &&
		inner.y + inner.height <= outer.y + outer.height
	);
}

export function resolveDrawMarqueeLayerHits(
	doc: DrawDocument,
	camera: DrawCamera,
	viewport: DrawViewport,
	marquee: DrawScreenRect,
	crossing: boolean,
): string[] {
	const hits: string[] = [];
	for (const layer of flattenDrawLayers(doc.layers)) {
		if (!layer.visible || layer.kind === "group") continue;
		const bounds = drawLayerWorldBounds(layer);
		if (!bounds) continue;
		const topLeft = drawWorldToScreen(camera, viewport, { x: bounds.x, y: bounds.y });
		const bottomRight = drawWorldToScreen(camera, viewport, { x: bounds.x + bounds.width, y: bounds.y + bounds.height });
		const screenBounds = {
			x: Math.min(topLeft.x, bottomRight.x),
			y: Math.min(topLeft.y, bottomRight.y),
			width: Math.abs(bottomRight.x - topLeft.x),
			height: Math.abs(bottomRight.y - topLeft.y),
		};
		if (crossing ? drawScreenRectIntersects(marquee, screenBounds) : drawScreenRectContains(marquee, screenBounds)) {
			hits.push(layer.id);
		}
	}
	return hits;
}

export function resolveDrawLayerAtScreenPoint(
	doc: DrawDocument,
	camera: DrawCamera,
	viewport: DrawViewport,
	point: { readonly x: number; readonly y: number },
): string | null {
	const targets = resolveDrawPickTargetsAtScreenPoint(doc, camera, viewport, point);
	if (targets.length === 0) return null;
	let best = targets[0]!;
	for (const target of targets) {
		if (target.generality > best.generality) best = target;
	}
	return best.id;
}

/** @emoji 🎯 Draw pick-target domain generality (lower = more general). */
export const DRAW_PICK_GENERALITY: Readonly<Record<string, number>> = {
	group: 0,
	boolean: 1,
	trace: 1,
	shape: 2,
	path: 2,
	text: 2,
	image: 2,
	layer: 2,
	controlPoint: 4,
};

export interface DrawPickTarget {
	readonly domain: string;
	readonly id: string;
	readonly generality: number;
	readonly label?: string;
	readonly layerId?: string;
}

function drawPickTargetForLayer(layer: DrawLayerNode): DrawPickTarget {
	const domain = drawKindHoverDomainForLayer(layer);
	return {
		domain,
		id: layer.id,
		generality: DRAW_PICK_GENERALITY[domain] ?? 2,
		label: layer.name,
		layerId: layer.id,
	};
}

function drawAncestorGroupTargets(doc: DrawDocument, layerId: string): DrawPickTarget[] {
	const out: DrawPickTarget[] = [];
	const walk = (layers: readonly DrawLayerNode[], ancestors: readonly DrawGroupLayer[]): void => {
		for (const layer of layers) {
			const nextAncestors = layer.kind === "group" ? [...ancestors, layer] : ancestors;
			if (layer.id === layerId) {
				for (const group of nextAncestors) {
					if (group.visible && !group.locked) out.push(drawPickTargetForLayer(group));
				}
				return;
			}
			if (layer.kind === "group") walk(layer.children, nextAncestors);
		}
	};
	walk(doc.layers, []);
	return out;
}

function drawControlPointTargets(
	layer: DrawLayerNode,
	world: { readonly x: number; readonly y: number },
	toleranceWorld: number,
): DrawPickTarget[] {
	if (layer.locked || !layer.visible) return [];
	const segments = layerToPathSegments(layer);
	if (!segments.length) return [];
	const out: DrawPickTarget[] = [];
	const pushPoint = (id: string, point: readonly [number, number], label: string) => {
		const worldPoint = drawTransformWorldPoint(layer.transform ?? defaultDrawTransform(), { x: point[0], y: point[1] });
		if (Math.hypot(world.x - worldPoint.x, world.y - worldPoint.y) <= toleranceWorld) {
			out.push({
				domain: "controlPoint",
				id,
				generality: DRAW_PICK_GENERALITY.controlPoint!,
				label,
				layerId: layer.id,
			});
		}
	};
	let pointIndex = 0;
	for (const segment of segments) {
		if ("to" in segment) {
			pushPoint(`${layer.id}:pt:${pointIndex}`, segment.to, `Point ${pointIndex + 1}`);
			pointIndex += 1;
		}
		if (segment.kind === "quad") pushPoint(`${layer.id}:ctrl:${pointIndex}`, segment.ctrl, `Control ${pointIndex}`);
		if (segment.kind === "cubic") {
			pushPoint(`${layer.id}:ctrl1:${pointIndex}`, segment.ctrl1, `Control 1`);
			pushPoint(`${layer.id}:ctrl2:${pointIndex}`, segment.ctrl2, `Control 2`);
		}
	}
	return out;
}

/** @emoji 🎯 All pick targets under a screen point (groups, layers, optional control points). */
export function resolveDrawPickTargetsAtScreenPoint(
	doc: DrawDocument,
	camera: DrawCamera,
	viewport: DrawViewport,
	point: { readonly x: number; readonly y: number },
	options: { readonly includeControlPoints?: boolean } = {},
): DrawPickTarget[] {
	const world = drawScreenToWorld(camera, viewport, point);
	const toleranceWorld = 8 / Math.max(camera.zoom, 1e-6);
	const hits: DrawPickTarget[] = [];
	const layers = flattenDrawLayers(doc.layers);
	for (let index = layers.length - 1; index >= 0; index -= 1) {
		const layer = layers[index]!;
		if (!layer.visible || layer.locked) continue;
		const bounds = drawLayerWorldBounds(layer);
		if (!bounds || !drawScreenRectContainsPoint(bounds, world)) continue;
		if (layer.kind === "group") {
			hits.push(drawPickTargetForLayer(layer));
			continue;
		}
		hits.push(drawPickTargetForLayer(layer));
		for (const groupTarget of drawAncestorGroupTargets(doc, layer.id)) {
			if (!hits.some((row) => row.id === groupTarget.id)) hits.push(groupTarget);
		}
		if (options.includeControlPoints && (layer.kind === "path" || layer.kind === "shape")) {
			for (const cp of drawControlPointTargets(layer, world, toleranceWorld)) hits.push(cp);
		}
	}
	return hits;
}
// #endregion 🌳TreeIds

// #region ✏️EditOps
function mapLayers(
	layers: readonly DrawLayerNode[],
	fn: (layer: DrawLayerNode) => DrawLayerNode | null,
): DrawLayerNode[] {
	const out: DrawLayerNode[] = [];
	for (const layer of layers) {
		const mapped = fn(layer);
		if (mapped) out.push(mapped);
	}
	return out;
}

function updateLayerInTree(layers: readonly DrawLayerNode[], layerId: string, update: (layer: DrawLayerNode) => DrawLayerNode): DrawLayerNode[] {
	return mapLayers(layers, (layer) => {
		if (layer.id === layerId) return update(layer);
		if (layer.kind === "group") return { ...layer, children: updateLayerInTree(layer.children, layerId, update) };
		return layer;
	});
}

function removeLayerFromTree(layers: readonly DrawLayerNode[], layerId: string): DrawLayerNode[] {
	return mapLayers(layers, (layer) => {
		if (layer.id === layerId) return null;
		if (layer.kind === "group") return { ...layer, children: removeLayerFromTree(layer.children, layerId) };
		return layer;
	});
}

function insertLayer(
	layers: readonly DrawLayerNode[],
	parentId: string | undefined,
	index: number,
	inserted: DrawLayerNode,
): DrawLayerNode[] {
	if (!parentId) {
		const next = [...layers];
		next.splice(Math.max(0, Math.min(index, next.length)), 0, inserted);
		return next;
	}
	return mapLayers(layers, (layer) => {
		if (layer.kind === "group" && layer.id === parentId) {
			const children = [...layer.children];
			children.splice(Math.max(0, Math.min(index, children.length)), 0, inserted);
			return { ...layer, children };
		}
		if (layer.kind === "group") return { ...layer, children: insertLayer(layer.children, parentId, index, inserted) };
		return layer;
	});
}

export function mutateDrawLayer(doc: DrawDocument, layerId: string, mutate: (layer: DrawLayerNode) => DrawLayerNode): DrawDocument {
	return { ...doc, layers: updateLayerInTree(doc.layers, layerId, mutate) };
}

export function applyDrawEditOp(doc: DrawDocument, edit: DrawEditOp): DrawDocument {
	switch (edit.op) {
		case "setLayerVisible":
			return { ...doc, layers: updateLayerInTree(doc.layers, edit.layerId, (layer) => ({ ...layer, visible: edit.visible })) };
		case "setLayerLocked":
			return { ...doc, layers: updateLayerInTree(doc.layers, edit.layerId, (layer) => ({ ...layer, locked: edit.locked })) };
		case "setLayerOpacity":
			return { ...doc, layers: updateLayerInTree(doc.layers, edit.layerId, (layer) => ({ ...layer, opacity: edit.opacity })) };
		case "setLayerBlendMode":
			return { ...doc, layers: updateLayerInTree(doc.layers, edit.layerId, (layer) => ({ ...layer, blendMode: edit.blendMode })) };
		case "setLayerName":
			return { ...doc, layers: updateLayerInTree(doc.layers, edit.layerId, (layer) => ({ ...layer, name: edit.name })) };
		case "setLayerTransform":
			return { ...doc, layers: updateLayerInTree(doc.layers, edit.layerId, (layer) => ({ ...layer, transform: edit.transform })) };
		case "setFill":
			return {
				...doc,
				layers: updateLayerInTree(doc.layers, edit.layerId, (layer) => ({
					...layer,
					attributes: { ...layer.attributes, fill: edit.fill },
				})),
			};
		case "setStroke":
			return {
				...doc,
				layers: updateLayerInTree(doc.layers, edit.layerId, (layer) => ({
					...layer,
					attributes: { ...layer.attributes, stroke: edit.stroke },
				})),
			};
		case "setBooleanOp":
			return {
				...doc,
				layers: updateLayerInTree(doc.layers, edit.layerId, (layer) =>
					layer.kind === "boolean" ? { ...layer, op: edit.booleanOp } : layer,
				),
			};
		case "setTraceParams":
			return {
				...doc,
				layers: updateLayerInTree(doc.layers, edit.layerId, (layer) =>
					layer.kind === "trace" ? { ...layer, params: edit.params } : layer,
				),
			};
		case "addShapeLayer":
			return { ...doc, layers: insertLayer(doc.layers, edit.parentId, edit.index ?? doc.layers.length, edit.layer) };
		case "addPathLayer":
			return { ...doc, layers: insertLayer(doc.layers, edit.parentId, edit.index ?? doc.layers.length, edit.layer) };
		case "addTextLayer":
			return { ...doc, layers: insertLayer(doc.layers, edit.parentId, edit.index ?? doc.layers.length, edit.layer) };
		case "addImageLayer":
			return { ...doc, layers: insertLayer(doc.layers, edit.parentId, edit.index ?? doc.layers.length, edit.layer) };
		case "addGroupLayer":
			return { ...doc, layers: insertLayer(doc.layers, edit.parentId, edit.index ?? doc.layers.length, edit.layer) };
		case "addBooleanLayer":
			return { ...doc, layers: insertLayer(doc.layers, edit.parentId, edit.index ?? doc.layers.length, edit.layer) };
		case "addTraceLayer":
			return { ...doc, layers: insertLayer(doc.layers, edit.parentId, edit.index ?? doc.layers.length, edit.layer) };
		case "duplicateLayer": {
			const layer = findDrawLayer(doc, edit.layerId);
			if (!layer) return doc;
			const location = findDrawLayerLocation(doc, edit.layerId);
			if (!location) return doc;
			const clone = cloneDrawLayerNode(layer);
			return { ...doc, layers: insertLayer(doc.layers, location.parentId, location.index + 1, clone) };
		}
		case "removeLayer":
			return { ...doc, layers: removeLayerFromTree(doc.layers, edit.layerId) };
		case "reorderLayer": {
			const layer = findDrawLayer(doc, edit.layerId);
			if (!layer) return doc;
			const without = removeLayerFromTree(doc.layers, edit.layerId);
			return { ...doc, layers: insertLayer(without, edit.parentId, edit.index, layer) };
		}
		case "setActiveTool":
			return { ...doc, activeTool: edit.tool };
		case "setCamera":
			return { ...doc, camera: edit.camera };
		case "setDocument":
			return edit.document;
		default:
			return doc;
	}
}
// #endregion ✏️EditOps

//#region 🔖DocumentVcs
export type DrawDocumentVcsEnvelope = DocumentVcsEnvelope<DrawDocument, DrawEditOp>;

/** @emoji ↩️ Inverts a draw edit from the pre-apply projection. */
export function backwardsDrawEditOp(projection: DrawDocument, operation: DrawEditOp): readonly DrawEditOp[] {
	switch (operation.op) {
		case "setDocument":
			return [{ op: "setDocument", document: projection }];
		case "setCamera":
			return [{ op: "setCamera", camera: projection.camera }];
		case "setActiveTool":
			return [{ op: "setActiveTool", tool: projection.activeTool }];
		case "setLayerVisible": {
			const layer = findDrawLayer(projection, operation.layerId);
			return layer ? [{ op: "setLayerVisible", layerId: operation.layerId, visible: layer.visible }] : [{ op: "setDocument", document: projection }];
		}
		case "setLayerLocked": {
			const layer = findDrawLayer(projection, operation.layerId);
			return layer ? [{ op: "setLayerLocked", layerId: operation.layerId, locked: layer.locked }] : [{ op: "setDocument", document: projection }];
		}
		case "setLayerOpacity": {
			const layer = findDrawLayer(projection, operation.layerId);
			return layer ? [{ op: "setLayerOpacity", layerId: operation.layerId, opacity: layer.opacity }] : [{ op: "setDocument", document: projection }];
		}
		case "setLayerName": {
			const layer = findDrawLayer(projection, operation.layerId);
			return layer ? [{ op: "setLayerName", layerId: operation.layerId, name: layer.name }] : [{ op: "setDocument", document: projection }];
		}
		default:
			return [{ op: "setDocument", document: projection }];
	}
}

/** @emoji 📊 Returns the draw edit payload for persistence diffs. */
export function diffDrawEditOp(_projection: DrawDocument, operation: DrawEditOp): unknown {
	return operation;
}

/** @emoji 📦 Creates a draw document VCS envelope with an empty or seeded projection. */
export function createDrawDocumentVcsEnvelope(id: string, projection: DrawDocument = defaultDrawDocument(id)): DrawDocumentVcsEnvelope {
	return createDocumentVcsEnvelope("draw.document", id, projection);
}

/** @emoji 🔁 Materializes a draw document from its VCS envelope. */
export function materializeDrawDocument(envelope: DrawDocumentVcsEnvelope, appliedChangeIds: readonly string[] = []): DrawDocument {
	return materializeDocumentProjection(envelope, appliedChangeIds, applyDrawEditOp);
}

/** @emoji 🧩 S app VCS handler factory for draw documents. */
export function createDrawAppVcsHandler() {
	return {
		format: "draw.document",
		createEnvelope: (id: string) => createDrawDocumentVcsEnvelope(id),
		applyOp: applyDrawEditOp,
		serializeEnvelope: (envelope: DrawDocumentVcsEnvelope) => JSON.stringify(envelope),
		deserializeEnvelope: (json: string) => JSON.parse(json) as DrawDocumentVcsEnvelope,
		materializeProjection: (source: { readonly vcsJson?: string; readonly inline?: string }) => {
			if (source.vcsJson) {
				const envelope = JSON.parse(source.vcsJson) as DrawDocumentVcsEnvelope;
				return materializeDrawDocument(envelope, envelope.vcs.edits.map((edit) => edit.id));
			}
			if (source.inline) return drawDocumentFromJson(source.inline);
			return defaultDrawDocument("draw");
		},
	};
}
//#endregion 🔖DocumentVcs
