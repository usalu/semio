// #region 🧲Header
/** @emoji 🛝 Presentation playground harness — fixtures and dev-host wiring only. */
// #endregion 🧲Header

import { Platform, Playground, createProductPlaygroundPlatform, type PlaygroundAppDefinition } from "@semio-tech/framework-playground-core";
import {
	PRESENTATION_PLAY_APP_ID,
	PRESENTATION_PLAY_CONTROLLER_ID,
	PresentationPlayController,
	buildPresentationPlayAppRuntime,
	registerPresentationPlayDeclarativeBodies,
} from "./index.ts";

/** @emoji 🛝 Presentation playground app. */
export class PresentationPlay extends Playground {
	readonly id = PRESENTATION_PLAY_APP_ID;
	readonly keybindings = [
		{ key: "Delete", controllerId: PRESENTATION_PLAY_CONTROLLER_ID, command: "deleteSelection" },
		{ key: "Backspace", controllerId: PRESENTATION_PLAY_CONTROLLER_ID, command: "deleteSelection" },
	];

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id);
		const ctrl = new PresentationPlayController(runtime.commandBus, () => runtime.notify());
		runtime.addApp(buildPresentationPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerPresentationPlayDeclarativeBodies();
	}
}

//#region 🔖PlaygroundAppDefinition
import type { PlaygroundAppDefinition } from "@semio-tech/framework-playground-core";

/** @emoji 🛝 Presentation playground app definition. */
export const presentationPlayAppDefinition: PlaygroundAppDefinition = {
	id: PRESENTATION_PLAY_APP_ID,
	label: "Presentation",
	controllerId: PRESENTATION_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	createPlayground: () => new PresentationPlay(),
	bootRenderer: async (pg) => {
		const { bootPresentationPlay } = await import("@semio-tech/framework-playground-renderer-react/presentation");
		bootPresentationPlay(pg);
	},
	devHost: {
		playEntryKind: "presentation",
		resolveDedupe: ["react", "react-dom", "./internal.ts"],
		optimizeDeps: { include: ["react", "react-dom", "react/jsx-runtime", "react/jsx-dev-runtime", "./internal.ts"] },
	},
};
//#endregion 🔖PlaygroundAppDefinition

