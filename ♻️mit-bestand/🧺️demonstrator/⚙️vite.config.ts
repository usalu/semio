import path from "node:path";
import { fileURLToPath } from "node:url";
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";
import { playgroundAssetVitePlugins, playgroundFlowWasmDevStubPlugin, playgroundSceneHostResolveAliases, resolveGisMapTileServeMode, semioAssetsVitePlugin, semioEmojiIndexHtmlVitePlugin, semioHostHtmlVitePlugin, semioViteProductionBuild, staticDirVitePlugin } from "../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️.ts";
import { MODULE_EXTENSION_ROUTE, MODULE_PLUGIN_ROUTE } from "../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import { semioBackboneVitePlugin, semioBlobVitePlugin, semioPluginHotSwapVitePlugin } from "../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🔌️vite-plugins.ts";
import { defaultExtensionInstallRoot, semioExtensionStoreVitePlugin } from "../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📥️store.ts";
import { DEMONSTRATOR_ASSETS_DIR, DEMONSTRATOR_HOST, DEMONSTRATOR_RUNTIME_TARGETS, demonstratorRuntimeModuleLayout } from "./🔨️modules/🧩️runtime/🟦️.ts";

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
const resolvedPlaygroundAssets = DEMONSTRATOR_RUNTIME_TARGETS.flatMap((target) => target.assets);
/** @emoji 🔌️ Transitive runtime assets for every pane, split by the exact public roots encoded in the generated catalog. */
const { pluginModuleDirNames, extensionModuleDirNames } = demonstratorRuntimeModuleLayout([...new Set(DEMONSTRATOR_RUNTIME_TARGETS.map((target) => target.pluginId))]);
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
      { find: MODULE_PLUGIN_ROUTE, replacement: pluginModulesDir },
      { find: MODULE_EXTENSION_ROUTE, replacement: installedExtensionsDir },
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
    ...pluginModuleDirNames.flatMap((name) => staticDirVitePlugin(repoRoot, { kind: "static-dir", route: `${MODULE_PLUGIN_ROUTE}/${name}`, root: path.relative(repoRoot, path.join(pluginModulesDir, name)) })),
    // 🗄️ Catch-all behind the per-union entries above: modules outside this demonstrator's closure are
    // still fetched at runtime (stdio has no app descriptor but IS loaded), and without this they fall
    // through to the SPA fallback and fail the same way.
    staticDirVitePlugin(repoRoot, { kind: "static-dir", route: MODULE_PLUGIN_ROUTE, root: path.relative(repoRoot, pluginModulesDir) }),
    // 🧩️ The whole install root, not the computed closure: the generated runtime session lists EVERY
    // installed extension's `moduleUrl`, so serving only the transitive subset leaves the rest to the
    // SPA fallback, which answers descriptor fetches with HTML (`plugin.descriptor-invalid … returned
    // HTML`) and fails the shell boot. `os/dev`'s own config serves this route whole for the same reason.
    staticDirVitePlugin(repoRoot, { kind: "static-dir", route: MODULE_EXTENSION_ROUTE, root: path.relative(repoRoot, installedExtensionsDir) }),
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
