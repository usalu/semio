import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { createReadStream, existsSync, statSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";

const sceneRoot = path.resolve(__dirname, "..");
const repoRoot = path.resolve(__dirname, "../../../../../..");
const meshRoot = path.resolve(repoRoot, "semio/assets/fixtures/metabolism/representations");
const sharedPlaceholderMesh = path.resolve(repoRoot, "semio/assets/fixtures/placeholder.glb");

export default defineConfig({
	root: sceneRoot,
	plugins: [
		tailwindcss(),
		react(),
		{
			name: "scene-play-meshes",
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
		},
	],
	server: {
		fs: {
			allow: [repoRoot],
		},
	},
	build: {
		target: "esnext",
		outDir: "play/dist",
		emptyOutDir: true,
	},
	resolve: {
		alias: [
			{ find: "@elements/ui", replacement: path.resolve(__dirname, "../../../index.tsx") },
			{ find: /^three$/, replacement: path.resolve(repoRoot, "node_modules/three/build/three.module.js") },
		],
		dedupe: ["three"],
	},
});
