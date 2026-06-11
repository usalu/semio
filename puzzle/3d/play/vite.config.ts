// #region 🧲Header
/** @emoji 🛝 Vite dev/build for `@puzzle/3d/play` (mesh middleware + three aliases). */
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
		{ find: "@puzzle/3d/react", replacement: path.resolve(playDir, "../react/index.tsx") },
		{ find: /^three$/, replacement: threeModule },
		{ find: /^three\/addons\/(.*)$/, replacement: `${threePackageRoot}/example/jsm/$1` },
	],
	build: { outDir: "dist", emptyOutDir: true },
	resolveDedupe: ["react", "react-dom", "three", "@puzzle/3d/react"],
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
			"@infinite/world/r3f",
			"@infinite/cavas/react-renderer",
			"@puzzle/3d/react",
		],
		esbuildOptions: { target: "esnext" },
	},
});
