// #region 🧲Header
/** @emoji 🖥️ OS dev runner — boots the Rust plugin framework with a selectable renderer. */
// #endregion 🧲Header

import "../globals.css";

const renderer = import.meta.env.VITE_SEMIO_RENDERER ?? import.meta.env.SEMIO_RENDERER ?? "react";
const pluginFilter = import.meta.env.VITE_SEMIO_PLUGIN ?? import.meta.env.SEMIO_PLUGIN ?? "s";

if (typeof document !== "undefined" && document.getElementById("root") != null && !import.meta.vitest) {
	if (renderer === "wgpu") {
		const { bootFrameworkOsWgpu } = await import("@semio-tech/framework-renderer-wgpu");
		const { PLUGIN_BUILD_TARGETS, pluginModuleUrl } = await import("./plugin-registry.ts");
		void bootFrameworkOsWgpu({
			plugin: pluginFilter,
			plugins: PLUGIN_BUILD_TARGETS.map((target) => ({
				pluginId: target.pluginId,
				moduleUrl: pluginModuleUrl(target.pluginId, target.wasmOut),
			})),
		}).catch((error) => {
			console.error("[DEBUG] os-dev wgpu boot failed", error);
		});
	} else {
		const { bootstrapElementsSurfaceChromeDocument } = await import("@semio-tech/ui-react");
		const { bootFrameworkOs } = await import("@semio-tech/framework-renderer-react");
		const { PLUGIN_BUILD_TARGETS, pluginModuleUrl } = await import("./plugin-registry.ts");
		bootstrapElementsSurfaceChromeDocument("system");
		void bootFrameworkOs({
			plugin: pluginFilter,
			plugins: PLUGIN_BUILD_TARGETS.map((target) => ({
				pluginId: target.pluginId,
				moduleUrl: pluginModuleUrl(target.pluginId, target.wasmOut),
			})),
		}).catch((error) => {
			console.error("[DEBUG] os-dev react boot failed", error);
		});
	}
}
