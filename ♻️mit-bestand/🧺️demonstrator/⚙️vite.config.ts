import path from "node:path";
import { fileURLToPath } from "node:url";
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";
import { playgroundAssetVitePlugins, playgroundFlowWasmDevStubPlugin, playgroundSceneHostResolveAliases, resolveGisMapTileServeMode, semioAssetsVitePlugin, semioEmojiIndexHtmlVitePlugin, semioHostHtmlVitePlugin, semioViteProductionBuild, staticDirVitePlugin } from "../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️.ts";
import { PLAYGROUND_BUILD_TARGETS } from "../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds.ts";
import { semioBackboneVitePlugin, semioBlobVitePlugin, semioPluginHotSwapVitePlugin } from "../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts";
import { defaultExtensionInstallRoot, semioExtensionStoreVitePlugin } from "../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📥️store.ts";
import { DEMONSTRATOR_ASSETS_DIR, DEMONSTRATOR_HOST, DEMONSTRATOR_PANES, demonstratorPaneRuntimeVariant } from "./🪧️brand.ts";
import { demonstratorRuntimeModuleLayout } from "./📜️script.ts";

const playDir = path.dirname(fileURLToPath(import.meta.url));

/** @emoji 🚫️ Keep wasm-pack engine packages out of Vite's dep optimizer — their `pkg/` entries are produced by `buildEngineWasm`. */
const FRAMEWORK_ENGINE_OPTIMIZE_DEPS_EXCLUDE = [
  "@semio-tech/framework-surface-rs",
  "@semio-tech/framework-editor-rs",
  "@semio-tech/framework-surface-node-graph-rs",
  "@semio-tech/framework-surface-board-2d-rs",
  "@semio-tech/flow-core",
];

const repoRoot = path.resolve(playDir, "../..");
const pluginModulesDir = path.join(playDir, "../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules");
const installedExtensionsDir = defaultExtensionInstallRoot(repoRoot);

//#region 🔖️DemonstratorUnionAssets
/** @emoji 🎪️ Registry rows for exactly this demonstrator's six panes — the union this page needs to
 * actually mount, not every playground variant in the monorepo (mirrors `os/dev`'s own `resolvedPlaygroundAssets`,
 * scoped down from its "studio serves everything" fallback since a demonstrator pane list is fixed). */
const demonstratorRuntimeVariants = new Set(DEMONSTRATOR_PANES.flatMap((pane) => [pane.variant, demonstratorPaneRuntimeVariant(pane.variant)]));
const demonstratorTargets = PLAYGROUND_BUILD_TARGETS.filter((target) => demonstratorRuntimeVariants.has(target.variant));
const resolvedPlaygroundAssets = demonstratorTargets.flatMap((target) => target.assets);
/** @emoji 🔌️ Transitive runtime assets for every pane, split by the exact public roots encoded in the generated catalog. */
const { pluginModuleDirNames, extensionModuleDirNames } = demonstratorRuntimeModuleLayout([...new Set(demonstratorTargets.map((target) => target.pluginId))]);
//#endregion 🔖️DemonstratorUnionAssets

export default defineConfig({
  root: playDir,
  cacheDir: path.join(repoRoot, "node_modules/.vite-mit-bestand-demonstrator"),
  publicDir: path.join(playDir, "public"),
  assetsInclude: ["**/*.wasm"],
  worker: { format: "es" },
  define: { "import.meta.vitest": "undefined" },
  resolve: {
    alias: [
      ...playgroundSceneHostResolveAliases(repoRoot),
      { find: "@semio-tech/ui-react/test", replacement: path.resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🖌️render.ts") },
      { find: "@semio-tech/ui-react/runtime", replacement: path.resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/⚛️runtime.ts") },
      { find: "@semio-tech/ui-react", replacement: path.resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx") },
      { find: "@semio-tech/assets", replacement: path.resolve(repoRoot, "./🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/🟦️.ts") },
      { find: "@semio-tech/ui-styling", replacement: path.resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript") },
      { find: "@semio-tech/infinite-canvas-react-renderer", replacement: path.resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/📦️packages/🟦️typescript/🟦️.tsx") },
      { find: "@semio-tech/infinite-world-r3f", replacement: path.resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/🟦️.tsx") },
      { find: "@semio-tech/framework-renderer-react", replacement: path.resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx") },
      { find: "@semio-tech/framework", replacement: path.resolve(repoRoot, "./🧰️framework/📦️packages/🟦️typescript/🟦️.ts") },
      { find: "@semio-tech/framework-os", replacement: path.resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🟦️.ts") },
      { find: "/plugin-modules", replacement: pluginModulesDir },
      { find: "/extensions", replacement: installedExtensionsDir },
    ],
    dedupe: ["react", "react-dom", "three", "@react-three/fiber", "@react-three/drei"],
  },
  server: {
    port: Number(process.env.MIT_BESTAND_DEMONSTRATOR_PORT ?? 6029),
    strictPort: true,
    fs: { allow: [repoRoot, pluginModulesDir, installedExtensionsDir] },
    watch: {
      // Generated registry/session rewrites must not bounce Vite.
      ignored: ["**/📇️registry/🤖️generated/**", "**/🤖️generated/**", "**/.vscode/launch.json"],
    },
  },
  plugins: [
    ...semioHostHtmlVitePlugin(repoRoot, {
      title: "Entwerfen mit Bestand · Demonstrator",
      entry: "./🟦️.tsx",
      bodyClass: "h-screen w-screen overflow-hidden bg-background text-foreground",
      cnameHost: DEMONSTRATOR_HOST,
    }),
    semioEmojiIndexHtmlVitePlugin(playDir),
    playgroundFlowWasmDevStubPlugin(repoRoot),
    semioBackboneVitePlugin(),
    semioBlobVitePlugin(),
    semioPluginHotSwapVitePlugin(),
    semioExtensionStoreVitePlugin({ installRoot: installedExtensionsDir, repoRoot }),
    ...semioAssetsVitePlugin(repoRoot),
    // 🔌️ Same reasoning as `os/dev`'s vite config: the bundler `resolve.alias` above only covers static
    // imports — plugins are also fetched at runtime via absolute-URL `import()`, which a production build
    // never bundles, so each union plugin dir needs its own static-dir copy into `dist/`.
    ...pluginModuleDirNames.flatMap((name) => staticDirVitePlugin(repoRoot, { kind: "static-dir", route: `/plugin-modules/${name}`, root: path.relative(repoRoot, path.join(pluginModulesDir, name)) })),
    ...extensionModuleDirNames.flatMap((name) => staticDirVitePlugin(repoRoot, { kind: "static-dir", route: `/extensions/${name}`, root: path.relative(repoRoot, path.join(installedExtensionsDir, name)) })),
    staticDirVitePlugin(repoRoot, { kind: "static-dir", route: `/${DEMONSTRATOR_ASSETS_DIR}`, root: DEMONSTRATOR_ASSETS_DIR }),
    ...playgroundAssetVitePlugins(repoRoot, resolvedPlaygroundAssets, resolveGisMapTileServeMode(process.env.GIS_MAP_TILE_SERVE_MODE)),
    react(),
    tailwindcss(),
  ],
  optimizeDeps: {
    entries: [path.join(playDir, "🌐️.html")],
    include: ["three", "@react-three/fiber"],
    exclude: ["playwright", "playwright-core", "chromium-bidi", "fsevents", ...FRAMEWORK_ENGINE_OPTIMIZE_DEPS_EXCLUDE],
  },
  build: semioViteProductionBuild(),
});
