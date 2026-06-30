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

export interface RasterPixelLayer extends RasterLayerBase {
	readonly kind: "pixel";
	readonly imageKey?: string;
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
	| { readonly op: "addPixelLayer"; readonly parentId?: string; readonly layer: RasterPixelLayer }
	| { readonly op: "addGroupLayer"; readonly parentId?: string; readonly layer: RasterGroupLayer }
	| { readonly op: "addAdjustmentLayer"; readonly parentId?: string; readonly layer: RasterAdjustmentLayer }
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
			},
		};
	}
	return {
		...base,
		kind: "pixel",
		imageKey: typeof raw.imageKey === "string" ? raw.imageKey : undefined,
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
		case "addPixelLayer":
		case "addGroupLayer":
		case "addAdjustmentLayer":
			return {
				...doc,
				layers: insertLayer(doc.layers, edit.parentId, Number.MAX_SAFE_INTEGER, edit.layer),
			};
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
	});

	describe("applyRasterEditOp", () => {
		it("toggles visibility", () => {
			const doc = defaultRasterDocument();
			const layerId = doc.layers[0]!.id;
			const next = applyRasterEditOp(doc, { op: "setLayerVisible", layerId, visible: false });
			expect(next.layers[0]?.visible).toBe(false);
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
