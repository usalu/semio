// #region 🧲Header
/** @emoji 🛝 `@semio-tech/puzzle-3d-core` — playground harness exports. */
// #endregion 🧲Header

export { puzzle3dPlayAppDefinition } from "./playground.ts";
export * from "./playground.ts";

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";
import { puzzle3dPlayAppDefinition } from "./playground.ts";

/** @emoji 🧩 S program definition for puzzle 3d. */
export function buildPuzzle3dProgramDefinition(): PlatformDefinition {
	const app = puzzle3dPlayAppDefinition;
	return {
		id: "puzzle.3d",
		name: "Puzzle 3D",
		apiVersion: "1",
		apps: [{ id: "puzzle3d", label: app.label, controllerId: app.controllerId, modes: app.modes, defaultModeId: app.defaultModeId }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension
