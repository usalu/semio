// #region 🧲Header
/** @emoji 🖥️ OS dev runner — boots the Rust plugin framework with a selectable renderer. */
// #endregion 🧲Header

import "../globals.css";

export type { PluginBuildTarget } from "../../../../plugin/registry/generated/plugins.ts";
export { PLUGIN_BUILD_TARGETS, PLUGIN_TARGETS, pluginModuleUrl } from "../../../../plugin/registry/generated/plugins.ts";

import { PLUGIN_BUILD_TARGETS, pluginModuleUrl } from "../../../../plugin/registry/generated/plugins.ts";
import { resolveShellBrandById } from "../brand/index.ts";

const renderer = import.meta.env.VITE_SEMIO_RENDERER ?? import.meta.env.SEMIO_RENDERER ?? "react";
const pluginFromUrl = typeof location !== "undefined" ? new URLSearchParams(location.search).get("plugin") : null;
const pluginFilter = pluginFromUrl ?? import.meta.env.VITE_SEMIO_PLUGIN ?? import.meta.env.SEMIO_PLUGIN ?? "s";
const appId = import.meta.env.VITE_SEMIO_APP_ID;

/** @emoji 🏷️ Baked-in shell brand for this artifact (registry `brand` column or `SEMIO_BRAND`); no `?query=` override. */
const brand = resolveShellBrandById(import.meta.env.VITE_SEMIO_BRAND || undefined);

/** @emoji 🔒 Boot-time-only shell preference locks; unlike `plugin`, these have no `?query=` override. */
const locks = {
  exampleId: import.meta.env.VITE_SEMIO_LOCKED_EXAMPLE || undefined,
  locale: import.meta.env.VITE_SEMIO_LOCKED_LOCALE || undefined,
  terminology: import.meta.env.VITE_SEMIO_LOCKED_TERMINOLOGY || undefined,
  themeId: import.meta.env.VITE_SEMIO_LOCKED_THEME || undefined,
  appearance: import.meta.env.VITE_SEMIO_LOCKED_APPEARANCE || undefined,
};

/** @emoji 🎛️ Boot-time shell preference defaults — seed values that keep their in-app switcher visible. */
const defaults = {
  exampleId: import.meta.env.VITE_SEMIO_DEFAULT_EXAMPLE || undefined,
};

if (typeof document !== "undefined" && document.getElementById("root") != null && !import.meta.vitest) {
  const plugins = PLUGIN_BUILD_TARGETS.map((target) => ({
    pluginId: target.pluginId,
    moduleUrl: pluginModuleUrl(target.pluginId, target.wasmOut),
  }));
  if (renderer !== "wgpu") {
    const { bootFrameworkOs } = await import("@semio-tech/framework-renderer-react");
    void bootFrameworkOs({ plugin: pluginFilter, plugins, appId, locks, defaults, brand }).catch((error) => {
      console.error("[DEBUG] os-dev react boot failed", error);
    });
  }
}
