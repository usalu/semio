import path from "node:path";
import { fileURLToPath } from "node:url";
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";
import { cadFixtureVitePlugin, gisMapTilesVitePlugins, playgroundSceneHostResolveAliases, resolveGisMapTileServeMode, terrainTilesVitePlugins, uiAssetsVitePlugin, puzzle3dMeshesVitePlugin } from "../../../../../ui/styling/vite-elements-assets.ts";
import { semioBackboneVitePlugin, semioBlobVitePlugin } from "../script.ts";

const configDir = path.dirname(fileURLToPath(import.meta.url));
const playDir = path.resolve(configDir, "..");
const repoRoot = path.resolve(playDir, "../../../..");
const pluginModulesDir = path.join(playDir, "plugin-modules");
const rendererModulesDir = path.join(playDir, "renderer-modules");
const renderer = process.env.SEMIO_RENDERER ?? "react";
const plugin = process.env.SEMIO_PLUGIN ?? process.env.PLAYGROUND_APP_KIND ?? "s";
const uiAssetsRoot = path.join(repoRoot, "ui/asset");

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
    ...cadFixtureVitePlugin(repoRoot),
    ...puzzle3dMeshesVitePlugin(repoRoot),
    ...(plugin === "gis2d" ? gisMapTilesVitePlugins(repoRoot, resolveGisMapTileServeMode(process.env.GIS_MAP_TILE_SERVE_MODE)) : []),
    ...(plugin === "gis3d" ? terrainTilesVitePlugins(repoRoot, resolveGisMapTileServeMode(process.env.GIS_MAP_TILE_SERVE_MODE)) : []),
    ...(renderer === "wgpu" ? [tailwindcss()] : [react(), tailwindcss()]),
  ],
  optimizeDeps: {
    include: ["react-reconciler", "react-reconciler/constants", "three", "@react-three/fiber", "fuse.js"],
    exclude: [...(renderer === "wgpu" ? ["@semio-tech/framework-renderer-react"] : []), "@semio-tech/gis-2d-rs", "@semio-tech/gis-3d-rs", "@semio-tech/framework-graph-rs", "@semio-tech/framework-editor-rs", "@semio-tech/raster-rs"],
  },
  define: {
    "import.meta.env.VITE_SEMIO_PLUGIN": JSON.stringify(process.env.SEMIO_PLUGIN ?? "s"),
    "import.meta.env.VITE_SEMIO_RENDERER": JSON.stringify(renderer),
  },
});
