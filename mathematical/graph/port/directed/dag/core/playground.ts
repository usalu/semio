// #region 🧲Header
/** @emoji 🛝 DAG playground harness — fixtures and dev-host wiring only. */
// #endregion 🧲Header

import { Platform, Playground, createProductPlaygroundPlatform, type PlaygroundAppDefinition } from "@semio-tech/framework-playground-core";
import {
	DAG_PLAY_APP_ID,
	DAG_PLAY_CONTROLLER_ID,
	DagPlayController,
	buildDagPlayAppRuntime,
	registerDagPlayDeclarativeBodies,
} from "./index.ts";

/** @emoji 🛝 DAG playground app. */
export class PlaygroundDag extends Playground {
	readonly id = DAG_PLAY_APP_ID;

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id);
		const ctrl = new DagPlayController(runtime.commandBus, () => runtime.notify());
		runtime.addApp(buildDagPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerDagPlayDeclarativeBodies();
	}
}

//#region 🔖PlaygroundAppDefinition

/** @emoji 🛝 DAG playground app definition. */
export const dagPlayAppDefinition: PlaygroundAppDefinition = {
	id: DAG_PLAY_APP_ID,
	label: "DAG",
	controllerId: DAG_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	createPlayground: () => new PlaygroundDag(),
	bootRenderer: async (pg) => {
		const { bootDagPlay } = await import("@semio-tech/framework-playground-renderer-react/dag");
		bootDagPlay(pg);
	},
	devHost: {
		playEntryKind: "dag",
		resolveDedupe: ["react", "react-dom"],
		watchIgnored: ["../lib.rs", "../target/**", "../Cargo.toml", "../Cargo.lock", "../script.ts"],
		optimizeDeps: { include: ["react", "react-dom", "react/jsx-runtime", "react/jsx-dev-runtime"] },
	},
};
//#endregion 🔖PlaygroundAppDefinition
