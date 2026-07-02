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
	isPlaygroundFixtureLocked,
	isPlaygroundNoFixtureId,
	PLAYGROUND_NO_FIXTURE_ID,
	playgroundResolvedFixtureId,
	registerWindowBody,
	FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
	FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
	FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
	type AppTools,
	type PlaygroundFixtureCatalog,
	type PlaygroundFixtureHost,
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
} from "@semio-tech/framework-playground-core";
import { DocumentVcsStore, recordProjectionChange } from "@semio-tech/vcs-core/internal";
import type { TreeDataItem, TreeDragAndDropController, TreeDropPosition } from "@semio-tech/ui-react";
import {
	applyNoteEditOp,
	backwardsNoteEditOp,
	createNoteBlockByKind,
	createNoteDocumentVcsEnvelope,
	diffNoteEditOp,
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

export type NotePlayFixtureHostConfig = {
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
				notePlayInspectorTextField(blockIds, "note-play-inspector.text-content", "Content", blocks.map((b) => (b.kind === "text" ? b.content : "")), "textContent"),
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
			fields: [uiInspectorReadonlyField("note-play-inspector.table-shape", "Shape", blocks.map((b) => (b.kind === "table" ? `${b.columns.length}×${b.rows.length}` : "")).join(", "))],
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
			return applyNoteEditOp(doc, { op: "updateBlock", blockId, block: { ...block, content: String(value ?? "") } });
		case "textSize":
			if (block.kind !== "text") return doc;
			return applyNoteEditOp(doc, { op: "updateBlock", blockId, block: { ...block, fontSize: Number(value) } });
		case "mathTex":
			if (block.kind !== "math") return doc;
			return applyNoteEditOp(doc, { op: "updateBlock", blockId, block: { ...block, tex: String(value ?? "") } });
		case "inkWidth":
			if (block.kind !== "ink") return doc;
			return applyNoteEditOp(doc, { op: "updateBlock", blockId, block: { ...block, strokeWidth: Number(value) } });
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

export class NotePlayController extends Controller implements PlaygroundFixtureHost {
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

	constructor(bus: CommandBus, notifyPlatform: () => void, private readonly fixtureHost?: NotePlayFixtureHostConfig) {
		super(NOTE_PLAY_CONTROLLER_ID, bus, notifyPlatform);
		this.rebuildShellMode();
	}

	private canvasMeasures(): readonly WindowMeasure[] {
		const doc = this.projection();
		return [
			{ kind: "slider", id: "note-canvas-zoom", label: "Zoom", value: doc.camera.zoom, min: 0.1, max: 8, step: 0.05, onChange: notePlayCmd("setCameraZoom") },
			{ kind: "slider", id: "note-canvas-pencil", label: "Pencil", value: doc.pencilWidth ?? 3, min: 1, max: 24, step: 1, onChange: notePlayCmd("setPencilWidth") },
			{ kind: "toggle", id: "note-canvas-grid", label: "Grid", pressed: doc.gridVisible ?? true, onChange: notePlayCmd("toggleGrid") },
			{ kind: "toggle", id: "note-canvas-snap", label: "Snap", pressed: doc.snapEnabled ?? false, onChange: notePlayCmd("toggleSnap") },
		];
	}

	private navigatorMeasures(): readonly WindowMeasure[] {
		return [{ kind: "slider", id: "note-navigator-zoom", label: "Navigator zoom", value: this.projection().camera.zoom, min: 0.05, max: 2, step: 0.05, onChange: notePlayCmd("setCameraZoom") }];
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
			status: [{ id: "note-block-count", text: `${flattenNoteBlocks(doc.blocks).length} blocks · ${this.getSelectedIds().length} selected · zoom ${doc.camera.zoom.toFixed(2)}` }],
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
			toolCollection("draw", "pencil", [toolToggle("pencil", "Pencil", "pencil", "pencil"), toolToggle("eraser", "Eraser", "eraser", "eraser")]),
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

	getFixtureCatalog(): PlaygroundFixtureCatalog | null {
		if (isPlaygroundFixtureLocked() || !this.fixtureHost) return null;
		return {
			activeFixtureId: playgroundResolvedFixtureId(this.projection().id === "empty" ? PLAYGROUND_NO_FIXTURE_ID : this.projection().id, this.fixtureHost.defaultId),
			options: this.fixtureHost.options,
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
			case "toggleGrid": {
				this.dispatchEditOp({ op: "setGridVisible", visible: !(this.projection().gridVisible ?? true) });
				return;
			}
			case "toggleSnap": {
				this.dispatchEditOp({ op: "setSnapEnabled", enabled: !(this.projection().snapEnabled ?? false) });
				return;
			}
			case "setActiveFixture": {
				const fixtureId = String(args.fixtureId ?? "");
				if (isPlaygroundNoFixtureId(fixtureId)) {
					this.dispatchEditOp({ op: "setDocument", document: NOTE_PLAY_EMPTY_DOCUMENT });
					this.pointerFocus.setSelection([]);
					return;
				}
				const json = this.fixtureHost?.fileJsonById[fixtureId];
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
			default:
				return;
		}
	}
}

export function buildNotePlayAppRuntime(ctrl: NotePlayController): AppRuntime {
	return createPlayAppRuntime(NOTE_PLAY_APP_ID, "Note", ctrl, NOTE_PLAY_LAYOUT, ctrl.mainMode);
}

export function registerNotePlayDeclarativeBodies(): void {
	registerWindowBody(NOTE_PLAY_BODY_KEY_COMPOSITE, () => buildNoteWindowBody(NOTE_PLAY_SURFACE_ID_COMPOSITE, NOTE_PLAY_CONTROLLER_ID, "composite", "composite"));
	registerWindowBody(NOTE_PLAY_BODY_KEY_NAVIGATOR, () => buildNoteWindowBody(NOTE_PLAY_SURFACE_ID_NAVIGATOR, NOTE_PLAY_CONTROLLER_ID, "navigator", "navigator"));
}

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";
import { notePlayAppDefinition } from "./playground.ts";

/** @emoji 🧩 S program definition for note. */
export function buildNoteProgramDefinition(): PlatformDefinition {
	const app = notePlayAppDefinition;
	return {
		id: "note",
		name: "Note",
		apiVersion: "1",
		apps: [{ id: "note", label: app.label, controllerId: app.controllerId, modes: app.modes, defaultModeId: app.defaultModeId }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension

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
	});
}
// #endregion 🧪Tests

export { notePlayAppDefinition, PlaygroundNote } from "./playground.ts";
