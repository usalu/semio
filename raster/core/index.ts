// #region 🧲Header
/// <reference types="vite/client" />
/// <reference types="vitest/importMeta" />
/** @emoji 🖼️ `@semio-tech/raster-core` — non-destructive raster document model, edit ops, hover/selection mapping. */
// #endregion 🧲Header

// #region 📐Types
export const RASTER_BLEND_MODES = [
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

export type RasterBlendMode = (typeof RASTER_BLEND_MODES)[number];

export const RASTER_ADJUSTMENT_KINDS = ["brightnessContrast", "levels", "hueSaturation", "curves"] as const;
export type RasterAdjustmentKind = (typeof RASTER_ADJUSTMENT_KINDS)[number];

export const RASTER_FILTER_KINDS = ["blur", "sharpen", "gaussianBlur", "motionBlur"] as const;
export type RasterFilterKind = (typeof RASTER_FILTER_KINDS)[number];

export const RASTER_TOOL_IDS = [
	"selectMarquee",
	"selectLasso",
	"selectWand",
	"paintBrush",
	"paintEraser",
	"paintClone",
	"transformMove",
	"transformScale",
	"transformRotate",
] as const;

export type RasterToolId = (typeof RASTER_TOOL_IDS)[number];

export interface RasterCamera {
	readonly x: number;
	readonly y: number;
	readonly zoom: number;
}

export interface RasterTransform {
	readonly x: number;
	readonly y: number;
	readonly scaleX: number;
	readonly scaleY: number;
	readonly rotation: number;
}

export interface RasterLayerMask {
	readonly enabled: boolean;
	readonly linked: boolean;
	readonly invert: boolean;
	readonly width?: number;
	readonly height?: number;
}

export interface RasterFilterEntry {
	readonly kind: RasterFilterKind;
	readonly radius?: number;
	readonly amount?: number;
}

export interface RasterAdjustmentParams {
	readonly brightness?: number;
	readonly contrast?: number;
	readonly hue?: number;
	readonly saturation?: number;
	readonly levelsBlack?: number;
	readonly levelsWhite?: number;
	readonly curves?: readonly (readonly [number, number])[];
}

export interface RasterLayerBase {
	readonly id: string;
	readonly name: string;
	readonly visible: boolean;
	readonly opacity: number;
	readonly blendMode: RasterBlendMode;
	readonly transform: RasterTransform;
	readonly mask?: RasterLayerMask;
	readonly clipToBelow?: boolean;
	readonly width?: number;
	readonly height?: number;
}

export interface RasterImageAsset {
	readonly mime: string;
	readonly data: string;
}

export interface RasterPixelLayer extends RasterLayerBase {
	readonly kind: "pixel";
	readonly imageKey?: string;
	readonly filters?: readonly RasterFilterEntry[];
}

export interface RasterGroupLayer extends RasterLayerBase {
	readonly kind: "group";
	readonly children: readonly RasterLayerNode[];
}

export interface RasterAdjustmentLayer extends RasterLayerBase {
	readonly kind: "adjustment";
	readonly adjustmentKind: RasterAdjustmentKind;
	readonly params: RasterAdjustmentParams;
}

export type RasterLayerNode = RasterPixelLayer | RasterGroupLayer | RasterAdjustmentLayer;

export interface RasterDocument {
	readonly schema: "raster.document/v1";
	readonly id: string;
	readonly title?: string;
	readonly camera: RasterCamera;
	readonly layers: readonly RasterLayerNode[];
	readonly assets?: Readonly<Record<string, RasterImageAsset>>;
	readonly activeTool?: RasterToolId;
	readonly brushSize?: number;
	readonly brushOpacity?: number;
}

export type RasterKindHoverDomain = "layer" | "group" | "mask" | "adjustment" | "blendMode";

export interface RasterKindHover {
	readonly domain: RasterKindHoverDomain;
	readonly kindId: string;
}

export interface RasterHoverPayload {
	readonly id: string | null;
	readonly kind: RasterKindHover | null;
}

export type RasterEditOp =
	| { readonly op: "setLayerVisible"; readonly layerId: string; readonly visible: boolean }
	| { readonly op: "setLayerOpacity"; readonly layerId: string; readonly opacity: number }
	| { readonly op: "setLayerBlendMode"; readonly layerId: string; readonly blendMode: RasterBlendMode }
	| { readonly op: "setLayerName"; readonly layerId: string; readonly name: string }
	| { readonly op: "addPixelLayer"; readonly parentId?: string; readonly index?: number; readonly layer: RasterPixelLayer }
	| { readonly op: "addGroupLayer"; readonly parentId?: string; readonly index?: number; readonly layer: RasterGroupLayer }
	| { readonly op: "addAdjustmentLayer"; readonly parentId?: string; readonly index?: number; readonly layer: RasterAdjustmentLayer }
	| { readonly op: "duplicateLayer"; readonly layerId: string }
	| { readonly op: "setLayerMask"; readonly layerId: string; readonly mask: RasterLayerMask | undefined }
	| { readonly op: "setLayerSize"; readonly layerId: string; readonly width?: number; readonly height?: number }
	| { readonly op: "setAdjustmentKind"; readonly layerId: string; readonly adjustmentKind: RasterAdjustmentKind }
	| { readonly op: "appendLayerFilter"; readonly layerId: string; readonly filter: RasterFilterEntry }
	| { readonly op: "removeLayer"; readonly layerId: string }
	| { readonly op: "reorderLayer"; readonly layerId: string; readonly parentId?: string; readonly index: number }
	| { readonly op: "setActiveTool"; readonly tool: RasterToolId }
	| { readonly op: "setBrushSize"; readonly size: number }
	| { readonly op: "setCamera"; readonly camera: RasterCamera };
// #endregion 📐Types

// #region 🔧Helpers
let rasterIdCounter = 0;

/** @emoji 🆔 Allocates a stable raster entity id. */
export function createRasterId(prefix = "layer"): string {
	rasterIdCounter += 1;
	return `${prefix}-${rasterIdCounter}`;
}

/** @emoji 📐 Default infinite-canvas transform at origin. */
export function defaultRasterTransform(): RasterTransform {
	return { x: 0, y: 0, scaleX: 1, scaleY: 1, rotation: 0 };
}

/** @emoji 📍 Parent group id and sibling index for a layer in the document tree. */
export interface RasterLayerLocation {
	readonly parentId?: string;
	readonly index: number;
}

/** @emoji 🆕 Default pixel layer sized for painting. */
export function createRasterPixelLayer(name = "Layer", width = 512, height = 512): RasterPixelLayer {
	const id = createRasterId("layer");
	return {
		kind: "pixel",
		id,
		name,
		visible: true,
		opacity: 1,
		blendMode: "normal",
		transform: defaultRasterTransform(),
		width,
		height,
	};
}

/** @emoji 📁 Empty group layer. */
export function createRasterGroupLayer(name = "Group"): RasterGroupLayer {
	return {
		kind: "group",
		id: createRasterId("group"),
		name,
		visible: true,
		opacity: 1,
		blendMode: "normal",
		transform: defaultRasterTransform(),
		children: [],
	};
}

/** @emoji 🎚️ Default brightness/contrast adjustment layer. */
export function createRasterAdjustmentLayer(name = "Adjustment"): RasterAdjustmentLayer {
	return {
		kind: "adjustment",
		id: createRasterId("adjust"),
		name,
		visible: true,
		opacity: 1,
		blendMode: "normal",
		transform: defaultRasterTransform(),
		adjustmentKind: "brightnessContrast",
		params: { brightness: 0, contrast: 0 },
	};
}

/** @emoji 📋 Deep-clones a layer subtree with fresh ids. */
export function cloneRasterLayerNode(node: RasterLayerNode, nameSuffix = " copy"): RasterLayerNode {
	const id = createRasterId(node.kind === "group" ? "group" : node.kind === "adjustment" ? "adjust" : "layer");
	if (node.kind === "group") {
		return {
			...node,
			id,
			name: `${node.name}${nameSuffix}`,
			children: node.children.map((child) => cloneRasterLayerNode(child, "")),
		};
	}
	if (node.kind === "adjustment") {
		return { ...node, id, name: `${node.name}${nameSuffix}` };
	}
	return { ...node, id, name: `${node.name}${nameSuffix}` };
}

/** @emoji 🖼️ Empty raster document with one paintable layer. */
export function defaultRasterDocument(id = "raster-default"): RasterDocument {
	const background = createRasterId("bg");
	return {
		schema: "raster.document/v1",
		id,
		title: "Untitled",
		camera: { x: 0, y: 0, zoom: 1 },
		layers: [
			{
				kind: "pixel",
				id: background,
				name: "Background",
				visible: true,
				opacity: 1,
				blendMode: "normal",
				transform: defaultRasterTransform(),
				width: 512,
				height: 512,
			},
		],
		activeTool: "selectMarquee",
		brushSize: 24,
		brushOpacity: 1,
	};
}

function isRecord(value: unknown): value is Record<string, unknown> {
	return typeof value === "object" && value !== null;
}

function parseBlendMode(raw: unknown): RasterBlendMode {
	if (typeof raw === "string" && (RASTER_BLEND_MODES as readonly string[]).includes(raw)) {
		return raw as RasterBlendMode;
	}
	return "normal";
}

function parseTransform(raw: unknown): RasterTransform {
	if (!isRecord(raw)) return defaultRasterTransform();
	return {
		x: typeof raw.x === "number" ? raw.x : 0,
		y: typeof raw.y === "number" ? raw.y : 0,
		scaleX: typeof raw.scaleX === "number" ? raw.scaleX : 1,
		scaleY: typeof raw.scaleY === "number" ? raw.scaleY : 1,
		rotation: typeof raw.rotation === "number" ? raw.rotation : 0,
	};
}

function parseMask(raw: unknown): RasterLayerMask | undefined {
	if (!isRecord(raw)) return undefined;
	return {
		enabled: raw.enabled !== false,
		linked: raw.linked !== false,
		invert: raw.invert === true,
		width: typeof raw.width === "number" ? raw.width : undefined,
		height: typeof raw.height === "number" ? raw.height : undefined,
	};
}

function parseFilterEntry(raw: unknown): RasterFilterEntry | null {
	if (!isRecord(raw)) return null;
	const kind = raw.kind;
	if (typeof kind !== "string" || !(RASTER_FILTER_KINDS as readonly string[]).includes(kind)) return null;
	return {
		kind: kind as RasterFilterKind,
		radius: typeof raw.radius === "number" ? raw.radius : undefined,
		amount: typeof raw.amount === "number" ? raw.amount : undefined,
	};
}

function parseAssets(raw: unknown): Readonly<Record<string, RasterImageAsset>> | undefined {
	if (!isRecord(raw)) return undefined;
	const out: Record<string, RasterImageAsset> = {};
	for (const [key, value] of Object.entries(raw)) {
		if (!isRecord(value)) continue;
		if (typeof value.mime !== "string" || typeof value.data !== "string") continue;
		out[key] = { mime: value.mime, data: value.data };
	}
	return Object.keys(out).length > 0 ? out : undefined;
}

function parseLayerBase(raw: Record<string, unknown>): Omit<RasterLayerBase, never> {
	return {
		id: typeof raw.id === "string" ? raw.id : createRasterId(),
		name: typeof raw.name === "string" ? raw.name : "Layer",
		visible: raw.visible !== false,
		opacity: typeof raw.opacity === "number" ? Math.min(1, Math.max(0, raw.opacity)) : 1,
		blendMode: parseBlendMode(raw.blendMode),
		transform: parseTransform(raw.transform),
		mask: parseMask(raw.mask),
		clipToBelow: raw.clipToBelow === true,
		width: typeof raw.width === "number" ? raw.width : undefined,
		height: typeof raw.height === "number" ? raw.height : undefined,
	};
}

function parseLayerNode(raw: unknown): RasterLayerNode {
	if (!isRecord(raw)) {
		throw new Error("raster layer must be an object");
	}
	const base = parseLayerBase(raw);
	const kind = raw.kind;
	if (kind === "group") {
		const childrenRaw = Array.isArray(raw.children) ? raw.children : [];
		return {
			...base,
			kind: "group",
			children: childrenRaw.map(parseLayerNode),
		};
	}
	if (kind === "adjustment") {
		const adjustmentKind =
			typeof raw.adjustmentKind === "string" && (RASTER_ADJUSTMENT_KINDS as readonly string[]).includes(raw.adjustmentKind)
				? (raw.adjustmentKind as RasterAdjustmentKind)
				: "brightnessContrast";
		const paramsRaw = isRecord(raw.params) ? raw.params : {};
		const curvesRaw = Array.isArray(paramsRaw.curves) ? paramsRaw.curves : [];
		const curves = curvesRaw
			.map((point) => (Array.isArray(point) && point.length === 2 && typeof point[0] === "number" && typeof point[1] === "number" ? ([point[0], point[1]] as const) : null))
			.filter((point): point is readonly [number, number] => point !== null);
		return {
			...base,
			kind: "adjustment",
			adjustmentKind,
			params: {
				brightness: typeof paramsRaw.brightness === "number" ? paramsRaw.brightness : undefined,
				contrast: typeof paramsRaw.contrast === "number" ? paramsRaw.contrast : undefined,
				hue: typeof paramsRaw.hue === "number" ? paramsRaw.hue : undefined,
				saturation: typeof paramsRaw.saturation === "number" ? paramsRaw.saturation : undefined,
				levelsBlack: typeof paramsRaw.levelsBlack === "number" ? paramsRaw.levelsBlack : undefined,
				levelsWhite: typeof paramsRaw.levelsWhite === "number" ? paramsRaw.levelsWhite : undefined,
				curves: curves.length > 0 ? curves : undefined,
			},
		};
	}
	const filtersRaw = Array.isArray(raw.filters) ? raw.filters : [];
	const filters = filtersRaw.map(parseFilterEntry).filter((entry): entry is RasterFilterEntry => entry !== null);
	return {
		...base,
		kind: "pixel",
		imageKey: typeof raw.imageKey === "string" ? raw.imageKey : undefined,
		filters: filters.length > 0 ? filters : undefined,
	};
}

/** @emoji 📥 Parses and validates a raster document fixture. */
export function parseRasterDocument(raw: unknown): RasterDocument {
	if (!isRecord(raw)) {
		throw new Error("raster document must be an object");
	}
	if (raw.schema !== "raster.document/v1") {
		throw new Error(`unsupported raster schema: ${String(raw.schema)}`);
	}
	const layersRaw = Array.isArray(raw.layers) ? raw.layers : [];
	const cameraRaw = isRecord(raw.camera) ? raw.camera : {};
	return {
		schema: "raster.document/v1",
		id: typeof raw.id === "string" ? raw.id : "raster",
		title: typeof raw.title === "string" ? raw.title : undefined,
		camera: {
			x: typeof cameraRaw.x === "number" ? cameraRaw.x : 0,
			y: typeof cameraRaw.y === "number" ? cameraRaw.y : 0,
			zoom: typeof cameraRaw.zoom === "number" ? cameraRaw.zoom : 1,
		},
		layers: layersRaw.map(parseLayerNode),
		assets: parseAssets(raw.assets),
		activeTool:
			typeof raw.activeTool === "string" && (RASTER_TOOL_IDS as readonly string[]).includes(raw.activeTool)
				? (raw.activeTool as RasterToolId)
				: undefined,
		brushSize: typeof raw.brushSize === "number" ? raw.brushSize : undefined,
		brushOpacity: typeof raw.brushOpacity === "number" ? raw.brushOpacity : undefined,
	};
}

/** @emoji 📤 Serializes a raster document to JSON. */
export function rasterDocumentToJson(doc: RasterDocument): string {
	return JSON.stringify(doc);
}

/** @emoji 📤 Serializes a raster document for file export. */
export function rasterDocumentToExportJson(doc: RasterDocument): string {
	return `${JSON.stringify(doc, null, 2)}\n`;
}

/** @emoji 📡 Serializes document JSON for the WASM compositor (omits embedded assets). */
export function rasterDocumentToSyncJson(doc: RasterDocument): string {
	const { assets: _assets, ...syncDoc } = doc;
	return JSON.stringify(syncDoc);
}

/** @emoji 🧩 Decodes a base64 raster image asset payload. */
export function decodeRasterImageAsset(asset: RasterImageAsset): Uint8Array {
	if (typeof Buffer !== "undefined") {
		return new Uint8Array(Buffer.from(asset.data, "base64"));
	}
	const binary = atob(asset.data);
	const bytes = new Uint8Array(binary.length);
	for (let index = 0; index < binary.length; index += 1) {
		bytes[index] = binary.charCodeAt(index);
	}
	return bytes;
}

/** @emoji 📥 Parses JSON text into a raster document. */
export function rasterDocumentFromJson(json: string): RasterDocument {
	return parseRasterDocument(JSON.parse(json));
}

export function rasterKindHoversEqual(a: RasterKindHover | null, b: RasterKindHover | null): boolean {
	if (a === b) return true;
	if (!a || !b) return false;
	return a.domain === b.domain && a.kindId === b.kindId;
}
// #endregion 🔧Helpers

// #region 🌳TreeIds
export const RASTER_PLAY_TREE_PREFIX = "raster-play-layers";

/** @emoji 🔍 Locates a layer's parent group id and sibling index. */
export function findRasterLayerLocation(doc: RasterDocument, layerId: string): RasterLayerLocation | null {
	const search = (layers: readonly RasterLayerNode[], parentId?: string): RasterLayerLocation | null => {
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

/** @emoji 🌳 Parses a hierarchy tree row id back to a layer id. */
export function rasterPlayLayerIdFromTreeRowId(rowId: string): string | null {
	const layerMatch = rowId.match(/^raster-play-layers\.(layer|group|adjustment)\.(.+)$/);
	return layerMatch?.[2] ?? null;
}

/** @emoji 📍 Resolves a tree drop gesture into a layer insert location. */
export function resolveRasterPlayReorderTarget(
	doc: RasterDocument,
	targetRowId: string,
	dropPosition: "before" | "after" | "inside",
): RasterLayerLocation | null {
	if (targetRowId.includes(".mask.")) return null;
	const layerId = rasterPlayLayerIdFromTreeRowId(targetRowId);
	if (!layerId) {
		if (dropPosition === "inside") return { index: doc.layers.length };
		return null;
	}
	const layer = findRasterLayer(doc, layerId);
	if (!layer) return null;
	if (dropPosition === "inside" && layer.kind === "group") {
		return { parentId: layer.id, index: layer.children.length };
	}
	const location = findRasterLayerLocation(doc, layerId);
	if (!location) return null;
	if (dropPosition === "before") return location;
	if (dropPosition === "after") return { parentId: location.parentId, index: location.index + 1 };
	return { parentId: location.parentId, index: location.index + 1 };
}

/** @emoji 🌳 Stable hierarchy tree row id for a layer node. */
export function rasterPlayLayersTreeRowId(layer: RasterLayerNode): string {
	const segment = layer.kind === "group" ? "group" : layer.kind === "adjustment" ? "adjustment" : "layer";
	return `${RASTER_PLAY_TREE_PREFIX}.${segment}.${layer.id}`;
}

/** @emoji 🌳 Stable mask tree row id. */
export function rasterPlayMaskTreeRowId(layerId: string): string {
	return `${RASTER_PLAY_TREE_PREFIX}.mask.${layerId}`;
}

/** @emoji 🌳 Blend-mode catalogue row id for transitive hover. */
export function rasterPlayBlendModeTreeRowId(blendMode: RasterBlendMode): string {
	return `${RASTER_PLAY_TREE_PREFIX}.blend.${blendMode}`;
}

/** @emoji 🔍 Finds a layer by id in the document tree. */
export function findRasterLayer(doc: RasterDocument, layerId: string): RasterLayerNode | null {
	for (const layer of doc.layers) {
		const found = findRasterLayerInNode(layer, layerId);
		if (found) return found;
	}
	return null;
}

function findRasterLayerInNode(node: RasterLayerNode, layerId: string): RasterLayerNode | null {
	if (node.id === layerId) return node;
	if (node.kind === "group") {
		for (const child of node.children) {
			const found = findRasterLayerInNode(child, layerId);
			if (found) return found;
		}
	}
	return null;
}

/** @emoji 🌳 Flattens visible layer nodes depth-first. */
export function flattenRasterLayers(layers: readonly RasterLayerNode[]): RasterLayerNode[] {
	const out: RasterLayerNode[] = [];
	const walk = (nodes: readonly RasterLayerNode[]) => {
		for (const node of nodes) {
			out.push(node);
			if (node.kind === "group") walk(node.children);
		}
	};
	walk(layers);
	return out;
}

/** @emoji 🖱️ Maps hover focus to tree row ids (direct instance hover). */
export function rasterPlayLayersTreeSelectedIds(layerId: string | null): readonly string[] {
	if (!layerId) return [];
	return [rasterPlayLayersTreeRowId({ id: layerId, kind: "pixel", name: "", visible: true, opacity: 1, blendMode: "normal", transform: defaultRasterTransform() })];
}

/** @emoji 🖱️ Resolves hover payload from a tree row id. */
export function rasterPlayHoverPayloadFromTreeRowId(doc: RasterDocument, rowId: string | null): RasterHoverPayload {
	if (!rowId) return { id: null, kind: null };
	const layerMatch = rowId.match(/^raster-play-layers\.(layer|group|adjustment)\.(.+)$/);
	if (layerMatch) {
		const layer = findRasterLayer(doc, layerMatch[2]!);
		if (layer) {
			return {
				id: layer.id,
				kind: { domain: layer.kind === "group" ? "group" : layer.kind === "adjustment" ? "adjustment" : "layer", kindId: layer.id },
			};
		}
	}
	const maskMatch = rowId.match(/^raster-play-layers\.mask\.(.+)$/);
	if (maskMatch) {
		return { id: maskMatch[1]!, kind: { domain: "mask", kindId: maskMatch[1]! } };
	}
	const blendMatch = rowId.match(/^raster-play-layers\.blend\.(.+)$/);
	if (blendMatch) {
		return { id: null, kind: { domain: "blendMode", kindId: blendMatch[1]! } };
	}
	return { id: null, kind: null };
}

/** @emoji 🌳 Transitive kind hover → all matching layer tree row ids. */
export function rasterPlayLayersTreeHighlightedIdsForKind(doc: RasterDocument, kindHover: RasterKindHover | null): readonly string[] {
	if (!kindHover?.kindId) return [];
	const ids: string[] = [];
	if (kindHover.domain === "blendMode") {
		for (const layer of flattenRasterLayers(doc.layers)) {
			if (layer.blendMode === kindHover.kindId) {
				ids.push(rasterPlayLayersTreeRowId(layer));
			}
		}
		return ids;
	}
	if (kindHover.domain === "mask") {
		return [rasterPlayMaskTreeRowId(kindHover.kindId)];
	}
	for (const layer of flattenRasterLayers(doc.layers)) {
		const domain = layer.kind === "group" ? "group" : layer.kind === "adjustment" ? "adjustment" : "layer";
		if (domain === kindHover.domain && layer.id === kindHover.kindId) {
			ids.push(rasterPlayLayersTreeRowId(layer));
		}
	}
	return ids;
}

export interface RasterViewport {
	readonly width: number;
	readonly height: number;
}

export interface RasterScreenRect {
	readonly x: number;
	readonly y: number;
	readonly width: number;
	readonly height: number;
}

/** @emoji 📐 Maps world coordinates to viewport screen space (matches infinite_cavas camera). */
export function rasterWorldToScreen(
	camera: RasterCamera,
	viewport: RasterViewport,
	world: { readonly x: number; readonly y: number },
): { x: number; y: number } {
	return {
		x: (world.x - camera.x) * camera.zoom + viewport.width / 2,
		y: (world.y - camera.y) * camera.zoom + viewport.height / 2,
	};
}

/** @emoji 📐 Applies layer transform to a local point (translate → rotate → scale). */
export function rasterTransformWorldPoint(
	transform: RasterTransform,
	local: { readonly x: number; readonly y: number },
): { x: number; y: number } {
	const sx = local.x * transform.scaleX;
	const sy = local.y * transform.scaleY;
	const cos = Math.cos(transform.rotation);
	const sin = Math.sin(transform.rotation);
	return {
		x: transform.x + sx * cos - sy * sin,
		y: transform.y + sx * sin + sy * cos,
	};
}

/** @emoji 📐 Axis-aligned screen bounds for a pixel layer. */
export function rasterPixelLayerScreenBounds(
	layer: RasterPixelLayer,
	camera: RasterCamera,
	viewport: RasterViewport,
): RasterScreenRect | null {
	if (!layer.visible) return null;
	const w = layer.width ?? 512;
	const h = layer.height ?? 512;
	const corners = [
		{ x: -w / 2, y: -h / 2 },
		{ x: w / 2, y: -h / 2 },
		{ x: w / 2, y: h / 2 },
		{ x: -w / 2, y: h / 2 },
	].map((local) => rasterWorldToScreen(camera, viewport, rasterTransformWorldPoint(layer.transform, local)));
	const xs = corners.map((point) => point.x);
	const ys = corners.map((point) => point.y);
	const minX = Math.min(...xs);
	const minY = Math.min(...ys);
	const maxX = Math.max(...xs);
	const maxY = Math.max(...ys);
	return { x: minX, y: minY, width: maxX - minX, height: maxY - minY };
}

function rasterScreenRectContains(outer: RasterScreenRect, inner: RasterScreenRect): boolean {
	return (
		inner.x >= outer.x &&
		inner.y >= outer.y &&
		inner.x + inner.width <= outer.x + outer.width &&
		inner.y + inner.height <= outer.y + outer.height
	);
}

function rasterScreenRectIntersects(a: RasterScreenRect, b: RasterScreenRect): boolean {
	return a.x <= b.x + b.width && a.x + a.width >= b.x && a.y <= b.y + b.height && a.y + a.height >= b.y;
}

function rasterScreenRectContainsPoint(rect: RasterScreenRect, point: { readonly x: number; readonly y: number }): boolean {
	return point.x >= rect.x && point.x <= rect.x + rect.width && point.y >= rect.y && point.y <= rect.y + rect.height;
}

/** @emoji 🖱️ Resolves pixel layer ids hit by a screen-space marquee. */
export function resolveRasterMarqueeLayerHits(
	doc: RasterDocument,
	camera: RasterCamera,
	viewport: RasterViewport,
	marquee: RasterScreenRect,
	crossing: boolean,
): string[] {
	const hits: string[] = [];
	for (const layer of flattenRasterLayers(doc.layers)) {
		if (layer.kind !== "pixel") continue;
		const bounds = rasterPixelLayerScreenBounds(layer, camera, viewport);
		if (!bounds) continue;
		if (crossing ? rasterScreenRectIntersects(marquee, bounds) : rasterScreenRectContains(marquee, bounds)) {
			hits.push(layer.id);
		}
	}
	return hits;
}

/** @emoji 🖱️ Topmost pixel layer under a screen point. */
export function resolveRasterLayerAtScreenPoint(
	doc: RasterDocument,
	camera: RasterCamera,
	viewport: RasterViewport,
	point: { readonly x: number; readonly y: number },
): string | null {
	const targets = resolveRasterPickTargetsAtScreenPoint(doc, camera, viewport, point);
	if (targets.length === 0) return null;
	let best = targets[0]!;
	for (const target of targets) {
		if (target.generality > best.generality) best = target;
	}
	return best.id;
}

/** @emoji 🎯 Raster pick-target domain generality (lower = more general). */
export const RASTER_PICK_GENERALITY: Readonly<Record<string, number>> = {
	group: 0,
	adjustment: 1,
	mask: 1,
	pixel: 2,
	layer: 2,
};

export interface RasterPickTarget {
	readonly domain: string;
	readonly id: string;
	readonly generality: number;
	readonly label?: string;
}

function rasterPickTargetForLayer(layer: RasterLayerNode): RasterPickTarget {
	if (layer.kind === "group") {
		return { domain: "group", id: layer.id, generality: RASTER_PICK_GENERALITY.group!, label: layer.name };
	}
	if (layer.kind === "adjustment") {
		return { domain: "adjustment", id: layer.id, generality: RASTER_PICK_GENERALITY.adjustment!, label: layer.name };
	}
	return { domain: "pixel", id: layer.id, generality: RASTER_PICK_GENERALITY.pixel!, label: layer.name };
}

function rasterAncestorGroupTargets(doc: RasterDocument, layerId: string): RasterPickTarget[] {
	const out: RasterPickTarget[] = [];
	const walk = (layers: readonly RasterLayerNode[], ancestors: readonly RasterGroupLayer[]): void => {
		for (const layer of layers) {
			const nextAncestors = layer.kind === "group" ? [...ancestors, layer] : ancestors;
			if (layer.id === layerId) {
				for (const group of nextAncestors) {
					if (group.visible) out.push(rasterPickTargetForLayer(group));
				}
				return;
			}
			if (layer.kind === "group") walk(layer.children, nextAncestors);
		}
	};
	walk(doc.layers, []);
	return out;
}

/** @emoji 🎯 All pick targets under a screen point (groups and pixel layers). */
export function resolveRasterPickTargetsAtScreenPoint(
	doc: RasterDocument,
	camera: RasterCamera,
	viewport: RasterViewport,
	point: { readonly x: number; readonly y: number },
): RasterPickTarget[] {
	const hits: RasterPickTarget[] = [];
	const layers = flattenRasterLayers(doc.layers);
	for (let index = layers.length - 1; index >= 0; index -= 1) {
		const layer = layers[index]!;
		if (!layer.visible) continue;
		if (layer.kind === "group") {
			const bounds = rasterGroupScreenBounds(layer, camera, viewport);
			if (bounds && rasterScreenRectContainsPoint(bounds, point)) hits.push(rasterPickTargetForLayer(layer));
			continue;
		}
		if (layer.kind !== "pixel") continue;
		const bounds = rasterPixelLayerScreenBounds(layer, camera, viewport);
		if (!bounds || !rasterScreenRectContainsPoint(bounds, point)) continue;
		hits.push(rasterPickTargetForLayer(layer));
		for (const groupTarget of rasterAncestorGroupTargets(doc, layer.id)) {
			if (!hits.some((row) => row.id === groupTarget.id)) hits.push(groupTarget);
		}
	}
	return hits;
}

function rasterGroupScreenBounds(
	group: RasterGroupLayer,
	camera: RasterCamera,
	viewport: RasterViewport,
): RasterScreenRect | null {
	let minX = Number.POSITIVE_INFINITY;
	let minY = Number.POSITIVE_INFINITY;
	let maxX = Number.NEGATIVE_INFINITY;
	let maxY = Number.NEGATIVE_INFINITY;
	for (const child of flattenRasterLayers(group.children)) {
		if (child.kind !== "pixel") continue;
		const bounds = rasterPixelLayerScreenBounds(child, camera, viewport);
		if (!bounds) continue;
		minX = Math.min(minX, bounds.x);
		minY = Math.min(minY, bounds.y);
		maxX = Math.max(maxX, bounds.x + bounds.width);
		maxY = Math.max(maxY, bounds.y + bounds.height);
	}
	if (!Number.isFinite(minX)) return null;
	return { x: minX, y: minY, width: maxX - minX, height: maxY - minY };
}

/** @emoji 🌳 Combined hierarchy highlight ids for hover focus. */
export function rasterPlayLayersTreeHighlightedIds(
	doc: RasterDocument,
	hoveredId: string | null,
	kindHover: RasterKindHover | null,
): readonly string[] {
	if (hoveredId) {
		const layer = findRasterLayer(doc, hoveredId);
		if (layer) return [rasterPlayLayersTreeRowId(layer)];
	}
	if (kindHover) return rasterPlayLayersTreeHighlightedIdsForKind(doc, kindHover);
	return [];
}
// #endregion 🌳TreeIds

// #region ✏️EditOps
function mapLayers(
	layers: readonly RasterLayerNode[],
	fn: (layer: RasterLayerNode, parent: RasterGroupLayer | null, index: number) => RasterLayerNode | null,
	parent: RasterGroupLayer | null = null,
): RasterLayerNode[] {
	const out: RasterLayerNode[] = [];
	for (let index = 0; index < layers.length; index += 1) {
		const layer = layers[index]!;
		const mapped = fn(layer, parent, index);
		if (mapped) out.push(mapped);
	}
	return out;
}

function updateLayerInTree(layers: readonly RasterLayerNode[], layerId: string, update: (layer: RasterLayerNode) => RasterLayerNode): RasterLayerNode[] {
	return mapLayers(layers, (layer) => {
		if (layer.id === layerId) return update(layer);
		if (layer.kind === "group") {
			return { ...layer, children: updateLayerInTree(layer.children, layerId, update) };
		}
		return layer;
	});
}

function removeLayerFromTree(layers: readonly RasterLayerNode[], layerId: string): RasterLayerNode[] {
	return mapLayers(layers, (layer) => {
		if (layer.id === layerId) return null;
		if (layer.kind === "group") {
			return { ...layer, children: removeLayerFromTree(layer.children, layerId) };
		}
		return layer;
	});
}

function insertLayer(
	layers: readonly RasterLayerNode[],
	parentId: string | undefined,
	index: number,
	inserted: RasterLayerNode,
): RasterLayerNode[] {
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
		if (layer.kind === "group") {
			return { ...layer, children: insertLayer(layer.children, parentId, index, inserted) };
		}
		return layer;
	});
}

/** @emoji ✏️ Applies a structural or property edit to a raster document. */
export function applyRasterEditOp(doc: RasterDocument, edit: RasterEditOp): RasterDocument {
	switch (edit.op) {
		case "setLayerVisible":
			return { ...doc, layers: updateLayerInTree(doc.layers, edit.layerId, (layer) => ({ ...layer, visible: edit.visible })) };
		case "setLayerOpacity":
			return { ...doc, layers: updateLayerInTree(doc.layers, edit.layerId, (layer) => ({ ...layer, opacity: edit.opacity })) };
		case "setLayerBlendMode":
			return { ...doc, layers: updateLayerInTree(doc.layers, edit.layerId, (layer) => ({ ...layer, blendMode: edit.blendMode })) };
		case "setLayerName":
			return { ...doc, layers: updateLayerInTree(doc.layers, edit.layerId, (layer) => ({ ...layer, name: edit.name })) };
		case "setLayerMask":
			return { ...doc, layers: updateLayerInTree(doc.layers, edit.layerId, (layer) => ({ ...layer, mask: edit.mask })) };
		case "setLayerSize":
			return {
				...doc,
				layers: updateLayerInTree(doc.layers, edit.layerId, (layer) =>
					layer.kind === "pixel"
						? {
								...layer,
								width: typeof edit.width === "number" ? edit.width : layer.width,
								height: typeof edit.height === "number" ? edit.height : layer.height,
							}
						: layer,
				),
			};
		case "setAdjustmentKind":
			return {
				...doc,
				layers: updateLayerInTree(doc.layers, edit.layerId, (layer) =>
					layer.kind === "adjustment" ? { ...layer, adjustmentKind: edit.adjustmentKind } : layer,
				),
			};
		case "appendLayerFilter":
			return {
				...doc,
				layers: updateLayerInTree(doc.layers, edit.layerId, (layer) => {
					if (layer.kind !== "pixel") return layer;
					const filters = [...(layer.filters ?? []), edit.filter];
					return { ...layer, filters };
				}),
			};
		case "addPixelLayer":
		case "addGroupLayer":
		case "addAdjustmentLayer":
			return {
				...doc,
				layers: insertLayer(doc.layers, edit.parentId, edit.index ?? Number.MAX_SAFE_INTEGER, edit.layer),
			};
		case "duplicateLayer": {
			const layer = findRasterLayer(doc, edit.layerId);
			if (!layer) return doc;
			const location = findRasterLayerLocation(doc, edit.layerId);
			const clone = cloneRasterLayerNode(layer);
			return {
				...doc,
				layers: insertLayer(doc.layers, location?.parentId, (location?.index ?? doc.layers.length) + 1, clone),
			};
		}
		case "removeLayer":
			return { ...doc, layers: removeLayerFromTree(doc.layers, edit.layerId) };
		case "reorderLayer": {
			const removed = removeLayerFromTree(doc.layers, edit.layerId);
			const layer = findRasterLayer(doc, edit.layerId);
			if (!layer) return doc;
			return { ...doc, layers: insertLayer(removed, edit.parentId, edit.index, layer) };
		}
		case "setActiveTool":
			return { ...doc, activeTool: edit.tool };
		case "setBrushSize":
			return { ...doc, brushSize: edit.size };
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

	describe("parseRasterDocument", () => {
		it("parses minimal document", () => {
			const doc = parseRasterDocument({
				schema: "raster.document/v1",
				id: "test",
				camera: { x: 0, y: 0, zoom: 1 },
				layers: [{ kind: "pixel", id: "a", name: "A", visible: true, opacity: 1, blendMode: "multiply", transform: { x: 0, y: 0, scaleX: 1, scaleY: 1, rotation: 0 } }],
			});
			expect(doc.layers[0]?.blendMode).toBe("multiply");
		});

		it("rejects wrong schema", () => {
			expect(() => parseRasterDocument({ schema: "other" })).toThrow();
		});

		it("round-trips assets and filters", () => {
			const raw = {
				schema: "raster.document/v1",
				id: "semio",
				camera: { x: 0, y: 0, zoom: 1 },
				assets: { emblem: { mime: "image/png", data: "aGVsbG8=" } },
				layers: [
					{
						kind: "pixel",
						id: "logo",
						name: "Logo",
						visible: true,
						opacity: 1,
						blendMode: "normal",
						imageKey: "emblem",
						filters: [{ kind: "gaussianBlur", radius: 8 }],
						transform: defaultRasterTransform(),
					},
				],
			};
			const doc = parseRasterDocument(raw);
			const restored = parseRasterDocument(JSON.parse(rasterDocumentToExportJson(doc)));
			expect(restored.assets?.emblem?.data).toBe("aGVsbG8=");
			expect(restored.layers[0]?.kind === "pixel" && restored.layers[0].filters?.[0]?.kind).toBe("gaussianBlur");
		});
	});

	describe("applyRasterEditOp", () => {
		it("toggles visibility", () => {
			const doc = defaultRasterDocument();
			const layerId = doc.layers[0]!.id;
			const next = applyRasterEditOp(doc, { op: "setLayerVisible", layerId, visible: false });
			expect(next.layers[0]?.visible).toBe(false);
		});

		it("reorders layers", () => {
			const doc = parseRasterDocument({
				schema: "raster.document/v1",
				id: "t",
				camera: { x: 0, y: 0, zoom: 1 },
				layers: [
					{ kind: "pixel", id: "a", name: "A", visible: true, opacity: 1, blendMode: "normal", transform: defaultRasterTransform() },
					{ kind: "pixel", id: "b", name: "B", visible: true, opacity: 1, blendMode: "normal", transform: defaultRasterTransform() },
				],
			});
			const next = applyRasterEditOp(doc, { op: "reorderLayer", layerId: "a", index: 1 });
			expect(next.layers.map((layer) => layer.id)).toEqual(["b", "a"]);
		});

		it("duplicates a layer", () => {
			const doc = defaultRasterDocument();
			const layerId = doc.layers[0]!.id;
			const next = applyRasterEditOp(doc, { op: "duplicateLayer", layerId });
			expect(next.layers).toHaveLength(2);
			expect(next.layers[1]?.name).toContain("copy");
		});
	});

	describe("resolveRasterMarqueeLayerHits", () => {
		it("selects layers inside a full marquee", () => {
			const doc = parseRasterDocument({
				schema: "raster.document/v1",
				id: "test",
				camera: { x: 0, y: 0, zoom: 1 },
				layers: [
					{
						kind: "pixel",
						id: "a",
						name: "A",
						visible: true,
						opacity: 1,
						blendMode: "normal",
						width: 100,
						height: 100,
						transform: { x: 0, y: 0, scaleX: 1, scaleY: 1, rotation: 0 },
					},
				],
			});
			const hits = resolveRasterMarqueeLayerHits(doc, doc.camera, { width: 800, height: 600 }, { x: 350, y: 250, width: 100, height: 100 }, false);
			expect(hits).toEqual(["a"]);
		});
	});

	describe("rasterPlayLayersTreeHighlightedIdsForKind", () => {
		it("highlights all layers sharing a blend mode", () => {
			const doc = parseRasterDocument({
				schema: "raster.document/v1",
				id: "t",
				camera: { x: 0, y: 0, zoom: 1 },
				layers: [
					{ kind: "pixel", id: "a", name: "A", visible: true, opacity: 1, blendMode: "screen", transform: defaultRasterTransform() },
					{ kind: "pixel", id: "b", name: "B", visible: true, opacity: 1, blendMode: "screen", transform: defaultRasterTransform() },
					{ kind: "pixel", id: "c", name: "C", visible: true, opacity: 1, blendMode: "normal", transform: defaultRasterTransform() },
				],
			});
			const ids = rasterPlayLayersTreeHighlightedIdsForKind(doc, { domain: "blendMode", kindId: "screen" });
			expect(ids).toHaveLength(2);
		});
	});
}
// #endregion 🧪Tests
