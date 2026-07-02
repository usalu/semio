// #region 🧲Header
/** @emoji 🔷 Lowpoly play — low-poly mesh editing playground. */
// #endregion 🧲Header

import {
	AppRuntime,
	CommandBus,
	Controller,
	ModeRuntime,
	Platform,
	Playground,
	WindowKindRuntime,
	buildPuzzle3dWindowBody,
	createPlayAppRuntime,
	createDefaultLayout,
	createProductPlaygroundPlatform,
	enforcePlaygroundWindowEngagementInput,
	registerWindowBody,
	FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
	FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
	FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
	uiDeclarativeSectionsToTree,
	uiInspectorGroupsToTree,
	uiInspectorReadonlyField,
	type AppTools,
	type CommandDescriptor,
	type ToolLeaf,
	type UiInspectorFieldGroup,
	type UiNode,
	type UiTreeItemNode,
	type WindowBodyViewContext,
	type WindowEngagement,
	type WindowMeasure,
	toolCollection,
} from "@semio-tech/framework-playground-core";
import { bootstrapElementsSurfaceChromeDocument, selectionMergeIds } from "@semio-tech/ui-react";
import {
	DEFAULT_LOWPOLY_SELECTION,
	LOWPOLY_FIXTURE_SCHEMA,
	type LowpolyFixture,
	type LowpolySelectionMode,
	type LowpolyTarget,
	lowpolyTopologyFromMeshJson,
	lowpolyFixtureToJson,
	parseLowpolyFixtureJson,
	encodeLowpolyPointerFocusKey,
	decodeLowpolyPointerFocusKey,
} from "./internal.ts";
import type { LowpolyPaintTool, LowpolyTransformTool } from "@semio-tech/lowpoly-react";
import {
	DocumentVcsStore,
	createDocumentVcsId,
	type DocumentVcsCommand,
} from "@semio-tech/vcs-core/internal";
import {
	applyLowpolyPaintEditOp,
	backwardsLowpolyPaintEditOp,
	createLowpolyPaintVcsEnvelope,
	type LowpolyPaintDocument,
  type LowpolyPaintEditOp,
} from "./internal.ts";

export const LOWPOLY_PLAY_APP_ID = "lowpoly-play";
export const LOWPOLY_PLAY_CONTROLLER_ID = "lowpoly-play";
export const LOWPOLY_PLAY_BODY_KEY_MAIN = "lowpoly.play.main";
export const LOWPOLY_PLAY_SURFACE_ID = "lowpoly.play/v1";
export const LOWPOLY_PLAY_WINDOW_KIND_ID = "lowpoly-main";
export const LOWPOLY_PLAY_HIERARCHY_TAB_ID = "framework.panel.hierarchy";
export const LOWPOLY_PLAY_CATALOGUE_TAB_ID = "framework.panel.catalogue";
export const LOWPOLY_PLAY_INSPECTION_TAB_ID = "framework.panel.inspection";
export const LOWPOLY_PLAY_BODY_KEY_UV = "lowpoly.play.uv";
export const LOWPOLY_PLAY_UV_SURFACE_ID = "lowpoly.play/uv";
export const LOWPOLY_PLAY_UV_WINDOW_KIND_ID = "lowpoly-uv";
export const LOWPOLY_PLAY_LAYERS_TAB_ID = "framework.panel.layers";

const EMPTY_FIXTURE: LowpolyFixture = {
	schema: LOWPOLY_FIXTURE_SCHEMA,
	objects: [],
	activeObjectId: "",
	selection: DEFAULT_LOWPOLY_SELECTION,
};

export const LOWPOLY_PLAY_DEFAULT_FIXTURE_JSON = lowpolyFixtureToJson(EMPTY_FIXTURE);
export const LOWPOLY_PLAY_LAYOUT = createDefaultLayout([LOWPOLY_PLAY_WINDOW_KIND_ID], "row", [100], ["Model"]);
export const LOWPOLY_PLAY_PAINT_LAYOUT = createDefaultLayout(
	[LOWPOLY_PLAY_WINDOW_KIND_ID, LOWPOLY_PLAY_UV_WINDOW_KIND_ID],
	"row",
	[60, 40],
	["Paint", "UV"],
);

export type LowpolyEditTool =
	| "extrude"
	| "inset"
	| "bevel"
	| "loop_cut"
	| "merge"
	| "dissolve"
	| "subdivide"
	| "triangulate"
	| "mirror"
	| "decimate";

function lowpolyPlayCmd(command: string, args?: Record<string, unknown>): CommandDescriptor {
	return { controllerId: LOWPOLY_PLAY_CONTROLLER_ID, command, args };
}

/** @emoji 🧰 Lowpoly play footer toolbar. */
export function buildLowpolyPlayToolbarTools(
	controllerId: string,
	selectionMode: LowpolySelectionMode,
	transformTool: LowpolyTransformTool,
): AppTools {
	const modeToggle = (id: string, label: string, mode: LowpolySelectionMode): ToolLeaf => ({
		id,
		kind: "toggle",
		label,
		iconId: mode === "vertex" ? "circle" : mode === "edge" ? "minus" : mode === "face" ? "square" : "box",
		pressed: selectionMode === mode,
		controllerId,
		command: "setSelectionMode",
		args: { mode },
	});
	const transformToggle = (id: string, label: string, tool: LowpolyTransformTool): ToolLeaf => ({
		id,
		kind: "toggle",
		label,
		iconId: tool === "move" ? "move" : tool === "rotate" ? "rotate-cw" : "maximize-2",
		pressed: transformTool === tool,
		controllerId,
		command: "setTransformTool",
		args: { tool },
	});
	return [
		toolCollection("selection", "mouse-pointer", [
			modeToggle("lowpoly.mode.object", "Object", "object"),
			modeToggle("lowpoly.mode.vertex", "Vertex", "vertex"),
			modeToggle("lowpoly.mode.edge", "Edge", "edge"),
			modeToggle("lowpoly.mode.face", "Face", "face"),
		]),
		toolCollection("transform", "move", [
			transformToggle("lowpoly.transform.move", "Move", "move"),
			transformToggle("lowpoly.transform.rotate", "Rotate", "rotate"),
			transformToggle("lowpoly.transform.scale", "Scale", "scale"),
		]),
		toolCollection("edit", "pen-tool", [
			{ kind: "button", id: "lowpoly.extrude", label: "Extrude", iconId: "box", controllerId, command: "extrude" },
			{ kind: "button", id: "lowpoly.inset", label: "Inset", iconId: "square", controllerId, command: "inset" },
			{ kind: "button", id: "lowpoly.flip-faces", label: "Flip Normals", iconId: "flip-vertical", controllerId, command: "flipFaces" },
			{ kind: "button", id: "lowpoly.bevel", label: "Bevel", iconId: "git-branch", controllerId, command: "bevel" },
			{ kind: "button", id: "lowpoly.loop_cut", label: "Loop Cut", iconId: "git-commit", controllerId, command: "loopCut" },
			{ kind: "button", id: "lowpoly.merge", label: "Merge", iconId: "git-merge", controllerId, command: "merge" },
			{ kind: "button", id: "lowpoly.dissolve", label: "Dissolve", iconId: "eraser", controllerId, command: "dissolve" },
			{ kind: "button", id: "lowpoly.subdivide", label: "Subdivide", iconId: "grid-3x3", controllerId, command: "subdivide" },
			{ kind: "button", id: "lowpoly.triangulate", label: "Triangulate", iconId: "triangle", controllerId, command: "triangulate" },
			{ kind: "button", id: "lowpoly.mirror", label: "Mirror", iconId: "flip-horizontal", controllerId, command: "mirror" },
			{ kind: "button", id: "lowpoly.decimate", label: "Decimate", iconId: "minimize-2", controllerId, command: "decimate" },
		]),
		toolCollection("options", "settings", [
			{ kind: "button", id: "lowpoly.snap", label: "Snap", iconId: "magnet", controllerId, command: "snap" },
			{ kind: "button", id: "lowpoly.smooth", label: "Smooth", iconId: "sun", controllerId, command: "toggleSmooth" },
		]),
	];
}

/** @emoji 🎨 Lowpoly paint toolbar. */
export function buildLowpolyPlayPaintToolbarTools(controllerId: string, paintTool: LowpolyPaintTool): AppTools {
	const paintToggle = (id: string, label: string, tool: LowpolyPaintTool): ToolLeaf => ({
		id,
		kind: "toggle",
		label,
		iconId: tool === "brush" ? "paintbrush" : tool === "eraser" ? "eraser" : tool === "fill" ? "paint-bucket" : "pipette",
		pressed: paintTool === tool,
		controllerId,
		command: "setPaintTool",
		args: { tool },
	});
	return [
		toolCollection("paint", "paintbrush", [
			paintToggle("lowpoly.paint.brush", "Brush", "brush"),
			paintToggle("lowpoly.paint.eraser", "Eraser", "eraser"),
			paintToggle("lowpoly.paint.fill", "Fill", "fill"),
			paintToggle("lowpoly.paint.eyedropper", "Eyedropper", "eyedropper"),
		]),
		toolCollection("uv", "grid-3x3", [
			{ kind: "button", id: "lowpoly.unwrap", label: "Unwrap", iconId: "grid-3x3", controllerId, command: "unwrapActive" },
			{ kind: "button", id: "lowpoly.seam.mark", label: "Mark Seam", iconId: "scissors", controllerId, command: "markUvSeam", args: { seam: true } },
			{ kind: "button", id: "lowpoly.seam.clear", label: "Clear Seam", iconId: "unlink", controllerId, command: "markUvSeam", args: { seam: false } },
		]),
		toolCollection("history", "undo", [
			{ kind: "button", id: "lowpoly.undo", label: "Undo", iconId: "undo", controllerId, command: "paintUndo" },
			{ kind: "button", id: "lowpoly.redo", label: "Redo", iconId: "redo", controllerId, command: "paintRedo" },
		]),
	];
}

//#region Panels

export function lowpolyHierarchyTargetRowId(target: LowpolyTarget): string {
	return `lowpoly-hierarchy.${target.objectId}.${target.mode}.${target.id}`;
}

export type LowpolyHierarchyTreeOptions = {
	readonly hoveredTarget?: LowpolyTarget | null;
	readonly onHover?: (target: LowpolyTarget | null) => void;
	readonly onFlipFace?: (objectId: string, faceId: number) => void;
};

export function buildLowpolyPlayHierarchyTree(
	fixtureJson: string,
	selectionMode: LowpolySelectionMode,
	selectedIds: readonly number[],
	options?: LowpolyHierarchyTreeOptions,
): UiNode {
	const fixture = parseLowpolyFixtureJson(fixtureJson);
	if (!fixture) {
		return {
			type: "tree",
			sections: [{ id: "lowpoly-hierarchy.invalid", label: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, defaultOpen: true, items: [{ id: "lowpoly-hierarchy.invalid.msg", label: "Invalid fixture" }] }],
		};
	}
	const targetItem = (target: LowpolyTarget, label: string, icon: string, actions?: UiTreeItemNode["actions"]): UiTreeItemNode => ({
		id: lowpolyHierarchyTargetRowId(target),
		label,
		icon,
		command: lowpolyPlayCmd("toggleSelectionTarget", { target }),
		onPointerEnter: options?.onHover ? () => options.onHover?.(target) : undefined,
		onPointerLeave: options?.onHover ? () => options.onHover?.(null) : undefined,
		actions,
	});
	const items: UiTreeItemNode[] = fixture.objects.map((object, objectIndex) => {
		const objectTarget: LowpolyTarget = { objectId: object.id, objectIndex, mode: "object", id: objectIndex };
		const topology = lowpolyTopologyFromMeshJson(object.meshJson);
		const componentGroup = (mode: Exclude<LowpolySelectionMode, "object">, ids: readonly number[], icon: string): UiTreeItemNode => ({
			id: `lowpoly-hierarchy.${object.id}.${mode}`,
			label: mode === "vertex" ? "Vertices" : mode === "edge" ? "Edges" : "Faces",
			description: String(ids.length),
			icon,
			items: ids.map((id) => {
				const target: LowpolyTarget = { objectId: object.id, objectIndex, mode, id };
				return targetItem(
					target,
					`${mode[0]!.toUpperCase()}${mode.slice(1)} ${id}`,
					icon,
					mode === "face"
						? [{
								id: `lowpoly-hierarchy.${object.id}.face.${id}.flip`,
								icon: "flip-vertical",
								title: "Flip normal",
								revealOnHover: true,
								onClick: () => options?.onFlipFace?.(object.id, id),
							}]
						: undefined,
				);
			}),
		});
		return {
			...targetItem(objectTarget, object.name, "box"),
			description: object.id,
			defaultOpen: object.id === fixture.activeObjectId,
			items: [
				componentGroup("vertex", topology.vertexIds, "circle"),
				componentGroup("edge", topology.edgeIds, "minus"),
				componentGroup("face", topology.faceIds, "square"),
			],
		};
	});
	const selectedRowIds =
		selectionMode === "object"
			? selectedIds.flatMap((id) => {
					const object = fixture.objects[id];
					return object ? [lowpolyHierarchyTargetRowId({ objectId: object.id, objectIndex: id, mode: "object", id })] : [];
				})
			: selectedIds.map((id) =>
					lowpolyHierarchyTargetRowId({
						objectId: fixture.activeObjectId,
						objectIndex: Math.max(0, fixture.objects.findIndex((object) => object.id === fixture.activeObjectId)),
						mode: selectionMode,
						id,
					}),
				);
	return {
		type: "tree",
		sections: [
			{
				id: "lowpoly-hierarchy.objects",
				label: "Objects",
				defaultOpen: true,
				items: items.length ? items : [{ id: "lowpoly-hierarchy.empty", label: "(none)" }],
			},
		],
		selectedIds: selectedRowIds,
		highlightedIds: options?.hoveredTarget ? [lowpolyHierarchyTargetRowId(options.hoveredTarget)] : [],
	};
}

export function buildLowpolyPlayCatalogueTree(): UiNode {
	const primitives = [
		{ kind: "box", label: "Cube" },
		{ kind: "plane", label: "Plane" },
		{ kind: "cylinder", label: "Cylinder" },
		{ kind: "cone", label: "Cone" },
		{ kind: "ico_sphere", label: "Ico Sphere" },
	];
	return {
		type: "tree",
		sections: [
			{
				id: "lowpoly-catalogue.primitives",
				label: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
				defaultOpen: true,
				items: primitives.map((p) => ({
					id: `lowpoly-catalogue.${p.kind}`,
					label: p.label,
					description: p.kind,
					command: lowpolyPlayCmd("addPrimitive", { kind: p.kind }),
				})),
			},
		],
	};
}

export function buildLowpolyPlayInspectorTree(fixtureJson: string, toolParams: Record<string, number>): UiNode {
	const fixture = parseLowpolyFixtureJson(fixtureJson);
	if (!fixture?.objects.length) {
		return uiDeclarativeSectionsToTree([
			{ type: "section", id: "lowpoly-inspector.empty", label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, children: [{ type: "text", value: "Add or select an object." }] },
		]);
	}
	const active = fixture.objects.find((o) => o.id === fixture.activeObjectId) ?? fixture.objects[0];
	const groups: UiInspectorFieldGroup[] = [
		{
			id: "lowpoly-inspector.object",
			label: "Object",
			fields: [
				uiInspectorReadonlyField("lowpoly-inspector.name", "Name", active?.name ?? ""),
				uiInspectorReadonlyField("lowpoly-inspector.mode", "Selection", fixture.selection.mode),
			],
		},
		{
			id: "lowpoly-inspector.tool",
			label: "Tool Params",
			fields: [
				{
					type: "field",
					id: "lowpoly-inspector.extrude",
					label: "Extrude distance",
					child: {
						type: "input",
						id: "lowpoly-inspector.extrude.input",
						inputKind: "number",
						value: String(toolParams.extrudeDistance ?? 0.25),
						onChange: lowpolyPlayCmd("setToolParam", { field: "extrudeDistance" }),
					},
				},
				{
					type: "field",
					id: "lowpoly-inspector.inset",
					label: "Inset amount",
					child: {
						type: "input",
						id: "lowpoly-inspector.inset.input",
						inputKind: "number",
						value: String(toolParams.insetAmount ?? 0.1),
						onChange: lowpolyPlayCmd("setToolParam", { field: "insetAmount" }),
					},
				},
				{
					type: "field",
					id: "lowpoly-inspector.bevel",
					label: "Bevel amount",
					child: {
						type: "input",
						id: "lowpoly-inspector.bevel.input",
						inputKind: "number",
						value: String(toolParams.bevelAmount ?? 0.05),
						onChange: lowpolyPlayCmd("setToolParam", { field: "bevelAmount" }),
					},
				},
				{
					type: "field",
					id: "lowpoly-inspector.loop",
					label: "Loop cuts",
					child: {
						type: "input",
						id: "lowpoly-inspector.loop.input",
						inputKind: "number",
						value: String(toolParams.loopCuts ?? 1),
						onChange: lowpolyPlayCmd("setToolParam", { field: "loopCuts" }),
					},
				},
				{
					type: "field",
					id: "lowpoly-inspector.snap",
					label: "Snap grid",
					child: {
						type: "input",
						id: "lowpoly-inspector.snap.input",
						inputKind: "number",
						value: String(toolParams.snapGrid ?? 0.25),
						onChange: lowpolyPlayCmd("setToolParam", { field: "snapGrid" }),
					},
				},
				{
					type: "field",
					id: "lowpoly-inspector.mirror",
					label: "Mirror axis (0=x,1=y,2=z)",
					child: {
						type: "input",
						id: "lowpoly-inspector.mirror.input",
						inputKind: "number",
						value: String(toolParams.mirrorAxis ?? 0),
						onChange: lowpolyPlayCmd("setToolParam", { field: "mirrorAxis" }),
					},
				},
				{
					type: "field",
					id: "lowpoly-inspector.decimate",
					label: "Decimate ratio",
					child: {
						type: "input",
						id: "lowpoly-inspector.decimate.input",
						inputKind: "number",
						value: String(toolParams.decimateRatio ?? 0.5),
						onChange: lowpolyPlayCmd("setToolParam", { field: "decimateRatio" }),
					},
				},
				{
					type: "field",
					id: "lowpoly-inspector.brushSize",
					label: "Brush size",
					child: {
						type: "input",
						id: "lowpoly-inspector.brushSize.input",
						inputKind: "number",
						value: String(toolParams.brushSize ?? 16),
						onChange: lowpolyPlayCmd("setToolParam", { field: "brushSize" }),
					},
				},
				{
					type: "field",
					id: "lowpoly-inspector.brushOpacity",
					label: "Brush opacity",
					child: {
						type: "input",
						id: "lowpoly-inspector.brushOpacity.input",
						inputKind: "number",
						value: String(toolParams.brushOpacity ?? 1),
						onChange: lowpolyPlayCmd("setToolParam", { field: "brushOpacity" }),
					},
				},
				{
					type: "field",
					id: "lowpoly-inspector.brushHardness",
					label: "Brush hardness",
					child: {
						type: "input",
						id: "lowpoly-inspector.brushHardness.input",
						inputKind: "number",
						value: String(toolParams.brushHardness ?? 0.5),
						onChange: lowpolyPlayCmd("setToolParam", { field: "brushHardness" }),
					},
				},
			],
		},
	];
	return uiInspectorGroupsToTree(groups);
}

export function buildLowpolyPlayLayersTree(fixtureJson: string, activeLayerIndex: number): UiNode {
	const fixture = parseLowpolyFixtureJson(fixtureJson);
	const active = fixture?.objects.find((object) => object.id === fixture.activeObjectId) ?? fixture?.objects[0];
	const layers = active?.paintLayers ?? [];
	return {
		type: "tree",
		sections: [
			{
				id: "lowpoly-layers.stack",
				label: "Layers",
				defaultOpen: true,
				items: layers.length
					? layers.map((layer, index) => ({
							id: `lowpoly-layer.${index}`,
							label: layer.name,
							description: `${Math.round(layer.opacity * 100)}% · ${layer.blendMode}`,
							command: lowpolyPlayCmd("setActivePaintLayer", { layerIndex: index }),
						}))
					: [{ id: "lowpoly-layer.empty", label: "(none)" }],
			},
		],
		selectedIds: [`lowpoly-layer.${activeLayerIndex}`],
	};
}

//#endregion Panels

/** @emoji 🎮 Lowpoly play controller. */
export class LowpolyPlayController extends Controller {
	readonly mainMode = new ModeRuntime("main", "Model", undefined);
	readonly paintMode = new ModeRuntime("paint", "Paint", undefined);
	private fixtureJson = LOWPOLY_PLAY_DEFAULT_FIXTURE_JSON;
	private selectionMode: LowpolySelectionMode = "object";
	private transformTool: LowpolyTransformTool = "move";
	private paintTool: LowpolyPaintTool = "brush";
	private activePaintLayerIndex = 0;
	private paintColor: [number, number, number, number] = [255, 64, 64, 255];
	private paintVcs: DocumentVcsStore<LowpolyPaintDocument, LowpolyPaintEditOp>;
	private toolParams: Record<string, number> = {
		extrudeDistance: 0.25,
		insetAmount: 0.1,
		bevelAmount: 0.05,
		bevelSegments: 1,
		loopCuts: 1,
		decimateRatio: 0.5,
		snapGrid: 0.25,
		mirrorAxis: 0,
		brushSize: 16,
		brushOpacity: 1,
		brushHardness: 0.5,
	};
	private smoothShading = false;
	private meshCommandEpoch = 0;
	private pendingMeshCommand: string | null = null;
	private pendingPaintCommand: { command: string; args?: Record<string, unknown> } | null = null;
	private interactionRevision = 0;
	private hoverRevision = 0;
	private readonly snapshotListeners = new Set<() => void>();

	private encodeSelectionKeys(ids: readonly number[]): string[] {
		const fixture = parseLowpolyFixtureJson(this.fixtureJson);
		if (!fixture) return [];
		if (this.selectionMode === "object") {
			return ids.flatMap((id) => {
				const object = fixture.objects[id];
				return object ? [encodeLowpolyPointerFocusKey({ objectId: object.id, objectIndex: id, mode: "object", id })] : [];
			});
		}
		const objectIndex = Math.max(0, fixture.objects.findIndex((object) => object.id === fixture.activeObjectId));
		const objectId = fixture.activeObjectId;
		return ids.map((id) => encodeLowpolyPointerFocusKey({ objectId, objectIndex, mode: this.selectionMode, id }));
	}

	private decodeSelectionIds(): number[] {
		return this.pointerFocus
			.getSnapshot()
			.selection.map((key) => decodeLowpolyPointerFocusKey(key)?.id)
			.filter((id): id is number => typeof id === "number");
	}

	constructor(commandBus: CommandBus, hostNotify: () => void) {
		super(LOWPOLY_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.paintVcs = new DocumentVcsStore({
			envelope: createLowpolyPaintVcsEnvelope(createDocumentVcsId("lowpoly-paint")),
			applyOp: applyLowpolyPaintEditOp,
			backwardsOp: backwardsLowpolyPaintEditOp,
			diffOp: () => null,
		});
		this.rebuildShellMode();
		this.rebuildPaintMode();
	}

	getFixtureJson(): string {
		return this.fixtureJson;
	}

	getSelectionMode(): LowpolySelectionMode {
		return this.selectionMode;
	}

	getSelectedIds(): readonly number[] {
		return this.decodeSelectionIds();
	}

	getHoveredTarget(): LowpolyTarget | null {
		const hover = this.pointerFocus.getSnapshot().hover;
		return hover ? decodeLowpolyPointerFocusKey(hover) : null;
	}

	getTransformTool(): LowpolyTransformTool {
		return this.transformTool;
	}

	getPaintTool(): LowpolyPaintTool {
		return this.paintTool;
	}

	getActivePaintLayerIndex(): number {
		return this.activePaintLayerIndex;
	}

	getPaintColor(): readonly [number, number, number, number] {
		return this.paintColor;
	}

	getPendingPaintCommand(): { command: string; args?: Record<string, unknown> } | null {
		return this.pendingPaintCommand;
	}

	clearPendingPaintCommand(): void {
		this.pendingPaintCommand = null;
	}

	dispatchPaintVcs(command: DocumentVcsCommand<LowpolyPaintEditOp>): void {
		this.paintVcs.dispatch(command);
		this.interactionRevision += 1;
		this.notifySnapshot();
		this.emit();
	}

	getPaintVcsGeneration(): number {
		return this.paintVcs.getGeneration();
	}

	subscribePaintVcs(listener: () => void): () => void {
		return this.paintVcs.subscribe(listener);
	}

	getPaintProjection(): LowpolyPaintDocument {
		return this.paintVcs.projection();
	}

	getToolParams(): Readonly<Record<string, number>> {
		return this.toolParams;
	}

	getPendingMeshCommand(): string | null {
		return this.pendingMeshCommand;
	}

	clearPendingMeshCommand(): void {
		this.pendingMeshCommand = null;
	}

	getMeshCommandEpoch(): number {
		return this.meshCommandEpoch;
	}

	getSmoothShading(): boolean {
		return this.smoothShading;
	}

	getInteractionRevision(): number {
		return this.interactionRevision;
	}

	getHoverRevision(): number {
		return this.hoverRevision;
	}

	getHoveredTargetSnapshot(): LowpolyTarget | null {
		return this.getHoveredTarget();
	}

	subscribeHover(listener: () => void): () => void {
		return this.pointerFocus.subscribe(listener);
	}

	subscribeSnapshot(listener: () => void): () => void {
		this.snapshotListeners.add(listener);
		return () => this.snapshotListeners.delete(listener);
	}

	private notifySnapshot(): void {
		for (const listener of this.snapshotListeners) {
			listener();
		}
	}

	private commitFixture(json: string): void {
		if (json === this.fixtureJson) return;
		this.fixtureJson = json;
		this.interactionRevision += 1;
		this.notifySnapshot();
		this.emit();
	}

	private bumpMeshCommand(): void {
		this.meshCommandEpoch += 1;
		this.interactionRevision += 1;
		this.notifySnapshot();
		this.emit();
	}

	private windowEngagement(): WindowEngagement {
		return {
			sessionActive: false,
			input: {
				id: "lowpoly-engagement",
				value: "",
				placeholder: "extrude, inset, mirror, decimate",
				onChange: lowpolyPlayCmd("engagementInput"),
				onSubmit: lowpolyPlayCmd("engagementSubmit"),
			},
			possibleEngagements: [
				{ id: "lowpoly.eng.extrude", label: "Extrude", command: lowpolyPlayCmd("extrude") },
				{ id: "lowpoly.eng.triangulate", label: "Triangulate", command: lowpolyPlayCmd("triangulate") },
			],
			controls: [],
			status: [{ id: "lowpoly-status", text: `${this.selectionMode} · ${this.transformTool}` }],
		};
	}

	private rebuildShellMode(): void {
		this.mainMode.tools = buildLowpolyPlayToolbarTools(LOWPOLY_PLAY_CONTROLLER_ID, this.selectionMode, this.transformTool);
		this.mainMode.windowKinds = [
			new WindowKindRuntime(LOWPOLY_PLAY_WINDOW_KIND_ID, "Model", LOWPOLY_PLAY_BODY_KEY_MAIN, undefined, [], this.windowEngagement()),
		];
		for (const windowKind of this.mainMode.windowKinds) {
			enforcePlaygroundWindowEngagementInput(windowKind.engagement, `Lowpoly play window "${windowKind.id}"`);
		}
	}

	private rebuildPaintMode(): void {
		this.paintMode.tools = buildLowpolyPlayPaintToolbarTools(LOWPOLY_PLAY_CONTROLLER_ID, this.paintTool);
		this.paintMode.defaultLayout = LOWPOLY_PLAY_PAINT_LAYOUT;
		this.paintMode.windowKinds = [
			new WindowKindRuntime(LOWPOLY_PLAY_WINDOW_KIND_ID, "Paint", LOWPOLY_PLAY_BODY_KEY_MAIN, undefined, [], this.windowEngagement()),
			new WindowKindRuntime(LOWPOLY_PLAY_UV_WINDOW_KIND_ID, "UV", LOWPOLY_PLAY_BODY_KEY_UV, undefined, [], this.windowEngagement()),
		];
		for (const windowKind of this.paintMode.windowKinds) {
			enforcePlaygroundWindowEngagementInput(windowKind.engagement, `Lowpoly paint window "${windowKind.id}"`);
		}
	}

	override run(command: string, args?: unknown): void {
		if (command === "setFixtureJson") {
			const json = (args as { json?: string }).json;
			if (typeof json === "string") this.commitFixture(json);
			return;
		}
		if (command === "setSelectionMode") {
			const mode = (args as { mode?: LowpolySelectionMode }).mode;
			if (mode) {
				this.selectionMode = mode;
				this.pointerFocus.setSelection([]);
				this.rebuildShellMode();
				const fixture = parseLowpolyFixtureJson(this.fixtureJson);
				if (fixture) this.commitFixture(lowpolyFixtureToJson({ ...fixture, selection: { mode, ids: [] } }));
			}
			return;
		}
		if (command === "setSelection") {
			const mode = (args as { mode?: LowpolySelectionMode }).mode;
			const ids = (args as { ids?: number[] }).ids;
			const activeObjectId = (args as { activeObjectId?: string }).activeObjectId;
			if (mode) this.selectionMode = mode;
			if (Array.isArray(ids)) this.pointerFocus.setSelection(this.encodeSelectionKeys(ids));
			if (typeof activeObjectId === "string") {
				const fixture = parseLowpolyFixtureJson(this.fixtureJson);
				if (fixture) {
					this.commitFixture(
						lowpolyFixtureToJson({
							...fixture,
							activeObjectId,
							selection: { mode: this.selectionMode, ids: this.decodeSelectionIds() },
						}),
					);
					return;
				}
			}
			this.interactionRevision += 1;
			this.notifySnapshot();
			this.emit();
			return;
		}
		if (command === "setHover") {
			const target = (args as { target?: LowpolyTarget | null }).target ?? null;
			const current = this.getHoveredTarget();
			const unchanged =
				target?.objectId === current?.objectId &&
				target?.mode === current?.mode &&
				target?.id === current?.id;
			if (!unchanged) {
				this.pointerFocus.setHoverFromSource("canvas", target ? encodeLowpolyPointerFocusKey(target) : null);
				this.hoverRevision += 1;
			}
			return;
		}
		if (command === "setPaintTool") {
			const tool = (args as { tool?: LowpolyPaintTool }).tool;
			if (tool) {
				this.paintTool = tool;
				this.rebuildPaintMode();
				this.emit();
			}
			return;
		}
		if (command === "setActivePaintLayer") {
			const layerIndex = (args as { layerIndex?: number }).layerIndex;
			if (typeof layerIndex === "number") {
				this.activePaintLayerIndex = layerIndex;
				this.interactionRevision += 1;
				this.notifySnapshot();
				this.emit();
			}
			return;
		}
		if (command === "paintUndo") {
			this.dispatchPaintVcs({ kind: "undo" });
			return;
		}
		if (command === "paintRedo") {
			this.dispatchPaintVcs({ kind: "redo" });
			return;
		}
		if (command === "unwrapActive" || command === "markUvSeam") {
			this.pendingPaintCommand = { command, args: args as Record<string, unknown> };
			this.bumpMeshCommand();
			return;
		}
		if (command === "setTransformTool") {
			const tool = (args as { tool?: LowpolyTransformTool }).tool;
			if (tool) {
				this.transformTool = tool;
				this.rebuildShellMode();
				this.emit();
			}
			return;
		}
		if (command === "setToolParam") {
			const field = (args as { field?: string; value?: number }).field;
			const value = (args as { value?: number }).value;
			if (field && typeof value === "number") {
				this.toolParams = { ...this.toolParams, [field]: value };
				this.interactionRevision += 1;
				this.notifySnapshot();
				this.emit();
			}
			return;
		}
		if (command === "toggleSelectionTarget") {
			const target = (args as { target?: LowpolyTarget }).target;
			const fixture = parseLowpolyFixtureJson(this.fixtureJson);
			if (!target || !fixture) return;
			const sameSelection = this.selectionMode === target.mode && (target.mode === "object" || fixture.activeObjectId === target.objectId);
			const current = sameSelection ? this.decodeSelectionIds() : [];
			this.selectionMode = target.mode;
			const nextIds = selectionMergeIds("invertive", current.map(String), [String(target.id)]).map(Number);
			this.pointerFocus.setSelection(this.encodeSelectionKeys(nextIds));
			this.rebuildShellMode();
			this.commitFixture(
				lowpolyFixtureToJson({
					...fixture,
					activeObjectId: target.objectId,
					selection: { mode: target.mode, ids: nextIds },
				}),
			);
			return;
		}
		if (command === "addPrimitive") {
			const kind = (args as { kind?: string }).kind ?? "box";
			this.pendingMeshCommand = `addPrimitive:${kind}`;
			this.bumpMeshCommand();
			return;
		}
		if (command === "flipFace") {
			const objectId = (args as { objectId?: string }).objectId;
			const faceId = (args as { faceId?: number }).faceId;
			if (objectId && typeof faceId === "number") {
				this.pendingMeshCommand = `flipFace:${objectId}:${faceId}`;
				this.bumpMeshCommand();
			}
			return;
		}
		const meshCommands = ["extrude", "inset", "flipFaces", "bevel", "loopCut", "merge", "dissolve", "subdivide", "triangulate", "mirror", "decimate", "snap", "toggleSmooth"];
		if (meshCommands.includes(command)) {
			this.pendingMeshCommand = command;
			this.bumpMeshCommand();
			return;
		}
		if (command === "engagementSubmit") {
			const value = ((args as { value?: string }).value ?? "").trim().toLowerCase();
			if (value) this.run(value);
		}
	}
}

function buildLowpolyPlayMainDeclarativeBody(_ctx: WindowBodyViewContext): UiNode {
	return buildPuzzle3dWindowBody(LOWPOLY_PLAY_SURFACE_ID, LOWPOLY_PLAY_CONTROLLER_ID, LOWPOLY_PLAY_WINDOW_KIND_ID);
}

function buildLowpolyPlayUvDeclarativeBody(_ctx: WindowBodyViewContext): UiNode {
	return buildPuzzle3dWindowBody(LOWPOLY_PLAY_UV_SURFACE_ID, LOWPOLY_PLAY_CONTROLLER_ID, LOWPOLY_PLAY_UV_WINDOW_KIND_ID);
}

export function registerLowpolyPlayDeclarativeBodies(): void {
	registerWindowBody(LOWPOLY_PLAY_BODY_KEY_MAIN, buildLowpolyPlayMainDeclarativeBody);
	registerWindowBody(LOWPOLY_PLAY_BODY_KEY_UV, buildLowpolyPlayUvDeclarativeBody);
}

export function buildLowpolyPlayAppRuntime(controller: LowpolyPlayController): AppRuntime {
	const app = createPlayAppRuntime(LOWPOLY_PLAY_APP_ID, "Lowpoly", controller, LOWPOLY_PLAY_LAYOUT, controller.mainMode);
	app.addMode(controller.paintMode);
	return app;
}

/** @emoji 🛝 Lowpoly playground app. */
export class PlaygroundLowpoly extends Playground {
	readonly id = LOWPOLY_PLAY_APP_ID;

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id);
		const ctrl = new LowpolyPlayController(runtime.commandBus, () => runtime.notify());
		runtime.addApp(buildLowpolyPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerLowpolyPlayDeclarativeBodies();
	}
}

export { lowpolyPlayCmd };

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
	describe("LowpolyPlayController", () => {
		it("default fixture json is valid schema", () => {
			expect(LOWPOLY_PLAY_DEFAULT_FIXTURE_JSON).toContain(LOWPOLY_FIXTURE_SCHEMA);
		});
		it("mesh command bumps epoch", () => {
			const bus = new CommandBus();
			const ctrl = new LowpolyPlayController(bus, () => {});
			const before = ctrl.getMeshCommandEpoch();
			ctrl.run("extrude");
			expect(ctrl.getMeshCommandEpoch()).toBeGreaterThan(before);
		});
			it("hover updates do not rebuild side panels", () => {
				const bus = new CommandBus();
				const ctrl = new LowpolyPlayController(bus, () => {});
				const before = ctrl.getInteractionRevision();
				ctrl.run("setHover", { target: { objectId: "obj-1", objectIndex: 0, mode: "face", id: 0 } });
				expect(ctrl.getInteractionRevision()).toBe(before);
				expect(ctrl.getHoverRevision()).toBeGreaterThan(0);
			});
			it("registers model and paint modes", () => {
				const bus = new CommandBus();
				const ctrl = new LowpolyPlayController(bus, () => {});
				const app = buildLowpolyPlayAppRuntime(ctrl);
				expect(app.modes.length).toBe(2);
				expect(ctrl.paintMode.windowKinds.length).toBe(2);
			});
			it("lists and composes mesh topology selections", () => {
				const fixtureJson = lowpolyFixtureToJson({
					...EMPTY_FIXTURE,
					activeObjectId: "obj-1",
					objects: [{
						id: "obj-1",
						name: "Triangle",
						transform: { position: [0, 0, 0], rotation: [0, 0, 0], scale: [1, 1, 1] },
						smoothShading: false,
						meshJson: JSON.stringify({
							vertices: [{}, {}, {}],
							halfedges: [{ twin: null }, { twin: null }, { twin: null }],
							faces: [{}],
						}),
					}],
				});
				let hovered: LowpolyTarget | null = null;
				let flipped: number | null = null;
				const tree = buildLowpolyPlayHierarchyTree(fixtureJson, "face", [0], {
					onHover: (target) => {
						hovered = target;
					},
					onFlipFace: (_objectId, faceId) => {
						flipped = faceId;
					},
				});
				expect(tree.sections[0]?.items[0]?.items?.map((item) => item.label)).toEqual(["Vertices", "Edges", "Faces"]);
				const faceItem = tree.sections[0]?.items[0]?.items?.[2]?.items?.[0];
				faceItem?.onPointerEnter?.();
				faceItem?.actions?.[0]?.onClick();
				expect(hovered).toMatchObject({ mode: "face", id: 0 });
				expect(flipped).toBe(0);
				const bus = new CommandBus();
				const ctrl = new LowpolyPlayController(bus, () => {});
				ctrl.run("setFixtureJson", { json: fixtureJson });
				ctrl.run("toggleSelectionTarget", { target: { objectId: "obj-1", objectIndex: 0, mode: "vertex", id: 0 } });
				ctrl.run("toggleSelectionTarget", { target: { objectId: "obj-1", objectIndex: 0, mode: "vertex", id: 1 } });
				expect(ctrl.getSelectedIds()).toEqual([0, 1]);
			});
		});
	}

//#region 🔖PlaygroundAppDefinition
import type { PlaygroundAppDefinition } from "@semio-tech/framework-playground-core";

/** @emoji 🛝 Lowpoly playground app definition. */
export const lowpolyPlayAppDefinition: PlaygroundAppDefinition = {
	id: LOWPOLY_PLAY_APP_ID,
	label: "Lowpoly",
	controllerId: LOWPOLY_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	createPlayground: () => new PlaygroundLowpoly(),
	bootRenderer: async (pg) => {
		const { bootLowpolyPlay } = await import("@semio-tech/framework-playground-renderer-react/lowpoly");
		await bootLowpolyPlay(pg);
	},
	devHost: {
		playEntryKind: "lowpoly",
		resolveDedupe: ["react", "react-dom", "three", "scheduler", "@semio-tech/lowpoly-react"],
		watchIgnored: ["../core/lib.rs", "../core/target/**", "../core/pkg/**"],
		optimizeDeps: { include: ["react", "react-dom", "three", "@react-three/fiber", "@react-three/drei"] },
	},
};
//#endregion 🔖PlaygroundAppDefinition
