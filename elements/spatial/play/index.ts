// #region 🧲Header
// 💻 elements/spatial/play/index.ts — Framework-free spatial play: controller, declarative UI trees, workbench wiring (no React).
// #endregion 🧲Header

import {
	CommandBus,
	Controller,
	Workbench,
	WorkbenchApp,
	WorkbenchMode,
	WorkbenchWindowKind,
	buildScene3dWindowBody,
	createDefaultLayout,
	type ShellSidePanelBodyViewContext,
	type ShellWindowBodyViewContext,
	type ShellToolItem,
	type UiNode,
} from "@elements/ui-shell";

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

//#region 🔖Helpers
function kindLabel(kind: TopologicKind): string {
	return spatialKindLabel(kind);
}

function createAllKindsEnabled(): Record<TopologicKind, boolean> {
	return Object.fromEntries(TOPOLOGIC_KINDS.map((kind) => [kind, true])) as Record<TopologicKind, boolean>;
}

function shellToggles(prefix: "selection" | "filter", kinds: Readonly<Record<TopologicKind, boolean>>, command: string): ShellToolItem[] {
	return TOPOLOGIC_KINDS.map((kind, order) => ({
		id: `spatial.${prefix}.${kind}`,
		kind: "toggle" as const,
		text: kindLabel(kind),
		order,
		pressed: kinds[kind],
		controllerId: SPATIAL_PLAY_CONTROLLER_ID,
		command,
		args: { kind },
	}));
}
//#endregion 🔖Helpers

//#region 🔖Ids
export const SPATIAL_PLAY_APP_ID = "elements-geometry-spatial";
export const SPATIAL_PLAY_WINDOW_ID = "geometry-spatial-window";
export const SPATIAL_PLAY_WINDOW_LABEL = "Spatial Surface";
export const SPATIAL_PLAY_BODY_KEY = "elements.geometry.spatial.window";
export const SPATIAL_PLAY_CONTROLLER_ID = "geometry-spatial-play";
export const SPATIAL_PLAY_WORKBENCH_TAB_BODY_KEY = "elements.geometry.spatial.panel.workbench";
export const SPATIAL_PLAY_DETAILS_TAB_BODY_KEY = "elements.geometry.spatial.panel.details";
export const SPATIAL_PLAY_WORKBENCH_ICON_ID = "elements.geometry.spatial.icon.workbench";
export const SPATIAL_PLAY_DETAILS_ICON_ID = "elements.geometry.spatial.icon.details";
export const SPATIAL_PLAY_SCENE3D_SURFACE_ID = "elements.geometry.spatial.scene/v1";
export const SPATIAL_PLAY_PANEL_WORKBENCH_SURFACE_ID = "elements.geometry.spatial.panel.workbench/v1";
export const SPATIAL_PLAY_PANEL_DETAILS_SURFACE_ID = "elements.geometry.spatial.panel.details/v1";
const SPATIAL_PLAY_DEFAULT_LAYOUT = createDefaultLayout([SPATIAL_PLAY_WINDOW_ID], "row", [100], [SPATIAL_PLAY_WINDOW_LABEL]);
//#endregion 🔖Ids

//#region 🔖Controller
/** @emoji 🧭 Elements workbench controller for the spatial React surface. */
export class SpatialPlayShellController extends Controller {
	readonly browseMode = new WorkbenchMode("browse", "Browse", undefined);
	private fixture: TopologicFixtureV1 | null;
	private model: SpatialModel | null;
	private status: SpatialStatus;
	private error: string | null;
	private selectedId: string | null;
	private focusedKind: SpatialSurfaceKindFilter;
	private query: string;
	readonly selectableKinds: Record<TopologicKind, boolean>;
	readonly visibleKinds: Record<TopologicKind, boolean>;

	constructor(commandBus: CommandBus, hostNotify: () => void, initialFixture: TopologicFixtureV1 | null = null) {
		super(SPATIAL_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.fixture = initialFixture;
		this.model = initialFixture ? buildSpatialModel(initialFixture) : null;
		this.status = initialFixture ? "ready" : "loading";
		this.error = null;
		this.selectedId = null;
		this.focusedKind = "all";
		this.query = "";
		this.selectableKinds = createAllKindsEnabled();
		this.visibleKinds = createAllKindsEnabled();
		this.rebuildShellMode();
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
		this.rebuildShellMode();
		this.emit();
	}

	private rebuildShellMode(): void {
		const separatorOrder = TOPOLOGIC_KINDS.length;
		this.browseMode.tools = {
			selection: [
				...shellToggles("selection", this.selectableKinds, "toggleSelectableKind"),
				{ id: "spatial.selection.separator", kind: "separator", order: separatorOrder },
				{
					id: "spatial.selection.clear",
					kind: "button",
					label: "Clear",
					order: separatorOrder + 1,
					controllerId: SPATIAL_PLAY_CONTROLLER_ID,
					command: "setSelectedId",
					args: { id: null },
				},
			],
			filter: shellToggles("filter", this.visibleKinds, "toggleVisibleKind"),
		};
	}

	private isSelectableId(id: string | null): boolean {
		if (!id || !this.model) return false;
		const entity = this.model.get(id);
		return Boolean(entity && this.selectableKinds[entity.kind] && this.visibleKinds[entity.kind]);
	}

	private ensureSelectionValidity(): void {
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
		switch (command) {
			case "toggleSelectableKind": {
				const kind = (args as { kind: TopologicKind }).kind;
				this.selectableKinds[kind] = !this.selectableKinds[kind];
				break;
			}
			case "toggleVisibleKind": {
				const kind = (args as { kind: TopologicKind }).kind;
				this.visibleKinds[kind] = !this.visibleKinds[kind];
				break;
			}
			case "setSelectedId": {
				const id = (args as { id: string | null }).id;
				if (!id || this.isSelectableId(id)) this.selectedId = id;
				break;
			}
			case "setFocusedKind": {
				this.focusedKind = (args as { kind: SpatialSurfaceKindFilter }).kind;
				break;
			}
			case "setQuery": {
				this.query = (args as { query: string }).query;
				break;
			}
			case "setEntityTransform": {
				const { id, transform } = args as { id: string; transform: TopologicTransform };
				this.updateEntityTransform(id, transform);
				break;
			}
			default:
				break;
		}
		this.ensureSelectionValidity();
		this.rebuildShellMode();
		this.emit();
	}

	getSnapshot(): SpatialSurfaceSnapshot {
		const setFocusedKind = (kind: SpatialSurfaceKindFilter) => this.commandBus.dispatch(this.id, "setFocusedKind", { kind });
		const setSelectedId = (id: string | null) => this.commandBus.dispatch(this.id, "setSelectedId", { id });
		const setEntityTransform = (id: string, transform: TopologicTransform) => this.commandBus.dispatch(this.id, "setEntityTransform", { id, transform });
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

export function buildSpatialWorkbenchApp(controller: SpatialPlayShellController): WorkbenchApp {
	const app = new WorkbenchApp(
		SPATIAL_PLAY_APP_ID,
		"Spatial",
		undefined,
		controller,
		SPATIAL_PLAY_DEFAULT_LAYOUT as never,
		[new WorkbenchWindowKind(SPATIAL_PLAY_WINDOW_ID, SPATIAL_PLAY_WINDOW_LABEL, SPATIAL_PLAY_BODY_KEY)],
	);
	app.defaultModeId = controller.browseMode.id;
	app.addMode(controller.browseMode);
	app.leftTabs = [{ id: "spatial-browser", iconId: SPATIAL_PLAY_WORKBENCH_ICON_ID, order: 0, bodyKey: SPATIAL_PLAY_WORKBENCH_TAB_BODY_KEY }];
	app.rightTabs = [{ id: "spatial-details", iconId: SPATIAL_PLAY_DETAILS_ICON_ID, order: 0, bodyKey: SPATIAL_PLAY_DETAILS_TAB_BODY_KEY }];
	controller.run("setQuery", { query: "" });
	return app;
}

function spatialControllerFromContext(ctx: ShellWindowBodyViewContext): SpatialPlayShellController | undefined {
	return ctx.workbench.getActiveApp()?.controller as SpatialPlayShellController | undefined;
}

/** @emoji 🧩 Declarative main window: fullscreen scene3d canvas only (chrome via shell toolbar and side panels). */
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
