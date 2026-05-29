// #region 🧲Header
// 💻 elements/lib/board/play/vite.config.ts — Vite dev/build for the board multi-pane play harness.
// #endregion 🧲Header

import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, "../../../../../");

export default defineConfig({
	root: __dirname,
	plugins: [tailwindcss(), react()],
	build: {
		target: "esnext",
	},
	resolve: {
		alias: [
			{ find: "@elements/playground/react", replacement: path.resolve(__dirname, "../../playground/react/index.tsx") },
			{ find: "@elements/playground", replacement: path.resolve(__dirname, "../../playground/index.ts") },
			{ find: "@elements/ui", replacement: path.resolve(__dirname, "../../react/core/index.tsx") },
		],
	},
	server: {
		fs: {
			allow: [repoRoot],
		},
		watch: {
			ignored: ["../rs/**"],
		},
	},
});
