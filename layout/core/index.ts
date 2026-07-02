export * from "./internal.ts";
export * from "./playground.ts";

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";
import { layoutPlayAppDefinition } from "./playground.ts";

/** @emoji 🧩 S program definition for layout. */
export function buildLayoutProgramDefinition(): PlatformDefinition {
	const app = layoutPlayAppDefinition;
	return {
		id: "layout",
		name: "Layout",
		apiVersion: "1",
		apps: [{ id: "layout", label: app.label, controllerId: app.controllerId, modes: app.modes, defaultModeId: app.defaultModeId }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension
