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
	uiInspectorGroupsToTree,
	uiInspectorReadonlyField,
	type UiInspectorFieldGroup,
	type UiNode,
	type UiTreeItemNode,
	type UiTreeNode,
} from "@semio-tech/framework-playground-core";
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

function drawPlayInspectorNumberField(layerId: string, fieldId: string, label: string, value: number, field: string): UiNode {
	return {
		type: "field",
		id: fieldId,
		label,
		child: {
			type: "input",
			id: `${fieldId}.input`,
			inputKind: "number",
			value: String(value),
			onChange: drawPlayCmd("patchLayer", { layerId, field }),
		},
	};
}

function drawPlayInspectorTextField(layerId: string, fieldId: string, label: string, value: string, field: string): UiNode {
	return {
		type: "field",
		id: fieldId,
		label,
		child: {
			type: "input",
			id: `${fieldId}.input`,
			inputKind: "text",
			value,
			onChange: drawPlayCmd("patchLayer", { layerId, field }),
		},
	};
}

function drawPlayLayerKindLabel(layer: DrawLayerNode): string {
	if (layer.kind === "shape") return `shape:${layer.shapeKind}`;
	return layer.kind;
}

function drawPlayInspectorKindSpecificGroup(doc: DrawDocument, layer: DrawLayerNode): UiInspectorFieldGroup | null {
	const layerId = layer.id;
	const fields: UiNode[] = [];
	if (layer.kind === "boolean") {
		fields.push({
			type: "field",
			id: "draw-play-inspector.boolean-op",
			label: "Boolean Op",
			child: {
				type: "select",
				id: "draw-play-inspector.boolean-op.select",
				value: layer.op,
				items: DRAW_BOOLEAN_OPS.map((op) => ({ value: op, label: op })),
				onChange: drawPlayCmd("patchLayer", { layerId, field: "booleanOp" }),
			},
		});
		const childLabels = layer.children
			.map((childId) => findDrawLayer(doc, childId))
			.filter((child): child is DrawLayerNode => Boolean(child))
			.map((child) => child.name || child.id)
			.join(", ");
		fields.push(uiInspectorReadonlyField("draw-play-inspector.boolean-children", "Children", childLabels || "—"));
		return { id: "draw-play-inspector.kind.boolean", label: "Boolean", fields };
	}
	if (layer.kind === "trace") {
		fields.push(
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
					onChange: drawPlayCmd("patchLayer", { layerId, field: "traceThreshold" }),
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
					onChange: drawPlayCmd("patchLayer", { layerId, field: "traceSimplify" }),
				},
			},
			uiInspectorReadonlyField("draw-play-inspector.trace-source", "Source Key", layer.sourceKey),
		);
		return { id: "draw-play-inspector.kind.trace", label: "Trace", fields };
	}
	if (layer.kind === "shape" && layer.shapeKind === "rect" && layer.rect) {
		fields.push(
			drawPlayInspectorNumberField(layerId, "draw-play-inspector.rect-width", "Width", layer.rect.width, "rectWidth"),
			drawPlayInspectorNumberField(layerId, "draw-play-inspector.rect-height", "Height", layer.rect.height, "rectHeight"),
		);
		return { id: "draw-play-inspector.kind.rect", label: "Rectangle", fields };
	}
	if (layer.kind === "shape" && layer.shapeKind === "ellipse" && layer.ellipse) {
		fields.push(
			drawPlayInspectorNumberField(layerId, "draw-play-inspector.ellipse-rx", "RX", layer.ellipse.rx, "ellipseRx"),
			drawPlayInspectorNumberField(layerId, "draw-play-inspector.ellipse-ry", "RY", layer.ellipse.ry, "ellipseRy"),
		);
		return { id: "draw-play-inspector.kind.ellipse", label: "Ellipse", fields };
	}
	if (layer.kind === "shape" && layer.shapeKind === "circle" && layer.circle) {
		fields.push(drawPlayInspectorNumberField(layerId, "draw-play-inspector.circle-r", "R", layer.circle.r, "circleR"));
		return { id: "draw-play-inspector.kind.circle", label: "Circle", fields };
	}
	if (layer.kind === "shape" && layer.shapeKind === "line" && layer.line) {
		fields.push(
			drawPlayInspectorNumberField(layerId, "draw-play-inspector.line-x1", "X1", layer.line.x1, "lineX1"),
			drawPlayInspectorNumberField(layerId, "draw-play-inspector.line-y1", "Y1", layer.line.y1, "lineY1"),
			drawPlayInspectorNumberField(layerId, "draw-play-inspector.line-x2", "X2", layer.line.x2, "lineX2"),
			drawPlayInspectorNumberField(layerId, "draw-play-inspector.line-y2", "Y2", layer.line.y2, "lineY2"),
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
			drawPlayInspectorTextField(layerId, "draw-play-inspector.text-content", "Content", layer.content, "textContent"),
			drawPlayInspectorNumberField(layerId, "draw-play-inspector.text-size", "Size", layer.size, "textSize"),
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

function drawPlayInspectorPositionGroup(layer: DrawLayerNode): UiInspectorFieldGroup | null {
	const layerId = layer.id;
	const fields: UiNode[] = [];
	if (layer.kind === "shape" && layer.shapeKind === "rect" && layer.rect) {
		fields.push(
			drawPlayInspectorNumberField(layerId, "draw-play-inspector.rect-x", "X", layer.rect.x, "rectX"),
			drawPlayInspectorNumberField(layerId, "draw-play-inspector.rect-y", "Y", layer.rect.y, "rectY"),
		);
	}
	if (layer.kind === "shape" && layer.shapeKind === "ellipse" && layer.ellipse) {
		fields.push(
			drawPlayInspectorNumberField(layerId, "draw-play-inspector.ellipse-cx", "CX", layer.ellipse.cx, "ellipseCx"),
			drawPlayInspectorNumberField(layerId, "draw-play-inspector.ellipse-cy", "CY", layer.ellipse.cy, "ellipseCy"),
		);
	}
	if (layer.kind === "shape" && layer.shapeKind === "circle" && layer.circle) {
		fields.push(
			drawPlayInspectorNumberField(layerId, "draw-play-inspector.circle-cx", "CX", layer.circle.cx, "circleCx"),
			drawPlayInspectorNumberField(layerId, "draw-play-inspector.circle-cy", "CY", layer.circle.cy, "circleCy"),
		);
	}
	if (layer.kind === "text") {
		fields.push(
			drawPlayInspectorNumberField(layerId, "draw-play-inspector.text-x", "X", layer.x, "textX"),
			drawPlayInspectorNumberField(layerId, "draw-play-inspector.text-y", "Y", layer.y, "textY"),
		);
	}
	if (fields.length === 0) return null;
	return { id: "draw-play-inspector.position", label: "Position", fields };
}

function drawPlayInspectorOrientationGroup(layer: DrawLayerNode): UiInspectorFieldGroup {
	const layerId = layer.id;
	const matrix = drawTransformToMatrix(layer.transform);
	const fields: UiNode[] = [
		drawPlayInspectorNumberField(layerId, "draw-play-inspector.transform-x", "Position X", layer.transform.x, "transformX"),
		drawPlayInspectorNumberField(layerId, "draw-play-inspector.transform-y", "Position Y", layer.transform.y, "transformY"),
		drawPlayInspectorNumberField(layerId, "draw-play-inspector.transform-scale-x", "Scale X", layer.transform.scaleX, "transformScaleX"),
		drawPlayInspectorNumberField(layerId, "draw-play-inspector.transform-scale-y", "Scale Y", layer.transform.scaleY, "transformScaleY"),
		drawPlayInspectorNumberField(layerId, "draw-play-inspector.transform-rotation", "Rotation", layer.transform.rotation, "transformRotation"),
		drawPlayInspectorNumberField(layerId, "draw-play-inspector.matrix-a", "Matrix A", matrix[0], "transformMatrixA"),
		drawPlayInspectorNumberField(layerId, "draw-play-inspector.matrix-b", "Matrix B", matrix[1], "transformMatrixB"),
		drawPlayInspectorNumberField(layerId, "draw-play-inspector.matrix-c", "Matrix C", matrix[2], "transformMatrixC"),
		drawPlayInspectorNumberField(layerId, "draw-play-inspector.matrix-d", "Matrix D", matrix[3], "transformMatrixD"),
		drawPlayInspectorNumberField(layerId, "draw-play-inspector.matrix-e", "Matrix E", matrix[4], "transformMatrixE"),
		drawPlayInspectorNumberField(layerId, "draw-play-inspector.matrix-f", "Matrix F", matrix[5], "transformMatrixF"),
	];
	return { id: "draw-play-inspector.orientation", label: "Orientation", fields };
}

function drawPlayInspectorAppearanceGroup(layer: DrawLayerNode): UiInspectorFieldGroup {
	const layerId = layer.id;
	const fillColor = layer.attributes.fill?.kind === "solid" ? rgbaToHex(layer.attributes.fill.color) : "#000000";
	const fillAlpha = layer.attributes.fill?.kind === "solid" ? layer.attributes.fill.color[3] : 1;
	const strokeColor = layer.attributes.stroke ? rgbaToHex(layer.attributes.stroke.color) : "#000000";
	const strokeWidth = layer.attributes.stroke?.width ?? 1;
	return {
		id: "draw-play-inspector.appearance",
		label: "Appearance",
		fields: [
			drawPlayInspectorTextField(layerId, "draw-play-inspector.fill", "Fill", fillColor, "fillColor"),
			{
				type: "field",
				id: "draw-play-inspector.fill-alpha",
				label: "Fill Alpha",
				child: {
					type: "slider",
					id: "draw-play-inspector.fill-alpha.slider",
					value: fillAlpha,
					min: 0,
					max: 1,
					step: 0.01,
					onChange: drawPlayCmd("patchLayer", { layerId, field: "fillAlpha" }),
				},
			},
			drawPlayInspectorTextField(layerId, "draw-play-inspector.stroke", "Stroke", strokeColor, "strokeColor"),
			drawPlayInspectorNumberField(layerId, "draw-play-inspector.stroke-width", "Stroke Width", strokeWidth, "strokeWidth"),
		],
	};
}

function drawPlayInspectorLayerGroup(layer: DrawLayerNode): UiInspectorFieldGroup {
	const layerId = layer.id;
	return {
		id: "draw-play-inspector.layer",
		label: "Layer",
		fields: [
			drawPlayInspectorTextField(layerId, "draw-play-inspector.name", "Name", layer.name, "name"),
			uiInspectorReadonlyField("draw-play-inspector.id", "Id", layer.id),
			uiInspectorReadonlyField("draw-play-inspector.kind", "Kind", drawPlayLayerKindLabel(layer)),
			{
				type: "field",
				id: "draw-play-inspector.visible",
				label: "Visible",
				child: {
					type: "toggle",
					id: "draw-play-inspector.visible.toggle",
					pressed: layer.visible,
					onChange: drawPlayCmd("patchLayer", { layerId, field: "visible" }),
				},
			},
			{
				type: "field",
				id: "draw-play-inspector.locked",
				label: "Locked",
				child: {
					type: "toggle",
					id: "draw-play-inspector.locked.toggle",
					pressed: layer.locked,
					onChange: drawPlayCmd("patchLayer", { layerId, field: "locked" }),
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
					onChange: drawPlayCmd("patchLayer", { layerId, field: "opacity" }),
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
					onChange: drawPlayCmd("patchLayer", { layerId, field: "blendMode" }),
				},
			},
		],
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
	const groups: UiInspectorFieldGroup[] = [];
	const kindSpecific = drawPlayInspectorKindSpecificGroup(doc, layer);
	if (kindSpecific) groups.push(kindSpecific);
	const position = drawPlayInspectorPositionGroup(layer);
	if (position) groups.push(position);
	groups.push(drawPlayInspectorOrientationGroup(layer));
	groups.push(drawPlayInspectorAppearanceGroup(layer));
	groups.push(drawPlayInspectorLayerGroup(layer));
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

export class DrawPlayController extends Controller implements PlaygroundFixtureHost {
	readonly mainMode = new ModeRuntime("main", "Draw", undefined);
	private document: DrawDocument = DRAW_PLAY_EMPTY_DOCUMENT;
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

	private buildTools(): AppTools {
		const activeTool = this.document.activeTool ?? "selectDirect";
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
		return this.pointerFocus.getSnapshot().hover ?? this.hoveredId;
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
				this.document = applyDrawEditOp(this.document, { op: "setActiveTool", tool: String(args.tool) as DrawToolId });
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
				const target = resolveDrawPlayReorderTarget(this.document, targetRowId, dropPosition === "before" || dropPosition === "after" ? dropPosition : "inside");
				const parentId = target?.parentId;
				const index = target?.index ?? this.document.layers.length;
				this.patchDocument((doc) => applyDrawEditOp(doc, drawPlayAddLayerOp(kind, layer, parentId, index)), layer.id);
				return;
			}
			case "commitDocument": {
				const document = args.document as DrawDocument;
				if (!document || document.schema !== "draw.document/v1") return;
				this.document = document;
				if (typeof args.selectLayerId === "string") this.selectedIds = [args.selectLayerId];
				this.bump();
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
	});
}
// #endregion 🧪Tests
