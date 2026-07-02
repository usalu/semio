export * from "./internal.ts";
export * from "./playground.ts";

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";
import { imperativePlayAppDefinition } from "./playground.ts";

/** @emoji 🧩 S program definition for imperative. */
export function buildImperativeProgramDefinition(): PlatformDefinition {
	const app = imperativePlayAppDefinition;
	return {
		id: "imperative",
		name: "Imperative",
		apiVersion: "1",
		apps: [{ id: "imperative", label: app.label, controllerId: app.controllerId, modes: app.modes, defaultModeId: app.defaultModeId }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension
