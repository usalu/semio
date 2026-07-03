// #region 🧲Header
/** @emoji 🖥️ OS dev runner — boots the Rust plugin framework with the React renderer. */
// #endregion 🧲Header

import "../globals.css";
import { bootstrapElementsSurfaceChromeDocument } from "@semio-tech/ui-react";
import { bootFrameworkOs } from "@semio-tech/framework-renderer-react";
import { PLUGIN_BUILD_TARGETS, pluginModuleUrl } from "./plugin-registry.ts";

if (typeof document !== "undefined" && document.getElementById("root") != null && !import.meta.vitest) {
	bootstrapElementsSurfaceChromeDocument("system");
	const pluginFilter = import.meta.env.VITE_SEMIO_PLUGIN ?? import.meta.env.SEMIO_PLUGIN ?? "draw";
	void bootFrameworkOs({
		plugin: pluginFilter,
		plugins: PLUGIN_BUILD_TARGETS.map((target) => ({
			pluginId: target.pluginId,
			moduleUrl: pluginModuleUrl(target.pluginId, target.wasmOut),
		})),
	}).catch((error) => {
		console.error("[DEBUG] os-dev boot failed", error);
	});
}
