// #region 🧲Header
/** @emoji 🌐 Vite plugin: serve and copy `elements/assets` at `/assets/*` (fonts, cursors, …). */
// #endregion 🧲Header

import { cpSync, createReadStream, existsSync, mkdirSync, statSync } from "node:fs";
import { isAbsolute, relative, resolve } from "node:path";
import type { Connect } from "vite";
import type { Plugin } from "vite";

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
//#endregion 🔖ViteElementsAssets
