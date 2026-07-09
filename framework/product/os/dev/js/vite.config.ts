import path from "node:path";
import { fileURLToPath } from "node:url";
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";
import {
	cadFixtureVitePlugin,
	gisMapTilesVitePlugins,
	resolveGisMapTileServeMode,
	uiAssetsVitePlugin,
	puzzle3dMeshesVitePlugin,
} from "../../../../../ui/styling/vite-elements-assets.ts";

const configDir = path.dirname(fileURLToPath(import.meta.url));
const playDir = path.resolve(configDir, "..");
const repoRoot = path.resolve(playDir, "../../../..");
const pluginModulesDir = path.join(playDir, "plugin-modules");
const rendererModulesDir = path.join(playDir, "renderer-modules");
const renderer = process.env.SEMIO_RENDERER ?? "react";
const plugin = process.env.SEMIO_PLUGIN ?? process.env.PLAYGROUND_APP_KIND ?? "s";
const uiAssetsRoot = path.join(repoRoot, "ui/asset");

export default defineConfig({
	root: playDir,
	publicDir: path.join(playDir, "public"),
	assetsInclude: ["**/*.wasm"],
	resolve: {
		alias: {
			"@semio-tech/ui-react": path.resolve(repoRoot, "ui/js/react/index.tsx"),
			"@semio-tech/ui-asset": path.resolve(repoRoot, "ui/asset"),
			"@semio-tech/ui-styling": path.resolve(repoRoot, "ui/styling/js"),
			"@semio-tech/infinite-cavas-react-renderer": path.resolve(repoRoot, "infinite/cavas/react-renderer/index.tsx"),
			"@semio-tech/infinite-world-r3f": path.resolve(repoRoot, "infinite/world/r3f/index.tsx"),
			"@semio-tech/framework-renderer-react": path.resolve(repoRoot, "framework/renderer/react/index.tsx"),
			"@semio-tech/framework-renderer-wgpu": path.resolve(repoRoot, "framework/renderer/wgpu/index.ts"),
			"@semio-tech/framework-core": path.resolve(repoRoot, "framework/core/js/index.ts"),
			"@semio-tech/framework-os-core": path.resolve(repoRoot, "framework/product/os/core/js/index.ts"),
			"/plugin-modules": pluginModulesDir,
			"/renderer-modules": rendererModulesDir,
		},
		dedupe: ["react", "react-dom", "three", "@react-three/fiber", "@react-three/drei"],
	},
	server: {
		port: Number(process.env.S_OS_PORT ?? 6066),
		strictPort: true,
		fs: { allow: [repoRoot, pluginModulesDir, rendererModulesDir] },
	},
	plugins: [
		...uiAssetsVitePlugin(uiAssetsRoot),
		...cadFixtureVitePlugin(repoRoot),
		...puzzle3dMeshesVitePlugin(repoRoot),
		...(plugin === "gis2d"
			? gisMapTilesVitePlugins(repoRoot, resolveGisMapTileServeMode(process.env.GIS_MAP_TILE_SERVE_MODE))
			: []),
		...(renderer === "wgpu" ? [tailwindcss()] : [react(), tailwindcss()]),
	],
	optimizeDeps: {
		include: [
			"react-reconciler",
			"react-reconciler/constants",
			"three",
			"@react-three/fiber",
			"fuse.js",
		],
		exclude: [
			...(renderer === "wgpu" ? ["@semio-tech/framework-renderer-react"] : []),
			"@semio-tech/gis-2d-rs",
			"@semio-tech/framework-graph-rs",
			"@semio-tech/framework-editor-rs",
		],
	},
	define: {
		"import.meta.env.VITE_SEMIO_PLUGIN": JSON.stringify(process.env.SEMIO_PLUGIN ?? "s"),
		"import.meta.env.VITE_SEMIO_RENDERER": JSON.stringify(renderer),
	},
});
