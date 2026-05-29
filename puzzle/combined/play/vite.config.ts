import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";
import { elementsAssetsVitePlugin } from "../../../styling/vite-elements-assets.ts";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, "../../../../../..");
const elementsAssetsRoot = path.resolve(__dirname, "../../../../assets");

export default defineConfig({
	root: __dirname,
	plugins: [elementsAssetsVitePlugin(elementsAssetsRoot), tailwindcss(), react()],
	build: {
		target: "esnext",
	},
	resolve: {
		alias: [
			{ find: "@elements/playground/react", replacement: path.resolve(__dirname, "../../../playground/react/index.tsx") },
			{ find: "@elements/playground", replacement: path.resolve(__dirname, "../../../playground/index.ts") },
			{ find: "@elements/ui", replacement: path.resolve(__dirname, "../../core/index.tsx") },
			{ find: "@elements/board", replacement: path.resolve(__dirname, "../../../board/index.tsx") },
			{ find: "@elements/scene", replacement: path.resolve(__dirname, "../../scene/index.tsx") },
		],
	},
	server: {
		fs: {
			allow: [repoRoot],
		},
	},
});
