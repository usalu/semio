// #region 🧲Header
/** @emoji 🛝 Vite dev/build for `@semio-tech/draw-play`. */
// #endregion 🧲Header

import path from "node:path";
import { fileURLToPath } from "node:url";
import { createPlaygroundPlayViteConfig } from "../../ui/styling/vite-elements-assets.ts";

const playDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(playDir, "../..");

export default createPlaygroundPlayViteConfig({
	playDir,
	repoRoot,
	playEntryKind: "draw",
	extraAliases: [
		{ find: "@semio-tech/draw-play", replacement: path.resolve(playDir, "./index.ts") },
		{ find: "@semio-tech/draw-react", replacement: path.resolve(playDir, "../react/index.tsx") },
		{ find: "@semio-tech/draw-core", replacement: path.resolve(playDir, "../core/index.ts") },
		{ find: "@semio-tech/geometry-drawing-js", replacement: path.resolve(playDir, "../../geometry/drawing/js/index.ts") },
	],
	resolveDedupe: ["react", "react-dom", "@semio-tech/draw-react"],
	optimizeDeps: {
		include: ["react", "react-dom", "@semio-tech/draw-react"],
		esbuildOptions: { target: "esnext" },
	},
});
