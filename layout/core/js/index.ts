// #region 🧲Header
/** @emoji 📄 Layout play app — blueprint/preview document editor. */
// #endregion 🧲Header

import {
	AppRuntime,
	CommandBus,
	Controller,
	ModeRuntime,
	WindowKindRuntime,
	buildLayoutWindowBody,
	createPlayAppRuntime,
	createDefaultLayout,
	enforcePlaygroundWindowEngagementInput,
	registerWindowBody,
	FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
	FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
	FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
	uiDeclarativeSectionsToTree,
	uiInspectorGroupsToTree,
	type AppTools,
	type CommandDescriptor,
	type UiInspectorFieldGroup,
	type UiNode,
	type UiTreeItemNode,
	type WindowBodyViewContext,
	type WindowEngagement,
	toolCollection,
	createPlaygroundApp,
	createProductPlaygroundPlatform,
} from "@semio-tech/framework-playground-core";
import { registerOsMediaExportHandler } from "@semio-tech/framework-os-core";
import { type TreeDataItem, type TreeDragAndDropController } from "@semio-tech/ui-react";
import initLayout, { LayoutSession } from "@semio-tech/layout-rs";
import {
	DEFAULT_LAYOUT_DOCUMENT_JSON,
	LayoutHistory,
	createDefaultFrame,
	createDefaultPage,
	findFrame,
	findLink,
	findPage,
	findStory,
	layoutDocumentToJson,
	parseLayoutDocumentJson,
	runLayoutPreflight,
	type FrameKind,
	type LayoutCatalogueKind,
	type LayoutDocument,
	type LayoutFramePropsPatch,
	type LayoutPagePropsPatch,
	type PreflightIssue,
	type TextWrapMode,
} from "./internal.ts";

export * from "./internal.ts";

export const LAYOUT_PLAY_APP_ID = "layout-play";
export const LAYOUT_PLAY_CONTROLLER_ID = "layout-play";
export const LAYOUT_PLAY_BODY_KEY_BLUEPRINT = "layout.play.blueprint";
export const LAYOUT_PLAY_BODY_KEY_PREVIEW = "layout.play.preview";
export const LAYOUT_PLAY_SURFACE_BLUEPRINT = "layout.play.blueprint";
export const LAYOUT_PLAY_SURFACE_PREVIEW = "layout.play.preview";
export const LAYOUT_PLAY_WINDOW_BLUEPRINT = "layout-blueprint";
export const LAYOUT_PLAY_WINDOW_PREVIEW = "layout-preview";
export const LAYOUT_PLAY_LAYOUT = createDefaultLayout(
	[LAYOUT_PLAY_WINDOW_BLUEPRINT, LAYOUT_PLAY_WINDOW_PREVIEW],
	"row",
	[55, 45],
	["Blueprint", "Preview"],
);
export const LAYOUT_PLAY_DEFAULT_FIXTURE_JSON = DEFAULT_LAYOUT_DOCUMENT_JSON;
export const LAYOUT_PLAY_HIERARCHY_TAB_ID = "framework.panel.hierarchy";
export const LAYOUT_PLAY_CATALOGUE_TAB_ID = "framework.panel.catalogue";
export const LAYOUT_PLAY_INSPECTION_TAB_ID = "framework.panel.inspection";
export const LAYOUT_PLAY_PREFLIGHT_TAB_ID = "layout.panel.preflight";
export const LAYOUT_CATALOGUE_KIND_DRAG_MIME = "application/x-semio-layout-catalogue-kind";

export type LayoutHoverPayload = { readonly id: string | null };

function layoutPlayCmd(command: string, args?: Record<string, unknown>): CommandDescriptor {
	return { controllerId: LAYOUT_PLAY_CONTROLLER_ID, command, args };
}

function layoutPlayInspectorPatchPage(pageId: string, field: string): CommandDescriptor {
	return layoutPlayCmd("patchPage", { pageId, field });
}

function layoutPlayInspectorPatchFrame(objectId: string, field: string): CommandDescriptor {
	return layoutPlayCmd("patchFrame", { objectId, field });
}

function layoutPlayFrameIcon(kind: FrameKind): string {
	if (kind === "rect") return "square";
	if (kind === "text") return "type";
	return "image";
}

function layoutPlayPageRowId(pageId: string): string {
	return `layout-hierarchy.page.${pageId}`;
}

function layoutPlayFrameRowId(frameId: string): string {
	return `layout-hierarchy.frame.${frameId}`;
}

function layoutPlayLayerRowId(pageId: string, layerId: string): string {
	return `layout-hierarchy.layer.${pageId}.${layerId}`;
}

function layoutPlayPageIdFromTreeRowId(rowId: string | undefined): string | null {
	if (!rowId?.startsWith("layout-hierarchy.page.")) return null;
	return rowId.slice("layout-hierarchy.page.".length);
}

function layoutPlayFrameIdFromTreeRowId(rowId: string | undefined): string | null {
	if (!rowId?.startsWith("layout-hierarchy.frame.")) return null;
	return rowId.slice("layout-hierarchy.frame.".length);
}

function layoutPlayLayerTargetFromTreeRowId(rowId: string | undefined): { readonly pageId: string; readonly layerId: string } | null {
	const prefix = "layout-hierarchy.layer.";
	if (!rowId?.startsWith(prefix)) return null;
	const rest = rowId.slice(prefix.length);
	const dot = rest.indexOf(".");
	if (dot < 0) return null;
	return { pageId: rest.slice(0, dot), layerId: rest.slice(dot + 1) };
}

function layoutPlaySpreadIdFromTreeRowId(rowId: string | undefined): string | null {
	if (!rowId?.startsWith("layout-hierarchy.spread.")) return null;
	return rowId.slice("layout-hierarchy.spread.".length);
}

function layoutPlayHierarchyTreeHighlightedIds(hoveredId: string | null): string[] {
	if (!hoveredId) return [];
	return [layoutPlayPageRowId(hoveredId), layoutPlayFrameRowId(hoveredId)];
}

function layoutPlayHoverSink(hoverSink: ((payload: LayoutHoverPayload) => void) | undefined, id: string | null) {
	return {
		onPointerEnter: hoverSink ? () => hoverSink({ id }) : undefined,
		onPointerLeave: hoverSink ? () => hoverSink({ id: null }) : undefined,
	};
}

function layoutPlayInspectorNumberField(fieldId: string, label: string, value: number, onChange: CommandDescriptor): UiNode {
	return {
		type: "field",
		id: fieldId,
		label,
		child: {
			type: "input",
			id: `${fieldId}.input`,
			inputKind: "number",
			value: String(value),
			onChange,
		},
	};
}

function layoutPlayInspectorTextField(fieldId: string, label: string, value: string, onChange: CommandDescriptor, multiline = false): UiNode {
	return {
		type: "field",
		id: fieldId,
		label,
		child: {
			type: "input",
			id: `${fieldId}.input`,
			inputKind: multiline ? "textarea" : "text",
			value,
			commit: "blur",
			onChange,
		},
	};
}

function layoutPlayInspectorSelectField(fieldId: string, label: string, value: string, options: readonly { readonly value: string; readonly label: string }[], onChange: CommandDescriptor): UiNode {
	return {
		type: "field",
		id: fieldId,
		label,
		child: {
			type: "select",
			id: `${fieldId}.select`,
			value,
			options: options.map((option) => ({ id: option.value, label: option.label })),
			onChange,
		},
	};
}

function rgbaToInspectorText(color: readonly [number, number, number, number] | undefined): string {
	if (!color) return "";
	return color.map((channel) => String(channel)).join(", ");
}

function inspectorTextToRgba(text: string): readonly [number, number, number, number] | undefined {
	const parts = text.split(",").map((part) => Number(part.trim()));
	if (parts.length !== 4 || parts.some((part) => Number.isNaN(part))) return undefined;
	return [parts[0]!, parts[1]!, parts[2]!, parts[3]!];
}

/** @emoji 🧰 Layout play footer toolbar. */
export function buildLayoutPlayToolbarTools(controllerId: string): AppTools {
	return [
		toolCollection("document", "file-text", [
			{ kind: "button", id: "layout.undo", label: "Undo", iconId: "rotate-ccw", controllerId, command: "undo" },
			{ kind: "button", id: "layout.redo", label: "Redo", iconId: "rotate-cw", controllerId, command: "redo" },
		]),
		toolCollection("export", "download", [
			{ kind: "button", id: "layout.export.png", label: "PNG", iconId: "image", controllerId, command: "exportPng" },
			{ kind: "button", id: "layout.export.svg", label: "SVG", iconId: "file-image", controllerId, command: "exportSvg" },
			{ kind: "button", id: "layout.export.pdf", label: "PDF", iconId: "file-type", controllerId, command: "exportPdf" },
			{ kind: "button", id: "layout.package", label: "Package", iconId: "archive", controllerId, command: "exportPackage" },
		]),
	];
}

function downloadBytes(bytes: Uint8Array, mime: string, filename: string): void {
	const blob = new Blob([bytes], { type: mime });
	const url = URL.createObjectURL(blob);
	const anchor = document.createElement("a");
	anchor.href = url;
	anchor.download = filename;
	anchor.click();
	URL.revokeObjectURL(url);
}

/** @emoji 🌳 Layout play hierarchy panel tree. */
export function buildLayoutPlayHierarchyTree(
	documentJson: string,
	selectedIds: readonly string[],
	hoveredId: string | null = null,
	hoverSink?: (payload: LayoutHoverPayload) => void,
): UiNode {
	const doc = parseLayoutDocumentJson(documentJson);
	if (!doc) {
		return { type: "tree", sections: [{ id: "layout-hierarchy.invalid", label: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, defaultOpen: true, items: [{ id: "layout-hierarchy.invalid.msg", label: "Invalid document" }] }] };
	}
	const frameItems: UiTreeItemNode[] = doc.pages.flatMap((page) =>
		page.frames.map((frame) => ({
			id: layoutPlayFrameRowId(frame.id),
			label: frame.id,
			description: `${page.name} · ${frame.kind}`,
			icon: layoutPlayFrameIcon(frame.kind),
			command: layoutPlayCmd("setSelection", { ids: [frame.id] }),
			...layoutPlayHoverSink(hoverSink, frame.id),
		})),
	);
	const sections = [
		{
			id: "layout-hierarchy.document",
			label: "Document",
			defaultOpen: true,
			items: [{ id: "layout-hierarchy.document.root", label: doc.name, description: doc.schema }],
		},
		{
			id: "layout-hierarchy.spreads",
			label: "Spreads",
			defaultOpen: true,
			items: doc.spreads.map((spread) => ({ id: `layout-hierarchy.spread.${spread.id}`, label: spread.name, description: spread.pageIds.join(", ") })),
		},
		{
			id: "layout-hierarchy.pages",
			label: "Pages",
			defaultOpen: true,
			items: doc.pages.map((page) => ({
				id: layoutPlayPageRowId(page.id),
				label: page.name,
				description: page.parentPageId ? `parent: ${page.parentPageId}` : undefined,
				command: layoutPlayCmd("setActivePage", { pageId: page.id }),
				...layoutPlayHoverSink(hoverSink, page.id),
			})),
		},
		{
			id: "layout-hierarchy.frames",
			label: "Frames",
			defaultOpen: true,
			items: frameItems.length > 0 ? frameItems : [{ id: "layout-hierarchy.frames.empty", label: "Drop catalogue items here", icon: "square" as const }],
		},
		{
			id: "layout-hierarchy.parentPages",
			label: "Parent Pages",
			defaultOpen: false,
			items: doc.parentPages.map((parent) => ({ id: `layout-hierarchy.parent.${parent.id}`, label: parent.name })),
		},
		{
			id: "layout-hierarchy.layers",
			label: "Layers",
			defaultOpen: false,
			items: doc.pages.flatMap((page) =>
				page.layers.map((layer) => ({
					id: layoutPlayLayerRowId(page.id, layer.id),
					label: `${page.name} · ${layer.name}`,
					description: `${layer.objectIds.length} objects`,
					...layoutPlayHoverSink(hoverSink, null),
				})),
			),
		},
		{
			id: "layout-hierarchy.stories",
			label: "Stories",
			defaultOpen: false,
			items: doc.stories.map((story) => ({ id: `layout-hierarchy.story.${story.id}`, label: story.id, description: `${story.content.length} chars` })),
		},
		{
			id: "layout-hierarchy.links",
			label: "Links",
			defaultOpen: false,
			items: doc.links.map((link) => ({
				id: `layout-hierarchy.link.${link.id}`,
				label: link.path,
				description: link.state ?? "ok",
				command: layoutPlayCmd("setSelection", { ids: doc.pages.flatMap((p) => p.frames.filter((f) => f.kind === "image" && f.linkId === link.id).map((f) => f.id)) }),
			})),
		},
		{
			id: "layout-hierarchy.styles",
			label: "Styles",
			defaultOpen: false,
			items: [
				...doc.paragraphStyles.map((style) => ({ id: `layout-hierarchy.paragraph.${style.id}`, label: style.name, description: style.id })),
				...doc.characterStyles.map((style) => ({ id: `layout-hierarchy.character.${style.id}`, label: style.name, description: style.id })),
			],
		},
	];
	return {
		type: "tree",
		sections,
		selectedIds: selectedIds.flatMap((id) => [layoutPlayPageRowId(id), layoutPlayFrameRowId(id)]),
		highlightedIds: layoutPlayHierarchyTreeHighlightedIds(hoveredId),
	};
}

/** @emoji 📚 Layout play catalogue panel tree. */
export function buildLayoutPlayCatalogueTree(hoverSink?: (payload: LayoutHoverPayload) => void): UiNode {
	const catalogueItems: readonly { readonly id: string; readonly label: string; readonly icon: string; readonly kind: LayoutCatalogueKind }[] = [
		{ id: "layout-play-catalogue.page", label: "Page", icon: "file", kind: "page" },
		{ id: "layout-play-catalogue.rect", label: "Rectangle", icon: "square", kind: "rect" },
		{ id: "layout-play-catalogue.text", label: "Text", icon: "type", kind: "text" },
		{ id: "layout-play-catalogue.image", label: "Image", icon: "image", kind: "image" },
	];
	return {
		type: "tree",
		sections: [
			{
				id: "layout-play-catalogue",
				label: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
				defaultOpen: true,
				items: catalogueItems.map((item) => ({
					id: item.id,
					label: item.label,
					icon: item.icon,
					draggable: true,
					dragData: { [LAYOUT_CATALOGUE_KIND_DRAG_MIME]: JSON.stringify({ kind: item.kind }) },
					...layoutPlayHoverSink(hoverSink, null),
				})),
			},
		],
	};
}

/** @emoji 🖱️ Hierarchy tree drag controller for layout catalogue drops. */
export function createLayoutPlayHierarchyTreeDragController(getController: () => LayoutPlayController | undefined): TreeDragAndDropController {
	return {
		handleDrop: ({ target, targetKind, data }) => {
			const catalogueRaw = data[LAYOUT_CATALOGUE_KIND_DRAG_MIME];
			if (!catalogueRaw) return;
			const parsed = JSON.parse(catalogueRaw) as { kind?: LayoutCatalogueKind };
			if (!parsed.kind) return;
			const ctrl = getController();
			if (!ctrl) return;
			const targetRowId = targetKind === "item" ? (target as TreeDataItem).id : undefined;
			if (parsed.kind === "page") {
				ctrl.run("addPage", { spreadId: layoutPlaySpreadIdFromTreeRowId(targetRowId) ?? undefined });
				return;
			}
			const layerTarget = layoutPlayLayerTargetFromTreeRowId(targetRowId);
			const pageId = layoutTarget?.pageId ?? layoutPlayPageIdFromTreeRowId(targetRowId) ?? ctrl.getActivePageId();
			const layerId = layerTarget?.layerId ?? ctrl.getDocument().pages.find((page) => page.id === pageId)?.layerIds[0];
			ctrl.run("addFrame", { kind: parsed.kind, pageId, layerId });
		},
	};
}

/** @emoji 🔍 Layout play preflight panel tree. */
export function buildLayoutPlayPreflightTree(documentJson: string): UiNode {
	const doc = parseLayoutDocumentJson(documentJson);
	const issues = doc ? runLayoutPreflight(doc) : [];
	return {
		type: "tree",
		sections: [
			{
				id: "layout-preflight.issues",
				label: "Preflight",
				defaultOpen: true,
				items: issues.length
					? issues.map((issue) => ({
							id: `layout-preflight.${issue.code}.${issue.objectId ?? issue.pageId ?? issue.message}`,
							label: issue.message,
							description: `${issue.severity} · ${issue.code}`,
							command: layoutPlayCmd("focusPreflightIssue", { issue }),
						}))
					: [{ id: "layout-preflight.empty", label: "No issues" }],
			},
		],
	};
}

/** @emoji 🔎 Layout play inspector panel tree. */
export function buildLayoutPlayInspectorTree(documentJson: string, selectedIds: readonly string[]): UiNode {
	const doc = parseLayoutDocumentJson(documentJson);
	if (!doc || !selectedIds.length) {
		return uiDeclarativeSectionsToTree([
			{ type: "section", id: "layout-inspector.empty", label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, children: [{ type: "text", value: "Select a page or frame." }] },
		]);
	}
	const frame = findFrame(doc, selectedIds[0] ?? "");
	const page = doc.pages.find((entry) => entry.id === selectedIds[0]) ?? (frame ? doc.pages.find((entry) => entry.frames.some((candidate) => candidate.id === frame.id)) : undefined);
	const groups: UiInspectorFieldGroup[] = [];
	if (page && selectedIds[0] === page.id) {
		groups.push({
			id: "layout-inspector.page",
			label: "Page",
			fields: [
				layoutPlayInspectorTextField("layout-inspector.page.name", "Name", page.name, layoutPlayInspectorPatchPage(page.id, "name")),
				layoutPlayInspectorNumberField("layout-inspector.page.width", "Width", page.width, layoutPlayInspectorPatchPage(page.id, "width")),
				layoutPlayInspectorNumberField("layout-inspector.page.height", "Height", page.height, layoutPlayInspectorPatchPage(page.id, "height")),
				layoutPlayInspectorNumberField("layout-inspector.page.margin.top", "Margin Top", page.margins.top, layoutPlayInspectorPatchPage(page.id, "marginTop")),
				layoutPlayInspectorNumberField("layout-inspector.page.margin.right", "Margin Right", page.margins.right, layoutPlayInspectorPatchPage(page.id, "marginRight")),
				layoutPlayInspectorNumberField("layout-inspector.page.margin.bottom", "Margin Bottom", page.margins.bottom, layoutPlayInspectorPatchPage(page.id, "marginBottom")),
				layoutPlayInspectorNumberField("layout-inspector.page.margin.left", "Margin Left", page.margins.left, layoutPlayInspectorPatchPage(page.id, "marginLeft")),
				layoutPlayInspectorNumberField("layout-inspector.page.columns.count", "Columns", page.columns.count, layoutPlayInspectorPatchPage(page.id, "columnsCount")),
				layoutPlayInspectorNumberField("layout-inspector.page.columns.gutter", "Gutter", page.columns.gutter, layoutPlayInspectorPatchPage(page.id, "columnsGutter")),
			],
		});
	}
	if (frame) {
		const fields: UiNode[] = [
			layoutPlayInspectorNumberField("layout-inspector.frame.bounds.x", "X", frame.bounds.x, layoutPlayInspectorPatchFrame(frame.id, "boundsX")),
			layoutPlayInspectorNumberField("layout-inspector.frame.bounds.y", "Y", frame.bounds.y, layoutPlayInspectorPatchFrame(frame.id, "boundsY")),
			layoutPlayInspectorNumberField("layout-inspector.frame.bounds.w", "Width", frame.bounds.w, layoutPlayInspectorPatchFrame(frame.id, "boundsW")),
			layoutPlayInspectorNumberField("layout-inspector.frame.bounds.h", "Height", frame.bounds.h, layoutPlayInspectorPatchFrame(frame.id, "boundsH")),
		];
		if (frame.kind === "rect") {
			fields.push(
				layoutPlayInspectorTextField("layout-inspector.frame.fill", "Fill", rgbaToInspectorText(frame.fill), layoutPlayInspectorPatchFrame(frame.id, "fill")),
				layoutPlayInspectorTextField("layout-inspector.frame.stroke", "Stroke", rgbaToInspectorText(frame.stroke), layoutPlayInspectorPatchFrame(frame.id, "stroke")),
			);
		}
		if (frame.kind === "text") {
			const story = findStory(doc, frame.storyId);
			fields.push(
				layoutPlayInspectorTextField("layout-inspector.frame.story", "Story", story?.content ?? "", layoutPlayInspectorPatchFrame(frame.id, "storyContent"), true),
				layoutPlayInspectorSelectField(
					"layout-inspector.frame.wrapMode",
					"Wrap Mode",
					frame.wrapMode,
					[
						{ value: "none", label: "None" },
						{ value: "box", label: "Box" },
						{ value: "contour", label: "Contour" },
					],
					layoutPlayInspectorPatchFrame(frame.id, "wrapMode"),
				),
				layoutPlayInspectorNumberField("layout-inspector.frame.columns", "Columns", frame.columns, layoutPlayInspectorPatchFrame(frame.id, "columns")),
			);
		}
		if (frame.kind === "image") {
			const link = findLink(doc, frame.linkId);
			fields.push(layoutPlayInspectorTextField("layout-inspector.frame.linkPath", "Link Path", link?.path ?? "", layoutPlayInspectorPatchFrame(frame.id, "linkPath")));
		}
		groups.push({ id: "layout-inspector.frame", label: `Frame (${frame.kind})`, fields });
	}
	return uiInspectorGroupsToTree(groups);
}

/** @emoji 🎮 Layout play controller. */
export class LayoutPlayController extends Controller {
	readonly mainMode = new ModeRuntime("main", "Edit", undefined);
	private history = new LayoutHistory(parseLayoutDocumentJson(DEFAULT_LAYOUT_DOCUMENT_JSON)!);
	private activePageId = "page-1";
	private interactionRevision = 0;
	private readonly snapshotListeners = new Set<() => void>();

	constructor(commandBus: CommandBus, hostNotify: () => void) {
		super(LAYOUT_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.rebuildShellMode();
	}

	getDocumentJson(): string {
		return layoutDocumentToJson(this.history.getDocument());
	}

	getSelectedIds(): readonly string[] {
		return this.pointerFocus.getSnapshot().selection;
	}

	getHoveredId(): string | null {
		return this.pointerFocus.getSnapshot().hover;
	}

	getDocument(): LayoutDocument {
		return this.history.getDocument();
	}

	getActivePageId(): string {
		return this.activePageId;
	}

	getPreflightIssues(): readonly PreflightIssue[] {
		return runLayoutPreflight(this.history.getDocument());
	}

	getInteractionRevision(): number {
		return this.interactionRevision;
	}

	canUndo(): boolean {
		return this.history.canUndo();
	}

	canRedo(): boolean {
		return this.history.canRedo();
	}

	subscribeSnapshot(listener: () => void): () => void {
		this.snapshotListeners.add(listener);
		return () => this.snapshotListeners.delete(listener);
	}

	private notifySnapshot(): void {
		for (const listener of this.snapshotListeners) listener();
	}

	private commitDocument(next: LayoutDocument): void {
		const json = layoutDocumentToJson(next);
		if (json === this.getDocumentJson()) return;
		this.history = new LayoutHistory(next);
		this.interactionRevision += 1;
		this.notifySnapshot();
		this.emit();
	}

	private applyDocumentJson(json: string): void {
		const parsed = parseLayoutDocumentJson(json);
		if (!parsed) return;
		this.history = new LayoutHistory(parsed);
		this.interactionRevision += 1;
		this.notifySnapshot();
		this.emit();
	}

	private bumpInteraction(): void {
		this.interactionRevision += 1;
		this.notifySnapshot();
		this.emit();
	}

	private windowEngagement(label: string): WindowEngagement {
		return {
			sessionActive: false,
			input: {
				id: `layout-engagement-${label}`,
				value: "",
				placeholder: "undo, redo, export png",
				onChange: layoutPlayCmd("engagementInput"),
				onSubmit: layoutPlayCmd("engagementSubmit"),
			},
			possibleEngagements: [
				{ id: "layout.undo", label: "Undo", command: layoutPlayCmd("undo") },
				{ id: "layout.redo", label: "Redo", command: layoutPlayCmd("redo") },
			],
			controls: [],
			status: [{ id: `layout-status-${label}`, text: `Page ${this.activePageId}` }],
		};
	}

	private rebuildShellMode(): void {
		this.mainMode.tools = buildLayoutPlayToolbarTools(LAYOUT_PLAY_CONTROLLER_ID);
		this.mainMode.windowKinds = [
			new WindowKindRuntime(LAYOUT_PLAY_WINDOW_BLUEPRINT, "Blueprint", LAYOUT_PLAY_BODY_KEY_BLUEPRINT, undefined, [], this.windowEngagement("blueprint")),
			new WindowKindRuntime(LAYOUT_PLAY_WINDOW_PREVIEW, "Preview", LAYOUT_PLAY_BODY_KEY_PREVIEW, undefined, [], this.windowEngagement("preview")),
		];
		for (const windowKind of this.mainMode.windowKinds) {
			enforcePlaygroundWindowEngagementInput(windowKind.engagement, `Layout play window "${windowKind.id}"`);
		}
	}

	override run(command: string, args?: unknown): void {
		if (command === "setDocumentJson") {
			const json = (args as { json?: string }).json;
			if (typeof json === "string") this.applyDocumentJson(json);
			return;
		}
		if (command === "setSelection") {
			const ids = (args as { ids?: string[] }).ids;
			if (!Array.isArray(ids)) return;
			this.pointerFocus.setSelection([...new Set(ids.filter((id) => typeof id === "string"))]);
			this.bumpInteraction();
			return;
		}
		if (command === "setHover") {
			const id = (args as { id?: string | null }).id ?? null;
			const sourceId = (args as { sourceId?: string }).sourceId ?? "canvas";
			if (id) this.pointerFocus.setHoverFromSource(sourceId, id);
			else this.pointerFocus.clearHoverFromSource(sourceId);
			this.bumpInteraction();
			return;
		}
		if (command === "setActivePage") {
			const pageId = (args as { pageId?: string }).pageId;
			if (typeof pageId !== "string") return;
			this.activePageId = pageId;
			this.pointerFocus.setSelection([pageId]);
			this.rebuildShellMode();
			this.bumpInteraction();
			return;
		}
		if (command === "addPage") {
			const spreadId =
				(args as { spreadId?: string }).spreadId ??
				findPage(this.history.getDocument(), this.activePageId)?.spreadId ??
				this.history.getDocument().spreads[0]?.id;
			if (!spreadId) return;
			const spread = this.history.getDocument().spreads.find((entry) => entry.id === spreadId);
			if (!spread) return;
			const page = createDefaultPage(spreadId, spread.pageIds.length);
			this.history.apply({ type: "add_page", spreadId, page });
			this.activePageId = page.id;
			this.pointerFocus.setSelection([page.id]);
			this.rebuildShellMode();
			this.bumpInteraction();
			return;
		}
		if (command === "addFrame") {
			const kind = (args as { kind?: FrameKind }).kind;
			const pageId = (args as { pageId?: string }).pageId ?? this.activePageId;
			const page = findPage(this.history.getDocument(), pageId);
			const layerId = (args as { layerId?: string }).layerId ?? page?.layerIds[0];
			if (!kind || !page || !layerId) return;
			const created = createDefaultFrame(kind, layerId);
			this.history.apply({ type: "add_frame", pageId, frame: created.frame, story: created.story, link: created.link });
			this.pointerFocus.setSelection([created.frame.id]);
			this.bumpInteraction();
			return;
		}
		if (command === "patchPage") {
			const pageId = (args as { pageId?: string }).pageId;
			const field = (args as { field?: string }).field;
			const value = (args as { value?: unknown }).value;
			if (!pageId || typeof field !== "string") return;
			const page = findPage(this.history.getDocument(), pageId);
			if (!page) return;
			const before: LayoutPagePropsPatch = {};
			const after: LayoutPagePropsPatch = {};
			if (field === "name") {
				before.name = page.name;
				after.name = String(value ?? "");
			} else if (field === "width") {
				before.width = page.width;
				after.width = Number(value);
			} else if (field === "height") {
				before.height = page.height;
				after.height = Number(value);
			} else if (field === "marginTop") {
				before.margins = page.margins;
				after.margins = { ...page.margins, top: Number(value) };
			} else if (field === "marginRight") {
				before.margins = page.margins;
				after.margins = { ...page.margins, right: Number(value) };
			} else if (field === "marginBottom") {
				before.margins = page.margins;
				after.margins = { ...page.margins, bottom: Number(value) };
			} else if (field === "marginLeft") {
				before.margins = page.margins;
				after.margins = { ...page.margins, left: Number(value) };
			} else if (field === "columnsCount") {
				before.columns = page.columns;
				after.columns = { ...page.columns, count: Number(value) };
			} else if (field === "columnsGutter") {
				before.columns = page.columns;
				after.columns = { ...page.columns, gutter: Number(value) };
			} else {
				return;
			}
			this.history.apply({ type: "patch_page_props", pageId, before, after });
			this.bumpInteraction();
			return;
		}
		if (command === "patchFrame") {
			const objectId = (args as { objectId?: string }).objectId;
			const field = (args as { field?: string }).field;
			const value = (args as { value?: unknown }).value;
			if (!objectId || typeof field !== "string") return;
			const frame = findFrame(this.history.getDocument(), objectId);
			if (!frame) return;
			if (field === "boundsX" || field === "boundsY" || field === "boundsW" || field === "boundsH") {
				const before = frame.bounds;
				const after = {
					...before,
					...(field === "boundsX" ? { x: Number(value) } : {}),
					...(field === "boundsY" ? { y: Number(value) } : {}),
					...(field === "boundsW" ? { w: Number(value) } : {}),
					...(field === "boundsH" ? { h: Number(value) } : {}),
				};
				this.history.apply({ type: "set_object_bounds", objectId, before, after });
				this.bumpInteraction();
				return;
			}
			const before: LayoutFramePropsPatch = {};
			const after: LayoutFramePropsPatch = {};
			if (field === "fill") {
				const rgba = inspectorTextToRgba(String(value ?? ""));
				if (!rgba || frame.kind !== "rect") return;
				before.fill = frame.fill;
				after.fill = rgba;
			} else if (field === "stroke") {
				const rgba = inspectorTextToRgba(String(value ?? ""));
				if (!rgba || frame.kind !== "rect") return;
				before.stroke = frame.stroke;
				after.stroke = rgba;
			} else if (field === "storyContent") {
				if (frame.kind !== "text") return;
				const story = findStory(this.history.getDocument(), frame.storyId);
				if (!story) return;
				before.storyContent = story.content;
				after.storyContent = String(value ?? "");
			} else if (field === "wrapMode") {
				if (frame.kind !== "text") return;
				before.wrapMode = frame.wrapMode;
				after.wrapMode = String(value ?? "box") as TextWrapMode;
			} else if (field === "columns") {
				if (frame.kind !== "text") return;
				before.columns = frame.columns;
				after.columns = Number(value);
			} else if (field === "linkPath") {
				if (frame.kind !== "image") return;
				const link = findLink(this.history.getDocument(), frame.linkId);
				before.linkPath = link?.path;
				after.linkPath = String(value ?? "");
			} else {
				return;
			}
			this.history.apply({ type: "patch_frame_props", objectId, before, after });
			this.bumpInteraction();
			return;
		}
		if (command === "focusPreflightIssue") {
			const issue = (args as { issue?: PreflightIssue }).issue;
			if (!issue) return;
			if (issue.objectId) this.pointerFocus.setSelection([issue.objectId]);
			if (issue.pageId) this.activePageId = issue.pageId;
			this.bumpInteraction();
			return;
		}
		if (command === "undo") {
			const doc = this.history.undo();
			if (doc) this.bumpInteraction();
			return;
		}
		if (command === "redo") {
			const doc = this.history.redo();
			if (doc) this.bumpInteraction();
			return;
		}
		if (command === "exportPng" || command === "exportSvg" || command === "exportPdf" || command === "exportPackage") {
			if (typeof document === "undefined") return;
			void (async () => {
				const { LayoutEngineSession } = await import("@semio-tech/layout-react");
				const session = new LayoutEngineSession("preview");
				await session.ensureReady();
				session.setDocumentJson(this.getDocumentJson());
				session.setPageId(this.activePageId);
				if (command === "exportPng") {
					downloadBytes(session.exportPng(this.activePageId), "image/png", `${this.activePageId}.png`);
				} else if (command === "exportSvg") {
					const svg = session.exportSvg(this.activePageId);
					downloadBytes(new TextEncoder().encode(svg), "image/svg+xml", `${this.activePageId}.svg`);
				} else if (command === "exportPdf") {
					downloadBytes(session.exportPdf(this.activePageId), "application/pdf", `${this.activePageId}.pdf`);
				} else if (command === "exportPackage") {
					downloadBytes(session.exportPackage(JSON.stringify(this.getPreflightIssues())), "application/zip", "layout-package.zip");
				}
			})();
			return;
		}
	}
}

function buildLayoutPlayBlueprintBody(_ctx: WindowBodyViewContext): UiNode {
	return buildLayoutWindowBody(LAYOUT_PLAY_SURFACE_BLUEPRINT, LAYOUT_PLAY_CONTROLLER_ID, LAYOUT_PLAY_WINDOW_BLUEPRINT, "blueprint");
}

function buildLayoutPlayPreviewBody(_ctx: WindowBodyViewContext): UiNode {
	return buildLayoutWindowBody(LAYOUT_PLAY_SURFACE_PREVIEW, LAYOUT_PLAY_CONTROLLER_ID, LAYOUT_PLAY_WINDOW_PREVIEW, "preview");
}

/** @emoji 🧩 Registers layout play window bodies. */
export function registerLayoutPlayDeclarativeBodies(): void {
	registerWindowBody(LAYOUT_PLAY_BODY_KEY_BLUEPRINT, buildLayoutPlayBlueprintBody);
	registerWindowBody(LAYOUT_PLAY_BODY_KEY_PREVIEW, buildLayoutPlayPreviewBody);
}

/** @emoji 🛝 Builds layout play {@link AppRuntime}. */
export function buildLayoutPlayAppRuntime(controller: LayoutPlayController): AppRuntime {
	return createPlayAppRuntime(LAYOUT_PLAY_APP_ID, "Layout", controller, LAYOUT_PLAY_LAYOUT, controller.mainMode);
}

export { layoutPlayCmd, downloadBytes };

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";

/** @emoji 🧩 S program definition for layout. */
export function buildLayoutProgramDefinition(): PlatformDefinition {
	return {
		id: "layout",
		name: "Layout",
		apiVersion: "1",
		apps: [{ id: "layout", label: "Layout", controllerId: LAYOUT_PLAY_CONTROLLER_ID, modes: [{ id: "edit", label: "Edit" }], defaultModeId: "edit" }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension

//#region 🔖MediaExport
let layoutExportWasmReady: Promise<void> | null = null;

async function ensureLayoutExportWasm(): Promise<void> {
	if (!layoutExportWasmReady) layoutExportWasmReady = initLayout().then(() => undefined);
	await layoutExportWasmReady;
}

async function exportLayoutDocumentMedia(doc: LayoutDocument): Promise<{ svg: string; png: Uint8Array }> {
	await ensureLayoutExportWasm();
	const session = new LayoutSession();
	session.setDocumentJson(layoutDocumentToJson(doc));
	const pageId = doc.pages[0]?.id ?? "page-1";
	session.setPageId(pageId);
	return { svg: session.exportSvg(pageId), png: session.exportPng(pageId) };
}

/** @emoji 💾 Registers layout document SVG/PNG export handlers for the OS media graph. */
export function registerLayoutMediaExportHandlers(): void {
	registerOsMediaExportHandler("2d.layout", "svg", async (doc) => {
		const { svg } = await exportLayoutDocumentMedia(doc as LayoutDocument);
		return { data: svg, mimeType: "image/svg+xml", fileName: "layout.svg" };
	});
	registerOsMediaExportHandler("2d.layout", "png", async (doc) => {
		const { png } = await exportLayoutDocumentMedia(doc as LayoutDocument);
		return { data: png, mimeType: "image/png", fileName: "layout.png" };
	});
}
//#endregion 🔖MediaExport

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
	describe("LayoutPlayController", () => {
		it("default document json is valid", () => {
			expect(parseLayoutDocumentJson(DEFAULT_LAYOUT_DOCUMENT_JSON)?.pages.length).toBe(2);
		});
		it("layout exposes blueprint and preview windows", () => {
			expect(LAYOUT_PLAY_LAYOUT.root.kind).toBe("row");
		});
	});
	describe("buildLayoutPlayHierarchyTree", () => {
		it("builds hierarchy sections from default document", () => {
			const tree = buildLayoutPlayHierarchyTree(DEFAULT_LAYOUT_DOCUMENT_JSON, []);
			expect(tree.type).toBe("tree");
			if (tree.type !== "tree") return;
			const sectionIds = tree.sections.map((section) => section.id);
			expect(sectionIds).toContain("layout-hierarchy.spreads");
			expect(sectionIds).toContain("layout-hierarchy.pages");
			expect(sectionIds).toContain("layout-hierarchy.frames");
			expect(sectionIds).toContain("layout-hierarchy.parentPages");
			expect(sectionIds).toContain("layout-hierarchy.layers");
			const spreads = tree.sections.find((section) => section.id === "layout-hierarchy.spreads");
			expect(spreads?.items[0]?.description).toBe("page-1, page-2");
			const pages = tree.sections.find((section) => section.id === "layout-hierarchy.pages");
			expect(pages?.items.find((item) => item.id === "layout-hierarchy.page.page-1")?.description).toBe("parent: parent-master");
			const layers = tree.sections.find((section) => section.id === "layout-hierarchy.layers");
			expect(layers?.items.some((item) => item.description === "3 objects")).toBe(true);
			const frames = tree.sections.find((section) => section.id === "layout-hierarchy.frames");
			expect(frames?.items.some((item) => item.id === "layout-hierarchy.frame.frame-text-1")).toBe(true);
		});
	});
	describe("buildLayoutPlayCatalogueTree", () => {
		it("exposes draggable catalogue kinds", () => {
			const tree = buildLayoutPlayCatalogueTree();
			expect(tree.type).toBe("tree");
			if (tree.type !== "tree") return;
			const rect = tree.sections[0]?.items.find((item) => item.id === "layout-play-catalogue.rect");
			expect(rect?.draggable).toBe(true);
			expect(rect?.dragData?.[LAYOUT_CATALOGUE_KIND_DRAG_MIME]).toContain("\"kind\":\"rect\"");
		});
	});
	describe("buildLayoutPlayInspectorTree", () => {
		it("returns editable fields for selected frame", () => {
			const tree = buildLayoutPlayInspectorTree(DEFAULT_LAYOUT_DOCUMENT_JSON, ["frame-text-1"]);
			expect(tree.type).toBe("tree");
			if (tree.type !== "tree") return;
			const field = tree.sections[0]?.items.find((item) => item.id === "layout-inspector.frame.bounds.x");
			expect(field?.control?.type).toBe("input");
			if (field?.control?.type === "input") {
				expect(field.control.inputKind).toBe("number");
				expect(field.control.onChange?.command).toBe("patchFrame");
			}
		});
	});
	describe("LayoutPlayController commands", () => {
		it("addFrame selects the created frame", () => {
			const bus = new CommandBus();
			const ctrl = new LayoutPlayController(bus, () => {});
			ctrl.run("addFrame", { kind: "rect", pageId: "page-1", layerId: "layer-1" });
			const selected = ctrl.getSelectedIds();
			expect(selected.length).toBe(1);
			expect(findFrame(ctrl.getDocument(), selected[0] ?? "")?.kind).toBe("rect");
		});
		it("setHover stores hovered id", () => {
			const bus = new CommandBus();
			const ctrl = new LayoutPlayController(bus, () => {});
			ctrl.run("setHover", { id: "frame-text-1", sourceId: "hierarchy" });
			expect(ctrl.getHoveredId()).toBe("frame-text-1");
		});
	});
}

//#region 🔖Play

/** @emoji 🛝 Layout playground app. */


export const layoutPlayAppDefinition = createPlaygroundApp({
	id: LAYOUT_PLAY_APP_ID,
	label: "Layout",
	controllerId: "layout-play",
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	devHost: {
		playEntryKind: "layout",
		resolveDedupe: ["react", "react-dom", "@semio-tech/layout-react"],
		watchIgnored: ["../rs/lib.rs", "../rs/target/**", "../rs/pkg/**"],
		optimizeDeps: { include: ["react", "react-dom"] },
	},
	createRuntime: () => {
		const runtime = createProductPlaygroundPlatform(LAYOUT_PLAY_APP_ID);
			const ctrl = new LayoutPlayController(runtime.commandBus, () => runtime.notify());
			runtime.addApp(buildLayoutPlayAppRuntime(ctrl));
			return runtime;
	},
	registerBodies: () => {
		registerLayoutPlayDeclarativeBodies();
	},
	bootRenderer: async (pg) => {
		const { bootLayoutPlay } = await import("@semio-tech/framework-playground-renderer-react/layout");
		await bootLayoutPlay(pg);
	},
});
//#endregion 🔖Play
