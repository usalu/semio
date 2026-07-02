// #region 🧲Header
/** @emoji 🛝 Layout playground harness — fixtures and dev-host wiring only. */
// #endregion 🧲Header

import { Platform, Playground, createProductPlaygroundPlatform, type PlaygroundAppDefinition } from "@semio-tech/framework-playground-core";
import {
	LAYOUT_PLAY_APP_ID,
	LayoutPlayController,
	buildLayoutPlayAppRuntime,
	registerLayoutPlayDeclarativeBodies,
} from "./index.ts";

/** @emoji 🛝 Layout playground app. */
export class PlaygroundLayout extends Playground {
	readonly id = LAYOUT_PLAY_APP_ID;

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id);
		const ctrl = new LayoutPlayController(runtime.commandBus, () => runtime.notify());
		runtime.addApp(buildLayoutPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerLayoutPlayDeclarativeBodies();
	}
}

//#region 🔖PlaygroundAppDefinition
/** @emoji 🛝 Layout playground app definition. */
export const layoutPlayAppDefinition: PlaygroundAppDefinition = {
	id: LAYOUT_PLAY_APP_ID,
	label: "Layout",
	controllerId: "layout-play",
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	createPlayground: () => new PlaygroundLayout(),
	bootRenderer: async (pg) => {
		const { bootLayoutPlay } = await import("@semio-tech/framework-playground-renderer-react/layout");
		bootLayoutPlay(pg);
	},
	devHost: {
		playEntryKind: "layout",
		resolveDedupe: ["react", "react-dom", "@semio-tech/layout-react"],
		watchIgnored: ["../rs/lib.rs", "../rs/target/**", "../rs/pkg/**"],
		optimizeDeps: { include: ["react", "react-dom"] },
	},
};
//#endregion 🔖PlaygroundAppDefinition
