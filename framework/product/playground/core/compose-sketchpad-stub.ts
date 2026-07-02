// #region 🧲Header
/** @emoji 🧱 Browser-safe stub for `@semio-tech/compose-sketchpad` outside the s playground. */
// #endregion 🧲Header

export const COMPOSE_SKETCHPAD_PROGRAM_ID = "compose.sketchpad";

/** @emoji 🚫 Sketchpad platform is only booted from the s playground. */
export async function ensureSketchpadPlatform(): Promise<never> {
	throw new Error("compose-sketchpad is only available in the s playground");
}

/** @emoji 🧩 Minimal program definition for s registry wiring in non-s bundles. */
export function buildSketchpadProgramDefinition() {
	return {
		id: COMPOSE_SKETCHPAD_PROGRAM_ID,
		name: "Compose Sketchpad",
		apiVersion: "1",
		apps: [],
		createPlatformApi: () => ({}),
	};
}
