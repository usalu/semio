// #region 🧲Header
// 💻 elements/spatial/play/index.ts — Framework-free spatial play on {@link @elements/playground} (no React).
// #endregion 🧲Header

import {
	CommandBus,
	Workbench,
	buildScene3dWindowBody,
	type ShellSidePanelBodyViewContext,
	type ShellWindowBodyViewContext,
	type UiNode,
} from "@elements/framework";
import {
	PlaygroundController,
	buildPlaygroundWorkbenchApp,
	createPlaygroundWorkbench,
	registerPlaygroundDeclarativeBodies,
	type PlaygroundIds,
} from "@elements/playground";

import topologyJson from "../fixtures/topology.json";
import {
	Cell,
	TOPOLOGIC_KINDS,
	buildSpatialDetailsPanelState,
	buildSpatialModel,
	buildSpatialWorkbenchPanelState,
	buildTopologicRenderPacketV1,
	ensureSpatialKernelLoaded,
	ensureTopologicWasmLoaded,
	loadTopologicFixtureV1,
	parseTopologicFixtureV1,
	spatialKindLabel,
	updateTopologicFixtureTransformV1,
	type SpatialModel,
	type SpatialStatus,
	type SpatialSurfaceKindFilter,
	type SpatialSurfaceSnapshot,
	type TopologicFixtureV1,
	type TopologicKind,
	type TopologicTransform,
} from "@elements/geometry-spatial-js";

//#region 🔖Ids
export const SPATIAL_PLAY_IDS: PlaygroundIds = {
	appId: "elements-geometry-spatial",
	controllerId: "geometry-spatial-play",
	windowId: "geometry-spatial-window",
	windowLabel: "Spatial Surface",
	mainBodyKey: "elements.geometry.spatial.window",
	workbenchTabBodyKey: "elements.geometry.spatial.panel.workbench",
	detailsTabBodyKey: "elements.geometry.spatial.panel.details",
	workbenchIconId: "elements.geometry.spatial.icon.workbench",
	detailsIconId: "elements.geometry.spatial.icon.details",
	mainSceneSurfaceId: "elements.geometry.spatial.scene/v1",
	workbenchPanelSurfaceId: "elements.geometry.spatial.panel.workbench/v1",
	detailsPanelSurfaceId: "elements.geometry.spatial.panel.details/v1",
};

export const SPATIAL_PLAY_APP_ID = SPATIAL_PLAY_IDS.appId;
export const SPATIAL_PLAY_WINDOW_ID = SPATIAL_PLAY_IDS.windowId;
export const SPATIAL_PLAY_WINDOW_LABEL = SPATIAL_PLAY_IDS.windowLabel;
export const SPATIAL_PLAY_BODY_KEY = SPATIAL_PLAY_IDS.mainBodyKey;
export const SPATIAL_PLAY_CONTROLLER_ID = SPATIAL_PLAY_IDS.controllerId;
export const SPATIAL_PLAY_WORKBENCH_TAB_BODY_KEY = SPATIAL_PLAY_IDS.workbenchTabBodyKey;
export const SPATIAL_PLAY_DETAILS_TAB_BODY_KEY = SPATIAL_PLAY_IDS.detailsTabBodyKey;
export const SPATIAL_PLAY_WORKBENCH_ICON_ID = SPATIAL_PLAY_IDS.workbenchIconId;
export const SPATIAL_PLAY_DETAILS_ICON_ID = SPATIAL_PLAY_IDS.detailsIconId;
export const SPATIAL_PLAY_SCENE3D_SURFACE_ID = SPATIAL_PLAY_IDS.mainSceneSurfaceId;
export const SPATIAL_PLAY_PANEL_WORKBENCH_SURFACE_ID = SPATIAL_PLAY_IDS.workbenchPanelSurfaceId;
export const SPATIAL_PLAY_PANEL_DETAILS_SURFACE_ID = SPATIAL_PLAY_IDS.detailsPanelSurfaceId;
//#endregion 🔖Ids

//#region 🔖Controller
/** @emoji 🧭 Spatial playground controller: topologic fixture, model, and transform editing. */
export class SpatialPlayShellController extends PlaygroundController<TopologicKind> {
	private fixture: TopologicFixtureV1 | null;
	private model: SpatialModel | null;
	private status: SpatialStatus;
	private error: string | null;

	constructor(commandBus: CommandBus, hostNotify: () => void, initialFixture: TopologicFixtureV1 | null = null) {
		super(
			SPATIAL_PLAY_CONTROLLER_ID,
			{ kinds: TOPOLOGIC_KINDS, label: spatialKindLabel },
			commandBus,
			hostNotify,
		);
		this.fixture = initialFixture;
		this.model = initialFixture ? buildSpatialModel(initialFixture) : null;
		this.status = initialFixture ? "ready" : "loading";
		this.error = null;
		if (!initialFixture) void this.bootstrapFixture();
	}

	private async bootstrapFixture(): Promise<void> {
		try {
			await ensureSpatialKernelLoaded();
			const fixture = await loadTopologicFixtureV1(topologyJson as unknown);
			if (!fixture) throw new Error("Spatial fixture failed to parse.");
			this.fixture = fixture;
			this.model = buildSpatialModel(fixture);
			this.status = "ready";
			this.error = null;
		} catch (error) {
			this.status = "error";
			this.error = error instanceof Error ? error.message : String(error);
		}
		this.finishPlaygroundCommand();
	}

	private isSelectableId(id: string | null): boolean {
		if (!id || !this.model) return false;
		const entity = this.model.get(id);
		return Boolean(entity && this.selectableKinds[entity.kind] && this.visibleKinds[entity.kind]);
	}

	protected override canSelectId(id: string): boolean {
		return this.isSelectableId(id);
	}

	protected override ensureSelectionValidity(): void {
		if (!this.isSelectableId(this.selectedId)) this.selectedId = null;
	}

	private updateEntityTransform(id: string, transform: TopologicTransform): void {
		if (!this.fixture || !this.model?.get(id)) return;
		const nextFixture = updateTopologicFixtureTransformV1(this.fixture, id, transform);
		if (!nextFixture) return;
		this.fixture = nextFixture;
		this.model = buildSpatialModel(nextFixture);
		this.status = "ready";
		this.error = null;
	}

	override run(command: string, args?: unknown): void {
		if (this.handlePlaygroundCommand(command, args)) {
			this.finishPlaygroundCommand();
			return;
		}
		switch (command) {
			case "setEntityTransform": {
				const { id, transform } = args as { id: string; transform: TopologicTransform };
				this.updateEntityTransform(id, transform);
				break;
			}
			default:
				break;
		}
		this.finishPlaygroundCommand();
	}

	getSnapshot(): SpatialSurfaceSnapshot {
		const setFocusedKind = (kind: SpatialSurfaceKindFilter) => this.commandBus.dispatch(this.id, "setFocusedKind", { kind });
		const setSelectedId = (id: string | null) => this.commandBus.dispatch(this.id, "setSelectedId", { id });
		const setEntityTransform = (id: string, transform: TopologicTransform) =>
			this.commandBus.dispatch(this.id, "setEntityTransform", { id, transform });
		const setQuery = (query: string) => this.commandBus.dispatch(this.id, "setQuery", { query });
		return {
			status: this.status,
			fixtureLabel: this.fixture?.label,
			model: this.model,
			focusedKind: this.focusedKind,
			selectedId: this.selectedId,
			query: this.query,
			error: this.error,
			selectableKinds: this.selectableKinds,
			visibleKinds: this.visibleKinds,
			setSelectedId,
			setEntityTransform,
			workbenchPanel: buildSpatialWorkbenchPanelState({
				fixtureLabel: this.fixture?.label,
				model: this.model,
				focusedKind: this.focusedKind,
				selectedId: this.selectedId,
				query: this.query,
				selectableKinds: this.selectableKinds,
				visibleKinds: this.visibleKinds,
				setFocusedKind,
				setSelectedId,
				setQuery,
			}),
			detailsPanel: buildSpatialDetailsPanelState({
				status: this.status,
				model: this.model,
				focusedKind: this.focusedKind,
				selectedId: this.selectedId,
				query: this.query,
			}),
		};
	}
}

export function buildSpatialWorkbenchApp(controller: SpatialPlayShellController) {
	return buildPlaygroundWorkbenchApp(SPATIAL_PLAY_IDS, controller);
}

export function createSpatialPlayWorkbench(controller: SpatialPlayShellController): Workbench {
	return createPlaygroundWorkbench(SPATIAL_PLAY_IDS, controller);
}

function spatialControllerFromContext(
	ctx: ShellWindowBodyViewContext | ShellSidePanelBodyViewContext,
): SpatialPlayShellController | undefined {
	return ctx.workbench.getActiveApp()?.controller as SpatialPlayShellController | undefined;
}

/** @emoji 🧩 Declarative main window: fullscreen scene3d canvas only. */
export function buildSpatialPlayDeclarativeBody(ctx: ShellWindowBodyViewContext): UiNode {
	if (!spatialControllerFromContext(ctx)) {
		return { type: "text", value: "Missing spatial controller" };
	}
	return buildScene3dWindowBody(SPATIAL_PLAY_SCENE3D_SURFACE_ID, SPATIAL_PLAY_CONTROLLER_ID);
}

/** @emoji 🧩 Declarative workbench side tab (entity browser). */
export function buildSpatialWorkbenchDeclarativePanel(ctx: ShellSidePanelBodyViewContext): UiNode {
	if (!spatialControllerFromContext(ctx)) {
		return { type: "text", value: "Missing spatial controller" };
	}
	return { type: "table", surfaceId: SPATIAL_PLAY_PANEL_WORKBENCH_SURFACE_ID, controllerId: SPATIAL_PLAY_CONTROLLER_ID };
}

/** @emoji 🧩 Declarative details side tab (selection inspector). */
export function buildSpatialDetailsDeclarativePanel(ctx: ShellSidePanelBodyViewContext): UiNode {
	if (!spatialControllerFromContext(ctx)) {
		return { type: "text", value: "Missing spatial controller" };
	}
	return { type: "table", surfaceId: SPATIAL_PLAY_PANEL_DETAILS_SURFACE_ID, controllerId: SPATIAL_PLAY_CONTROLLER_ID };
}

/** @emoji 📝 Registers spatial declarative bodies on the framework host. */
export function registerSpatialPlayDeclarativeBodies(): void {
	registerPlaygroundDeclarativeBodies(SPATIAL_PLAY_IDS, {
		buildMainWindow: buildSpatialPlayDeclarativeBody,
		buildWorkbenchPanel: buildSpatialWorkbenchDeclarativePanel,
		buildDetailsPanel: buildSpatialDetailsDeclarativePanel,
	});
}
//#endregion 🔖Controller

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("spatial imperative core (shared fixture)", () => {
		it(
			"parses the shared topologic fixture and builds brep-backed renderables",
			async () => {
				await ensureSpatialKernelLoaded();
				const fixture = parseTopologicFixtureV1(topologyJson);
				expect(fixture).not.toBeNull();
				const model = buildSpatialModel(fixture!);
				expect(model.listByKind("cell").length).toBeGreaterThan(0);
				const cell = model.listByKind("cell")[0];
				expect(cell).toBeInstanceOf(Cell);
				const renderable = cell.toRenderable(model);
				expect((renderable.fill?.position.length ?? 0) > 0 || (renderable.children?.length ?? 0) > 0).toBe(true);
				const rootRenderable = model.rootNodes()[0]?.toRenderable(model);
				expect(rootRenderable?.children?.length ?? 0).toBeGreaterThan(0);
			},
			60000,
		);

		it("updates one entity transform immutably from the shared fixture", () => {
			const fixture = parseTopologicFixtureV1(topologyJson);
			expect(fixture).not.toBeNull();
			const next = updateTopologicFixtureTransformV1(fixture!, "cell-room", { position: [1, 2, 3] });
			const updated = next?.topologies.find((entity) => entity.id === "cell-room");
			expect(updated?.transform?.position).toEqual([1, 2, 3]);
			expect(fixture!.topologies.find((entity) => entity.id === "cell-room")?.transform).toBeUndefined();
		});

		it("exports topologic-compatible bindings and render packets", async () => {
			const fixture = await loadTopologicFixtureV1(topologyJson);
			expect(fixture).not.toBeNull();
			const bindings = await ensureTopologicWasmLoaded();
			expect(bindings.parseFixture(topologyJson)?.schema).toBe("elements.geometry.topologic.fixture/v1");
			expect(bindings.edgeCurve(fixture!, "edge-arc").length).toBeGreaterThanOrEqual(2);
			const packet = buildTopologicRenderPacketV1(fixture!);
			expect(packet?.entries.find((entry) => entry.id === "edge-arc")?.points).toBeInstanceOf(Float32Array);
		});
	});

	describe("spatial play controller", () => {
		it("tracks focused kind and query in the surface snapshot", async () => {
			const fixture = await loadTopologicFixtureV1(topologyJson as unknown);
			expect(fixture).not.toBeNull();
			const bus = new CommandBus();
			const controller = new SpatialPlayShellController(bus, () => undefined, fixture);
			bus.dispatch(SPATIAL_PLAY_CONTROLLER_ID, "setFocusedKind", { kind: "cell" });
			bus.dispatch(SPATIAL_PLAY_CONTROLLER_ID, "setQuery", { query: "room" });
			const snapshot = controller.getSnapshot();
			expect(snapshot.focusedKind).toBe("cell");
			expect(snapshot.query).toBe("room");
		});

		it("clears the selection when the selected kind becomes hidden", async () => {
			const fixture = await loadTopologicFixtureV1(topologyJson as unknown);
			expect(fixture).not.toBeNull();
			const bus = new CommandBus();
			const controller = new SpatialPlayShellController(bus, () => undefined, fixture);
			const selectedId = controller.getSnapshot().model?.listByKind("cell")[0]?.id ?? null;
			expect(selectedId).not.toBeNull();
			bus.dispatch(SPATIAL_PLAY_CONTROLLER_ID, "setSelectedId", { id: selectedId });
			expect(controller.getSnapshot().selectedId).toBe(selectedId);
			bus.dispatch(SPATIAL_PLAY_CONTROLLER_ID, "toggleVisibleKind", { kind: "cell" });
			expect(controller.getSnapshot().selectedId).toBeNull();
		});

		it("updates one selected entity transform through controller commands", async () => {
			const fixture = await loadTopologicFixtureV1(topologyJson as unknown);
			expect(fixture).not.toBeNull();
			const bus = new CommandBus();
			const controller = new SpatialPlayShellController(bus, () => undefined, fixture);
			bus.dispatch(SPATIAL_PLAY_CONTROLLER_ID, "setEntityTransform", { id: "face-front", transform: { position: [2, 3, 4] } });
			expect(controller.getSnapshot().model?.get("face-front")?.transform?.position).toEqual([2, 3, 4]);
		});

		it("declarative window body is a lone scene3d surface", async () => {
			const fixture = await loadTopologicFixtureV1(topologyJson as unknown);
			const bus = new CommandBus();
			const wb = new Workbench();
			const ctrl = new SpatialPlayShellController(bus, () => wb.notify(), fixture);
			wb.addApp(buildSpatialWorkbenchApp(ctrl));
			const tree = buildSpatialPlayDeclarativeBody({
				workbench: wb,
				windowKindId: SPATIAL_PLAY_WINDOW_ID,
				bodyKey: SPATIAL_PLAY_BODY_KEY,
				activeModeId: "browse",
				generation: wb.generation,
			});
			expect(tree).toEqual(buildScene3dWindowBody(SPATIAL_PLAY_SCENE3D_SURFACE_ID, SPATIAL_PLAY_CONTROLLER_ID));
		});
	});
}
//#endregion 🧪Tests
