// #region 🧲Header
/** @emoji 🛝 `@semio-tech/procedural-2d-core` — playground harness exports. */
// #endregion 🧲Header

export { procedural2dPlayAppDefinition } from "./playground.ts";
export * from "./playground.ts";

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";
import { procedural2dPlayAppDefinition } from "./playground.ts";

/** @emoji 🧩 S program definition for procedural 2d. */
export function buildProcedural2dProgramDefinition(): PlatformDefinition {
	const app = procedural2dPlayAppDefinition;
	return {
		id: "procedural.2d",
		name: "Procedural 2D",
		apiVersion: "1",
		apps: [{ id: "procedural2d", label: app.label, controllerId: app.controllerId, modes: app.modes, defaultModeId: app.defaultModeId }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension
