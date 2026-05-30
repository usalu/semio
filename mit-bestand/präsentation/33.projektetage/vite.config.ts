// #region 🔌Adapters
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";
import {
	uiAssetsVitePlugin,
	playgroundIframeEmbedHeadersPlugin,
	playgroundStaticSiteBuildOptions,
} from "../../../ui/styling/vite-elements-assets.ts";
// #endregion 🔌Adapters

const dir = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(dir, "../../../");
const uiAssetsRoot = resolve(repoRoot, "ui/assets");
const uiReact = resolve(repoRoot, "ui/react/index.tsx");

export default defineConfig({
	root: dir,
	base: "./",
	publicDir: resolve(dir, "public"),
	plugins: [
		...uiAssetsVitePlugin(uiAssetsRoot),
		tailwindcss(),
		react(),
		playgroundIframeEmbedHeadersPlugin(),
	],
	build: playgroundStaticSiteBuildOptions(),
	server: {
		fs: { allow: [repoRoot] },
	},
	resolve: {
		alias: [{ find: "@ui/react", replacement: uiReact }],
	},
});
