// #region 🧲Header
/** @emoji 🛝 Raster playground harness — fixtures and dev-host wiring only. */
// #endregion 🧲Header

import { Platform, Playground, createProductPlaygroundPlatform, playgroundResolvedFixtureId, type PlaygroundAppDefinition } from "@semio-tech/framework-playground-core";
import {
	RASTER_PLAY_APP_ID,
	RASTER_PLAY_CONTROLLER_ID,
	RASTER_PLAY_FILE_FIXTURE_JSON_BY_ID,
	RASTER_PLAY_FIXTURE_DEFAULT_ID,
	RasterPlayController,
	buildRasterPlayAppRuntime,
	registerRasterPlayDeclarativeBodies,
} from "./index.ts";

/** @emoji 🛝 Raster playground app. */
export class PlaygroundRaster extends Playground {
	readonly id = RASTER_PLAY_APP_ID;
	readonly keybindings = [{ key: "ctrl+a,meta+a", controllerId: RASTER_PLAY_CONTROLLER_ID, command: "selectAll" }];

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id);
		const ctrl = new RasterPlayController(runtime.commandBus, () => runtime.notify());
		const resolved = playgroundResolvedFixtureId(RASTER_PLAY_FIXTURE_DEFAULT_ID);
		const fixtureJson = RASTER_PLAY_FILE_FIXTURE_JSON_BY_ID[resolved];
		if (fixtureJson) {
			ctrl.run("setActiveFixture", { fixtureId: resolved });
		}
		runtime.addApp(buildRasterPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerRasterPlayDeclarativeBodies();
	}
}

//#region 🔖PlaygroundAppDefinition
/** @emoji 🛝 Raster playground app definition. */
export const rasterPlayAppDefinition: PlaygroundAppDefinition = {
	id: RASTER_PLAY_APP_ID,
	label: "Raster",
	controllerId: RASTER_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	createPlayground: () => new PlaygroundRaster(),
	bootRenderer: async (pg) => {
		const { bootRasterPlay } = await import("@semio-tech/framework-playground-renderer-react/raster");
		bootRasterPlay(pg);
	},
	devHost: {
		playEntryKind: "raster",
		resolveDedupe: ["react", "react-dom", "@semio-tech/raster-react", "three"],
		optimizeDeps: { include: ["react", "react-dom", "@semio-tech/raster-react"] },
	},
};
//#endregion 🔖PlaygroundAppDefinition
