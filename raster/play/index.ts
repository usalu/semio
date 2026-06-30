// #region 🧲Header
/** @emoji 🖼️ Raster play harness on `@semio-tech/framework-playground-core`. */
// #endregion 🧲Header

import {
	AppRuntime,
	CommandBus,
	Controller,
	ModeRuntime,
	Platform,
	Playground,
	WindowKindRuntime,
	buildRasterWindowBody,
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
	type UiTreeContextMenuItem,
	type UiTreeItemNode,
	type UiTreeNode,
} from "@semio-tech/framework-playground-core";
import { bootstrapElementsSurfaceChromeDocument, type TreeDataItem, type TreeDragAndDropController, type TreeDropPosition } from "@semio-tech/ui-react";
import {
	applyRasterEditOp,
	createRasterAdjustmentLayer,
	createRasterGroupLayer,
	createRasterPixelLayer,
	defaultRasterDocument,
	findRasterLayer,
	flattenRasterLayers,
	parseRasterDocument,
	rasterDocumentFromJson,
	rasterDocumentToExportJson,
	rasterPlayBlendModeTreeRowId,
	rasterPlayHoverPayloadFromTreeRowId,
	rasterPlayLayerIdFromTreeRowId,
	rasterPlayLayersTreeHighlightedIds,
	rasterPlayLayersTreeRowId,
	rasterPlayMaskTreeRowId,
	resolveRasterPlayReorderTarget,
	type RasterBlendMode,
	type RasterDocument,
	type RasterHoverPayload,
	type RasterKindHover,
	type RasterLayerNode,
	type RasterToolId,
	RASTER_ADJUSTMENT_KINDS,
	RASTER_BLEND_MODES,
	RASTER_FILTER_KINDS,
} from "@semio-tech/raster-core";
import { RASTER_PLAY_FIXTURE_DEFAULT_ID, resolveRasterPlayFixtureSlug } from "./fixture-slugs.ts";

export const RASTER_PLAY_APP_ID = "raster-play";
export const RASTER_PLAY_CONTROLLER_ID = "raster-play";
export const RASTER_PLAY_SURFACE_ID_COMPOSITE = "raster.play.composite/v1";
export const RASTER_PLAY_SURFACE_ID_NAVIGATOR = "raster.play.navigator/v1";
export const RASTER_PLAY_SURFACE_ID_LAYER_PREFIX = "raster.play.layer.";
export const RASTER_PLAY_SURFACE_ID_MASK_PREFIX = "raster.play.mask.";
export const RASTER_PLAY_BODY_KEY_COMPOSITE = "raster.play.composite";
export const RASTER_PLAY_BODY_KEY_NAVIGATOR = "raster.play.navigator";
export const RASTER_PLAY_WINDOW_KIND_COMPOSITE = "raster-composite";
export const RASTER_PLAY_WINDOW_KIND_NAVIGATOR = "raster-navigator";
export const RASTER_PLAY_LAYERS_TAB_ID = "framework.panel.hierarchy";
export const RASTER_PLAY_CATALOGUE_TAB_ID = "framework.panel.catalogue";
export const RASTER_PLAY_MASKS_TAB_ID = "raster.panel.masks";
export const RASTER_PLAY_PROPERTIES_TAB_ID = "framework.panel.inspection";

export const RASTER_LAYER_KIND_DRAG_MIME = "application/x-semio-raster-layer-kind";

type RasterCatalogueLayerKind = "pixel" | "group" | "adjustment";

export const RASTER_PLAY_LAYOUT = createDefaultLayout(
	[RASTER_PLAY_WINDOW_KIND_COMPOSITE, RASTER_PLAY_WINDOW_KIND_NAVIGATOR],
	"row",
	[72, 28],
	["Composite", "Navigator"],
);

export { RASTER_PLAY_FIXTURE_DEFAULT_ID, resolveRasterPlayFixtureSlug };

const rasterFixtureModules = import.meta.glob("../fixture/*.raster.json", { eager: true }) as Record<string, { default: unknown }>;

function rasterFixtureIdFromGlobPath(globPath: string): string {
	const base = globPath.split("/").pop() ?? globPath;
	return base.replace(/\.raster\.json$/, "");
}

function rasterFixtureLabelFromId(id: string): string {
	return id
		.split("-")
		.filter(Boolean)
		.map((word) => word.charAt(0).toUpperCase() + word.slice(1))
		.join(" ");
}

const RASTER_PLAY_FILE_FIXTURE_JSON_BY_ID: Record<string, string> = Object.fromEntries(
	Object.entries(rasterFixtureModules).map(([path, mod]) => {
		const id = rasterFixtureIdFromGlobPath(path);
		const json = typeof mod.default === "string" ? mod.default : JSON.stringify(mod.default);
		return [id, json];
	}),
);

export const RASTER_PLAY_FIXTURE_OPTIONS: ReadonlyArray<{ readonly id: string; readonly label: string }> = Object.keys(
	RASTER_PLAY_FILE_FIXTURE_JSON_BY_ID,
)
	.sort()
	.map((id) => ({ id, label: rasterFixtureLabelFromId(id) }));

export const RASTER_PLAY_EMPTY_DOCUMENT: RasterDocument = defaultRasterDocument("empty");

function rasterPlayCmd(command: string, args: Record<string, unknown> = {}): { controllerId: string; command: string; args: Record<string, unknown> } {
	return { controllerId: RASTER_PLAY_CONTROLLER_ID, command, args };
}

function rasterPlayHierarchyHoverHandlers(
	onHover: ((payload: RasterHoverPayload) => void) | undefined,
	doc: RasterDocument,
	rowId: string,
): Pick<UiTreeItemNode, "onPointerEnter" | "onPointerLeave"> {
	if (!onHover) return {};
	return {
		onPointerEnter: () => onHover(rasterPlayHoverPayloadFromTreeRowId(doc, rowId)),
		onPointerLeave: () => onHover({ id: null, kind: null }),
	};
}

export interface RasterPlayHierarchyBuildOptions {
	readonly onToggleVisible?: (layerId: string) => void;
	readonly onDeleteLayer?: (layerId: string) => void;
	readonly onDuplicateLayer?: (layerId: string) => void;
	readonly onAddMask?: (layerId: string) => void;
}

function rasterPlayLayerChrome(
	layer: RasterLayerNode,
	options?: RasterPlayHierarchyBuildOptions,
): Pick<UiTreeItemNode, "isHidden" | "actions" | "contextMenu"> {
	const contextMenu: UiTreeContextMenuItem[] = [];
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
	if (layer.kind === "pixel" && !layer.mask?.enabled && options?.onAddMask) {
		contextMenu.push({
			id: "add-mask",
			label: "Add Mask",
			icon: "square-dashed",
			onSelect: () => options.onAddMask!(layer.id),
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

function rasterPlayLayerTreeItem(
	doc: RasterDocument,
	layer: RasterLayerNode,
	options?: RasterPlayHierarchyBuildOptions,
	hoverSink?: (payload: RasterHoverPayload) => void,
): UiTreeItemNode {
	const rowId = rasterPlayLayersTreeRowId(layer);
	const nestedItems =
		layer.kind === "group"
			? layer.children.map((child) => rasterPlayLayerTreeItem(doc, child, options, hoverSink))
			: layer.mask?.enabled
				? [
						{
							id: rasterPlayMaskTreeRowId(layer.id),
							label: "Mask",
							icon: "square-dashed",
							command: rasterPlayCmd("setSelection", { ids: [layer.id], focus: "mask" }),
							...rasterPlayHierarchyHoverHandlers(hoverSink, doc, rasterPlayMaskTreeRowId(layer.id)),
						},
					]
				: undefined;
	return {
		id: rowId,
		label: layer.name,
		description: layer.kind === "adjustment" ? layer.adjustmentKind : layer.blendMode,
		icon: layer.kind === "group" ? "folder" : layer.kind === "adjustment" ? "sliders-horizontal" : "image",
		defaultOpen: layer.kind === "group",
		draggable: true,
		dragData: { "application/x-semio-raster-layer-id": layer.id },
		command: rasterPlayCmd("setSelection", { ids: [layer.id] }),
		items: nestedItems,
		...rasterPlayLayerChrome(layer, options),
		...rasterPlayHierarchyHoverHandlers(hoverSink, doc, rowId),
	};
}

function rasterPlayCatalogueLayerDragData(kind: RasterCatalogueLayerKind): Record<string, string> {
	return { [RASTER_LAYER_KIND_DRAG_MIME]: JSON.stringify({ kind }) };
}

export function buildRasterPlayLayersTree(
	doc: RasterDocument,
	selectedIds: readonly string[],
	hoveredId: string | null,
	kindHover: RasterKindHover | null,
	hoverSink?: (payload: RasterHoverPayload) => void,
	options?: RasterPlayHierarchyBuildOptions,
): UiTreeNode {
	const highlightedIds = rasterPlayLayersTreeHighlightedIds(doc, hoveredId, kindHover);
	const selectedTreeIds = selectedIds
		.map((id) => findRasterLayer(doc, id))
		.filter((layer): layer is RasterLayerNode => Boolean(layer))
		.map((layer) => rasterPlayLayersTreeRowId(layer));
	const toolbarItems: UiTreeItemNode[] = [
		{
			id: "raster-play-layers.add.pixel",
			label: "Add Pixel Layer",
			icon: "plus",
			command: rasterPlayCmd("addLayer", { kind: "pixel" }),
		},
		{
			id: "raster-play-layers.add.group",
			label: "Add Group",
			icon: "folder-plus",
			command: rasterPlayCmd("addLayer", { kind: "group" }),
		},
		{
			id: "raster-play-layers.add.adjustment",
			label: "Add Adjustment",
			icon: "sliders-horizontal",
			command: rasterPlayCmd("addLayer", { kind: "adjustment" }),
		},
	];
	const layerItems =
		doc.layers.length > 0
			? doc.layers.map((layer) => rasterPlayLayerTreeItem(doc, layer, options, hoverSink))
			: [{ id: "raster-play-layers.empty", label: "Drop layers here", icon: "image" as const }];
	return {
		type: "tree",
		sections: [
			{
				id: "raster-play-layers",
				label: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
				defaultOpen: true,
				items: [...toolbarItems, ...layerItems],
			},
		],
		selectedIds: selectedTreeIds,
		highlightedIds: [...highlightedIds],
	};
}

export function buildRasterPlayMasksTree(
	doc: RasterDocument,
	selectedIds: readonly string[],
	hoveredId: string | null,
	kindHover: RasterKindHover | null,
	hoverSink?: (payload: RasterHoverPayload) => void,
): UiTreeNode {
	const masks = flattenRasterLayers(doc.layers).filter((layer) => layer.mask?.enabled);
	const highlightedIds = rasterPlayLayersTreeHighlightedIds(doc, hoveredId, kindHover);
	return {
		type: "tree",
		sections: [
			{
				id: "raster-play-masks",
				label: "Masks",
				items:
					masks.length > 0
						? masks.map((layer) => ({
								id: rasterPlayMaskTreeRowId(layer.id),
								label: `${layer.name} Mask`,
								icon: "square-dashed" as const,
								command: rasterPlayCmd("setSelection", { ids: [layer.id], focus: "mask" }),
								...rasterPlayHierarchyHoverHandlers(hoverSink, doc, rasterPlayMaskTreeRowId(layer.id)),
							}))
						: [{ id: "raster-play-masks.empty", label: "No layer masks", icon: "square-dashed" as const }],
			},
		],
		selectedIds: selectedIds.map((id) => rasterPlayMaskTreeRowId(id)),
		highlightedIds: [...highlightedIds],
	};
}

export function buildRasterPlayInspectorTree(doc: RasterDocument, selectedIds: readonly string[]): UiNode {
	const layerId = selectedIds[0];
	const layer = layerId ? findRasterLayer(doc, layerId) : undefined;
	if (!layer) {
		return uiDeclarativeSectionsToTree([
			{
				type: "section",
				id: "raster-play-inspector.empty",
				label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
				children: [{ type: "text", value: "Select a layer in the hierarchy." }],
			},
		]);
	}
	const children: UiSectionNode["children"] = [
		{
			type: "field",
			id: "raster-play-inspector.name",
			label: "Name",
			child: {
				type: "input",
				id: "raster-play-inspector.name.input",
				inputKind: "text",
				value: layer.name,
				onChange: rasterPlayCmd("patchLayer", { layerId: layer.id, field: "name" }),
			},
		},
		{
			type: "field",
			id: "raster-play-inspector.opacity",
			label: "Opacity",
			child: {
				type: "slider",
				id: "raster-play-inspector.opacity.slider",
				min: 0,
				max: 1,
				step: 0.01,
				value: layer.opacity,
				onChange: rasterPlayCmd("patchLayer", { layerId: layer.id, field: "opacity" }),
			},
		},
		{
			type: "field",
			id: "raster-play-inspector.blend",
			label: "Blend Mode",
			child: {
				type: "select",
				id: "raster-play-inspector.blend.select",
				value: layer.blendMode,
				items: RASTER_BLEND_MODES.map((mode) => ({ id: mode, value: mode, label: mode })),
				onChange: rasterPlayCmd("patchLayer", { layerId: layer.id, field: "blendMode" }),
			},
		},
		{
			type: "field",
			id: "raster-play-inspector.visible",
			label: "Visible",
			child: {
				type: "toggle",
				id: "raster-play-inspector.visible.toggle",
				pressed: layer.visible,
				onChange: rasterPlayCmd("patchLayer", { layerId: layer.id, field: "visible" }),
			},
		},
	];
	if (layer.kind === "pixel") {
		children.push(
			{
				type: "field",
				id: "raster-play-inspector.width",
				label: "Width",
				child: {
					type: "input",
					id: "raster-play-inspector.width.input",
					inputKind: "number",
					value: String(layer.width ?? 512),
					onChange: rasterPlayCmd("patchLayer", { layerId: layer.id, field: "width" }),
				},
			},
			{
				type: "field",
				id: "raster-play-inspector.height",
				label: "Height",
				child: {
					type: "input",
					id: "raster-play-inspector.height.input",
					inputKind: "number",
					value: String(layer.height ?? 512),
					onChange: rasterPlayCmd("patchLayer", { layerId: layer.id, field: "height" }),
				},
			},
		);
	}
	if (layer.kind === "adjustment") {
		children.push({
			type: "field",
			id: "raster-play-inspector.adjustmentKind",
			label: "Adjustment",
			child: {
				type: "select",
				id: "raster-play-inspector.adjustmentKind.select",
				value: layer.adjustmentKind,
				items: RASTER_ADJUSTMENT_KINDS.map((kind) => ({ id: kind, value: kind, label: kind })),
				onChange: rasterPlayCmd("patchLayer", { layerId: layer.id, field: "adjustmentKind" }),
			},
		});
	}
	children.push(
		{ type: "button", id: "raster-play-inspector.duplicate", label: "Duplicate Layer", command: rasterPlayCmd("duplicateLayer", { layerId: layer.id }) },
		{ type: "button", id: "raster-play-inspector.delete", label: "Delete Layer", command: rasterPlayCmd("deleteLayer", { layerId: layer.id }) },
	);
	return uiDeclarativeSectionsToTree([
		{
			type: "section",
			id: "raster-play-inspector.layer",
			label: layer.name,
			children,
		},
	] as readonly UiSectionNode[]);
}

/** @emoji 🔍 Alias for the details / inspection panel tree. */
export function buildRasterPlayPropertiesTree(doc: RasterDocument, selectedIds: readonly string[]): UiTreeNode {
	const inspector = buildRasterPlayInspectorTree(doc, selectedIds);
	return inspector.type === "tree" ? inspector : { type: "tree", sections: [{ id: "raster-play-properties", label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, items: [] }] };
}

export function buildRasterPlayCatalogueTree(
	selectedIds: readonly string[],
	hoverSink?: (payload: RasterHoverPayload) => void,
): UiTreeNode {
	const selectedLayerId = selectedIds[0];
	return {
		type: "tree",
		sections: [
			{
				id: "raster-play-catalogue.layers",
				label: "New Layers",
				defaultOpen: true,
				items: [
					{
						id: "raster-play-catalogue.layer.pixel",
						label: "Pixel Layer",
						description: "paintable bitmap",
						icon: "image",
						draggable: true,
						dragData: rasterPlayCatalogueLayerDragData("pixel"),
						command: rasterPlayCmd("addLayer", { kind: "pixel" }),
					},
					{
						id: "raster-play-catalogue.layer.group",
						label: "Group",
						description: "nested stack",
						icon: "folder",
						draggable: true,
						dragData: rasterPlayCatalogueLayerDragData("group"),
						command: rasterPlayCmd("addLayer", { kind: "group" }),
					},
					{
						id: "raster-play-catalogue.layer.adjustment",
						label: "Adjustment",
						description: "non-destructive",
						icon: "sliders-horizontal",
						draggable: true,
						dragData: rasterPlayCatalogueLayerDragData("adjustment"),
						command: rasterPlayCmd("addLayer", { kind: "adjustment" }),
					},
				],
			},
			{
				id: "raster-play-catalogue.blend",
				label: "Blend Modes",
				items: RASTER_BLEND_MODES.map((mode) => ({
					id: rasterPlayBlendModeTreeRowId(mode),
					label: mode,
					icon: "blend" as const,
					command: selectedLayerId ? rasterPlayCmd("setLayerBlendMode", { layerId: selectedLayerId, blendMode: mode }) : undefined,
					onPointerEnter: hoverSink ? () => hoverSink({ id: null, kind: { domain: "blendMode", kindId: mode } }) : undefined,
					onPointerLeave: hoverSink ? () => hoverSink({ id: null, kind: null }) : undefined,
				})),
			},
			{
				id: "raster-play-catalogue.filters",
				label: "Filters",
				defaultOpen: false,
				items: RASTER_FILTER_KINDS.map((kind) => ({
					id: `raster-play-catalogue.filter.${kind}`,
					label: kind,
					icon: "sparkles" as const,
					description: selectedLayerId ? "append to selected pixel layer" : "select a pixel layer",
					command: selectedLayerId ? rasterPlayCmd("appendFilter", { layerId: selectedLayerId, filterKind: kind }) : undefined,
				})),
			},
		],
	};
}

export function buildRasterPlayBlendCatalogueTree(hoverSink?: (payload: RasterHoverPayload) => void): UiTreeNode {
	return buildRasterPlayCatalogueTree([], hoverSink);
}

function rasterPlayParseCatalogueLayerKind(data: Record<string, string>): RasterCatalogueLayerKind | null {
	const raw = data[RASTER_LAYER_KIND_DRAG_MIME];
	if (!raw) return null;
	try {
		const payload = JSON.parse(raw) as { kind?: string };
		if (payload.kind === "pixel" || payload.kind === "group" || payload.kind === "adjustment") return payload.kind;
	} catch {
		return null;
	}
	return null;
}

function rasterPlayCreateLayerByKind(kind: RasterCatalogueLayerKind): RasterLayerNode {
	if (kind === "group") return createRasterGroupLayer();
	if (kind === "adjustment") return createRasterAdjustmentLayer();
	return createRasterPixelLayer();
}

/** @emoji 🖱️ Hierarchy drag: reorder layers and accept catalogue drops. */
export function createRasterPlayHierarchyTreeDragController(getController: () => RasterPlayController | undefined): TreeDragAndDropController {
	return {
		handleDrop: ({ target, targetKind, data, sourceItems, dropPosition }) => {
			const catalogueKind = rasterPlayParseCatalogueLayerKind(data);
			if (catalogueKind) {
				const targetRowId = targetKind === "item" ? (target as TreeDataItem).id : "raster-play-layers";
				getController()?.run("dropLayerKind", {
					kind: catalogueKind,
					targetRowId,
					dropPosition: dropPosition ?? "inside",
				});
				return;
			}
			const sourceItem = sourceItems[0];
			if (!sourceItem || targetKind !== "item") return;
			const layerId = sourceItem.dragData?.["application/x-semio-raster-layer-id"] ?? rasterPlayLayerIdFromTreeRowId(sourceItem.id);
			if (!layerId) return;
			getController()?.run("moveLayer", {
				layerId,
				targetRowId: (target as TreeDataItem).id,
				dropPosition: dropPosition ?? "after",
			});
		},
	};
}

export interface RasterPlayHostBridge {
	runHostCommand(command: string, args?: unknown): void;
}

export class RasterPlayController extends Controller implements PlaygroundFixtureHost {
	readonly mainMode = new ModeRuntime("main", "Raster", undefined);
	private document: RasterDocument = RASTER_PLAY_EMPTY_DOCUMENT;
	private selectedIds: string[] = [];
	private hoveredId: string | null = null;
	private hoveredKind: RasterKindHover | null = null;
	private interactionRevision = 0;
	private listeners = new Set<() => void>();
	private hostBridge: RasterPlayHostBridge | null = null;

	constructor(bus: CommandBus, notifyPlatform: () => void) {
		super(RASTER_PLAY_CONTROLLER_ID, bus, notifyPlatform);
		this.rebuildShellMode();
	}

	private rebuildShellMode(): void {
		this.mainMode.tools = RASTER_PLAY_TOOLS;
		this.mainMode.windowKinds = [
			new WindowKindRuntime(RASTER_PLAY_WINDOW_KIND_COMPOSITE, "Composite", RASTER_PLAY_BODY_KEY_COMPOSITE),
			new WindowKindRuntime(RASTER_PLAY_WINDOW_KIND_NAVIGATOR, "Navigator", RASTER_PLAY_BODY_KEY_NAVIGATOR),
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

	getDocument(): RasterDocument {
		return this.document;
	}

	getDocumentJson(): string {
		return rasterDocumentToExportJson(this.document);
	}

	setHostBridge(bridge: RasterPlayHostBridge | null): void {
		this.hostBridge = bridge;
	}

	private applyDocument(doc: RasterDocument, resetSelection = false): void {
		this.document = doc;
		if (resetSelection) {
			this.selectedIds = doc.layers[0] ? [doc.layers[0].id] : [];
		}
		this.bump();
	}

	getSelectedIds(): readonly string[] {
		return this.selectedIds;
	}

	getHoveredId(): string | null {
		return this.hoveredId;
	}

	getHoveredKind(): RasterKindHover | null {
		return this.hoveredKind;
	}

	getFixtureCatalog(): PlaygroundFixtureCatalog | null {
		if (isPlaygroundFixtureLocked()) return null;
		return {
			activeFixtureId: playgroundResolvedFixtureId(
				this.document.id === "empty" ? PLAYGROUND_NO_FIXTURE_ID : this.document.id,
				RASTER_PLAY_FIXTURE_DEFAULT_ID,
			),
			options: RASTER_PLAY_FIXTURE_OPTIONS,
		};
	}

	private patchDocument(edit: (doc: RasterDocument) => RasterDocument, selectLayerId?: string): void {
		this.document = edit(this.document);
		if (selectLayerId) this.selectedIds = [selectLayerId];
		this.bump();
	}

	private insertLayerAt(kind: RasterCatalogueLayerKind, targetRowId: string, dropPosition: TreeDropPosition): string {
		const layer = rasterPlayCreateLayerByKind(kind);
		const target = resolveRasterPlayReorderTarget(this.document, targetRowId, dropPosition === "before" || dropPosition === "after" ? dropPosition : "inside");
		const parentId = target?.parentId;
		const index = target?.index ?? this.document.layers.length;
		const op =
			kind === "group"
				? ({ op: "addGroupLayer", parentId, index, layer } as const)
				: kind === "adjustment"
					? ({ op: "addAdjustmentLayer", parentId, index, layer } as const)
					: ({ op: "addPixelLayer", parentId, index, layer } as const);
		this.document = applyRasterEditOp(this.document, op);
		return layer.id;
	}

	run(command: string, args: Record<string, unknown> = {}): void {
		switch (command) {
			case "setActiveFixture": {
				const fixtureId = String(args.fixtureId ?? "");
				if (isPlaygroundNoFixtureId(fixtureId)) {
					this.document = RASTER_PLAY_EMPTY_DOCUMENT;
					this.selectedIds = [];
					this.bump();
					return;
				}
				const json = RASTER_PLAY_FILE_FIXTURE_JSON_BY_ID[fixtureId];
				if (json) {
					this.applyDocument(rasterDocumentFromJson(json), true);
					console.log("[DEBUG] raster fixture loaded", fixtureId);
				}
				return;
			}
			case "setFixtureJson": {
				const json = typeof args.json === "string" ? args.json : "";
				if (!json.includes("raster.document/v1")) {
					console.log("[DEBUG] raster import rejected: not a raster document");
					return;
				}
				this.applyDocument(rasterDocumentFromJson(json), args.resetInteraction !== false);
				console.log("[DEBUG] raster document imported");
				return;
			}
			case "saveDownload":
			case "loadRequest": {
				this.hostBridge?.runHostCommand(command, args);
				return;
			}
			case "setSelection": {
				const ids = Array.isArray(args.ids) ? args.ids.map(String) : [];
				this.selectedIds = ids;
				this.bump();
				return;
			}
			case "setHover": {
				this.hoveredId = typeof args.id === "string" ? args.id : null;
				this.hoveredKind = (args.kind as RasterKindHover | null) ?? null;
				this.bump();
				return;
			}
			case "setActiveTool": {
				const tool = String(args.tool ?? "") as RasterToolId;
				this.document = applyRasterEditOp(this.document, { op: "setActiveTool", tool });
				this.bump();
				return;
			}
			case "setLayerVisible": {
				this.document = applyRasterEditOp(this.document, {
					op: "setLayerVisible",
					layerId: String(args.layerId),
					visible: args.visible !== false,
				});
				this.bump();
				return;
			}
			case "addLayer": {
				const kind = String(args.kind ?? "pixel") as RasterCatalogueLayerKind;
				const parentId = typeof args.parentId === "string" ? args.parentId : undefined;
				const layer = rasterPlayCreateLayerByKind(kind);
				const op =
					kind === "group"
						? ({ op: "addGroupLayer", parentId, layer } as const)
						: kind === "adjustment"
							? ({ op: "addAdjustmentLayer", parentId, layer } as const)
							: ({ op: "addPixelLayer", parentId, layer } as const);
				this.patchDocument((doc) => applyRasterEditOp(doc, op), layer.id);
				return;
			}
			case "dropLayerKind": {
				const kind = String(args.kind ?? "") as RasterCatalogueLayerKind;
				const targetRowId = String(args.targetRowId ?? "raster-play-layers");
				const dropPosition = (args.dropPosition ?? "inside") as TreeDropPosition;
				if (kind !== "pixel" && kind !== "group" && kind !== "adjustment") return;
				const layerId = this.insertLayerAt(kind, targetRowId, dropPosition);
				this.selectedIds = [layerId];
				this.bump();
				return;
			}
			case "moveLayer": {
				const layerId = String(args.layerId ?? "");
				const targetRowId = String(args.targetRowId ?? "");
				const dropPosition = (args.dropPosition ?? "after") as TreeDropPosition;
				const target = resolveRasterPlayReorderTarget(this.document, targetRowId, dropPosition);
				if (!target || !layerId) return;
				this.patchDocument((doc) => applyRasterEditOp(doc, { op: "reorderLayer", layerId, parentId: target.parentId, index: target.index }));
				return;
			}
			case "deleteLayer": {
				const layerId = String(args.layerId ?? "");
				if (!layerId) return;
				this.document = applyRasterEditOp(this.document, { op: "removeLayer", layerId });
				this.selectedIds = this.selectedIds.filter((id) => id !== layerId);
				if (this.selectedIds.length === 0) {
					this.selectedIds = flattenRasterLayers(this.document.layers).map((layer) => layer.id).slice(0, 1);
				}
				this.bump();
				return;
			}
			case "duplicateLayer": {
				const layerId = String(args.layerId ?? "");
				const source = findRasterLayer(this.document, layerId);
				if (!source) return;
				this.document = applyRasterEditOp(this.document, { op: "duplicateLayer", layerId });
				const duplicate = flattenRasterLayers(this.document.layers).find((layer) => layer.id !== layerId && layer.name === `${source.name} copy`);
				this.selectedIds = duplicate ? [duplicate.id] : this.selectedIds;
				this.bump();
				return;
			}
			case "toggleLayerVisible": {
				const layerId = String(args.layerId ?? "");
				const layer = findRasterLayer(this.document, layerId);
				if (!layer) return;
				this.patchDocument((doc) => applyRasterEditOp(doc, { op: "setLayerVisible", layerId, visible: !layer.visible }));
				return;
			}
			case "addLayerMask": {
				const layerId = String(args.layerId ?? "");
				const layer = findRasterLayer(this.document, layerId);
				if (!layer || layer.kind !== "pixel") return;
				const width = layer.width ?? 512;
				const height = layer.height ?? 512;
				this.patchDocument((doc) =>
					applyRasterEditOp(doc, {
						op: "setLayerMask",
						layerId,
						mask: { enabled: true, linked: true, invert: false, width, height },
					}),
				);
				return;
			}
			case "appendFilter": {
				const layerId = String(args.layerId ?? "");
				const filterKind = String(args.filterKind ?? "");
				if (!(RASTER_FILTER_KINDS as readonly string[]).includes(filterKind)) return;
				this.patchDocument((doc) =>
					applyRasterEditOp(doc, {
						op: "appendLayerFilter",
						layerId,
						filter: { kind: filterKind as (typeof RASTER_FILTER_KINDS)[number], radius: 8 },
					}),
				);
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
							return applyRasterEditOp(doc, { op: "setLayerName", layerId, name: String(value ?? "") });
						case "opacity":
							return applyRasterEditOp(doc, { op: "setLayerOpacity", layerId, opacity: Number(value) });
						case "blendMode":
							return applyRasterEditOp(doc, { op: "setLayerBlendMode", layerId, blendMode: String(value) as RasterBlendMode });
						case "visible":
							return applyRasterEditOp(doc, { op: "setLayerVisible", layerId, visible: Boolean(value) });
						case "width":
							return applyRasterEditOp(doc, { op: "setLayerSize", layerId, width: Number(value) });
						case "height":
							return applyRasterEditOp(doc, { op: "setLayerSize", layerId, height: Number(value) });
						case "adjustmentKind":
							return applyRasterEditOp(doc, {
								op: "setAdjustmentKind",
								layerId,
								adjustmentKind: String(value) as (typeof RASTER_ADJUSTMENT_KINDS)[number],
							});
						default:
							return doc;
					}
				});
				return;
			}
			case "setLayerBlendMode": {
				this.document = applyRasterEditOp(this.document, {
					op: "setLayerBlendMode",
					layerId: String(args.layerId),
					blendMode: String(args.blendMode) as RasterBlendMode,
				});
				this.bump();
				return;
			}
			case "setCamera": {
				const camera = args.camera as RasterDocument["camera"];
				if (camera) {
					this.document = applyRasterEditOp(this.document, { op: "setCamera", camera });
					this.bump();
				}
				return;
			}
			case "selectAll": {
				this.selectedIds = flattenRasterLayers(this.document.layers).map((layer) => layer.id);
				this.bump();
				return;
			}
			default:
				return;
		}
	}
}

function rasterPlayTool(id: string, label: string, iconId: string, command: string, args?: Record<string, unknown>): ToolLeaf {
	return { id, kind: "button", label, iconId, controllerId: RASTER_PLAY_CONTROLLER_ID, command, args };
}

function selectionTools(): readonly ToolLeaf[] {
	return [
		rasterPlayTool("selectMarquee", "Marquee", "square-dashed", "setActiveTool", { tool: "selectMarquee" }),
		rasterPlayTool("selectLasso", "Lasso", "lasso", "setActiveTool", { tool: "selectLasso" }),
		rasterPlayTool("selectWand", "Wand", "wand-2", "setActiveTool", { tool: "selectWand" }),
	];
}

function paintTools(): readonly ToolLeaf[] {
	return [
		rasterPlayTool("paintBrush", "Brush", "paintbrush", "setActiveTool", { tool: "paintBrush" }),
		rasterPlayTool("paintEraser", "Eraser", "eraser", "setActiveTool", { tool: "paintEraser" }),
		rasterPlayTool("paintClone", "Clone", "copy", "setActiveTool", { tool: "paintClone" }),
	];
}

function transformTools(): readonly ToolLeaf[] {
	return [
		rasterPlayTool("transformMove", "Move", "move", "setActiveTool", { tool: "transformMove" }),
		rasterPlayTool("transformScale", "Scale", "scaling", "setActiveTool", { tool: "transformScale" }),
		rasterPlayTool("transformRotate", "Rotate", "rotate-cw", "setActiveTool", { tool: "transformRotate" }),
	];
}

export const RASTER_PLAY_TOOLS: AppTools = [
	toolCollection("open", "folder-open", [
		rasterPlayTool("raster-import", "Import Raster", "folder-open", "loadRequest"),
	]),
	toolCollection("save", "save", [
		rasterPlayTool("raster-export", "Export Raster", "save", "saveDownload"),
	]),
	toolCollection("selection", "mouse-pointer-2", selectionTools()),
	toolCollection("paint", "paintbrush", paintTools()),
	toolCollection("transform", "move", transformTools()),
	toolCollection("adjust", "sliders-horizontal", [
		rasterPlayTool("adjust-brightness", "Brightness", "sun", "setActiveTool", { tool: "selectMarquee" }),
	]),
	toolCollection("filter", "sparkles", [
		rasterPlayTool("filter-blur", "Blur", "blur", "setActiveTool", { tool: "selectMarquee" }),
	]),
];

export function buildRasterPlayAppRuntime(ctrl: RasterPlayController): AppRuntime {
	return createPlayAppRuntime(RASTER_PLAY_APP_ID, "Raster", ctrl, RASTER_PLAY_LAYOUT, ctrl.mainMode);
}

export function registerRasterPlayDeclarativeBodies(): void {
	registerWindowBody(RASTER_PLAY_BODY_KEY_COMPOSITE, () =>
		buildRasterWindowBody(RASTER_PLAY_SURFACE_ID_COMPOSITE, RASTER_PLAY_CONTROLLER_ID, "composite", "composite"));
	registerWindowBody(RASTER_PLAY_BODY_KEY_NAVIGATOR, () =>
		buildRasterWindowBody(RASTER_PLAY_SURFACE_ID_NAVIGATOR, RASTER_PLAY_CONTROLLER_ID, "navigator", "navigator"));
}

export class PlaygroundRaster extends Playground {
	readonly id = RASTER_PLAY_APP_ID;
	readonly keybindings = [{ key: "ctrl+a,meta+a", controllerId: RASTER_PLAY_CONTROLLER_ID, command: "selectAll" }];

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id);
		const ctrl = new RasterPlayController(runtime.commandBus, () => runtime.notify());
		const resolved = playgroundResolvedFixtureId(RASTER_PLAY_FIXTURE_DEFAULT_ID);
		const fixtureJson = RASTER_PLAY_FILE_FIXTURE_JSON_BY_ID[resolved];
		if (fixtureJson) {
			ctrl.run("setActiveFixture", { fixtureId: resolved });
		}
		runtime.addApp(buildRasterPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerRasterPlayDeclarativeBodies();
	}
}

if (typeof document !== "undefined" && document.getElementById("root") != null && !import.meta.vitest && import.meta.env.PUZZLE_PLAY_ENTRY === "raster") {
	bootstrapElementsSurfaceChromeDocument("system");
	void (async () => {
		await import("./globals.css");
		const { bootRasterPlay } = await import("@semio-tech/framework-playground-renderer-react/raster");
		bootRasterPlay(new PlaygroundRaster());
	})();
}

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("RASTER_PLAY_FIXTURE_OPTIONS", () => {
		it("includes semio fixture", () => {
			expect(RASTER_PLAY_FIXTURE_OPTIONS.some((row) => row.id === "semio")).toBe(true);
		});
	});

	describe("buildRasterPlayAppRuntime", () => {
		it("wires composite and navigator window kinds", () => {
			const bus = new CommandBus();
			const ctrl = new RasterPlayController(bus, () => {});
			const app = buildRasterPlayAppRuntime(ctrl);
			expect(app.windowKinds.map((kind) => kind.id)).toEqual([
				RASTER_PLAY_WINDOW_KIND_COMPOSITE,
				RASTER_PLAY_WINDOW_KIND_NAVIGATOR,
			]);
		});
	});
	describe("buildRasterPlayMasksTree", () => {
		it("lists mask rows from the semio fixture", () => {
			const doc = rasterDocumentFromJson(RASTER_PLAY_FILE_FIXTURE_JSON_BY_ID.semio!);
			const tree = buildRasterPlayMasksTree(doc, [], null, null);
			expect(tree.sections[0]?.items.some((item) => item.id.includes(".mask."))).toBe(true);
		});
	});

	describe("buildRasterPlayLayersTree", () => {
		it("builds nested layer rows", () => {
			const doc = rasterDocumentFromJson(RASTER_PLAY_FILE_FIXTURE_JSON_BY_ID.semio!);
			const tree = buildRasterPlayLayersTree(doc, [], null, null);
			const emblem = tree.sections[0]?.items.find((item) => item.id.includes("logo-group"));
			expect(emblem?.items?.length).toBeGreaterThan(0);
		});
	});

	describe("RasterPlayController layer edits", () => {
		it("adds and deletes layers", () => {
			const bus = new CommandBus();
			const ctrl = new RasterPlayController(bus, () => {});
			ctrl.run("setActiveFixture", { fixtureId: "semio" });
			const before = flattenRasterLayers(ctrl.getDocument().layers).length;
			ctrl.run("addLayer", { kind: "pixel" });
			expect(flattenRasterLayers(ctrl.getDocument().layers).length).toBe(before + 1);
			const addedId = ctrl.getSelectedIds()[0]!;
			ctrl.run("deleteLayer", { layerId: addedId });
			expect(flattenRasterLayers(ctrl.getDocument().layers).length).toBe(before);
		});
	});

	describe("RasterPlayController import export", () => {
		it("round-trips fixture json", () => {
			const bus = new CommandBus();
			const ctrl = new RasterPlayController(bus, () => {});
			ctrl.run("setActiveFixture", { fixtureId: "semio" });
			const exported = ctrl.getDocumentJson();
			ctrl.run("setFixtureJson", { json: exported, resetInteraction: true });
			expect(ctrl.getDocument().id).toBe("semio");
			expect(ctrl.getDocument().assets?.["semio-emblem"]?.mime).toBe("image/png");
		});
	});

	describe("RASTER_PLAY_FILE_FIXTURE_JSON_BY_ID", () => {
		for (const fixtureId of Object.keys(RASTER_PLAY_FILE_FIXTURE_JSON_BY_ID)) {
			it(`loads ${fixtureId} fixture into trees without empty sections`, () => {
				const doc = rasterDocumentFromJson(RASTER_PLAY_FILE_FIXTURE_JSON_BY_ID[fixtureId]!);
				const layers = buildRasterPlayLayersTree(doc, [], null, null);
				const masks = buildRasterPlayMasksTree(doc, [], null, null);
				const properties = buildRasterPlayInspectorTree(doc, doc.layers[0] ? [doc.layers[0].id] : []);
				expect(layers.sections[0]?.items.length).toBeGreaterThan(0);
				expect(masks.sections[0]?.items.length).toBeGreaterThan(0);
				expect(properties.type).toBe("tree");
			});
		}
	});
}
// #endregion 🧪Tests
