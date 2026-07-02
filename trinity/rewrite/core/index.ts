// #region 🧲Header
/** @emoji 🛝 `@semio-tech/trinity-rewrite-core` — playground harness exports. */
// #endregion 🧲Header

export { trinityRewritePlayAppDefinition } from "./playground.ts";
export * from "./playground.ts";

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";
import { trinityRewritePlayAppDefinition } from "./playground.ts";

/** @emoji 🧩 S program definition for trinity rewrite. */
export function buildTrinityRewriteProgramDefinition(): PlatformDefinition {
	const app = trinityRewritePlayAppDefinition;
	return {
		id: "trinity.rewrite",
		name: "Trinity Rewrite",
		apiVersion: "1",
		apps: [{ id: "trinity-rewrite", label: app.label, controllerId: app.controllerId, modes: app.modes, defaultModeId: app.defaultModeId }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension
