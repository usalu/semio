// #region 🧲Header
// 💻 cad/js/renderer-r3f/play/index.ts — Spatial play on `@framework/playground`: viewport window + scene3d host (React in main.tsx).
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
import { playgroundTreePanelRootItems } from "@framework/playground/renderer/react";
import type { TreeDataItem, TreeDataSection } from "@ui/react";
import {
	SHAPE_MODEL_DEFINITION_ID,
	buildModelTopologyHierarchy,
	createInteractionRuntime,
	listModelDefinitionManifests,
	listModelObjectsForModelDefinition,
	listTransformationsFromModelDefinition,
	listTransformationsIntoModelDefinition,
	loadSpatialInteraction,
	Model,
	objectPrimitiveEntries,
	parseModelJson,
	qualifiedTransformationId,
	resolvePrimitiveRefKind,
	typologyObjectPascalFromLabel,
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
	return playgroundTreePanelRootItems("spatial-play-hierarchy.root", [modelSpaceRoot]);
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
}
//#endregion 🧪Tests
