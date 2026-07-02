// #region 🧲Header
/** @emoji 🛝 Sequence playground harness — fixtures and dev-host wiring only. */
// #endregion 🧲Header

import { Platform, Playground, createProductPlaygroundPlatform, type PlaygroundAppDefinition } from "@semio-tech/framework-playground-core";
import {
	SEQUENCE_PLAY_APP_ID,
	SequencePlayController,
	buildSequencePlayAppRuntime,
	registerSequencePlayDeclarativeBodies,
} from "./index.ts";

/** @emoji 🛝 Sequence playground app. */
export class PlaygroundSequence extends Playground {
	readonly id = SEQUENCE_PLAY_APP_ID;

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id);
		const ctrl = new SequencePlayController(runtime.commandBus, () => runtime.notify());
		runtime.addApp(buildSequencePlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerSequencePlayDeclarativeBodies();
	}
}

//#region 🔖PlaygroundAppDefinition
/** @emoji 🛝 Sequence playground app definition. */
export const sequencePlayAppDefinition: PlaygroundAppDefinition = {
	id: SEQUENCE_PLAY_APP_ID,
	label: "Sequence",
	controllerId: "sequence-play",
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	createPlayground: () => new PlaygroundSequence(),
	bootRenderer: async (pg) => {
		const { bootSequencePlay } = await import("@semio-tech/framework-playground-renderer-react/sequence");
		bootSequencePlay(pg);
	},
	devHost: {
		playEntryKind: "sequence",
		resolveDedupe: ["react", "react-dom", "@semio-tech/sequence-react"],
		watchIgnored: ["../core/lib.rs", "../../imperative/**", "../core/target/**", "../core/pkg/**"],
		optimizeDeps: { include: ["react", "react-dom"] },
	},
};
//#endregion 🔖PlaygroundAppDefinition
