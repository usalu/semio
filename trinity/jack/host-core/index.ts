// #region 🧲Header
/** @emoji 🛝 `@semio-tech/trinity-jack-host-core` — playground harness exports. */
// #endregion 🧲Header

export { trinityJackPlayAppDefinition } from "./playground.ts";
export * from "./playground.ts";

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";
import { trinityJackPlayAppDefinition } from "./playground.ts";

/** @emoji 🧩 S program definition for trinity jack. */
export function buildTrinityProgramDefinition(): PlatformDefinition {
	const app = trinityJackPlayAppDefinition;
	return {
		id: "trinity",
		name: "Trinity",
		apiVersion: "1",
		apps: [{ id: "trinity-jack", label: app.label, controllerId: app.controllerId, modes: app.modes, defaultModeId: app.defaultModeId }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension
