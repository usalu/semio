// #region 🧲Header
/** @emoji 🛝 Vite dev/build for `@semio-tech/writer-play`. */
// #endregion 🧲Header

import path from "node:path";
import { fileURLToPath } from "node:url";
import { createPlaygroundPlayViteConfig } from "../../ui/styling/vite-elements-assets.ts";

const playDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(playDir, "../..");

export default createPlaygroundPlayViteConfig({
	playDir,
	repoRoot,
	playEntryKind: "writer",
	extraAliases: [
		{ find: "@semio-tech/writer-react", replacement: path.resolve(playDir, "../react/index.tsx") },
		{ find: "@semio-tech/writer-core", replacement: path.resolve(playDir, "../core/index.ts") },
		{ find: "@semio-tech/trinity-jack-lsp-worker", replacement: path.resolve(playDir, "../../trinity/jack/lsp/worker.ts") },
	],
	resolveDedupe: ["react", "react-dom", "@semio-tech/writer-react"],
	optimizeDeps: {
		include: ["react", "react-dom"],
		esbuildOptions: { target: "esnext" },
	},
});
