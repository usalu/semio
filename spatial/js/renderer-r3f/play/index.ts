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
	listModelDefinitionManifests,
	listModelObjectsForModelDefinition,
	objectPrimitiveEntries,
	resolvePrimitiveRefKind,
	typologyObjectPascalFromLabel,
	parseModelJson,
	type Model,
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
		const objectItems: TreeDataItem[] = listModelObjectsForModelDefinition(model, modelDefinitionId).map((object) => {
			const objectId = String(object.id);
			const typologyTail = object.typology.split(".").pop() ?? object.typology;
			const primitiveItems: TreeDataItem[] = objectPrimitiveEntries(object).map(([slot, primitiveRef]) => {
				const kind = resolvePrimitiveRefKind(model, primitiveRef) ?? "solid";
				const primitiveId = String(primitiveRef);
				return {
					id: `spatial-play-hierarchy.primitive.${modelDefinitionId}.${objectId}.${slot}`,
					label: `${slot}: ${kind} ${primitiveId}`,
					isSelected: isSelected(kind, primitiveId),
					onClick: () => onSelect(modelDefinitionId, { kind, id: primitiveId, editable: true }),
				};
			});
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

		it("buildSpatialPlayHierarchySections nests model definitions, objects, and primitives", () => {
			const model = parseModelJson({
				schema: "spatial.model/v1",
				revision: 0,
				objects: [{ id: "box1", typology: "spatial.shape.primitive.box", primitives: { solid: "solid-1" } }],
				geometry: {
					anchors: [],
					vertices: [],
					edges: [],
					wires: [],
					faces: [],
					shells: [],
					solids: [{ id: "solid-1", shellIds: [] }],
				},
			});
			expect(model).not.toBeNull();
			const sections = buildSpatialPlayHierarchySections(
				{ "spatial.shape": model! },
				"spatial.shape",
				[],
				() => {},
			);
			expect(sections[0]?.items?.[0]?.label).toBe("ModelSpace");
			const objectNode = sections[0]?.items?.[0]?.items?.[0]?.items?.[0];
			expect(objectNode?.items?.[0]?.label).toContain("solid:");
		});
	});
}
//#endregion 🧪Tests
