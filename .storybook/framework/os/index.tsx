// #region 🧲Header
// 💻 .storybook/framework/os/index.tsx
// Specs: Boot `FrameworkOsShell` inside Storybook's own React tree, filtered to one plugin from the generated registry.
// Summary: Mirrors `bootFrameworkOs` (`framework/os/renderer/js/react/index.tsx`) minus its `createRoot` call — Storybook already owns the tree, so this renders the shell directly and lets decorator/story unmount handle cleanup. Serves prebuilt plugin WASM from `/plugin-modules` (aliased + static-dir'd by the `framework/os` scope in `.storybook/scopes.ts`) and never triggers a cargo build: a missing artifact renders an instruction panel instead of failing silently.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import { useEffect, useMemo, useState } from "react";
import { FrameworkOsShell, resolveShellLocks, type FrameworkOsLocks } from "../../../framework/product/os/module/renderer/js/react/index.tsx";
import { PLUGIN_BUILD_TARGETS, pluginModuleUrl, type PluginBuildTarget } from "../../../framework/product/os/module/plugin/registry/generated/plugins.ts";
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

/** @emoji 🖥️ One entry from `PLUGIN_BUILD_TARGETS` resolved to its dev-build module URL, mirroring `framework/os/dev/js/index.ts`. */
function resolveTargetPlugin(pluginId: string): PluginBuildTarget | undefined {
  return PLUGIN_BUILD_TARGETS.find((t) => t.pluginId === pluginId);
}

/** @emoji 🖥️ Boots the real `FrameworkOsShell` filtered to one plugin — the app-boot story mechanism
 * "filters for starting apps" refers to. Keyed by `program` so switching the Storybook `program` control
 * fully remounts the shell (plugin runtimes are module-singletons and must not be reused across boots). */
export function OsBootHost({ program, appId, locks }: OsBootHostProps) {
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
        unknown program {JSON.stringify(plugin)} — not in `framework/plugin/registry/generated/plugins.ts`
      </div>
    );
  }
  if (available === undefined) {
    return <div className="p-4 text-sm opacity-60">probing {target.pluginId} program artifact…</div>;
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

// #region 🔖WgpuBootHost
export type WgpuBootHostProps = {
  /** Registry `pluginId` passed through to the wgpu renderer as its `pluginFilter`. */
  readonly plugin: string;
};

type WgpuBootState = { readonly kind: "booting" } | { readonly kind: "unavailable"; readonly reason: string } | { readonly kind: "missing-artifact" } | { readonly kind: "error"; readonly message: string } | { readonly kind: "ready" };

const WGPU_RENDERER_DIR_URL = "/renderer-modules/wgpu";

/** @emoji 🔍 Trunk hashes the wgpu bundle's filename per build (`semio-framework-renderer-wgpu-<hash>.js`);
 * parse it out of the built `index.html`'s module script instead of hardcoding a hash that goes stale on
 * every rebuild. */
async function resolveWgpuRendererModuleUrl(): Promise<string> {
  const indexUrl = `${WGPU_RENDERER_DIR_URL}/index.html`;
  const res = await fetch(indexUrl);
  if (!res.ok) throw new Error(`fetch ${indexUrl} failed: ${res.status}`);
  const html = await res.text();
  const match = html.match(/from ['"]\/([^'"]+\.js)['"]/);
  if (!match) throw new Error(`could not locate renderer module script inside ${indexUrl}`);
  return `${WGPU_RENDERER_DIR_URL}/${match[1]}`;
}

function navigatorGpuUnavailableReason(): string | undefined {
  if (typeof navigator === "undefined") return "no `navigator` (non-browser environment)";
  if (!("gpu" in navigator) || !(navigator as Navigator & { gpu?: unknown }).gpu) return "`navigator.gpu` is undefined — this browser/context has no WebGPU support";
  return undefined;
}

/** @emoji 🧊 Boots the real `@semio-tech/framework-renderer-wgpu` raw-wgpu host for one registry program,
 * with a graceful fallback when WebGPU itself is unavailable (headless CI Chromium without `--enable-unsafe-webgpu`,
 * Safari/Firefox, …) and when the plugin has no prebuilt artifact — mirrors {@link OsBootHost}'s artifact probe. */
export function WgpuBootHost({ program }: WgpuBootHostProps) {
  const target = resolveTargetPlugin(plugin);
  const gpuUnavailableReason = navigatorGpuUnavailableReason();
  const [state, setState] = useState<WgpuBootState>({ kind: "booting" });

  useEffect(() => {
    if (gpuUnavailableReason) {
      setState({ kind: "unavailable", reason: gpuUnavailableReason });
      return;
    }
    if (!target) {
      setState({ kind: "error", message: `unknown program ${JSON.stringify(plugin)} — not in \`framework/plugin/registry/generated/plugins.ts\`` });
      return;
    }
    let cancelled = false;
    let dispose: (() => void) | undefined;
    setState({ kind: "booting" });
    (async () => {
      const moduleUrl = pluginModuleUrl(target.pluginId, target.wasmOut);
      const artifactRes = await fetch(moduleUrl, { method: "HEAD" }).catch(() => undefined);
      if (cancelled) return;
      if (!artifactRes?.ok) {
        setState({ kind: "missing-artifact" });
        return;
      }
      const [{ bootFrameworkOsWgpu }, rendererModuleUrl] = await Promise.all([import("@semio-tech/framework-renderer-wgpu"), resolveWgpuRendererModuleUrl()]);
      if (cancelled) return;
      dispose = await bootFrameworkOsWgpu({ plugin: target.pluginId, plugins: [{ pluginId: target.pluginId, moduleUrl }], rendererModuleUrl });
      if (cancelled) {
        dispose();
        return;
      }
      setState({ kind: "ready" });
    })().catch((error: unknown) => {
      if (!cancelled) setState({ kind: "error", message: error instanceof Error ? error.message : String(error) });
    });
    return () => {
      cancelled = true;
      dispose?.();
    };
  }, [plugin, target?.pluginId, gpuUnavailableReason]);

  if (state.kind === "unavailable") {
    return <div className="p-4 text-sm opacity-80">WebGPU unavailable: {state.reason}</div>;
  }
  if (state.kind === "missing-artifact") {
    return (
      <div className="p-4 text-sm">
        <p className="font-medium text-amber-600">plugin artifact missing: {plugin}</p>
        <p className="mt-1 opacity-80">
          Build it once with <code>bun nx run {plugin}:build-wasm</code> — this story never triggers a cargo build itself.
        </p>
      </div>
    );
  }
  if (state.kind === "error") {
    return <div className="p-4 text-sm text-red-600">wgpu boot failed: {state.message}</div>;
  }
  return <div id="root" className="h-full w-full" data-testid="wgpu-boot-root" />;
}
// #endregion 🔖WgpuBootHost
