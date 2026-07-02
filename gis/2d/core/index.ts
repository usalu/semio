// #region 🧲Header
/** @emoji 🛝 `@semio-tech/gis-2d-core` — playground harness exports. */
// #endregion 🧲Header

export { gis2dPlayAppDefinition } from "./playground.ts";
export * from "./playground.ts";

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";
import { gis2dPlayAppDefinition } from "./playground.ts";

/** @emoji 🧩 S program definition for map. */
export function buildGisMapProgramDefinition(): PlatformDefinition {
	const app = gis2dPlayAppDefinition;
	return {
		id: "gis.map",
		name: "GIS Map",
		apiVersion: "1",
		apps: [{ id: "map", label: app.label, controllerId: app.controllerId, modes: app.modes, defaultModeId: app.defaultModeId }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension
