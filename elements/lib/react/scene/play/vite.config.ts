import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { createReadStream, existsSync, statSync } from "node:fs";
import path from "node:path";
import { defineConfig } from "vite";

const repoRoot = path.resolve(__dirname, "../../../../../");
const meshRoot = path.resolve(repoRoot, "semio/fixtures/metabolism/representations");
const sharedPlaceholderMesh = path.resolve(repoRoot, "semio/fixtures/placeholder.glb");
const threeModule = path.resolve(__dirname, "../../core/node_modules/three/build/three.module.js");
const threePackageRoot = path.resolve(__dirname, "../../core/node_modules/three");

export default defineConfig({
	root: __dirname,
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
			{ find: "@elements/ui", replacement: path.resolve(__dirname, "../../core/index.tsx") },
			{ find: "@elements/playground/react", replacement: path.resolve(__dirname, "../../../playground/react/index.tsx") },
			{ find: "@elements/playground", replacement: path.resolve(__dirname, "../../../playground/index.ts") },
			{ find: /^three$/, replacement: threeModule },
			{ find: /^three\/addons\/(.*)$/, replacement: `${threePackageRoot}/examples/jsm/$1` },
		],
		dedupe: ["react", "react-dom", "three"],
	},
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
		esbuildOptions: {
			target: "esnext",
		},
	},
});
