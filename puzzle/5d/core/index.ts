// #region 🧲Header
/** @emoji 🛝 `@semio-tech/puzzle-5d-core` — playground harness exports. */
// #endregion 🧲Header

export { puzzle5dPlayAppDefinition } from "./playground.ts";
export * from "./playground.ts";

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";
import { puzzle5dPlayAppDefinition } from "./playground.ts";

/** @emoji 🧩 S program definition for puzzle 5d. */
export function buildPuzzle5dProgramDefinition(): PlatformDefinition {
	const app = puzzle5dPlayAppDefinition;
	return {
		id: "puzzle.5d",
		name: "Puzzle 5D",
		apiVersion: "1",
		apps: [{ id: "puzzle5d", label: app.label, controllerId: app.controllerId, modes: app.modes, defaultModeId: app.defaultModeId }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension
