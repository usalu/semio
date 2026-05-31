// #region 🔌Adapters
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";
import { playgroundStaticSiteBuildOptions } from "../../../ui/styling/vite-elements-assets.ts";
// #endregion 🔌Adapters

const dir = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(dir, "../../../");
const presentationCore = resolve(repoRoot, "framework/product/presentation/core/index.ts");
const presentationRenderer = resolve(repoRoot, "framework/product/presentation/renderer/react/index.tsx");
const frameworkCore = resolve(repoRoot, "framework/core/index.ts");

export default defineConfig({
	root: dir,
	base: "./",
	publicDir: resolve(dir, "public"),
	plugins: [tailwindcss(), react()],
	build: playgroundStaticSiteBuildOptions(),
	server: {
		fs: { allow: [repoRoot] },
	},
	resolve: {
		alias: [
			{ find: "@framework/presentation/core", replacement: presentationCore },
			{ find: "@framework/presentation/renderer/react", replacement: presentationRenderer },
			{ find: "@framework/core", replacement: frameworkCore },
		],
	},
});
