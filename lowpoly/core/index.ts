export * from "./internal.ts";
export * from "./playground.ts";

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";
import { lowpolyPlayAppDefinition } from "./playground.ts";

/** @emoji 🧩 S program definition for lowpoly. */
export function buildLowpolyProgramDefinition(): PlatformDefinition {
	const app = lowpolyPlayAppDefinition;
	return {
		id: "lowpoly",
		name: "Lowpoly",
		apiVersion: "1",
		apps: [{ id: "lowpoly", label: app.label, controllerId: app.controllerId, modes: app.modes, defaultModeId: app.defaultModeId }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension
