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
	"eraser",
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

export interface NoteTextBlock extends NoteBlockBase {
	readonly kind: "text";
	readonly content: string;
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

export function createNoteTextBlock(name = "Text", x = 0, y = 0): NoteTextBlock {
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
		content: "Text block",
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
	});
}
// #endregion 🧪Tests
