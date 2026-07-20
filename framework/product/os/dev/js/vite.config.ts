import path from "node:path";
import { fileURLToPath } from "node:url";
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";
import { playgroundAssetVitePlugins, playgroundSceneHostResolveAliases, resolveGisMapTileServeMode, semioBrandHtmlVitePlugins, uiAssetsVitePlugin } from "../../../../../ui/styling/vite-elements-assets.ts";
import { PLAYGROUND_BUILD_TARGETS } from "../../../../plugin/registry/generated/playgrounds.ts";
import { isStudioPluginFilter } from "../../../../plugin/registry/script.ts";
import { resolveShellBrandById } from "../brand/index.ts";
import { semioBackboneVitePlugin, semioBlobVitePlugin } from "../script.ts";

const configDir = path.dirname(fileURLToPath(import.meta.url));
const playDir = path.resolve(configDir, "..");
const repoRoot = path.resolve(playDir, "../../../..");
const pluginModulesDir = path.join(playDir, "plugin-modules");
const rendererModulesDir = path.join(playDir, "renderer-modules");
const renderer = process.env.SEMIO_RENDERER ?? "react";
const plugin = process.env.SEMIO_PLUGIN ?? process.env.PLAYGROUND_APP_KIND ?? "s";
const brandId = process.env.SEMIO_BRAND ?? PLAYGROUND_BUILD_TARGETS.find((target) => target.variant === plugin || target.aliases.includes(plugin))?.brand;
const brand = resolveShellBrandById(brandId);
const uiAssetsRoot = path.join(repoRoot, "ui/asset");

//#region 🔖RegistryDrivenAssetsAndEngines
/** @emoji 🔌 Framework engine crates every react-renderer dev session needs regardless of the active
 * plugin (the node-graph/editor host engines back shared studio chrome, not any one app) — kept as a
 * literal baseline rather than per-plugin metadata, mirroring the equally-unconditional pre-registry
 * build in `os/dev/script.ts`'s `buildEngineWasm`. */
const FRAMEWORK_ENGINE_OPTIMIZE_DEPS_EXCLUDE = ["@semio-tech/framework-surface-node-graph-rs", "@semio-tech/framework-editor-rs"];

/** @emoji 📦 Maps a registry `engines` crate path (e.g. `framework/surface/tiled-map/rs`) to its wasm-pack npm package name. */
function engineNpmPackage(cratePath: string): string {
  const slug = (cratePath.endsWith("/rs") ? cratePath.slice(0, -"/rs".length) : cratePath).replace(/\//g, "-");
  return `@semio-tech/${slug}-rs`;
}

const registryEngineOptimizeDepsExclude = [...new Set(PLAYGROUND_BUILD_TARGETS.flatMap((target) => target.engines))].map(engineNpmPackage);

/** @emoji 🗂️ The active playground's declared asset needs — every playground's assets when unfiltered
 * (the "s" studio hub can open any app, so it needs every app's dev-time asset routes available), else
 * just the resolved variant's own `assets` row. */
const resolvedPlaygroundAssets = isStudioPluginFilter(plugin) ? PLAYGROUND_BUILD_TARGETS.flatMap((target) => target.assets) : (PLAYGROUND_BUILD_TARGETS.find((target) => target.variant === plugin)?.assets ?? []);
//#endregion 🔖RegistryDrivenAssetsAndEngines

export default defineConfig({
  root: playDir,
  publicDir: path.join(playDir, "public"),
  assetsInclude: ["**/*.wasm"],
  resolve: {
    alias: [
      ...playgroundSceneHostResolveAliases(repoRoot),
      { find: "@semio-tech/ui-react", replacement: path.resolve(repoRoot, "ui/js/react/index.tsx") },
      { find: "@semio-tech/ui-asset", replacement: path.resolve(repoRoot, "ui/asset") },
      { find: "@semio-tech/ui-styling", replacement: path.resolve(repoRoot, "ui/styling/js") },
      { find: "@semio-tech/infinite-cavas-react-renderer", replacement: path.resolve(repoRoot, "infinite/cavas/react-renderer/index.tsx") },
      { find: "@semio-tech/infinite-world-r3f", replacement: path.resolve(repoRoot, "infinite/world/r3f/index.tsx") },
      { find: "@semio-tech/framework-renderer-react", replacement: path.resolve(repoRoot, "framework/renderer/react/index.tsx") },
      { find: "@semio-tech/framework-renderer-wgpu", replacement: path.resolve(repoRoot, "framework/renderer/wgpu/index.ts") },
      { find: "@semio-tech/framework-core", replacement: path.resolve(repoRoot, "framework/core/js/index.ts") },
      { find: "@semio-tech/framework-os-core", replacement: path.resolve(repoRoot, "framework/product/os/core/js/index.ts") },
      { find: "/plugin-modules", replacement: pluginModulesDir },
      { find: "/renderer-modules", replacement: rendererModulesDir },
    ],
    dedupe: ["react", "react-dom", "three", "@react-three/fiber", "@react-three/drei"],
  },
  server: {
    port: Number(process.env.S_OS_PORT ?? 6066),
    strictPort: true,
    fs: { allow: [repoRoot, pluginModulesDir, rendererModulesDir] },
  },
  plugins: [
    semioBackboneVitePlugin(),
    semioBlobVitePlugin(),
    ...uiAssetsVitePlugin(uiAssetsRoot),
    ...semioBrandHtmlVitePlugins(repoRoot, brand),
    ...playgroundAssetVitePlugins(repoRoot, resolvedPlaygroundAssets, resolveGisMapTileServeMode(process.env.GIS_MAP_TILE_SERVE_MODE)),
    ...(renderer === "wgpu" ? [tailwindcss()] : [react(), tailwindcss()]),
  ],
  optimizeDeps: {
    include: ["react-reconciler", "react-reconciler/constants", "three", "@react-three/fiber", "fuse.js"],
    exclude: [...(renderer === "wgpu" ? ["@semio-tech/framework-renderer-react"] : []), ...FRAMEWORK_ENGINE_OPTIMIZE_DEPS_EXCLUDE, ...registryEngineOptimizeDepsExclude],
  },
  define: {
    "import.meta.env.VITE_SEMIO_PLUGIN": JSON.stringify(process.env.SEMIO_PLUGIN ?? "s"),
    "import.meta.env.VITE_SEMIO_RENDERER": JSON.stringify(renderer),
    "import.meta.env.VITE_SEMIO_BRAND": JSON.stringify(brand?.id ?? ""),
  },
});
