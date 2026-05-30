// #region 🧲Header
// 💻 cad/js/renderer/play/main.tsx — Spatial play shell (headless + React chrome + Vite entry).
// #endregion 🧲Header

import {
	CommandBus,
	Controller,
	ProductRuntime,
	AppRuntime,
	ModeRuntime,
	WindowKindRuntime,
	buildScene3dWindowBody,
	createWindowLayout,
	registerWindowBody,
	type AppTools,
	type ToolItem,
	type WindowBodyViewContext,
	type WindowMeasure,
	type UiNode,
	type WindowLayout,
} from "@framework/playground";
import {
	DocumentHistory,
	SHAPE_MODEL_DEFINITION_ID,
	applyTransformation,
	buildModelTopologyHierarchy,
	countViewObjectsForModelDefinition,
	createInteractionRuntime,
	isEmptyModelDiff,
	isInteractionSessionActive,
	isShapeModelDefinition,
	listModelDefinitionManifests,
	listModelObjectsForModelDefinition,
	listSpatialInteractionsForModelDefinition,
	listTransformationsFromModelDefinition,
	listTransformationsIntoModelDefinition,
	loadSpatialInteraction,
	Model,
	ModelSpace,
	modelDefinitionSelectionEntityKinds,
	modelDefinitionUsesGeometryPicking,
	objectPrimitiveEntries,
	parseModelJson,
	qualifiedTransformationId,
	resolveModelDefinitionScope,
	resolvePrimitiveRefKind,
	typologyObjectPascalFromLabel,
	type InteractionRuntime,
	type InteractionRuntimeOptions,
	type InteractionSnapshot,
	type InteractionSpec,
	type ModelDocument,
	type ModelTopologyHierarchyNode,
	type SelectionTarget,
	type SpatialComputeMode,
	type TransformationSpec,
} from "@cad/js/core";

/** @emoji ⚡ Per-window compute mode options for spatial play window measures. */
export const SPATIAL_PLAY_COMPUTE_MODES: readonly SpatialComputeMode[] = ["fast", "precise"];

//#region 🔖Ids
export const SPATIAL_PLAY_APP_ID = "spatial-play";
export const SPATIAL_PLAY_CONTROLLER_ID = "spatial-play";
export const SPATIAL_PLAY_HIERARCHY_TAB_ID = "spatial-play-hierarchy";

export const SPATIAL_PLAY_BUILDING_MODEL_DEFINITION_ID = "aec.building";
export const SPATIAL_PLAY_ENERGY_MODEL_DEFINITION_ID = "aec.building.energy";
export const SPATIAL_PLAY_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID = "aec.building.structure.classic";

export type SpatialPlayPaneId = "shape" | "building" | "energy" | "structure-classic";

export const SPATIAL_PLAY_SHAPE_WINDOW_ID = "spatial-play-shape";
export const SPATIAL_PLAY_BUILDING_WINDOW_ID = "spatial-play-building";
export const SPATIAL_PLAY_ENERGY_WINDOW_ID = "spatial-play-energy";
export const SPATIAL_PLAY_STRUCTURE_CLASSIC_WINDOW_ID = "spatial-play-structure-classic";

export const SPATIAL_PLAY_SHAPE_WINDOW_LABEL = "Shape";
export const SPATIAL_PLAY_BUILDING_WINDOW_LABEL = "Building";
export const SPATIAL_PLAY_ENERGY_WINDOW_LABEL = "Energy";
export const SPATIAL_PLAY_STRUCTURE_CLASSIC_WINDOW_LABEL = "Structure Classic";

export const SPATIAL_PLAY_SHAPE_BODY_KEY = "spatial.play.shape";
export const SPATIAL_PLAY_BUILDING_BODY_KEY = "spatial.play.building";
export const SPATIAL_PLAY_ENERGY_BODY_KEY = "spatial.play.energy";
export const SPATIAL_PLAY_STRUCTURE_CLASSIC_BODY_KEY = "spatial.play.structure-classic";

export const SPATIAL_PLAY_SHAPE_SCENE_SURFACE_ID = "spatial.play.scene3d/shape";
export const SPATIAL_PLAY_BUILDING_SCENE_SURFACE_ID = "spatial.play.scene3d/building";
export const SPATIAL_PLAY_ENERGY_SCENE_SURFACE_ID = "spatial.play.scene3d/energy";
export const SPATIAL_PLAY_STRUCTURE_CLASSIC_SCENE_SURFACE_ID = "spatial.play.scene3d/structure-classic";

/** @emoji 🪟 Quad play layout: shape/building left, energy/structure classic right. */
export const SPATIAL_PLAY_LAYOUT: WindowLayout = {
	root: {
		kind: "row",
		children: [
			{
				kind: "column",
				size: 50,
				children: [
					{ kind: "stack", size: 50, children: [createWindowLayout(SPATIAL_PLAY_SHAPE_WINDOW_ID, SPATIAL_PLAY_SHAPE_WINDOW_LABEL)] },
					{ kind: "stack", size: 50, children: [createWindowLayout(SPATIAL_PLAY_BUILDING_WINDOW_ID, SPATIAL_PLAY_BUILDING_WINDOW_LABEL)] },
				],
			},
			{
				kind: "column",
				size: 50,
				children: [
					{ kind: "stack", size: 50, children: [createWindowLayout(SPATIAL_PLAY_ENERGY_WINDOW_ID, SPATIAL_PLAY_ENERGY_WINDOW_LABEL)] },
					{
						kind: "stack",
						size: 50,
						children: [createWindowLayout(SPATIAL_PLAY_STRUCTURE_CLASSIC_WINDOW_ID, SPATIAL_PLAY_STRUCTURE_CLASSIC_WINDOW_LABEL)],
					},
				],
			},
		],
	},
};

const SPATIAL_PLAY_PANE_SPECS: readonly {
	readonly pane: SpatialPlayPaneId;
	readonly windowKindId: string;
	readonly label: string;
	readonly bodyKey: string;
	readonly surfaceId: string;
	readonly modelDefinitionId: string;
}[] = [
	{
		pane: "shape",
		windowKindId: SPATIAL_PLAY_SHAPE_WINDOW_ID,
		label: SPATIAL_PLAY_SHAPE_WINDOW_LABEL,
		bodyKey: SPATIAL_PLAY_SHAPE_BODY_KEY,
		surfaceId: SPATIAL_PLAY_SHAPE_SCENE_SURFACE_ID,
		modelDefinitionId: SHAPE_MODEL_DEFINITION_ID,
	},
	{
		pane: "building",
		windowKindId: SPATIAL_PLAY_BUILDING_WINDOW_ID,
		label: SPATIAL_PLAY_BUILDING_WINDOW_LABEL,
		bodyKey: SPATIAL_PLAY_BUILDING_BODY_KEY,
		surfaceId: SPATIAL_PLAY_BUILDING_SCENE_SURFACE_ID,
		modelDefinitionId: SPATIAL_PLAY_BUILDING_MODEL_DEFINITION_ID,
	},
	{
		pane: "energy",
		windowKindId: SPATIAL_PLAY_ENERGY_WINDOW_ID,
		label: SPATIAL_PLAY_ENERGY_WINDOW_LABEL,
		bodyKey: SPATIAL_PLAY_ENERGY_BODY_KEY,
		surfaceId: SPATIAL_PLAY_ENERGY_SCENE_SURFACE_ID,
		modelDefinitionId: SPATIAL_PLAY_ENERGY_MODEL_DEFINITION_ID,
	},
	{
		pane: "structure-classic",
		windowKindId: SPATIAL_PLAY_STRUCTURE_CLASSIC_WINDOW_ID,
		label: SPATIAL_PLAY_STRUCTURE_CLASSIC_WINDOW_LABEL,
		bodyKey: SPATIAL_PLAY_STRUCTURE_CLASSIC_BODY_KEY,
		surfaceId: SPATIAL_PLAY_STRUCTURE_CLASSIC_SCENE_SURFACE_ID,
		modelDefinitionId: SPATIAL_PLAY_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID,
	},
];

/** @emoji 🧭 Maps a spatial play scene surface id to its pane id. */
export function spatialPlayPaneFromSurfaceId(surfaceId: string): SpatialPlayPaneId | null {
	return SPATIAL_PLAY_PANE_SPECS.find((row) => row.surfaceId === surfaceId)?.pane ?? null;
}

/** @emoji 🧭 Active model definition for a spatial play pane. */
export function spatialPlayModelDefinitionIdForPane(pane: SpatialPlayPaneId): string {
	return SPATIAL_PLAY_PANE_SPECS.find((row) => row.pane === pane)!.modelDefinitionId;
}

/** @emoji 🧭 Scene surface id for a spatial play pane. */
export function spatialPlaySceneSurfaceIdForPane(pane: SpatialPlayPaneId): string {
	return SPATIAL_PLAY_PANE_SPECS.find((row) => row.pane === pane)!.surfaceId;
}

/** @emoji 🧭 Maps a spatial play window kind id to its pane id. */
export function spatialPlayPaneFromWindowKindId(windowKindId: string): SpatialPlayPaneId | null {
	return SPATIAL_PLAY_PANE_SPECS.find((row) => row.windowKindId === windowKindId)?.pane ?? null;
}

function isSpatialComputeMode(value: string): value is SpatialComputeMode {
	return value === "fast" || value === "precise";
}
//#endregion 🔖Ids

//#region 🔖SpatialPlayHierarchy
function spatialPlayModelDefinitionLabel(modelDefinitionId: string): string {
	const manifest = listModelDefinitionManifests().find((row) => row.id === modelDefinitionId);
	if (manifest?.label?.trim()) {
		return `${manifest.label}`;
	}
	const tail = modelDefinitionId.split(".").pop() ?? modelDefinitionId;
	return typologyObjectPascalFromLabel(tail.replace(/[._-]+/g, " "));
}

function spatialPlaySelectionKey(target: SelectionTarget): string {
	return `${target.kind}:${target.id}`;
}

/** @emoji 🔢 Digest for hierarchy chrome when {@link Model} instances mutate in place (revision, objects, topology counts). */
export function spatialPlayModelsDigest(modelsByDefinitionId: Record<string, Model>): string {
	return Object.keys(modelsByDefinitionId)
		.sort((a, b) => a.localeCompare(b))
		.map((modelDefinitionId) => {
			const model = modelsByDefinitionId[modelDefinitionId];
			if (!model) return `${modelDefinitionId}:missing`;
			return [
				modelDefinitionId,
				model.revision,
				Object.keys(model.objects).length,
				Object.keys(model.solids).length,
				Object.keys(model.faces).length,
				Object.keys(model.vertices).length,
			].join(":");
		})
		.join("|");
}

type SpatialPlayHierarchyPickContext = {
	readonly modelDefinitionId: string;
	readonly isSelected: (kind: SelectionTarget["kind"], id: string) => boolean;
	readonly onSelect: (modelDefinitionId: string, target: SelectionTarget) => void;
};

function spatialPlayTopologyTreeItem(
	node: ModelTopologyHierarchyNode,
	path: string,
	ctx: SpatialPlayHierarchyPickContext,
): TreeDataItem {
	const childItems = node.children.map((child) =>
		spatialPlayTopologyTreeItem(child, `${path}.${child.kind}.${child.id}`, ctx),
	);
	return {
		id: `spatial-play-hierarchy.topology.${path}`,
		label: `${node.kind} ${node.id}`,
		isSelected: ctx.isSelected(node.kind, node.id),
		defaultOpen: node.kind === "solid" || node.kind === "shell" || node.kind === "face",
		onClick: () => ctx.onSelect(ctx.modelDefinitionId, { kind: node.kind, id: node.id, editable: true }),
		...(childItems.length > 0 ? { items: childItems } : {}),
	};
}

function spatialPlayPrimitiveSlotTreeItems(
	model: Model,
	modelDefinitionId: string,
	objectId: string,
	slot: string,
	primitiveRef: string,
	ctx: SpatialPlayHierarchyPickContext,
): TreeDataItem {
	const kind = resolvePrimitiveRefKind(model, primitiveRef) ?? "solid";
	const primitiveId = String(primitiveRef);
	const topology = buildModelTopologyHierarchy(model, primitiveId);
	const topologyItems = (topology?.children ?? []).map((child) =>
		spatialPlayTopologyTreeItem(
			child,
			`${modelDefinitionId}.${objectId}.${slot}.${child.kind}.${child.id}`,
			ctx,
		),
	);
	return {
		id: `spatial-play-hierarchy.primitive.${modelDefinitionId}.${objectId}.${slot}`,
		label: `${slot}: ${kind} ${primitiveId}`,
		isSelected: ctx.isSelected(kind, primitiveId),
		defaultOpen: true,
		onClick: () => ctx.onSelect(ctx.modelDefinitionId, { kind, id: primitiveId, editable: true }),
		items: topologyItems.length
			? topologyItems
			: [{ id: `spatial-play-hierarchy.primitive.${modelDefinitionId}.${objectId}.${slot}.topology.empty`, label: "(empty)" }],
	};
}

/** @emoji 🌳 ModelSpace → model definition → object → primitive slot tree for spatial play workbench. */
export function buildSpatialPlayHierarchySections(
	modelsByDefinitionId: Record<string, Model>,
	activeModelDefinitionId: string,
	selection: readonly SelectionTarget[],
	onSelect: (modelDefinitionId: string, target: SelectionTarget) => void,
): TreeDataSection[] {
	const selectedKeys = new Set(selection.map(spatialPlaySelectionKey));
	const isSelected = (kind: SelectionTarget["kind"], id: string): boolean => selectedKeys.has(`${kind}:${id}`);
	const modelDefinitionIds = Object.keys(modelsByDefinitionId).sort((a, b) => a.localeCompare(b));
	const modelBranches: TreeDataItem[] = [];
	for (const modelDefinitionId of modelDefinitionIds) {
		const model = modelsByDefinitionId[modelDefinitionId];
		if (!model) {
			continue;
		}
		const pickCtx: SpatialPlayHierarchyPickContext = { modelDefinitionId, isSelected, onSelect };
		const objectItems: TreeDataItem[] = listModelObjectsForModelDefinition(model, modelDefinitionId).map((object) => {
			const objectId = String(object.id);
			const typologyTail = object.typology.split(".").pop() ?? object.typology;
			const primitiveItems: TreeDataItem[] = objectPrimitiveEntries(object).map(([slot, primitiveRef]) =>
				spatialPlayPrimitiveSlotTreeItems(model, modelDefinitionId, objectId, slot, primitiveRef, pickCtx),
			);
			return {
				id: `spatial-play-hierarchy.object.${modelDefinitionId}.${objectId}`,
				label: `${typologyObjectPascalFromLabel(typologyTail.replace(/[._-]+/g, " "))} (${objectId})`,
				description: object.typology,
				isSelected: isSelected("object", objectId),
				defaultOpen: true,
				onClick: () => onSelect(modelDefinitionId, { kind: "object", id: objectId, editable: true }),
				items: primitiveItems.length
					? primitiveItems
					: [{ id: `spatial-play-hierarchy.object.${modelDefinitionId}.${objectId}.primitives.empty`, label: "(none)" }],
			};
		});
		modelBranches.push({
			id: `spatial-play-hierarchy.model.${modelDefinitionId}`,
			label: spatialPlayModelDefinitionLabel(modelDefinitionId),
			description: modelDefinitionId,
			defaultOpen: modelDefinitionId === activeModelDefinitionId,
			items: objectItems.length
				? objectItems
				: [{ id: `spatial-play-hierarchy.model.${modelDefinitionId}.objects.empty`, label: "(no objects)" }],
		});
	}
	const modelSpaceRoot: TreeDataItem = {
		id: "spatial-play-hierarchy.modelspace",
		label: "ModelSpace",
		defaultOpen: true,
		items: modelBranches.length
			? modelBranches
			: [{ id: "spatial-play-hierarchy.modelspace.empty", label: "(empty)" }],
	};
	return [{ id: "spatial-play-hierarchy.root", defaultOpen: true, items: [modelSpaceRoot] }];
}
//#endregion 🔖SpatialPlayHierarchy

//#region 🔖Toolbar
/** @emoji 🧰 Snapshot for {@link buildSpatialPlayToolbarTools}. */
export interface SpatialPlayToolbarState {
	readonly activeModelDefinitionId: string;
	readonly selectionCount: number;
	readonly transformsTo: readonly TransformationSpec[];
	readonly transformsFrom: readonly TransformationSpec[];
}

/** @emoji 🔗 React host bridge for spatial play toolbar commands. */
export interface SpatialPlayHostBridge {
	getToolbarState(): SpatialPlayToolbarState;
	runHostCommand(command: string, args?: unknown): void;
}

/** @emoji 🧰 Playground {@link AppTools} for spatial play (view, save, transform). */
export function buildSpatialPlayToolbarTools(state: SpatialPlayToolbarState, controllerId: string): AppTools {
	const viewTools: ToolItem[] = listModelDefinitionManifests().map((row, index) => ({
		id: `spatial.play.view.${row.id}`,
		kind: "toggle",
		text: row.label,
		title: row.id,
		order: index,
		pressed: state.activeModelDefinitionId === row.id,
		controllerId,
		command: "focusModelDefinition",
		args: { modelDefinitionId: row.id },
	}));
	const saveTools: ToolItem[] = [
		{
			id: "spatial.play.save.selected",
			kind: "button",
			label: "Selected",
			order: 0,
			disabled: state.selectionCount === 0,
			controllerId,
			command: "saveSelected",
		},
		{
			id: "spatial.play.save.modelspace",
			kind: "button",
			label: "Model space",
			order: 1,
			controllerId,
			command: "saveInPlay",
		},
		{
			id: "spatial.play.save.current",
			kind: "button",
			label: "Current",
			order: 2,
			controllerId,
			command: "saveCurrent",
		},
		{
			id: "spatial.play.save.load",
			kind: "button",
			label: "Load",
			order: 3,
			controllerId,
			command: "loadRawRequest",
		},
	];
	const transformTools: ToolItem[] = [
		...state.transformsTo.map((spec, index) => ({
			id: `spatial.play.transform.to.${qualifiedTransformationId(spec.modelDefinitionId, spec.id)}`,
			kind: "button" as const,
			label: `→ ${spec.label}`,
			title: spec.target.modelDefinition,
			order: index,
			controllerId,
			command: "applyTransformation",
			args: { qid: qualifiedTransformationId(spec.modelDefinitionId, spec.id) },
		})),
		...(state.transformsTo.length > 0 && state.transformsFrom.length > 0
			? [{ id: "spatial.play.transform.separator", kind: "separator" as const, order: state.transformsTo.length }]
			: []),
		...state.transformsFrom.map((spec, index) => ({
			id: `spatial.play.transform.from.${qualifiedTransformationId(spec.modelDefinitionId, spec.id)}`,
			kind: "button" as const,
			label: `← ${spec.label}`,
			title: spec.source.modelDefinition,
			order: state.transformsTo.length + (state.transformsTo.length > 0 && state.transformsFrom.length > 0 ? 1 : 0) + index,
			controllerId,
			command: "applyTransformation",
			args: { qid: qualifiedTransformationId(spec.modelDefinitionId, spec.id) },
		})),
	];
	return {
		view: viewTools,
		save: saveTools,
		...(transformTools.length > 0 ? { transform: transformTools } : {}),
	};
}
//#endregion 🔖Toolbar

//#region 🔖Controller
/** @emoji 🎛 Spatial play shell controller: quad viewports + playground toolbar categories. */
export class SpatialPlayShellController extends Controller {
	readonly mainMode = new ModeRuntime("main", "Spatial", undefined);
	private hostBridge: SpatialPlayHostBridge | null = null;
	private computeModeByPane: Record<SpatialPlayPaneId, SpatialComputeMode>;

	constructor(commandBus: CommandBus, hostNotify: () => void) {
		super(SPATIAL_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.computeModeByPane = {
			shape: "fast",
			building: "fast",
			energy: "fast",
			"structure-classic": "fast",
		};
		this.rebuildShellMode();
	}

	private computeMeasureForPane(pane: SpatialPlayPaneId): WindowMeasure {
		return {
			kind: "select",
			id: `${pane}-compute`,
			label: "Compute",
			value: this.computeModeByPane[pane],
			items: SPATIAL_PLAY_COMPUTE_MODES.map((mode) => ({
				id: mode,
				value: mode,
				label: mode === "fast" ? "Fast" : "Precise",
			})),
			onChange: { controllerId: SPATIAL_PLAY_CONTROLLER_ID, command: "setComputeModeForPane", args: { pane } },
		};
	}

	/** @emoji 🔄 Rebuilds quad window kinds with per-pane compute measures. */
	rebuildShellMode(): void {
		this.mainMode.windowKinds = SPATIAL_PLAY_PANE_SPECS.map(
			(row) => new WindowKindRuntime(row.windowKindId, row.label, row.bodyKey, undefined, [this.computeMeasureForPane(row.pane)]),
		);
	}

	/** @emoji ⚡ Returns compute mode for one quad pane. */
	getComputeModeForPane(pane: SpatialPlayPaneId): SpatialComputeMode {
		return this.computeModeByPane[pane];
	}

	/** @emoji ⚡ Snapshot of compute modes for all quad panes. */
	getComputeModeByPane(): Readonly<Record<SpatialPlayPaneId, SpatialComputeMode>> {
		return this.computeModeByPane;
	}

	/** @emoji 🔗 Attaches the React host bridge used for toolbar commands and snapshots. */
	setHostBridge(bridge: SpatialPlayHostBridge | null): void {
		this.hostBridge = bridge;
		this.rebuildToolbarTools();
	}

	/** @emoji 🔄 Rebuilds {@link ModeRuntime.tools} from the latest host toolbar snapshot. */
	rebuildToolbarTools(): void {
		if (!this.hostBridge) {
			this.mainMode.tools = undefined;
			return;
		}
		this.mainMode.tools = buildSpatialPlayToolbarTools(this.hostBridge.getToolbarState(), this.id);
	}

	override run(command: string, args?: unknown): void {
		switch (command) {
			case "setComputeModeForPane": {
				const { pane, value } = args as { pane?: SpatialPlayPaneId; value?: string };
				if (!pane || !SPATIAL_PLAY_PANE_SPECS.some((row) => row.pane === pane)) break;
				if (!value || !isSpatialComputeMode(value)) break;
				if (this.computeModeByPane[pane] === value) break;
				this.computeModeByPane = { ...this.computeModeByPane, [pane]: value };
				this.rebuildShellMode();
				break;
			}
			case "focusModelDefinition":
			case "applyTransformation":
			case "saveSelected":
			case "saveInPlay":
			case "saveCurrent":
			case "loadRawRequest":
				this.hostBridge?.runHostCommand(command, args);
				break;
			default:
				break;
		}
		this.rebuildToolbarTools();
		this.emit();
	}
}
//#endregion 🔖Controller

//#region 🔖Runtime
function spatialControllerFromContext(ctx: WindowBodyViewContext): SpatialPlayShellController | undefined {
	return ctx.runtime.getActiveApp()?.controller as SpatialPlayShellController | undefined;
}

function buildSpatialPlayDeclarativeBodyForPane(pane: SpatialPlayPaneId): (ctx: WindowBodyViewContext) => UiNode {
	return (ctx) => {
		if (!spatialControllerFromContext(ctx)) {
			return { type: "text", value: "Missing spatial play controller" };
		}
		return buildScene3dWindowBody(spatialPlaySceneSurfaceIdForPane(pane), SPATIAL_PLAY_CONTROLLER_ID);
	};
}

export const buildSpatialPlayShapeDeclarativeBody = buildSpatialPlayDeclarativeBodyForPane("shape");
export const buildSpatialPlayBuildingDeclarativeBody = buildSpatialPlayDeclarativeBodyForPane("building");
export const buildSpatialPlayEnergyDeclarativeBody = buildSpatialPlayDeclarativeBodyForPane("energy");
export const buildSpatialPlayStructureClassicDeclarativeBody = buildSpatialPlayDeclarativeBodyForPane("structure-classic");

export function buildSpatialPlayAppRuntime(controller: SpatialPlayShellController): AppRuntime {
	const app = new AppRuntime(
		SPATIAL_PLAY_APP_ID,
		"Spatial play",
		undefined,
		controller,
		SPATIAL_PLAY_LAYOUT as never,
		controller.mainMode.windowKinds,
	);
	app.defaultModeId = controller.mainMode.id;
	app.addMode(controller.mainMode);
	app.leftTabs = [];
	app.rightTabs = [];
	return app;
}

/** @emoji 📝 Registers spatial play window bodies on the playground host. */
export function registerSpatialPlayDeclarativeBodies(): void {
	registerWindowBody(SPATIAL_PLAY_SHAPE_BODY_KEY, buildSpatialPlayShapeDeclarativeBody);
	registerWindowBody(SPATIAL_PLAY_BUILDING_BODY_KEY, buildSpatialPlayBuildingDeclarativeBody);
	registerWindowBody(SPATIAL_PLAY_ENERGY_BODY_KEY, buildSpatialPlayEnergyDeclarativeBody);
	registerWindowBody(SPATIAL_PLAY_STRUCTURE_CLASSIC_BODY_KEY, buildSpatialPlayStructureClassicDeclarativeBody);
}

/** @emoji 🚀 Creates spatial play {@link ProductRuntime} with declarative viewport body registered. */
export function buildSpatialPlayRuntime(): ProductRuntime {
	registerSpatialPlayDeclarativeBodies();
	const runtime = new ProductRuntime();
	const controller = new SpatialPlayShellController(runtime.commandBus, () => runtime.notify());
	runtime.addApp(buildSpatialPlayAppRuntime(controller));
	return runtime;
}
//#endregion 🔖Runtime

import "./globals.css";
// #region 🔌Adapters
import { getLevelBgClass, LevelProvider, reactHostPort, type TreeDataItem, type TreeDataSection } from "@ui/react";
import { StrictMode, type ChangeEvent, type ReactNode } from "react";
// #endregion 🔌Adapters
import {
	PlaygroundView,
	CallbackTreePanelDefinition,
	PureSidePanelTabDefinition,
	StaticTreePanelDefinition,
	mountPlaygroundApp,
	registerUiScene3DSurfaceHost,
	type SidePanelTabConfig,
	type UiScene3DHostSurfaceNode,
} from "@framework/playground/renderer/react";
import { ListTree, Shapes } from "lucide-react";
import { defaultConstructRunner } from "@cad/js/query";
import geometryNakagin from "../../../fixtures/geometry.json";
import geometryLoom from "../../../fixtures/geometry-loom.json";
import geometryRoutes from "../../../fixtures/geometry-routes.json";
import geometrySmallBuilding from "../../../fixtures/small-building.model.json";
import geometryTallBuilding from "../../../fixtures/tall-building.model.json";
import geometryLargeBuilding from "../../../fixtures/large-building.model.json";
import { BrepjsKernel } from "@cad/js/kernel/brepjs";
import { statelyStateEngineProvider } from "@cad/js/machine/stately";
import {
	InteractionRepl,
	InteractionReplViewport,
	SelectionAttributesPanel,
	SelectionPropertiesPanel,
	replDisplayedSelectionTargets,
	replWithRendererSelectionTargets,
	r3fPreviewKernel,
	useDocumentHistory,
	useInteractionRuntime,
	type SpatialInteractionSelectionByState,
	type SpatialRendererSelectionByModel,
} from "../index";

//#region 🔖GeometryCatalog
function modelVertexCount(json: Record<string, unknown>): number {
	const modelSpace = parseModelSpaceJson(json);
	if (modelSpace) return Object.values(modelSpace.models).reduce((count, model) => count + Object.keys(model.vertices).length, 0);
	const model = parseModelJson(json);
	if (model) return Object.keys(model.vertices).length;
	const geo = json.geometry;
	if (geo && typeof geo === "object") {
		const nested = (geo as Record<string, unknown>).vertices;
		if (Array.isArray(nested)) return nested.length;
	}
	const verts = json.vertices;
	return Array.isArray(verts) ? verts.length : 0;
}

const SHAPE_ASSETS = [
	{ id: "nakagin-slice", key: "a", label: "Nakagin capsule", json: geometryNakagin as Record<string, unknown> },
	{ id: "geometry-loom", key: "l", label: "Loom deck + pent loop + rail", json: geometryLoom as Record<string, unknown> },
	{ id: "geometry-routes", key: "r", label: "Multi-route lattice", json: geometryRoutes as Record<string, unknown> },
	{ id: "small-building", key: "s", label: "Small building", json: geometrySmallBuilding as Record<string, unknown> },
	{ id: "tall-building", key: "t", label: "Tall building", json: geometryTallBuilding as Record<string, unknown> },
	{ id: "large-building", key: "b", label: "Large building", json: geometryLargeBuilding as Record<string, unknown> },
] as const;

const PLAY_REPL_SPEC: InteractionSpec = {
	schema: "spatial.interaction/v1",
	id: "",
	version: "1.0.0",
	label: "Play",
	machine: {
		initial: "idle",
		states: [{ name: "idle" }],
	},
	display: {
		states: [{ state: "idle", items: [] }],
	},
	commit: {
		fromStates: [],
		operation: { kind: "action", action: "play.repl.noop" },
	},
};

type ModelJsonSnapshot = ReturnType<Model["toJSON"]>;
type ModelSpaceJsonSnapshot = ReturnType<ModelSpace["toJSON"]>;

interface SpatialExchangeBundle {
	readonly model?: ModelJsonSnapshot;
	readonly modelSpace?: ModelSpaceJsonSnapshot;
	readonly activeModelDefinitionId?: string;
}

interface SaveFilePickerTypeOption {
	readonly description?: string;
	readonly accept: Record<string, readonly string[]>;
}

interface SaveFilePickerOptionsLike {
	readonly suggestedName?: string;
	readonly types?: readonly SaveFilePickerTypeOption[];
	readonly excludeAcceptAllOption?: boolean;
}

interface FileSystemWritableFileStreamLike {
	write(data: string): Promise<void>;
	close(): Promise<void>;
}

interface FileSystemFileHandleLike {
	createWritable(): Promise<FileSystemWritableFileStreamLike>;
}

interface SavePickerWindow extends Window {
	showSaveFilePicker?: (options?: SaveFilePickerOptionsLike) => Promise<FileSystemFileHandleLike>;
}

function ensurePlayShapeModel(models: Readonly<Record<string, Model>>): Record<string, Model> {
	if (models[SHAPE_MODEL_DEFINITION_ID]) return { ...models };
	return { ...models, [SHAPE_MODEL_DEFINITION_ID]: new Model() };
}

function parseModelSpaceJson(raw: unknown): ModelSpace | null {
	if (!raw || typeof raw !== "object") return null;
	const row = raw as Record<string, unknown>;
	if (row.schema !== "spatial.modelspace/v1" || !Array.isArray(row.models)) return null;
	return ModelSpace.fromJSON(row as ModelSpaceJsonSnapshot);
}

function fileStem(name: string): string {
	const trimmed = name.trim();
	if (!trimmed) return "spatial";
	return trimmed
		.replace(/\.analytic\.spatial\.json$/i, "")
		.replace(/\.raw\.spatial\.json$/i, "")
		.replace(/\.spatial\.json$/i, "")
		.replace(/\.json$/i, "")
		.replace(/[^a-z0-9._-]+/gi, "-")
		.replace(/^-+|-+$/g, "") || "spatial";
}

function selectRawModel(model: Model, selection: readonly SelectionTarget[]): ModelJsonSnapshot {
	const selectedModel = new Model();
	selectedModel.revision = model.revision;
	const anchors = new Set<string>();
	const vertices = new Set<string>();
	const edges = new Set<string>();
	const wires = new Set<string>();
	const faces = new Set<string>();
	const shells = new Set<string>();
	const solids = new Set<string>();
	const visitById = (id: string): void => {
		if (model.anchors[id]) {
			visitAnchor(id);
			return;
		}
		if (model.vertices[id]) {
			visitVertex(id);
			return;
		}
		if (model.edges[id]) {
			visitEdge(id);
			return;
		}
		if (model.wires[id]) {
			visitWire(id);
			return;
		}
		if (model.faces[id]) {
			visitFace(id);
			return;
		}
		if (model.shells[id]) {
			visitShell(id);
			return;
		}
		if (model.solids[id]) {
			visitSolid(id);
			return;
		}
	};

	const visitAnchor = (id: string): void => {
		if (anchors.has(id)) return;
		const rec = model.anchors[id];
		if (!rec) return;
		anchors.add(id);
		visitById(rec.attachment.id);
	};

	const visitVertex = (id: string): void => {
		if (vertices.has(id) || !model.vertices[id]) return;
		vertices.add(id);
	};

	const visitEdge = (id: string): void => {
		if (edges.has(id)) return;
		const rec = model.edges[id];
		if (!rec) return;
		edges.add(id);
		for (const vertexId of rec.vertexIds) visitVertex(vertexId);
	};

	const visitWire = (id: string): void => {
		if (wires.has(id)) return;
		const rec = model.wires[id];
		if (!rec) return;
		wires.add(id);
		for (const edgeId of rec.edgeIds) visitEdge(edgeId);
	};

	const visitFace = (id: string): void => {
		if (faces.has(id)) return;
		const rec = model.faces[id];
		if (!rec) return;
		faces.add(id);
		for (const wireId of rec.wireIds) visitWire(wireId);
	};

	const visitShell = (id: string): void => {
		if (shells.has(id)) return;
		const rec = model.shells[id];
		if (!rec) return;
		shells.add(id);
		for (const faceId of rec.faceIds) visitFace(faceId);
	};

	const visitSolid = (id: string): void => {
		if (solids.has(id)) return;
		const rec = model.solids[id];
		if (!rec) return;
		solids.add(id);
		for (const shellId of rec.shellIds) visitShell(shellId);
	};

	for (const target of selection) {
		switch (target.kind) {
			case "object": {
				const object = model.objects[target.id];
				if (!object) break;
				selectedModel.objects[object.id] = object;
				for (const primitiveId of Object.values(object.primitives)) visitById(primitiveId);
				break;
			}
			case "anchor":
				visitAnchor(target.id);
				break;
			case "vertex":
				visitVertex(target.id);
				break;
			case "edge":
				visitEdge(target.id);
				break;
			case "wire":
				visitWire(target.id);
				break;
			case "face":
				visitFace(target.id);
				break;
			case "shell":
				visitShell(target.id);
				break;
			case "solid":
				visitSolid(target.id);
				break;
			default:
				break;
		}
	}

	const sortIds = (ids: Set<string>) => [...ids].sort((a, b) => a.localeCompare(b));
	selectedModel.anchors = Object.fromEntries(sortIds(anchors).map((id) => [id, model.anchors[id]!])) as typeof selectedModel.anchors;
	selectedModel.vertices = Object.fromEntries(sortIds(vertices).map((id) => [id, model.vertices[id]!])) as typeof selectedModel.vertices;
	selectedModel.edges = Object.fromEntries(sortIds(edges).map((id) => [id, model.edges[id]!])) as typeof selectedModel.edges;
	selectedModel.wires = Object.fromEntries(sortIds(wires).map((id) => [id, model.wires[id]!])) as typeof selectedModel.wires;
	selectedModel.faces = Object.fromEntries(sortIds(faces).map((id) => [id, model.faces[id]!])) as typeof selectedModel.faces;
	selectedModel.shells = Object.fromEntries(sortIds(shells).map((id) => [id, model.shells[id]!])) as typeof selectedModel.shells;
	selectedModel.solids = Object.fromEntries(sortIds(solids).map((id) => [id, model.solids[id]!])) as typeof selectedModel.solids;
	selectedModel.metadata.loadSnapshot(model.metadata.toJSON(), false);
	return selectedModel.toJSON();
}

async function writeTextFile(
	name: string,
	text: string,
	types: readonly SaveFilePickerTypeOption[],
	fallbackMime = "application/octet-stream",
): Promise<void> {
	const pickerWindow = window as SavePickerWindow;
	if (pickerWindow.showSaveFilePicker) {
		const handle = await pickerWindow.showSaveFilePicker({ suggestedName: name, types });
		const writable = await handle.createWritable();
		await writable.write(text);
		await writable.close();
		return;
	}
	const href = URL.createObjectURL(new Blob([text], { type: fallbackMime }));
	const link = document.createElement("a");
	link.href = href;
	link.download = name;
	link.click();
	URL.revokeObjectURL(href);
}

async function writeJsonFile(name: string, payload: SpatialExchangeBundle): Promise<void> {
	await writeTextFile(
		name,
		`${JSON.stringify(payload, null, 2)}\n`,
		[{ description: "Spatial JSON", accept: { "application/json": [".json", ".spatial.json"] } }],
		"application/json",
	);
}

async function writeStepFile(name: string, stepText: string): Promise<void> {
	await writeTextFile(
		name,
		stepText,
		[{ description: "STEP AP242", accept: { "application/step": [".stp", ".step"], "model/step": [".stp", ".step"] } }],
		"application/step",
	);
}

function sanitizeModelDefinitionFileStem(modelDefinitionId: string): string {
	return modelDefinitionId.replace(/[^a-z0-9._-]+/gi, "-").replace(/^-+|-+$/g, "") || "model";
}

function modelsFromSpatialJson(json: unknown): Record<string, Model> {
	const bundle = json && typeof json === "object" ? (json as SpatialExchangeBundle) : null;
	const modelSpace = parseModelSpaceJson(bundle?.modelSpace ?? json);
	if (modelSpace) return ensurePlayShapeModel(recordFromModelSpace(modelSpace));
	return ensurePlayShapeModel({
		[SHAPE_MODEL_DEFINITION_ID]: parseModelJson(bundle?.model ?? json) ?? new Model(),
	});
}

function activeModelDefinitionIdFromSpatialJson(json: unknown): string {
	const bundle = json && typeof json === "object" ? (json as SpatialExchangeBundle) : null;
	if (typeof bundle?.activeModelDefinitionId === "string") return bundle.activeModelDefinitionId;
	const modelSpace = parseModelSpaceJson(bundle?.modelSpace ?? json);
	return Object.keys(modelSpace?.models ?? {})[0] ?? SHAPE_MODEL_DEFINITION_ID;
}

function flushModelsRecord(models: Readonly<Record<string, Model>>, activeId: string, live: Model): Record<string, Model> {
	return { ...models, [activeId]: Model.fromJSON(live.toJSON()) };
}

function modelSpaceFromRecord(models: Readonly<Record<string, Model>>): ModelSpace {
	const space = new ModelSpace();
	for (const id of Object.keys(models).sort()) space.link(id, models[id]!);
	return space;
}

function recordFromModelSpace(space: ModelSpace): Record<string, Model> {
	const out: Record<string, Model> = {};
	for (const id of Object.keys(space.models).sort()) {
		const model = space.models[id];
		if (model) out[id] = Model.fromJSON(model.toJSON());
	}
	return out;
}

function ensureDerivedModelInSpace(models: Readonly<Record<string, Model>>, definitionId: string): Record<string, Model> {
	const withShape = ensurePlayShapeModel(models);
	if (withShape[definitionId]) return withShape;
	if (isShapeModelDefinition(definitionId)) return withShape;
	const candidates = listTransformationsIntoModelDefinition(definitionId);
	const fromShape = candidates.find((row) => isShapeModelDefinition(row.source.modelDefinition));
	const shape = withShape[SHAPE_MODEL_DEFINITION_ID];
	if (fromShape && shape) {
		return { ...withShape, [definitionId]: applyTransformation(fromShape, shape) };
	}
	const fromLinked = candidates.find((row) => withShape[row.source.modelDefinition]);
	if (fromLinked) {
		return { ...withShape, [definitionId]: applyTransformation(fromLinked, withShape[fromLinked.source.modelDefinition]!) };
	}
	return withShape;
}

/** @emoji 🌌 Ensures all four spatial play quad models exist and stay derived from shape. */
export function ensureSpatialPlayQuadModels(models: Readonly<Record<string, Model>>): Record<string, Model> {
	let next = ensurePlayShapeModel(models);
	if (!next[SPATIAL_PLAY_BUILDING_MODEL_DEFINITION_ID]) {
		next = { ...next, [SPATIAL_PLAY_BUILDING_MODEL_DEFINITION_ID]: new Model() };
	}
	next = ensureDerivedModelInSpace(next, SPATIAL_PLAY_ENERGY_MODEL_DEFINITION_ID);
	next = ensureDerivedModelInSpace(next, "aec.building.structure");
	next = ensureDerivedModelInSpace(next, SPATIAL_PLAY_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID);
	return next;
}

function emptyPlayModels(): Record<string, Model> {
	return ensureSpatialPlayQuadModels({});
}

function pickShapeForModelDefinition(
	models: Readonly<Record<string, Model>>,
	activeModelDefinitionId: string,
	liveModel: Model,
): Model {
	if (isShapeModelDefinition(activeModelDefinitionId)) {
		return models[SHAPE_MODEL_DEFINITION_ID] ?? liveModel;
	}
	if (modelDefinitionUsesGeometryPicking(activeModelDefinitionId)) {
		return models[activeModelDefinitionId] ?? models[SHAPE_MODEL_DEFINITION_ID] ?? liveModel;
	}
	return liveModel;
}

//#region 🔖SpatialPlayChrome
export interface SpatialPlayChromeSnapshot {
	readonly modelsByDefinitionId: Record<string, Model>;
	readonly activeModelDefinitionId: string;
	readonly selection: readonly SelectionTarget[];
	readonly selectTarget: (modelDefinitionId: string, target: SelectionTarget) => void;
}

interface SpatialPlayChromeContextValue {
	readonly snapshot: SpatialPlayChromeSnapshot | null;
	readonly publishSnapshot: (snapshot: SpatialPlayChromeSnapshot | null) => void;
}

const SpatialPlayChromeContext = reactHostPort.createContext<SpatialPlayChromeContextValue | null>(null);

function useSpatialPlayChrome(): SpatialPlayChromeContextValue {
	const value = reactHostPort.useContext(SpatialPlayChromeContext);
	if (!value) {
		throw new Error("useSpatialPlayChrome must be used inside SpatialPlayChromeContext.");
	}
	return value;
}

function useSpatialPlayChromePublish(): (snapshot: SpatialPlayChromeSnapshot | null) => void {
	return useSpatialPlayChrome().publishSnapshot;
}

class SpatialPlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
	constructor(private readonly buildSections: () => TreeDataSection[]) {
		super();
	}

	resolveTab(): SidePanelTabConfig {
		return {
			id: SPATIAL_PLAY_HIERARCHY_TAB_ID,
			icon: ListTree,
			order: 0,
			tree: new CallbackTreePanelDefinition(() => this.buildSections()),
		};
	}
}
//#endregion 🔖SpatialPlayChrome

//#region 🔖PlaySession
interface PlaySessionProps {
	readonly interactionId: string;
	readonly spec: InteractionSpec;
	readonly onInteractionId: (id: string) => void;
	readonly documentModel: ModelDocument;
	readonly history: DocumentHistory;
	readonly kernel: InteractionRuntimeOptions["kernel"];
	readonly mode: SpatialComputeMode;
	readonly asideExtra: ReactNode;
	readonly sessionRestartNonce: number;
	readonly activeModelDefinitionId: string;
	readonly onActiveModelDefinitionId: (value: string) => void;
	readonly rendererSelectionByModel: SpatialRendererSelectionByModel;
	readonly onRendererSelectionByModel: (value: SpatialRendererSelectionByModel) => void;
	readonly interactionSelectionByState: SpatialInteractionSelectionByState;
	readonly onInteractionSelectionByState: (value: SpatialInteractionSelectionByState) => void;
	readonly modelDefinitionRevision: number;
	readonly onModelDefinitionRevision: (revision: number) => void;
	readonly onApplyTransformation: (spec: TransformationSpec) => void;
	readonly pickGeometry: Model;
	readonly onDocumentModelChange: (model: Model) => void;
	readonly onSnapshot: (snapshot: InteractionSnapshot) => void;
}

/** @emoji 🎮 Hosts `useInteractionRuntime` + `InteractionRepl`; same-interaction restarts use `sessionRestartNonce` without remounting GL. */
function PlaySession({
	interactionId,
	spec,
	onInteractionId,
	documentModel,
	history,
	kernel,
	mode,
	asideExtra,
	sessionRestartNonce,
	activeModelDefinitionId,
	onActiveModelDefinitionId,
	rendererSelectionByModel,
	onRendererSelectionByModel,
	interactionSelectionByState,
	onInteractionSelectionByState,
	modelDefinitionRevision,
	onModelDefinitionRevision,
	onApplyTransformation,
	pickGeometry,
	onDocumentModelChange,
	onSnapshot,
}: PlaySessionProps) {
	const rtOpts = reactHostPort.useMemo(
		(): InteractionRuntimeOptions => ({
			kernel,
			previewKernel: r3fPreviewKernel,
			mode,
			document: documentModel,
			history,
			stateEngine: statelyStateEngineProvider,
			query: defaultConstructRunner,
			activeModelDefinitionId,
		}),
		[kernel, mode, documentModel, history, activeModelDefinitionId],
	);
	const rt = useInteractionRuntime(spec, rtOpts);
	reactHostPort.useEffect(() => {
		return rt.subscribe(() => {
			const snap = rt.getSnapshot();
			onSnapshot(snap);
			const res = snap.lastResponse;
			if (res?.ok && res.diff && !isEmptyModelDiff(res.diff)) {
				onDocumentModelChange(Model.fromJSON(documentModel.model.toJSON()));
				onModelDefinitionRevision((revision) => revision + 1);
			}
		});
	}, [rt, documentModel, onSnapshot, onDocumentModelChange, onModelDefinitionRevision]);
	return (
		<InteractionRepl
			fillHost
			showAside={false}
			interactionId={interactionId}
			spec={spec}
			onInteractionId={onInteractionId}
			runtime={rt}
			history={history}
			document={documentModel}
			geometry={documentModel.model}
			pickGeometry={pickGeometry}
			onDocumentModelChange={onDocumentModelChange}
			asideExtra={asideExtra}
			sessionRestartNonce={sessionRestartNonce}
			activeModelDefinitionId={activeModelDefinitionId}
			onActiveModelDefinitionIdChange={onActiveModelDefinitionId}
			rendererSelectionByModel={rendererSelectionByModel}
			onRendererSelectionByModelChange={onRendererSelectionByModel}
			interactionSelectionByState={interactionSelectionByState}
			onInteractionSelectionByStateChange={onInteractionSelectionByState}
			modelDefinitionRevision={modelDefinitionRevision}
			onModelDefinitionRevisionChange={onModelDefinitionRevision}
			onApplyTransformation={onApplyTransformation}
			hideModelDefinitionControls
			onSnapshotChange={onSnapshot}
		/>
	);
}
//#endregion

//#region 🔖SpatialPlayModelSpace
interface SpatialPlayModelSpaceValue {
	readonly activeModelDefinitionId: string;
	readonly setActiveModelDefinitionId: (value: string) => void;
	readonly focusModelDefinition: (modelDefinitionId: string) => void;
	readonly interactionId: string;
	readonly handleInteractionPick: (id: string) => void;
	readonly spec: InteractionSpec;
	readonly documentModel: ModelDocument;
	readonly history: DocumentHistory;
	readonly kernel: InteractionRuntimeOptions["kernel"];
	readonly computeModeForPane: (pane: SpatialPlayPaneId) => SpatialComputeMode;
	readonly sessionRestartNonce: number;
	readonly rendererSelectionByModel: SpatialRendererSelectionByModel;
	readonly setRendererSelectionByModel: (value: SpatialRendererSelectionByModel) => void;
	readonly interactionSelectionByState: SpatialInteractionSelectionByState;
	readonly setInteractionSelectionByState: (value: SpatialInteractionSelectionByState) => void;
	readonly modelDefinitionRevision: number;
	readonly setModelDefinitionRevision: (value: number | ((revision: number) => number)) => void;
	readonly handleApplyTransformation: (spec: TransformationSpec) => void;
	readonly pickGeometry: Model;
	readonly handleModelAttributesChange: (model: Model) => void;
	readonly handleSnapshotChange: (snapshot: InteractionSnapshot) => void;
	readonly flushedModelsByDefinitionId: Record<string, Model>;
	readonly playModelSpace: ModelSpace;
	readonly viewObjectCount: number;
	readonly selectionInScope: readonly SelectionTarget[];
	readonly shapeAssetId: string;
	readonly handleShapeAssetChange: (id: string) => void;
	readonly fileStatus: string;
	readonly loadInputRef: React.RefObject<HTMLInputElement | null>;
	readonly exportBaseName: string;
	readonly handleSaveSelected: () => Promise<void>;
	readonly handleSaveInPlay: () => Promise<void>;
	readonly handleSaveCurrent: () => Promise<void>;
	readonly handleLoadRawRequest: () => void;
	readonly handleLoadRaw: (event: ChangeEvent<HTMLInputElement>) => Promise<void>;
	readonly liveModel: Model;
	readonly brepjsKernel: BrepjsKernel;
}

const SpatialPlayModelSpaceContext = reactHostPort.createContext<SpatialPlayModelSpaceValue | null>(null);

function useSpatialPlayModelSpace(): SpatialPlayModelSpaceValue {
	const value = reactHostPort.useContext(SpatialPlayModelSpaceContext);
	if (!value) {
		throw new Error("useSpatialPlayModelSpace must be used inside SpatialPlayModelSpaceProvider.");
	}
	return value;
}

function SpatialPlayModelSpaceProvider({
	children,
	runtime,
	shellController,
}: {
	readonly children: ReactNode;
	readonly runtime: ProductRuntime;
	readonly shellController: SpatialPlayShellController;
}) {
	const shellGeneration = reactHostPort.useSyncExternalStore(
		(onStoreChange) => runtime.subscribe(onStoreChange),
		() => runtime.generation,
		() => 0,
	);
	void shellGeneration;
	const computeModeForPane = reactHostPort.useCallback(
		(pane: SpatialPlayPaneId) => shellController.getComputeModeForPane(pane),
		[shellController, shellGeneration],
	);
	const publishSpatialPlayChrome = useSpatialPlayChromePublish();
	const [activeModelDefinitionId, setActiveModelDefinitionId] = reactHostPort.useState(SHAPE_MODEL_DEFINITION_ID);
	const scopedInteractions = reactHostPort.useMemo(
		() => listSpatialInteractionsForModelDefinition(activeModelDefinitionId),
		[activeModelDefinitionId],
	);
	const [interactionId, setInteractionId] = reactHostPort.useState("");
	const [interactionBootId, setInteractionBootId] = reactHostPort.useState(0);
	const [shapeAssetId, setShapeAssetId] = reactHostPort.useState("");
	const [modelsByDefinitionId, setModelsByDefinitionId] = useState<Record<string, Model>>(emptyPlayModels);
	const [loadedRawName, setLoadedRawName] = reactHostPort.useState("");
	const [rendererSelectionByModel, setRendererSelectionByModel] = useState<SpatialRendererSelectionByModel>({});
	const [interactionSelectionByState, setInteractionSelectionByState] = useState<SpatialInteractionSelectionByState>({});
	const [modelDefinitionRevision, setModelDefinitionRevision] = reactHostPort.useState(0);
	const [snapshot, setSnapshot] = useState<InteractionSnapshot | null>(null);
	const [fileStatus, setFileStatus] = useState<string>("");
	const loadInputRef = useRef<HTMLInputElement>(null);
	const spec = useMemo<InteractionSpec | null>(() => (interactionId ? loadSpatialInteraction(interactionId) : PLAY_REPL_SPEC), [interactionId]);
	const history = useDocumentHistory();
	const brepjsKernel = reactHostPort.useMemo(() => new BrepjsKernel(), []);
	const kernel = useMemo<InteractionRuntimeOptions["kernel"]>(
		() => brepjsKernel as unknown as InteractionRuntimeOptions["kernel"],
		[brepjsKernel],
	);

	reactHostPort.useEffect(() => {
		if (!interactionId) return;
		if (!scopedInteractions.some((row) => row.id === interactionId)) setInteractionId("");
	}, [activeModelDefinitionId, interactionId, scopedInteractions]);

	reactHostPort.useEffect(() => {
		setModelsByDefinitionId((prev) => ensureSpatialPlayQuadModels(prev));
	}, [activeModelDefinitionId]);

	const handleInteractionPick = reactHostPort.useCallback(
		(id: string) => {
			if (id === interactionId) setInteractionBootId((b) => b + 1);
			else {
				setInteractionId(id);
				setInteractionBootId(0);
			}
		},
		[interactionId],
	);

	const handleShapeAssetChange = reactHostPort.useCallback((id: string) => {
		setShapeAssetId(id);
		setLoadedRawName("");
		setFileStatus("");
		if (!id) {
			setModelsByDefinitionId(emptyPlayModels());
			setActiveModelDefinitionId(SHAPE_MODEL_DEFINITION_ID);
		} else {
			const asset = SHAPE_ASSETS.find((candidate) => candidate.id === id);
			if (!asset) return;
			setModelsByDefinitionId(modelsFromSpatialJson(asset.json));
			setActiveModelDefinitionId(activeModelDefinitionIdFromSpatialJson(asset.json));
		}
		setModelDefinitionRevision((r) => r + 1);
	}, []);

	const modelsForActiveDefinition = reactHostPort.useMemo(
		() => ensureSpatialPlayQuadModels(modelsByDefinitionId),
		[activeModelDefinitionId, modelsByDefinitionId],
	);

	const activeModel = reactHostPort.useMemo(() => {
		const resolved = modelsForActiveDefinition[activeModelDefinitionId];
		if (resolved) return resolved;
		if (isShapeModelDefinition(activeModelDefinitionId)) {
			return modelsForActiveDefinition[SHAPE_MODEL_DEFINITION_ID] ?? new Model();
		}
		throw new Error(`Play model space missing model for ${activeModelDefinitionId}.`);
	}, [activeModelDefinitionId, modelsForActiveDefinition]);

	const documentModel = reactHostPort.useMemo((): ModelDocument => {
		const model = Model.fromJSON(activeModel.toJSON());
		return { model: model, nodes: [] };
	}, [activeModel, modelDefinitionRevision]);
	const liveModel = documentModel.model;

	const flushedModelsByDefinitionId = reactHostPort.useMemo(() => {
		const flushed = flushModelsRecord(modelsByDefinitionId, activeModelDefinitionId, liveModel);
		return ensureSpatialPlayQuadModels(flushed);
	}, [activeModelDefinitionId, liveModel, liveModel.revision, modelsByDefinitionId]);

	const playModelSpace = reactHostPort.useMemo(
		() => modelSpaceFromRecord(flushedModelsByDefinitionId),
		[flushedModelsByDefinitionId],
	);

	const visibleExportModel = reactHostPort.useMemo(
		() => flushedModelsByDefinitionId[activeModelDefinitionId] ?? liveModel,
		[activeModelDefinitionId, flushedModelsByDefinitionId, liveModel],
	);

	const pickGeometry = reactHostPort.useMemo(
		() => pickShapeForModelDefinition(flushedModelsByDefinitionId, activeModelDefinitionId, liveModel),
		[activeModelDefinitionId, flushedModelsByDefinitionId, liveModel],
	);

	const handleActiveModelDefinitionChange = reactHostPort.useCallback(
		(nextId: string) => {
			setModelsByDefinitionId((prev) => {
				const flushed = flushModelsRecord(prev, activeModelDefinitionId, liveModel);
				return ensureSpatialPlayQuadModels(flushed);
			});
			setActiveModelDefinitionId(nextId);
			setModelDefinitionRevision((r) => r + 1);
		},
		[activeModelDefinitionId, liveModel],
	);

	const focusModelDefinition = reactHostPort.useCallback(
		(modelDefinitionId: string) => {
			if (modelDefinitionId !== activeModelDefinitionId) {
				handleActiveModelDefinitionChange(modelDefinitionId);
			}
		},
		[activeModelDefinitionId, handleActiveModelDefinitionChange],
	);

	const handleModelAttributesChange = reactHostPort.useCallback(
		(model: Model) => {
			setModelsByDefinitionId((prev) =>
				ensureSpatialPlayQuadModels({ ...prev, [activeModelDefinitionId]: Model.fromJSON(model.toJSON()) }),
			);
			setModelDefinitionRevision((r) => r + 1);
		},
		[activeModelDefinitionId],
	);

	const interactionActive = reactHostPort.useMemo(
		() => Boolean(snapshot) && isInteractionSessionActive(spec ?? PLAY_REPL_SPEC, snapshot?.state ?? "idle"),
		[spec, snapshot],
	);
	const boundInteractionSession = Boolean(interactionId) && interactionActive;
	const handleSnapshotChange = reactHostPort.useCallback((next: InteractionSnapshot) => {
		setSnapshot((prev) => {
			if (prev && prev.revision === next.revision && prev.state === next.state) return prev;
			return next;
		});
	}, []);
	const currentSelection = reactHostPort.useMemo(
		() =>
			replDisplayedSelectionTargets(
				boundInteractionSession,
				activeModelDefinitionId,
				snapshot?.state ?? "idle",
				rendererSelectionByModel,
				interactionSelectionByState,
			),
		[
			boundInteractionSession,
			activeModelDefinitionId,
			snapshot?.state,
			rendererSelectionByModel,
			interactionSelectionByState,
		],
	);
	const selectionKinds = reactHostPort.useMemo(
		() => new Set(modelDefinitionSelectionEntityKinds(activeModelDefinitionId)),
		[activeModelDefinitionId],
	);
	const viewObjectCount = reactHostPort.useMemo(
		() => countViewObjectsForModelDefinition(liveModel, activeModelDefinitionId),
		[liveModel, activeModelDefinitionId, modelDefinitionRevision],
	);

	const selectionInScope = reactHostPort.useMemo(
		() =>
			currentSelection.filter((target) => {
				if (target.kind === "object" && target.editable === false) return selectionKinds.has("object");
				return selectionKinds.has(target.kind);
			}),
		[currentSelection, selectionKinds],
	);

	const selectHierarchyTarget = reactHostPort.useCallback(
		(modelDefinitionId: string, target: SelectionTarget) => {
			if (modelDefinitionId !== activeModelDefinitionId) {
				handleActiveModelDefinitionChange(modelDefinitionId);
			}
			setRendererSelectionByModel((prev) => replWithRendererSelectionTargets(prev, modelDefinitionId, [target]));
		},
		[activeModelDefinitionId, handleActiveModelDefinitionChange],
	);

	const flushedModelsDigest = reactHostPort.useMemo(
		() => spatialPlayModelsDigest(flushedModelsByDefinitionId),
		[flushedModelsByDefinitionId, liveModel.revision, modelDefinitionRevision],
	);

	reactHostPort.useEffect(() => {
		publishSpatialPlayChrome({
			modelsByDefinitionId: flushedModelsByDefinitionId,
			activeModelDefinitionId,
			selection: selectionInScope,
			selectTarget: selectHierarchyTarget,
		});
		return () => publishSpatialPlayChrome(null);
	}, [
		activeModelDefinitionId,
		flushedModelsByDefinitionId,
		flushedModelsDigest,
		modelDefinitionRevision,
		publishSpatialPlayChrome,
		selectHierarchyTarget,
		selectionInScope,
	]);

	const exportBaseName = reactHostPort.useMemo(() => {
		if (loadedRawName) return fileStem(loadedRawName);
		const asset = SHAPE_ASSETS.find((g) => g.id === shapeAssetId);
		return fileStem(asset?.id ?? "spatial");
	}, [shapeAssetId, loadedRawName]);

	const handleApplyTransformation = reactHostPort.useCallback(
		(spec: TransformationSpec) => {
			const space = modelSpaceFromRecord(flushModelsRecord(modelsByDefinitionId, activeModelDefinitionId, liveModel));
			try {
				space.transform(spec.source.modelDefinition, spec.target.modelDefinition, spec);
			} catch (error) {
				setFileStatus(`Transform failed: ${String(error)}`);
				return;
			}
			setModelsByDefinitionId(ensureSpatialPlayQuadModels(recordFromModelSpace(space)));
			setActiveModelDefinitionId(spec.target.modelDefinition);
			setModelDefinitionRevision((r) => r + 1);
			setFileStatus(`Transformed ${spec.source.modelDefinition} → ${spec.target.modelDefinition}.`);
		},
		[activeModelDefinitionId, liveModel, modelsByDefinitionId],
	);

	reactHostPort.useEffect(() => {
		history.clear();
		setSnapshot(null);
	}, [history, modelDefinitionRevision]);

	const saveBundle = reactHostPort.useCallback(
		async (name: string, payload: SpatialExchangeBundle, message: string) => {
			try {
				await writeJsonFile(name, payload);
				setFileStatus(message);
			} catch (error) {
				setFileStatus(`Save failed: ${String(error)}`);
			}
		},
		[],
	);

	const handleSaveSelected = reactHostPort.useCallback(async () => {
		const selectedModel = Model.fromJSON(selectRawModel(liveModel, selectionInScope));
		const selectedModelSpace = new ModelSpace();
		selectedModelSpace.link(activeModelDefinitionId, selectedModel);
		await saveBundle(
			`${exportBaseName}.selected.spatial.json`,
			{ model: selectedModel.toJSON(), modelSpace: selectedModelSpace.toJSON(), activeModelDefinitionId },
			`Saved ${selectionInScope.length} selected item(s) for ${activeModelDefinitionId}.`,
		);
	}, [activeModelDefinitionId, exportBaseName, liveModel, saveBundle, selectionInScope]);

	const handleSaveInPlay = reactHostPort.useCallback(async () => {
		try {
			const stepText = await brepjsKernel.exportModelSpaceToStep(playModelSpace, exportBaseName);
			await writeStepFile(`${exportBaseName}.modelspace.stp`, stepText);
			setFileStatus(`Saved model space (${Object.keys(playModelSpace.models).length} model(s)) to STEP.`);
		} catch (error) {
			setFileStatus(`Save failed: ${String(error)}`);
		}
	}, [brepjsKernel, exportBaseName, playModelSpace]);

	const handleSaveCurrent = reactHostPort.useCallback(async () => {
		try {
			const modelId = activeModelDefinitionId;
			const stepText = await brepjsKernel.exportModelToStep(visibleExportModel, modelId);
			const stem = sanitizeModelDefinitionFileStem(modelId);
			await writeStepFile(`${exportBaseName}.${stem}.stp`, stepText);
			setFileStatus(`Saved ${modelId} to STEP.`);
		} catch (error) {
			setFileStatus(`Save failed: ${String(error)}`);
		}
	}, [activeModelDefinitionId, brepjsKernel, exportBaseName, visibleExportModel]);

	const handleLoadRawRequest = reactHostPort.useCallback(() => {
		loadInputRef.current?.click();
	}, []);

	reactHostPort.useEffect(() => {
		const bridge = {
			getToolbarState: () => ({
				activeModelDefinitionId,
				selectionCount: selectionInScope.length,
				transformsTo: listTransformationsFromModelDefinition(activeModelDefinitionId),
				transformsFrom: listTransformationsIntoModelDefinition(activeModelDefinitionId),
			}),
			runHostCommand: (command: string, args?: unknown) => {
				switch (command) {
					case "focusModelDefinition": {
						const modelDefinitionId = (args as { modelDefinitionId?: string })?.modelDefinitionId;
						if (modelDefinitionId) focusModelDefinition(modelDefinitionId);
						break;
					}
					case "applyTransformation": {
						const qid = (args as { qid?: string })?.qid;
						if (!qid) break;
						const spec =
							listTransformationsFromModelDefinition(activeModelDefinitionId).find(
								(row) => qualifiedTransformationId(row.modelDefinitionId, row.id) === qid,
							) ??
							listTransformationsIntoModelDefinition(activeModelDefinitionId).find(
								(row) => qualifiedTransformationId(row.modelDefinitionId, row.id) === qid,
							);
						if (spec) void handleApplyTransformation(spec);
						break;
					}
					case "saveSelected":
						void handleSaveSelected();
						break;
					case "saveInPlay":
						void handleSaveInPlay();
						break;
					case "saveCurrent":
						void handleSaveCurrent();
						break;
					case "loadRawRequest":
						handleLoadRawRequest();
						break;
					default:
						break;
				}
			},
		};
		shellController.setHostBridge(bridge);
		return () => shellController.setHostBridge(null);
	}, [
		activeModelDefinitionId,
		focusModelDefinition,
		handleApplyTransformation,
		handleLoadRawRequest,
		handleSaveCurrent,
		handleSaveInPlay,
		handleSaveSelected,
		selectionInScope,
		shellController,
	]);

	const handleLoadRaw = reactHostPort.useCallback(async (event: ChangeEvent<HTMLInputElement>) => {
		const file = event.target.files?.[0];
		if (!file) return;
		try {
			const parsed = JSON.parse(await file.text()) as unknown;
			const envelope = parsed as Record<string, unknown>;
			const snapshot =
				envelope && typeof envelope === "object" && "modelSpace" in envelope
					? envelope.modelSpace
					: envelope && typeof envelope === "object" && "model" in envelope
					? envelope.model
					: envelope && typeof envelope === "object" && "raw" in envelope
						? envelope.raw
						: parsed;
			const modelSpace = parseModelSpaceJson(snapshot);
			if (modelSpace) {
				const nextActiveModelDefinitionId =
					typeof envelope.activeModelDefinitionId === "string" && modelSpace.get(envelope.activeModelDefinitionId)
						? envelope.activeModelDefinitionId
						: activeModelDefinitionIdFromSpatialJson(snapshot);
				setShapeAssetId("");
				setLoadedRawName(file.name);
				setModelsByDefinitionId(recordFromModelSpace(modelSpace));
				setActiveModelDefinitionId(nextActiveModelDefinitionId);
				setModelDefinitionRevision((r) => r + 1);
				setFileStatus(`Loaded model space from ${file.name}.`);
				return;
			}
			const model = parseModelJson(snapshot);
			if (!model) throw new Error("No spatial model found in file.");
			setShapeAssetId("");
			setLoadedRawName(file.name);
			setModelsByDefinitionId(modelsFromSpatialJson(model.toJSON()));
			setActiveModelDefinitionId(SHAPE_MODEL_DEFINITION_ID);
			setModelDefinitionRevision((r) => r + 1);
			setFileStatus(`Loaded model from ${file.name}.`);
		} catch (error) {
			setFileStatus(`Load failed: ${String(error)}`);
		} finally {
			event.target.value = "";
		}
	}, []);

	const modelSpaceValue = useMemo<SpatialPlayModelSpaceValue>(
		() => ({
			activeModelDefinitionId,
			setActiveModelDefinitionId,
			focusModelDefinition,
			interactionId,
			handleInteractionPick,
			spec: spec ?? PLAY_REPL_SPEC,
			documentModel,
			history,
			kernel,
			computeModeForPane,
			sessionRestartNonce: interactionBootId,
			rendererSelectionByModel,
			setRendererSelectionByModel,
			interactionSelectionByState,
			setInteractionSelectionByState,
			modelDefinitionRevision,
			setModelDefinitionRevision,
			handleApplyTransformation,
			pickGeometry,
			handleModelAttributesChange,
			handleSnapshotChange,
			flushedModelsByDefinitionId,
			playModelSpace,
			viewObjectCount,
			selectionInScope,
			shapeAssetId,
			handleShapeAssetChange,
			fileStatus,
			loadInputRef,
			exportBaseName,
			handleSaveSelected,
			handleSaveInPlay,
			handleSaveCurrent,
			handleLoadRawRequest,
			handleLoadRaw,
			liveModel,
			brepjsKernel,
		}),
		[
			activeModelDefinitionId,
			documentModel,
			exportBaseName,
			fileStatus,
			flushedModelsByDefinitionId,
			focusModelDefinition,
			handleApplyTransformation,
			handleInteractionPick,
			handleLoadRaw,
			handleLoadRawRequest,
			handleModelAttributesChange,
			handleSaveCurrent,
			handleSaveInPlay,
			handleSaveSelected,
			handleShapeAssetChange,
			handleSnapshotChange,
			history,
			interactionBootId,
			interactionId,
			interactionSelectionByState,
			kernel,
			liveModel,
			computeModeForPane,
			modelDefinitionRevision,
			pickGeometry,
			playModelSpace,
			rendererSelectionByModel,
			selectionInScope,
			shapeAssetId,
			spec,
			viewObjectCount,
			brepjsKernel,
		],
	);

	return <SpatialPlayModelSpaceContext.Provider value={modelSpaceValue}>{children}</SpatialPlayModelSpaceContext.Provider>;
}

/** @emoji 📂 Hidden file input for playground Save → Load. */
function SpatialPlayLoadInput(): ReactNode {
	const { loadInputRef, handleLoadRaw } = useSpatialPlayModelSpace();
	return <input ref={loadInputRef} type="file" accept=".json,.spatial.json" hidden onChange={(event) => void handleLoadRaw(event)} />;
}

/** @emoji 🎯 Details panel: attribute and property editors for the current selection only. */
function SpatialPlayDetailsAside(): ReactNode {
	const { activeModelDefinitionId, liveModel, selectionInScope, handleModelAttributesChange, brepjsKernel } =
		useSpatialPlayModelSpace();
	if (selectionInScope.length === 0) {
		return (
			<div style={{ fontSize: 12, opacity: 0.75, lineHeight: 1.4 }}>
				Select a primitive or object in the canvas or workbench hierarchy to edit attributes and properties.
			</div>
		);
	}
	return (
		<>
			<SelectionAttributesPanel
				model={liveModel}
				activeModelDefinitionId={activeModelDefinitionId}
				selection={selectionInScope}
				selectionCount={selectionInScope.length}
				onModelChange={handleModelAttributesChange}
			/>
			<SelectionPropertiesPanel
				model={liveModel}
				kernel={brepjsKernel}
				activeModelDefinitionId={activeModelDefinitionId}
				selection={selectionInScope}
				selectionCount={selectionInScope.length}
			/>
		</>
	);
}

/** @emoji 📦 Workbench catalog: shape fixtures and file I/O status (toolbar handles save/load). */
function SpatialPlayCatalogAside(): ReactNode {
	const { activeModelDefinitionId, shapeAssetId, handleShapeAssetChange, fileStatus } = useSpatialPlayModelSpace();
	return (
		<>
			{isShapeModelDefinition(activeModelDefinitionId) ? (
				<label style={{ display: "flex", flexDirection: "column", gap: 4, fontSize: 12 }}>
					<span style={{ fontWeight: 600, color: "#c8c8e0" }}>Shape asset</span>
					<select
						value={shapeAssetId}
						onChange={(e) => handleShapeAssetChange(e.target.value)}
						style={{ padding: 6, borderRadius: 6, background: "#1a1a28", color: "#e8e8f0" }}
					>
						<option value="">No asset</option>
						{SHAPE_ASSETS.map((g) => (
							<option key={g.id} value={g.id}>
								[{g.key}] {g.label} ({modelVertexCount(g.json)} verts)
							</option>
						))}
					</select>
				</label>
			) : (
				<span style={{ fontSize: 12, opacity: 0.75, lineHeight: 1.4 }}>
					Shape assets apply to <code style={{ color: "#e8e8f0" }}>spatial.shape</code>. Focus the Shape pane to load source geometry.
				</span>
			)}
			{fileStatus ? (
				<span style={{ fontSize: 12, color: fileStatus.startsWith("Load failed") || fileStatus.startsWith("Save failed") ? "#ff9a9a" : "#a8d8a8" }}>
					{fileStatus}
				</span>
			) : null}
		</>
	);
}

/** @emoji 🎮 Shape pane: interaction editing on spatial.shape. */
function SpatialPlayShapePane(): ReactNode {
	const {
		interactionId,
		spec,
		handleInteractionPick,
		documentModel,
		history,
		kernel,
		computeModeForPane,
		sessionRestartNonce,
		activeModelDefinitionId,
		focusModelDefinition,
		rendererSelectionByModel,
		setRendererSelectionByModel,
		interactionSelectionByState,
		setInteractionSelectionByState,
		modelDefinitionRevision,
		setModelDefinitionRevision,
		handleApplyTransformation,
		pickGeometry,
		handleModelAttributesChange,
		handleSnapshotChange,
	} = useSpatialPlayModelSpace();

	if (!spec) {
		return (
			<div style={{ padding: 16, color: "#f88" }}>
				Unknown interaction <code>{interactionId}</code>.
				<button type="button" onClick={() => handleInteractionPick("")}>
					Reset
				</button>
			</div>
		);
	}

	const mode = computeModeForPane("shape");

	return (
		<div className="absolute inset-0 min-h-0 min-w-0" onPointerDown={() => focusModelDefinition(SHAPE_MODEL_DEFINITION_ID)}>
			<PlaySession
				interactionId={interactionId}
				spec={spec}
				onInteractionId={handleInteractionPick}
				documentModel={documentModel}
				history={history}
				kernel={kernel}
				mode={mode}
				asideExtra={null}
				sessionRestartNonce={sessionRestartNonce}
				activeModelDefinitionId={activeModelDefinitionId}
				onActiveModelDefinitionId={focusModelDefinition}
				rendererSelectionByModel={rendererSelectionByModel}
				onRendererSelectionByModel={setRendererSelectionByModel}
				interactionSelectionByState={interactionSelectionByState}
				onInteractionSelectionByState={setInteractionSelectionByState}
				modelDefinitionRevision={modelDefinitionRevision}
				onModelDefinitionRevision={setModelDefinitionRevision}
				onApplyTransformation={handleApplyTransformation}
				pickGeometry={pickGeometry}
				onDocumentModelChange={handleModelAttributesChange}
				onSnapshot={handleSnapshotChange}
			/>
		</div>
	);
}

/** @emoji 👁️ Derived model pane: read-only viewport for one model definition in the quad. */
function SpatialPlayViewPane({ pane }: { readonly pane: SpatialPlayPaneId }): ReactNode {
	const modelDefinitionId = spatialPlayModelDefinitionIdForPane(pane);
	const {
		focusModelDefinition,
		flushedModelsByDefinitionId,
		modelDefinitionRevision,
		rendererSelectionByModel,
		setRendererSelectionByModel,
		interactionSelectionByState,
		setInteractionSelectionByState,
		kernel,
		computeModeForPane,
		handleSnapshotChange,
	} = useSpatialPlayModelSpace();
	const mode = computeModeForPane(pane);
	const paneModel = flushedModelsByDefinitionId[modelDefinitionId] ?? new Model();
	const documentModel = reactHostPort.useMemo(
		(): ModelDocument => ({ model: Model.fromJSON(paneModel.toJSON()), nodes: [] }),
		[paneModel, modelDefinitionRevision],
	);
	const pickGeometry = reactHostPort.useMemo(
		() => pickShapeForModelDefinition(flushedModelsByDefinitionId, modelDefinitionId, paneModel),
		[flushedModelsByDefinitionId, modelDefinitionId, paneModel, modelDefinitionRevision],
	);
	const history = useDocumentHistory();
	const rtOpts = reactHostPort.useMemo(
		(): InteractionRuntimeOptions => ({
			kernel,
			previewKernel: r3fPreviewKernel,
			mode,
			document: documentModel,
			history,
			stateEngine: statelyStateEngineProvider,
			query: defaultConstructRunner,
			activeModelDefinitionId: modelDefinitionId,
		}),
		[kernel, mode, documentModel, history, modelDefinitionId],
	);
	const rt = useInteractionRuntime(PLAY_REPL_SPEC, rtOpts);
	reactHostPort.useEffect(() => {
		return rt.subscribe(() => {
			handleSnapshotChange(rt.getSnapshot());
		});
	}, [rt, handleSnapshotChange]);

	return (
		<div className="absolute inset-0 min-h-0 min-w-0" onPointerDown={() => focusModelDefinition(modelDefinitionId)}>
			<InteractionReplViewport
				interactionId=""
				spec={PLAY_REPL_SPEC}
				onInteractionId={() => {}}
				runtime={rt}
				history={history}
				document={documentModel}
				geometry={paneModel}
				pickGeometry={pickGeometry}
				hideModelDefinitionControls
				activeModelDefinitionId={modelDefinitionId}
				onActiveModelDefinitionIdChange={focusModelDefinition}
				rendererSelectionByModel={rendererSelectionByModel}
				onRendererSelectionByModelChange={setRendererSelectionByModel}
				interactionSelectionByState={interactionSelectionByState}
				onInteractionSelectionByStateChange={setInteractionSelectionByState}
				modelDefinitionRevision={modelDefinitionRevision}
				autoFitMeshes
				autoFitBehavior="initial"
			/>
		</div>
	);
}
//#endregion

//#region 🔖PlaygroundHost
let spatialPlayChromeRegistered = false;

function registerSpatialPlayChrome(): void {
	if (spatialPlayChromeRegistered) return;
	spatialPlayChromeRegistered = true;
	for (const pane of ["shape", "building", "energy", "structure-classic"] as const) {
		registerUiScene3DSurfaceHost(spatialPlaySceneSurfaceIdForPane(pane), SpatialPlaySurfaceHost);
	}
}

/** @emoji 🧊 R3F viewport for one spatial play quad pane. */
function SpatialPlaySurfaceHost({ node }: { readonly node: UiScene3DHostSurfaceNode }): ReactNode {
	if (node.controllerId !== SPATIAL_PLAY_CONTROLLER_ID) {
		return <div style={{ padding: 8, fontSize: 12, color: "#f88" }}>Invalid spatial play surface binding</div>;
	}
	const pane = spatialPlayPaneFromSurfaceId(node.surfaceId);
	if (!pane) {
		return <div style={{ padding: 8, fontSize: 12, color: "#f88" }}>Unknown spatial play surface</div>;
	}
	return (
		<div className="absolute inset-0 flex min-h-0 min-w-0 flex-col overflow-hidden">
			{pane === "shape" ? <SpatialPlayShapePane /> : <SpatialPlayViewPane pane={pane} />}
		</div>
	);
}

class SpatialPlayCatalogPanelDefinition extends PureSidePanelTabDefinition {
	resolveTab(): SidePanelTabConfig {
		return {
			id: "spatial-play-catalog",
			icon: Shapes,
			order: 1,
			tree: new StaticTreePanelDefinition({
				sections: [
					{
						id: "spatial-play-catalog.section",
						label: "Catalog",
						defaultOpen: true,
						items: [{ id: "spatial-play-catalog.body", label: "Shape fixtures", description: <SpatialPlayCatalogAside /> }],
					},
				],
			}),
		};
	}
}

class SpatialPlayDetailsPanelDefinition extends PureSidePanelTabDefinition {
	resolveTab(): SidePanelTabConfig {
		return {
			id: "spatial-play-details",
			icon: ListTree,
			order: 0,
			tree: new StaticTreePanelDefinition({
				sections: [
					{
						id: "spatial-play-details.section",
						label: "Selection",
						defaultOpen: true,
						items: [{ id: "spatial-play-details.body", label: "Properties", description: <SpatialPlayDetailsAside /> }],
					},
				],
			}),
		};
	}
}

function SpatialPlayRoot(): ReactNode {
	const runtimeRef = useRef<ProductRuntime | null>(null);
	const shellControllerRef = useRef<SpatialPlayShellController | null>(null);
	const [chromeSnapshot, setChromeSnapshot] = useState<SpatialPlayChromeSnapshot | null>(null);
	if (!runtimeRef.current) {
		registerSpatialPlayChrome();
		runtimeRef.current = buildSpatialPlayRuntime();
		runtimeRef.current.setActiveAppId(SPATIAL_PLAY_APP_ID);
		shellControllerRef.current = runtimeRef.current.getActiveApp()?.controller as SpatialPlayShellController;
	}
	const shellController = shellControllerRef.current;
	if (!shellController) {
		return null;
	}
	const chromeContextValue = useMemo<SpatialPlayChromeContextValue>(
		() => ({ snapshot: chromeSnapshot, publishSnapshot: setChromeSnapshot }),
		[chromeSnapshot],
	);
	const chromeSnapshotRef = reactHostPort.useRef(chromeSnapshot);
	chromeSnapshotRef.current = chromeSnapshot;
	const chromeKey = chromeSnapshot
		? `${chromeSnapshot.activeModelDefinitionId}\u0001${chromeSnapshot.selection.map((row) => `${row.kind}:${row.id}`).join(",")}\u0001${spatialPlayModelsDigest(chromeSnapshot.modelsByDefinitionId)}`
		: "";
	const workbenchTabs = reactHostPort.useMemo(
		() => [
			new SpatialPlayCatalogPanelDefinition().resolveTab(),
			...(chromeSnapshot
				? [
						new SpatialPlayHierarchyPanelDefinition(() => {
							const snap = chromeSnapshotRef.current;
							if (!snap) return [];
							return buildSpatialPlayHierarchySections(
								snap.modelsByDefinitionId,
								snap.activeModelDefinitionId,
								snap.selection,
								snap.selectTarget,
							);
						}).resolveTab(),
					]
				: []),
		],
		[chromeSnapshot, chromeKey],
	);
	const detailsTabs = reactHostPort.useMemo(
		() => [new SpatialPlayDetailsPanelDefinition().resolveTab()],
		[],
	);
	return (
		<SpatialPlayChromeContext.Provider value={chromeContextValue}>
			<SpatialPlayModelSpaceProvider runtime={runtimeRef.current} shellController={shellController}>
				<LevelProvider level="window">
					<div className={`flex h-screen min-h-0 w-screen flex-col ${getLevelBgClass("window")}`}>
						<SpatialPlayLoadInput />
						<PlaygroundView
							runtime={runtimeRef.current}
							defaultAppId={SPATIAL_PLAY_APP_ID}
							augmentPanelTabs={{ workbench: workbenchTabs, details: detailsTabs }}
							initialPanelVisibility={{ leftSidePanel: true, rightSidePanel: true }}
							className="min-h-0 flex-1"
						/>
					</div>
				</LevelProvider>
			</SpatialPlayModelSpaceProvider>
		</SpatialPlayChromeContext.Provider>
	);
}

if (typeof document !== "undefined" && !import.meta.vitest) {
	const el = document.getElementById("root");
	if (el) {
		mountPlaygroundApp(
			<StrictMode>
				<SpatialPlayRoot />
			</StrictMode>,
		);
	}
}
//#endregion 🔖PlaygroundHost


//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("SpatialPlayShellController compute mode", () => {
		it("stores independent compute modes per quad pane", () => {
			const runtime = new ProductRuntime();
			const controller = new SpatialPlayShellController(runtime.commandBus, () => runtime.notify());
			expect(controller.getComputeModeForPane("shape")).toBe("fast");
			controller.run("setComputeModeForPane", { pane: "energy", value: "precise" });
			expect(controller.getComputeModeForPane("energy")).toBe("precise");
			expect(controller.getComputeModeForPane("shape")).toBe("fast");
			const energyWindow = controller.mainMode.windowKinds.find((row) => row.id === SPATIAL_PLAY_ENERGY_WINDOW_ID);
			expect(energyWindow?.measures[0]).toMatchObject({ kind: "select", value: "precise" });
		});
	});

	describe("buildSpatialPlayToolbarTools", () => {
		it("registers view, save, and transform categories", () => {
			const tools = buildSpatialPlayToolbarTools(
				{
					activeModelDefinitionId: SHAPE_MODEL_DEFINITION_ID,
					selectionCount: 0,
					transformsTo: [],
					transformsFrom: [],
				},
				SPATIAL_PLAY_CONTROLLER_ID,
			);
			expect(tools.view?.length).toBeGreaterThan(0);
			expect(tools.save?.map((row) => row.id)).toEqual([
				"spatial.play.save.selected",
				"spatial.play.save.modelspace",
				"spatial.play.save.current",
				"spatial.play.save.load",
			]);
			expect(tools.save?.[0]?.disabled).toBe(true);
		});
	});

	describe("spatial play runtime", () => {
		it("builds quad viewport bodies for each pane", () => {
			const runtime = buildSpatialPlayRuntime();
			const ctx = { runtime, activeModeId: "main", generation: 0 } as const;
			expect(
				buildSpatialPlayShapeDeclarativeBody({
					...ctx,
					windowKindId: SPATIAL_PLAY_SHAPE_WINDOW_ID,
					bodyKey: SPATIAL_PLAY_SHAPE_BODY_KEY,
				}),
			).toEqual(buildScene3dWindowBody(SPATIAL_PLAY_SHAPE_SCENE_SURFACE_ID, SPATIAL_PLAY_CONTROLLER_ID));
			expect(
				buildSpatialPlayEnergyDeclarativeBody({
					...ctx,
					windowKindId: SPATIAL_PLAY_ENERGY_WINDOW_ID,
					bodyKey: SPATIAL_PLAY_ENERGY_BODY_KEY,
				}),
			).toEqual(buildScene3dWindowBody(SPATIAL_PLAY_ENERGY_SCENE_SURFACE_ID, SPATIAL_PLAY_CONTROLLER_ID));
		});

		it("registers four window kinds in quad layout", () => {
			const app = buildSpatialPlayRuntime().getActiveApp();
			expect(app?.defaultLayout).toEqual(SPATIAL_PLAY_LAYOUT);
			expect(app?.windowKinds.map((row) => row.id)).toEqual([
				SPATIAL_PLAY_SHAPE_WINDOW_ID,
				SPATIAL_PLAY_BUILDING_WINDOW_ID,
				SPATIAL_PLAY_ENERGY_WINDOW_ID,
				SPATIAL_PLAY_STRUCTURE_CLASSIC_WINDOW_ID,
			]);
		});

		it("uses empty declarative side tab slots", () => {
			const app = buildSpatialPlayRuntime().getActiveApp();
			expect(app?.leftTabs).toEqual([]);
			expect(app?.rightTabs).toEqual([]);
		});

		it("spatialPlayModelsDigest changes when object rows are added", () => {
			const model = parseModelJson({
				schema: "spatial.model/v1",
				revision: 0,
				objects: {},
				geometry: { anchors: [], vertices: [], edges: [], wires: [], faces: [], shells: [], solids: [] },
			});
			expect(model).not.toBeNull();
			const before = spatialPlayModelsDigest({ "spatial.shape": model! });
			model!.objects["box1"] = {
				id: "box1",
				typology: "spatial.shape.primitive.box",
				primitives: { solid: "solid-1" },
			};
			model!.bump();
			const after = spatialPlayModelsDigest({ "spatial.shape": model! });
			expect(after).not.toBe(before);
		});

		it("buildSpatialPlayHierarchySections lists objects after box commit object binding", async () => {
			const { BrepjsKernel } = await import("@cad/js/kernel/brepjs");
			const spec = loadSpatialInteraction("primitive.box")!;
			const model = new Model();
			const kernel = new BrepjsKernel() as never;
			const rt = createInteractionRuntime(spec, {
				kernel,
				document: { model, nodes: [] },
				activeModelDefinitionId: SHAPE_MODEL_DEFINITION_ID,
			});
			await rt.send({ kind: "pointer.down", point: [0, 0, 0], modifiers: {} });
			await rt.send({ kind: "pointer.down", point: [2, 3, 0], modifiers: {} });
			await rt.send({ kind: "set.height", value: 4, modifiers: {} });
			await rt.send({ kind: "confirm", modifiers: {} });
			const sections = buildSpatialPlayHierarchySections({ [SHAPE_MODEL_DEFINITION_ID]: model }, SHAPE_MODEL_DEFINITION_ID, [], () => {});
			const modelBranch = sections[0]?.items?.[0]?.items?.[0];
			expect(modelBranch?.items?.some((row) => row.label !== "(no objects)")).toBe(true);
		});

		it("buildSpatialPlayHierarchySections nests topology under primitive slots", async () => {
			const { preciseSpatialKernelMath: M } = await import("@cad/js/kernel/brepjs");
			const { applyModelDiff, solidRef } = await import("@cad/js/core");
			const model = new Model();
			const solid = solidRef("solid-1");
			applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solid));
			model.objects["box1"] = {
				id: "box1",
				typology: "spatial.shape.primitive.box",
				primitives: { solid: String(solid) },
			};
			const sections = buildSpatialPlayHierarchySections({ "spatial.shape": model }, "spatial.shape", [], () => {});
			const primitiveNode = sections[0]?.items?.[0]?.items?.[0]?.items?.[0]?.items?.[0];
			expect(primitiveNode?.label).toContain("solid:");
			const shellNode = primitiveNode?.items?.[0];
			expect(shellNode?.label).toContain("shell");
			const faceNode = shellNode?.items?.[0];
			expect(faceNode?.label).toContain("face");
			const wireNode = faceNode?.items?.[0];
			expect(wireNode?.label).toContain("wire");
			const edgeNode = wireNode?.items?.[0];
			expect(edgeNode?.label).toContain("edge");
			expect(edgeNode?.items?.some((row) => row.label.includes("vertex"))).toBe(true);
		});
	});

	describe("spatial play typology chrome", () => {
		it("lists energy typologies from model definition scope", () => {
			const scope = resolveModelDefinitionScope("aec.building.energy");
			const labels = scope.typologies.map((row) => typologyObjectPascalFromLabel(row.label));
			expect(labels).toContain("BasePlate");
			expect(labels).toContain("ExternalWall");
			expect(labels).toContain("Roof");
		});
	});

	describe("spatial play model bootstrap", () => {
		it("emptyPlayModels always seeds spatial.shape", () => {
			expect(emptyPlayModels()[SHAPE_MODEL_DEFINITION_ID]).toBeInstanceOf(Model);
		});

		it("modelsFromSpatialJson on empty model space still seeds spatial.shape", () => {
			const models = modelsFromSpatialJson(new ModelSpace().toJSON());
			expect(models[SHAPE_MODEL_DEFINITION_ID]).toBeInstanceOf(Model);
		});

		it("modelsFromSpatialJson loads fixture models under spatial.shape", () => {
			const models = modelsFromSpatialJson(geometrySmallBuilding);
			expect(models[SHAPE_MODEL_DEFINITION_ID]?.objects).not.toEqual({});
		});

		it("ensureDerivedModelInSpace keeps spatial.shape for shape definition", () => {
			const models = ensureDerivedModelInSpace({}, SHAPE_MODEL_DEFINITION_ID);
			expect(models[SHAPE_MODEL_DEFINITION_ID]).toBeInstanceOf(Model);
		});

		it("ensureSpatialPlayQuadModels seeds all four play panes", () => {
			const models = ensureSpatialPlayQuadModels({});
			expect(models[SHAPE_MODEL_DEFINITION_ID]).toBeInstanceOf(Model);
			expect(models[SPATIAL_PLAY_BUILDING_MODEL_DEFINITION_ID]).toBeInstanceOf(Model);
			expect(models[SPATIAL_PLAY_ENERGY_MODEL_DEFINITION_ID]).toBeInstanceOf(Model);
			expect(models[SPATIAL_PLAY_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID]).toBeInstanceOf(Model);
		});
	});
}
//#endregion 🧪Tests
