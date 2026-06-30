// #region 🧲Header
/** @emoji ✏️ Draw play harness on `@semio-tech/framework-playground-core`. */
// #endregion 🧲Header

import {
	AppRuntime,
	CommandBus,
	Controller,
	ModeRuntime,
	Platform,
	Playground,
	WindowKindRuntime,
	buildDrawWindowBody,
	createDefaultLayout,
	createPlayAppRuntime,
	createProductPlaygroundPlatform,
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
	type UiNode,
	type UiSectionNode,
	type UiTreeItemNode,
	type UiTreeNode,
} from "@semio-tech/framework-playground-core";
import { bootstrapElementsSurfaceChromeDocument, type TreeDataItem, type TreeDragAndDropController, type TreeDropPosition } from "@semio-tech/ui-react";
import {
	applyDrawEditOp,
	createDrawBooleanLayer,
	createDrawGroupLayer,
	createDrawPathLayer,
	createDrawTraceLayer,
	defaultDrawDocument,
	drawDocumentFromJson,
	drawDocumentToJson,
	drawPlayHoverPayloadFromTreeRowId,
	drawPlayLayerIdFromTreeRowId,
	drawPlayLayersTreeHighlightedIds,
	drawPlayLayersTreeRowId,
	findDrawLayer,
	flattenDrawLayers,
	resolveDrawPlayReorderTarget,
	type DrawBlendMode,
	type DrawBooleanOp,
	type DrawDocument,
	type DrawHoverPayload,
	type DrawKindHover,
	type DrawLayerNode,
	type DrawToolId,
	DRAW_BLEND_MODES,
	DRAW_BOOLEAN_OPS,
} from "@semio-tech/draw-core";
import { DRAW_PLAY_FIXTURE_DEFAULT_ID, resolveDrawPlayFixtureSlug } from "./fixture-slugs.ts";

export const DRAW_PLAY_APP_ID = "draw-play";
export const DRAW_PLAY_CONTROLLER_ID = "draw-play";
export const DRAW_PLAY_SURFACE_ID_COMPOSITE = "draw.play.composite/v1";
export const DRAW_PLAY_SURFACE_ID_NAVIGATOR = "draw.play.navigator/v1";
export const DRAW_PLAY_BODY_KEY_COMPOSITE = "draw.play.composite";
export const DRAW_PLAY_BODY_KEY_NAVIGATOR = "draw.play.navigator";
export const DRAW_PLAY_WINDOW_KIND_COMPOSITE = "draw-composite";
export const DRAW_PLAY_WINDOW_KIND_NAVIGATOR = "draw-navigator";
export const DRAW_PLAY_LAYERS_TAB_ID = "framework.panel.hierarchy";
export const DRAW_PLAY_CATALOGUE_TAB_ID = "framework.panel.catalogue";
export const DRAW_PLAY_PROPERTIES_TAB_ID = "framework.panel.inspection";

export const DRAW_LAYER_KIND_DRAG_MIME = "application/x-semio-draw-layer-kind";

type DrawCatalogueLayerKind = "path" | "group" | "boolean" | "trace";

export const DRAW_PLAY_LAYOUT = createDefaultLayout(
	[DRAW_PLAY_WINDOW_KIND_COMPOSITE, DRAW_PLAY_WINDOW_KIND_NAVIGATOR],
	"row",
	[72, 28],
	["Canvas", "Navigator"],
);

export { DRAW_PLAY_FIXTURE_DEFAULT_ID, resolveDrawPlayFixtureSlug };

const drawFixtureModules = import.meta.glob("../fixture/*.draw.json", { eager: true }) as Record<string, { default: unknown }>;

function drawFixtureIdFromGlobPath(globPath: string): string {
	const base = globPath.split("/").pop() ?? globPath;
	return base.replace(/\.draw\.json$/, "");
}

function drawFixtureLabelFromId(id: string): string {
	return id
		.split("-")
		.filter(Boolean)
		.map((word) => word.charAt(0).toUpperCase() + word.slice(1))
		.join(" ");
}

const DRAW_PLAY_FILE_FIXTURE_JSON_BY_ID: Record<string, string> = Object.fromEntries(
	Object.entries(drawFixtureModules).map(([path, mod]) => {
		const id = drawFixtureIdFromGlobPath(path);
		const json = typeof mod.default === "string" ? mod.default : JSON.stringify(mod.default);
		return [id, json];
	}),
);

export const DRAW_PLAY_FIXTURE_OPTIONS: ReadonlyArray<{ readonly id: string; readonly label: string }> = Object.keys(
	DRAW_PLAY_FILE_FIXTURE_JSON_BY_ID,
)
	.sort()
	.map((id) => ({ id, label: drawFixtureLabelFromId(id) }));

export const DRAW_PLAY_EMPTY_DOCUMENT: DrawDocument = defaultDrawDocument("empty");

function drawPlayCmd(command: string, args: Record<string, unknown> = {}): { controllerId: string; command: string; args: Record<string, unknown> } {
	return { controllerId: DRAW_PLAY_CONTROLLER_ID, command, args };
}

export interface DrawPlayHierarchyBuildOptions {
	readonly onToggleVisible?: (layerId: string) => void;
	readonly onDeleteLayer?: (layerId: string) => void;
	readonly onDuplicateLayer?: (layerId: string) => void;
}

function drawPlayLayerChrome(
	layer: DrawLayerNode,
	options?: DrawPlayHierarchyBuildOptions,
): Pick<UiTreeItemNode, "isHidden" | "actions" | "contextMenu"> {
	const contextMenu = [];
	if (options?.onToggleVisible) {
		contextMenu.push({
			id: "visible",
			label: layer.visible ? "Hide" : "Show",
			icon: layer.visible ? "eye-off" : "eye",
			onSelect: () => options.onToggleVisible!(layer.id),
		});
	}
	if (options?.onDuplicateLayer) {
		contextMenu.push({
			id: "duplicate",
			label: "Duplicate",
			icon: "copy",
			onSelect: () => options.onDuplicateLayer!(layer.id),
		});
	}
	if (options?.onDeleteLayer) {
		contextMenu.push({
			id: "delete",
			label: "Delete",
			icon: "trash-2",
			onSelect: () => options.onDeleteLayer!(layer.id),
		});
	}
	return {
		isHidden: !layer.visible,
		actions: options?.onToggleVisible
			? [
					{
						id: "visible",
						icon: layer.visible ? "eye" : "eye-off",
						title: layer.visible ? "Hide" : "Show",
						onClick: () => options.onToggleVisible!(layer.id),
						revealOnHover: layer.visible,
					},
				]
			: undefined,
		contextMenu: contextMenu.length > 0 ? contextMenu : undefined,
	};
}

function drawPlayLayerTreeItem(
	doc: DrawDocument,
	layer: DrawLayerNode,
	options?: DrawPlayHierarchyBuildOptions,
	hoverSink?: (payload: DrawHoverPayload) => void,
): UiTreeItemNode {
	const rowId = drawPlayLayersTreeRowId(layer);
	const nestedItems = layer.kind === "group" ? layer.children.map((child) => drawPlayLayerTreeItem(doc, child, options, hoverSink)) : undefined;
	return {
		id: rowId,
		label: layer.name,
		description: layer.kind === "boolean" ? layer.op : layer.blendMode,
		icon:
			layer.kind === "group"
				? "folder"
				: layer.kind === "boolean"
					? "combine"
					: layer.kind === "trace"
						? "scan-line"
						: layer.kind === "path"
							? "pen-tool"
							: "shapes",
		defaultOpen: layer.kind === "group",
		draggable: true,
		dragData: { "application/x-semio-draw-layer-id": layer.id },
		command: drawPlayCmd("setSelection", { ids: [layer.id] }),
		items: nestedItems,
		...drawPlayLayerChrome(layer, options),
		onPointerEnter: hoverSink ? () => hoverSink(drawPlayHoverPayloadFromTreeRowId(doc, rowId)) : undefined,
		onPointerLeave: hoverSink ? () => hoverSink({ id: null, kind: null }) : undefined,
	};
}

export function buildDrawPlayLayersTree(
	doc: DrawDocument,
	selectedIds: readonly string[],
	hoveredId: string | null,
	kindHover: DrawKindHover | null,
	hoverSink?: (payload: DrawHoverPayload) => void,
	options?: DrawPlayHierarchyBuildOptions,
): UiTreeNode {
	const highlightedIds = drawPlayLayersTreeHighlightedIds(doc, hoveredId, kindHover);
	const selectedTreeIds = selectedIds
		.map((id) => findDrawLayer(doc, id))
		.filter((layer): layer is DrawLayerNode => Boolean(layer))
		.map((layer) => drawPlayLayersTreeRowId(layer));
	const toolbarItems: UiTreeItemNode[] = [
		{ id: "draw-play-layers.add.path", label: "Add Path", icon: "pen-tool", command: drawPlayCmd("addLayer", { kind: "path" }) },
		{ id: "draw-play-layers.add.group", label: "Add Group", icon: "folder-plus", command: drawPlayCmd("addLayer", { kind: "group" }) },
		{ id: "draw-play-layers.add.boolean", label: "Add Boolean", icon: "combine", command: drawPlayCmd("addLayer", { kind: "boolean" }) },
	];
	const layerItems =
		doc.layers.length > 0
			? doc.layers.map((layer) => drawPlayLayerTreeItem(doc, layer, options, hoverSink))
			: [{ id: "draw-play-layers.empty", label: "Drop layers here", icon: "pen-tool" as const }];
	return {
		type: "tree",
		sections: [{ id: "draw-play-layers", label: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, defaultOpen: true, items: [...toolbarItems, ...layerItems] }],
		selectedIds: selectedTreeIds,
		highlightedIds: [...highlightedIds],
	};
}

export function buildDrawPlayCatalogueTree(
	selectedIds: readonly string[],
	hoverSink?: (payload: DrawHoverPayload) => void,
): UiTreeNode {
	const items: UiTreeItemNode[] = [
		{
			id: "draw-play-catalogue.path",
			label: "Path",
			icon: "pen-tool",
			draggable: true,
			dragData: { [DRAW_LAYER_KIND_DRAG_MIME]: JSON.stringify({ kind: "path" }) },
		},
		{
			id: "draw-play-catalogue.group",
			label: "Group",
			icon: "folder",
			draggable: true,
			dragData: { [DRAW_LAYER_KIND_DRAG_MIME]: JSON.stringify({ kind: "group" }) },
		},
		{
			id: "draw-play-catalogue.boolean",
			label: "Boolean",
			icon: "combine",
			draggable: true,
			dragData: { [DRAW_LAYER_KIND_DRAG_MIME]: JSON.stringify({ kind: "boolean" }) },
		},
		{
			id: "draw-play-catalogue.trace",
			label: "Trace",
			icon: "scan-line",
			draggable: true,
			dragData: { [DRAW_LAYER_KIND_DRAG_MIME]: JSON.stringify({ kind: "trace" }) },
		},
		...DRAW_BOOLEAN_OPS.map((op) => ({
			id: `draw-play-catalogue.bool.${op}`,
			label: `Boolean ${op}`,
			icon: "combine" as const,
			command: drawPlayCmd("combineBoolean", { op, ids: selectedIds }),
			onPointerEnter: hoverSink ? () => hoverSink({ id: null, kind: { domain: "boolean", kindId: op } }) : undefined,
			onPointerLeave: hoverSink ? () => hoverSink({ id: null, kind: null }) : undefined,
		})),
	];
	return {
		type: "tree",
		sections: [{ id: "draw-play-catalogue", label: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, defaultOpen: true, items }],
	};
}

export function buildDrawPlayInspectorTree(doc: DrawDocument, selectedIds: readonly string[]): UiNode {
	const layerId = selectedIds[0];
	const layer = layerId ? findDrawLayer(doc, layerId) : undefined;
	if (!layer) {
		return uiDeclarativeSectionsToTree([
			{
				type: "section",
				id: "draw-play-inspector.empty",
				label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
				children: [{ type: "text", value: "Select a layer in the hierarchy." }],
			},
		]);
	}
	const children: UiSectionNode["children"] = [
		{
			type: "field",
			id: "draw-play-inspector.name",
			label: "Name",
			child: {
				type: "input",
				id: "draw-play-inspector.name.input",
				inputKind: "text",
				value: layer.name,
				onChange: drawPlayCmd("patchLayer", { layerId: layer.id, field: "name" }),
			},
		},
		{
			type: "field",
			id: "draw-play-inspector.opacity",
			label: "Opacity",
			child: {
				type: "slider",
				id: "draw-play-inspector.opacity.slider",
				value: layer.opacity,
				min: 0,
				max: 1,
				step: 0.01,
				onChange: drawPlayCmd("patchLayer", { layerId: layer.id, field: "opacity" }),
			},
		},
		{
			type: "field",
			id: "draw-play-inspector.blend",
			label: "Blend",
			child: {
				type: "select",
				id: "draw-play-inspector.blend.select",
				value: layer.blendMode,
				items: DRAW_BLEND_MODES.map((mode) => ({ value: mode, label: mode })),
				onChange: drawPlayCmd("patchLayer", { layerId: layer.id, field: "blendMode" }),
			},
		},
		{
			type: "field",
			id: "draw-play-inspector.visible",
			label: "Visible",
			child: {
				type: "toggle",
				id: "draw-play-inspector.visible.toggle",
				pressed: layer.visible,
				onChange: drawPlayCmd("patchLayer", { layerId: layer.id, field: "visible" }),
			},
		},
	];
	if (layer.kind === "boolean") {
		children.push({
			type: "field",
			id: "draw-play-inspector.boolean-op",
			label: "Boolean Op",
			child: {
				type: "select",
				id: "draw-play-inspector.boolean-op.select",
				value: layer.op,
				items: DRAW_BOOLEAN_OPS.map((op) => ({ value: op, label: op })),
				onChange: drawPlayCmd("patchLayer", { layerId: layer.id, field: "booleanOp" }),
			},
		});
	}
	if (layer.kind === "trace") {
		children.push(
			{
				type: "field",
				id: "draw-play-inspector.trace-threshold",
				label: "Trace Threshold",
				child: {
					type: "slider",
					id: "draw-play-inspector.trace-threshold.slider",
					value: layer.params.threshold,
					min: 0,
					max: 1,
					step: 0.01,
					onChange: drawPlayCmd("patchLayer", { layerId: layer.id, field: "traceThreshold" }),
				},
			},
			{
				type: "field",
				id: "draw-play-inspector.trace-simplify",
				label: "Simplify",
				child: {
					type: "slider",
					id: "draw-play-inspector.trace-simplify.slider",
					value: layer.params.simplifyEpsilon,
					min: 0,
					max: 10,
					step: 0.1,
					onChange: drawPlayCmd("patchLayer", { layerId: layer.id, field: "traceSimplify" }),
				},
			},
		);
	}
	return uiDeclarativeSectionsToTree([
		{
			type: "section",
			id: "draw-play-inspector.layer",
			label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
			defaultOpen: true,
			children,
		},
	]);
}

function drawPlayCreateLayerByKind(kind: DrawCatalogueLayerKind): DrawLayerNode {
	if (kind === "group") return createDrawGroupLayer();
	if (kind === "boolean") return createDrawBooleanLayer();
	if (kind === "trace") return createDrawTraceLayer("Trace", "emblem-trace-source");
	return createDrawPathLayer();
}

export function createDrawPlayHierarchyTreeDragController(
	getController: () => DrawPlayController | undefined,
): TreeDragAndDropController {
	return {
		handleDrop: ({ target, targetKind, data, sourceItems, dropPosition }) => {
			const catalogueRaw = data[DRAW_LAYER_KIND_DRAG_MIME];
			if (catalogueRaw) {
				const parsed = JSON.parse(catalogueRaw) as { kind?: DrawCatalogueLayerKind };
				if (parsed.kind) {
					const targetRowId = targetKind === "item" ? (target as TreeDataItem).id : "draw-play-layers";
					getController()?.run("dropLayerKind", { kind: parsed.kind, targetRowId, dropPosition: dropPosition ?? "inside" });
				}
				return;
			}
			const sourceItem = sourceItems[0];
			if (!sourceItem || targetKind !== "item") return;
			const layerId = sourceItem.dragData?.["application/x-semio-draw-layer-id"] ?? drawPlayLayerIdFromTreeRowId(sourceItem.id);
			if (!layerId) return;
			getController()?.run("moveLayer", {
				layerId,
				targetRowId: (target as TreeDataItem).id,
				dropPosition: dropPosition ?? "after",
			});
		},
	};
}

export interface DrawPlayHostBridge {
	runHostCommand(command: string, args?: unknown): void;
}

export class DrawPlayController extends Controller implements PlaygroundFixtureHost {
	readonly mainMode = new ModeRuntime("main", "Draw", undefined);
	private document: DrawDocument = DRAW_PLAY_EMPTY_DOCUMENT;
	private selectedIds: string[] = [];
	private hoveredId: string | null = null;
	private hoveredKind: DrawKindHover | null = null;
	private interactionRevision = 0;
	private listeners = new Set<() => void>();
	private hostBridge: DrawPlayHostBridge | null = null;

	constructor(bus: CommandBus, notifyPlatform: () => void) {
		super(DRAW_PLAY_CONTROLLER_ID, bus, notifyPlatform);
		this.rebuildShellMode();
	}

	private rebuildShellMode(): void {
		this.mainMode.tools = DRAW_PLAY_TOOLS;
		this.mainMode.windowKinds = [
			new WindowKindRuntime(DRAW_PLAY_WINDOW_KIND_COMPOSITE, "Canvas", DRAW_PLAY_BODY_KEY_COMPOSITE),
			new WindowKindRuntime(DRAW_PLAY_WINDOW_KIND_NAVIGATOR, "Navigator", DRAW_PLAY_BODY_KEY_NAVIGATOR),
		];
	}

	subscribe(listener: () => void): () => void {
		this.listeners.add(listener);
		return () => this.listeners.delete(listener);
	}

	private bump(): void {
		this.interactionRevision += 1;
		for (const listener of this.listeners) listener();
		this.emit();
	}

	getInteractionRevision(): number {
		return this.interactionRevision;
	}

	getDocument(): DrawDocument {
		return this.document;
	}

	getDocumentJson(): string {
		return drawDocumentToJson(this.document);
	}

	setHostBridge(bridge: DrawPlayHostBridge | null): void {
		this.hostBridge = bridge;
	}

	private applyDocument(doc: DrawDocument, resetSelection = false): void {
		this.document = doc;
		if (resetSelection) this.selectedIds = doc.layers[0] ? [doc.layers[0].id] : [];
		this.bump();
	}

	getSelectedIds(): readonly string[] {
		return this.selectedIds;
	}

	getHoveredId(): string | null {
		return this.hoveredId;
	}

	getHoveredKind(): DrawKindHover | null {
		return this.hoveredKind;
	}

	getFixtureCatalog(): PlaygroundFixtureCatalog | null {
		if (isPlaygroundFixtureLocked()) return null;
		return {
			activeFixtureId: playgroundResolvedFixtureId(
				this.document.id === "empty" ? PLAYGROUND_NO_FIXTURE_ID : this.document.id,
				DRAW_PLAY_FIXTURE_DEFAULT_ID,
			),
			options: DRAW_PLAY_FIXTURE_OPTIONS,
		};
	}

	private patchDocument(edit: (doc: DrawDocument) => DrawDocument, selectLayerId?: string): void {
		this.document = edit(this.document);
		if (selectLayerId) this.selectedIds = [selectLayerId];
		this.bump();
	}

	run(command: string, args: Record<string, unknown> = {}): void {
		switch (command) {
			case "setActiveFixture": {
				const fixtureId = String(args.fixtureId ?? "");
				if (isPlaygroundNoFixtureId(fixtureId)) {
					this.document = DRAW_PLAY_EMPTY_DOCUMENT;
					this.selectedIds = [];
					this.bump();
					return;
				}
				const json = DRAW_PLAY_FILE_FIXTURE_JSON_BY_ID[fixtureId];
				if (json) {
					this.applyDocument(drawDocumentFromJson(json), true);
					console.log("[DEBUG] draw fixture loaded", fixtureId);
				}
				return;
			}
			case "setFixtureJson": {
				const json = typeof args.json === "string" ? args.json : "";
				if (!json.includes("draw.document/v1")) {
					console.log("[DEBUG] draw import rejected: not a draw document");
					return;
				}
				this.applyDocument(drawDocumentFromJson(json), args.resetInteraction !== false);
				console.log("[DEBUG] draw document imported");
				return;
			}
			case "saveDownload":
			case "loadRequest":
				this.hostBridge?.runHostCommand(command, args);
				return;
			case "setSelection": {
				this.selectedIds = Array.isArray(args.ids) ? args.ids.map(String) : [];
				console.log("[DEBUG] draw selection", this.selectedIds);
				this.bump();
				return;
			}
			case "setHover": {
				this.hoveredId = typeof args.id === "string" ? args.id : null;
				this.hoveredKind = (args.kind as DrawKindHover | null) ?? null;
				this.bump();
				return;
			}
			case "setActiveTool": {
				this.document = applyDrawEditOp(this.document, { op: "setActiveTool", tool: String(args.tool) as DrawToolId });
				this.bump();
				return;
			}
			case "addLayer": {
				const kind = String(args.kind ?? "path") as DrawCatalogueLayerKind;
				const layer = drawPlayCreateLayerByKind(kind);
				const op =
					kind === "group"
						? ({ op: "addGroupLayer", layer } as const)
						: kind === "boolean"
							? ({ op: "addBooleanLayer", layer } as const)
							: kind === "trace"
								? ({ op: "addTraceLayer", layer } as const)
								: ({ op: "addPathLayer", layer } as const);
				this.patchDocument((doc) => applyDrawEditOp(doc, op), layer.id);
				return;
			}
			case "dropLayerKind": {
				const kind = String(args.kind ?? "") as DrawCatalogueLayerKind;
				const targetRowId = String(args.targetRowId ?? "draw-play-layers");
				const dropPosition = (args.dropPosition ?? "inside") as TreeDropPosition;
				const layer = drawPlayCreateLayerByKind(kind);
				const target = resolveDrawPlayReorderTarget(this.document, targetRowId, dropPosition === "before" || dropPosition === "after" ? dropPosition : "inside");
				const parentId = target?.parentId;
				const index = target?.index ?? this.document.layers.length;
				const op =
					kind === "group"
						? ({ op: "addGroupLayer", parentId, index, layer } as const)
						: kind === "boolean"
							? ({ op: "addBooleanLayer", parentId, index, layer } as const)
							: kind === "trace"
								? ({ op: "addTraceLayer", parentId, index, layer } as const)
								: ({ op: "addPathLayer", parentId, index, layer } as const);
				this.patchDocument((doc) => applyDrawEditOp(doc, op), layer.id);
				return;
			}
			case "moveLayer": {
				const layerId = String(args.layerId ?? "");
				const targetRowId = String(args.targetRowId ?? "");
				const dropPosition = (args.dropPosition ?? "after") as TreeDropPosition;
				const target = resolveDrawPlayReorderTarget(this.document, targetRowId, dropPosition);
				if (!target || !layerId) return;
				this.patchDocument((doc) => applyDrawEditOp(doc, { op: "reorderLayer", layerId, parentId: target.parentId, index: target.index }));
				return;
			}
			case "deleteLayer": {
				const layerId = String(args.layerId ?? "");
				this.document = applyDrawEditOp(this.document, { op: "removeLayer", layerId });
				this.selectedIds = this.selectedIds.filter((id) => id !== layerId);
				this.bump();
				return;
			}
			case "duplicateLayer": {
				this.document = applyDrawEditOp(this.document, { op: "duplicateLayer", layerId: String(args.layerId) });
				this.bump();
				return;
			}
			case "toggleLayerVisible": {
				const layerId = String(args.layerId ?? "");
				const layer = findDrawLayer(this.document, layerId);
				if (!layer) return;
				this.patchDocument((doc) => applyDrawEditOp(doc, { op: "setLayerVisible", layerId, visible: !layer.visible }));
				return;
			}
			case "combineBoolean": {
				const ids = Array.isArray(args.ids) ? args.ids.map(String) : this.selectedIds;
				if (ids.length < 2) return;
				const layer = createDrawBooleanLayer("Boolean", String(args.op ?? "union") as DrawBooleanOp, ids);
				this.patchDocument((doc) => applyDrawEditOp(doc, { op: "addBooleanLayer", layer }), layer.id);
				return;
			}
			case "patchLayer": {
				const layerId = String(args.layerId ?? "");
				const field = String(args.field ?? "");
				const value = args.value ?? args.pressed;
				if (!layerId || !field) return;
				this.patchDocument((doc) => {
					switch (field) {
						case "name":
							return applyDrawEditOp(doc, { op: "setLayerName", layerId, name: String(value ?? "") });
						case "opacity":
							return applyDrawEditOp(doc, { op: "setLayerOpacity", layerId, opacity: Number(value) });
						case "blendMode":
							return applyDrawEditOp(doc, { op: "setLayerBlendMode", layerId, blendMode: String(value) as DrawBlendMode });
						case "visible":
							return applyDrawEditOp(doc, { op: "setLayerVisible", layerId, visible: Boolean(value) });
						case "booleanOp":
							return applyDrawEditOp(doc, { op: "setBooleanOp", layerId, booleanOp: String(value) as DrawBooleanOp });
						case "traceThreshold": {
							const layer = findDrawLayer(doc, layerId);
							if (!layer || layer.kind !== "trace") return doc;
							return applyDrawEditOp(doc, {
								op: "setTraceParams",
								layerId,
								params: { ...layer.params, threshold: Number(value) },
							});
						}
						case "traceSimplify": {
							const layer = findDrawLayer(doc, layerId);
							if (!layer || layer.kind !== "trace") return doc;
							return applyDrawEditOp(doc, {
								op: "setTraceParams",
								layerId,
								params: { ...layer.params, simplifyEpsilon: Number(value) },
							});
						}
						default:
							return doc;
					}
				});
				return;
			}
			case "setCamera": {
				const camera = args.camera as DrawDocument["camera"];
				if (camera) this.patchDocument((doc) => applyDrawEditOp(doc, { op: "setCamera", camera }));
				return;
			}
			case "selectAll": {
				this.selectedIds = flattenDrawLayers(this.document.layers).map((layer) => layer.id);
				this.bump();
				return;
			}
			default:
				return;
		}
	}
}

function drawPlayTool(id: string, label: string, iconId: string, command: string, args?: Record<string, unknown>): ToolLeaf {
	return { id, kind: "button", label, iconId, controllerId: DRAW_PLAY_CONTROLLER_ID, command, args };
}

export const DRAW_PLAY_TOOLS: AppTools = [
	toolCollection("open", "folder-open", [drawPlayTool("draw-import", "Import Draw", "folder-open", "loadRequest")]),
	toolCollection("save", "save", [drawPlayTool("draw-export", "Export Draw", "save", "saveDownload")]),
	toolCollection("selection", "mouse-pointer-2", [
		drawPlayTool("selectDirect", "Direct", "mouse-pointer", "setActiveTool", { tool: "selectDirect" }),
		drawPlayTool("selectMarquee", "Marquee", "square-dashed", "setActiveTool", { tool: "selectMarquee" }),
	]),
	toolCollection("draw", "pen-tool", [
		drawPlayTool("pen", "Pen", "pen-tool", "setActiveTool", { tool: "pen" }),
		drawPlayTool("shapeRect", "Rectangle", "square", "setActiveTool", { tool: "shapeRect" }),
	]),
	toolCollection("boolean", "combine", [
		drawPlayTool("booleanCombine", "Combine", "combine", "combineBoolean", { op: "union" }),
	]),
	toolCollection("trace", "scan-line", [drawPlayTool("trace", "Trace", "scan-line", "setActiveTool", { tool: "trace" })]),
	toolCollection("transform", "move", [drawPlayTool("transformMove", "Pan", "move", "setActiveTool", { tool: "transformMove" })]),
];

export function buildDrawPlayAppRuntime(ctrl: DrawPlayController): AppRuntime {
	return createPlayAppRuntime(DRAW_PLAY_APP_ID, "Draw", ctrl, DRAW_PLAY_LAYOUT, ctrl.mainMode);
}

export function registerDrawPlayDeclarativeBodies(): void {
	registerWindowBody(DRAW_PLAY_BODY_KEY_COMPOSITE, () =>
		buildDrawWindowBody(DRAW_PLAY_SURFACE_ID_COMPOSITE, DRAW_PLAY_CONTROLLER_ID, "composite", "composite"));
	registerWindowBody(DRAW_PLAY_BODY_KEY_NAVIGATOR, () =>
		buildDrawWindowBody(DRAW_PLAY_SURFACE_ID_NAVIGATOR, DRAW_PLAY_CONTROLLER_ID, "navigator", "navigator"));
}

export class PlaygroundDraw extends Playground {
	readonly id = DRAW_PLAY_APP_ID;
	readonly keybindings = [{ key: "ctrl+a,meta+a", controllerId: DRAW_PLAY_CONTROLLER_ID, command: "selectAll" }];

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id);
		const ctrl = new DrawPlayController(runtime.commandBus, () => runtime.notify());
		const resolved = playgroundResolvedFixtureId(DRAW_PLAY_FIXTURE_DEFAULT_ID);
		const fixtureJson = DRAW_PLAY_FILE_FIXTURE_JSON_BY_ID[resolved];
		if (fixtureJson) ctrl.run("setActiveFixture", { fixtureId: resolved });
		runtime.addApp(buildDrawPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerDrawPlayDeclarativeBodies();
	}
}

if (typeof document !== "undefined" && document.getElementById("root") != null && !import.meta.vitest && import.meta.env.PUZZLE_PLAY_ENTRY === "draw") {
	bootstrapElementsSurfaceChromeDocument("system");
	void (async () => {
		await import("./globals.css");
		const { bootDrawPlay } = await import("@semio-tech/framework-playground-renderer-react/draw");
		bootDrawPlay(new PlaygroundDraw());
	})();
}

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("DRAW_PLAY_FIXTURE_OPTIONS", () => {
		it("includes semio fixture", () => {
			expect(DRAW_PLAY_FIXTURE_OPTIONS.some((row) => row.id === "semio")).toBe(true);
		});
	});

	describe("buildDrawPlayLayersTree", () => {
		it("builds hierarchy for default document", () => {
			const doc = defaultDrawDocument("test");
			const tree = buildDrawPlayLayersTree(doc, [], null, null);
			expect(tree.sections[0]?.items.length).toBeGreaterThan(0);
		});
	});
}
// #endregion 🧪Tests
