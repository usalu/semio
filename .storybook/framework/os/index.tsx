// #region 🧲Header
// 💻 .storybook/framework/os/index.tsx
// Specs: Boot `FrameworkOsShell` inside Storybook's own React tree, filtered to one plugin from the generated registry.
// Summary: Mirrors `bootFrameworkOs` (`framework/renderer/react/os-shell.tsx`) minus its `createRoot` call — Storybook already owns the tree, so this renders the shell directly and lets decorator/story unmount handle cleanup. Serves prebuilt plugin WASM from `/plugin-modules` (aliased + static-dir'd by the `framework/os` scope in `.storybook/scopes.ts`) and never triggers a cargo build: a missing artifact renders an instruction panel instead of failing silently.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import { useEffect, useMemo, useState } from "react";
import { FrameworkOsShell, resolveShellLocks, type FrameworkOsLocks } from "../../../framework/renderer/react/os-shell.tsx";
import { PLUGIN_BUILD_TARGETS, pluginModuleUrl, type PluginBuildTarget } from "../../../framework/plugin/registry/generated/plugins.ts";
import { bootstrapElementsSurfaceChromeDocument, readStoredUiChromeAppearance } from "@semio-tech/ui-react";

export { PLUGIN_BUILD_TARGETS };
export type { PluginBuildTarget };

// #region 🔖ArtifactProbe
/** @emoji 🔍 HEAD-probes a plugin's module URL; `undefined` while probing, then true/false. Never blocks on a cargo build — a missing artifact just renders an instruction panel. */
function usePluginArtifactAvailable(moduleUrl: string): boolean | undefined {
  const [available, setAvailable] = useState<boolean | undefined>(undefined);
  useEffect(() => {
    let cancelled = false;
    setAvailable(undefined);
    fetch(moduleUrl, { method: "HEAD" })
      .then((res) => !cancelled && setAvailable(res.ok))
      .catch(() => !cancelled && setAvailable(false));
    return () => {
      cancelled = true;
    };
  }, [moduleUrl]);
  return available;
}
// #endregion 🔖ArtifactProbe

// #region 🔖OsBootHost
export type OsBootHostProps = {
  /** Registry `pluginId`, e.g. `"s"`, `"puzzle"`, `"gis"`. Also used as the shell's `pluginFilter`. */
  readonly plugin: string;
  readonly appId?: string;
  readonly locks?: FrameworkOsLocks;
};

/** @emoji 🖥️ One entry from `PLUGIN_BUILD_TARGETS` resolved to its dev-build module URL, mirroring `framework/product/os/dev/js/index.ts`. */
function resolveTargetPlugin(pluginId: string): PluginBuildTarget | undefined {
  return PLUGIN_BUILD_TARGETS.find((t) => t.pluginId === pluginId);
}

/** @emoji 🖥️ Boots the real `FrameworkOsShell` filtered to one plugin — the app-boot story mechanism
 * "filters for starting apps" refers to. Keyed by `plugin` so switching the Storybook `plugin` control
 * fully remounts the shell (plugin runtimes are module-singletons and must not be reused across boots). */
export function OsBootHost({ plugin, appId, locks }: OsBootHostProps) {
  const target = resolveTargetPlugin(plugin);
  const moduleUrl = target ? pluginModuleUrl(target.pluginId, target.wasmOut) : undefined;
  const available = usePluginArtifactAvailable(moduleUrl ?? "");
  const resolvedLocks = useMemo(() => resolveShellLocks(locks), [locks]);

  useEffect(() => {
    bootstrapElementsSurfaceChromeDocument(resolvedLocks.appearance ?? readStoredUiChromeAppearance());
  }, [resolvedLocks.appearance]);

  if (!target) {
    return (
      <div className="p-4 text-sm text-red-600">
        unknown plugin {JSON.stringify(plugin)} — not in `framework/plugin/registry/generated/plugins.ts`
      </div>
    );
  }
  if (available === undefined) {
    return <div className="p-4 text-sm opacity-60">probing {target.pluginId} plugin artifact…</div>;
  }
  if (available === false) {
    return (
      <div className="p-4 text-sm">
        <p className="font-medium text-amber-600">plugin artifact missing: {target.pluginId}</p>
        <p className="mt-1 opacity-80">
          {moduleUrl} returned a non-OK response. Build it once with <code>bun nx run {target.pluginId}:build-wasm</code> (or the matching wasm target for{" "}
          <code>{target.cratePath}</code>) — this story never triggers a cargo build itself.
        </p>
      </div>
    );
  }
  return (
    <div key={`${target.pluginId}:${appId ?? ""}`} className="h-full w-full">
      <FrameworkOsShell pluginFilter={target.pluginId} plugins={[{ pluginId: target.pluginId, moduleUrl: moduleUrl! }]} appId={appId} locks={resolvedLocks} />
    </div>
  );
}
// #endregion 🔖OsBootHost
