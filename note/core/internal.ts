// #region 🧲Header
/// <reference types="vitest/importMeta" />
/** @emoji 📝 `@semio-tech/note-core` — infinite canvas document model, edit ops, hover/selection mapping. */
// #endregion 🧲Header

import {
	createDocumentVcsEnvelope,
	type DocumentVcsEnvelope,
	materializeDocumentProjection,
} from "@semio-tech/vcs-core/internal";

//#region 📐Types
export type Vec2 = readonly [number, number];

export const NOTE_BLOCK_KINDS = ["text", "image", "table", "math", "ink", "group"] as const;
export type NoteBlockKind = (typeof NOTE_BLOCK_KINDS)[number];

export const NOTE_TOOL_IDS = [
	"selectDirect",
	"selectMarquee",
	"pan",
	"text",
	"image",
	"table",
	"math",
	"pencil",
	"eraserStroke",
	"eraserPoint",
] as const;
export type NoteToolId = (typeof NOTE_TOOL_IDS)[number];

export interface NoteCamera {
	readonly x: number;
	readonly y: number;
	readonly zoom: number;
}

export interface NoteBlockBase {
	readonly id: string;
	readonly name: string;
	readonly x: number;
	readonly y: number;
	readonly width: number;
	readonly height: number;
	readonly rotation?: number;
	readonly visible: boolean;
	readonly locked: boolean;
}

export interface NoteTextRun {
	readonly text: string;
	readonly bold?: boolean;
	readonly italic?: boolean;
	readonly underline?: boolean;
	readonly link?: string;
}

export interface NoteTextParagraph {
	readonly runs: readonly NoteTextRun[];
}

export interface NoteTextBlock extends NoteBlockBase {
	readonly kind: "text";
	readonly paragraphs: readonly NoteTextParagraph[];
	readonly fontSize: number;
	readonly fontWeight: "normal" | "bold";
	readonly align: "left" | "center" | "right";
}

export interface NoteImageAsset {
	readonly mime: string;
	readonly data: string;
	readonly width?: number;
	readonly height?: number;
}

export interface NoteImageBlock extends NoteBlockBase {
	readonly kind: "image";
	readonly imageKey: string;
}

export interface NoteTableCell {
	readonly content: string;
}

export interface NoteTableBlock extends NoteBlockBase {
	readonly kind: "table";
	readonly columns: readonly string[];
	readonly rows: readonly (readonly NoteTableCell[])[];
}

export interface NoteMathBlock extends NoteBlockBase {
	readonly kind: "math";
	readonly tex: string;
	readonly displayMode: boolean;
}

export interface NoteInkBlock extends NoteBlockBase {
	readonly kind: "ink";
	readonly points: readonly Vec2[];
	readonly strokeWidth: number;
	readonly color: readonly [number, number, number, number];
}

export interface NoteGroupBlock extends NoteBlockBase {
	readonly kind: "group";
	readonly children: readonly NoteBlockNode[];
}

export type NoteBlockNode = NoteTextBlock | NoteImageBlock | NoteTableBlock | NoteMathBlock | NoteInkBlock | NoteGroupBlock;

export interface NoteDocument {
	readonly schema: "note.document";
	readonly id: string;
	readonly title?: string;
	readonly camera: NoteCamera;
	readonly blocks: readonly NoteBlockNode[];
	readonly assets?: Readonly<Record<string, NoteImageAsset>>;
	readonly activeTool?: NoteToolId;
	readonly gridVisible?: boolean;
	readonly snapEnabled?: boolean;
	readonly pencilWidth?: number;
	readonly eraserRadius?: number;
}

export type NoteKindHoverDomain = NoteBlockKind | "block";

export interface NoteKindHover {
	readonly domain: NoteKindHoverDomain;
	readonly kindId: string;
}

export interface NoteHoverPayload {
	readonly id: string | null;
	readonly kind: NoteKindHover | null;
}

export type NoteEditOp =
	| { readonly op: "setDocument"; readonly document: NoteDocument }
	| { readonly op: "setCamera"; readonly camera: NoteCamera }
	| { readonly op: "setActiveTool"; readonly tool: NoteToolId }
	| { readonly op: "setGridVisible"; readonly visible: boolean }
	| { readonly op: "setSnapEnabled"; readonly enabled: boolean }
	| { readonly op: "setPencilWidth"; readonly width: number }
	| { readonly op: "setEraserRadius"; readonly radius: number }
	| { readonly op: "addBlock"; readonly parentId?: string; readonly index?: number; readonly block: NoteBlockNode }
	| { readonly op: "updateBlock"; readonly blockId: string; readonly block: NoteBlockNode }
	| { readonly op: "removeBlock"; readonly blockId: string }
	| { readonly op: "reorderBlock"; readonly blockId: string; readonly parentId?: string; readonly index: number }
	| { readonly op: "duplicateBlock"; readonly blockId: string }
	| { readonly op: "setBlockName"; readonly blockId: string; readonly name: string }
	| { readonly op: "setBlockVisible"; readonly blockId: string; readonly visible: boolean }
	| { readonly op: "setBlockLocked"; readonly blockId: string; readonly locked: boolean };
//#endregion 📐Types

//#region 🔧Helpers
let noteIdCounter = 0;

export function createNoteId(prefix = "block"): string {
	noteIdCounter += 1;
	return `${prefix}-${noteIdCounter}`;
}

export function defaultNoteDocument(id = "empty", title?: string): NoteDocument {
	return {
		schema: "note.document",
		id,
		title,
		camera: { x: 0, y: 0, zoom: 1 },
		blocks: [],
		activeTool: "selectDirect",
		gridVisible: true,
		snapEnabled: false,
		pencilWidth: 3,
		eraserRadius: 12,
	};
}

export function encodeNotePointerFocusKey(kind: string, id: string): string {
	return `note:${kind}:${id}`;
}

export function decodeNotePointerFocusKey(key: string): { readonly kind: string; readonly id: string } | null {
	if (!key.startsWith("note:")) return null;
	const rest = key.slice("note:".length);
	const colon = rest.indexOf(":");
	if (colon < 0) return null;
	return { kind: rest.slice(0, colon), id: rest.slice(colon + 1) };
}

export function noteHoverPayloadFromPointerFocusKey(key: string | null): NoteHoverPayload {
	if (!key) return { id: null, kind: null };
	const decoded = decodeNotePointerFocusKey(key);
	if (!decoded) return { id: key, kind: null };
	return { id: decoded.id, kind: { domain: decoded.kind as NoteKindHoverDomain, kindId: decoded.kind } };
}

export function noteKindHoverForBlock(block: NoteBlockNode | null): NoteKindHover | null {
	if (!block) return null;
	return { domain: block.kind, kindId: block.kind };
}

export interface NoteBlockLocation {
	readonly parentId?: string;
	readonly index: number;
}

export function noteTextParagraphsFromPlainText(text: string): readonly NoteTextParagraph[] {
	const lines = text.split(/\n/);
	return lines.map((line) => ({ runs: [{ text: line }] }));
}

export function noteTextPlainText(paragraphs: readonly NoteTextParagraph[]): string {
	return paragraphs.map((paragraph) => paragraph.runs.map((run) => run.text).join("")).join("\n");
}

export function noteImageAssetDataUrl(asset: NoteImageAsset): string {
	return asset.data.startsWith("data:") ? asset.data : `data:${asset.mime};base64,${asset.data}`;
}

export interface NoteBounds {
	readonly x: number;
	readonly y: number;
	readonly width: number;
	readonly height: number;
}

export type NoteResizeHandle = "nw" | "n" | "ne" | "e" | "se" | "s" | "sw" | "w";

export function noteSelectionBounds(blocks: readonly NoteBlockNode[], ids: readonly string[]): NoteBounds | null {
	const idSet = new Set(ids);
	const selected = flattenNoteBlocks(blocks).filter((block) => idSet.has(block.id));
	if (!selected.length) return null;
	let minX = Infinity;
	let minY = Infinity;
	let maxX = -Infinity;
	let maxY = -Infinity;
	for (const block of selected) {
		const bounds = noteBlockBounds(block);
		minX = Math.min(minX, bounds.x);
		minY = Math.min(minY, bounds.y);
		maxX = Math.max(maxX, bounds.x + bounds.width);
		maxY = Math.max(maxY, bounds.y + bounds.height);
	}
	return { x: minX, y: minY, width: Math.max(1, maxX - minX), height: Math.max(1, maxY - minY) };
}

function scaleValue(value: number, fromMin: number, fromSize: number, toMin: number, toSize: number): number {
	if (fromSize <= 0) return toMin;
	return toMin + ((value - fromMin) / fromSize) * toSize;
}

export function noteScaleBlockWithinGroup(block: NoteBlockNode, fromBounds: NoteBounds, toBounds: NoteBounds): NoteBlockNode {
	const blockBounds = noteBlockBounds(block);
	const nextX = scaleValue(blockBounds.x, fromBounds.x, fromBounds.width, toBounds.x, toBounds.width);
	const nextY = scaleValue(blockBounds.y, fromBounds.y, fromBounds.height, toBounds.y, toBounds.height);
	const nextWidth = Math.max(8, scaleValue(blockBounds.x + blockBounds.width, fromBounds.x, fromBounds.width, toBounds.x, toBounds.width) - nextX);
	const nextHeight = Math.max(8, scaleValue(blockBounds.y + blockBounds.height, fromBounds.y, fromBounds.height, toBounds.y, toBounds.height) - nextY);
	if (block.kind === "ink") {
		const scaleX = fromBounds.width > 0 ? toBounds.width / fromBounds.width : 1;
		const scaleY = fromBounds.height > 0 ? toBounds.height / fromBounds.height : 1;
		const points = block.points.map(([px, py]) => [px * scaleX, py * scaleY] as Vec2);
		return { ...block, x: nextX, y: nextY, width: nextWidth, height: nextHeight, points };
	}
	if (block.kind === "group") {
		return {
			...block,
			x: nextX,
			y: nextY,
			width: nextWidth,
			height: nextHeight,
			children: block.children.map((child) => noteScaleBlockWithinGroup(child, fromBounds, toBounds)),
		};
	}
	return { ...block, x: nextX, y: nextY, width: nextWidth, height: nextHeight };
}

export function noteResizeBounds(fromBounds: NoteBounds, handle: NoteResizeHandle, dx: number, dy: number, minSize = 8): NoteBounds {
	let { x, y, width, height } = fromBounds;
	if (handle.includes("e")) width = Math.max(minSize, width + dx);
	if (handle.includes("w")) {
		const nextWidth = Math.max(minSize, width - dx);
		x += width - nextWidth;
		width = nextWidth;
	}
	if (handle.includes("s")) height = Math.max(minSize, height + dy);
	if (handle.includes("n")) {
		const nextHeight = Math.max(minSize, height - dy);
		y += height - nextHeight;
		height = nextHeight;
	}
	return { x, y, width, height };
}

function pointToSegmentDistance(px: number, py: number, x1: number, y1: number, x2: number, y2: number): number {
	const dx = x2 - x1;
	const dy = y2 - y1;
	if (dx === 0 && dy === 0) return Math.hypot(px - x1, py - y1);
	const t = Math.max(0, Math.min(1, ((px - x1) * dx + (py - y1) * dy) / (dx * dx + dy * dy)));
	return Math.hypot(px - (x1 + t * dx), py - (y1 + t * dy));
}

function inkWorldPoints(block: NoteInkBlock): Vec2[] {
	return block.points.map(([px, py]) => [block.x + px, block.y + py] as Vec2);
}

function inkHitsPoint(block: NoteInkBlock, x: number, y: number, threshold: number): boolean {
	const points = inkWorldPoints(block);
	if (points.length < 2) {
		if (!points[0]) return false;
		return Math.hypot(x - points[0][0], y - points[0][1]) <= threshold;
	}
	for (let index = 1; index < points.length; index += 1) {
		const prev = points[index - 1]!;
		const next = points[index]!;
		if (pointToSegmentDistance(x, y, prev[0], prev[1], next[0], next[1]) <= threshold + block.strokeWidth / 2) return true;
	}
	return false;
}

export function noteEraseInkStrokeAtPoint(doc: NoteDocument, x: number, y: number, threshold = 8): NoteDocument {
	const hits = flattenNoteBlocks(doc.blocks).filter((block): block is NoteInkBlock => block.kind === "ink" && inkHitsPoint(block, x, y, threshold));
	if (!hits.length) return doc;
	let next = doc;
	for (const block of hits) next = applyNoteEditOp(next, { op: "removeBlock", blockId: block.id });
	return next;
}

function noteEraseInkPointsInBlock(block: NoteInkBlock, x: number, y: number, radius: number): NoteInkBlock[] {
	const keptIndices: number[] = [];
	for (let index = 0; index < block.points.length; index += 1) {
		const point = block.points[index]!;
		if (Math.hypot(block.x + point[0] - x, block.y + point[1] - y) > radius) keptIndices.push(index);
	}
	if (keptIndices.length === block.points.length) return [block];
	if (!keptIndices.length) return [];
	const runs: Vec2[][] = [];
	let current: Vec2[] = [block.points[keptIndices[0]!]!];
	for (let index = 1; index < keptIndices.length; index += 1) {
		if (keptIndices[index]! - keptIndices[index - 1]! > 1) {
			if (current.length >= 2) runs.push(current);
			current = [block.points[keptIndices[index]!]!];
		} else {
			current.push(block.points[keptIndices[index]!]!);
		}
	}
	if (current.length >= 2) runs.push(current);
	return runs.map((points, index) => ({
		...block,
		id: index === 0 ? block.id : createNoteId("ink"),
		name: index === 0 ? block.name : `${block.name} fragment`,
		points,
	}));
}

export function noteEraseInkPointsNearPoint(doc: NoteDocument, x: number, y: number, radius: number): NoteDocument {
	let next = doc;
	const inkBlocks = flattenNoteBlocks(doc.blocks).filter((block): block is NoteInkBlock => block.kind === "ink");
	for (const block of inkBlocks) {
		const fragments = noteEraseInkPointsInBlock(block, x, y, radius);
		if (fragments.length === 1 && fragments[0] === block) continue;
		next = applyNoteEditOp(next, { op: "removeBlock", blockId: block.id });
		for (const fragment of fragments) next = applyNoteEditOp(next, { op: "addBlock", block: fragment });
	}
	return next;
}

export interface NoteClipboardPayload {
	readonly schema: "note.clipboard";
	readonly blocks: readonly NoteBlockNode[];
}

export function noteClipboardPayload(blocks: readonly NoteBlockNode[]): string {
	const payload: NoteClipboardPayload = { schema: "note.clipboard", blocks: [...blocks] };
	return JSON.stringify(payload);
}

export function noteBlocksFromClipboardPayload(json: string): readonly NoteBlockNode[] | null {
	try {
		const parsed = JSON.parse(json) as NoteClipboardPayload;
		if (parsed.schema !== "note.clipboard" || !Array.isArray(parsed.blocks)) return null;
		return parsed.blocks;
	} catch {
		return null;
	}
}

export function noteCloneBlocksWithOffset(blocks: readonly NoteBlockNode[], dx: number, dy: number): NoteBlockNode[] {
	return blocks.map((block) => {
		const clone = cloneNoteBlock(block);
		return { ...clone, x: clone.x + dx, y: clone.y + dy };
	});
}

export function noteTableCellAtPoint(block: NoteTableBlock, localX: number, localY: number): { readonly row: number; readonly col: number } | null {
	const rowCount = block.rows.length + 1;
	const colCount = block.columns.length;
	if (rowCount <= 0 || colCount <= 0) return null;
	const rowHeight = block.height / rowCount;
	const colWidth = block.width / colCount;
	const row = Math.floor(localY / rowHeight) - 1;
	const col = Math.floor(localX / colWidth);
	if (row < 0 || row >= block.rows.length || col < 0 || col >= colCount) return null;
	return { row, col };
}

export function createNoteImageAssetFromDataUrl(dataUrl: string, mime?: string): NoteImageAsset {
	const match = /^data:([^;]+);base64,(.+)$/.exec(dataUrl);
	if (match) return { mime: match[1]!, data: dataUrl };
	return { mime: mime ?? "image/png", data: dataUrl };
}

export function createNoteImageAssetKey(): string {
	return `asset-${createNoteId("image")}`;
}

export function createNoteTextBlock(name = "Text", x = 0, y = 0, seedText = ""): NoteTextBlock {
	return {
		kind: "text",
		id: createNoteId("text"),
		name,
		x,
		y,
		width: 280,
		height: 120,
		visible: true,
		locked: false,
		paragraphs: noteTextParagraphsFromPlainText(seedText),
		fontSize: 18,
		fontWeight: "normal",
		align: "left",
	};
}

export function createNoteImageBlock(name = "Image", imageKey = "placeholder", x = 0, y = 0): NoteImageBlock {
	return {
		kind: "image",
		id: createNoteId("image"),
		name,
		x,
		y,
		width: 240,
		height: 160,
		visible: true,
		locked: false,
		imageKey,
	};
}

export function createNoteTableBlock(name = "Table", x = 0, y = 0): NoteTableBlock {
	return {
		kind: "table",
		id: createNoteId("table"),
		name,
		x,
		y,
		width: 320,
		height: 160,
		visible: true,
		locked: false,
		columns: ["A", "B", "C"],
		rows: [
			[{ content: "" }, { content: "" }, { content: "" }],
			[{ content: "" }, { content: "" }, { content: "" }],
		],
	};
}

export function createNoteMathBlock(name = "Math", tex = "E = mc^2", x = 0, y = 0): NoteMathBlock {
	return {
		kind: "math",
		id: createNoteId("math"),
		name,
		x,
		y,
		width: 200,
		height: 80,
		visible: true,
		locked: false,
		tex,
		displayMode: true,
	};
}

export function createNoteInkBlock(name = "Ink", x = 0, y = 0, strokeWidth = 3): NoteInkBlock {
	return {
		kind: "ink",
		id: createNoteId("ink"),
		name,
		x,
		y,
		width: 1,
		height: 1,
		visible: true,
		locked: false,
		points: [],
		strokeWidth,
		color: [0, 0, 0, 1],
	};
}

export function createNoteGroupBlock(name = "Group", children: readonly NoteBlockNode[] = []): NoteGroupBlock {
	return {
		kind: "group",
		id: createNoteId("group"),
		name,
		x: 0,
		y: 0,
		width: 1,
		height: 1,
		visible: true,
		locked: false,
		children: [...children],
	};
}

export function createNoteBlockByKind(kind: NoteBlockKind, x = 0, y = 0): NoteBlockNode {
	if (kind === "text") return createNoteTextBlock("Text", x, y);
	if (kind === "image") return createNoteImageBlock("Image", "placeholder", x, y);
	if (kind === "table") return createNoteTableBlock("Table", x, y);
	if (kind === "math") return createNoteMathBlock("Math", "E = mc^2", x, y);
	if (kind === "ink") return createNoteInkBlock("Ink", x, y);
	return createNoteGroupBlock("Group");
}

export function findNoteBlock(doc: NoteDocument, blockId: string): NoteBlockNode | null {
	for (const block of doc.blocks) {
		const found = findNoteBlockInNode(block, blockId);
		if (found) return found;
	}
	return null;
}

function findNoteBlockInNode(node: NoteBlockNode, blockId: string): NoteBlockNode | null {
	if (node.id === blockId) return node;
	if (node.kind === "group") {
		for (const child of node.children) {
			const found = findNoteBlockInNode(child, blockId);
			if (found) return found;
		}
	}
	return null;
}

export function findNoteBlockLocation(doc: NoteDocument, blockId: string): NoteBlockLocation | null {
	for (let index = 0; index < doc.blocks.length; index += 1) {
		const block = doc.blocks[index]!;
		if (block.id === blockId) return { index };
		if (block.kind === "group") {
			const nested = findNoteBlockLocationInGroup(block, blockId);
			if (nested) return { parentId: block.id, index: nested.index };
		}
	}
	return null;
}

function findNoteBlockLocationInGroup(group: NoteGroupBlock, blockId: string): { readonly index: number } | null {
	for (let index = 0; index < group.children.length; index += 1) {
		const child = group.children[index]!;
		if (child.id === blockId) return { index };
		if (child.kind === "group") {
			const nested = findNoteBlockLocationInGroup(child, blockId);
			if (nested) return nested;
		}
	}
	return null;
}

export function flattenNoteBlocks(blocks: readonly NoteBlockNode[]): NoteBlockNode[] {
	const out: NoteBlockNode[] = [];
	for (const block of blocks) {
		out.push(block);
		if (block.kind === "group") out.push(...flattenNoteBlocks(block.children));
	}
	return out;
}

function insertBlock(blocks: readonly NoteBlockNode[], parentId: string | undefined, index: number, block: NoteBlockNode): NoteBlockNode[] {
	if (!parentId) {
		const next = [...blocks];
		next.splice(index, 0, block);
		return next;
	}
	return blocks.map((node) => {
		if (node.kind !== "group" || node.id !== parentId) return node;
		const children = [...node.children];
		children.splice(index, 0, block);
		return { ...node, children };
	});
}

function removeBlockFromTree(blocks: readonly NoteBlockNode[], blockId: string): NoteBlockNode[] {
	return blocks
		.filter((block) => block.id !== blockId)
		.map((block) => (block.kind === "group" ? { ...block, children: removeBlockFromTree(block.children, blockId) } : block));
}

function updateBlockInTree(blocks: readonly NoteBlockNode[], blockId: string, nextBlock: NoteBlockNode): NoteBlockNode[] {
	return blocks.map((block) => {
		if (block.id === blockId) return nextBlock;
		if (block.kind === "group") return { ...block, children: updateBlockInTree(block.children, blockId, nextBlock) };
		return block;
	});
}

function mutateBlockInTree(blocks: readonly NoteBlockNode[], blockId: string, mutate: (block: NoteBlockNode) => NoteBlockNode): NoteBlockNode[] {
	return blocks.map((block) => {
		if (block.id === blockId) return mutate(block);
		if (block.kind === "group") return { ...block, children: mutateBlockInTree(block.children, blockId, mutate) };
		return block;
	});
}

export function cloneNoteBlock(block: NoteBlockNode, nameSuffix = " copy"): NoteBlockNode {
	const id = createNoteId(block.kind);
	if (block.kind === "group") {
		return { ...block, id, name: `${block.name}${nameSuffix}`, children: block.children.map((child) => cloneNoteBlock(child, "")) };
	}
	return { ...block, id, name: `${block.name}${nameSuffix}` };
}

export function noteBlockBounds(block: NoteBlockNode): { readonly x: number; readonly y: number; readonly width: number; readonly height: number } {
	if (block.kind === "ink" && block.points.length > 0) {
		let minX = block.points[0]![0];
		let minY = block.points[0]![1];
		let maxX = minX;
		let maxY = minY;
		for (const point of block.points) {
			minX = Math.min(minX, point[0]);
			minY = Math.min(minY, point[1]);
			maxX = Math.max(maxX, point[0]);
			maxY = Math.max(maxY, point[1]);
		}
		return { x: block.x + minX, y: block.y + minY, width: Math.max(1, maxX - minX), height: Math.max(1, maxY - minY) };
	}
	return { x: block.x, y: block.y, width: block.width, height: block.height };
}

export function noteBlocksIntersectingRect(blocks: readonly NoteBlockNode[], rect: { readonly x: number; readonly y: number; readonly width: number; readonly height: number }): string[] {
	const hits: string[] = [];
	for (const block of flattenNoteBlocks(blocks)) {
		const bounds = noteBlockBounds(block);
		const intersects =
			bounds.x < rect.x + rect.width &&
			bounds.x + bounds.width > rect.x &&
			bounds.y < rect.y + rect.height &&
			bounds.y + bounds.height > rect.y;
		if (intersects) hits.push(block.id);
	}
	return hits;
}

export function noteBlocksAtPoint(blocks: readonly NoteBlockNode[], x: number, y: number): NoteBlockNode[] {
	const hits: NoteBlockNode[] = [];
	for (const block of [...flattenNoteBlocks(blocks)].reverse()) {
		const bounds = noteBlockBounds(block);
		if (x >= bounds.x && x <= bounds.x + bounds.width && y >= bounds.y && y <= bounds.y + bounds.height) hits.push(block);
	}
	return hits;
}

export function noteDocumentToJson(doc: NoteDocument): string {
	return JSON.stringify(doc, null, 2);
}

export function parseNoteDocument(raw: unknown): NoteDocument {
	if (!raw || typeof raw !== "object" || Array.isArray(raw)) throw new Error("note document must be an object");
	const record = raw as NoteDocument;
	if (record.schema !== "note.document") throw new Error(`unsupported note schema: ${String((raw as { schema?: string }).schema)}`);
	if (!Array.isArray(record.blocks)) throw new Error("note document blocks must be an array");
	return record;
}

export function noteDocumentFromJson(json: string): NoteDocument {
	return parseNoteDocument(JSON.parse(json));
}

export function notePlayBlocksTreeRowId(block: NoteBlockNode): string {
	return `note-play-block:${block.id}`;
}

export function notePlayBlockIdFromTreeRowId(rowId: string): string | null {
	if (!rowId.startsWith("note-play-block:")) return null;
	return rowId.slice("note-play-block:".length);
}

export function notePlayBlocksTreeHighlightedIds(doc: NoteDocument, hoveredId: string | null, kindHover: NoteKindHover | null): readonly string[] {
	if (!hoveredId) {
		if (!kindHover) return [];
		return flattenNoteBlocks(doc.blocks)
			.filter((block) => block.kind === kindHover.kindId || kindHover.domain === block.kind)
			.map((block) => notePlayBlocksTreeRowId(block));
	}
	const block = findNoteBlock(doc, hoveredId);
	return block ? [notePlayBlocksTreeRowId(block)] : [];
}
//#endregion 🔧Helpers

//#region ✏️EditOps
export function applyNoteEditOp(doc: NoteDocument, edit: NoteEditOp): NoteDocument {
	switch (edit.op) {
		case "setDocument":
			return edit.document;
		case "setCamera":
			return { ...doc, camera: edit.camera };
		case "setActiveTool":
			return { ...doc, activeTool: edit.tool };
		case "setGridVisible":
			return { ...doc, gridVisible: edit.visible };
		case "setSnapEnabled":
			return { ...doc, snapEnabled: edit.enabled };
		case "setPencilWidth":
			return { ...doc, pencilWidth: edit.width };
		case "setEraserRadius":
			return { ...doc, eraserRadius: edit.radius };
		case "addBlock":
			return { ...doc, blocks: insertBlock(doc.blocks, edit.parentId, edit.index ?? doc.blocks.length, edit.block) };
		case "updateBlock":
			return { ...doc, blocks: updateBlockInTree(doc.blocks, edit.blockId, edit.block) };
		case "removeBlock":
			return { ...doc, blocks: removeBlockFromTree(doc.blocks, edit.blockId) };
		case "reorderBlock": {
			const block = findNoteBlock(doc, edit.blockId);
			if (!block) return doc;
			const without = removeBlockFromTree(doc.blocks, edit.blockId);
			return { ...doc, blocks: insertBlock(without, edit.parentId, edit.index, block) };
		}
		case "duplicateBlock": {
			const block = findNoteBlock(doc, edit.blockId);
			if (!block) return doc;
			const location = findNoteBlockLocation(doc, edit.blockId);
			if (!location) return doc;
			const clone = cloneNoteBlock(block);
			return { ...doc, blocks: insertBlock(doc.blocks, location.parentId, location.index + 1, clone) };
		}
		case "setBlockName":
			return { ...doc, blocks: mutateBlockInTree(doc.blocks, edit.blockId, (block) => ({ ...block, name: edit.name })) };
		case "setBlockVisible":
			return { ...doc, blocks: mutateBlockInTree(doc.blocks, edit.blockId, (block) => ({ ...block, visible: edit.visible })) };
		case "setBlockLocked":
			return { ...doc, blocks: mutateBlockInTree(doc.blocks, edit.blockId, (block) => ({ ...block, locked: edit.locked })) };
		default:
			return doc;
	}
}

export function backwardsNoteEditOp(projection: NoteDocument, operation: NoteEditOp): readonly NoteEditOp[] {
	switch (operation.op) {
		case "setDocument":
			return [{ op: "setDocument", document: projection }];
		case "setCamera":
			return [{ op: "setCamera", camera: projection.camera }];
		case "setActiveTool":
			return [{ op: "setActiveTool", tool: projection.activeTool ?? "selectDirect" }];
		case "setGridVisible":
			return [{ op: "setGridVisible", visible: projection.gridVisible ?? true }];
		case "setSnapEnabled":
			return [{ op: "setSnapEnabled", enabled: projection.snapEnabled ?? false }];
		case "setPencilWidth":
			return [{ op: "setPencilWidth", width: projection.pencilWidth ?? 3 }];
		case "setEraserRadius":
			return [{ op: "setEraserRadius", radius: projection.eraserRadius ?? 12 }];
		default:
			return [{ op: "setDocument", document: projection }];
	}
}

export function diffNoteEditOp(_projection: NoteDocument, operation: NoteEditOp): unknown {
	return operation;
}
//#endregion ✏️EditOps

//#region 🔖DocumentVcs
export type NoteDocumentVcsEnvelope = DocumentVcsEnvelope<NoteDocument, NoteEditOp>;

export function createNoteDocumentVcsEnvelope(id: string, projection: NoteDocument = defaultNoteDocument(id)): NoteDocumentVcsEnvelope {
	return createDocumentVcsEnvelope("note.document", id, projection);
}

export function materializeNoteDocument(envelope: NoteDocumentVcsEnvelope, appliedChangeIds: readonly string[] = []): NoteDocument {
	return materializeDocumentProjection(envelope, appliedChangeIds, applyNoteEditOp);
}

/** @emoji 🧩 S app VCS handler factory for note documents. */
export function createNoteAppVcsHandler() {
	return {
		format: "note.document",
		createEnvelope: (id: string) => createNoteDocumentVcsEnvelope(id),
		applyOp: applyNoteEditOp,
		serializeEnvelope: (envelope: NoteDocumentVcsEnvelope) => JSON.stringify(envelope),
		deserializeEnvelope: (json: string) => JSON.parse(json) as NoteDocumentVcsEnvelope,
		materializeProjection: (source: { readonly vcsJson?: string; readonly inline?: string }) => {
			if (source.vcsJson) {
				const envelope = JSON.parse(source.vcsJson) as NoteDocumentVcsEnvelope;
				return materializeNoteDocument(envelope, envelope.vcs.edits.map((edit) => edit.id));
			}
			if (source.inline) return noteDocumentFromJson(source.inline);
			return defaultNoteDocument("note");
		},
	};
}
//#endregion 🔖DocumentVcs

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("note document", () => {
		it("creates blocks and applies edit ops", () => {
			let doc = defaultNoteDocument("test");
			const text = createNoteTextBlock();
			doc = applyNoteEditOp(doc, { op: "addBlock", block: text });
			expect(findNoteBlock(doc, text.id)?.kind).toBe("text");
			doc = applyNoteEditOp(doc, { op: "setBlockName", blockId: text.id, name: "Renamed" });
			expect(findNoteBlock(doc, text.id)?.name).toBe("Renamed");
		});

		it("encodes pointer focus keys", () => {
			const key = encodeNotePointerFocusKey("text", "abc");
			expect(noteHoverPayloadFromPointerFocusKey(key).id).toBe("abc");
		});

		it("round-trips plain text through paragraphs", () => {
			const paragraphs = noteTextParagraphsFromPlainText("hello\nworld");
			expect(noteTextPlainText(paragraphs)).toBe("hello\nworld");
		});

		it("scales selection bounds", () => {
			const text = createNoteTextBlock("A", 0, 0);
			const image = createNoteImageBlock("B", "k", 100, 0);
			const from = noteSelectionBounds([text, image], [text.id, image.id]);
			expect(from?.width).toBe(340);
			const scaled = noteScaleBlockWithinGroup(text, from!, { x: 0, y: 0, width: 680, height: 160 });
			expect(scaled.x).toBe(0);
			expect(scaled.width).toBe(560);
		});

		it("erases ink strokes and points", () => {
			let doc = defaultNoteDocument("ink");
			const ink = createNoteInkBlock("Ink", 0, 0);
			const withPoints = { ...ink, points: [[0, 0], [40, 0], [80, 0]] as const };
			doc = applyNoteEditOp(doc, { op: "addBlock", block: withPoints });
			doc = noteEraseInkStrokeAtPoint(doc, 40, 0, 8);
			expect(doc.blocks.length).toBe(0);
			doc = applyNoteEditOp(doc, { op: "addBlock", block: withPoints });
			doc = noteEraseInkPointsNearPoint(doc, 10, 0, 6);
			expect(flattenNoteBlocks(doc.blocks).length).toBe(1);
		});

		it("round-trips clipboard payload", () => {
			const blocks = [createNoteTextBlock("A"), createNoteMathBlock("B")];
			const payload = noteClipboardPayload(blocks);
			const parsed = noteBlocksFromClipboardPayload(payload);
			expect(parsed?.length).toBe(2);
			expect(parsed?.[0]?.kind).toBe("text");
		});
	});
}
// #endregion 🧪Tests
