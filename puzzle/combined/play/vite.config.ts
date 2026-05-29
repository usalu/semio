import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";
import { elementsAssetsVitePlugin } from "../../../../ui/styling/vite-elements-assets.ts";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, "../../../../../");
const elementsAssetsRoot = path.resolve(__dirname, "../../../../ui/assets");

export default defineConfig({
	root: __dirname,
	plugins: [elementsAssetsVitePlugin(elementsAssetsRoot), tailwindcss(), react()],
	build: {
		target: "esnext",
	},
	resolve: {
		alias: [
			{ find: "@framework/playground-react", replacement: path.resolve(__dirname, "../../../framework/playground/renderer/react/index.tsx") },
			{ find: "@framework/playground", replacement: path.resolve(__dirname, "../../../framework/playground/core/index.ts") },
			{ find: "@ui/react", replacement: path.resolve(__dirname, "../../../ui/react/index.tsx") },
			{ find: "@puzzle/board", replacement: path.resolve(__dirname, "../../2d/index.tsx") },
			{ find: "@puzzle/scene", replacement: path.resolve(__dirname, "../../3d/index.tsx") },
		],
	},
	server: {
		fs: {
			allow: [repoRoot],
		},
	},
});
