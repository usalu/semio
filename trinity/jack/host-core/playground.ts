// #region 🧲Header
/** @emoji 🛝 Trinity Jack playground harness — fixtures and dev-host wiring only. */
// #endregion 🧲Header

import { Platform, Playground, createProductPlaygroundPlatform, type PlaygroundAppDefinition } from "@semio-tech/framework-playground-core";
import {
	TRINITY_JACK_PLAY_APP_ID,
	TRINITY_JACK_PLAY_CONTROLLER_ID,
	TrinityJackPlayController,
	buildTrinityJackPlayAppRuntime,
	registerTrinityJackPlayDeclarativeBodies,
} from "./index.ts";

/** @emoji 🛝 Trinity Jack playground app. */
export class PlaygroundTrinityJack extends Playground {
	readonly id = TRINITY_JACK_PLAY_APP_ID;

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id);
		const ctrl = new TrinityJackPlayController(runtime.commandBus, () => runtime.notify());
		runtime.addApp(buildTrinityJackPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerTrinityJackPlayDeclarativeBodies();
	}
}

//#region 🔖PlaygroundAppDefinition

/** @emoji 🛝 Trinity Jack playground app definition. */
export const trinityJackPlayAppDefinition: PlaygroundAppDefinition = {
	id: TRINITY_JACK_PLAY_APP_ID,
	label: "Trinity Jack",
	controllerId: TRINITY_JACK_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	createPlayground: () => new PlaygroundTrinityJack(),
	bootRenderer: async (pg) => {
		const { bootTrinityJackPlay } = await import("@semio-tech/framework-playground-renderer-react/trinity-jack");
		bootTrinityJackPlay(pg);
	},
	devHost: {
		playEntryKind: "trinity-jack",
		resolveDedupe: ["react", "react-dom", "three"],
		watchIgnored: ["../../rewrite/engine/lib.rs", "../../rewrite/engine/target/**", "../../rewrite/engine/Cargo.toml"],
		optimizeDeps: { include: ["react", "react-dom", "react/jsx-runtime", "react/jsx-dev-runtime"] },
	},
};
//#endregion 🔖PlaygroundAppDefinition
