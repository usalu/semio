export * from "./internal.ts";
export * from "./playground.ts";

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";
import { vcsPlayAppDefinition } from "./playground.ts";

/** @emoji 🧩 S program definition for vcs. */
export function buildVcsProgramDefinition(): PlatformDefinition {
	const app = vcsPlayAppDefinition;
	return {
		id: "vcs",
		name: "VCS",
		apiVersion: "1",
		apps: [{ id: "vcs", label: app.label, controllerId: app.controllerId, modes: app.modes, defaultModeId: app.defaultModeId }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension
