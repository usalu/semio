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
	AppPointerFocusStore,
	CANVAS_HOVER_SOURCE_CANVAS,
	CANVAS_HOVER_SOURCE_HIERARCHY,
	CANVAS_HOVER_SOURCE_PICK_MENU,
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
	UI_INSPECTOR_MIXED_PLACEHOLDER,
	uiInspectorAllEqual,
	uiInspectorGroupsToTree,
	uiInspectorMixedNumber,
	uiInspectorMixedSelect,
	uiInspectorMixedSlider,
	uiInspectorMixedText,
	uiInspectorMixedToggle,
	uiInspectorReadonlyField,
	type UiInspectorFieldGroup,
	type UiNode,
	type UiTreeItemNode,
	type UiTreeNode,
} from "@semio-tech/framework-playground-core";
import {
	DocumentVcsStore,
	applyJsonReplaceOp,
	createDocumentVcsEnvelope,
	recordJsonProjectionChange,
	type JsonReplaceOp,
} from "@semio-tech/framework-core";
import { bootstrapElementsSurfaceChromeDocument, type TreeDataItem, type TreeDragAndDropController, type TreeDropPosition } from "@semio-tech/ui-react";
import {
	applyDrawEditOp,
	createDrawBooleanLayer,
	createDrawGroupLayer,
	createDrawImageLayer,
	createDrawPathLayer,
	createDrawShapeLayer,
	createDrawTextLayer,
	createDrawTraceLayer,
	defaultDrawDocument,
	drawDocumentFromJson,
	drawDocumentToJson,
	drawLayerDescendantLeafIds,
	drawMatrixToTransform,
	drawPlayBooleanChildRowId,
	drawPlayHoverPayloadFromTreeRowId,
	drawPlayLayerIdFromBooleanChildRowId,
	drawPlayLayerIdFromTreeRowId,
	drawPlayLayersTreeHighlightedIds,
	drawPlayLayersTreeRowId,
	drawTransformToMatrix,
	findDrawLayer,
	flattenDrawLayers,
	hexToRgba,
	layerToPathSegments,
	mutateDrawLayer,
	resolveDrawPlayReorderTarget,
	rgbaToHex,
	type DrawBlendMode,
	type DrawBooleanOp,
	type DrawDocument,
	type DrawHoverPayload,
	type DrawKindHover,
	type DrawLayerNode,
	type DrawToolId,
	DRAW_BLEND_MODES,
	DRAW_BOOLEAN_OPS,
	type DrawLayerKindId,
} from "@semio-tech/draw-core";
import type { DrawShapeKind } from "@semio-tech/draw-core";
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

type DrawCatalogueLayerKind = DrawLayerKindId | `shape:${DrawShapeKind}`;

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

function drawPlayLayerIcon(layer: DrawLayerNode): string {
	if (layer.kind === "group") return "folder";
	if (layer.kind === "boolean") return "combine";
	if (layer.kind === "trace") return "scan-line";
	if (layer.kind === "path") return "pen-tool";
	if (layer.kind === "shape") return "square";
	if (layer.kind === "text") return "type";
	if (layer.kind === "image") return "image";
	return "shapes";
}

function drawPlayBooleanChildTreeItem(doc: DrawDocument, booleanId: string, childId: string, hoverSink?: (payload: DrawHoverPayload) => void): UiTreeItemNode {
	const child = findDrawLayer(doc, childId);
	const rowId = drawPlayBooleanChildRowId(booleanId, childId);
	if (!child) {
		return {
			id: rowId,
			label: `${childId} (missing)`,
			icon: "alert-circle",
			disabled: true,
		};
	}
	return {
		id: rowId,
		label: child.name,
		description: child.kind,
		icon: drawPlayLayerIcon(child),
		draggable: false,
		command: drawPlayCmd("setSelection", { ids: [child.id] }),
		onPointerEnter: hoverSink ? () => hoverSink(drawPlayHoverPayloadFromTreeRowId(doc, rowId)) : undefined,
		onPointerLeave: hoverSink ? () => hoverSink({ id: null, kind: null }) : undefined,
	};
}

function drawPlayLayerTreeItem(
	doc: DrawDocument,
	layer: DrawLayerNode,
	options?: DrawPlayHierarchyBuildOptions,
	hoverSink?: (payload: DrawHoverPayload) => void,
): UiTreeItemNode {
	const rowId = drawPlayLayersTreeRowId(layer);
	const nestedItems =
		layer.kind === "group"
			? layer.children.map((child) => drawPlayLayerTreeItem(doc, child, options, hoverSink))
			: layer.kind === "boolean"
				? layer.children.map((childId) => drawPlayBooleanChildTreeItem(doc, layer.id, childId, hoverSink))
				: undefined;
	return {
		id: rowId,
		label: layer.name,
		description: layer.kind === "boolean" ? layer.op : layer.blendMode,
		icon: drawPlayLayerIcon(layer),
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
		{ id: "draw-play-layers.add.rect", label: "Add Rectangle", icon: "square", command: drawPlayCmd("addLayer", { kind: "shape:rect" }) },
		{ id: "draw-play-layers.add.text", label: "Add Text", icon: "type", command: drawPlayCmd("addLayer", { kind: "text" }) },
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
			id: "draw-play-catalogue.rect",
			label: "Rectangle",
			icon: "square",
			draggable: true,
			dragData: { [DRAW_LAYER_KIND_DRAG_MIME]: JSON.stringify({ kind: "shape:rect" }) },
		},
		{
			id: "draw-play-catalogue.ellipse",
			label: "Ellipse",
			icon: "circle",
			draggable: true,
			dragData: { [DRAW_LAYER_KIND_DRAG_MIME]: JSON.stringify({ kind: "shape:ellipse" }) },
		},
		{
			id: "draw-play-catalogue.line",
			label: "Line",
			icon: "minus",
			draggable: true,
			dragData: { [DRAW_LAYER_KIND_DRAG_MIME]: JSON.stringify({ kind: "shape:line" }) },
		},
		{
			id: "draw-play-catalogue.polygon",
			label: "Polygon",
			icon: "pentagon",
			draggable: true,
			dragData: { [DRAW_LAYER_KIND_DRAG_MIME]: JSON.stringify({ kind: "shape:polygon" }) },
		},
		{
			id: "draw-play-catalogue.text",
			label: "Text",
			icon: "type",
			draggable: true,
			dragData: { [DRAW_LAYER_KIND_DRAG_MIME]: JSON.stringify({ kind: "text" }) },
		},
		{
			id: "draw-play-catalogue.image",
			label: "Image",
			icon: "image",
			draggable: true,
			dragData: { [DRAW_LAYER_KIND_DRAG_MIME]: JSON.stringify({ kind: "image" }) },
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

function drawPlayInspectorPatch(layerIds: readonly string[], field: string) {
	return drawPlayCmd("patchLayers", { layerIds, field });
}

function drawPlayInspectorNumberField(
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
			onChange: drawPlayInspectorPatch(layerIds, field),
		},
	};
}

function drawPlayInspectorTextField(
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
			onChange: drawPlayInspectorPatch(layerIds, field),
		},
	};
}

function drawPlayLayersUniformKind(layers: readonly DrawLayerNode[]): DrawLayerNode[] | null {
	if (!layers.length) return null;
	const kindKey = drawPlayLayerKindLabel(layers[0]!);
	return layers.every((layer) => drawPlayLayerKindLabel(layer) === kindKey) ? [...layers] : null;
}

function drawPlayLayerKindLabel(layer: DrawLayerNode): string {
	if (layer.kind === "shape") return `shape:${layer.shapeKind}`;
	return layer.kind;
}

function drawPlayInspectorKindSpecificGroup(doc: DrawDocument, layers: readonly DrawLayerNode[]): UiInspectorFieldGroup | null {
	const uniformLayers = drawPlayLayersUniformKind(layers);
	if (!uniformLayers) return null;
	const layer = uniformLayers[0]!;
	const layerIds = uniformLayers.map((entry) => entry.id);
	const fields: UiNode[] = [];
	if (layer.kind === "boolean") {
		const ops = uniformLayers.map((entry) => (entry.kind === "boolean" ? entry.op : ""));
		const opMixed = uiInspectorMixedSelect(ops);
		fields.push({
			type: "field",
			id: "draw-play-inspector.boolean-op",
			label: "Boolean Op",
			child: {
				type: "select",
				id: "draw-play-inspector.boolean-op.select",
				value: opMixed.value,
				placeholder: opMixed.placeholder,
				items: DRAW_BOOLEAN_OPS.map((op) => ({ value: op, label: op })),
				onChange: drawPlayInspectorPatch(layerIds, "booleanOp"),
			},
		});
		const childLabels = uniformLayers
			.flatMap((entry) =>
				entry.kind === "boolean"
					? entry.children
							.map((childId) => findDrawLayer(doc, childId))
							.filter((child): child is DrawLayerNode => Boolean(child))
							.map((child) => child.name || child.id)
					: [],
			)
			.join(", ");
		fields.push(uiInspectorReadonlyField("draw-play-inspector.boolean-children", "Children", childLabels || "—"));
		return { id: "draw-play-inspector.kind.boolean", label: "Boolean", fields };
	}
	if (layer.kind === "trace") {
		const thresholds = uniformLayers.map((entry) => (entry.kind === "trace" ? entry.params.threshold : 0));
		const simplifies = uniformLayers.map((entry) => (entry.kind === "trace" ? entry.params.simplifyEpsilon : 0));
		const thresholdMixed = uiInspectorMixedSlider(thresholds);
		const simplifyMixed = uiInspectorMixedSlider(simplifies);
		fields.push(
			{
				type: "field",
				id: "draw-play-inspector.trace-threshold",
				label: "Trace Threshold",
				child: {
					type: "slider",
					id: "draw-play-inspector.trace-threshold.slider",
					value: thresholdMixed.uniform ? thresholdMixed.value : 0,
					min: 0,
					max: 1,
					step: 0.01,
					onChange: drawPlayInspectorPatch(layerIds, "traceThreshold"),
				},
			},
			{
				type: "field",
				id: "draw-play-inspector.trace-simplify",
				label: "Simplify",
				child: {
					type: "slider",
					id: "draw-play-inspector.trace-simplify.slider",
					value: simplifyMixed.uniform ? simplifyMixed.value : 0,
					min: 0,
					max: 10,
					step: 0.1,
					onChange: drawPlayInspectorPatch(layerIds, "traceSimplify"),
				},
			},
			uiInspectorReadonlyField("draw-play-inspector.trace-source", "Source Key", layer.sourceKey),
		);
		return { id: "draw-play-inspector.kind.trace", label: "Trace", fields };
	}
	if (layer.kind === "shape" && layer.shapeKind === "rect" && layer.rect) {
		fields.push(
			drawPlayInspectorNumberField(layerIds, "draw-play-inspector.rect-width", "Width", uniformLayers.map((entry) => (entry.kind === "shape" && entry.rect ? entry.rect.width : 0)), "rectWidth"),
			drawPlayInspectorNumberField(layerIds, "draw-play-inspector.rect-height", "Height", uniformLayers.map((entry) => (entry.kind === "shape" && entry.rect ? entry.rect.height : 0)), "rectHeight"),
		);
		return { id: "draw-play-inspector.kind.rect", label: "Rectangle", fields };
	}
	if (layer.kind === "shape" && layer.shapeKind === "ellipse" && layer.ellipse) {
		fields.push(
			drawPlayInspectorNumberField(layerIds, "draw-play-inspector.ellipse-rx", "RX", uniformLayers.map((entry) => (entry.kind === "shape" && entry.ellipse ? entry.ellipse.rx : 0)), "ellipseRx"),
			drawPlayInspectorNumberField(layerIds, "draw-play-inspector.ellipse-ry", "RY", uniformLayers.map((entry) => (entry.kind === "shape" && entry.ellipse ? entry.ellipse.ry : 0)), "ellipseRy"),
		);
		return { id: "draw-play-inspector.kind.ellipse", label: "Ellipse", fields };
	}
	if (layer.kind === "shape" && layer.shapeKind === "circle" && layer.circle) {
		fields.push(drawPlayInspectorNumberField(layerIds, "draw-play-inspector.circle-r", "R", uniformLayers.map((entry) => (entry.kind === "shape" && entry.circle ? entry.circle.r : 0)), "circleR"));
		return { id: "draw-play-inspector.kind.circle", label: "Circle", fields };
	}
	if (layer.kind === "shape" && layer.shapeKind === "line" && layer.line) {
		fields.push(
			drawPlayInspectorNumberField(layerIds, "draw-play-inspector.line-x1", "X1", uniformLayers.map((entry) => (entry.kind === "shape" && entry.line ? entry.line.x1 : 0)), "lineX1"),
			drawPlayInspectorNumberField(layerIds, "draw-play-inspector.line-y1", "Y1", uniformLayers.map((entry) => (entry.kind === "shape" && entry.line ? entry.line.y1 : 0)), "lineY1"),
			drawPlayInspectorNumberField(layerIds, "draw-play-inspector.line-x2", "X2", uniformLayers.map((entry) => (entry.kind === "shape" && entry.line ? entry.line.x2 : 0)), "lineX2"),
			drawPlayInspectorNumberField(layerIds, "draw-play-inspector.line-y2", "Y2", uniformLayers.map((entry) => (entry.kind === "shape" && entry.line ? entry.line.y2 : 0)), "lineY2"),
		);
		return { id: "draw-play-inspector.kind.line", label: "Line", fields };
	}
	if (layer.kind === "shape" && layer.shapeKind === "polygon") {
		fields.push(
			uiInspectorReadonlyField(
				"draw-play-inspector.polygon-points",
				"Polygon Points",
				`${layer.polygon?.points.length ?? 0} points — edit on canvas`,
			),
		);
		return { id: "draw-play-inspector.kind.polygon", label: "Polygon", fields };
	}
	if (layer.kind === "text") {
		fields.push(
			drawPlayInspectorTextField(layerIds, "draw-play-inspector.text-content", "Content", uniformLayers.map((entry) => (entry.kind === "text" ? entry.content : "")), "textContent"),
			drawPlayInspectorNumberField(layerIds, "draw-play-inspector.text-size", "Size", uniformLayers.map((entry) => (entry.kind === "text" ? entry.size : 0)), "textSize"),
		);
		return { id: "draw-play-inspector.kind.text", label: "Text", fields };
	}
	if (layer.kind === "image") {
		fields.push(
			uiInspectorReadonlyField("draw-play-inspector.image-key", "Image Key", layer.imageKey),
			uiInspectorReadonlyField("draw-play-inspector.image-width", "Width", String(layer.width)),
			uiInspectorReadonlyField("draw-play-inspector.image-height", "Height", String(layer.height)),
		);
		return { id: "draw-play-inspector.kind.image", label: "Image", fields };
	}
	if (layer.kind === "path") {
		fields.push(uiInspectorReadonlyField("draw-play-inspector.path-segments", "Segment Count", String(layerToPathSegments(layer).length)));
		return { id: "draw-play-inspector.kind.path", label: "Path", fields };
	}
	if (layer.kind === "group") {
		fields.push(uiInspectorReadonlyField("draw-play-inspector.group-children", "Children Count", String(layer.children.length)));
		return { id: "draw-play-inspector.kind.group", label: "Group", fields };
	}
	return null;
}

function drawPlayInspectorPositionGroup(layers: readonly DrawLayerNode[]): UiInspectorFieldGroup | null {
	const uniformLayers = drawPlayLayersUniformKind(layers);
	if (!uniformLayers) return null;
	const layer = uniformLayers[0]!;
	const layerIds = uniformLayers.map((entry) => entry.id);
	const fields: UiNode[] = [];
	if (layer.kind === "shape" && layer.shapeKind === "rect" && layer.rect) {
		fields.push(
			drawPlayInspectorNumberField(layerIds, "draw-play-inspector.rect-x", "X", uniformLayers.map((entry) => (entry.kind === "shape" && entry.rect ? entry.rect.x : 0)), "rectX"),
			drawPlayInspectorNumberField(layerIds, "draw-play-inspector.rect-y", "Y", uniformLayers.map((entry) => (entry.kind === "shape" && entry.rect ? entry.rect.y : 0)), "rectY"),
		);
	}
	if (layer.kind === "shape" && layer.shapeKind === "ellipse" && layer.ellipse) {
		fields.push(
			drawPlayInspectorNumberField(layerIds, "draw-play-inspector.ellipse-cx", "CX", uniformLayers.map((entry) => (entry.kind === "shape" && entry.ellipse ? entry.ellipse.cx : 0)), "ellipseCx"),
			drawPlayInspectorNumberField(layerIds, "draw-play-inspector.ellipse-cy", "CY", uniformLayers.map((entry) => (entry.kind === "shape" && entry.ellipse ? entry.ellipse.cy : 0)), "ellipseCy"),
		);
	}
	if (layer.kind === "shape" && layer.shapeKind === "circle" && layer.circle) {
		fields.push(
			drawPlayInspectorNumberField(layerIds, "draw-play-inspector.circle-cx", "CX", uniformLayers.map((entry) => (entry.kind === "shape" && entry.circle ? entry.circle.cx : 0)), "circleCx"),
			drawPlayInspectorNumberField(layerIds, "draw-play-inspector.circle-cy", "CY", uniformLayers.map((entry) => (entry.kind === "shape" && entry.circle ? entry.circle.cy : 0)), "circleCy"),
		);
	}
	if (layer.kind === "text") {
		fields.push(
			drawPlayInspectorNumberField(layerIds, "draw-play-inspector.text-x", "X", uniformLayers.map((entry) => (entry.kind === "text" ? entry.x : 0)), "textX"),
			drawPlayInspectorNumberField(layerIds, "draw-play-inspector.text-y", "Y", uniformLayers.map((entry) => (entry.kind === "text" ? entry.y : 0)), "textY"),
		);
	}
	if (fields.length === 0) return null;
	return { id: "draw-play-inspector.position", label: "Position", fields };
}

function drawPlayInspectorOrientationGroup(layers: readonly DrawLayerNode[]): UiInspectorFieldGroup {
	const layerIds = layers.map((entry) => entry.id);
	const fields: UiNode[] = [
		drawPlayInspectorNumberField(layerIds, "draw-play-inspector.transform-x", "Position X", layers.map((entry) => entry.transform.x), "transformX"),
		drawPlayInspectorNumberField(layerIds, "draw-play-inspector.transform-y", "Position Y", layers.map((entry) => entry.transform.y), "transformY"),
		drawPlayInspectorNumberField(layerIds, "draw-play-inspector.transform-scale-x", "Scale X", layers.map((entry) => entry.transform.scaleX), "transformScaleX"),
		drawPlayInspectorNumberField(layerIds, "draw-play-inspector.transform-scale-y", "Scale Y", layers.map((entry) => entry.transform.scaleY), "transformScaleY"),
		drawPlayInspectorNumberField(layerIds, "draw-play-inspector.transform-rotation", "Rotation", layers.map((entry) => entry.transform.rotation), "transformRotation"),
		drawPlayInspectorNumberField(layerIds, "draw-play-inspector.matrix-a", "Matrix A", layers.map((entry) => drawTransformToMatrix(entry.transform)[0]), "transformMatrixA"),
		drawPlayInspectorNumberField(layerIds, "draw-play-inspector.matrix-b", "Matrix B", layers.map((entry) => drawTransformToMatrix(entry.transform)[1]), "transformMatrixB"),
		drawPlayInspectorNumberField(layerIds, "draw-play-inspector.matrix-c", "Matrix C", layers.map((entry) => drawTransformToMatrix(entry.transform)[2]), "transformMatrixC"),
		drawPlayInspectorNumberField(layerIds, "draw-play-inspector.matrix-d", "Matrix D", layers.map((entry) => drawTransformToMatrix(entry.transform)[3]), "transformMatrixD"),
		drawPlayInspectorNumberField(layerIds, "draw-play-inspector.matrix-e", "Matrix E", layers.map((entry) => drawTransformToMatrix(entry.transform)[4]), "transformMatrixE"),
		drawPlayInspectorNumberField(layerIds, "draw-play-inspector.matrix-f", "Matrix F", layers.map((entry) => drawTransformToMatrix(entry.transform)[5]), "transformMatrixF"),
	];
	return { id: "draw-play-inspector.orientation", label: "Orientation", fields };
}

function drawPlayInspectorAppearanceGroup(layers: readonly DrawLayerNode[]): UiInspectorFieldGroup {
	const layerIds = layers.map((entry) => entry.id);
	const fillColors = layers.map((entry) => (entry.attributes.fill?.kind === "solid" ? rgbaToHex(entry.attributes.fill.color) : "#000000"));
	const fillAlphas = layers.map((entry) => (entry.attributes.fill?.kind === "solid" ? entry.attributes.fill.color[3] : 1));
	const strokeColors = layers.map((entry) => (entry.attributes.stroke ? rgbaToHex(entry.attributes.stroke.color) : "#000000"));
	const strokeWidths = layers.map((entry) => entry.attributes.stroke?.width ?? 1);
	const fillAlphaMixed = uiInspectorMixedSlider(fillAlphas);
	return {
		id: "draw-play-inspector.appearance",
		label: "Appearance",
		fields: [
			drawPlayInspectorTextField(layerIds, "draw-play-inspector.fill", "Fill", fillColors, "fillColor"),
			{
				type: "field",
				id: "draw-play-inspector.fill-alpha",
				label: "Fill Alpha",
				child: {
					type: "slider",
					id: "draw-play-inspector.fill-alpha.slider",
					value: fillAlphaMixed.uniform ? fillAlphaMixed.value : 0,
					min: 0,
					max: 1,
					step: 0.01,
					onChange: drawPlayInspectorPatch(layerIds, "fillAlpha"),
				},
			},
			drawPlayInspectorTextField(layerIds, "draw-play-inspector.stroke", "Stroke", strokeColors, "strokeColor"),
			drawPlayInspectorNumberField(layerIds, "draw-play-inspector.stroke-width", "Stroke Width", strokeWidths, "strokeWidth"),
		],
	};
}

function drawPlayInspectorLayerGroup(layers: readonly DrawLayerNode[]): UiInspectorFieldGroup {
	const layerIds = layers.map((entry) => entry.id);
	const names = layers.map((entry) => entry.name);
	const kinds = layers.map((entry) => drawPlayLayerKindLabel(entry));
	const visibles = layers.map((entry) => entry.visible);
	const locked = layers.map((entry) => entry.locked);
	const opacities = layers.map((entry) => entry.opacity);
	const blends = layers.map((entry) => entry.blendMode);
	const visibleMixed = uiInspectorMixedToggle(visibles);
	const lockedMixed = uiInspectorMixedToggle(locked);
	const opacityMixed = uiInspectorMixedSlider(opacities);
	const blendMixed = uiInspectorMixedSelect(blends);
	const kindMixed = uiInspectorMixedText(kinds);
	return {
		id: "draw-play-inspector.layer",
		label: "Layer",
		fields: [
			drawPlayInspectorTextField(layerIds, "draw-play-inspector.name", "Name", names, "name"),
			uiInspectorReadonlyField("draw-play-inspector.id", "Id", layerIds.length === 1 ? (layerIds[0] ?? "") : `${layerIds.length} selected`),
			uiInspectorReadonlyField("draw-play-inspector.kind", "Kind", kindMixed.uniform ? (kinds[0] ?? "") : kindMixed.placeholder ?? UI_INSPECTOR_MIXED_PLACEHOLDER),
			{
				type: "field",
				id: "draw-play-inspector.visible",
				label: "Visible",
				child: {
					type: "toggle",
					id: "draw-play-inspector.visible.toggle",
					iconId: "check",
					pressed: visibleMixed.pressed,
					text: visibleMixed.uniform ? undefined : UI_INSPECTOR_MIXED_PLACEHOLDER,
					onChange: drawPlayInspectorPatch(layerIds, "visible"),
				},
			},
			{
				type: "field",
				id: "draw-play-inspector.locked",
				label: "Locked",
				child: {
					type: "toggle",
					id: "draw-play-inspector.locked.toggle",
					iconId: "check",
					pressed: lockedMixed.pressed,
					text: lockedMixed.uniform ? undefined : UI_INSPECTOR_MIXED_PLACEHOLDER,
					onChange: drawPlayInspectorPatch(layerIds, "locked"),
				},
			},
			{
				type: "field",
				id: "draw-play-inspector.opacity",
				label: "Opacity",
				child: {
					type: "slider",
					id: "draw-play-inspector.opacity.slider",
					value: opacityMixed.uniform ? opacityMixed.value : 0,
					min: 0,
					max: 1,
					step: 0.01,
					onChange: drawPlayInspectorPatch(layerIds, "opacity"),
				},
			},
			{
				type: "field",
				id: "draw-play-inspector.blend",
				label: "Blend",
				child: {
					type: "select",
					id: "draw-play-inspector.blend.select",
					value: blendMixed.value,
					placeholder: blendMixed.placeholder,
					items: DRAW_BLEND_MODES.map((mode) => ({ value: mode, label: mode })),
					onChange: drawPlayInspectorPatch(layerIds, "blendMode"),
				},
			},
		],
	};
}

export function buildDrawPlayInspectorTree(doc: DrawDocument, selectedIds: readonly string[]): UiNode {
	const layers = selectedIds.map((layerId) => findDrawLayer(doc, layerId)).filter((layer): layer is DrawLayerNode => Boolean(layer));
	if (!layers.length) {
		return uiDeclarativeSectionsToTree([
			{
				type: "section",
				id: "draw-play-inspector.empty",
				label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
				children: [{ type: "text", value: "Select a layer in the hierarchy." }],
			},
		]);
	}
	const groups: UiInspectorFieldGroup[] = [];
	const kindSpecific = drawPlayInspectorKindSpecificGroup(doc, layers);
	if (kindSpecific) groups.push(kindSpecific);
	const position = drawPlayInspectorPositionGroup(layers);
	if (position) groups.push(position);
	groups.push(drawPlayInspectorOrientationGroup(layers));
	groups.push(drawPlayInspectorAppearanceGroup(layers));
	groups.push(drawPlayInspectorLayerGroup(layers));
	return uiInspectorGroupsToTree(groups);
}

function drawPlayCreateLayerByKind(kind: DrawCatalogueLayerKind): DrawLayerNode {
	if (kind === "group") return createDrawGroupLayer();
	if (kind === "boolean") return createDrawBooleanLayer();
	if (kind === "trace") return createDrawTraceLayer("Trace", "emblem-trace-source");
	if (kind === "text") return createDrawTextLayer();
	if (kind === "image") return createDrawImageLayer("Image", "emblem-trace-source");
	if (kind === "shape:rect") return createDrawShapeLayer("Rectangle", { shapeKind: "rect", rect: { x: 0, y: 0, width: 100, height: 60 } });
	if (kind === "shape:ellipse") return createDrawShapeLayer("Ellipse", { shapeKind: "ellipse", ellipse: { cx: 50, cy: 40, rx: 50, ry: 40 } });
	if (kind === "shape:line") return createDrawShapeLayer("Line", { shapeKind: "line", line: { x1: 0, y1: 0, x2: 100, y2: 60 } });
	if (kind === "shape:polygon")
		return createDrawShapeLayer("Polygon", {
			shapeKind: "polygon",
			polygon: {
				points: [
					[0, 0],
					[80, 0],
					[100, 60],
					[20, 60],
				],
			},
		});
	return createDrawPathLayer();
}

function drawPlayAddLayerOp(kind: DrawCatalogueLayerKind, layer: DrawLayerNode, parentId?: string, index?: number) {
	if (kind === "group") return { op: "addGroupLayer", parentId, index, layer } as const;
	if (kind === "boolean") return { op: "addBooleanLayer", parentId, index, layer } as const;
	if (kind === "trace") return { op: "addTraceLayer", parentId, index, layer } as const;
	if (kind === "text") return { op: "addTextLayer", parentId, index, layer } as const;
	if (kind === "image") return { op: "addImageLayer", parentId, index, layer } as const;
	if (kind.startsWith("shape:")) return { op: "addShapeLayer", parentId, index, layer } as const;
	return { op: "addPathLayer", parentId, index, layer } as const;
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
			const layerId = sourceItem.dragData?.["application/x-semio-draw-layer-id"] ?? drawPlayLayerIdFromTreeRowId(sourceItem.id) ?? drawPlayLayerIdFromBooleanChildRowId(sourceItem.id);
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

/** @emoji 🔧 Applies one inspector field patch to a single draw layer. */
function drawPlayPatchLayerField(doc: DrawDocument, layerId: string, field: string, value: unknown): DrawDocument {
	const layer = findDrawLayer(doc, layerId);
	if (!layer) return doc;
	switch (field) {
		case "name":
			return applyDrawEditOp(doc, { op: "setLayerName", layerId, name: String(value ?? "") });
		case "opacity":
			return applyDrawEditOp(doc, { op: "setLayerOpacity", layerId, opacity: Number(value) });
		case "blendMode":
			return applyDrawEditOp(doc, { op: "setLayerBlendMode", layerId, blendMode: String(value) as DrawBlendMode });
		case "visible":
			return applyDrawEditOp(doc, { op: "setLayerVisible", layerId, visible: Boolean(value) });
		case "locked":
			return applyDrawEditOp(doc, { op: "setLayerLocked", layerId, locked: Boolean(value) });
		case "booleanOp":
			return applyDrawEditOp(doc, { op: "setBooleanOp", layerId, booleanOp: String(value) as DrawBooleanOp });
		case "fillColor": {
			const alpha = layer.attributes.fill?.kind === "solid" ? layer.attributes.fill.color[3] : 1;
			return applyDrawEditOp(doc, { op: "setFill", layerId, fill: { kind: "solid", color: [...hexToRgba(String(value ?? "#000000"), alpha)] } });
		}
		case "fillAlpha": {
			const color = layer.attributes.fill?.kind === "solid" ? [...layer.attributes.fill.color] : [0, 0, 0, 1];
			color[3] = Number(value);
			return applyDrawEditOp(doc, { op: "setFill", layerId, fill: { kind: "solid", color: color as [number, number, number, number] } });
		}
		case "strokeColor": {
			const stroke = layer.attributes.stroke ?? { color: [0, 0, 0, 1] as [number, number, number, number], width: 1, cap: "butt" as const, join: "miter" as const };
			return applyDrawEditOp(doc, {
				op: "setStroke",
				layerId,
				stroke: { ...stroke, color: [...hexToRgba(String(value ?? "#000000"), stroke.color[3])] as [number, number, number, number] },
			});
		}
		case "strokeWidth": {
			const stroke = layer.attributes.stroke ?? { color: [0, 0, 0, 1] as [number, number, number, number], width: 1, cap: "butt" as const, join: "miter" as const };
			return applyDrawEditOp(doc, { op: "setStroke", layerId, stroke: { ...stroke, width: Number(value) } });
		}
		case "transformX":
		case "transformY":
		case "transformScaleX":
		case "transformScaleY":
		case "transformRotation": {
			const key =
				field === "transformX"
					? "x"
					: field === "transformY"
						? "y"
						: field === "transformScaleX"
							? "scaleX"
							: field === "transformScaleY"
								? "scaleY"
								: "rotation";
			return applyDrawEditOp(doc, { op: "setLayerTransform", layerId, transform: { ...layer.transform, [key]: Number(value) } });
		}
		case "transformMatrixA":
		case "transformMatrixB":
		case "transformMatrixC":
		case "transformMatrixD":
		case "transformMatrixE":
		case "transformMatrixF": {
			const matrix = drawTransformToMatrix(layer.transform);
			const index =
				field === "transformMatrixA"
					? 0
					: field === "transformMatrixB"
						? 1
						: field === "transformMatrixC"
							? 2
							: field === "transformMatrixD"
								? 3
								: field === "transformMatrixE"
									? 4
									: 5;
			const next: [number, number, number, number, number, number] = [...matrix];
			next[index] = Number(value);
			return applyDrawEditOp(doc, { op: "setLayerTransform", layerId, transform: drawMatrixToTransform(next) });
		}
		case "textContent":
			if (layer.kind !== "text") return doc;
			return mutateDrawLayer(doc, layerId, (node) => (node.kind === "text" ? { ...node, content: String(value ?? "") } : node));
		case "textSize":
			if (layer.kind !== "text") return doc;
			return mutateDrawLayer(doc, layerId, (node) => (node.kind === "text" ? { ...node, size: Number(value) } : node));
		case "textX":
		case "textY":
			if (layer.kind !== "text") return doc;
			return mutateDrawLayer(doc, layerId, (node) => {
				if (node.kind !== "text") return node;
				return field === "textX" ? { ...node, x: Number(value) } : { ...node, y: Number(value) };
			});
		case "rectX":
		case "rectY":
		case "rectWidth":
		case "rectHeight":
			if (layer.kind !== "shape" || !layer.rect) return doc;
			return mutateDrawLayer(doc, layerId, (node) => {
				if (node.kind !== "shape" || !node.rect) return node;
				const rect = { ...node.rect };
				if (field === "rectX") rect.x = Number(value);
				if (field === "rectY") rect.y = Number(value);
				if (field === "rectWidth") rect.width = Number(value);
				if (field === "rectHeight") rect.height = Number(value);
				return { ...node, rect };
			});
		case "ellipseCx":
		case "ellipseCy":
		case "ellipseRx":
		case "ellipseRy":
			if (layer.kind !== "shape" || !layer.ellipse) return doc;
			return mutateDrawLayer(doc, layerId, (node) => {
				if (node.kind !== "shape" || !node.ellipse) return node;
				const ellipse = { ...node.ellipse };
				if (field === "ellipseCx") ellipse.cx = Number(value);
				if (field === "ellipseCy") ellipse.cy = Number(value);
				if (field === "ellipseRx") ellipse.rx = Number(value);
				if (field === "ellipseRy") ellipse.ry = Number(value);
				return { ...node, ellipse };
			});
		case "circleCx":
		case "circleCy":
		case "circleR":
			if (layer.kind !== "shape" || !layer.circle) return doc;
			return mutateDrawLayer(doc, layerId, (node) => {
				if (node.kind !== "shape" || !node.circle) return node;
				const circle = { ...node.circle };
				if (field === "circleCx") circle.cx = Number(value);
				if (field === "circleCy") circle.cy = Number(value);
				if (field === "circleR") circle.r = Number(value);
				return { ...node, circle };
			});
		case "lineX1":
		case "lineY1":
		case "lineX2":
		case "lineY2":
			if (layer.kind !== "shape" || !layer.line) return doc;
			return mutateDrawLayer(doc, layerId, (node) => {
				if (node.kind !== "shape" || !node.line) return node;
				const line = { ...node.line };
				if (field === "lineX1") line.x1 = Number(value);
				if (field === "lineY1") line.y1 = Number(value);
				if (field === "lineX2") line.x2 = Number(value);
				if (field === "lineY2") line.y2 = Number(value);
				return { ...node, line };
			});
		case "traceThreshold": {
			if (layer.kind !== "trace") return doc;
			return applyDrawEditOp(doc, {
				op: "setTraceParams",
				layerId,
				params: { ...layer.params, threshold: Number(value) },
			});
		}
		case "traceSimplify": {
			if (layer.kind !== "trace") return doc;
			return applyDrawEditOp(doc, {
				op: "setTraceParams",
				layerId,
				params: { ...layer.params, simplifyEpsilon: Number(value) },
			});
		}
		default:
			return doc;
	}
}

export class DrawPlayController extends Controller implements PlaygroundFixtureHost {
	readonly mainMode = new ModeRuntime("main", "Draw", undefined);
	private readonly docStore = new DocumentVcsStore<DrawDocument, JsonReplaceOp<DrawDocument>>({
		envelope: createDocumentVcsEnvelope("draw.document/v1", "draw-play", DRAW_PLAY_EMPTY_DOCUMENT),
		applyOp: applyJsonReplaceOp,
	});
	private selectedIds: string[] = [];
	private hoveredId: string | null = null;
	private hoveredKind: DrawKindHover | null = null;
	private readonly pointerFocus = new AppPointerFocusStore<string>();
	private interactionRevision = 0;
	private listeners = new Set<() => void>();
	private hostBridge: DrawPlayHostBridge | null = null;

	constructor(bus: CommandBus, notifyPlatform: () => void) {
		super(DRAW_PLAY_CONTROLLER_ID, bus, notifyPlatform);
		this.rebuildShellMode();
	}

	private rebuildShellMode(): void {
		this.mainMode.tools = this.buildTools();
		this.mainMode.windowKinds = [
			new WindowKindRuntime(DRAW_PLAY_WINDOW_KIND_COMPOSITE, "Canvas", DRAW_PLAY_BODY_KEY_COMPOSITE),
			new WindowKindRuntime(DRAW_PLAY_WINDOW_KIND_NAVIGATOR, "Navigator", DRAW_PLAY_BODY_KEY_NAVIGATOR),
		];
	}

	private projection(): DrawDocument {
		return this.docStore.projection();
	}

	private commitDocument(next: DrawDocument, selectLayerId?: string, resetSelection = false): void {
		recordJsonProjectionChange(this.docStore, next);
		if (resetSelection) this.selectedIds = next.layers[0] ? [next.layers[0].id] : [];
		else if (selectLayerId) this.selectedIds = [selectLayerId];
		this.bump();
	}

	private buildTools(): AppTools {
		const activeTool = this.projection().activeTool ?? "selectDirect";
		const toolToggle = (id: string, label: string, iconId: string, tool: DrawToolId): ToolLeaf => ({
			id,
			kind: "toggle",
			label,
			iconId,
			pressed: activeTool === tool,
			controllerId: DRAW_PLAY_CONTROLLER_ID,
			command: "setActiveTool",
			args: { tool },
		});
		return [
			toolCollection("open", "folder-open", [drawPlayTool("draw-import", "Import Draw", "folder-open", "loadRequest")]),
			toolCollection("save", "save", [drawPlayTool("draw-export", "Export Draw", "save", "saveDownload")]),
			toolCollection("selection", "mouse-pointer-2", [
				toolToggle("selectDirect", "Direct", "mouse-pointer", "selectDirect"),
				toolToggle("selectMarquee", "Marquee", "square-dashed", "selectMarquee"),
				toolToggle("selectLasso", "Lasso", "lasso", "selectLasso"),
			]),
			toolCollection("draw", "pen-tool", [
				toolToggle("pen", "Pen", "pen-tool", "pen"),
				toolToggle("shapeRect", "Rectangle", "square", "shapeRect"),
				toolToggle("shapeEllipse", "Ellipse", "circle", "shapeEllipse"),
				toolToggle("shapeLine", "Line", "minus", "shapeLine"),
				toolToggle("shapePolygon", "Polygon", "pentagon", "shapePolygon"),
			]),
			toolCollection("boolean", "combine", [
				drawPlayTool("booleanCombine", "Combine", "combine", "combineBoolean", { op: "union" }),
			]),
			toolCollection("trace", "scan-line", [toolToggle("trace", "Trace", "scan-line", "trace")]),
			toolCollection("transform", "move", [toolToggle("transformMove", "Pan", "move", "transformMove")]),
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

	getDocument(): DrawDocument {
		return this.docStore.projection();
	}

	getDocumentVcsStore(): DocumentVcsStore<DrawDocument, JsonReplaceOp<DrawDocument>> {
		return this.docStore;
	}

	getDocumentJson(): string {
		return drawDocumentToJson(this.projection());
	}

	setHostBridge(bridge: DrawPlayHostBridge | null): void {
		this.hostBridge = bridge;
	}

	private applyDocument(doc: DrawDocument, resetSelection = false): void {
		this.commitDocument(doc, undefined, resetSelection);
	}

	getSelectedIds(): readonly string[] {
		return this.selectedIds;
	}

	getHoveredId(): string | null {
		return this.pointerFocus.getSnapshot().hover ?? this.hoveredId;
	}

	getHoveredKind(): DrawKindHover | null {
		return this.hoveredKind;
	}

	getFixtureCatalog(): PlaygroundFixtureCatalog | null {
		if (isPlaygroundFixtureLocked()) return null;
		return {
			activeFixtureId: playgroundResolvedFixtureId(
				this.projection().id === "empty" ? PLAYGROUND_NO_FIXTURE_ID : this.projection().id,
				DRAW_PLAY_FIXTURE_DEFAULT_ID,
			),
			options: DRAW_PLAY_FIXTURE_OPTIONS,
		};
	}

	private patchDocument(edit: (doc: DrawDocument) => DrawDocument, selectLayerId?: string): void {
		this.commitDocument(edit(this.projection()), selectLayerId);
	}

	run(command: string, args: Record<string, unknown> = {}): void {
		switch (command) {
			case "setActiveFixture": {
				const fixtureId = String(args.fixtureId ?? "");
				if (isPlaygroundNoFixtureId(fixtureId)) {
					this.commitDocument(DRAW_PLAY_EMPTY_DOCUMENT, undefined, true);
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
				const sourceId =
					typeof args.sourceId === "string"
						? args.sourceId
						: args.fromPickMenu === true
							? CANVAS_HOVER_SOURCE_PICK_MENU
							: args.fromHierarchy === true
								? CANVAS_HOVER_SOURCE_HIERARCHY
								: CANVAS_HOVER_SOURCE_CANVAS;
				const id = typeof args.id === "string" ? args.id : null;
				this.hoveredId = id;
				this.hoveredKind = (args.kind as DrawKindHover | null) ?? null;
				if (id) this.pointerFocus.setHoverFromSource(sourceId, id);
				else this.pointerFocus.clearHoverFromSource(sourceId);
				this.bump();
				return;
			}
			case "setActiveTool": {
				this.commitDocument(applyDrawEditOp(this.projection(), { op: "setActiveTool", tool: String(args.tool) as DrawToolId }));
				this.bump();
				return;
			}
			case "addLayer": {
				const kind = String(args.kind ?? "path") as DrawCatalogueLayerKind;
				const layer = drawPlayCreateLayerByKind(kind);
				this.patchDocument((doc) => applyDrawEditOp(doc, drawPlayAddLayerOp(kind, layer)), layer.id);
				return;
			}
			case "dropLayerKind": {
				const kind = String(args.kind ?? "") as DrawCatalogueLayerKind;
				const targetRowId = String(args.targetRowId ?? "draw-play-layers");
				const dropPosition = (args.dropPosition ?? "inside") as TreeDropPosition;
				const layer = drawPlayCreateLayerByKind(kind);
				const target = resolveDrawPlayReorderTarget(this.projection(), targetRowId, dropPosition === "before" || dropPosition === "after" ? dropPosition : "inside");
				const parentId = target?.parentId;
				const index = target?.index ?? this.projection().layers.length;
				this.patchDocument((doc) => applyDrawEditOp(doc, drawPlayAddLayerOp(kind, layer, parentId, index)), layer.id);
				return;
			}
			case "commitDocument": {
				const document = args.document as DrawDocument;
				if (!document || document.schema !== "draw.document/v1") return;
				this.commitDocument(document, selectLayerId);
				if (typeof args.selectLayerId === "string") this.selectedIds = [args.selectLayerId];
				this.bump();
				return;
			}
			case "moveLayer": {
				const layerId = String(args.layerId ?? "");
				const targetRowId = String(args.targetRowId ?? "");
				const dropPosition = (args.dropPosition ?? "after") as TreeDropPosition;
				const target = resolveDrawPlayReorderTarget(this.projection(), targetRowId, dropPosition);
				if (!target || !layerId) return;
				this.patchDocument((doc) => applyDrawEditOp(doc, { op: "reorderLayer", layerId, parentId: target.parentId, index: target.index }));
				return;
			}
			case "deleteLayer": {
				const layerId = String(args.layerId ?? "");
				this.commitDocument(applyDrawEditOp(this.projection(), { op: "removeLayer", layerId }));
				this.selectedIds = this.selectedIds.filter((id) => id !== layerId);
				this.bump();
				return;
			}
			case "duplicateLayer": {
				this.commitDocument(applyDrawEditOp(this.projection(), { op: "duplicateLayer", layerId: String(args.layerId) }));
				this.bump();
				return;
			}
			case "toggleLayerVisible": {
				const layerId = String(args.layerId ?? "");
				const layer = findDrawLayer(this.projection(), layerId);
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
				this.patchDocument((doc) => drawPlayPatchLayerField(doc, layerId, field, value));
				return;
			}
			case "patchLayers": {
				const layerIds = (Array.isArray(args.layerIds) ? args.layerIds : []).map(String).filter(Boolean);
				const field = String(args.field ?? "");
				const value = args.value ?? args.pressed;
				if (!layerIds.length || !field) return;
				this.patchDocument((doc) => {
					let next = doc;
					for (const layerId of layerIds) {
						next = drawPlayPatchLayerField(next, layerId, field, value);
					}
					return next;
				});
				return;
			}
			case "setCamera": {
				const camera = args.camera as DrawDocument["camera"];
				if (camera) this.patchDocument((doc) => applyDrawEditOp(doc, { op: "setCamera", camera }));
				return;
			}
			case "selectAll": {
				this.selectedIds = flattenDrawLayers(this.projection().layers).map((layer) => layer.id);
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

		it("renders the semio emblem from only its three authored paths", () => {
			const doc = drawDocumentFromJson(DRAW_PLAY_FILE_FIXTURE_JSON_BY_ID.semio!);
			expect(doc.layers.map((layer) => layer.id)).toEqual(["emblem-group"]);
			expect(flattenDrawLayers(doc.layers).filter((layer) => layer.kind === "path").map((layer) => layer.id)).toEqual([
				"emblem-orange",
				"emblem-red",
				"emblem-teal",
			]);
		});
	});

	describe("buildDrawPlayLayersTree", () => {
		it("builds hierarchy for default document", () => {
			const doc = defaultDrawDocument("test");
			const tree = buildDrawPlayLayersTree(doc, [], null, null);
			expect(tree.sections[0]?.items.length).toBeGreaterThan(0);
		});
	});

	describe("buildDrawPlayInspectorTree", () => {
		it("orders inspector sections specific to general for ellipse layers", () => {
			const layer = createDrawShapeLayer("E", { shapeKind: "ellipse", ellipse: { cx: 0, cy: 0, rx: 10, ry: 5 } });
			const doc: DrawDocument = { ...defaultDrawDocument("ellipse"), layers: [layer] };
			const tree = buildDrawPlayInspectorTree(doc, [layer.id]);
			const labels = (tree.type === "tree" ? tree.sections : []).map((section) => section.label);
			expect(labels.indexOf("Ellipse")).toBeLessThan(labels.indexOf("Position"));
			expect(labels.indexOf("Position")).toBeLessThan(labels.indexOf("Orientation"));
			expect(labels.indexOf("Orientation")).toBeLessThan(labels.indexOf("Layer"));
		});

		it("exposes locked and matrix fields through patchLayer", () => {
			const layer = createDrawPathLayer("P");
			const doc: DrawDocument = { ...defaultDrawDocument("patch"), layers: [layer] };
			const bus = new CommandBus();
			const ctrl = new DrawPlayController(bus, () => {});
			ctrl.run("setFixtureJson", { json: drawDocumentToJson(doc), resetInteraction: false });
			ctrl.run("patchLayer", { layerId: layer.id, field: "locked", pressed: true });
			expect(findDrawLayer(ctrl.getDocument(), layer.id)?.locked).toBe(true);
			ctrl.run("patchLayer", { layerId: layer.id, field: "transformMatrixE", value: 42 });
			expect(ctrl.getDocument().layers[0]?.transform.x).toBeCloseTo(42);
		});

		it("batch-patches shared fields across multiple layers", () => {
			const first = createDrawPathLayer("A");
			const second = createDrawPathLayer("B");
			const doc: DrawDocument = { ...defaultDrawDocument("batch"), layers: [first, second] };
			const bus = new CommandBus();
			const ctrl = new DrawPlayController(bus, () => {});
			ctrl.run("setFixtureJson", { json: drawDocumentToJson(doc), resetInteraction: false });
			ctrl.run("patchLayers", { layerIds: [first.id, second.id], field: "opacity", value: 0.5 });
			expect(findDrawLayer(ctrl.getDocument(), first.id)?.opacity).toBe(0.5);
			expect(findDrawLayer(ctrl.getDocument(), second.id)?.opacity).toBe(0.5);
		});
	});
}
// #endregion 🧪Tests
