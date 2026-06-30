// #region 🧲Header
/** @emoji 🛝 Vite dev/build for `@semio-tech/raster-play`. */
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
	playEntryKind: "raster",
	extraAliases: [
		{ find: "@semio-tech/raster-play", replacement: path.resolve(playDir, "./index.ts") },
		{ find: "@semio-tech/raster-react", replacement: path.resolve(playDir, "../react/index.tsx") },
		{ find: "@semio-tech/raster-core", replacement: path.resolve(playDir, "../core/index.ts") },
		{ find: /^three$/, replacement: threeModule },
	],
	resolveDedupe: ["react", "react-dom", "@semio-tech/raster-react", "three"],
	optimizeDeps: {
		include: ["react", "react-dom", "@semio-tech/raster-react"],
		esbuildOptions: { target: "esnext" },
	},
});
