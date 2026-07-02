// #region 🧲Header
/** @emoji 🖥️ OS dev runner — boots S as the studio operating system. */
// #endregion 🧲Header

import { bootstrapElementsSurfaceChromeDocument } from "@semio-tech/ui-react";
import { bootOsDev } from "@semio-tech/s-core";

if (typeof document !== "undefined" && document.getElementById("root") != null && !import.meta.vitest) {
	bootstrapElementsSurfaceChromeDocument("system");
	void (async () => {
		await import("../globals.css");
		await bootOsDev();
	})().catch((error) => {
		console.error("[DEBUG] os-dev boot failed", error);
	});
}
