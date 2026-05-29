// #region 🧲Header
/** @emoji 🌐 Vite plugin: serve and copy `elements/assets` at `/assets/*` (fonts, cursors, …). */
// #endregion 🧲Header

// #region 🔌Adapters
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { cpSync, createReadStream, existsSync, mkdirSync, statSync } from "node:fs";
import { isAbsolute, relative, resolve } from "node:path";
import type { Connect, Plugin } from "vite";
import { defineConfig, type UserConfig } from "vite";
// #endregion 🔌Adapters

//#region 🔖ViteElementsAssets
function contentTypeForElementsAsset(filePath: string): string | undefined {
	if (filePath.endsWith(".woff2")) {
		return "font/woff2";
	}
	if (filePath.endsWith(".svg")) {
		return "image/svg+xml";
	}
	if (filePath.endsWith(".wasm")) {
		return "application/wasm";
	}
	return undefined;
}

function createElementsAssetsMiddleware(assetsRoot: string): Connect.NextHandleFunction {
	const assetsRootResolved = resolve(assetsRoot);
	return (req, res, next) => {
		if (!req.url?.startsWith("/assets/")) {
			next();
			return;
		}
		const rel = decodeURIComponent(req.url.slice("/assets/".length).split(/[?#]/, 1)[0] ?? "");
		const filePath = resolve(assetsRootResolved, rel);
		const relToRoot = relative(assetsRootResolved, filePath);
		if (relToRoot.startsWith("..") || isAbsolute(relToRoot) || !existsSync(filePath) || !statSync(filePath).isFile()) {
			next();
			return;
		}
		const contentType = contentTypeForElementsAsset(filePath);
		if (contentType) {
			res.setHeader("Content-Type", contentType);
		}
		createReadStream(filePath).pipe(res);
	};
}

/** @emoji 🌐 Vite: serve and copy `ui/assets` at `/assets/*` for palette fonts and cursors. */
export function elementsAssetsVitePlugin(assetsRoot: string): Plugin[] {
	let viteRoot = process.cwd();
	const serveAssets = createElementsAssetsMiddleware(assetsRoot);
	return [
		{
			name: "elements-assets-serve",
			enforce: "pre",
			configureServer(server) {
				server.middlewares.use(serveAssets);
			},
			configurePreviewServer(server) {
				server.middlewares.use(serveAssets);
			},
		},
		{
			name: "elements-assets-build",
			apply: "build",
			enforce: "pre",
			configResolved(config) {
				viteRoot = config.root;
			},
			closeBundle() {
				if (!existsSync(assetsRoot)) {
					return;
				}
				const dest = resolve(viteRoot, "dist", "assets");
				mkdirSync(resolve(viteRoot, "dist"), { recursive: true });
				cpSync(assetsRoot, dest, { recursive: true });
			},
		},
	];
}

/** @emoji 🛝 Shared Vite preset for puzzle play harnesses (assets, renderer subpaths, workspace aliases). */
export type PlaygroundPlayViteOptions = {
	readonly playDir: string;
	readonly repoRoot: string;
	readonly extraAliases?: ReadonlyArray<{ readonly find: string | RegExp; readonly replacement: string }>;
	readonly extraPlugins?: readonly Plugin[];
	readonly watchIgnored?: readonly string[];
	readonly build?: UserConfig["build"];
	readonly server?: UserConfig["server"];
	readonly optimizeDeps?: UserConfig["optimizeDeps"];
	readonly resolveDedupe?: readonly string[];
};

/** @emoji 🛝 `defineConfig` for `@puzzle/*-play` Vite entries with consistent renderer and core aliases. */
export function createPlaygroundPlayViteConfig(options: PlaygroundPlayViteOptions) {
	const { playDir, repoRoot, extraAliases = [], extraPlugins = [], watchIgnored, build, server, optimizeDeps, resolveDedupe } =
		options;
	const elementsAssetsRoot = resolve(repoRoot, "ui/assets");
	const rendererRoot = resolve(repoRoot, "framework/playground/renderer/react");
	const playgroundCore = resolve(repoRoot, "framework/playground/core/core.ts");
	const uiReact = resolve(repoRoot, "ui/react/index.tsx");
	const rendererIndex = resolve(rendererRoot, "index.tsx");
	const rendererAliases: ReadonlyArray<{ readonly find: string | RegExp; readonly replacement: string }> = [
		{ find: "@framework/playground/renderer/react/shell", replacement: rendererIndex },
		{ find: "@framework/playground/renderer/react/boot", replacement: rendererIndex },
		{ find: "@framework/playground/renderer/react/puzzle/board", replacement: resolve(rendererRoot, "puzzle/board-play-host.tsx") },
		{ find: "@framework/playground/renderer/react/puzzle/scene", replacement: resolve(rendererRoot, "puzzle/scene-play-host.tsx") },
		{ find: "@framework/playground/renderer/react/puzzle/topology", replacement: resolve(rendererRoot, "puzzle/topology-play-host.tsx") },
		{ find: "@framework/playground/renderer/react", replacement: rendererIndex },
		{ find: "@framework/playground", replacement: playgroundCore },
		{ find: "@ui/react", replacement: uiReact },
		{ find: "@puzzle/2d/play", replacement: resolve(repoRoot, "puzzle/2d/play/index.ts") },
		{ find: "@puzzle/3d/play", replacement: resolve(repoRoot, "puzzle/3d/play/index.ts") },
		{ find: "@puzzle/5d/play", replacement: resolve(repoRoot, "puzzle/5d/play/index.ts") },
		{ find: "@puzzle/2d/react", replacement: resolve(repoRoot, "puzzle/2d/react/index.tsx") },
		{ find: "@puzzle/3d/react", replacement: resolve(repoRoot, "puzzle/3d/react/index.tsx") },
		{ find: "@puzzle/5d/react", replacement: resolve(repoRoot, "puzzle/5d/react/index.tsx") },
	];
	return defineConfig({
		root: playDir,
		plugins: [...elementsAssetsVitePlugin(elementsAssetsRoot), tailwindcss(), react(), ...extraPlugins],
		build: { target: "esnext", ...build },
		server: {
			fs: { allow: [repoRoot] },
			...(watchIgnored ? { watch: { ignored: watchIgnored } } : {}),
			...server,
		},
		resolve: {
			alias: [...rendererAliases, ...extraAliases],
			...(resolveDedupe ? { dedupe: [...resolveDedupe] } : {}),
		},
		...(optimizeDeps ? { optimizeDeps } : {}),
	});
}
//#endregion 🔖ViteElementsAssets
