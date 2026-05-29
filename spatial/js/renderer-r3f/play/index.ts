// #region 🧲Header
// 💻 spatial/js/renderer-r3f/play/index.ts — Spatial play on `@elements/playground`: viewport window + scene3d host (React in main.tsx).
// #endregion 🧲Header

import {
	CommandBus,
	Controller,
	ProductRuntime,
	AppRuntime,
	ModeRuntime,
	WindowKindRuntime,
	buildScene3dWindowBody,
	createStackLayout,
	registerWindowBody,
	type WindowBodyViewContext,
	type UiNode,
} from "@elements/playground";
import type { TreeDataItem, TreeDataSection } from "@elements/ui";
import {
	SHAPE_MODEL_DEFINITION_ID,
	buildModelTopologyHierarchy,
	createInteractionRuntime,
	listModelDefinitionManifests,
	listModelObjectsForModelDefinition,
	loadSpatialInteraction,
	Model,
	objectPrimitiveEntries,
	parseModelJson,
	resolvePrimitiveRefKind,
	typologyObjectPascalFromLabel,
	type ModelTopologyHierarchyNode,
	type SelectionTarget,
} from "@spatial/js-core";

//#region 🔖Ids
export const SPATIAL_PLAY_APP_ID = "spatial-play";
export const SPATIAL_PLAY_CONTROLLER_ID = "spatial-play";
export const SPATIAL_PLAY_WINDOW_ID = "spatial-viewport";
export const SPATIAL_PLAY_WINDOW_LABEL = "Spatial";
export const SPATIAL_PLAY_BODY_KEY = "spatial.play.viewport";
export const SPATIAL_PLAY_SCENE_SURFACE_ID = "spatial.play.scene3d/v1";
export const SPATIAL_PLAY_HIERARCHY_TAB_ID = "spatial-play-hierarchy";
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
	return [
		{
			id: "spatial-play-hierarchy.section",
			label: "Hierarchy",
			defaultOpen: true,
			items: [modelSpaceRoot],
		},
	];
}
//#endregion 🔖SpatialPlayHierarchy

//#region 🔖Controller
/** @emoji 🎛 Spatial play shell controller (viewport tools and measures live in the React host for now). */
export class SpatialPlayShellController extends Controller {
	readonly mainMode = new ModeRuntime("main", "Spatial", undefined);

	constructor(commandBus: CommandBus, hostNotify: () => void) {
		super(SPATIAL_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.mainMode.windowKinds = [
			new WindowKindRuntime(SPATIAL_PLAY_WINDOW_ID, SPATIAL_PLAY_WINDOW_LABEL, SPATIAL_PLAY_BODY_KEY),
		];
	}

	override run(_command: string, _args?: unknown): void {
		this.emit();
	}
}
//#endregion 🔖Controller

//#region 🔖Runtime
function spatialControllerFromContext(ctx: WindowBodyViewContext): SpatialPlayShellController | undefined {
	return ctx.runtime.getActiveApp()?.controller as SpatialPlayShellController | undefined;
}

/** @emoji 🧊 Declarative spatial viewport: lone scene3d surface bound to the play host. */
export function buildSpatialPlayDeclarativeBody(ctx: WindowBodyViewContext): UiNode {
	if (!spatialControllerFromContext(ctx)) {
		return { type: "text", value: "Missing spatial play controller" };
	}
	return buildScene3dWindowBody(SPATIAL_PLAY_SCENE_SURFACE_ID, SPATIAL_PLAY_CONTROLLER_ID);
}

export function buildSpatialPlayAppRuntime(controller: SpatialPlayShellController): AppRuntime {
	const app = new AppRuntime(
		SPATIAL_PLAY_APP_ID,
		"Spatial play",
		undefined,
		controller,
		createStackLayout([SPATIAL_PLAY_WINDOW_ID], [SPATIAL_PLAY_WINDOW_LABEL]) as never,
		controller.mainMode.windowKinds,
	);
	app.defaultModeId = controller.mainMode.id;
	app.addMode(controller.mainMode);
	app.leftTabs = [];
	app.rightTabs = [];
	return app;
}

/** @emoji 📝 Registers spatial play window body on the playground host. */
export function registerSpatialPlayDeclarativeBodies(): void {
	registerWindowBody(SPATIAL_PLAY_BODY_KEY, buildSpatialPlayDeclarativeBody);
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

	describe("spatial play runtime", () => {
		it("builds canvas-only viewport body", () => {
			const runtime = buildSpatialPlayRuntime();
			const body = buildSpatialPlayDeclarativeBody({
				runtime,
				windowKindId: SPATIAL_PLAY_WINDOW_ID,
				bodyKey: SPATIAL_PLAY_BODY_KEY,
				activeModeId: "main",
				generation: 0,
			});
			expect(body).toEqual(buildScene3dWindowBody(SPATIAL_PLAY_SCENE_SURFACE_ID, SPATIAL_PLAY_CONTROLLER_ID));
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
			const { BrepjsKernel } = await import("@spatial/js-kernel-brepjs");
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
			const { preciseSpatialKernelMath: M } = await import("@spatial/js-kernel-brepjs");
			const { applyModelDiff, solidRef } = await import("@spatial/js-core");
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
