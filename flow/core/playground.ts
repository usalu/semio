// #region 🧲Header
/** @emoji 🛝 Flow playground harness — fixtures and dev-host wiring only. */
// #endregion 🧲Header

import {
	Platform,
	Playground,
	createProductPlaygroundPlatform,
	type PlaygroundAppDefinition,
} from "@semio-tech/framework-playground-core";
import {
	FLOW_PLAY_APP_ID,
	FlowPlayController,
	buildFlowPlayAppRuntime,
	registerFlowPlayDeclarativeBodies,
} from "./index.ts";

/** @emoji 🛝 Flow playground app. */
export class PlaygroundFlow extends Playground {
	readonly id = FLOW_PLAY_APP_ID;

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id);
		const ctrl = new FlowPlayController(runtime.commandBus, () => runtime.notify());
		runtime.addApp(buildFlowPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerFlowPlayDeclarativeBodies();
	}
}

//#region 🔖PlaygroundAppDefinition
/** @emoji 🛝 Flow playground app definition. */
export const flowPlayAppDefinition: PlaygroundAppDefinition = {
	id: FLOW_PLAY_APP_ID,
	label: "Flow",
	controllerId: "flow-play",
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	createPlayground: () => new PlaygroundFlow(),
	bootRenderer: async (pg) => {
		const { bootFlowPlay } = await import("@semio-tech/framework-playground-renderer-react/flow");
		bootFlowPlay(pg);
	},
	devHost: {
		playEntryKind: "flow",
		resolveDedupe: ["react", "react-dom", "@semio-tech/flow-react"],
		watchIgnored: [
			"../core/lib.rs",
			"../core/target/**",
			"../core/Cargo.toml",
			"../core/Cargo.lock",
			"../core/script.ts",
			"../module/**/lib.rs",
			"../module/**/target/**",
			"../module/**/Cargo.toml",
			"../module/**/script.ts",
		],
		optimizeDeps: { include: ["react", "react-dom", "react/jsx-runtime", "react/jsx-dev-runtime", "@semio-tech/flow-react"] },
	},
};
//#endregion 🔖PlaygroundAppDefinition
