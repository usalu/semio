import { LevelProvider, Workbench, WorkbenchView, getLevelBgClass, mountReactApp, registerElementIcon, registerWindowBody } from "@elements/ui";
import { CommandBus, Controller, WorkbenchApp, WorkbenchMode, WorkbenchWindowKind, createDefaultLayout, type ShellToolItem } from "@elements/ui-shell";
import { ListFilter, ScanSearch } from "lucide-react";
import * as React from "react";

import topologyJson from "../../play/fixtures/topology.json";
import {
	TOPOLOGIC_KINDS,
	buildSpatialDetailsPanelState,
	buildSpatialModel,
	buildSpatialWorkbenchPanelState,
	ensureSpatialKernelLoaded,
	loadTopologicFixtureV1,
	spatialKindLabel,
	type SpatialDetailsPanelState,
	type SpatialModel,
	type SpatialStatus,
	type SpatialSurfaceKindFilter,
	type SpatialSurfaceSnapshot,
	type TopologicFixtureV1,
	type TopologicKind,
} from "../js/index.ts";
import { SpatialDetailsPanelBody, SpatialPlayWindowBody, SpatialWorkbenchPanelBody } from "../react/index.tsx";

import "./globals.css";

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
//#endregion 🔖Controller

//#region 🔖Bootstrap
let spatialPlayChromeRegistered = false;

function registerSpatialPlayChrome(): void {
	if (spatialPlayChromeRegistered) return;
	spatialPlayChromeRegistered = true;
	registerElementIcon(SPATIAL_PLAY_WORKBENCH_ICON_ID, React.createElement(ListFilter, { className: "size-4", "aria-hidden": true }));
	registerElementIcon(SPATIAL_PLAY_DETAILS_ICON_ID, React.createElement(ScanSearch, { className: "size-4", "aria-hidden": true }));
	registerWindowBody(SPATIAL_PLAY_BODY_KEY, SpatialPlayWindowBody);
	registerWindowBody(SPATIAL_PLAY_WORKBENCH_TAB_BODY_KEY, SpatialWorkbenchPanelBody);
	registerWindowBody(SPATIAL_PLAY_DETAILS_TAB_BODY_KEY, SpatialDetailsPanelBody);
}

/** @emoji 🚀 Builds the workbench around the reusable spatial React surface. */
export async function bootstrapSpatialWorkbench(): Promise<Workbench> {
	registerSpatialPlayChrome();
	const workbench = new Workbench();
	const controller = new SpatialPlayShellController(workbench.commandBus, () => workbench.notify());
	workbench.addApp(buildSpatialWorkbenchApp(controller));
	return workbench;
}

const rootElement = typeof document === "undefined" ? null : document.getElementById("root");
if (rootElement) {
	void bootstrapSpatialWorkbench().then((workbench) => {
		mountReactApp(
			React.createElement(
				LevelProvider,
				null,
				React.createElement(WorkbenchView, {
					workbench,
					className: getLevelBgClass(0),
					defaultAppId: SPATIAL_PLAY_APP_ID,
					initialPanelVisibility: { leftSidePanel: true, rightSidePanel: true },
				}),
			),
		);
	});
}
//#endregion 🔖Bootstrap

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

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
	});
}
//#endregion 🧪Tests
