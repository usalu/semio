// #region 🧲Header
/** @emoji 🛝 `@semio-tech/reasoning-mindmap-wires-core` — playground harness exports. */
// #endregion 🧲Header

export { wiresPlayAppDefinition } from "./playground.ts";
export * from "./playground.ts";

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";
import { wiresPlayAppDefinition } from "./playground.ts";

/** @emoji 🧩 S program definition for wires. */
export function buildReasoningWiresProgramDefinition(): PlatformDefinition {
	const app = wiresPlayAppDefinition;
	return {
		id: "reasoning.wires",
		name: "Reasoning Wires",
		apiVersion: "1",
		apps: [{ id: "wires", label: app.label, controllerId: app.controllerId, modes: app.modes, defaultModeId: app.defaultModeId }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension
