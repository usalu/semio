// #region 🧲Header
/** @emoji 🛝 `@semio-tech/puzzle-2d-core` — playground harness exports. */
// #endregion 🧲Header

export { puzzle2dPlayAppDefinition } from "./playground.ts";
export * from "./playground.ts";

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";
import { puzzle2dPlayAppDefinition } from "./playground.ts";

/** @emoji 🧩 S program definition for puzzle 2d. */
export function buildPuzzle2dProgramDefinition(): PlatformDefinition {
	const app = puzzle2dPlayAppDefinition;
	return {
		id: "puzzle.2d",
		name: "Puzzle 2D",
		apiVersion: "1",
		apps: [{ id: "puzzle2d", label: app.label, controllerId: app.controllerId, modes: app.modes, defaultModeId: app.defaultModeId }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension
