import path from "node:path";
import { fileURLToPath } from "node:url";
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";
import { playgroundAssetVitePlugins, playgroundFlowWasmDevStubPlugin, playgroundSceneHostResolveAliases, resolveGisMapTileServeMode, semioAssetsVitePlugin, semioEmojiIndexHtmlVitePlugin, semioHostHtmlVitePlugin, semioViteProductionBuild, staticDirVitePlugin } from "../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🟦️vite-elements-assets.ts";
import { PLAYGROUND_BUILD_TARGETS } from "../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🟦️playgrounds.ts";
import { semioBackboneVitePlugin, semioBlobVitePlugin, semioPluginHotSwapVitePlugin } from "../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts";
import { EXTENSION_TARGETS, PLUGIN_BUILD_TARGETS } from "../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🟦️plugins.ts";
import { DEMONSTRATOR_ASSETS_DIR, DEMONSTRATOR_HOST, DEMONSTRATOR_PANES, demonstratorPaneRuntimeVariant } from "./🟦️brand.ts";

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
const pluginModulesDir = path.join(playDir, "../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules");

//#region 🔖️DemonstratorUnionAssets
/** @emoji 🎪️ Registry rows for exactly this demonstrator's six panes — the union this page needs to
 * actually mount, not every playground variant in the monorepo (mirrors `os/dev`'s own `resolvedPlaygroundAssets`,
 * scoped down from its "studio serves everything" fallback since a demonstrator pane list is fixed). */
const demonstratorRuntimeVariants = new Set(DEMONSTRATOR_PANES.flatMap((pane) => [pane.variant, demonstratorPaneRuntimeVariant(pane.variant)]));
const demonstratorTargets = PLAYGROUND_BUILD_TARGETS.filter((target) => demonstratorRuntimeVariants.has(target.variant));
const resolvedPlaygroundAssets = demonstratorTargets.flatMap((target) => target.assets);
/** @emoji 🔌️ `_vendor` (shared `🟨️host-shim.js` deps every plugin imports) plus each demonstrator pane's
 * own resolved plugin crate dir — mirrors `os/dev`'s single-variant `pluginModuleDirNames`, unioned
 * across all six panes instead of just one. */
/** @emoji 🔌️ Transitive `dependsOn` + consume-matched extensions of every pane's primary crate —
 * without these dirs the shell's PluginSource snapshot can't install cad/gis/puzzle/… and each pane's
 * `appId` (owned by those crates, not by `demonstrator`) never resolves. */
function demonstratorPluginModuleIds(rootPluginIds: readonly string[]): string[] {
  const catalog = [...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS];
  const byId = new Map(catalog.map((target) => [target.pluginId, target] as const));
  const selected = new Set(rootPluginIds);
  const queue = [...rootPluginIds];
  for (let index = 0; index < queue.length; index++) {
    const target = byId.get(queue[index]!);
    if (!target) continue;
    for (const dependency of target.dependsOn ?? []) {
      if (selected.has(dependency)) continue;
      selected.add(dependency);
      queue.push(dependency);
    }
    const consumes = new Set(target.consumes ?? []);
    if (consumes.size === 0) continue;
    for (const extension of EXTENSION_TARGETS) {
      if (selected.has(extension.pluginId)) continue;
      if (!(extension.contributes ?? []).some((tag) => consumes.has(tag))) continue;
      selected.add(extension.pluginId);
      queue.push(extension.pluginId);
    }
  }
  return [...selected];
}

const pluginModuleDirNames = ["_vendor", ...demonstratorPluginModuleIds([...new Set(demonstratorTargets.map((target) => target.pluginId))])];
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
      { find: "@semio-tech/framework", replacement: path.resolve(repoRoot, "./🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts") },
      { find: "@semio-tech/framework-os", replacement: path.resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🟦️glue.ts") },
      { find: "/plugin-modules", replacement: pluginModulesDir },
    ],
    dedupe: ["react", "react-dom", "three", "@react-three/fiber", "@react-three/drei"],
  },
  server: {
    port: Number(process.env.MIT_BESTAND_DEMONSTRATOR_PORT ?? 6029),
    strictPort: true,
    fs: { allow: [repoRoot, pluginModulesDir] },
    watch: {
      // Generated registry/session rewrites must not bounce Vite.
      ignored: ["**/📇️registry/🤖️generated/**", "**/🤖️generated/**", "**/.vscode/launch.json"],
    },
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
    semioPluginHotSwapVitePlugin(),
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
    exclude: ["playwright", "playwright-core", "chromium-bidi", "fsevents", ...FRAMEWORK_ENGINE_OPTIMIZE_DEPS_EXCLUDE],
  },
  build: semioViteProductionBuild(),
});
