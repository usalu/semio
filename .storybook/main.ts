// This file has been automatically migrated to valid ESM format by Storybook.
// #region 🧲Header
// 💻 .storybook/main.ts
// Specs: Aggregate the existing package-local Storybook trees into one root monorepo Storybook.
// Summary: Configures the workspace Storybook with shared aliases, MDX support, Vite `resolve.conditions` so `node_modules` `exports` resolve (`import` before `storybook`), scope-aware dev slices via `STORYBOOK_SCOPE`, and module-worker-safe Vite behavior.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";
import { dirname, join, resolve } from "node:path";

import tailwindcss from "@tailwindcss/vite";
import type { StorybookConfig } from "@storybook/react-vite";
import rehypeAutolinkHeadings from "rehype-autolink-headings";
import { uiAssetsVitePlugin } from "../ui/styling/vite-elements-assets.ts";
import rehypeSlug from "rehype-slug";
import remarkFrontmatter from "remark-frontmatter";
import remarkGfm from "remark-gfm";
import remarkMdxFrontmatter from "remark-mdx-frontmatter";

const require = createRequire(import.meta.url);
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const repoRootPath = resolve(__dirname, "..");
const storybookScope = process.env.STORYBOOK_SCOPE ?? "";
const storybookScopePrefix = storybookScope ? `${storybookScope}/` : "";
const productionStorySlices = (process.env.STORYBOOK_PRODUCTION_SLICES ?? "")
	.split(",")
	.map((slice) => slice.trim())
	.filter(Boolean);
const productionSliceBuild = productionStorySlices.length > 0;

const uiReactDir = resolve(repoRootPath, "ui/react");
const uiStylingDir = resolve(repoRootPath, "ui/styling/js");
const frameworkPlaygroundDir = resolve(repoRootPath, "framework/product/playground/core");
const frameworkPlaygroundReactDir = resolve(repoRootPath, "framework/product/playground/renderer/react");
const puzzle2dReactDir = resolve(repoRootPath, "puzzle/2d/react");
const puzzle3dReactDir = resolve(repoRootPath, "puzzle/3d/react");
const puzzle5dReactDir = resolve(repoRootPath, "puzzle/5d/react");
const composeJsDir = resolve(repoRootPath, "compose/client/lib/js");
const composeRsWasmEntryPath = resolve(repoRootPath, "compose/client/lib/rs/pkg/compose.js");
const composeAssetsDir = resolve(repoRootPath, "compose/asset");
const composeFixturesDir = resolve(repoRootPath, "compose/fixture");
const puzzleAssetsDir = resolve(repoRootPath, "puzzle/asset");
const composeAlgorithmsEntryPath = resolve(repoRootPath, "compose/dev/algorithm/index.ts");
const uiAssetsRootPath = resolve(repoRootPath, "ui/asset");

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

/** @emoji 🎯 True when `STORYBOOK_SCOPE` is unset (full Storybook) or matches `prefix` / `prefix/…`. */
function storybookScopeMatches(prefix: string): boolean {
	if (!storybookScope) return true;
	return storybookScope === prefix || storybookScope.startsWith(`${prefix}/`);
}

/** @emoji 🎯 Active storybook slice for ui / puzzle / compose stacks. */
function storybookSliceActive(prefix: string): boolean {
	if (storybookScope) return storybookScopeMatches(prefix);
	if (productionSliceBuild) return productionStorySlices.includes(prefix);
	return true;
}

function buildStoryGlobs(): string[] {
	if (storybookScope) {
		return [`./stories/${storybookScopePrefix}**/*.stories.@(js|jsx|mjs|ts|tsx|mdx)`];
	}
	if (productionSliceBuild) {
		return productionStorySlices.map((slice) => `./stories/${slice}/**/*.stories.@(js|jsx|mjs|ts|tsx|mdx)`);
	}
	return [`./stories/**/*.stories.@(js|jsx|mjs|ts|tsx|mdx)`];
}

const loadUiStack = storybookSliceActive("ui");
const loadPuzzleStack = storybookSliceActive("puzzle");
const loadComposeStack = storybookSliceActive("compose");

function buildStorybookAliases(): Record<string, string> {
	const alias: Record<string, string> = {};
	if (loadUiStack || loadPuzzleStack) {
		alias["@semio-tech/puzzle-asset"] = toVitePath(puzzleAssetsDir);
		alias["@semio-tech/ui-react"] = toVitePath(uiReactDir);
		alias["@semio-tech/ui-styling"] = toVitePath(uiStylingDir);
		alias["@semio-tech/framework-playground-core"] = toVitePath(frameworkPlaygroundDir);
		alias["@semio-tech/framework-playground-renderer-react"] = toVitePath(frameworkPlaygroundReactDir);
		alias["@semio-tech/puzzle-2d-react"] = toVitePath(puzzle2dReactDir);
		alias["@semio-tech/puzzle-3d-react"] = toVitePath(puzzle3dReactDir);
		alias["@semio-tech/puzzle-5d-react"] = toVitePath(puzzle5dReactDir);
		alias["@semio-tech/infinite-cavas-react-renderer"] = toVitePath(resolve(repoRootPath, "infinite/cavas/react-renderer/index.tsx"));
		alias["@elements/ui/globals.css"] = toVitePath(resolve(uiReactDir, "globals.css"));
		alias["@semio-tech/coda-desktop/renderer"] = toVitePath(resolve(repoRootPath, "coda/client/ui/desktop/renderer.tsx"));
	}
	if (loadComposeStack) {
		alias["@compose/ui"] = toVitePath(uiReactDir);
		alias["@compose/ui/globals.css"] = toVitePath(resolve(uiReactDir, "globals.css"));
		alias["@semio-tech/compose-react"] = toVitePath(composeJsDir);
		alias["@semio-tech/compose-js"] = toVitePath(composeJsDir);
		alias["@semio-tech/compose-rs-wasm"] = toVitePath(composeRsWasmEntryPath);
		alias["@semio-tech/compose-asset"] = toVitePath(composeAssetsDir);
		alias["@semio-tech/compose-fixture"] = toVitePath(composeFixturesDir);
		alias["@semio-tech/compose-algorithm"] = toVitePath(composeAlgorithmsEntryPath);
		alias["@semio-tech/ui-react"] = toVitePath(uiReactDir);
		alias["@semio-tech/ui-styling"] = toVitePath(uiStylingDir);
	}
	return alias;
}

function buildScopeWatchIgnores(): string[] {
	if (!storybookScope) return [];
	if ((loadUiStack || loadPuzzleStack) && !loadComposeStack) {
		return ["**/compose/**", "**/coda/**", "**/cad/**", "**/reuse/**", "**/mit-bestand/**"];
	}
	if (loadComposeStack && !loadUiStack && !loadPuzzleStack) {
		return ["**/coda/**", "**/cad/**", "**/reuse/**", "**/mit-bestand/**"];
	}
	return [];
}

const config: StorybookConfig = {
	stories: buildStoryGlobs(),
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
	async viteFinal(config, { configType }) {
		config.resolve = config.resolve || {};
		// #region 🔖ResolvePackageExports
		/** SB 10’s resolver prefers `storybook`/`stories` export conditions; most deps only declare `import`/`require`, so `"."` fails. Put standard bundler conditions first. */
		const previousConditions = config.resolve.conditions ?? [];
		config.resolve.conditions = [
			"import",
			"module",
			"browser",
			"default",
			...previousConditions.filter((c) => !["import", "module", "browser", "default"].includes(c)),
		];
		// #endregion 🔖ResolvePackageExports
		config.resolve.alias = {
			...(config.resolve.alias || {}),
			...buildStorybookAliases(),
		};
		config.assetsInclude = [...(config.assetsInclude ?? []), "**/*.wasm"];
		config.server = config.server || {};
		config.server.allowedHosts = Array.from(new Set([...(config.server.allowedHosts || []), "127.0.0.1", "localhost"]));
		config.server.fs = {
			...(config.server.fs || {}),
			allow: Array.from(new Set([...(config.server.fs?.allow || []), repoRootPath])),
		};
		const currentWatch = config.server.watch && typeof config.server.watch === "object" ? config.server.watch : {};
		const currentIgnored = currentWatch.ignored;
		const ignoredList = Array.isArray(currentIgnored) ? currentIgnored : currentIgnored ? [currentIgnored] : [];
		const scopeWatchIgnores = buildScopeWatchIgnores();
		config.server.watch = {
			...currentWatch,
			usePolling: true,
			ignored: [
				...ignoredList,
				"**/storybook-static/**",
				"**/.nx/**",
				"**/.repo/**",
				"**/dist/**",
				"**/.git/**",
				"**/node_modules/**",
				...scopeWatchIgnores,
			],
		};

		config.plugins = config.plugins || [];
		const hasTailwindPlugin = config.plugins.some(
			(plugin) => plugin && typeof plugin === "object" && "name" in plugin && plugin.name === "@tailwindcss/vite",
		);
		if (!hasTailwindPlugin) {
			config.plugins.push(...tailwindcss());
		}
		const hasUiAssetsPlugin = config.plugins.some(
			(plugin) => plugin && typeof plugin === "object" && "name" in plugin && plugin.name === "ui-assets-serve",
		);
		if (!hasUiAssetsPlugin) {
			config.plugins.push(...uiAssetsVitePlugin(uiAssetsRootPath));
		}
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

		const mdx = await import("@mdx-js/rollup");
		config.plugins.push(
			mdx.default({
				remarkPlugins: [remarkGfm, remarkFrontmatter, remarkMdxFrontmatter],
				rehypePlugins: [rehypeSlug, rehypeAutolinkHeadings],
			}),
		);

		config.optimizeDeps = config.optimizeDeps || {};
		config.optimizeDeps.include = [...(config.optimizeDeps.include || []), "golden-layout"];
		const optimizeExclude = new Set<string>([
			...(config.optimizeDeps.exclude || []),
			"@semio-tech/ui-react",
			"@semio-tech/framework-playground-core",
			"@semio-tech/framework-playground-renderer-react",
			"@semio-tech/puzzle-2d-react",
			"@semio-tech/infinite-cavas-react-renderer",
		]);
		if (loadComposeStack) {
			optimizeExclude.add("@compose/ui");
			optimizeExclude.add("@semio-tech/compose-react");
			optimizeExclude.add("@semio-tech/compose-js");
			optimizeExclude.add("@semio-tech/compose-asset");
		}
		config.optimizeDeps.exclude = Array.from(optimizeExclude);
		config.optimizeDeps.esbuildOptions = {
			...(config.optimizeDeps.esbuildOptions || {}),
			target: "es2022",
		};
		config.build = config.build || {};
		config.build.target = "es2022";
		if (configType === "DEVELOPMENT") {
			config.mode = "development";
			config.define = {
				...config.define,
				"process.env.NODE_ENV": JSON.stringify("development"),
				__STORYBOOK_SCOPE__: JSON.stringify(storybookScope),
				__STORYBOOK_LOAD_UI__: JSON.stringify(loadUiStack),
				__STORYBOOK_LOAD_PUZZLE__: JSON.stringify(loadPuzzleStack),
				__STORYBOOK_LOAD_COMPOSE__: JSON.stringify(loadComposeStack),
			};
		} else {
			config.mode = "production";
			config.define = {
				...config.define,
				"process.env.NODE_ENV": JSON.stringify("production"),
				__STORYBOOK_SCOPE__: JSON.stringify(storybookScope),
				__STORYBOOK_LOAD_UI__: JSON.stringify(loadUiStack),
				__STORYBOOK_LOAD_PUZZLE__: JSON.stringify(loadPuzzleStack),
				__STORYBOOK_LOAD_COMPOSE__: JSON.stringify(loadComposeStack),
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
