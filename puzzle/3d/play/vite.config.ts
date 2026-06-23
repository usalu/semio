// #region 🧲Header
/** @emoji 🛝 Vite dev/build for `@semio-tech/puzzle-3d-play` (mesh middleware + three aliases). */
// #endregion 🧲Header

// #region 🔌Adapters
import path from "node:path";
import { fileURLToPath } from "node:url";
import { createPlaygroundPlayViteConfig } from "../../../ui/styling/vite-elements-assets.ts";
// #endregion 🔌Adapters

const playDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(playDir, "../../..");
const threeModule = path.resolve(repoRoot, "node_modules/three/build/three.module.js");
const threePackageRoot = path.resolve(repoRoot, "node_modules/three");

export default createPlaygroundPlayViteConfig({
	playDir,
	repoRoot,
	playEntryKind: "3d",
	extraAliases: [
		{ find: "@semio-tech/puzzle-3d-react", replacement: path.resolve(playDir, "../react/index.tsx") },
		{ find: /^three$/, replacement: threeModule },
		{ find: /^three\/addons\/(.*)$/, replacement: `${threePackageRoot}/examples/jsm/$1` },
	],
	build: { outDir: "dist", emptyOutDir: true },
	resolveDedupe: ["react", "react-dom", "three", "@semio-tech/puzzle-3d-react"],
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
			"@semio-tech/infinite-cavas-react-renderer",
			"@semio-tech/puzzle-3d-react",
		],
		esbuildOptions: { target: "esnext" },
	},
});
