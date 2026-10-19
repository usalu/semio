import { canonicalWasmBuildVitePlugin, viteCacheDirectory } from "../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🎮️playground/🟦️.ts";
import { buildModePolicy, readBuildMode } from "../🧰️framework/🔨️modules/🏃️process/🏗️build/🎚️mode/🟦️.ts";
import { semioViteBuild } from "../🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🎚️mode/🟦️.ts";
// This file has been automatically migrated to valid ESM format by Storybook.
// #region 🧲️Header
// 💻️ .storybook/main.ts
// Specs: Aggregate the existing package-local Storybook trees into one root monorepo Storybook.
// Summary: Configures the workspace Storybook with shared aliases, Markdown docs support, Vite `resolve.conditions` so `node_modules` `exports` resolve (`import` before `storybook`), a composable scope system driven by `.storybook/📖️stories/🧭️coordination/🟦️.ts` (`STORYBOOK_SCOPE` is a comma-separated list of hierarchical scope ids), and module-worker-safe Vite behavior.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import { existsSync } from "node:fs";
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";
import { dirname, join, resolve } from "node:path";

import type { StorybookConfig } from "@storybook/react-vite";
import { semioAssetsVitePlugin } from "../🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/🏗️builder/🌐️vite/🗃️assets/🟦️.ts";
import { createWorkspaceViteResolveConfig, findWorkspacePackages } from "../🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts";
import { playgroundFlowWasmDevStubPlugin } from "../🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🪝️playground-stubs/🟦️.ts";
import { hostBuildPlugins, uiTailwindBuildPlugins } from "../🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🛠️build-tooling/🟦️.ts";
import { resolveActiveScopes, buildScopeStoryGlobs, buildScopeAliases, buildExactAliasEntries, buildScopeWatchIgnores, type StoryScope } from "./📖️stories/🧭️coordination/🟦️.ts";
import { repositoryCachePartitionRootDataV1 } from "../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/💾️storage/🟦️.ts";
import { createAssetBuildPluginsV1 } from "../🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/🔍️resolver/🧭️dispatch/🟦️.ts";
import { PLAYGROUND_ASSET_PROVIDERS_V1 } from "../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🖼️assets/🧩️composition/🟦️.ts";
import { flowBrowserDeliveryV1 } from "../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/📦️publication/📋️contract/🟦️.ts";
import { builtPluginModulesRootIn } from "../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/♻️activation/🟦️.ts";
import { RUNTIME_ASSET_CHANNELS } from "../🧰️framework/🔨️modules/🖼️canvas/📦️packages/🟦️typescript/🔌️runtime-assets/🟦️.ts";

const require = createRequire(import.meta.url);
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const repoRootPath = resolve(__dirname, "..");
const storybookScope = process.env.STORYBOOK_SCOPE ?? "";

const uiReactDir = resolve(repoRootPath, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript");
const uiStylingDir = resolve(repoRootPath, "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript");
const assetsDir = resolve(repoRootPath, "🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript");

function toVitePath(value: string): string {
  return value.replaceAll("\\", "/");
}

function getAbsolutePath(value: string): string {
  try {
    return dirname(require.resolve(join(value, "package.json")));
  } catch {
    return dirname(require.resolve(join(repoRootPath, "node_modules", value, "package.json")));
  }
}

// #region 🔖️ScopeDerivation
/** 🗂️ Active scopes for this process — computed once, reused by `stories`, aliases, watch-ignores, defines, and lazy scope `vitePlugins`. */
const activeScopes: readonly StoryScope[] = resolveActiveScopes(storybookScope);
const activeScopeIds: readonly string[] = activeScopes.map((s) => s.id);

/** 🔗️ Irregular per-scope aliases + a fixed baseline of always-present workspace shortcuts (css subpaths, single-file entries) not worth registering per-scope.
 * `viteFinal` turns every key into an exact-match entry (`buildExactAliasEntries`), so the bare package aliases never swallow a subpath. `🟦️Interpreter/🟦️.tsx` reaches `ui-react/test` through a runtime `await import(...)` that Vite still resolves at build time, so a plain (non-test) storybook build fails without the `ui-react/test` alias; this mirrors the same pair in os/dev's builder. */
function buildStorybookAliases(): Record<string, string> {
  const baseline: Record<string, string> = {
    "@semio-tech/ui-react/test": toVitePath(join(uiReactDir, "🟦️.ts")),
    // 📦️ Point at the package's real entry FILE, not the directory: Vite's alias substitution is
    // literal, and it only auto-resolves a directory via a bare `index.*`, which an emoji-prefixed
    // `📦️index.tsx` is not. The taxonomy sweep is renaming these entries to `🟦️.tsx` and updates
    // `package.json`'s `exports` to this canonical implementation leaf.
    "@semio-tech/ui-react": toVitePath(join(uiReactDir, "🟦️.tsx")),
    "@semio-tech/ui-styling": toVitePath(uiStylingDir),
    "@semio-tech/assets": toVitePath(assetsDir),
  };
  const scopeAliases = buildScopeAliases(activeScopes, {});
  const resolved: Record<string, string> = { ...baseline };
  for (const [key, value] of Object.entries(scopeAliases)) {
    resolved[key] = value.startsWith("/") ? value : toVitePath(resolve(repoRootPath, value));
  }
  return resolved;
}
// #endregion 🔖️ScopeDerivation

const config: StorybookConfig = {
  stories: buildScopeStoryGlobs(activeScopes),
  // 🔌️ Serve the materialized plugin fleet at `/plugin-modules/`, the exact URL `OsBootHost` probes
  // (`/plugin-modules/<pluginId>/<wasmOut>.js`). Without it every `framework/os` plugin story renders
  // the shell's "plugin artifact missing" panel instead of booting. These are build OUTPUTS produced
  // by `dev`'s materialize step, not sources — storybook only serves whatever is already on disk and
  // never triggers a cargo build itself, and a directory that is not built yet is left out (a missing static directory aborts the whole Storybook build). The runtime asset channels (compiler fonts, label face, icon pack)
  // serve every wasm session story the fonts and icons Canvas no longer embeds.
  staticDirs: [{ from: builtPluginModulesRootIn(repoRootPath, readBuildMode(process.env.SEMIO_BUILD_MODE)), to: "/plugin-modules" }, ...RUNTIME_ASSET_CHANNELS.map(({ route, directory }) => ({ from: resolve(repoRootPath, directory), to: route }))].filter(({ from }) => existsSync(from)),
  addons: [getAbsolutePath("@storybook/addon-vitest"), getAbsolutePath("@storybook/addon-docs")],
  framework: {
    name: getAbsolutePath("@storybook/react-vite"),
    options: {},
  },
  docs: {},
  typescript: {
    reactDocgen: "react-docgen-typescript",
  },
  core: {
    disableTelemetry: true,
  },
  /** 📡️ Storybook 10.4's `changeDetection` builder-agnostic resolver defaults on and crashes (`Cannot read properties of undefined (reading 'buildError')`) against this repo's workspace alias set; keep it off until upstream stabilizes it. */
  features: {
    changeDetection: false,
  },
  async viteFinal(config, { configType }) {
    // ⚡️ Routes Storybook's Vite dependency-optimizer cache into the ONE shared cache root instead of
    // the Vite default (`node_modules/.vite`), so disk is bounded by the pruner rather than by `node_modules`.
    const buildMode = configType === "DEVELOPMENT" ? "prototype" : readBuildMode(process.env.SEMIO_BUILD_MODE);
    config.cacheDir = viteCacheDirectory(repoRootPath, { ...process.env, SEMIO_BUILD_MODE: buildMode });
    config.resolve = config.resolve || {};
    // #region 🔖️ResolvePackageExports
    /** ⏪️ SB 10’s resolver prefers `storybook`/`stories` export conditions; most deps only declare `import`/`require`, so `"."` fails. Put standard bundler conditions first. */
    const previousConditions = config.resolve.conditions ?? [];
    config.resolve.conditions = ["import", "module", "browser", "default", ...previousConditions.filter((c) => !["import", "module", "browser", "default"].includes(c))];
    // #endregion 🔖️ResolvePackageExports
    const workspaceResolve = createWorkspaceViteResolveConfig(repoRootPath);
    const aliasRecord: Record<string, string> = {
      ...buildStorybookAliases(),
      "vite/internal": resolve(repoRootPath, "node_modules/vite/dist/node/index.js"),
      "@semio-tech/framework-platform-core": resolve(repoRootPath, "🧰️framework/📦️packages/🟦️typescript/🟦️.ts"),
      "@semio-tech/framework-playground-core": resolve(repoRootPath, "🧰️framework/📦️packages/🟦️typescript/🟦️.ts"),
      "@semio-tech/framework-platform-renderer-react": resolve(repoRootPath, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx"),
      "@semio-tech/framework-playground-renderer-react": resolve(repoRootPath, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx"),
    };
    for (const item of workspaceResolve.resolve?.alias ?? []) {
      if (typeof item === "object" && item && "find" in item && "replacement" in item && typeof item.find === "string") {
        aliasRecord[item.find] = item.replacement;
      }
    }
    config.resolve.alias = buildExactAliasEntries(aliasRecord, config.resolve.alias);
    config.assetsInclude = [...[config.assetsInclude ?? []].flat(), "**/*.wasm"];
    config.server = config.server || {};
    config.server.allowedHosts = config.server.allowedHosts === true ? true : Array.from(new Set([...(config.server.allowedHosts ?? []), "127.0.0.1", "localhost"]));
    config.server.fs = {
      ...(config.server.fs || {}),
      allow: Array.from(new Set([...(config.server.fs?.allow || []), repoRootPath])),
    };
    const currentWatch = config.server.watch && typeof config.server.watch === "object" ? config.server.watch : {};
    const currentIgnored = currentWatch.ignored;
    const ignoredList = Array.isArray(currentIgnored) ? currentIgnored : currentIgnored ? [currentIgnored] : [];
    const scopeWatchIgnores = buildScopeWatchIgnores(activeScopes);
    config.server.watch = {
      ...currentWatch,
      usePolling: true,
      ignored: [...ignoredList, "**/storybook-static/**", "**/.nx/**", "**/.🧬semio/**", "**/.repo/**", "**/dist/**", "**/.git/**", "**/node_modules/**", ...scopeWatchIgnores],
    };

    config.plugins = config.plugins || [];
    const hasTailwindPlugin = config.plugins.some((plugin) => plugin && typeof plugin === "object" && "name" in plugin && plugin.name === "@tailwindcss/vite");
    if (!hasTailwindPlugin) {
      config.plugins.push(...hostBuildPlugins(uiTailwindBuildPlugins()));
    }
    const hasUiAssetsPlugin = config.plugins.some((plugin) => plugin && typeof plugin === "object" && "name" in plugin && plugin.name === "ui-assets-serve");
    if (!hasUiAssetsPlugin) {
      config.plugins.push(...hostBuildPlugins(semioAssetsVitePlugin(repoRootPath)));
    }
    if (configType !== "DEVELOPMENT") config.plugins.unshift(...hostBuildPlugins([canonicalWasmBuildVitePlugin(repoRootPath, buildMode)]));
    config.plugins.push(...hostBuildPlugins([playgroundFlowWasmDevStubPlugin(repoRootPath, flowBrowserDeliveryV1(repoRootPath, repositoryCachePartitionRootDataV1(repoRootPath, "products"), { version: 1, mode: buildMode, command: configType === "DEVELOPMENT" ? "serve" : "build" }))]));
    // #region 🔖️ScopeAssetsAndPlugins
    /** 🌐️ Static-dir / tile-proxy / mesh-collection assets declared by active scopes (e.g. `framework/os`'s `/plugin-modules`, `/renderer-modules`). */
    const scopeAssets = activeScopes.flatMap((s) => s.assets ?? []);
    if (scopeAssets.length > 0) {
      config.plugins.push(...hostBuildPlugins(createAssetBuildPluginsV1(repoRootPath, scopeAssets, PLAYGROUND_ASSET_PROVIDERS_V1)));
    }
    /** 🌐️ Lazy scope-gated Vite plugins (only imported when the owning scope is active). */
    for (const scope of activeScopes) {
      if (scope.vitePlugins) {
        config.plugins.push(...hostBuildPlugins(await scope.vitePlugins()));
      }
    }
    // #endregion 🔖️ScopeAssetsAndPlugins
    const indicesToRemove: number[] = [];
    for (let i = 0; i < config.plugins.length; i++) {
      const plugin: any = config.plugins[i];
      if (plugin === "@mdx-js/rollup" || (plugin && typeof plugin === "object" && plugin.name === "@mdx-js/rollup")) {
        indicesToRemove.push(i);
        continue;
      }
      if (plugin instanceof Promise) {
        try {
          const resolved: any = await plugin;
          if (resolved && typeof resolved === "object" && resolved.name === "storybook:mdx-plugin") {
            indicesToRemove.push(i);
          }
        } catch {}
      }
    }
    for (let i = indicesToRemove.length - 1; i >= 0; i--) {
      config.plugins.splice(indicesToRemove[i], 1);
    }

    config.optimizeDeps = config.optimizeDeps || {};
    config.optimizeDeps.include = [...(config.optimizeDeps.include || []), "golden-layout"];
    const optimizeExclude = new Set<string>([...(config.optimizeDeps.exclude || []), "@semio-tech/ui-react", "@semio-tech/infinite-canvas-react-renderer", ...findWorkspacePackages(repoRootPath), ...activeScopes.flatMap((s) => s.optimizeDepsExclude ?? [])]);
    config.optimizeDeps.exclude = Array.from(optimizeExclude);
    config.optimizeDeps.esbuildOptions = {
      ...(config.optimizeDeps.esbuildOptions || {}),
      target: "es2022",
      loader: { ...(config.optimizeDeps.esbuildOptions?.loader || {}), ".ts": "tsx" },
    };
    const currentRollup = config.build?.rollupOptions;
    config.build = { ...config.build, ...semioViteBuild(undefined, buildModePolicy(buildMode)) };
    config.build.target = "es2022";
    config.build.rollupOptions = { ...currentRollup, ...config.build.rollupOptions };
    const existingExternal = config.build.rollupOptions.external;
    config.build.rollupOptions.external = Array.isArray(existingExternal)
      ? [...existingExternal, /\.node$/]
      : [/\.node$/];
    const scopeDefines = {
      "import.meta.vitest": "undefined",
      __STORYBOOK_SCOPE__: JSON.stringify(storybookScope),
      __STORYBOOK_ACTIVE_SCOPES__: JSON.stringify(activeScopeIds),
    };
    if (configType === "DEVELOPMENT") {
      config.mode = "development";
      config.define = {
        ...config.define,
        "process.env.NODE_ENV": JSON.stringify("development"),
        ...scopeDefines,
      };
    } else {
      config.mode = "production";
      config.define = {
        ...config.define,
        "process.env.NODE_ENV": JSON.stringify("production"),
        ...scopeDefines,
      };
    }
    config.worker = {
      ...(config.worker || {}),
      format: "es",
    };

    return config;
  },
};

export default config;
