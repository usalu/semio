// #region 🧲Header
/** @emoji 🛝 `@elements/playground` — One-app playground shell: fixture slot, selection + filter toolbars, workbench + details side tabs. */
// #endregion 🧲Header

import {
	CommandBus,
	Controller,
	ProductRuntime,
	AppRuntime,
	ModeRuntime,
	WindowKindRuntime,
	buildScene3dWindowBody,
	createDefaultLayout,
	getWindowBodyFactory,
	registerSidePanelBody,
	registerWindowBody,
	type SidePanelBodyViewContext,
	type ToolItem,
	type WindowBodyViewContext,
	type UiNode,
	type WindowLayout,
} from "@elements/framework";

//#region 🔖Ids
/** @emoji 🏷 Stable ids for a single-app playground (main window + workbench + details tabs). */
export interface PlaygroundIds {
	readonly appId: string;
	readonly controllerId: string;
	readonly windowId: string;
	readonly windowLabel: string;
	readonly mainBodyKey: string;
	readonly workbenchTabBodyKey: string;
	readonly detailsTabBodyKey: string;
	readonly workbenchIconId: string;
	readonly detailsIconId: string;
	readonly mainSceneSurfaceId: string;
	readonly workbenchPanelSurfaceId: string;
	readonly detailsPanelSurfaceId: string;
}

export interface PlaygroundKindSpec<K extends string> {
	readonly kinds: readonly K[];
	readonly label: (kind: K) => string;
}

export type PlaygroundFocusFilter<K extends string> = "all" | K;
//#endregion 🔖Ids

//#region 🔖Toolbar
function createAllKindsEnabled<K extends string>(kinds: readonly K[]): Record<K, boolean> {
	return Object.fromEntries(kinds.map((kind) => [kind, true])) as Record<K, boolean>;
}

function playgroundKindToggles<K extends string>(
	prefix: "selection" | "filter",
	kinds: readonly K[],
	labels: (kind: K) => string,
	values: Readonly<Record<K, boolean>>,
	controllerId: string,
	command: string,
): ToolItem[] {
	return kinds.map((kind, order) => ({
		id: `playground.${prefix}.${kind}`,
		kind: "toggle" as const,
		text: labels(kind),
		order,
		pressed: values[kind],
		controllerId,
		command,
		args: { kind },
	}));
}
//#endregion 🔖Toolbar

//#region 🔖Controller
/** @emoji 🎛 Base playground controller: selection/filter kind toggles, query, selected id, focused kind. */
export abstract class PlaygroundController<K extends string> extends Controller {
	readonly browseMode = new ModeRuntime("browse", "Browse", undefined);
	protected readonly kinds: readonly K[];
	protected readonly kindLabel: (kind: K) => string;
	readonly selectableKinds: Record<K, boolean>;
	readonly visibleKinds: Record<K, boolean>;
	focusedKind: PlaygroundFocusFilter<K> = "all";
	query = "";
	selectedId: string | null = null;

	protected constructor(
		controllerId: string,
		spec: PlaygroundKindSpec<K>,
		commandBus: CommandBus,
		hostNotify: () => void,
	) {
		super(controllerId, commandBus, hostNotify);
		this.kinds = spec.kinds;
		this.kindLabel = spec.label;
		this.selectableKinds = createAllKindsEnabled(spec.kinds);
		this.visibleKinds = createAllKindsEnabled(spec.kinds);
		this.rebuildBrowseModeTools();
	}

	protected rebuildBrowseModeTools(): void {
		const separatorOrder = this.kinds.length;
		this.browseMode.tools = {
			selection: [
				...playgroundKindToggles("selection", this.kinds, this.kindLabel, this.selectableKinds, this.id, "toggleSelectableKind"),
				{ id: "playground.selection.separator", kind: "separator", order: separatorOrder },
				{
					id: "playground.selection.clear",
					kind: "button",
					label: "Clear",
					order: separatorOrder + 1,
					controllerId: this.id,
					command: "setSelectedId",
					args: { id: null },
				},
			],
			filter: playgroundKindToggles("filter", this.kinds, this.kindLabel, this.visibleKinds, this.id, "toggleVisibleKind"),
		};
	}

	/** @emoji ✅ Domain hook: whether `id` may be selected given current kind toggles. */
	protected abstract canSelectId(id: string): boolean;

	/** @emoji 🔄 Domain hook: clear selection when it becomes invalid (e.g. hidden kind). */
	protected ensureSelectionValidity(): void {
		if (this.selectedId !== null && !this.canSelectId(this.selectedId)) {
			this.selectedId = null;
		}
	}

	protected handlePlaygroundCommand(command: string, args?: unknown): boolean {
		switch (command) {
			case "toggleSelectableKind": {
				const { kind } = args as { kind: K };
				this.selectableKinds[kind] = !this.selectableKinds[kind];
				return true;
			}
			case "toggleVisibleKind": {
				const { kind } = args as { kind: K };
				this.visibleKinds[kind] = !this.visibleKinds[kind];
				return true;
			}
			case "setSelectedId": {
				const { id } = args as { id: string | null };
				if (!id || this.canSelectId(id)) this.selectedId = id;
				return true;
			}
			case "setFocusedKind": {
				this.focusedKind = (args as { kind: PlaygroundFocusFilter<K> }).kind;
				return true;
			}
			case "setQuery": {
				this.query = (args as { query: string }).query;
				return true;
			}
			default:
				return false;
		}
	}

	protected finishPlaygroundCommand(): void {
		this.ensureSelectionValidity();
		this.rebuildBrowseModeTools();
		this.emit();
	}
}
//#endregion 🔖Controller

//#region 🔖Runtime
export interface BuildPlaygroundWorkbenchAppOptions {
	readonly layout?: WindowLayout;
	readonly initialQuery?: string;
}

/** @emoji 🧩 Registers the standard playground app (browse mode, left workbench + right details tabs). */
export function buildPlaygroundWorkbenchApp(
	ids: PlaygroundIds,
	controller: PlaygroundController<string>,
	options?: BuildPlaygroundWorkbenchAppOptions,
): AppRuntime {
	const layout =
		options?.layout ??
		createDefaultLayout([ids.windowId], "row", [100], [ids.windowLabel]);
	const app = new AppRuntime(
		ids.appId,
		ids.windowLabel,
		undefined,
		controller,
		layout,
		[new WindowKindRuntime(ids.windowId, ids.windowLabel, ids.mainBodyKey)],
	);
	app.defaultModeId = controller.browseMode.id;
	app.addMode(controller.browseMode);
	app.leftTabs = [{ id: `${ids.appId}.workbench`, iconId: ids.workbenchIconId, order: 0, bodyKey: ids.workbenchTabBodyKey }];
	app.rightTabs = [{ id: `${ids.appId}.details`, iconId: ids.detailsIconId, order: 0, bodyKey: ids.detailsTabBodyKey }];
	controller.commandBus.dispatch(controller.id, "setQuery", { query: options?.initialQuery ?? "" });
	return app;
}

export function playgroundControllerFromContext(
	ctx: WindowBodyViewContext | SidePanelBodyViewContext,
): PlaygroundController<string> | undefined {
	return ctx.runtime.getActiveApp()?.controller as PlaygroundController<string> | undefined;
}

/** @emoji 🪟 Declarative main window: lone scene3d surface. */
export function buildPlaygroundMainWindowBody(ids: PlaygroundIds, ctx: WindowBodyViewContext): UiNode {
	if (!playgroundControllerFromContext(ctx)) {
		return { type: "text", value: "Missing playground controller" };
	}
	return buildScene3dWindowBody(ids.mainSceneSurfaceId, ids.controllerId);
}

/** @emoji 📋 Declarative workbench side tab: host-bound table surface. */
export function buildPlaygroundWorkbenchPanelBody(ids: PlaygroundIds, ctx: SidePanelBodyViewContext): UiNode {
	if (!playgroundControllerFromContext(ctx)) {
		return { type: "text", value: "Missing playground controller" };
	}
	return { type: "table", surfaceId: ids.workbenchPanelSurfaceId, controllerId: ids.controllerId };
}

/** @emoji 🔎 Declarative details side tab: host-bound table surface. */
export function buildPlaygroundDetailsPanelBody(ids: PlaygroundIds, ctx: SidePanelBodyViewContext): UiNode {
	if (!playgroundControllerFromContext(ctx)) {
		return { type: "text", value: "Missing playground controller" };
	}
	return { type: "table", surfaceId: ids.detailsPanelSurfaceId, controllerId: ids.controllerId };
}

export interface RegisterPlaygroundDeclarativeBodiesOptions {
	readonly buildMainWindow?: (ctx: WindowBodyViewContext) => UiNode;
	readonly buildWorkbenchPanel?: (ctx: SidePanelBodyViewContext) => UiNode;
	readonly buildDetailsPanel?: (ctx: SidePanelBodyViewContext) => UiNode;
}

export interface PlaygroundSidePanelBodyRegistration {
	readonly bodyKey: string;
	readonly build: (ctx: SidePanelBodyViewContext) => UiNode;
}

/** @emoji 📝 Registers multiple side-panel declarative trees. */
export function registerPlaygroundSidePanelBodies(tabs: readonly PlaygroundSidePanelBodyRegistration[]): void {
	for (const tab of tabs) {
		registerSidePanelBody(tab.bodyKey, tab.build);
	}
}

/** @emoji 📝 Registers playground window + side-panel declarative trees on the framework host. */
export function registerPlaygroundDeclarativeBodies(ids: PlaygroundIds, options?: RegisterPlaygroundDeclarativeBodiesOptions): void {
	registerWindowBody(ids.mainBodyKey, options?.buildMainWindow ?? ((ctx) => buildPlaygroundMainWindowBody(ids, ctx)));
	registerSidePanelBody(
		ids.workbenchTabBodyKey,
		options?.buildWorkbenchPanel ?? ((ctx) => buildPlaygroundWorkbenchPanelBody(ids, ctx)),
	);
	registerSidePanelBody(
		ids.detailsTabBodyKey,
		options?.buildDetailsPanel ?? ((ctx) => buildPlaygroundDetailsPanelBody(ids, ctx)),
	);
}

/** @emoji 🚀 Creates a {@link ProductRuntime} with one playground app. */
export function createPlaygroundWorkbench(
	ids: PlaygroundIds,
	controller: PlaygroundController<string>,
	options?: BuildPlaygroundWorkbenchAppOptions,
): ProductRuntime {
	const runtime = new ProductRuntime();
	runtime.addApp(buildPlaygroundWorkbenchApp(ids, controller, options));
	return runtime;
}

export interface BootstrapPlaygroundWorkbenchOptions extends BuildPlaygroundWorkbenchAppOptions {
	/** @emoji 📝 When true (default), registers standard playground declarative bodies before returning. */
	readonly registerDeclarativeBodies?: boolean;
	readonly declarativeBodies?: RegisterPlaygroundDeclarativeBodiesOptions;
	/** @emoji 🧱 Reuse an existing product runtime shell (controller must use its {@link CommandBus}). */
	readonly runtime?: ProductRuntime;
}

/** @emoji 🚀 One-shot playground setup: optional declarative registration + one playground app. */
export function bootstrapPlaygroundWorkbench(
	ids: PlaygroundIds,
	controller: PlaygroundController<string>,
	options?: BootstrapPlaygroundWorkbenchOptions,
): ProductRuntime {
	if (options?.registerDeclarativeBodies !== false) {
		registerPlaygroundDeclarativeBodies(ids, options?.declarativeBodies);
	}
	const runtime = options?.runtime ?? new ProductRuntime();
	runtime.addApp(buildPlaygroundWorkbenchApp(ids, controller, options));
	return runtime;
}
//#endregion 🔖Runtime

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	const TEST_IDS: PlaygroundIds = {
		appId: "test-playground",
		controllerId: "test-playground-ctrl",
		windowId: "main",
		windowLabel: "Main",
		mainBodyKey: "test.playground.main",
		workbenchTabBodyKey: "test.playground.workbench",
		detailsTabBodyKey: "test.playground.details",
		workbenchIconId: "test.playground.icon.workbench",
		detailsIconId: "test.playground.icon.details",
		mainSceneSurfaceId: "test.playground.scene/v1",
		workbenchPanelSurfaceId: "test.playground.panel.workbench/v1",
		detailsPanelSurfaceId: "test.playground.panel.details/v1",
	};

	class DemoPlaygroundController extends PlaygroundController<"a" | "b"> {
		private readonly selectable = new Set<string>(["entity-a", "entity-b"]);

		constructor(bus: CommandBus, notify: () => void) {
			super(TEST_IDS.controllerId, { kinds: ["a", "b"], label: (k) => k.toUpperCase() }, bus, notify);
		}

		protected canSelectId(id: string): boolean {
			return this.selectable.has(id) && this.selectableKinds[id === "entity-a" ? "a" : "b"] && this.visibleKinds[id === "entity-a" ? "a" : "b"];
		}

		override run(command: string, args?: unknown): void {
			if (this.handlePlaygroundCommand(command, args)) {
				this.finishPlaygroundCommand();
				return;
			}
		}
	}

	describe("PlaygroundController", () => {
		it("tracks query and clears selection when kind is hidden", () => {
			const bus = new CommandBus();
			const wb = new ProductRuntime();
			const ctrl = new DemoPlaygroundController(bus, () => wb.notify());
			wb.addApp(buildPlaygroundWorkbenchApp(TEST_IDS, ctrl));
			bus.dispatch(TEST_IDS.controllerId, "setSelectedId", { id: "entity-a" });
			bus.dispatch(TEST_IDS.controllerId, "setQuery", { query: "find" });
			expect(ctrl.query).toBe("find");
			expect(ctrl.selectedId).toBe("entity-a");
			bus.dispatch(TEST_IDS.controllerId, "toggleVisibleKind", { kind: "a" });
			expect(ctrl.selectedId).toBeNull();
		});
	});

	describe("bootstrapPlaygroundWorkbench", () => {
		it("registers declarative bodies and adds one app", () => {
			const bus = new CommandBus();
			const ctrl = new DemoPlaygroundController(bus, () => undefined);
			const wb = bootstrapPlaygroundWorkbench(TEST_IDS, ctrl);
			expect(wb.apps.length).toBeGreaterThan(0);
			expect(getWindowBodyFactory(TEST_IDS.mainBodyKey)).toBeTypeOf("function");
		});
	});

	describe("registerPlaygroundDeclarativeBodies", () => {
		it("registers scene3d main window and table side panels", () => {
			const bus = new CommandBus();
			const wb = new ProductRuntime();
			const ctrl = new DemoPlaygroundController(bus, () => wb.notify());
			wb.addApp(buildPlaygroundWorkbenchApp(TEST_IDS, ctrl));
			registerPlaygroundDeclarativeBodies(TEST_IDS);
			const ctx: WindowBodyViewContext = {
				runtime: wb,
				windowKindId: TEST_IDS.windowId,
				bodyKey: TEST_IDS.mainBodyKey,
				activeModeId: "browse",
				generation: wb.generation,
			};
			const main = getWindowBodyFactory(TEST_IDS.mainBodyKey)?.(ctx);
			expect(main?.type).toBe("scene3d");
		});
	});
}
//#endregion 🧪Tests
