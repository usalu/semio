// #region 🧲Header
/** @emoji 🛝 `@semio-tech/procedural-3d-core` — playground harness exports. */
// #endregion 🧲Header

export { procedural3dPlayAppDefinition } from "./playground.ts";
export * from "./playground.ts";

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";
import { procedural3dPlayAppDefinition } from "./playground.ts";

/** @emoji 🧩 S program definition for procedural 3d. */
export function buildProcedural3dProgramDefinition(): PlatformDefinition {
	const app = procedural3dPlayAppDefinition;
	return {
		id: "procedural.3d",
		name: "Procedural 3D",
		apiVersion: "1",
		apps: [{ id: "procedural3d", label: app.label, controllerId: app.controllerId, modes: app.modes, defaultModeId: app.defaultModeId }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension
