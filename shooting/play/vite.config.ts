// #region 🧲Header
/** @emoji 🛝 Vite dev/build for `@semio-tech/shooting-play`. */
// #endregion 🧲Header

import path from "node:path";
import { fileURLToPath } from "node:url";
import { createPlaygroundPlayViteConfig } from "../../ui/styling/vite-elements-assets.ts";

const playDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(playDir, "../..");
const threeModule = path.resolve(repoRoot, "node_modules/three/build/three.module.js");

export default createPlaygroundPlayViteConfig({
	playDir,
	repoRoot,
	playEntryKind: "shooting",
	extraAliases: [
		{ find: "@semio-tech/shooting-react", replacement: path.resolve(playDir, "../react/index.tsx") },
		{ find: /^three$/, replacement: threeModule },
	],
	resolveDedupe: ["react", "react-dom", "three", "@semio-tech/shooting-react"],
	optimizeDeps: {
		include: ["react", "react-dom", "three", "@react-three/fiber", "@react-three/drei", "@semio-tech/infinite-world-r3f", "@semio-tech/shooting-react"],
		esbuildOptions: { target: "esnext" },
	},
});
