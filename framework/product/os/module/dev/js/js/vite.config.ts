import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";
import { playgroundAssetVitePlugins, playgroundFlowWasmDevStubPlugin, playgroundSceneHostResolveAliases, resolveGisMapTileServeMode, semioBrandHtmlVitePlugins, staticDirVitePlugin, uiAssetsVitePlugin } from "../../../../../../module/ui/styling/vite-elements-assets.ts";
import { PLAYGROUND_BUILD_TARGETS } from "../../../plugin/registry/generated/playgrounds.ts";
import { isStudioPluginFilter } from "../../../plugin/registry/script.ts";
import { resolveShellBrandById } from "../brand/index.ts";
import { semioBackboneVitePlugin, semioBlobVitePlugin } from "../script.ts";

const configDir = path.dirname(fileURLToPath(import.meta.url));
const playDir = path.resolve(configDir, "..");
const repoRoot = path.resolve(playDir, "../../../../../..");
const pluginModulesDir = path.join(playDir, "plugin-modules");
const rendererModulesDir = path.join(playDir, "renderer-modules");
const renderer = process.env.SEMIO_RENDERER ?? "react";
const plugin = process.env.SEMIO_PLUGIN ?? process.env.PLAYGROUND_APP_KIND ?? "s";
const brandId = process.env.SEMIO_BRAND ?? PLAYGROUND_BUILD_TARGETS.find((target) => target.variant === plugin || target.aliases.includes(plugin))?.brand;
const brand = resolveShellBrandById(brandId);
const uiAssetsRoot = path.join(repoRoot, "framework/module/ui/asset");

//#region 🔖RegistryDrivenAssetsAndEngines
/** @emoji 🔌 Framework engine crates every react-renderer dev session needs regardless of the active
 * plugin (the node-graph/editor host engines back shared studio chrome, not any one app) — kept as a
 * literal baseline rather than per-plugin metadata, mirroring the equally-unconditional pre-registry
 * build in `os/dev/script.ts`'s `buildEngineWasm`. */
const FRAMEWORK_ENGINE_OPTIMIZE_DEPS_EXCLUDE = ["@semio-tech/framework-surface-node-graph-rs", "@semio-tech/framework-editor-rs", "@semio-tech/flow-core"];

/** @emoji 📦 Maps a registry `engines` crate path (e.g. `framework/module/surface/tiled-map/rs`) to its wasm-pack
 * npm package name — read from the crate's own sibling `package.json`, not derived from its path, so a
 * crate keeps optimizing correctly across restructures/moves without touching this file. */
function engineNpmPackage(cratePath: string): string {
  const manifestPath = path.join(repoRoot, cratePath, "package.json");
  const name = JSON.parse(readFileSync(manifestPath, "utf8")).name as string | undefined;
  if (!name) throw new Error(`missing "name" in ${manifestPath}`);
  return name;
}

const registryEngineOptimizeDepsExclude = [...new Set(PLAYGROUND_BUILD_TARGETS.flatMap((target) => target.engines))].map(engineNpmPackage);

/** @emoji 🗄️ Isolates dependency-optimizer state for concurrent playground variants and renderers. */
const playgroundCacheDir = path.join(repoRoot, "node_modules/.vite-os-dev", `${plugin}-${renderer}`);

/** @emoji 🚫 Keeps Node-only browser automation packages outside Vite's browser dependency optimizer. */
const nodeOnlyOptimizeDepsExclude = ["playwright", "playwright-core", "chromium-bidi", "fsevents"];

/** @emoji 🗂️ The active playground's declared asset needs — every playground's assets when unfiltered
 * (the "s" studio hub can open any app, so it needs every app's dev-time asset routes available), else
 * just the resolved variant's own `assets` row. */
const resolvedPlaygroundAssets = isStudioPluginFilter(plugin) ? PLAYGROUND_BUILD_TARGETS.flatMap((target) => target.assets) : (PLAYGROUND_BUILD_TARGETS.find((target) => target.variant === plugin)?.assets ?? []);

/** @emoji 🔌 The wasm plugin crate(s) a production build's `dist/plugin-modules/` needs to actually ship
 * — the "s" studio hub can open any app so it needs every built plugin crate; a single-variant build
 * (e.g. the Aggregator's "aggregator" → `puzzle`) needs only its own, plus the shared `_vendor` shim
 * deps every plugin's `host-shim.js` imports. Falls back to "every crate" for an unresolved/unknown
 * filter rather than shipping nothing. */
const resolvedPluginId = PLAYGROUND_BUILD_TARGETS.find((target) => target.variant === plugin || target.aliases.includes(plugin))?.pluginId;
const pluginModuleDirNames = isStudioPluginFilter(plugin) || !resolvedPluginId ? undefined : ["_vendor", resolvedPluginId];
//#endregion 🔖RegistryDrivenAssetsAndEngines

export default defineConfig({
  root: playDir,
  cacheDir: playgroundCacheDir,
  publicDir: path.join(playDir, "public"),
  assetsInclude: ["**/*.wasm"],
  // 🏷️ A brand's own `distDir` (e.g. the Aggregator's `mit-bestand/aggregator/dist`) keeps its build output
  // self-contained alongside its brand config/assets instead of the shared playground `dist/`.
  build: brand?.distDir ? { outDir: path.join(repoRoot, brand.distDir) } : undefined,
  resolve: {
    alias: [
      ...playgroundSceneHostResolveAliases(repoRoot),
      { find: "@semio-tech/ui-react", replacement: path.resolve(repoRoot, "framework/module/ui/js/react/index.tsx") },
      { find: "@semio-tech/ui-asset", replacement: path.resolve(repoRoot, "framework/module/ui/asset") },
      { find: "@semio-tech/ui-styling", replacement: path.resolve(repoRoot, "framework/module/ui/styling/js") },
      { find: "@semio-tech/infinite-cavas-react-renderer", replacement: path.resolve(repoRoot, "framework/product/os/module/infinite/canvas/react-renderer/index.tsx") },
      { find: "@semio-tech/infinite-world-r3f", replacement: path.resolve(repoRoot, "framework/product/os/module/infinite/world/r3f/index.tsx") },
      { find: "@semio-tech/framework-renderer-react", replacement: path.resolve(repoRoot, "framework/product/os/module/renderer/js/react/index.tsx") },
      { find: "@semio-tech/framework-renderer-wgpu", replacement: path.resolve(repoRoot, "framework/product/os/module/renderer/wgpu/index.ts") },
      { find: "@semio-tech/framework-core", replacement: path.resolve(repoRoot, "framework/js/index.ts") },
      { find: "@semio-tech/framework-os-core", replacement: path.resolve(repoRoot, "framework/product/os/js/index.ts") },
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
    playgroundFlowWasmDevStubPlugin(repoRoot),
    semioBackboneVitePlugin(),
    semioBlobVitePlugin(),
    ...uiAssetsVitePlugin(uiAssetsRoot),
    // 🔌 `resolve.alias`'s `/plugin-modules` entry above only covers *bundler* resolution (static imports
    // Vite can inline) — the shell also fetches wasm plugin modules at runtime via plain absolute-URL
    // `import()`s, which a production build never bundles. Without an explicit static-dir copy, a
    // production `dist/` simply never contains `plugin-modules/` at all, so every wasm plugin 404s once
    // deployed (only ever worked in dev, where the dev server can serve arbitrary filesystem paths).
    // Copied per-crate-dir rather than the whole tree so a single-variant build doesn't ship every plugin
    // crate this shared dev directory happens to have accumulated from other variants' past builds — see
    // `pluginModuleDirNames` above. (`renderer-modules/wgpu` needs no equivalent copy: it's a wholly
    // separate wgpu-renderer trunk build — `BuildScript.run` skips `vite build` entirely for that
    // renderer — and nothing in the react app ever fetches `/renderer-modules` at runtime.)
    ...(pluginModuleDirNames
      ? pluginModuleDirNames.flatMap((name) => staticDirVitePlugin(repoRoot, { kind: "static-dir", route: `/plugin-modules/${name}`, root: path.relative(repoRoot, path.join(pluginModulesDir, name)) }))
      : staticDirVitePlugin(repoRoot, { kind: "static-dir", route: "/plugin-modules", root: path.relative(repoRoot, pluginModulesDir) })),
    // 🏷️ A brand's own static assets (e.g. the Aggregator's funding/partner logos) mount at `/<assetsDir>`
    // alongside the shared `framework/ui/asset` mount above.
    ...(brand?.assetsDir ? staticDirVitePlugin(repoRoot, { kind: "static-dir", route: `/${brand.assetsDir}`, root: brand.assetsDir }) : []),
    ...semioBrandHtmlVitePlugins(repoRoot, brand),
    ...playgroundAssetVitePlugins(repoRoot, resolvedPlaygroundAssets, resolveGisMapTileServeMode(process.env.GIS_MAP_TILE_SERVE_MODE)),
    ...(renderer === "wgpu" ? [tailwindcss()] : [react(), tailwindcss()]),
  ],
  optimizeDeps: {
    entries: [path.join(playDir, "index.html")],
    include: ["react-reconciler", "react-reconciler/constants", "three", "@react-three/fiber", "fuse.js"],
    exclude: [...nodeOnlyOptimizeDepsExclude, ...(renderer === "wgpu" ? ["@semio-tech/framework-renderer-react"] : []), ...FRAMEWORK_ENGINE_OPTIMIZE_DEPS_EXCLUDE, ...registryEngineOptimizeDepsExclude],
  },
  define: {
    "import.meta.env.VITE_SEMIO_PLUGIN": JSON.stringify(process.env.SEMIO_PLUGIN ?? "s"),
    "import.meta.env.VITE_SEMIO_RENDERER": JSON.stringify(renderer),
    "import.meta.env.VITE_SEMIO_BRAND": JSON.stringify(brand?.id ?? ""),
  },
});
