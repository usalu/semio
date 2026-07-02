export * from "./internal.ts";
export * from "./playground.ts";

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";
import { drawPlayAppDefinition } from "./playground.ts";

export { drawPlayAppDefinition } from "./playground.ts";

/** @emoji 🧩 S program definition for draw. */
export function buildDrawProgramDefinition(): PlatformDefinition {
	const app = drawPlayAppDefinition;
	return {
		id: "draw",
		name: "Draw",
		apiVersion: "1",
		apps: [{ id: "draw", label: app.label, controllerId: app.controllerId, modes: app.modes, defaultModeId: app.defaultModeId }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension
