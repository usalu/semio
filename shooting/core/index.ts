// #region 🧲Header
/** @emoji 🛝 `@semio-tech/shooting-core` — playground harness exports. */
// #endregion 🧲Header

export { shootingPlayAppDefinition } from "./playground.ts";
export * from "./playground.ts";

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";
import { shootingPlayAppDefinition } from "./playground.ts";

/** @emoji 🧩 S program definition for shooting. */
export function buildShootingProgramDefinition(): PlatformDefinition {
	const app = shootingPlayAppDefinition;
	return {
		id: "shooting",
		name: "Shooting",
		apiVersion: "1",
		apps: [{ id: "shooting", label: app.label, controllerId: app.controllerId, modes: app.modes, defaultModeId: app.defaultModeId }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension
