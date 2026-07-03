// #region 🧲Header
/** @emoji 📝 Note play app — infinite canvas shell. */
// #endregion 🧲Header

import {
	AppRuntime,
	CommandBus,
	Controller,
	ModeRuntime,
	WindowKindRuntime,
	CANVAS_HOVER_SOURCE_CANVAS,
	CANVAS_HOVER_SOURCE_HIERARCHY,
	CANVAS_HOVER_SOURCE_CATALOG,
	buildNoteWindowBody,
	createDefaultLayout,
	createPlayAppRuntime,
	isPlaygroundExampleLocked,
	isPlaygroundNoExampleId,
	PLAYGROUND_NO_EXAMPLE_ID,
	playgroundResolvedExampleId,
	registerWindowBody,
	FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
	FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
	FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
	type AppTools,
	type PlaygroundExampleCatalog,
	type PlaygroundExampleHost,
	type ToolLeaf,
	toolCollection,
	uiDeclarativeSectionsToTree,
	UI_INSPECTOR_MIXED_PLACEHOLDER,
	uiInspectorGroupsToTree,
	uiInspectorMixedNumber,
	uiInspectorMixedSelect,
	uiInspectorMixedText,
	uiInspectorMixedToggle,
	uiInspectorReadonlyField,
	type UiInspectorFieldGroup,
	type UiNode,
	type UiTreeItemNode,
	type UiTreeNode,
	type WindowMeasure,
	type WindowEngagement,
	enforcePlaygroundWindowEngagementInput,
  createPlaygroundApp,
  createProductPlaygroundPlatform,
  eagerPlayExampleGlob,
} from "@semio-tech/framework-playground-core";
import { registerOsMediaExportHandler } from "@semio-tech/framework-os-core";
import { pathSegmentsToSvgD, rasterizeSvgMarkupToPngDataUrl } from "@semio-tech/kernel-2d-js";
import { DocumentVcsStore, recordProjectionChange } from "@semio-tech/vcs-core/internal";
import type { TreeDataItem, TreeDragAndDropController, TreeDropPosition } from "@semio-tech/ui-react";
import {
	applyNoteEditOp,
	backwardsNoteEditOp,
	createNoteBlockByKind,
	createNoteDocumentVcsEnvelope,
	diffNoteEditOp,
	flattenNoteBlocks,
	noteImageAssetDataUrl,
	defaultNoteDocument,
	encodeNotePointerFocusKey,
	findNoteBlock,
	flattenNoteBlocks,
	noteDocumentFromJson,
	noteDocumentToJson,
	noteHoverPayloadFromPointerFocusKey,
	noteKindHoverForBlock,
	notePlayBlockIdFromTreeRowId,
	notePlayBlocksTreeHighlightedIds,
	notePlayBlocksTreeRowId,
	noteCloneBlocksWithOffset,
	noteTextParagraphsFromPlainText,
	noteTextPlainText,
	type NoteBlockKind,
	type NoteBlockNode,
	type NoteDocument,
	type NoteEditOp,
	type NoteHoverPayload,
	type NoteKindHover,
	type NoteToolId,
} from "./internal.ts";
export * from "./internal.ts";
import {
	NOTE_PLAY_APP_ID,
	NOTE_PLAY_CONTROLLER_ID,
	NOTE_PLAY_SURFACE_ID_COMPOSITE,
	NOTE_PLAY_SURFACE_ID_NAVIGATOR,
	NOTE_PLAY_BODY_KEY_COMPOSITE,
	NOTE_PLAY_BODY_KEY_NAVIGATOR,
	NOTE_PLAY_WINDOW_KIND_COMPOSITE,
	NOTE_PLAY_WINDOW_KIND_NAVIGATOR,
	NOTE_PLAY_HIERARCHY_TAB_ID,
	NOTE_PLAY_CATALOGUE_TAB_ID,
	NOTE_PLAY_PROPERTIES_TAB_ID,
	NOTE_BLOCK_KIND_DRAG_MIME,
} from "./play-ids.ts";
export {
	NOTE_PLAY_APP_ID,
	NOTE_PLAY_CONTROLLER_ID,
	NOTE_PLAY_SURFACE_ID_COMPOSITE,
	NOTE_PLAY_SURFACE_ID_NAVIGATOR,
	NOTE_PLAY_BODY_KEY_COMPOSITE,
	NOTE_PLAY_BODY_KEY_NAVIGATOR,
	NOTE_PLAY_WINDOW_KIND_COMPOSITE,
	NOTE_PLAY_WINDOW_KIND_NAVIGATOR,
	NOTE_PLAY_HIERARCHY_TAB_ID,
	NOTE_PLAY_CATALOGUE_TAB_ID,
	NOTE_PLAY_PROPERTIES_TAB_ID,
	NOTE_BLOCK_KIND_DRAG_MIME,
} from "./play-ids.ts";

export const NOTE_PLAY_LAYOUT = createDefaultLayout(
	[NOTE_PLAY_WINDOW_KIND_COMPOSITE, NOTE_PLAY_WINDOW_KIND_NAVIGATOR],
	"row",
	[72, 28],
	["Canvas", "Navigator"],
);

export const NOTE_PLAY_EMPTY_DOCUMENT: NoteDocument = defaultNoteDocument("empty");

export type NotePlayExampleHostConfig = {
	readonly defaultId: string;
	readonly options: ReadonlyArray<{ readonly id: string; readonly label: string }>;
	readonly fileJsonById: Readonly<Record<string, string>>;
};

function notePlayCmd(command: string, args: Record<string, unknown> = {}): { controllerId: string; command: string; args: Record<string, unknown> } {
	return { controllerId: NOTE_PLAY_CONTROLLER_ID, command, args };
}

function notePlayBlockIcon(block: NoteBlockNode): string {
	if (block.kind === "text") return "type";
	if (block.kind === "image") return "image";
	if (block.kind === "table") return "table";
	if (block.kind === "math") return "sigma";
	if (block.kind === "ink") return "pencil";
	return "folder";
}

function notePlayBlockTreeItem(block: NoteBlockNode, hoverSink?: (payload: NoteHoverPayload) => void): UiTreeItemNode {
	const rowId = notePlayBlocksTreeRowId(block);
	const nested = block.kind === "group" ? block.children.map((child) => notePlayBlockTreeItem(child, hoverSink)) : undefined;
	return {
		id: rowId,
		label: block.name,
		description: block.kind,
		icon: notePlayBlockIcon(block),
		defaultOpen: block.kind === "group",
		draggable: true,
		dragData: { "application/x-semio-note-block-id": block.id },
		command: notePlayCmd("setSelection", { ids: [block.id] }),
		items: nested,
		isHidden: !block.visible,
		onPointerEnter: hoverSink ? () => hoverSink({ id: block.id, kind: noteKindHoverForBlock(block) }) : undefined,
		onPointerLeave: hoverSink ? () => hoverSink({ id: null, kind: null }) : undefined,
	};
}

export function buildNotePlayHierarchyTree(
	doc: NoteDocument,
	selectedIds: readonly string[],
	hoveredId: string | null,
	kindHover: NoteKindHover | null,
	hoverSink?: (payload: NoteHoverPayload) => void,
): UiTreeNode {
	const highlightedIds = notePlayBlocksTreeHighlightedIds(doc, hoveredId, kindHover);
	const selectedTreeIds = selectedIds
		.map((id) => findNoteBlock(doc, id))
		.filter((block): block is NoteBlockNode => Boolean(block))
		.map((block) => notePlayBlocksTreeRowId(block));
	const toolbarItems: UiTreeItemNode[] = [
		{ id: "note-play-blocks.add.text", label: "Add Text", icon: "type", command: notePlayCmd("addBlock", { kind: "text" }) },
		{ id: "note-play-blocks.add.table", label: "Add Table", icon: "table", command: notePlayCmd("addBlock", { kind: "table" }) },
		{ id: "note-play-blocks.add.math", label: "Add Math", icon: "sigma", command: notePlayCmd("addBlock", { kind: "math" }) },
		{ id: "note-play-blocks.add.image", label: "Add Image", icon: "image", command: notePlayCmd("addBlock", { kind: "image" }) },
		{ id: "note-play-blocks.add.group", label: "Add Group", icon: "folder-plus", command: notePlayCmd("addBlock", { kind: "group" }) },
	];
	const blockItems =
		doc.blocks.length > 0
			? doc.blocks.map((block) => notePlayBlockTreeItem(block, hoverSink))
			: [{ id: "note-play-blocks.empty", label: "Drop blocks here", icon: "sticky-note" as const }];
	return {
		type: "tree",
		sections: [{ id: "note-play-blocks", label: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, defaultOpen: true, items: [...toolbarItems, ...blockItems] }],
		selectedIds: selectedTreeIds,
		highlightedIds: [...highlightedIds],
	};
}

export function buildNotePlayCatalogueTree(hoverSink?: (payload: NoteHoverPayload) => void): UiTreeNode {
	const kinds: readonly { readonly kind: NoteBlockKind; readonly label: string; readonly icon: string }[] = [
		{ kind: "text", label: "Text", icon: "type" },
		{ kind: "image", label: "Image", icon: "image" },
		{ kind: "table", label: "Table", icon: "table" },
		{ kind: "math", label: "Math", icon: "sigma" },
		{ kind: "ink", label: "Ink", icon: "pencil" },
		{ kind: "group", label: "Group", icon: "folder" },
	];
	const items: UiTreeItemNode[] = kinds.map((entry) => ({
		id: `note-play-catalogue.${entry.kind}`,
		label: entry.label,
		icon: entry.icon,
		draggable: true,
		dragData: { [NOTE_BLOCK_KIND_DRAG_MIME]: JSON.stringify({ kind: entry.kind }) },
		onPointerEnter: hoverSink ? () => hoverSink({ id: null, kind: { domain: entry.kind, kindId: entry.kind } }) : undefined,
		onPointerLeave: hoverSink ? () => hoverSink({ id: null, kind: null }) : undefined,
	}));
	return {
		type: "tree",
		sections: [{ id: "note-play-catalogue", label: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, defaultOpen: true, items }],
	};
}

function notePlayInspectorPatch(blockIds: readonly string[], field: string) {
	return notePlayCmd("patchBlocks", { blockIds, field });
}

function notePlayInspectorTextField(blockIds: readonly string[], fieldId: string, label: string, values: readonly string[], field: string): UiNode {
	const mixed = uiInspectorMixedText(values);
	return {
		type: "field",
		id: fieldId,
		label,
		child: {
			type: "input",
			id: `${fieldId}.input`,
			inputKind: "text",
			value: mixed.value,
			placeholder: mixed.placeholder,
			onChange: notePlayInspectorPatch(blockIds, field),
		},
	};
}

function notePlayInspectorNumberField(blockIds: readonly string[], fieldId: string, label: string, values: readonly number[], field: string): UiNode {
	const mixed = uiInspectorMixedNumber(values);
	return {
		type: "field",
		id: fieldId,
		label,
		child: {
			type: "input",
			id: `${fieldId}.input`,
			inputKind: "number",
			value: mixed.uniform ? String(mixed.value) : "",
			placeholder: mixed.uniform ? undefined : UI_INSPECTOR_MIXED_PLACEHOLDER,
			onChange: notePlayInspectorPatch(blockIds, field),
		},
	};
}

export function buildNotePlayInspectorTree(doc: NoteDocument, selectedIds: readonly string[]): UiNode {
	const blocks = selectedIds.map((id) => findNoteBlock(doc, id)).filter((block): block is NoteBlockNode => Boolean(block));
	if (!blocks.length) {
		return uiDeclarativeSectionsToTree([
			{ type: "section", id: "note-play-inspector.empty", label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, children: [{ type: "text", value: "Select a block in the hierarchy." }] },
		]);
	}
	const blockIds = blocks.map((block) => block.id);
	const groups: UiInspectorFieldGroup[] = [];
	const kind = blocks[0]!.kind;
	const uniformKind = blocks.every((block) => block.kind === kind) ? kind : null;
	if (uniformKind === "text") {
		groups.push({
			id: "note-play-inspector.text",
			label: "Text",
			fields: [
				notePlayInspectorTextField(blockIds, "note-play-inspector.text-content", "Content", blocks.map((b) => (b.kind === "text" ? noteTextPlainText(b.paragraphs) : "")), "textContent"),
				notePlayInspectorNumberField(blockIds, "note-play-inspector.text-size", "Size", blocks.map((b) => (b.kind === "text" ? b.fontSize : 0)), "textSize"),
			],
		});
	}
	if (uniformKind === "math") {
		groups.push({
			id: "note-play-inspector.math",
			label: "Math",
			fields: [notePlayInspectorTextField(blockIds, "note-play-inspector.math-tex", "TeX", blocks.map((b) => (b.kind === "math" ? b.tex : "")), "mathTex")],
		});
	}
	if (uniformKind === "table") {
		groups.push({
			id: "note-play-inspector.table",
			label: "Table",
			fields: [
				uiInspectorReadonlyField("note-play-inspector.table-shape", "Shape", blocks.map((b) => (b.kind === "table" ? `${b.columns.length}×${b.rows.length}` : "")).join(", ")),
				{ type: "button", id: "note-play-inspector.table-add-row", iconId: "plus", label: "Add Row", command: notePlayInspectorPatch(blockIds, "tableAddRow") },
				{ type: "button", id: "note-play-inspector.table-remove-row", iconId: "minus", label: "Remove Row", command: notePlayInspectorPatch(blockIds, "tableRemoveRow") },
				{ type: "button", id: "note-play-inspector.table-add-col", iconId: "plus", label: "Add Column", command: notePlayInspectorPatch(blockIds, "tableAddColumn") },
				{ type: "button", id: "note-play-inspector.table-remove-col", iconId: "minus", label: "Remove Column", command: notePlayInspectorPatch(blockIds, "tableRemoveColumn") },
			],
		});
	}
	if (uniformKind === "ink") {
		groups.push({
			id: "note-play-inspector.ink",
			label: "Ink",
			fields: [
				notePlayInspectorNumberField(blockIds, "note-play-inspector.ink-width", "Stroke Width", blocks.map((b) => (b.kind === "ink" ? b.strokeWidth : 0)), "inkWidth"),
				uiInspectorReadonlyField("note-play-inspector.ink-points", "Points", blocks.map((b) => (b.kind === "ink" ? String(b.points.length) : "0")).join(", ")),
			],
		});
	}
	const visibleMixed = uiInspectorMixedToggle(blocks.map((b) => b.visible));
	const lockedMixed = uiInspectorMixedToggle(blocks.map((b) => b.locked));
	groups.push({
		id: "note-play-inspector.block",
		label: "Block",
		fields: [
			notePlayInspectorTextField(blockIds, "note-play-inspector.name", "Name", blocks.map((b) => b.name), "name"),
			notePlayInspectorNumberField(blockIds, "note-play-inspector.x", "X", blocks.map((b) => b.x), "x"),
			notePlayInspectorNumberField(blockIds, "note-play-inspector.y", "Y", blocks.map((b) => b.y), "y"),
			notePlayInspectorNumberField(blockIds, "note-play-inspector.width", "Width", blocks.map((b) => b.width), "width"),
			notePlayInspectorNumberField(blockIds, "note-play-inspector.height", "Height", blocks.map((b) => b.height), "height"),
			{
				type: "field",
				id: "note-play-inspector.visible",
				label: "Visible",
				child: { type: "toggle", id: "note-play-inspector.visible.toggle", iconId: "check", pressed: visibleMixed.pressed, onChange: notePlayInspectorPatch(blockIds, "visible") },
			},
			{
				type: "field",
				id: "note-play-inspector.locked",
				label: "Locked",
				child: { type: "toggle", id: "note-play-inspector.locked.toggle", iconId: "check", pressed: lockedMixed.pressed, onChange: notePlayInspectorPatch(blockIds, "locked") },
			},
		],
	});
	return uiInspectorGroupsToTree(groups);
}

function notePlayPatchBlockField(doc: NoteDocument, blockId: string, field: string, value: unknown): NoteDocument {
	const block = findNoteBlock(doc, blockId);
	if (!block) return doc;
	switch (field) {
		case "name":
			return applyNoteEditOp(doc, { op: "setBlockName", blockId, name: String(value ?? "") });
		case "visible":
			return applyNoteEditOp(doc, { op: "setBlockVisible", blockId, visible: Boolean(value) });
		case "locked":
			return applyNoteEditOp(doc, { op: "setBlockLocked", blockId, locked: Boolean(value) });
		case "x":
		case "y":
		case "width":
		case "height":
			return applyNoteEditOp(doc, { op: "updateBlock", blockId, block: { ...block, [field]: Number(value) } as NoteBlockNode });
		case "textContent":
			if (block.kind !== "text") return doc;
			return applyNoteEditOp(doc, { op: "updateBlock", blockId, block: { ...block, paragraphs: noteTextParagraphsFromPlainText(String(value ?? "")) } });
		case "textSize":
			if (block.kind !== "text") return doc;
			return applyNoteEditOp(doc, { op: "updateBlock", blockId, block: { ...block, fontSize: Number(value) } });
		case "mathTex":
			if (block.kind !== "math") return doc;
			return applyNoteEditOp(doc, { op: "updateBlock", blockId, block: { ...block, tex: String(value ?? "") } });
		case "inkWidth":
			if (block.kind !== "ink") return doc;
			return applyNoteEditOp(doc, { op: "updateBlock", blockId, block: { ...block, strokeWidth: Number(value) } });
		case "tableAddRow":
			if (block.kind !== "table") return doc;
			return applyNoteEditOp(doc, { op: "updateBlock", blockId, block: { ...block, rows: [...block.rows, block.columns.map(() => ({ content: "" }))] } });
		case "tableRemoveRow":
			if (block.kind !== "table" || block.rows.length <= 1) return doc;
			return applyNoteEditOp(doc, { op: "updateBlock", blockId, block: { ...block, rows: block.rows.slice(0, -1) } });
		case "tableAddColumn":
			if (block.kind !== "table") return doc;
			return applyNoteEditOp(doc, {
				op: "updateBlock",
				blockId,
				block: {
					...block,
					columns: [...block.columns, String.fromCharCode(65 + block.columns.length)],
					rows: block.rows.map((row) => [...row, { content: "" }]),
				},
			});
		case "tableRemoveColumn":
			if (block.kind !== "table" || block.columns.length <= 1) return doc;
			return applyNoteEditOp(doc, {
				op: "updateBlock",
				blockId,
				block: { ...block, columns: block.columns.slice(0, -1), rows: block.rows.map((row) => row.slice(0, -1)) },
			});
		default:
			return doc;
	}
}

export function createNotePlayHierarchyTreeDragController(getController: () => NotePlayController | undefined): TreeDragAndDropController {
	return {
		handleDrop: ({ target, targetKind, data, sourceItems, dropPosition }) => {
			const catalogueRaw = data[NOTE_BLOCK_KIND_DRAG_MIME];
			if (catalogueRaw) {
				const parsed = JSON.parse(catalogueRaw) as { kind?: NoteBlockKind };
				if (parsed.kind) getController()?.run("dropBlockKind", { kind: parsed.kind, targetRowId: targetKind === "item" ? (target as TreeDataItem).id : "note-play-blocks", dropPosition: dropPosition ?? "inside" });
				return;
			}
			const sourceItem = sourceItems[0];
			if (!sourceItem || targetKind !== "item") return;
			const blockId = sourceItem.dragData?.["application/x-semio-note-block-id"] ?? notePlayBlockIdFromTreeRowId(sourceItem.id);
			if (!blockId) return;
			getController()?.run("moveBlock", { blockId, targetRowId: (target as TreeDataItem).id, dropPosition: dropPosition ?? "after" });
		},
	};
}

export interface NotePlayHostBridge {
	runHostCommand(command: string, args?: unknown): void;
}

export class NotePlayController extends Controller implements PlaygroundExampleHost {
	readonly mainMode = new ModeRuntime("main", "Note", undefined);
	private readonly docStore = new DocumentVcsStore<NoteDocument, NoteEditOp>({
		envelope: createNoteDocumentVcsEnvelope("note-play", NOTE_PLAY_EMPTY_DOCUMENT),
		applyOp: applyNoteEditOp,
		backwardsOp: backwardsNoteEditOp,
		diffOp: diffNoteEditOp,
	});
	private interactionRevision = 0;
	private listeners = new Set<() => void>();
	private hostBridge: NotePlayHostBridge | null = null;
	private engagementInput = "";

	constructor(bus: CommandBus, notifyPlatform: () => void, private readonlyexampleHost?: NotePlayExampleHostConfig) {
		super(NOTE_PLAY_CONTROLLER_ID, bus, notifyPlatform);
		this.rebuildShellMode();
	}

	private canvasMeasures(): readonly WindowMeasure[] {
		const doc = this.projection();
		const gridSpacing = doc.gridSpacing ?? 32;
		const gridSubdivisions = doc.gridSubdivisions ?? 4;
		const gridOpacity = doc.gridOpacity ?? 0.35;
		const snapSpacing = doc.snapGridSpacing ?? 8;
		return [
			{
				kind: "group",
				id: "note-canvas-camera",
				label: "Camera",
				defaultOpen: true,
				children: [
					{ kind: "slider", id: "note-canvas-zoom", label: "Zoom", value: doc.camera.zoom, min: 0.1, max: 8, step: 0.05, onChange: notePlayCmd("setCameraZoom") },
				],
			},
			{
				kind: "group",
				id: "note-canvas-grid",
				label: "Grid",
				defaultOpen: true,
				children: [
					{ kind: "toggle", id: "note-canvas-grid-visible", iconId: "layout-grid", text: "Show grid", pressed: doc.gridVisible ?? true, onChange: notePlayCmd("setGridVisible") },
					{ kind: "slider", id: "note-canvas-grid-spacing", label: "Major spacing", value: gridSpacing, min: 8, max: 256, step: 4, onChange: notePlayCmd("setGridSpacing") },
					{ kind: "slider", id: "note-canvas-grid-subdivisions", label: "Subdivisions", value: gridSubdivisions, min: 1, max: 16, step: 1, onChange: notePlayCmd("setGridSubdivisions") },
					{ kind: "slider", id: "note-canvas-grid-opacity", label: "Opacity", value: gridOpacity, min: 0.05, max: 1, step: 0.05, onChange: notePlayCmd("setGridOpacity") },
				],
			},
			{
				kind: "group",
				id: "note-canvas-snap",
				label: "Snap",
				defaultOpen: false,
				children: [
					{ kind: "toggle", id: "note-canvas-snap-enabled", iconId: "magnet", text: "Snap to grid", pressed: doc.snapEnabled ?? false, onChange: notePlayCmd("setSnapEnabled") },
					{ kind: "slider", id: "note-canvas-snap-spacing", label: "Snap spacing", value: snapSpacing, min: 1, max: 128, step: 1, onChange: notePlayCmd("setSnapGridSpacing") },
				],
			},
			{
				kind: "group",
				id: "note-canvas-draw",
				label: "Drawing",
				defaultOpen: true,
				children: [
					{ kind: "slider", id: "note-canvas-pencil", label: "Pencil width", value: doc.pencilWidth ?? 3, min: 1, max: 24, step: 1, onChange: notePlayCmd("setPencilWidth") },
					{ kind: "slider", id: "note-canvas-eraser", label: "Eraser radius", value: doc.eraserRadius ?? 12, min: 4, max: 48, step: 1, onChange: notePlayCmd("setEraserRadius") },
				],
			},
		];
	}

	private navigatorMeasures(): readonly WindowMeasure[] {
		const doc = this.projection();
		return [
			{
				kind: "group",
				id: "note-navigator-camera",
				label: "Navigator",
				defaultOpen: true,
				children: [
					{ kind: "slider", id: "note-navigator-zoom", label: "Zoom", value: doc.camera.zoom, min: 0.05, max: 2, step: 0.05, onChange: notePlayCmd("setCameraZoom") },
					{ kind: "toggle", id: "note-navigator-grid", iconId: "layout-grid", text: "Show grid", pressed: doc.gridVisible ?? true, onChange: notePlayCmd("setGridVisible") },
				],
			},
		];
	}

	private canvasEngagement(): WindowEngagement {
		const doc = this.projection();
		return {
			sessionActive: false,
			input: {
				id: "note-canvas-engagement",
				value: this.engagementInput,
				placeholder: "Block name",
				onChange: notePlayCmd("engagementInput"),
				onSubmit: notePlayCmd("engagementSubmit"),
			},
			status: [
				{ id: "note-block-count", text: `${flattenNoteBlocks(doc.blocks).length} blocks · ${this.getSelectedIds().length} selected · zoom ${doc.camera.zoom.toFixed(2)}` },
				{ id: "note-grid-status", text: `${doc.gridVisible !== false ? "grid on" : "grid off"} · ${doc.gridSpacing ?? 32}px major · snap ${doc.snapEnabled ? `${doc.snapGridSpacing ?? 8}px` : "off"}` },
			],
		};
	}

	private navigatorEngagement(): WindowEngagement {
		return {
			sessionActive: false,
			input: { id: "note-navigator-engagement", value: "", placeholder: "Select all", onChange: notePlayCmd("navigatorEngagementInput"), onSubmit: notePlayCmd("selectAll") },
			status: [{ id: "note-active-tool", text: this.projection().activeTool ?? "selectDirect" }],
		};
	}

	private rebuildShellMode(): void {
		this.mainMode.tools = this.buildTools();
		this.mainMode.windowKinds = [
			new WindowKindRuntime(NOTE_PLAY_WINDOW_KIND_COMPOSITE, "Canvas", NOTE_PLAY_BODY_KEY_COMPOSITE, undefined, this.canvasMeasures(), this.canvasEngagement()),
			new WindowKindRuntime(NOTE_PLAY_WINDOW_KIND_NAVIGATOR, "Navigator", NOTE_PLAY_BODY_KEY_NAVIGATOR, undefined, this.navigatorMeasures(), this.navigatorEngagement()),
		];
		for (const windowKind of this.mainMode.windowKinds) enforcePlaygroundWindowEngagementInput(windowKind.engagement, `Note play window "${windowKind.id}"`);
	}

	private projection(): NoteDocument {
		return this.docStore.projection();
	}

	private dispatchEditOp(op: NoteEditOp, selectBlockId?: string): void {
		recordProjectionChange(this.docStore, [op]);
		if (selectBlockId) this.pointerFocus.setSelection([selectBlockId]);
		this.bump();
	}

	private dispatchProjectionEdit(edit: (doc: NoteDocument) => NoteDocument, selectBlockId?: string): void {
		const previous = this.projection();
		const next = edit(previous);
		if (next === previous) return;
		this.dispatchEditOp({ op: "setDocument", document: next }, selectBlockId);
	}

	private buildTools(): AppTools {
		const activeTool = this.projection().activeTool ?? "selectDirect";
		const toolToggle = (id: string, label: string, iconId: string, tool: NoteToolId): ToolLeaf => ({
			id,
			kind: "toggle",
			label,
			iconId,
			pressed: activeTool === tool,
			controllerId: NOTE_PLAY_CONTROLLER_ID,
			command: "setActiveTool",
			args: { tool },
		});
		return [
			toolCollection("open", "folder-open", [{ id: "note-import", kind: "button", label: "Import Note", iconId: "folder-open", controllerId: NOTE_PLAY_CONTROLLER_ID, command: "loadRequest" }]),
			toolCollection("save", "save", [{ id: "note-export", kind: "button", label: "Export Note", iconId: "save", controllerId: NOTE_PLAY_CONTROLLER_ID, command: "saveDownload" }]),
			toolCollection("selection", "mouse-pointer-2", [toolToggle("selectDirect", "Direct", "mouse-pointer", "selectDirect"), toolToggle("selectMarquee", "Marquee", "square-dashed", "selectMarquee")]),
			toolCollection("blocks", "sticky-note", [
				toolToggle("text", "Text", "type", "text"),
				toolToggle("image", "Image", "image", "image"),
				toolToggle("table", "Table", "table", "table"),
				toolToggle("math", "Math", "sigma", "math"),
			]),
			toolCollection("draw", "pencil", [
				toolToggle("pencil", "Pencil", "pencil", "pencil"),
				toolToggle("eraserStroke", "Stroke Eraser", "eraser", "eraserStroke"),
				toolToggle("eraserPoint", "Point Eraser", "circle-dot", "eraserPoint"),
			]),
			toolCollection("transform", "move", [toolToggle("pan", "Pan", "move", "pan")]),
		];
	}

	subscribe(listener: () => void): () => void {
		this.listeners.add(listener);
		return () => this.listeners.delete(listener);
	}

	private bump(): void {
		this.interactionRevision += 1;
		this.rebuildShellMode();
		for (const listener of this.listeners) listener();
		this.emit();
	}

	getInteractionRevision(): number {
		return this.interactionRevision;
	}

	getDocument(): NoteDocument {
		return this.docStore.projection();
	}

	getDocumentVcsStore(): DocumentVcsStore<NoteDocument, NoteEditOp> {
		return this.docStore;
	}

	getDocumentJson(): string {
		return noteDocumentToJson(this.projection());
	}

	setHostBridge(bridge: NotePlayHostBridge | null): void {
		this.hostBridge = bridge;
	}

	getSelectedIds(): readonly string[] {
		return this.pointerFocus.getSnapshot().selection;
	}

	getHoveredId(): string | null {
		return noteHoverPayloadFromPointerFocusKey(this.pointerFocus.getSnapshot().hover).id;
	}

	getHoveredKind(): NoteKindHover | null {
		return noteHoverPayloadFromPointerFocusKey(this.pointerFocus.getSnapshot().hover).kind;
	}

	getExampleCatalog(): PlaygroundExampleCatalog | null {
		if (isPlaygroundExampleLocked() || !this.exampleHost) return null;
		return {
			activeExampleId: playgroundResolvedExampleId(this.projection().id === "empty" ? PLAYGROUND_NO_EXAMPLE_ID : this.projection().id, this.exampleHost.defaultId),
			options: this.exampleHost.options,
		};
	}

	run(command: string, args: Record<string, unknown> = {}): void {
		switch (command) {
			case "engagementInput": {
				const value = String(args.value ?? "");
				if (value !== this.engagementInput) {
					this.engagementInput = value;
					this.rebuildShellMode();
					this.emit();
				}
				return;
			}
			case "engagementSubmit": {
				const value = String(args.value ?? this.engagementInput).trim();
				const selected = this.getSelectedIds();
				if (value && selected.length === 1) this.run("patchBlocks", { blockIds: [...selected], field: "name", value });
				return;
			}
			case "navigatorEngagementInput":
				return;
			case "setCameraZoom": {
				const zoom = typeof args.value === "number" ? args.value : Number(args.value);
				if (!Number.isFinite(zoom)) return;
				this.dispatchEditOp({ op: "setCamera", camera: { ...this.projection().camera, zoom } });
				return;
			}
			case "setPencilWidth": {
				const width = typeof args.value === "number" ? args.value : Number(args.value);
				if (!Number.isFinite(width)) return;
				this.dispatchEditOp({ op: "setPencilWidth", width });
				return;
			}
			case "setEraserRadius": {
				const radius = typeof args.value === "number" ? args.value : Number(args.value);
				if (!Number.isFinite(radius)) return;
				this.dispatchEditOp({ op: "setEraserRadius", radius });
				return;
			}
			case "setGridVisible": {
				const pressed = (args as { pressed?: boolean }).pressed;
				if (typeof pressed === "boolean") this.dispatchEditOp({ op: "setGridVisible", visible: pressed });
				return;
			}
			case "setGridSpacing": {
				const spacing = typeof args.value === "number" ? args.value : Number(args.value);
				if (!Number.isFinite(spacing)) return;
				this.dispatchEditOp({ op: "setGridSpacing", spacing });
				return;
			}
			case "setGridSubdivisions": {
				const subdivisions = typeof args.value === "number" ? args.value : Number(args.value);
				if (!Number.isFinite(subdivisions)) return;
				this.dispatchEditOp({ op: "setGridSubdivisions", subdivisions });
				return;
			}
			case "setGridOpacity": {
				const opacity = typeof args.value === "number" ? args.value : Number(args.value);
				if (!Number.isFinite(opacity)) return;
				this.dispatchEditOp({ op: "setGridOpacity", opacity });
				return;
			}
			case "setSnapEnabled": {
				const pressed = (args as { pressed?: boolean }).pressed;
				if (typeof pressed === "boolean") this.dispatchEditOp({ op: "setSnapEnabled", enabled: pressed });
				return;
			}
			case "setSnapGridSpacing": {
				const spacing = typeof args.value === "number" ? args.value : Number(args.value);
				if (!Number.isFinite(spacing)) return;
				this.dispatchEditOp({ op: "setSnapGridSpacing", spacing });
				return;
			}
			case "toggleGrid": {
				this.dispatchEditOp({ op: "setGridVisible", visible: !(this.projection().gridVisible ?? true) });
				return;
			}
			case "toggleSnap": {
				this.dispatchEditOp({ op: "setSnapEnabled", enabled: !(this.projection().snapEnabled ?? false) });
				return;
			}
			case "setActiveExample": {
				const fixtureId = String(args.exampleId ?? "");
				if (isPlaygroundNoExampleId(fixtureId)) {
					this.dispatchEditOp({ op: "setDocument", document: NOTE_PLAY_EMPTY_DOCUMENT });
					this.pointerFocus.setSelection([]);
					return;
				}
				const json = this.exampleHost?.fileJsonById[fixtureId];
				if (json) {
					this.dispatchEditOp({ op: "setDocument", document: noteDocumentFromJson(json) });
					console.log("[DEBUG] note fixture loaded", fixtureId);
				}
				return;
			}
			case "setFixtureJson": {
				const json = typeof args.json === "string" ? args.json : "";
				if (!json.includes("note.document")) {
					console.log("[DEBUG] note import rejected: not a note document");
					return;
				}
				this.dispatchEditOp({ op: "setDocument", document: noteDocumentFromJson(json) });
				console.log("[DEBUG] note document imported");
				return;
			}
			case "saveDownload":
			case "loadRequest":
				this.hostBridge?.runHostCommand(command, args);
				return;
			case "setSelection": {
				this.pointerFocus.setSelection(Array.isArray(args.ids) ? args.ids.map(String) : []);
				console.log("[DEBUG] note selection", this.getSelectedIds());
				this.bump();
				return;
			}
			case "setHover": {
				const sourceId = typeof args.sourceId === "string" ? args.sourceId : CANVAS_HOVER_SOURCE_CANVAS;
				const id = typeof args.id === "string" ? args.id : null;
				const kind = (args.kind as NoteKindHover | null) ?? null;
				const hoverKey = id ? encodeNotePointerFocusKey(kind?.domain ?? "block", id) : null;
				if (hoverKey) this.pointerFocus.setHoverFromSource(sourceId, hoverKey);
				else this.pointerFocus.clearHoverFromSource(sourceId);
				this.bump();
				return;
			}
			case "setActiveTool": {
				this.dispatchEditOp({ op: "setActiveTool", tool: String(args.tool) as NoteToolId });
				return;
			}
			case "addBlock": {
				const kind = String(args.kind ?? "text") as NoteBlockKind;
				const block = createNoteBlockByKind(kind, 80, 80);
				this.dispatchEditOp({ op: "addBlock", block }, block.id);
				console.log("[DEBUG] note block added", kind, block.id);
				return;
			}
			case "dropBlockKind": {
				const kind = String(args.kind ?? "") as NoteBlockKind;
				const block = createNoteBlockByKind(kind, 80, 80);
				this.dispatchEditOp({ op: "addBlock", block }, block.id);
				return;
			}
			case "moveBlock": {
				const blockId = String(args.blockId ?? "");
				const targetRowId = String(args.targetRowId ?? "");
				const dropPosition = (args.dropPosition ?? "after") as TreeDropPosition;
				const targetId = notePlayBlockIdFromTreeRowId(targetRowId);
				const location = targetId ? findNoteBlock(this.projection(), targetId) : null;
				const index = dropPosition === "before" ? 0 : this.projection().blocks.length;
				this.dispatchEditOp({ op: "reorderBlock", blockId, parentId: location?.kind === "group" ? location.id : undefined, index });
				return;
			}
			case "deleteBlock": {
				const blockId = String(args.blockId ?? "");
				this.dispatchEditOp({ op: "removeBlock", blockId });
				this.pointerFocus.setSelection(this.getSelectedIds().filter((id) => id !== blockId));
				return;
			}
			case "duplicateBlock": {
				this.dispatchEditOp({ op: "duplicateBlock", blockId: String(args.blockId) });
				return;
			}
			case "commitDocument": {
				const document = args.document as NoteDocument;
				if (!document || document.schema !== "note.document") return;
				this.dispatchEditOp({ op: "setDocument", document }, typeof args.selectBlockId === "string" ? args.selectBlockId : undefined);
				return;
			}
			case "setCamera": {
				const camera = args.camera as NoteDocument["camera"];
				if (camera) this.dispatchEditOp({ op: "setCamera", camera });
				return;
			}
			case "patchBlocks": {
				const blockIds = (Array.isArray(args.blockIds) ? args.blockIds : []).map(String).filter(Boolean);
				const field = String(args.field ?? "");
				const value = args.value ?? args.pressed;
				if (!blockIds.length || !field) return;
				this.dispatchProjectionEdit((doc) => {
					let next = doc;
					for (const blockId of blockIds) next = notePlayPatchBlockField(next, blockId, field, value);
					return next;
				});
				return;
			}
			case "selectAll": {
				this.pointerFocus.setSelection(flattenNoteBlocks(this.projection().blocks).map((block) => block.id));
				this.bump();
				return;
			}
			case "clearSelection": {
				this.pointerFocus.setSelection([]);
				this.bump();
				return;
			}
			case "deleteSelection": {
				const ids = [...this.getSelectedIds()];
				for (const blockId of ids) this.dispatchEditOp({ op: "removeBlock", blockId });
				this.pointerFocus.setSelection([]);
				this.bump();
				return;
			}
			case "duplicateSelection": {
				const ids = [...this.getSelectedIds()];
				if (!ids.length) return;
				const blocks = ids.map((id) => findNoteBlock(this.projection(), id)).filter((block): block is NoteBlockNode => Boolean(block));
				const clones = noteCloneBlocksWithOffset(blocks, 24, 24);
				this.dispatchProjectionEdit((doc) => {
					let next = doc;
					for (const block of clones) next = applyNoteEditOp(next, { op: "addBlock", block });
					return next;
				});
				this.pointerFocus.setSelection(clones.map((block) => block.id));
				this.bump();
				return;
			}
			case "nudgeSelection": {
				const dx = Number(args.dx ?? 0);
				const dy = Number(args.dy ?? 0);
				if (!Number.isFinite(dx) || !Number.isFinite(dy)) return;
				const ids = new Set(this.getSelectedIds());
				if (!ids.size) return;
				this.dispatchProjectionEdit((doc) => {
					let next = doc;
					for (const block of flattenNoteBlocks(doc.blocks)) {
						if (!ids.has(block.id) || block.locked) continue;
						next = applyNoteEditOp(next, { op: "updateBlock", blockId: block.id, block: { ...block, x: block.x + dx, y: block.y + dy } });
					}
					return next;
				});
				return;
			}
			case "undo": {
				this.docStore.dispatch({ kind: "undo" });
				this.bump();
				return;
			}
			case "redo": {
				this.docStore.dispatch({ kind: "redo" });
				this.bump();
				return;
			}
			default:
				return;
		}
	}
}

export function buildNotePlayAppRuntime(ctrl: NotePlayController): AppRuntime {
	return createPlayAppRuntime(NOTE_PLAY_APP_ID, "Note", ctrl, NOTE_PLAY_LAYOUT, ctrl.mainMode);
}

export const notePlayWindowBodies: Readonly<Record<string, (ctx: import("@semio-tech/framework-platform-core").WindowBodyViewContext) => UiNode>> = {
	[NOTE_PLAY_BODY_KEY_COMPOSITE]: () => buildNoteWindowBody(NOTE_PLAY_SURFACE_ID_COMPOSITE, NOTE_PLAY_CONTROLLER_ID, "composite", "composite"),
	[NOTE_PLAY_BODY_KEY_NAVIGATOR]: () => buildNoteWindowBody(NOTE_PLAY_SURFACE_ID_NAVIGATOR, NOTE_PLAY_CONTROLLER_ID, "navigator", "navigator"),
};

export function registerNotePlayDeclarativeBodies(): void {
	for (const [key, build] of Object.entries(notePlayWindowBodies)) registerWindowBody(key, build);
}

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";

/** @emoji 🧩 S program definition for note. */
export function buildNoteProgramDefinition(): PlatformDefinition {
	return {
		id: "note",
		name: "Note",
		apiVersion: "1",
		apps: [{ id: "note", label: "Note", controllerId: NOTE_PLAY_CONTROLLER_ID, modes: [{ id: "edit", label: "Edit" }], defaultModeId: "edit" }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension

//#region 🔖OsProgram
import { mergeOsProgramDefinition, osBaselineResource, registerAppVcsHandler } from "@semio-tech/framework-os-core";
import type { OsProgramContribution } from "@semio-tech/framework-platform-core";
import { createNoteAppVcsHandler } from "./internal.ts";

const noteProgramContributionResources = {
		"note": osBaselineResource("2d.note", "note.document", "note"),
	};

/** @emoji 🧩 OS program contribution for note. */
export const noteProgramContribution: OsProgramContribution = {
	programId: "note",
	register() {
		mergeOsProgramDefinition("note", buildNoteProgramDefinition(), noteProgramContributionResources);
		registerNoteMediaExportHandlers();
		registerAppVcsHandler(createNoteAppVcsHandler());
	},
};
//#endregion 🔖OsProgram

//#region 🔖Play
import { NOTE_PLAY_EXAMPLE_DEFAULT_ID } from "./example-slugs.ts";
import semioNoteExample from "../../example/semio.note.json";

let notePlayExampleHostCache: NotePlayExampleHostConfig | undefined;

function noteFixtureIdFromGlobPath(globPath: string): string {
	const base = globPath.split("/").pop() ?? globPath;
	return base.replace(/\.note\.json$/, "");
}

function noteFixtureLabelFromId(id: string): string {
	return id
		.split("-")
		.filter(Boolean)
		.map((word) => word.charAt(0).toUpperCase() + word.slice(1))
		.join(" ");
}

/** @emoji 📂 Builds note playground fixture host config. */
export function createNotePlayExampleHost(): NotePlayExampleHostConfig {
	if (notePlayExampleHostCache) return notePlayExampleHostCache;
	const noteFixtureModules = eagerPlayExampleGlob("../../example/*.note.json");
	const fileJsonById = Object.keys(noteFixtureModules).length
		? Object.fromEntries(
				Object.entries(noteFixtureModules).map(([path, mod]) => {
					const id = noteFixtureIdFromGlobPath(path);
					const json = typeof mod.default === "string" ? mod.default : JSON.stringify(mod.default);
					return [id, json];
				}),
			)
		: { semio: JSON.stringify(semioNoteExample) };
	notePlayExampleHostCache = {
		defaultId: NOTE_PLAY_EXAMPLE_DEFAULT_ID,
		options: Object.keys(fileJsonById)
			.sort()
			.map((id) => ({ id, label: noteFixtureLabelFromId(id) })),
		fileJsonById,
	};
	return notePlayExampleHostCache;
}

/** @emoji 🛝 Note playground app. */


export const notePlayAppDefinition = createPlaygroundApp({
	id: NOTE_PLAY_APP_ID,
	label: "Note",
	controllerId: NOTE_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	devHost: {
		playEntryKind: "note",
		resolveDedupe: ["react", "react-dom", "@semio-tech/note-react"],
		optimizeDeps: { include: ["react", "react-dom", "@semio-tech/note-react"] },
	},
	createRuntime: () => {
		const runtime = createProductPlaygroundPlatform(NOTE_PLAY_APP_ID);
			const exampleHost = createNotePlayExampleHost();
			const ctrl = new NotePlayController(runtime.commandBus, () => runtime.notify(), exampleHost);
			const resolved = playgroundResolvedExampleId(NOTE_PLAY_EXAMPLE_DEFAULT_ID);
			if (exampleHost.fileJsonById[resolved]) ctrl.run("setActiveExample", { exampleId: resolved });
			runtime.addApp(buildNotePlayAppRuntime(ctrl));
			return runtime;
	},
	keybindings: [
		{ key: "ctrl+a,meta+a", controllerId: NOTE_PLAY_CONTROLLER_ID, command: "selectAll" },
		{ key: "delete,backspace", controllerId: NOTE_PLAY_CONTROLLER_ID, command: "deleteSelection" },
		{ key: "ctrl+d,meta+d", controllerId: NOTE_PLAY_CONTROLLER_ID, command: "duplicateSelection" },
		{ key: "ctrl+z,meta+z", controllerId: NOTE_PLAY_CONTROLLER_ID, command: "undo" },
		{ key: "ctrl+shift+z,meta+shift+z,ctrl+y,meta+y", controllerId: NOTE_PLAY_CONTROLLER_ID, command: "redo" },
		{ key: "escape", controllerId: NOTE_PLAY_CONTROLLER_ID, command: "clearSelection" },
		{ key: "up", controllerId: NOTE_PLAY_CONTROLLER_ID, command: "nudgeSelection", args: { dx: 0, dy: -1 } },
		{ key: "down", controllerId: NOTE_PLAY_CONTROLLER_ID, command: "nudgeSelection", args: { dx: 0, dy: 1 } },
		{ key: "left", controllerId: NOTE_PLAY_CONTROLLER_ID, command: "nudgeSelection", args: { dx: -1, dy: 0 } },
		{ key: "right", controllerId: NOTE_PLAY_CONTROLLER_ID, command: "nudgeSelection", args: { dx: 1, dy: 0 } },
		{ key: "shift+up", controllerId: NOTE_PLAY_CONTROLLER_ID, command: "nudgeSelection", args: { dx: 0, dy: -10 } },
		{ key: "shift+down", controllerId: NOTE_PLAY_CONTROLLER_ID, command: "nudgeSelection", args: { dx: 0, dy: 10 } },
		{ key: "shift+left", controllerId: NOTE_PLAY_CONTROLLER_ID, command: "nudgeSelection", args: { dx: -10, dy: 0 } },
		{ key: "shift+right", controllerId: NOTE_PLAY_CONTROLLER_ID, command: "nudgeSelection", args: { dx: 10, dy: 0 } },
	],
	loadRenderer: async () => (await import("@semio-tech/note-react/play")).noteAppRenderer,
});
//#endregion 🔖Play

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("buildNotePlayHierarchyTree", () => {
		it("builds hierarchy for default document", () => {
			const doc = defaultNoteDocument("test");
			const tree = buildNotePlayHierarchyTree(doc, [], null, null);
			expect(tree.sections[0]?.items.length).toBeGreaterThan(0);
		});
	});

	describe("NotePlayController", () => {
		it("adds blocks and syncs selection", () => {
			const bus = new CommandBus();
			const ctrl = new NotePlayController(bus, () => {});
			ctrl.run("addBlock", { kind: "text" });
			expect(ctrl.getDocument().blocks.length).toBe(1);
			expect(ctrl.getSelectedIds().length).toBe(1);
		});

		it("deletes and duplicates selection", () => {
			const bus = new CommandBus();
			const ctrl = new NotePlayController(bus, () => {});
			ctrl.run("addBlock", { kind: "text" });
			ctrl.run("duplicateSelection");
			expect(ctrl.getDocument().blocks.length).toBe(2);
			ctrl.run("deleteSelection");
			expect(ctrl.getDocument().blocks.length).toBe(1);
			ctrl.run("selectAll");
			ctrl.run("deleteSelection");
			expect(ctrl.getDocument().blocks.length).toBe(0);
		});
	});
}
// #endregion 🧪Tests

//#region 🔖MediaExport
function noteDocumentBounds(doc: NoteDocument): { width: number; height: number } {
	let maxX = 1024;
	let maxY = 768;
	for (const block of flattenNoteBlocks(doc.blocks)) {
		if (!block.visible) continue;
		maxX = Math.max(maxX, block.x + block.width);
		maxY = Math.max(maxY, block.y + block.height);
	}
	return { width: Math.max(1, Math.ceil(maxX)), height: Math.max(1, Math.ceil(maxY)) };
}

function noteBlockToSvg(block: NoteBlockNode, doc: NoteDocument): string {
	const transform = `translate(${block.x} ${block.y}) rotate(${block.rotation ?? 0})`;
	if (block.kind === "text") {
		const text = block.paragraphs.map((paragraph) => paragraph.runs.map((run) => run.text).join("")).join("\n");
		return `<g transform="${transform}"><text x="0" y="${block.fontSize}" font-size="${block.fontSize}" font-weight="${block.fontWeight}">${text.replace(/[<>&]/g, "")}</text></g>`;
	}
	if (block.kind === "image") {
		const asset = doc.assets?.[block.imageKey];
		if (!asset) return `<rect width="${block.width}" height="${block.height}" fill="#ddd"/>`;
		return `<g transform="${transform}"><image href="${noteImageAssetDataUrl(asset)}" width="${block.width}" height="${block.height}"/></g>`;
	}
	if (block.kind === "ink" && block.points.length > 1) {
		const segments = block.points.map((point, index) => (index === 0 ? { kind: "move" as const, to: point } : { kind: "line" as const, to: point }));
		const [r, g, b, a] = block.color;
		const stroke = `rgba(${Math.round(r * 255)},${Math.round(g * 255)},${Math.round(b * 255)},${a})`;
		return `<g transform="${transform}"><path d="${pathSegmentsToSvgD(segments)}" fill="none" stroke="${stroke}" stroke-width="${block.strokeWidth}" stroke-linecap="round" stroke-linejoin="round"/></g>`;
	}
	return `<g transform="${transform}"><rect width="${block.width}" height="${block.height}" fill="none" stroke="#888"/></g>`;
}

function noteDocumentToSvg(doc: NoteDocument): string {
	const { width, height } = noteDocumentBounds(doc);
	const body = flattenNoteBlocks(doc.blocks)
		.filter((block) => block.visible)
		.map((block) => noteBlockToSvg(block, doc))
		.join("");
	return `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${width} ${height}" width="${width}" height="${height}">${body}</svg>`;
}

/** @emoji 💾 Registers note document SVG/PNG export handlers for the OS media graph. */
export function registerNoteMediaExportHandlers(): void {
	registerOsMediaExportHandler("2d.note", "svg", async (doc) => ({
		data: noteDocumentToSvg(doc as NoteDocument),
		mimeType: "image/svg+xml",
		fileName: "note.svg",
	}));
	registerOsMediaExportHandler("2d.note", "png", async (doc) => {
		const note = doc as NoteDocument;
		const { width, height } = noteDocumentBounds(note);
		const svg = noteDocumentToSvg(note);
		const dataUrl = await rasterizeSvgMarkupToPngDataUrl(svg, width, height);
		const blob = await fetch(dataUrl).then((response) => response.blob());
		return { data: new Uint8Array(await blob.arrayBuffer()), mimeType: "image/png", fileName: "note.png" };
	});
}
//#endregion 🔖MediaExport

