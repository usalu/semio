// #region 🧲Header
/// <reference types="vite/client" />
/// <reference types="vitest/importMeta" />
/** @emoji ✏️ `@semio-tech/draw-core` — non-destructive vector document model, edit ops, hover/selection mapping. */
// #endregion 🧲Header

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

export const DRAW_TOOL_IDS = [
	"selectMarquee",
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

export interface DrawDocument {
	readonly schema: "draw.document/v1";
	readonly id: string;
	readonly title?: string;
	readonly camera: DrawCamera;
	readonly layers: readonly DrawLayerNode[];
	readonly assets?: Readonly<Record<string, DrawImageAsset>>;
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
	| { readonly op: "setCamera"; readonly camera: DrawCamera };
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

export function defaultDrawDocument(id = "empty", title?: string): DrawDocument {
	return {
		schema: "draw.document/v1",
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
	if (record.schema !== "draw.document/v1") throw new Error(`unsupported draw schema: ${String((raw as { schema?: string }).schema)}`);
	if (!Array.isArray(record.layers)) throw new Error("draw document layers must be an array");
	return record;
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
			const segments = layerToPathSegments(layer);
			if (segments.length === 0 && layer.kind !== "text") continue;
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

export function drawPlayLayerIdFromTreeRowId(rowId: string): string | null {
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

export function drawPlayHoverPayloadFromTreeRowId(doc: DrawDocument, rowId: string | null): DrawHoverPayload {
	if (!rowId) return { id: null, kind: null };
	const layerMatch = rowId.match(/^draw-play-layers\.(layer|group|boolean|trace|path|shape|text|image)\.(.+)$/);
	if (layerMatch) {
		const layer = findDrawLayer(doc, layerMatch[2]!);
		if (layer) {
			const domain: DrawKindHoverDomain =
				layer.kind === "group"
					? "group"
					: layer.kind === "boolean"
						? "boolean"
						: layer.kind === "trace"
							? "trace"
							: layer.kind === "shape"
								? "shape"
								: "layer";
			return { id: layer.id, kind: { domain, kindId: layer.id } };
		}
	}
	return { id: null, kind: null };
}

export function drawPlayLayersTreeHighlightedIds(doc: DrawDocument, hoveredId: string | null, kindHover: DrawKindHover | null): readonly string[] {
	if (hoveredId) {
		const layer = findDrawLayer(doc, hoveredId);
		if (layer) return [drawPlayLayersTreeRowId(layer)];
	}
	if (kindHover?.kindId) {
		for (const layer of flattenDrawLayers(doc.layers)) {
			if (layer.id === kindHover.kindId) return [drawPlayLayersTreeRowId(layer)];
		}
	}
	return [];
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

export function drawLayerWorldBounds(layer: DrawLayerNode): DrawScreenRect | null {
	const segments = layerToPathSegments(layer);
	if (!segments.length) return { x: -64, y: -64, width: 128, height: 128 };
	const local = pathBounds(segments);
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
	const world = drawScreenToWorld(camera, viewport, point);
	const layers = flattenDrawLayers(doc.layers);
	for (let index = layers.length - 1; index >= 0; index -= 1) {
		const layer = layers[index]!;
		if (!layer.visible || layer.kind === "group") continue;
		const bounds = drawLayerWorldBounds(layer);
		if (bounds && drawScreenRectContainsPoint(bounds, world)) return layer.id;
	}
	return null;
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
		default:
			return doc;
	}
}
// #endregion ✏️EditOps

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("@semio-tech/draw-core", () => {
		it("parses draw documents", () => {
			const doc = defaultDrawDocument("test");
			expect(parseDrawDocument(doc).schema).toBe("draw.document/v1");
		});

		it("applies visibility edits", () => {
			const doc = defaultDrawDocument("test");
			const layerId = doc.layers[0]!.id;
			const next = applyDrawEditOp(doc, { op: "setLayerVisible", layerId, visible: false });
			expect(findDrawLayer(next, layerId)?.visible).toBe(false);
		});

		it("maps tree row ids", () => {
			const layer = createDrawPathLayer("A");
			expect(drawPlayLayerIdFromTreeRowId(drawPlayLayersTreeRowId(layer))).toBe(layer.id);
		});

		it("flattens scene nodes with boolean placeholders", () => {
			const doc: DrawDocument = {
				...defaultDrawDocument("bool"),
				layers: [createDrawBooleanLayer("U", "union", ["a", "b"])],
			};
			const nodes = flattenDrawDocumentToSceneNodes(doc);
			expect(nodes[0]?.needsKernel).toBe(true);
			expect(nodes[0]?.kernelKind).toBe("boolean");
		});
	});
}
// #endregion 🧪Tests
