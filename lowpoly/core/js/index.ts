// #region 🧲Header
/** @emoji 🔷 Lowpoly play app — low-poly mesh editing. */
// #endregion 🧲Header

export * from "./internal.ts";

import {
	AppRuntime,
	CommandBus,
	Controller,
	ModeRuntime,
	WindowKindRuntime,
	buildPuzzle3dWindowBody,
	createPlayAppRuntime,
	createDefaultLayout,
	enforcePlaygroundWindowEngagementInput,
	registerWindowBody,
	windowEngagementsEqual,
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
  createPlaygroundApp,
  createProductPlaygroundPlatform,
} from "@semio-tech/framework-playground-core";
import { registerOsMediaExportHandler } from "@semio-tech/framework-os-core";
import { meshTransferToGlb, type MeshTransfer } from "@semio-tech/kernel-3d-js";
import { createLowpolySession } from "@semio-tech/lowpoly-react";
import { selectionMergeIds } from "@semio-tech/ui-react";
import {
	DEFAULT_LOWPOLY_SELECTION,
	LOWPOLY_FIXTURE_SCHEMA,
	type LowpolyFixture,
	type LowpolySelectionMode,
	type LowpolySelectionTargets,
	type LowpolyTarget,
	lowpolyTopologyFromMeshJson,
	lowpolyFixtureToJson,
	parseLowpolyFixtureJson,
	encodeLowpolyPointerFocusKey,
	decodeLowpolyPointerFocusKey,
	decodeLowpolySelectionTargets,
	formatLowpolySelectionTargetsLabel,
	lowpolySelectionFromState,
	LOWPOLY_SELECTION_TARGETS_DEFAULT,
	normalizeLowpolySelectionMode,
	selectedIdsForMode,
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
  type LowpolyTessellation,
} from "./internal.ts";

export const LOWPOLY_PLAY_APP_ID = "lowpoly-play";
export const LOWPOLY_PLAY_CONTROLLER_ID = "lowpoly-play";
export const LOWPOLY_PLAY_BODY_KEY_MAIN = "lowpoly.play.main";
export const LOWPOLY_PLAY_SURFACE_ID = "lowpoly.play";
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
	selectionTargets: LowpolySelectionTargets,
	transformTool: LowpolyTransformTool,
): AppTools {
	const kindToggle = (id: string, label: string, kind: LowpolySelectionMode): ToolLeaf => ({
		id,
		kind: "toggle",
		label,
		iconId: kind === "vertex" ? "circle" : kind === "edge" ? "minus" : kind === "face" ? "square" : "box",
		pressed: selectionTargets[kind],
		controllerId,
		command: "toggleSelectionKind",
		args: { kind },
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
			kindToggle("lowpoly.mode.mesh", "Mesh", "mesh"),
			kindToggle("lowpoly.mode.vertex", "Vertex", "vertex"),
			kindToggle("lowpoly.mode.edge", "Edge", "edge"),
			kindToggle("lowpoly.mode.face", "Face", "face"),
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
	selectedTargets: readonly LowpolyTarget[],
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
		const meshTarget: LowpolyTarget = { objectId: object.id, objectIndex, mode: "mesh", id: objectIndex };
		const topology = lowpolyTopologyFromMeshJson(object.meshJson);
		const componentGroup = (mode: Exclude<LowpolySelectionMode, "mesh">, ids: readonly number[], icon: string): UiTreeItemNode => ({
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
			...targetItem(meshTarget, object.name, "box"),
			description: object.id,
			defaultOpen: object.id === fixture.activeObjectId,
			items: [
				componentGroup("vertex", topology.vertexIds, "circle"),
				componentGroup("edge", topology.edgeIds, "minus"),
				componentGroup("face", topology.faceIds, "square"),
			],
		};
	});
	const selectedRowIds = selectedTargets.map((target) => lowpolyHierarchyTargetRowId(target));
	return {
		type: "tree",
		sections: [
			{
				id: "lowpoly-hierarchy.meshes",
				label: "Meshes",
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
			{ type: "section", id: "lowpoly-inspector.empty", label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, children: [{ type: "text", value: "Add or select a mesh." }] },
		]);
	}
	const active = fixture.objects.find((o) => o.id === fixture.activeObjectId) ?? fixture.objects[0];
	const groups: UiInspectorFieldGroup[] = [
		{
			id: "lowpoly-inspector.mesh",
			label: "Mesh",
			fields: [
				uiInspectorReadonlyField("lowpoly-inspector.name", "Name", active?.name ?? ""),
				uiInspectorReadonlyField("lowpoly-inspector.mode", "Selection", `${formatLowpolySelectionTargetsLabel(fixture.selection.targets)} · ${fixture.selection.keys.length} selected`),
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
	private selectionTargets: LowpolySelectionTargets = { ...LOWPOLY_SELECTION_TARGETS_DEFAULT };
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
	private hoveredTargetSnapshotKey: string | null = null;
	private hoveredTargetSnapshot: LowpolyTarget | null = null;
	private readonly snapshotListeners = new Set<() => void>();

	private encodeSelectionKeys(targets: readonly LowpolyTarget[]): string[] {
		return targets.map((target) => encodeLowpolyPointerFocusKey(target));
	}

	private decodeSelectionTargets(): LowpolyTarget[] {
		return decodeLowpolySelectionTargets(this.pointerFocus.getSnapshot().selection);
	}

	private currentSelection(): ReturnType<typeof lowpolySelectionFromState> {
		return lowpolySelectionFromState(this.selectionTargets, this.pointerFocus.getSnapshot().selection);
	}

	private persistSelection(fixture: LowpolyFixture, activeObjectId?: string): void {
		const selection = this.currentSelection();
		this.commitFixture(
			lowpolyFixtureToJson({
				...fixture,
				activeObjectId: activeObjectId ?? fixture.activeObjectId,
				selection,
			}),
		);
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

	getSelectionTargets(): LowpolySelectionTargets {
		return this.selectionTargets;
	}

	getSelectionMode(): LowpolySelectionMode {
		return this.currentSelection().mode;
	}

	getSelectedTargets(): readonly LowpolyTarget[] {
		return this.decodeSelectionTargets();
	}

	getSelectedIds(mode?: LowpolySelectionMode): readonly number[] {
		const targets = this.decodeSelectionTargets();
		if (mode) return selectedIdsForMode(targets, mode);
		return targets.map((target) => target.id);
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
		const hoverKey = this.pointerFocus.getSnapshot().hover;
		if (hoverKey === this.hoveredTargetSnapshotKey) {
			return this.hoveredTargetSnapshot;
		}
		this.hoveredTargetSnapshotKey = hoverKey;
		this.hoveredTargetSnapshot = hoverKey ? decodeLowpolyPointerFocusKey(hoverKey) : null;
		return this.hoveredTargetSnapshot;
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
		this.bumpInteraction();
	}

	private bumpMeshCommand(): void {
		this.meshCommandEpoch += 1;
		this.bumpInteraction();
	}

	private windowEngagement(): WindowEngagement {
		const transformOption = (tool: LowpolyTransformTool, label: string, iconId: string) => ({
			id: `lowpoly.opt.${tool}`,
			label,
			iconId,
			pressed: this.transformTool === tool,
			command: lowpolyPlayCmd("setTransformTool", { tool }),
		});
		return {
			sessionActive: true,
			options: [
				transformOption("move", "Move", "move"),
				transformOption("rotate", "Rotate", "rotate-cw"),
				transformOption("scale", "Scale", "maximize-2"),
				{ id: "lowpoly.opt.snap", label: "Snap", iconId: "magnet", command: lowpolyPlayCmd("snap") },
				{ id: "lowpoly.opt.smooth", label: "Smooth", iconId: "sun", command: lowpolyPlayCmd("toggleSmooth") },
			],
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
			status: [{ id: "lowpoly-status", text: `${formatLowpolySelectionTargetsLabel(this.selectionTargets)} · ${this.transformTool} · ${this.decodeSelectionTargets().length} selected` }],
		};
	}

	private syncWindowEngagement(): void {
		const next = this.windowEngagement();
		for (const mode of [this.mainMode, this.paintMode]) {
			let changed = false;
			for (const windowKind of mode.windowKinds) {
				if (windowEngagementsEqual(windowKind.engagement, next)) continue;
				windowKind.engagement = next;
				changed = true;
			}
			if (changed) mode.windowKinds = [...mode.windowKinds];
		}
	}

	private rebuildShellMode(): void {
		this.mainMode.tools = buildLowpolyPlayToolbarTools(LOWPOLY_PLAY_CONTROLLER_ID, this.selectionTargets, this.transformTool);
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

	private bumpInteraction(): void {
		this.interactionRevision += 1;
		this.notifySnapshot();
		this.syncWindowEngagement();
		this.emit();
	}

	override run(command: string, args?: unknown): void {
		if (command === "setFixtureJson") {
			const json = (args as { json?: string }).json;
			if (typeof json === "string") {
				const fixture = parseLowpolyFixtureJson(json);
				if (fixture) {
					this.selectionTargets = { ...fixture.selection.targets };
					this.pointerFocus.setSelection([...fixture.selection.keys]);
				}
				this.commitFixture(json);
			}
			return;
		}
		if (command === "toggleSelectionKind") {
			const kind = normalizeLowpolySelectionMode(String((args as { kind?: string }).kind ?? ""));
			const next = { ...this.selectionTargets, [kind]: !this.selectionTargets[kind] };
			if (!Object.values(next).some(Boolean)) return;
			this.selectionTargets = next;
			this.rebuildShellMode();
			const fixture = parseLowpolyFixtureJson(this.fixtureJson);
			if (fixture) this.persistSelection(fixture);
			else {
				this.interactionRevision += 1;
				this.notifySnapshot();
				this.syncWindowEngagement();
				this.emit();
			}
			return;
		}
		if (command === "setSelection") {
			const keys = (args as { keys?: string[] }).keys;
			const activeObjectId = (args as { activeObjectId?: string }).activeObjectId;
			if (Array.isArray(keys)) this.pointerFocus.setSelection([...keys]);
			const fixture = parseLowpolyFixtureJson(this.fixtureJson);
			if (fixture) {
				this.persistSelection(fixture, activeObjectId);
				return;
			}
			this.interactionRevision += 1;
			this.notifySnapshot();
			this.syncWindowEngagement();
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
			const currentKeys = this.pointerFocus.getSnapshot().selection;
			const targetKey = encodeLowpolyPointerFocusKey(target);
			const nextKeys = selectionMergeIds("invertive", currentKeys, [targetKey]);
			this.pointerFocus.setSelection(nextKeys);
			this.selectionTargets = { ...this.selectionTargets, [target.mode]: true };
			this.rebuildShellMode();
			this.persistSelection({ ...fixture, activeObjectId: target.objectId }, target.objectId);
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
		it("hover snapshot keeps a stable reference until hover changes", () => {
			const bus = new CommandBus();
			const ctrl = new LowpolyPlayController(bus, () => {});
			const target = { objectId: "obj-1", objectIndex: 0, mode: "face" as const, id: 0 };
			ctrl.run("setHover", { target });
			const first = ctrl.getHoveredTargetSnapshot();
			const second = ctrl.getHoveredTargetSnapshot();
			expect(first).toBe(second);
			ctrl.run("setHover", { target: { ...target, id: 1 } });
			expect(ctrl.getHoveredTargetSnapshot()).not.toBe(first);
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
			const selectedTargets: readonly LowpolyTarget[] = [{ objectId: "obj-1", objectIndex: 0, mode: "face", id: 0 }];
			const tree = buildLowpolyPlayHierarchyTree(fixtureJson, selectedTargets, {
				onHover: (target) => {
					hovered = target;
				},
				onFlipFace: (_objectId, faceId) => {
					flipped = faceId;
				},
			});
			expect(tree.sections[0]?.items[0]?.items?.map((item) => item.label)).toEqual(["Vertices", "Edges", "Faces"]);
			expect(tree.selectedIds).toEqual(["lowpoly-hierarchy.obj-1.face.0"]);
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

//#region 🔖MediaExport
function lowpolyTessellationToMeshTransfer(tess: LowpolyTessellation): MeshTransfer {
	return {
		position: tess.positions,
		normal: tess.normals,
		index: tess.indices,
		edges: tess.edgePositions,
		faceGroups: [],
		edgeGroups: [],
		faceInfos: [],
		edgeInfos: [],
	};
}

async function lowpolyFixtureToSession(fixture: LowpolyFixture) {
	const session = await createLowpolySession(lowpolyFixtureToJson(fixture));
	return session;
}

/** @emoji 💾 Registers lowpoly fixture OBJ/GLB export handlers for the OS media graph. */
export function registerLowpolyMediaExportHandlers(): void {
	registerOsMediaExportHandler("3d.lowpoly", "obj", async (doc) => {
		const fixture = doc as LowpolyFixture;
		const session = await lowpolyFixtureToSession(fixture);
		return { data: session.exportObjActive(), mimeType: "text/plain", fileName: "lowpoly.obj" };
	});
	registerOsMediaExportHandler("3d.lowpoly", "glb", async (doc) => {
		const fixture = doc as LowpolyFixture;
		const session = await lowpolyFixtureToSession(fixture);
		const { parseLowpolyTessellationJson } = await import("@semio-tech/lowpoly-react");
		const tess = parseLowpolyTessellationJson(session.tessellateActive());
		if (!tess) return { data: new Uint8Array([0x67, 0x6c, 0x54, 0x46, 0x02, 0x00, 0x00, 0x00]), mimeType: "model/gltf-binary", fileName: "lowpoly.glb" };
		return { data: meshTransferToGlb(lowpolyTessellationToMeshTransfer(tess)), mimeType: "model/gltf-binary", fileName: "lowpoly.glb" };
	});
}
//#endregion 🔖MediaExport

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";

/** @emoji 🧩 S program definition for lowpoly. */
export function buildLowpolyProgramDefinition(): PlatformDefinition {
	return {
		id: "lowpoly",
		name: "Lowpoly",
		apiVersion: "1",
		apps: [{ id: "lowpoly", label: "Lowpoly", controllerId: LOWPOLY_PLAY_CONTROLLER_ID, modes: [{ id: "edit", label: "Edit" }], defaultModeId: "edit" }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension

//#region 🔖Play

/** @emoji 🛝 Lowpoly playground app. */


export const lowpolyPlayAppDefinition = createPlaygroundApp({
	id: LOWPOLY_PLAY_APP_ID,
	label: "Lowpoly",
	controllerId: "lowpoly-play",
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	devHost: {
		playEntryKind: "lowpoly",
		resolveDedupe: ["react", "react-dom", "three", "scheduler", "@semio-tech/lowpoly-react"],
		watchIgnored: ["../core/lib.rs", "../core/target/**", "../core/pkg/**"],
		optimizeDeps: { include: ["react", "react-dom", "three", "@react-three/fiber", "@react-three/drei"] },
	},
	createRuntime: () => {
		const runtime = createProductPlaygroundPlatform(LOWPOLY_PLAY_APP_ID);
			const ctrl = new LowpolyPlayController(runtime.commandBus, () => runtime.notify());
			runtime.addApp(buildLowpolyPlayAppRuntime(ctrl));
			return runtime;
	},
	registerBodies: () => {
		registerLowpolyPlayDeclarativeBodies();
	},
	bootRenderer: async (pg) => {
		const { bootLowpolyPlay } = await import("@semio-tech/framework-playground-renderer-react/lowpoly");
		await bootLowpolyPlay(pg);
	},
});
//#endregion 🔖Play
