// #region 🧲Header
/** @emoji 🛝 Forms playground harness — fixtures and dev-host wiring only. */
// #endregion 🧲Header

import { Platform, Playground, createProductPlaygroundPlatform, type PlaygroundAppDefinition } from "@semio-tech/framework-playground-core";
import {
	FORMS_PLAY_APP_ID,
	FORMS_PLAY_CONTROLLER_ID,
	FormsPlayController,
	buildFormsPlayAppRuntime,
	registerFormsPlayDeclarativeBodies,
} from "./index.ts";

/** @emoji 🛝 Forms playground app. */
export class PlaygroundForms extends Playground {
	readonly id = FORMS_PLAY_APP_ID;

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id);
		const ctrl = new FormsPlayController(runtime.commandBus, () => runtime.notify());
		runtime.addApp(buildFormsPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerFormsPlayDeclarativeBodies();
	}
}

//#region 🔖PlaygroundAppDefinition
/** @emoji 🛝 Forms playground app definition. */
export const formsPlayAppDefinition: PlaygroundAppDefinition = {
	id: FORMS_PLAY_APP_ID,
	label: "Forms",
	controllerId: FORMS_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	createPlayground: () => new PlaygroundForms(),
	bootRenderer: async (pg) => {
		const { bootFormsPlay } = await import("@semio-tech/framework-playground-renderer-react/forms");
		bootFormsPlay(pg);
	},
	devHost: {
		playEntryKind: "forms",
		resolveDedupe: ["react", "react-dom", "three", "@react-three/fiber", "@react-three/drei", "@semio-tech/forms-react"],
		optimizeDeps: { include: ["react", "react-dom"] },
	},
};
//#endregion 🔖PlaygroundAppDefinition
