// #region 🧲Header
/** @emoji 🛝 Vite dev/build for `@puzzle/5d/play`. */
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
	playEntryKind: "5d",
	extraAliases: [
		{ find: "@puzzle/2d/react", replacement: path.resolve(repoRoot, "puzzle/2d/react/index.tsx") },
		{ find: "@puzzle/3d/react", replacement: path.resolve(repoRoot, "puzzle/3d/react/index.tsx") },
		{ find: "@puzzle/5d/react", replacement: path.resolve(playDir, "../react/index.tsx") },
		{ find: /^three$/, replacement: threeModule },
		{ find: /^three\/addons\/(.*)$/, replacement: `${threePackageRoot}/examples/jsm/$1` },
	],
	build: { outDir: "dist", emptyOutDir: true },
	resolveDedupe: ["react", "react-dom", "three", "@puzzle/2d/react", "@puzzle/3d/react", "@puzzle/5d/react"],
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
			"@puzzle/2d/react",
			"@puzzle/3d/react",
			"@puzzle/5d/react",
		],
		esbuildOptions: { target: "esnext" },
	},
});
