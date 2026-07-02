// #region 🧲Header
/** @emoji 🛝 Lowpoly playground harness — fixtures and dev-host wiring only. */
// #endregion 🧲Header

import { Platform, Playground, createProductPlaygroundPlatform, type PlaygroundAppDefinition } from "@semio-tech/framework-playground-core";
import {
	LOWPOLY_PLAY_APP_ID,
	LowpolyPlayController,
	buildLowpolyPlayAppRuntime,
	registerLowpolyPlayDeclarativeBodies,
} from "./index.ts";

/** @emoji 🛝 Lowpoly playground app. */
export class PlaygroundLowpoly extends Playground {
	readonly id = LOWPOLY_PLAY_APP_ID;

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id);
		const ctrl = new LowpolyPlayController(runtime.commandBus, () => runtime.notify());
		runtime.addApp(buildLowpolyPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerLowpolyPlayDeclarativeBodies();
	}
}

//#region 🔖PlaygroundAppDefinition
/** @emoji 🛝 Lowpoly playground app definition. */
export const lowpolyPlayAppDefinition: PlaygroundAppDefinition = {
	id: LOWPOLY_PLAY_APP_ID,
	label: "Lowpoly",
	controllerId: "lowpoly-play",
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	createPlayground: () => new PlaygroundLowpoly(),
	bootRenderer: async (pg) => {
		const { bootLowpolyPlay } = await import("@semio-tech/framework-playground-renderer-react/lowpoly");
		await bootLowpolyPlay(pg);
	},
	devHost: {
		playEntryKind: "lowpoly",
		resolveDedupe: ["react", "react-dom", "three", "scheduler", "@semio-tech/lowpoly-react"],
		watchIgnored: ["../core/lib.rs", "../core/target/**", "../core/pkg/**"],
		optimizeDeps: { include: ["react", "react-dom", "three", "@react-three/fiber", "@react-three/drei"] },
	},
};
//#endregion 🔖PlaygroundAppDefinition
