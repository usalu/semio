import path from "node:path";
import { fileURLToPath } from "node:url";
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";
import { playgroundAssetVitePlugins, playgroundFlowWasmDevStubPlugin, playgroundSceneHostResolveAliases, resolveGisMapTileServeMode, semioAssetsVitePlugin, semioEmojiIndexHtmlVitePlugin, semioHostHtmlVitePlugin, semioViteProductionBuild, staticDirVitePlugin } from "../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🟦️vite-elements-assets.ts";
import { PLAYGROUND_BUILD_TARGETS } from "../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/🤖️generated/🟦️playgrounds.ts";
import { semioBackboneVitePlugin, semioBlobVitePlugin } from "../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts";
import { DEMONSTRATOR_ASSETS_DIR, DEMONSTRATOR_HOST, DEMONSTRATOR_PANES } from "./🟦️brand.ts";

const playDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(playDir, "../..");
const pluginModulesDir = path.join(playDir, "../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules");

//#region 🔖️DemonstratorUnionAssets
/** @emoji 🎪️ Registry rows for exactly this demonstrator's six panes — the union this page needs to
 * actually mount, not every playground variant in the monorepo (mirrors `os/dev`'s own `resolvedPlaygroundAssets`,
 * scoped down from its "studio serves everything" fallback since a demonstrator pane list is fixed). */
const demonstratorTargets = PLAYGROUND_BUILD_TARGETS.filter((target) => DEMONSTRATOR_PANES.some((pane) => pane.variant === target.variant));
const resolvedPlaygroundAssets = demonstratorTargets.flatMap((target) => target.assets);
/** @emoji 🔌️ `_vendor` (shared `🟨️host-shim.js` deps every plugin imports) plus each demonstrator pane's
 * own resolved plugin crate dir — mirrors `os/dev`'s single-variant `pluginModuleDirNames`, unioned
 * across all six panes instead of just one. */
const pluginModuleDirNames = ["_vendor", ...new Set(demonstratorTargets.map((target) => target.pluginId))];
//#endregion 🔖️DemonstratorUnionAssets

export default defineConfig({
  root: playDir,
  cacheDir: path.join(repoRoot, "node_modules/.vite-mit-bestand-demonstrator"),
  publicDir: path.join(playDir, "public"),
  assetsInclude: ["**/*.wasm"],
  resolve: {
    alias: [
      ...playgroundSceneHostResolveAliases(repoRoot),
      { find: "@semio-tech/ui-react", replacement: path.resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx") },
      { find: "@semio-tech/assets", replacement: path.resolve(repoRoot, "./🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📦️index.ts") },
      { find: "@semio-tech/ui-styling", replacement: path.resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript") },
      { find: "@semio-tech/infinite-canvas-react-renderer", replacement: path.resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/📦️packages/🟦️typescript/🟦️glue.tsx") },
      { find: "@semio-tech/infinite-world-r3f", replacement: path.resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/🟦️glue.tsx") },
      { find: "@semio-tech/framework-renderer-react", replacement: path.resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx") },
      { find: "@semio-tech/framework-core", replacement: path.resolve(repoRoot, "./🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts") },
      { find: "@semio-tech/framework-os-core", replacement: path.resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🟦️glue.ts") },
      { find: "/plugin-modules", replacement: pluginModulesDir },
    ],
    dedupe: ["react", "react-dom", "three", "@react-three/fiber", "@react-three/drei"],
  },
  server: {
    port: Number(process.env.MIT_BESTAND_DEMONSTRATOR_PORT ?? 6029),
    strictPort: true,
    fs: { allow: [repoRoot, pluginModulesDir] },
  },
  plugins: [
    ...semioHostHtmlVitePlugin(repoRoot, {
      title: "Entwerfen mit Bestand · Demonstrator",
      entry: "./📦️index.tsx",
      bodyClass: "h-screen w-screen overflow-hidden bg-background text-foreground",
      cnameHost: DEMONSTRATOR_HOST,
    }),
    semioEmojiIndexHtmlVitePlugin(playDir),
    playgroundFlowWasmDevStubPlugin(repoRoot),
    semioBackboneVitePlugin(),
    semioBlobVitePlugin(),
    ...semioAssetsVitePlugin(repoRoot),
    // 🔌️ Same reasoning as `os/dev`'s vite config: the bundler `resolve.alias` above only covers static
    // imports — plugins are also fetched at runtime via absolute-URL `import()`, which a production build
    // never bundles, so each union plugin dir needs its own static-dir copy into `dist/`.
    ...pluginModuleDirNames.flatMap((name) => staticDirVitePlugin(repoRoot, { kind: "static-dir", route: `/plugin-modules/${name}`, root: path.relative(repoRoot, path.join(pluginModulesDir, name)) })),
    staticDirVitePlugin(repoRoot, { kind: "static-dir", route: `/${DEMONSTRATOR_ASSETS_DIR}`, root: DEMONSTRATOR_ASSETS_DIR }),
    ...playgroundAssetVitePlugins(repoRoot, resolvedPlaygroundAssets, resolveGisMapTileServeMode(process.env.GIS_MAP_TILE_SERVE_MODE)),
    react(),
    tailwindcss(),
  ],
  optimizeDeps: {
    entries: [path.join(playDir, "🌐️index.html")],
    include: ["react-reconciler", "react-reconciler/constants", "three", "@react-three/fiber", "fuse.js"],
    exclude: ["playwright", "playwright-core", "chromium-bidi", "fsevents"],
  },
  build: semioViteProductionBuild(),
});
