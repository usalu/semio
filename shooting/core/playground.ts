// #region 🧲Header
/** @emoji 🛝 Shooting playground harness — fixtures and dev-host wiring only. */
// #endregion 🧲Header

import { Platform, Playground, createProductPlaygroundPlatform, type PlaygroundAppDefinition } from "@semio-tech/framework-playground-core";
import {
	SHOOTING_PLAY_APP_ID,
	SHOOTING_PLAY_CONTROLLER_ID,
	ShootingPlayController,
	buildShootingPlayAppRuntime,
	registerShootingPlayDeclarativeBodies,
} from "./index.ts";

/** @emoji 🛝 Shooting playground app. */
export class PlaygroundShooting extends Playground {
	readonly id = SHOOTING_PLAY_APP_ID;

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id);
		const ctrl = new ShootingPlayController(runtime.commandBus, () => runtime.notify());
		runtime.addApp(buildShootingPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerShootingPlayDeclarativeBodies();
	}
}

//#region 🔖PlaygroundAppDefinition
/** @emoji 🛝 Shooting playground app definition. */
export const shootingPlayAppDefinition: PlaygroundAppDefinition = {
	id: SHOOTING_PLAY_APP_ID,
	label: "Shooting",
	controllerId: SHOOTING_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	createPlayground: () => new PlaygroundShooting(),
	bootRenderer: async (pg) => {
		const { bootShootingPlay } = await import("@semio-tech/framework-playground-renderer-react/shooting");
		bootShootingPlay(pg);
	},
	devHost: {
		playEntryKind: "shooting",
		resolveDedupe: ["react", "react-dom", "three", "@semio-tech/shooting-react"],
		optimizeDeps: { include: ["react", "react-dom", "three", "@react-three/fiber", "@react-three/drei", "@semio-tech/infinite-world-r3f", "@semio-tech/shooting-react"] },
	},
};
//#endregion 🔖PlaygroundAppDefinition
