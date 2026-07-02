// #region 🧲Header
/** @emoji 🛝 Trinity Rewrite playground harness — fixtures and dev-host wiring only. */
// #endregion 🧲Header

import { Platform, Playground, createProductPlaygroundPlatform, type PlaygroundAppDefinition } from "@semio-tech/framework-playground-core";
import {
	TRINITY_REWRITE_PLAY_APP_ID,
	TRINITY_REWRITE_PLAY_CONTROLLER_ID,
	TrinityRewritePlayController,
	buildTrinityRewritePlayAppRuntime,
	registerTrinityRewritePlayDeclarativeBodies,
} from "./index.ts";

/** @emoji 🛝 Trinity Rewrite playground app. */
export class PlaygroundTrinityRewrite extends Playground {
	readonly id = TRINITY_REWRITE_PLAY_APP_ID;

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id);
		const ctrl = new TrinityRewritePlayController(runtime.commandBus, () => runtime.notify());
		runtime.addApp(buildTrinityRewritePlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerTrinityRewritePlayDeclarativeBodies();
	}
}

//#region 🔖PlaygroundAppDefinition

/** @emoji 🛝 Trinity Rewrite playground app definition. */
export const trinityRewritePlayAppDefinition: PlaygroundAppDefinition = {
	id: TRINITY_REWRITE_PLAY_APP_ID,
	label: "Trinity Rewrite",
	controllerId: TRINITY_REWRITE_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	createPlayground: () => new PlaygroundTrinityRewrite(),
	bootRenderer: async (pg) => {
		const { bootTrinityRewritePlay } = await import("@semio-tech/framework-playground-renderer-react/trinity-rewrite");
		bootTrinityRewritePlay(pg);
	},
	devHost: {
		playEntryKind: "trinity-rewrite",
		resolveDedupe: ["react", "react-dom", "three"],
		watchIgnored: ["../engine/lib.rs", "../engine/target/**", "../engine/Cargo.toml"],
		optimizeDeps: { include: ["react", "react-dom", "react/jsx-runtime", "react/jsx-dev-runtime"] },
	},
};
//#endregion 🔖PlaygroundAppDefinition
