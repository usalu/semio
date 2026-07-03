// #region 🧲Header
/** @emoji 🛝 Generic playground dev runner — boots any registered {@link PlaygroundAppDefinition}. */
// #endregion 🧲Header

import "../globals.css";
import { bootstrapElementsSurfaceChromeDocument } from "@semio-tech/ui-react";
import { loadPlaygroundApp } from "@semio-tech/framework-playground-core/app-registry";

const playEntryKind = import.meta.env.PUZZLE_PLAY_ENTRY as string | undefined;

if (typeof document !== "undefined" && document.getElementById("root") != null && !import.meta.vitest && playEntryKind) {
	bootstrapElementsSurfaceChromeDocument("system");
	void (async () => {
		const app = await loadPlaygroundApp(playEntryKind);
		if (!app) throw new Error(`[playground-dev] unknown app entry kind: ${playEntryKind}`);
		// #region 🏷️PageTitle
		if (typeof document !== "undefined" && app.label) {
			document.title = `semio · ${app.label.toLowerCase()}`;
		}
		// #endregion 🏷️PageTitle
		const playground = app.createPlayground();
		await app.bootRenderer(playground);
	})().catch((error) => {
		console.error("[DEBUG] playground-dev boot failed", error);
	});
}

