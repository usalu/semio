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

const uiReactDir = resolve(repoRootPath, "ui/react");
const uiStylingDir = resolve(repoRootPath, "ui/styling/js");
const widgetsDir = resolve(repoRootPath, "widgets");
const frameworkPlaygroundDir = resolve(repoRootPath, "framework/product/playground/core");
const frameworkPlaygroundReactDir = resolve(repoRootPath, "framework/product/playground/renderer/react");
const puzzle2dReactDir = resolve(repoRootPath, "puzzle/2d/react");
const puzzle3dReactDir = resolve(repoRootPath, "puzzle/3d/react");
const puzzle5dReactDir = resolve(repoRootPath, "puzzle/5d/react");
const semioJsDir = resolve(repoRootPath, "semio/client/lib/js");
const semioRsWasmEntryPath = resolve(repoRootPath, "semio/client/lib/rs/pkg/semio.js");
const semioAssetsDir = resolve(repoRootPath, "semio/assets");
const semioFixturesDir = resolve(repoRootPath, "semio/fixtures");
const puzzleAssetsDir = resolve(repoRootPath, "puzzle/assets");
const semioAlgorithmsEntryPath = resolve(repoRootPath, "semio/dev/algorithms/index.ts");
const uiAssetsRootPath = resolve(repoRootPath, "ui/assets");

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

function packageExists(value: string): boolean {
	try {
		require.resolve(join(value, "package.json"));
		return true;
	} catch {
		try {
			require.resolve(join(repoRootPath, "node_modules", value, "package.json"));
			return true;
		} catch {
			return false;
		}
	}
}

/** @emoji 🎯 True when `STORYBOOK_SCOPE` is unset (full Storybook) or matches `prefix` / `prefix/…`. */
function storybookScopeMatches(prefix: string): boolean {
	if (!storybookScope) return true;
	return storybookScope === prefix || storybookScope.startsWith(`${prefix}/`);
}

const loadUiStack = storybookScopeMatches("ui");
const loadPuzzleStack = storybookScopeMatches("puzzle");
const loadSemioStack = storybookScopeMatches("semio");
const testLibraryStubPrefix = "\0storybook-test-library-stub:";
const storybookTestLibraryStubs = new Set(["@testing-library/react", "@testing-library/user-event"]);

function buildStorybookAliases(): Record<string, string> {
	const alias: Record<string, string> = {};
	if (loadUiStack || loadPuzzleStack) {
		alias["@puzzle/assets"] = toVitePath(puzzleAssetsDir);
		alias["@ui/react"] = toVitePath(uiReactDir);
		alias["@ui/styling"] = toVitePath(uiStylingDir);
		alias["@widgets/react/fixtures"] = toVitePath(resolve(widgetsDir, "fixtures/index.ts"));
		alias["@widgets/react"] = toVitePath(resolve(widgetsDir, "index.tsx"));
		alias["@framework/playground/core"] = toVitePath(frameworkPlaygroundDir);
		alias["@framework/playground/renderer/react"] = toVitePath(frameworkPlaygroundReactDir);
		alias["@puzzle/2d/react"] = toVitePath(puzzle2dReactDir);
		alias["@puzzle/3d/react"] = toVitePath(puzzle3dReactDir);
		alias["@puzzle/5d/react"] = toVitePath(puzzle5dReactDir);
		alias["@coda/desktop/renderer"] = toVitePath(resolve(repoRootPath, "coda/client/ui/desktop/renderer.tsx"));
	}
	if (loadSemioStack) {
		alias["@semio/ui"] = toVitePath(uiReactDir);
		alias["@semio/ui/globals.css"] = toVitePath(resolve(uiReactDir, "globals.css"));
		alias["@semio/react"] = toVitePath(semioJsDir);
		alias["@semio/js"] = toVitePath(semioJsDir);
		alias["@semio/rs-wasm"] = toVitePath(semioRsWasmEntryPath);
		alias["@semio/assets"] = toVitePath(semioAssetsDir);
		alias["@semio/fixtures"] = toVitePath(semioFixturesDir);
		alias["@semio/algorithms"] = toVitePath(semioAlgorithmsEntryPath);
		alias["@ui/react"] = toVitePath(uiReactDir);
		alias["@ui/styling"] = toVitePath(uiStylingDir);
		alias["@widgets/react/fixtures"] = toVitePath(resolve(widgetsDir, "fixtures/index.ts"));
		alias["@widgets/react"] = toVitePath(resolve(widgetsDir, "index.tsx"));
	}
	return alias;
}

function buildScopeWatchIgnores(): string[] {
	if (!storybookScope) return [];
	if ((loadUiStack || loadPuzzleStack) && !loadSemioStack) {
		return ["**/semio/**", "**/coda/**", "**/cad/**", "**/reuse/**", "**/mit-bestand/**"];
	}
	if (loadSemioStack && !loadUiStack && !loadPuzzleStack) {
		return ["**/coda/**", "**/cad/**", "**/reuse/**", "**/mit-bestand/**"];
	}
	return [];
}

/** @emoji 🧪 Resolves in-source test-only imports when Storybook scans source files. */
function storybookTestLibraryStubPlugin() {
	return {
		name: "storybook-test-library-stubs",
		resolveId(id: string) {
			if (storybookTestLibraryStubs.has(id)) return `${testLibraryStubPrefix}${id}`;
			return null;
		},
		load(id: string) {
			if (!id.startsWith(testLibraryStubPrefix)) return null;
			if (id.endsWith("@testing-library/user-event")) return "export default {};";
			return `
export const screen = {};
export const fireEvent = {};
export async function waitFor(callback) { return callback(); }
export function render() { throw new Error("Storybook test-library stub: render is unavailable."); }
`;
		},
	};
}

const config: StorybookConfig = {
	stories: [`./stories/${storybookScopePrefix}**/*.stories.@(js|jsx|mjs|ts|tsx|mdx)`],
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
		config.plugins.push(storybookTestLibraryStubPlugin());
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
		if (packageExists("golden-layout")) {
			config.optimizeDeps.include = Array.from(new Set([...(config.optimizeDeps.include || []), "golden-layout"]));
		}
		const optimizeExclude = new Set<string>([
			...(config.optimizeDeps.exclude || []),
			"@ui/react",
			"@widgets/react",
			"@framework/playground/core",
			"@framework/playground/renderer/react",
			"@puzzle/2d/react",
		]);
		if (loadSemioStack) {
			optimizeExclude.add("@semio/ui");
			optimizeExclude.add("@semio/react");
			optimizeExclude.add("@semio/js");
			optimizeExclude.add("@semio/assets");
		}
		config.optimizeDeps.exclude = Array.from(optimizeExclude);
		config.optimizeDeps.esbuildOptions = {
			...(config.optimizeDeps.esbuildOptions || {}),
			target: "es2022",
		};
		config.build = config.build || {};
		config.build.target = "es2022";
		config.build.rollupOptions = {
			...(config.build.rollupOptions || {}),
			external: Array.from(
				new Set([
					...((Array.isArray(config.build.rollupOptions?.external) ? config.build.rollupOptions.external : []) as string[]),
					"@testing-library/react",
					"@testing-library/user-event",
				]),
			),
		};
		if (configType === "DEVELOPMENT") {
			config.mode = "development";
			config.define = {
				...config.define,
				"process.env.NODE_ENV": JSON.stringify("development"),
				"import.meta.vitest": "undefined",
				__STORYBOOK_SCOPE__: JSON.stringify(storybookScope),
				__STORYBOOK_LOAD_UI__: JSON.stringify(loadUiStack),
				__STORYBOOK_LOAD_PUZZLE__: JSON.stringify(loadPuzzleStack),
				__STORYBOOK_LOAD_SEMIO__: JSON.stringify(loadSemioStack),
			};
		} else {
			config.mode = "production";
			config.define = {
				...config.define,
				"process.env.NODE_ENV": JSON.stringify("production"),
				"import.meta.vitest": "undefined",
				__STORYBOOK_SCOPE__: JSON.stringify(""),
				__STORYBOOK_LOAD_UI__: JSON.stringify(true),
				__STORYBOOK_LOAD_PUZZLE__: JSON.stringify(true),
				__STORYBOOK_LOAD_SEMIO__: JSON.stringify(true),
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
