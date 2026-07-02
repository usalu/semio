export * from "./internal.ts";
export * from "./playground.ts";

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";
import { sequencePlayAppDefinition } from "./playground.ts";

/** @emoji 🧩 S program definition for sequence. */
export function buildSequenceProgramDefinition(): PlatformDefinition {
	const app = sequencePlayAppDefinition;
	return {
		id: "sequence",
		name: "Sequence",
		apiVersion: "1",
		apps: [{ id: "sequence", label: app.label, controllerId: app.controllerId, modes: app.modes, defaultModeId: app.defaultModeId }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension
