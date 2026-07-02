// #region 🧲Header
/** @emoji 🛝 Imperative playground harness — fixtures and dev-host wiring only. */
// #endregion 🧲Header

import { Platform, Playground, createProductPlaygroundPlatform, type PlaygroundAppDefinition } from "@semio-tech/framework-playground-core";
import {
	IMPERATIVE_PLAY_APP_ID,
	ImperativePlayController,
	buildImperativePlayAppRuntime,
	registerImperativePlayDeclarativeBodies,
} from "./index.ts";

/** @emoji 🛝 Imperative playground app. */
export class PlaygroundImperative extends Playground {
	readonly id = IMPERATIVE_PLAY_APP_ID;

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id);
		const ctrl = new ImperativePlayController(runtime.commandBus, () => runtime.notify());
		runtime.addApp(buildImperativePlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerImperativePlayDeclarativeBodies();
	}
}

//#region 🔖PlaygroundAppDefinition
/** @emoji 🛝 Imperative playground app definition. */
export const imperativePlayAppDefinition: PlaygroundAppDefinition = {
	id: IMPERATIVE_PLAY_APP_ID,
	label: "Imperative",
	controllerId: "imperative-play",
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	createPlayground: () => new PlaygroundImperative(),
	bootRenderer: async (pg) => {
		const { bootImperativePlay } = await import("@semio-tech/framework-playground-renderer-react/imperative");
		bootImperativePlay(pg);
	},
	devHost: {
		playEntryKind: "imperative",
		resolveDedupe: ["react", "react-dom", "@semio-tech/imperative-react"],
		watchIgnored: ["../core/lib.rs", "../engine/**", "../module/**", "../core/target/**", "../core/pkg/**"],
		optimizeDeps: { include: ["react", "react-dom"] },
	},
};
//#endregion 🔖PlaygroundAppDefinition
