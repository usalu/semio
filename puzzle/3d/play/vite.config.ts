// #region 🧲Header
/** @emoji 🛝 Vite dev/build for `@puzzle/3d/play` (mesh middleware + three aliases). */
// #endregion 🧲Header

// #region 🔌Adapters
import { createReadStream, existsSync, statSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import type { Plugin } from "vite";
import { createPlaygroundPlayViteConfig } from "../../../ui/styling/vite-elements-assets.ts";
// #endregion 🔌Adapters

const playDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(playDir, "../../..");
const meshRoot = path.resolve(repoRoot, "semio/assets/semio/metabolism/representations");
const sharedPlaceholderMesh = path.resolve(repoRoot, "semio/assets/mesh/placeholder.glb");
const threeModule = path.resolve(repoRoot, "node_modules/three/build/three.module.js");
const threePackageRoot = path.resolve(repoRoot, "node_modules/three");

const scenePlayMeshes: Plugin = {
	name: "puzzle-3d-play-meshes",
	configureServer(server) {
		server.middlewares.use((req, res, next) => {
			if (!req.url?.startsWith("/meshes/")) {
				next();
				return;
			}
			const rawName = decodeURIComponent(req.url.slice("/meshes/".length).split(/[?#]/, 1)[0] ?? "");
			const filePath =
				rawName === "placeholder.glb" ? sharedPlaceholderMesh : path.resolve(meshRoot, rawName);
			if (!filePath.startsWith(`${meshRoot}${path.sep}`) || !existsSync(filePath) || !statSync(filePath).isFile()) {
				if (filePath !== sharedPlaceholderMesh) {
					next();
					return;
				}
			}
			if (!existsSync(filePath) || !statSync(filePath).isFile()) {
				next();
				return;
			}
			res.setHeader("Content-Type", "model/gltf-binary");
			createReadStream(filePath).pipe(res);
		});
	},
};

export default createPlaygroundPlayViteConfig({
	playDir,
	repoRoot,
	extraPlugins: [scenePlayMeshes],
	extraAliases: [
		{ find: "@puzzle/3d/react", replacement: path.resolve(playDir, "../react/index.tsx") },
		{ find: /^three$/, replacement: threeModule },
		{ find: /^three\/addons\/(.*)$/, replacement: `${threePackageRoot}/examples/jsm/$1` },
	],
	build: { outDir: "dist", emptyOutDir: true },
	resolveDedupe: ["react", "react-dom", "three"],
	optimizeDeps: {
		include: [
			"react",
			"react-dom",
			"react/jsx-runtime",
			"react/jsx-dev-runtime",
			"three",
			"@react-three/fiber",
			"@react-three/drei",
		],
		esbuildOptions: { target: "esnext" },
	},
});
