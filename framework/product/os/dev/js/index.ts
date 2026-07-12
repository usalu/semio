// #region 🧲Header
/** @emoji 🖥️ OS dev runner — boots the Rust plugin framework with a selectable renderer. */
// #endregion 🧲Header

import "../globals.css";

export type { PluginBuildTarget } from "../../../../plugin/registry/generated/plugins.ts";
export { PLUGIN_BUILD_TARGETS, PLUGIN_TARGETS, pluginModuleUrl } from "../../../../plugin/registry/generated/plugins.ts";

import { PLUGIN_BUILD_TARGETS, pluginModuleUrl } from "../../../../plugin/registry/generated/plugins.ts";

const renderer = import.meta.env.VITE_SEMIO_RENDERER ?? import.meta.env.SEMIO_RENDERER ?? "react";
const pluginFromUrl = typeof location !== "undefined" ? new URLSearchParams(location.search).get("plugin") : null;
const pluginFilter = pluginFromUrl ?? import.meta.env.VITE_SEMIO_PLUGIN ?? import.meta.env.SEMIO_PLUGIN ?? "s";
const appId = import.meta.env.VITE_SEMIO_APP_ID;

if (typeof document !== "undefined" && document.getElementById("root") != null && !import.meta.vitest) {
  const plugins = PLUGIN_BUILD_TARGETS.map((target) => ({
    pluginId: target.pluginId,
    moduleUrl: pluginModuleUrl(target.pluginId, target.wasmOut),
  }));
  if (renderer !== "wgpu") {
    const { bootFrameworkOs } = await import("@semio-tech/framework-renderer-react");
    void bootFrameworkOs({ plugin: pluginFilter, plugins, appId }).catch((error) => {
      console.error("[DEBUG] os-dev react boot failed", error);
    });
  }
}
