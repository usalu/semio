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

//#region 🔖Ids
export const SPATIAL_PLAY_APP_ID = "spatial-play";
export const SPATIAL_PLAY_CONTROLLER_ID = "spatial-play";
export const SPATIAL_PLAY_WINDOW_ID = "spatial-viewport";
export const SPATIAL_PLAY_WINDOW_LABEL = "Spatial";
export const SPATIAL_PLAY_BODY_KEY = "spatial.play.viewport";
export const SPATIAL_PLAY_SCENE_SURFACE_ID = "spatial.play.scene3d/v1";
//#endregion 🔖Ids

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
	});
}
//#endregion 🧪Tests
