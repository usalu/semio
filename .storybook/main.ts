// #region 🧲Header
// 💻 .storybook/main.ts
// Specs: Aggregate the existing package-local Storybook trees into one root monorepo Storybook.
// Summary: Configures the workspace Storybook with shared aliases, MDX support, and module-worker-safe Vite behavior.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";
import { dirname, join, resolve } from "node:path";

import type { StorybookConfig } from "@storybook/react-vite";
import rehypeAutolinkHeadings from "rehype-autolink-headings";
import rehypeSlug from "rehype-slug";
import remarkFrontmatter from "remark-frontmatter";
import remarkGfm from "remark-gfm";
import remarkMdxFrontmatter from "remark-mdx-frontmatter";

const require = createRequire(import.meta.url);
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const repoRootPath = resolve(__dirname, "..");
const elementsUiDir = resolve(repoRootPath, "elements/client/lib/react");
const semioUiDir = resolve(repoRootPath, "semio/client/lib/react/rendering");
const semioReactEntryPath = resolve(repoRootPath, "semio/client/lib/react/logic/index.tsx");
const semioJsEntryPath = resolve(repoRootPath, "semio/client/lib/js/index.ts");
const semioRsWasmEntryPath = resolve(repoRootPath, "semio/client/lib/rs/pkg/semio.js");
const semioAssetsDir = resolve(repoRootPath, "semio/assets");
const semioAlgorithmsEntryPath = resolve(repoRootPath, "semio/dev/algorithms/index.ts");

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

const config: StorybookConfig = {
	stories: ["./stories/**/*.stories.@(js|jsx|mjs|ts|tsx|mdx)"],
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
		config.resolve.alias = {
			...(config.resolve.alias || {}),
			"@elements/ui": toVitePath(elementsUiDir),
			"@elements/ui/elements": toVitePath(resolve(elementsUiDir, "index.tsx")),
			"@semio/ui": toVitePath(semioUiDir),
			"@semio/react": toVitePath(semioReactEntryPath),
			"@semio/js": toVitePath(semioJsEntryPath),
			"@semio/rs-wasm": toVitePath(semioRsWasmEntryPath),
			"@semio/assets": toVitePath(semioAssetsDir),
			"@semio/algorithms": toVitePath(semioAlgorithmsEntryPath),
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
			],
		};

		config.plugins = config.plugins || [];
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
		config.optimizeDeps.exclude = Array.from(
			new Set([...(config.optimizeDeps.exclude || []), "@semio/ui", "@semio/react", "@semio/js", "@semio/assets", "@elements/ui", "@elements/ui/elements"]),
		);
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
			};
		} else {
			config.mode = "production";
			config.define = {
				...config.define,
				"process.env.NODE_ENV": JSON.stringify("production"),
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
