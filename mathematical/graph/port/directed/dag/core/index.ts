// #region 🧲Header
/** @emoji 🛝 `@semio-tech/dag-host-core` — playground harness exports. */
// #endregion 🧲Header

export { dagPlayAppDefinition } from "./playground.ts";
export * from "./playground.ts";

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";
import { dagPlayAppDefinition } from "./playground.ts";

/** @emoji 🧩 S program definition for dag. */
export function buildDagProgramDefinition(): PlatformDefinition {
	const app = dagPlayAppDefinition;
	return {
		id: "dag",
		name: "DAG",
		apiVersion: "1",
		apps: [{ id: "dag", label: app.label, controllerId: app.controllerId, modes: app.modes, defaultModeId: app.defaultModeId }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension
