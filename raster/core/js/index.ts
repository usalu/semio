// #region 🧲Header
/** @emoji 🖼️ Raster play app — composite and navigator editor. */
// #endregion 🧲Header

import {
	createPlaygroundApp,
	createProductPlaygroundPlatform,
	AppRuntime,
	CommandBus,
	Controller,
	ModeRuntime,
	WindowKindRuntime,
	buildRasterWindowBody,
	createDefaultLayout,
	createPlayAppRuntime,
	eagerPlayExampleGlob,
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
	uiInspectorMixedSlider,
	uiInspectorMixedText,
	uiInspectorMixedToggle,
	uiInspectorReadonlyField,
	type UiInspectorFieldGroup,
	type UiNode,
	type UiTreeContextMenuItem,
	type UiTreeItemNode,
	type UiTreeNode,
	type WindowMeasure,
	type WindowEngagement,
	enforcePlaygroundWindowEngagementInput,
  createPlaygroundApp,
  createProductPlaygroundPlatform,
} from "@semio-tech/framework-playground-core";
import { registerOsMediaExportHandler } from "@semio-tech/framework-os-core";
import { rasterizeSvgMarkupToPngDataUrl } from "@semio-tech/kernel-2d-js";
import {
	CANVAS_HOVER_SOURCE_CANVAS,
	CANVAS_HOVER_SOURCE_CATALOG,
	CANVAS_HOVER_SOURCE_HIERARCHY,
	CANVAS_HOVER_SOURCE_PICK_MENU,
} from "@semio-tech/framework-core";
import { DocumentVcsStore, createDocumentVcsEnvelope, recordProjectionChange } from "@semio-tech/vcs-core/internal";
import { bootstrapElementsSurfaceChromeDocument, type TreeDataItem, type TreeDragAndDropController, type TreeDropPosition } from "@semio-tech/ui-react";
import {
	applyRasterEditOp,
	backwardsRasterEditOp,
	createRasterAdjustmentLayer,
	createRasterGroupLayer,
	createRasterPixelLayer,
	defaultRasterDocument,
	diffRasterEditOp,
	findRasterLayer,
	flattenRasterLayers,
	parseRasterDocument,
	rasterDocumentFromJson,
	rasterDocumentToExportJson,
	rasterPlayBlendModeTreeRowId,
	rasterPlayHoverPayloadFromTreeRowId,
	encodeRasterPointerFocusKey,
	rasterHoverPayloadFromPointerFocusKey,
	rasterPlayLayerIdFromTreeRowId,
	rasterPlayLayersTreeHighlightedIds,
	rasterPlayLayersTreeRowId,
	rasterPlayMaskTreeRowId,
	rasterPlayMaskTreeRowIdsForSelectionIds,
	rasterPlaySelectionIdsFromTreeRowIds,
	rasterPlayTreeRowIdsForSelectionIds,
	resolveRasterPlayReorderTarget,
	type RasterBlendMode,
	type RasterDocument,
	type RasterEditOp,
	type RasterHoverPayload,
	type RasterKindHover,
	type RasterLayerNode,
	type RasterToolId,
	type RasterViewport,
	RASTER_ADJUSTMENT_KINDS,
	RASTER_BLEND_MODES,
	RASTER_FILTER_KINDS,
} from "./internal.ts";
import { RASTER_PLAY_EXAMPLE_DEFAULT_ID, resolveRasterPlayExampleSlug } from "./example-slugs.ts";

export const RASTER_PLAY_APP_ID = "raster-play";
export const RASTER_PLAY_CONTROLLER_ID = "raster-play";
export const RASTER_PLAY_SURFACE_ID_COMPOSITE = "raster.play.composite";
export const RASTER_PLAY_SURFACE_ID_NAVIGATOR = "raster.play.navigator";
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

export { RASTER_PLAY_EXAMPLE_DEFAULT_ID, resolveRasterPlayExampleSlug };

const rasterExampleModules = eagerPlayExampleGlob("../../example/*.raster.json");

function rasterExampleIdFromGlobPath(globPath: string): string {
	const base = globPath.split("/").pop() ?? globPath;
	return base.replace(/\.raster\.json$/, "");
}

function rasterExampleLabelFromId(id: string): string {
	return id
		.split("-")
		.filter(Boolean)
		.map((word) => word.charAt(0).toUpperCase() + word.slice(1))
		.join(" ");
}

export const RASTER_PLAY_FILE_EXAMPLE_JSON_BY_ID: Record<string, string> = Object.fromEntries(
	Object.entries(rasterExampleModules).map(([path, mod]) => {
		const id = rasterExampleIdFromGlobPath(path);
		const json = typeof mod.default === "string" ? mod.default : JSON.stringify(mod.default);
		return [id, json];
	}),
);

export const RASTER_PLAY_EXAMPLE_OPTIONS: ReadonlyArray<{ readonly id: string; readonly label: string }> = Object.keys(
	RASTER_PLAY_FILE_EXAMPLE_JSON_BY_ID,
)
	.sort()
		.map((id) => ({ id, label: rasterExampleLabelFromId(id) }));

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
	const selectedTreeIds = rasterPlayTreeRowIdsForSelectionIds(doc, selectedIds);
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
		selectionChange: rasterPlayCmd("setSelection"),
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
		selectedIds: rasterPlayMaskTreeRowIdsForSelectionIds(doc, selectedIds),
		highlightedIds: [...highlightedIds],
		selectionChange: rasterPlayCmd("setSelection"),
	};
}

function rasterPlayInspectorPatch(layerIds: readonly string[], field: string) {
	return rasterPlayCmd("patchLayers", { layerIds, field });
}

function rasterPlayInspectorNumberField(
	layerIds: readonly string[],
	fieldId: string,
	label: string,
	values: readonly number[],
	field: string,
): UiNode {
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
			onChange: rasterPlayInspectorPatch(layerIds, field),
		},
	};
}

function rasterPlayInspectorTextField(
	layerIds: readonly string[],
	fieldId: string,
	label: string,
	values: readonly string[],
	field: string,
): UiNode {
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
			onChange: rasterPlayInspectorPatch(layerIds, field),
		},
	};
}

function rasterPlayLayersUniformKind(layers: readonly RasterLayerNode[]): RasterLayerNode[] | null {
	if (!layers.length) return null;
	const kindKey = rasterPlayLayerKindLabel(layers[0]!);
	return layers.every((layer) => rasterPlayLayerKindLabel(layer) === kindKey) ? [...layers] : null;
}

function rasterPlayLayerKindLabel(layer: RasterLayerNode): string {
	return layer.kind === "adjustment" ? `adjustment:${layer.adjustmentKind}` : layer.kind;
}

function rasterPlayInspectorPixelGroup(layers: readonly RasterLayerNode[]): UiInspectorFieldGroup | null {
	const uniformLayers = rasterPlayLayersUniformKind(layers);
	if (!uniformLayers || uniformLayers[0]!.kind !== "pixel") return null;
	const layerIds = uniformLayers.map((entry) => entry.id);
	return {
		id: "raster-play-inspector.pixel",
		label: "Pixel",
		fields: [
			rasterPlayInspectorNumberField(layerIds, "raster-play-inspector.width", "Width", uniformLayers.map((entry) => entry.width ?? 512), "width"),
			rasterPlayInspectorNumberField(layerIds, "raster-play-inspector.height", "Height", uniformLayers.map((entry) => entry.height ?? 512), "height"),
		],
	};
}

function rasterPlayInspectorAdjustmentGroup(layers: readonly RasterLayerNode[]): UiInspectorFieldGroup | null {
	const uniformLayers = rasterPlayLayersUniformKind(layers);
	if (!uniformLayers || uniformLayers[0]!.kind !== "adjustment") return null;
	const layerIds = uniformLayers.map((entry) => entry.id);
	const kinds = uniformLayers.map((entry) => (entry.kind === "adjustment" ? entry.adjustmentKind : ""));
	const kindMixed = uiInspectorMixedSelect(kinds);
	return {
		id: "raster-play-inspector.adjustment",
		label: "Adjustment",
		fields: [
			{
				type: "field",
				id: "raster-play-inspector.adjustmentKind",
				label: "Kind",
				child: {
					type: "select",
					id: "raster-play-inspector.adjustmentKind.select",
					value: kindMixed.value,
					placeholder: kindMixed.placeholder,
					items: RASTER_ADJUSTMENT_KINDS.map((kind) => ({ id: kind, value: kind, label: kind })),
					onChange: rasterPlayInspectorPatch(layerIds, "adjustmentKind"),
				},
			},
		],
	};
}

function rasterPlayInspectorMaskGroup(layers: readonly RasterLayerNode[]): UiInspectorFieldGroup | null {
	if (!layers.length || !layers.every((layer) => layer.mask?.enabled)) return null;
	const layerIds = layers.map((entry) => entry.id);
	return {
		id: "raster-play-inspector.mask",
		label: "Mask",
		fields: [
			uiInspectorReadonlyField(
				"raster-play-inspector.mask-linked",
				"Linked Layer",
				layerIds.length === 1 ? (layerIds[0] ?? "") : `${layerIds.length} selected`,
			),
			...(layerIds.length === 1
				? [{ type: "button" as const, id: "raster-play-inspector.mask-select", label: "Focus Mask", command: rasterPlayCmd("setSelection", { ids: [layerIds[0]!] }) }]
				: []),
		],
	};
}

function rasterPlayInspectorActionsGroup(layers: readonly RasterLayerNode[]): UiInspectorFieldGroup | null {
	if (layers.length !== 1) return null;
	const layerId = layers[0]!.id;
	return {
		id: "raster-play-inspector.actions",
		label: "Actions",
		fields: [
			{ type: "button", id: "raster-play-inspector.duplicate", label: "Duplicate Layer", command: rasterPlayCmd("duplicateLayer", { layerId }) },
			{ type: "button", id: "raster-play-inspector.delete", label: "Delete Layer", command: rasterPlayCmd("deleteLayer", { layerId }) },
		],
	};
}

function rasterPlayInspectorLayerGroup(layers: readonly RasterLayerNode[]): UiInspectorFieldGroup {
	const layerIds = layers.map((entry) => entry.id);
	const names = layers.map((entry) => entry.name);
	const kinds = layers.map((entry) => rasterPlayLayerKindLabel(entry));
	const visibles = layers.map((entry) => entry.visible);
	const opacities = layers.map((entry) => entry.opacity);
	const blends = layers.map((entry) => entry.blendMode);
	const visibleMixed = uiInspectorMixedToggle(visibles);
	const opacityMixed = uiInspectorMixedSlider(opacities);
	const blendMixed = uiInspectorMixedSelect(blends);
	const kindMixed = uiInspectorMixedText(kinds);
	return {
		id: "raster-play-inspector.layer",
		label: "Layer",
		fields: [
			rasterPlayInspectorTextField(layerIds, "raster-play-inspector.name", "Name", names, "name"),
			uiInspectorReadonlyField("raster-play-inspector.id", "Id", layerIds.length === 1 ? (layerIds[0] ?? "") : `${layerIds.length} selected`),
			uiInspectorReadonlyField("raster-play-inspector.kind", "Kind", kindMixed.uniform ? (kinds[0] ?? "") : (kindMixed.placeholder ?? UI_INSPECTOR_MIXED_PLACEHOLDER)),
			{
				type: "field",
				id: "raster-play-inspector.visible",
				label: "Visible",
				child: {
					type: "toggle",
					id: "raster-play-inspector.visible.toggle",
					iconId: "check",
					pressed: visibleMixed.pressed,
					text: visibleMixed.uniform ? undefined : UI_INSPECTOR_MIXED_PLACEHOLDER,
					onChange: rasterPlayInspectorPatch(layerIds, "visible"),
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
					value: opacityMixed.uniform ? opacityMixed.value : 0,
					onChange: rasterPlayInspectorPatch(layerIds, "opacity"),
				},
			},
			{
				type: "field",
				id: "raster-play-inspector.blend",
				label: "Blend Mode",
				child: {
					type: "select",
					id: "raster-play-inspector.blend.select",
					value: blendMixed.value,
					placeholder: blendMixed.placeholder,
					items: RASTER_BLEND_MODES.map((mode) => ({ id: mode, value: mode, label: mode })),
					onChange: rasterPlayInspectorPatch(layerIds, "blendMode"),
				},
			},
		],
	};
}

export function buildRasterPlayInspectorTree(doc: RasterDocument, selectedIds: readonly string[]): UiNode {
	const layers = selectedIds.map((layerId) => findRasterLayer(doc, layerId)).filter((layer): layer is RasterLayerNode => Boolean(layer));
	if (!layers.length) {
		return uiDeclarativeSectionsToTree([
			{
				type: "section",
				id: "raster-play-inspector.empty",
				label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
				children: [{ type: "text", value: "Select a layer in the hierarchy." }],
			},
		]);
	}
	const groups: UiInspectorFieldGroup[] = [];
	const pixel = rasterPlayInspectorPixelGroup(layers);
	if (pixel) groups.push(pixel);
	const adjustment = rasterPlayInspectorAdjustmentGroup(layers);
	if (adjustment) groups.push(adjustment);
	const mask = rasterPlayInspectorMaskGroup(layers);
	if (mask) groups.push(mask);
	const actions = rasterPlayInspectorActionsGroup(layers);
	if (actions) groups.push(actions);
	groups.push(rasterPlayInspectorLayerGroup(layers));
	return uiInspectorGroupsToTree(groups);
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

/** @emoji 🔧 Applies one inspector field patch to a single raster layer. */
function rasterPlayPatchLayerField(doc: RasterDocument, layerId: string, field: string, value: unknown): RasterDocument {
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
}

export class RasterPlayController extends Controller implements PlaygroundExampleHost {
	readonly mainMode = new ModeRuntime("main", "Raster", undefined);
	private readonly docStore = new DocumentVcsStore<RasterDocument, RasterEditOp>({
		envelope: createDocumentVcsEnvelope("raster.document", "raster-play", RASTER_PLAY_EMPTY_DOCUMENT),
		applyOp: applyRasterEditOp,
		backwardsOp: backwardsRasterEditOp,
		diffOp: diffRasterEditOp,
	});
	private interactionRevision = 0;
	private listeners = new Set<() => void>();
	private hostBridge: RasterPlayHostBridge | null = null;
	private compositeViewport: RasterViewport = { width: 1, height: 1 };

	constructor(bus: CommandBus, notifyPlatform: () => void) {
		super(RASTER_PLAY_CONTROLLER_ID, bus, notifyPlatform);
		this.rebuildShellMode();
	}

	private compositeMeasures(): readonly WindowMeasure[] {
		const doc = this.projection();
		return [
			{
				kind: "slider",
				id: "raster-composite-zoom",
				label: "Zoom",
				value: doc.camera.zoom,
				min: 0.1,
				max: 8,
				step: 0.05,
				onChange: rasterPlayCmd("setCameraZoom"),
			},
			{
				kind: "slider",
				id: "raster-composite-brush",
				label: "Brush",
				value: doc.brushSize ?? 24,
				min: 1,
				max: 128,
				step: 1,
				onChange: rasterPlayCmd("setBrushSize"),
			},
		];
	}

	private navigatorMeasures(): readonly WindowMeasure[] {
		const doc = this.projection();
		return [
			{
				kind: "slider",
				id: "raster-navigator-zoom",
				label: "Navigator zoom",
				value: doc.camera.zoom,
				min: 0.05,
				max: 2,
				step: 0.05,
				onChange: rasterPlayCmd("setCameraZoom"),
			},
			{
				kind: "slider",
				id: "raster-navigator-brush-opacity",
				label: "Brush opacity",
				value: doc.brushOpacity ?? 1,
				min: 0,
				max: 1,
				step: 0.01,
				onChange: rasterPlayCmd("setBrushOpacity"),
			},
		];
	}

	private compositeEngagement(): WindowEngagement {
		const doc = this.projection();
		return {
			sessionActive: false,
			input: {
				id: "raster-composite-engagement",
				value: "",
				placeholder: "Select all",
				onChange: rasterPlayCmd("compositeEngagementInput"),
				onSubmit: rasterPlayCmd("selectAll"),
			},
			status: [{ id: "raster-layer-count", text: `${flattenRasterLayers(doc.layers).length} layers · tool ${doc.activeTool ?? "selectMarquee"}` }],
		};
	}

	private navigatorEngagement(): WindowEngagement {
		return {
			sessionActive: false,
			input: {
				id: "raster-navigator-engagement",
				value: "",
				placeholder: "Import raster",
				onChange: rasterPlayCmd("navigatorEngagementInput"),
				onSubmit: rasterPlayCmd("loadRequest"),
			},
			status: [{ id: "raster-selection-count", text: `${this.getSelectedIds().length} selected` }],
		};
	}

	private rebuildShellMode(): void {
		this.mainMode.tools = RASTER_PLAY_TOOLS;
		this.mainMode.windowKinds = [
			new WindowKindRuntime(RASTER_PLAY_WINDOW_KIND_COMPOSITE, "Composite", RASTER_PLAY_BODY_KEY_COMPOSITE, undefined, this.compositeMeasures(), this.compositeEngagement()),
			new WindowKindRuntime(RASTER_PLAY_WINDOW_KIND_NAVIGATOR, "Navigator", RASTER_PLAY_BODY_KEY_NAVIGATOR, undefined, this.navigatorMeasures(), this.navigatorEngagement()),
		];
		for (const windowKind of this.mainMode.windowKinds) {
			enforcePlaygroundWindowEngagementInput(windowKind.engagement, `Raster play window "${windowKind.id}"`);
		}
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

	private projection(): RasterDocument {
		return this.docStore.projection();
	}

	private dispatchEditOp(op: RasterEditOp, selectLayerId?: string, resetSelection = false): void {
		recordProjectionChange(this.docStore, [op]);
		const doc = this.projection();
		if (resetSelection) this.pointerFocus.setSelection(doc.layers[0] ? [doc.layers[0].id] : []);
		else if (selectLayerId) this.pointerFocus.setSelection([selectLayerId]);
		this.bump();
	}

	private dispatchProjectionEdit(edit: (doc: RasterDocument) => RasterDocument, selectLayerId?: string, resetSelection = false): void {
		const previous = this.projection();
		const next = edit(previous);
		if (next === previous) return;
		this.dispatchEditOp({ op: "setDocument", document: next }, selectLayerId, resetSelection);
	}

	replaceDocument(doc: RasterDocument, resetSelection = false): void {
		this.dispatchEditOp({ op: "setDocument", document: doc }, undefined, resetSelection);
	}

	getDocument(): RasterDocument {
		return this.projection();
	}

	getDocumentJson(): string {
		return rasterDocumentToExportJson(this.projection());
	}

	getDocumentVcsStore(): DocumentVcsStore<RasterDocument, RasterEditOp> {
		return this.docStore;
	}

	setHostBridge(bridge: RasterPlayHostBridge | null): void {
		this.hostBridge = bridge;
	}

	private applyDocument(doc: RasterDocument, resetSelection = false): void {
		this.replaceDocument(doc, resetSelection);
	}

	getSelectedIds(): readonly string[] {
		return this.pointerFocus.getSnapshot().selection;
	}

	getHoveredId(): string | null {
		return rasterHoverPayloadFromPointerFocusKey(this.pointerFocus.getSnapshot().hover).id;
	}

	getHoveredKind(): RasterKindHover | null {
		return rasterHoverPayloadFromPointerFocusKey(this.pointerFocus.getSnapshot().hover).kind;
	}

	getCompositeViewport(): RasterViewport {
		return this.compositeViewport;
	}

	getExampleCatalog(): PlaygroundExampleCatalog | null {
		if (isPlaygroundExampleLocked()) return null;
		return {
			activeExampleId: playgroundResolvedExampleId(
				this.projection().id === "empty" ? PLAYGROUND_NO_EXAMPLE_ID : this.projection().id,
				RASTER_PLAY_EXAMPLE_DEFAULT_ID,
			),
			options: RASTER_PLAY_EXAMPLE_OPTIONS,
		};
	}

	private insertLayerAt(kind: RasterCatalogueLayerKind, targetRowId: string, dropPosition: TreeDropPosition): string {
		const layer = rasterPlayCreateLayerByKind(kind);
		const target = resolveRasterPlayReorderTarget(this.projection(), targetRowId, dropPosition === "before" || dropPosition === "after" ? dropPosition : "inside");
		const parentId = target?.parentId;
		const index = target?.index ?? this.projection().layers.length;
		const op =
			kind === "group"
				? ({ op: "addGroupLayer", parentId, index, layer } as const)
				: kind === "adjustment"
					? ({ op: "addAdjustmentLayer", parentId, index, layer } as const)
					: ({ op: "addPixelLayer", parentId, index, layer } as const);
		this.dispatchEditOp(op, layer.id);
		return layer.id;
	}

	run(command: string, args: Record<string, unknown> = {}): void {
		switch (command) {
			case "compositeEngagementInput":
			case "navigatorEngagementInput":
				return;
			case "setCameraZoom": {
				const zoom = typeof args.value === "number" ? args.value : Number(args.value);
				if (!Number.isFinite(zoom)) return;
				const camera = { ...this.projection().camera, zoom };
				this.dispatchEditOp({ op: "setCamera", camera });
				return;
			}
			case "setBrushSize": {
				const size = typeof args.value === "number" ? args.value : Number(args.value);
				if (!Number.isFinite(size)) return;
				this.dispatchEditOp({ op: "setBrushSize", size });
				return;
			}
			case "setBrushOpacity": {
				const opacity = typeof args.value === "number" ? args.value : Number(args.value);
				if (!Number.isFinite(opacity)) return;
				this.dispatchEditOp({ op: "setBrushOpacity", opacity });
				return;
			}
			case "setActiveExample": {
				const exampleId = String(args.exampleId ?? "");
				if (isPlaygroundNoExampleId(exampleId)) {
					this.replaceDocument(RASTER_PLAY_EMPTY_DOCUMENT, true);
					this.pointerFocus.setSelection([]);
					return;
				}
				const json = RASTER_PLAY_FILE_EXAMPLE_JSON_BY_ID[exampleId];
				if (json) {
					this.applyDocument(rasterDocumentFromJson(json), true);
					console.log("[DEBUG] raster example loaded", exampleId);
				}
				return;
			}
			case "setFixtureJson": {
				const json = typeof args.json === "string" ? args.json : "";
				if (!json.includes("raster.document")) {
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
				const rawIds = Array.isArray(args.ids) ? args.ids.map(String) : [];
				const resolved = rasterPlaySelectionIdsFromTreeRowIds(this.projection(), rawIds);
				this.pointerFocus.setSelection(resolved.length > 0 ? resolved : rawIds.filter((id) => findRasterLayer(this.projection(), id)));
				this.bump();
				return;
			}
			case "setHover": {
				const sourceId =
					typeof args.sourceId === "string"
						? args.sourceId
						: args.fromPickMenu === true
							? CANVAS_HOVER_SOURCE_PICK_MENU
							: args.fromHierarchy === true
								? CANVAS_HOVER_SOURCE_HIERARCHY
								: CANVAS_HOVER_SOURCE_CANVAS;
				const id = typeof args.id === "string" ? args.id : null;
				const kind = (args.kind as RasterKindHover | null) ?? null;
				const hoverKey = id ? encodeRasterPointerFocusKey(kind?.domain ?? "layer", id) : null;
				if (hoverKey) {
					this.pointerFocus.setHoverFromSource(sourceId, hoverKey);
				} else {
					this.pointerFocus.clearHoverFromSource(sourceId);
				}
				this.bump();
				return;
			}
			case "setActiveTool": {
				const tool = String(args.tool ?? "") as RasterToolId;
				this.dispatchEditOp({ op: "setActiveTool", tool });
				return;
			}
			case "setLayerVisible": {
				this.dispatchEditOp({
					op: "setLayerVisible",
					layerId: String(args.layerId),
					visible: args.visible !== false,
				});
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
				this.dispatchEditOp(op, layer.id);
				return;
			}
			case "dropLayerKind": {
				const kind = String(args.kind ?? "") as RasterCatalogueLayerKind;
				const targetRowId = String(args.targetRowId ?? "raster-play-layers");
				const dropPosition = (args.dropPosition ?? "inside") as TreeDropPosition;
				if (kind !== "pixel" && kind !== "group" && kind !== "adjustment") return;
				const layerId = this.insertLayerAt(kind, targetRowId, dropPosition);
				this.pointerFocus.setSelection([layerId]);
				this.bump();
				return;
			}
			case "moveLayer": {
				const layerId = String(args.layerId ?? "");
				const targetRowId = String(args.targetRowId ?? "");
				const dropPosition = (args.dropPosition ?? "after") as TreeDropPosition;
				const target = resolveRasterPlayReorderTarget(this.projection(), targetRowId, dropPosition);
				if (!target || !layerId) return;
				this.dispatchEditOp({ op: "reorderLayer", layerId, parentId: target.parentId, index: target.index });
				return;
			}
			case "deleteLayer": {
				const layerId = String(args.layerId ?? "");
				if (!layerId) return;
				this.dispatchEditOp({ op: "removeLayer", layerId });
				let nextSelection = this.getSelectedIds().filter((id) => id !== layerId);
				if (nextSelection.length === 0) {
					nextSelection = flattenRasterLayers(this.projection().layers).map((layer) => layer.id).slice(0, 1);
				}
				this.pointerFocus.setSelection(nextSelection);
				return;
			}
			case "duplicateLayer": {
				const layerId = String(args.layerId ?? "");
				const source = findRasterLayer(this.projection(), layerId);
				if (!source) return;
				this.dispatchEditOp({ op: "duplicateLayer", layerId });
				const duplicate = flattenRasterLayers(this.projection().layers).find((layer) => layer.id !== layerId && layer.name === `${source.name} copy`);
				if (duplicate) this.pointerFocus.setSelection([duplicate.id]);
				return;
			}
			case "toggleLayerVisible": {
				const layerId = String(args.layerId ?? "");
				const layer = findRasterLayer(this.projection(), layerId);
				if (!layer) return;
				this.dispatchEditOp({ op: "setLayerVisible", layerId, visible: !layer.visible });
				return;
			}
			case "addLayerMask": {
				const layerId = String(args.layerId ?? "");
				const layer = findRasterLayer(this.projection(), layerId);
				if (!layer || layer.kind !== "pixel") return;
				const width = layer.width ?? 512;
				const height = layer.height ?? 512;
				this.dispatchEditOp({
					op: "setLayerMask",
					layerId,
					mask: { enabled: true, linked: true, invert: false, width, height },
				});
				return;
			}
			case "appendFilter": {
				const layerId = String(args.layerId ?? "");
				const filterKind = String(args.filterKind ?? "");
				if (!(RASTER_FILTER_KINDS as readonly string[]).includes(filterKind)) return;
				this.dispatchEditOp({
					op: "appendLayerFilter",
					layerId,
					filter: { kind: filterKind as (typeof RASTER_FILTER_KINDS)[number], radius: 8 },
				});
				return;
			}
			case "patchLayer": {
				const layerId = String(args.layerId ?? "");
				const field = String(args.field ?? "");
				const value = args.value ?? args.pressed;
				if (!layerId || !field) return;
				this.dispatchProjectionEdit((doc) => rasterPlayPatchLayerField(doc, layerId, field, value));
				return;
			}
			case "patchLayers": {
				const layerIds = (Array.isArray(args.layerIds) ? args.layerIds : []).map(String).filter(Boolean);
				const field = String(args.field ?? "");
				const value = args.value ?? args.pressed;
				if (!layerIds.length || !field) return;
				this.dispatchProjectionEdit((doc) => {
					let next = doc;
					for (const layerId of layerIds) {
						next = rasterPlayPatchLayerField(next, layerId, field, value);
					}
					return next;
				});
				return;
			}
			case "setLayerBlendMode": {
				this.dispatchEditOp({
					op: "setLayerBlendMode",
					layerId: String(args.layerId),
					blendMode: String(args.blendMode) as RasterBlendMode,
				});
				return;
			}
			case "setCamera": {
				const camera = args.camera as RasterDocument["camera"];
				if (camera) {
					this.dispatchEditOp({ op: "setCamera", camera });
				}
				return;
			}
			case "setCompositeViewport": {
				const width = Number(args.width);
				const height = Number(args.height);
				if (width > 0 && height > 0) {
					this.compositeViewport = { width, height };
					this.bump();
				}
				return;
			}
			case "commitDocument": {
				const document = args.document as RasterDocument;
				if (!document || document.schema !== "raster.document") return;
				const selectLayerId = typeof args.selectLayerId === "string" ? args.selectLayerId : undefined;
				this.dispatchEditOp({ op: "setDocument", document }, selectLayerId);
				return;
			}
			case "selectAll": {
				this.pointerFocus.setSelection(flattenRasterLayers(this.projection().layers).map((layer) => layer.id));
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




//#region 🔖Play

/** @emoji 🛝 Raster playground app. */


export const rasterPlayAppDefinition = createPlaygroundApp({
	id: RASTER_PLAY_APP_ID,
	label: "Raster",
	controllerId: RASTER_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	devHost: {
		playEntryKind: "raster",
		resolveDedupe: ["react", "react-dom", "@semio-tech/raster-react", "three"],
		optimizeDeps: { include: ["react", "react-dom", "@semio-tech/raster-react"] },
	},
	createRuntime: () => {
		const runtime = createProductPlaygroundPlatform(RASTER_PLAY_APP_ID);
			const ctrl = new RasterPlayController(runtime.commandBus, () => runtime.notify());
			const resolved = playgroundResolvedExampleId(RASTER_PLAY_EXAMPLE_DEFAULT_ID);
			const fixtureJson = RASTER_PLAY_FILE_EXAMPLE_JSON_BY_ID[resolved];
			if (fixtureJson) {
				ctrl.run("setActiveExample", { exampleId: resolved });
			}
			runtime.addApp(buildRasterPlayAppRuntime(ctrl));
			return runtime;
	},
	registerBodies: () => {
		registerRasterPlayDeclarativeBodies();
	},
	keybindings: [{ key: "ctrl+a,meta+a", controllerId: RASTER_PLAY_CONTROLLER_ID, command: "selectAll" }],
	bootRenderer: async (pg) => {
		const { bootRasterPlay } = await import("@semio-tech/raster-react/play");
		bootRasterPlay(pg);
	},
});
//#endregion 🔖Play

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("RASTER_PLAY_EXAMPLE_OPTIONS", () => {
		it("includes semio example", () => {
			expect(RASTER_PLAY_EXAMPLE_OPTIONS.some((row) => row.id === "semio")).toBe(true);
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
			const doc = rasterDocumentFromJson(RASTER_PLAY_FILE_EXAMPLE_JSON_BY_ID.semio!);
			const tree = buildRasterPlayMasksTree(doc, [], null, null);
			expect(tree.sections[0]?.items.some((item) => item.id.includes(".mask."))).toBe(true);
		});
	});

	describe("RasterPlayController composite viewport", () => {
		it("stores composite viewport dimensions for navigator overlay", () => {
			const bus = new CommandBus();
			const ctrl = new RasterPlayController(bus, () => {});
			ctrl.run("setCompositeViewport", { width: 1440, height: 900 });
			expect(ctrl.getCompositeViewport()).toEqual({ width: 1440, height: 900 });
		});
	});

	describe("RasterPlayController setSelection", () => {
		it("resolves tree row ids from hierarchy selectionChange", () => {
			const bus = new CommandBus();
			const ctrl = new RasterPlayController(bus, () => {});
			ctrl.run("setActiveExample", { exampleId: "semio" });
			const doc = ctrl.getDocument();
			const layer = flattenRasterLayers(doc.layers)[0]!;
			const rowId = rasterPlayLayersTreeRowId(layer);
			ctrl.run("setSelection", { ids: [rowId] });
			expect(ctrl.getSelectedIds()).toEqual([layer.id]);
		});
	});

	describe("buildRasterPlayLayersTree", () => {
		it("builds nested layer rows", () => {
			const doc = rasterDocumentFromJson(RASTER_PLAY_FILE_EXAMPLE_JSON_BY_ID.semio!);
			const tree = buildRasterPlayLayersTree(doc, [], null, null);
			const emblem = tree.sections[0]?.items.find((item) => item.id.includes("logo-group"));
			expect(emblem?.items?.length).toBeGreaterThan(0);
			expect(tree.selectionChange?.command).toBe("setSelection");
		});
	});

	describe("RasterPlayController layer edits", () => {
		it("adds and deletes layers", () => {
			const bus = new CommandBus();
			const ctrl = new RasterPlayController(bus, () => {});
			ctrl.run("setActiveExample", { exampleId: "semio" });
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
			ctrl.run("setActiveExample", { exampleId: "semio" });
			const exported = ctrl.getDocumentJson();
			ctrl.run("setFixtureJson", { json: exported, resetInteraction: true });
			expect(ctrl.getDocument().id).toBe("semio");
			expect(ctrl.getDocument().assets?.["semio-emblem"]?.mime).toBe("image/png");
		});
	});

	describe("buildRasterPlayInspectorTree", () => {
		it("orders inspector sections specific to general for pixel layers", () => {
			const doc = rasterDocumentFromJson(RASTER_PLAY_FILE_EXAMPLE_JSON_BY_ID.semio!);
			const layer = flattenRasterLayers(doc.layers).find((entry) => entry.kind === "pixel");
			if (!layer) return;
			const tree = buildRasterPlayInspectorTree(doc, [layer.id]);
			const labels = (tree.type === "tree" ? tree.sections : []).map((section) => section.label);
			expect(labels.indexOf("Pixel")).toBeLessThan(labels.indexOf("Layer"));
		});

		it("batch-patches shared fields across multiple layers", () => {
			const doc = rasterDocumentFromJson(RASTER_PLAY_FILE_EXAMPLE_JSON_BY_ID.semio!);
			const layers = flattenRasterLayers(doc.layers).slice(0, 2);
			if (layers.length < 2) return;
			const bus = new CommandBus();
			const ctrl = new RasterPlayController(bus, () => {});
			ctrl.run("setFixtureJson", { json: rasterDocumentToExportJson(doc), resetInteraction: false });
			ctrl.run("patchLayers", { layerIds: layers.map((entry) => entry.id), field: "opacity", value: 0.25 });
			for (const layer of layers) {
				expect(findRasterLayer(ctrl.getDocument(), layer.id)?.opacity).toBe(0.25);
			}
		});
	});

	describe("RASTER_PLAY_FILE_EXAMPLE_JSON_BY_ID", () => {
		for (const fixtureId of Object.keys(RASTER_PLAY_FILE_EXAMPLE_JSON_BY_ID)) {
			it(`loads ${fixtureId} fixture into trees without empty sections`, () => {
				const doc = rasterDocumentFromJson(RASTER_PLAY_FILE_EXAMPLE_JSON_BY_ID[fixtureId]!);
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

export * from "./internal.ts";

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";

/** @emoji 🧩 S program definition for raster. */
export function buildRasterProgramDefinition(): PlatformDefinition {
	return {
		id: "raster",
		name: "Raster",
		apiVersion: "1",
		apps: [{ id: "raster", label: "Raster", controllerId: RASTER_PLAY_CONTROLLER_ID, modes: [{ id: "edit", label: "Edit" }], defaultModeId: "edit" }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension


// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("parseRasterDocument", () => {
		it("parses minimal document", () => {
			const doc = parseRasterDocument({
				schema: "raster.document",
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
				schema: "raster.document",
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
				schema: "raster.document",
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
				schema: "raster.document",
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

	describe("rasterDocumentToSyncJson", () => {
		it("omits camera so zoom does not re-sync compositor layers", () => {
			const doc = defaultRasterDocument("t");
			const zoomed = applyRasterEditOp(doc, { op: "setCamera", camera: { x: 12, y: -4, zoom: 2.5 } });
			expect(rasterDocumentToSyncJson(doc)).toBe(rasterDocumentToSyncJson(zoomed));
			expect(zoomed.camera.zoom).toBe(2.5);
		});
	});

	describe("rasterCameraEqual", () => {
		it("compares camera tuples", () => {
			expect(rasterCameraEqual({ x: 0, y: 0, zoom: 1 }, { x: 0, y: 0, zoom: 1 })).toBe(true);
			expect(rasterCameraEqual({ x: 0, y: 0, zoom: 1 }, { x: 0, y: 0, zoom: 2 })).toBe(false);
		});
	});

	describe("rasterNavigatorFitCamera", () => {
		it("fits visible pixel layers into the navigator viewport", () => {
			const doc = parseRasterDocument({
				schema: "raster.document",
				id: "t",
				camera: { x: 0, y: 0, zoom: 1 },
				layers: [
					{
						kind: "pixel",
						id: "a",
						name: "A",
						visible: true,
						opacity: 1,
						blendMode: "normal",
						width: 200,
						height: 100,
						transform: { x: 0, y: 0, scaleX: 1, scaleY: 1, rotation: 0 },
					},
				],
			});
			const fit = rasterNavigatorFitCamera(doc, { width: 400, height: 200 }, 0);
			expect(fit.zoom).toBeGreaterThan(1);
			expect(fit.x).toBe(0);
			expect(fit.y).toBe(0);
		});
	});

	describe("rasterWheelCamera", () => {
		it("zooms toward the cursor", () => {
			const camera = { x: 0, y: 0, zoom: 1 };
			const viewport = { width: 400, height: 300 };
			const zoomedIn = rasterWheelCamera(camera, viewport, { x: 200, y: 150 }, -100);
			expect(zoomedIn.zoom).toBeGreaterThan(camera.zoom);
			const zoomedOut = rasterWheelCamera(camera, viewport, { x: 200, y: 150 }, 100);
			expect(zoomedOut.zoom).toBeLessThan(camera.zoom);
		});
	});

	describe("rasterNavigatorViewportOverlay", () => {
		it("maps the composite viewport into navigator screen space", () => {
			const contentCamera = { x: 0, y: 0, zoom: 2 };
			const contentViewport = { width: 800, height: 600 };
			const navigatorCamera = { x: 0, y: 0, zoom: 0.5 };
			const navigatorViewport = { width: 200, height: 150 };
			const overlay = rasterNavigatorViewportOverlay(contentCamera, contentViewport, navigatorCamera, navigatorViewport);
			expect(overlay.width).toBeGreaterThan(0);
			expect(overlay.height).toBeGreaterThan(0);
		});
	});

	describe("rasterPlaySelectionIdsFromTreeRowIds", () => {
		it("maps hierarchy and mask tree rows to layer ids", () => {
			const doc = parseRasterDocument({
				schema: "raster.document",
				id: "t",
				camera: { x: 0, y: 0, zoom: 1 },
				layers: [
					{
						kind: "pixel",
						id: "logo",
						name: "Logo",
						visible: true,
						opacity: 1,
						blendMode: "normal",
						transform: defaultRasterTransform(),
						mask: { enabled: true, width: 64, height: 64 },
					},
				],
			});
			const layerRow = rasterPlayLayersTreeRowId(doc.layers[0]!);
			const maskRow = rasterPlayMaskTreeRowId("logo");
			expect(rasterPlaySelectionIdsFromTreeRowIds(doc, [layerRow])).toEqual(["logo"]);
			expect(rasterPlaySelectionIdsFromTreeRowIds(doc, [maskRow])).toEqual(["logo"]);
			expect(rasterPlayTreeRowIdsForSelectionIds(doc, ["logo"])).toEqual([layerRow]);
			expect(rasterPlayMaskTreeRowIdsForSelectionIds(doc, ["logo"])).toEqual([maskRow]);
		});
	});

	describe("rasterPlayLayersTreeHighlightedIdsForKind", () => {
		it("highlights all layers sharing a blend mode", () => {
			const doc = parseRasterDocument({
				schema: "raster.document",
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

	describe("createRasterAppVcsHandler", () => {
		it("materializes inline raster documents", () => {
			const doc = defaultRasterDocument("inline");
			const projection = createRasterAppVcsHandler().materializeProjection({ inline: rasterDocumentToJson(doc) });
			expect(projection.id).toBe("inline");
		});
	});
}
// #endregion 🧪Tests

//#region 🔖MediaExport
function rasterAssetDataUrl(doc: RasterDocument, key: string | undefined): string | null {
	if (!key) return null;
	const asset = doc.assets?.[key];
	if (!asset) return null;
	return asset.data.startsWith("data:") ? asset.data : `data:${asset.mime};base64,${asset.data}`;
}

function rasterDocumentBounds(doc: RasterDocument): { width: number; height: number } {
	let maxX = 512;
	let maxY = 512;
	for (const layer of flattenRasterLayers(doc.layers)) {
		if (!layer.visible) continue;
		const width = layer.width ?? 512;
		const height = layer.height ?? 512;
		maxX = Math.max(maxX, layer.transform.x + width);
		maxY = Math.max(maxY, layer.transform.y + height);
	}
	return { width: Math.max(1, Math.ceil(maxX)), height: Math.max(1, Math.ceil(maxY)) };
}

async function rasterDocumentToPngBytes(doc: RasterDocument): Promise<{ png: Uint8Array; width: number; height: number }> {
	const { width, height } = rasterDocumentBounds(doc);
	if (typeof document === "undefined") return { png: new Uint8Array(0), width, height };
	const canvas = document.createElement("canvas");
	canvas.width = width;
	canvas.height = height;
	const ctx = canvas.getContext("2d");
	if (!ctx) return { png: new Uint8Array(0), width, height };
	for (const layer of flattenRasterLayers(doc.layers)) {
		if (!layer.visible || layer.kind !== "pixel") continue;
		const src = rasterAssetDataUrl(doc, layer.imageKey);
		if (!src) continue;
		await new Promise<void>((resolve) => {
			const image = new Image();
			image.onload = () => {
				ctx.save();
				ctx.globalAlpha = layer.opacity;
				ctx.translate(layer.transform.x, layer.transform.y);
				ctx.rotate(layer.transform.rotation);
				ctx.scale(layer.transform.scaleX, layer.transform.scaleY);
				ctx.drawImage(image, 0, 0, layer.width ?? image.naturalWidth, layer.height ?? image.naturalHeight);
				ctx.restore();
				resolve();
			};
			image.onerror = () => resolve();
			image.src = src;
		});
	}
	const dataUrl = canvas.toDataURL("image/png");
	const blob = await fetch(dataUrl).then((response) => response.blob());
	return { png: new Uint8Array(await blob.arrayBuffer()), width, height };
}

function rasterDocumentToSvg(width: number, height: number, pngDataUrl: string): string {
	return `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${width} ${height}" width="${width}" height="${height}"><image href="${pngDataUrl}" width="${width}" height="${height}"/></svg>`;
}

/** @emoji 💾 Registers raster document SVG/PNG export handlers for the OS media graph. */
export function registerRasterMediaExportHandlers(): void {
	registerOsMediaExportHandler("2d.raster", "png", async (doc) => {
		const raster = doc as RasterDocument;
		const { png } = await rasterDocumentToPngBytes(raster);
		return { data: png, mimeType: "image/png", fileName: "raster.png" };
	});
	registerOsMediaExportHandler("2d.raster", "svg", async (doc) => {
		const raster = doc as RasterDocument;
		const { width, height, png } = await rasterDocumentToPngBytes(raster);
		const pngDataUrl =
			png.length > 0
				? `data:image/png;base64,${btoa(String.fromCharCode(...png))}`
				: await rasterizeSvgMarkupToPngDataUrl(`<svg xmlns="http://www.w3.org/2000/svg" width="${width}" height="${height}"/>`, width, height);
		return {
			data: rasterDocumentToSvg(width, height, pngDataUrl),
			mimeType: "image/svg+xml",
			fileName: "raster.svg",
		};
	});
}
//#endregion 🔖MediaExport
