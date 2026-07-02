// #region 🧲Header
/** @emoji 🛝 VCS playground harness — fixtures and dev-host wiring only. */
// #endregion 🧲Header

import { Platform, Playground, createProductPlaygroundPlatform, type PlaygroundAppDefinition } from "@semio-tech/framework-playground-core";
import {
	VCS_PLAY_APP_ID,
	VcsPlayController,
	buildVcsPlayAppRuntime,
	registerVcsPlayDeclarativeBodies,
} from "./index.ts";

/** @emoji 🛝 VCS playground app. */
export class PlaygroundVcs extends Playground {
	readonly id = VCS_PLAY_APP_ID;

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id, "VCS");
		const ctrl = new VcsPlayController(runtime.commandBus, () => runtime.notify());
		runtime.addApp(buildVcsPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerVcsPlayDeclarativeBodies();
	}
}

//#region 🔖PlaygroundAppDefinition
/** @emoji 🛝 VCS playground app definition. */
export const vcsPlayAppDefinition: PlaygroundAppDefinition = {
	id: VCS_PLAY_APP_ID,
	label: "VCS",
	controllerId: "vcs-play",
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	createPlayground: () => new PlaygroundVcs(),
	bootRenderer: async (pg) => {
		const { bootVcsPlay } = await import("@semio-tech/framework-playground-renderer-react/vcs");
		bootVcsPlay(pg);
	},
	devHost: {
		playEntryKind: "vcs",
		resolveDedupe: ["react", "react-dom", "@semio-tech/ui-react", "@semio-tech/vcs-react"],
		optimizeDeps: { include: ["react", "react-dom"] },
	},
};
//#endregion 🔖PlaygroundAppDefinition
