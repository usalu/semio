// #region 🧲Header
/** @emoji 🛝 Vite dev/build for `@semio-tech/puzzle-5d-play`. */
// #endregion 🧲Header

// #region 🔌Adapters
import { cpSync, existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import type { Plugin } from "vite";
import { createPlaygroundPlayViteConfig } from "../../../ui/styling/vite-elements-assets.ts";
// #endregion 🔌Adapters

const playDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(playDir, "../../..");
const puzzle5dFixtureRoot = path.resolve(playDir, "../fixture");

function puzzle5dFixtureServePlugin(fixtureRoot: string): Plugin {
	const prefix = "/puzzle-5d-fixture/";
	return {
		name: "puzzle-5d-fixture-serve",
		configureServer(server) {
			server.middlewares.use((req, res, next) => {
				if (!req.url?.startsWith(prefix)) {
					next();
					return;
				}
				const rel = decodeURIComponent(req.url.slice(prefix.length).split(/[?#]/, 1)[0] ?? "");
				const filePath = path.resolve(fixtureRoot, rel);
				if (!filePath.startsWith(fixtureRoot)) {
					res.statusCode = 403;
					res.end();
					return;
				}
				try {
					res.setHeader("Content-Type", "application/json");
					res.end(readFileSync(filePath));
				} catch {
					next();
				}
			});
		},
		closeBundle() {
			if (!existsSync(fixtureRoot)) return;
			const dest = path.resolve(playDir, "dist", "puzzle-5d-fixture");
			cpSync(fixtureRoot, dest, { recursive: true });
		},
	};
}
const threeModule = path.resolve(repoRoot, "node_modules/three/build/three.module.js");
const threePackageRoot = path.resolve(repoRoot, "node_modules/three");

export default createPlaygroundPlayViteConfig({
	playDir,
	repoRoot,
	playEntryKind: "5d",
	extraPlugins: [puzzle5dFixtureServePlugin(puzzle5dFixtureRoot)],
	extraAliases: [
		{ find: "@semio-tech/puzzle-2d-react", replacement: path.resolve(repoRoot, "puzzle/2d/react/index.tsx") },
		{ find: "@semio-tech/puzzle-3d-react", replacement: path.resolve(repoRoot, "puzzle/3d/react/index.tsx") },
		{ find: "@semio-tech/puzzle-5d-react", replacement: path.resolve(playDir, "../react/index.tsx") },
		{ find: /^three$/, replacement: threeModule },
		{ find: /^three\/addons\/(.*)$/, replacement: `${threePackageRoot}/examples/jsm/$1` },
	],
	build: { outDir: "dist", emptyOutDir: true },
	resolveDedupe: ["react", "react-dom", "three", "@semio-tech/puzzle-2d-react", "@semio-tech/puzzle-3d-react", "@semio-tech/puzzle-5d-react"],
	optimizeDeps: {
		include: [
			"react",
			"react-dom",
			"react/jsx-runtime",
			"react/jsx-dev-runtime",
			"three",
			"@react-three/fiber",
			"@react-three/drei",
			"lucide-react",
			"@semio-tech/infinite-world-r3f",
			"@semio-tech/puzzle-2d-react",
			"@semio-tech/puzzle-3d-react",
			"@semio-tech/puzzle-5d-react",
		],
		esbuildOptions: { target: "esnext" },
	},
});
